mod app;
mod ui;
mod event_handler;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use ratatui::Terminal;
use ratatui::crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{Backend, CrosstermBackend};
use std::{io, error::Error};
use app::App;
use event_handler::handle_key_event;

use crate::app::models::AppEvents;

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        if app.handle_rx() {
            return Ok(());
        }
        terminal.draw(|f| ui::draw(f, app))?;
        thread::sleep(Duration::from_millis(16));
    }
}

fn event_handler(tx: Sender<AppEvents>, should_stop: Arc<AtomicBool>) {
    loop {
        if should_stop.load(Ordering::Relaxed) {
            break;
        }
        handle_key_event(&tx);
    }
}

fn update_system(tx: Sender<AppEvents>, should_stop: Arc<AtomicBool>) {
    loop {
        if should_stop.load(Ordering::Relaxed) {
            break;
        }
        thread::sleep(Duration::from_millis(750));
        tx.send(AppEvents::UPDATE).unwrap();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // setup atomic bool
    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_app = should_stop.clone();
    let should_stop_event = should_stop_app.clone();
    
    // mpsc
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    // app
    let mut app = App::new(rx);
    
    let update_app_thread = thread::spawn(move || {
        update_system(tx1, should_stop_app);
    });
    
    let update_event_handler_thread = thread::spawn(move || {
        event_handler(tx, should_stop_event)
    });

    // main loop
    run_app(&mut terminal, &mut app)?;
    
    // shut down
    should_stop.store(true, Ordering::Relaxed);
    
    update_app_thread.join().unwrap();
    update_event_handler_thread.join().unwrap();
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    Ok(())
}
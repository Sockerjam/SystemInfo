mod app;
mod ui;
mod event_handler;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
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

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
    should_stop: Arc<AtomicBool>) -> io::Result<()> {
    loop {
        if should_stop.load(Ordering::Relaxed) { return Ok(()) }
        {
            let mut app_guard = app.lock().unwrap();
            terminal.draw(|f| ui::draw(f, &mut app_guard))?;
        }
        // Target ~60 FPS: sleep for ~16ms between frames
        thread::sleep(Duration::from_millis(16));
    }
}

fn event_handler(app: Arc<Mutex<App>>, should_stop: Arc<AtomicBool>) -> io::Result<()> {
    loop {
        {
            if handle_key_event(&app)? {
                should_stop.store(true, Ordering::Relaxed);
                return Ok(())
            }
        }
    }
}

fn update_system(app: Arc<Mutex<App>>, should_stop: Arc<AtomicBool>) {
    loop {
        if should_stop.load(Ordering::Relaxed) {
            break;
        }
        thread::sleep(Duration::from_secs(1));
        let mut app_guard = app.lock().unwrap();
        app_guard.update();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let should_stop = Arc::new(AtomicBool::new(false));
    let should_stop_app = should_stop.clone();
    let should_stop_ui = should_stop.clone();
    let app = Arc::new(Mutex::new(App::new()));
    let app_update = app.clone();
    let event_handler_app = app.clone();
    
    let update_app_thread = thread::spawn(move || {
        update_system(app_update, should_stop_app);
    });
    
    let update_event_handler_thread = thread::spawn(move || {
        event_handler(event_handler_app, should_stop)
    });

    run_app(&mut terminal, app, should_stop_ui)?;
    
    update_app_thread.join().unwrap();
    update_event_handler_thread.join().unwrap()?;
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    
    Ok(())
}
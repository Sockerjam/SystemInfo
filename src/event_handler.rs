use ratatui::crossterm::event::{self, Event, KeyCode};
use std::{io, sync::{Arc, Mutex}, time::Duration};
use crate::app::App;

pub fn handle_key_event(app: &Arc<Mutex<App>>) -> io::Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        let mut app_guard = app.lock().unwrap();
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                return Ok(false) 
            }

            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Down => {
                    app_guard.cpu_scroll_position = app_guard.cpu_scroll_position.saturating_add(1);
                    app_guard.cpu_scroll_state = app_guard.cpu_scroll_state.position(app_guard.cpu_scroll_position);
                    return Ok(false)
                },
                KeyCode::Up => {
                    app_guard.cpu_scroll_position = app_guard.cpu_scroll_position.saturating_sub(1);
                    app_guard.cpu_scroll_state = app_guard.cpu_scroll_state.position(app_guard.cpu_scroll_position);
                    return Ok(false)
                },
                
                _ => return Ok(false)
            }

        }
    }

    Ok(false)
}
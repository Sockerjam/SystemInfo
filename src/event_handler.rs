use ratatui::crossterm::event::{self, Event, KeyCode};
use std::{io, sync::{Arc, Mutex, mpsc::Sender}, time::Duration};
use crate::app::{App, models::AppEvents};

pub fn handle_key_event(tx: &Sender<AppEvents>) {
    match event::poll(Duration::from_millis(50)) {
        Ok(true) => {
            match event::read() {
                Ok(Event::Key(key)) => {
                    if key.kind == event::KeyEventKind::Release {
                        return
                    }
                    match key.code {
                        KeyCode::Char('q') => tx.send(AppEvents::QUIT).unwrap(),
                        KeyCode::Down => {
                            tx.send(AppEvents::DOWN).unwrap();
                        },
                        KeyCode::Up => {
                            tx.send(AppEvents::UP).unwrap();
                        },

                        _ => return
                    }
                },
                _ => return
            }
        },
        _ => {}
    }
}
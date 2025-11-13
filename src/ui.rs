mod colors;
mod widgets;
use ratatui::{Frame, 
    layout::{Constraint, Direction, Layout, Margin}, 
    prelude::Color, style::Stylize, 
    symbols::border, text::Line, 
    widgets::{Block, Borders, Padding, Scrollbar, ScrollbarOrientation}};
use crate::{app::App, 
    ui::widgets::{cpu_widget::{self, CPUWidget}, memory_widget::{self, MemoryWidget}, os_info_widget::{self, OsInfoWidget}}};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let main_window = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Fill(1)
        ])
        .split(frame.area());

    let terminal = main_frame();
    let inner_area = terminal.inner(main_window[0]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(30)
        ])
        .split(inner_area);

    let section = make_section_block(true);
    let os_info_section = make_section_block(false);
    
    let os_info_area = os_info_section.inner(chunks[0]);
    let memory_area = section.inner(chunks[1]);
    let cpu_area = section.inner(chunks[2]);
    let disk_area = section.inner(chunks[3]);

    let memory_section = section.clone().title(make_title("Memory".to_string()));
    let cpu_section = section.clone().title(make_title("CPU's".to_string()));
    let disk_section = section.clone().title(make_title("Disk's".to_string()));
    
    frame.render_widget(terminal, main_window[0]);
    frame.render_widget(os_info_section, chunks[0]);
    frame.render_widget(memory_section, chunks[1]);
    frame.render_widget(cpu_section, chunks[2]);
    frame.render_widget(disk_section, chunks[3]);
    
    let os_info_widget = OsInfoWidget::new(app.get_os_info());
    frame.render_widget(os_info_widget, os_info_area);

    let memory_widget = MemoryWidget::new(app.get_memory());
    frame.render_widget(memory_widget, memory_area);
    
    let cpu_widget = CPUWidget::new(app.get_cpus(), app.cpu_scroll_position, cpu_area.height);
    frame.render_widget(cpu_widget, cpu_area);
    
    frame.render_stateful_widget(
    Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓")),
        cpu_area.inner(Margin{vertical: 1, horizontal: 0}),
        &mut app.cpu_scroll_state,
    );
}

fn main_frame() -> Block<'static> {
    let title = Line::from(" System Info ".bold());
    let instructions = Line::from(vec![
        " Quit ".into(),
        "<Q> ".blue().bold()
    ]);
    Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK)
}

fn make_title(title: String) -> Line<'static> {
    Line::from(title.bold().fg(Color::Green))
}

fn make_section_block(use_padding: bool) -> Block<'static> {
    let padding = if use_padding {Padding::vertical(1)} else {Padding::vertical(0)};
    Block::new()
        .borders(Borders::ALL)
        .border_set(border::EMPTY)
        .padding(padding)
}

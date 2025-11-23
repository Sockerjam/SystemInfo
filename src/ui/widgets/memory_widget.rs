use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use ratatui::prelude::*;
use crate::ui::colors::{GREEN, ORANGE, RED};
use crate::app::models::Memory;

pub struct MemoryWidget<'a> {
    memory: &'a Memory
}

impl<'a> Widget for MemoryWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(2),
                Constraint::Length(1),
            ])
            .split(area);

        let memory_gauge = MemoryWidget::create_gauge("RAM Usage", self.memory.used_memory, self.memory.total_memory);
        let memory_paragraph = MemoryWidget::create_paragraph(self.memory.used_memory, self.memory.total_memory);
        memory_gauge.render(chunks[0], buf);
        memory_paragraph.render(chunks[1], buf);
        
        if MemoryWidget::swap_is_active(self.memory.total_swap) {
            let swap_gauge = MemoryWidget::create_gauge("Swap Usage", self.memory.used_swap, self.memory.total_swap);
            let swap_paragraph = MemoryWidget::create_paragraph(self.memory.used_swap, self.memory.total_swap);
            swap_gauge.render(chunks[2], buf);
            swap_paragraph.render(chunks[3], buf);
        } else {
            let swap_not_allocated = Paragraph::new("Swap Memory Not Allocated")
            .block(MemoryWidget::title_block("Swap Usage"));
            let x = chunks[2].x;
            let y = chunks[2].y + 1;
            let new_rectangle_width = chunks[2].width + chunks[3].width;
            let new_rectangle_height = chunks[2].height + chunks[3].height;
            swap_not_allocated.render(Rect::new(x, y, new_rectangle_width, new_rectangle_height), buf);
        }
    }
}

impl<'a> MemoryWidget<'a> {
    
    pub fn new(memory: &'a Memory) -> Self {
        Self {
            memory
        }
    }

    fn create_gauge(title: &'static str, used: f32, total: f32) -> Gauge<'static> {
        let ratio = (used / total) as f64;
        Gauge::default()
            .block(MemoryWidget::title_block(title))
            .ratio(ratio)
            .use_unicode(true)
            .gauge_style(Style::default().bg(Color::Gray).fg(MemoryWidget::get_color(used, total)))
            .label(Span::styled(
                MemoryWidget::get_percentage(used, total), 
                Style::default().fg(Color::White)))
    }
    
    fn get_color(used: f32, total: f32) -> Color {
        let ratio = used / total;
        match ratio {
            0.0..=0.50 => GREEN,
            0.51..=0.70 => ORANGE,
            0.71..=1.0 => RED,
            _ => GREEN
        }
    }
    
    fn create_paragraph(used: f32, total: f32) -> Paragraph<'static> {
        let label = format!("({:.2} GB / {:.2} GB)", used, total);
        Paragraph::new(label)
    }
    
    fn swap_is_active(total: f32) -> bool {
        total != 0.0
    }
    
    fn get_percentage(used: f32, total: f32) -> String {
        let percentage = (used / total) * 100.0;
        format!("{:.2}%", percentage)
    }
    
    fn title_block(title: &'a str) -> Block<'a> {
        let title = Line::from(title)
            .left_aligned()
            .style(Style::default()
            .underline_color(Color::White))
            .add_modifier(Modifier::UNDERLINED);
        Block::new()
            .borders(Borders::NONE)
            .title(title)
    }
}
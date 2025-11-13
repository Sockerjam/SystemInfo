use ratatui::{buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::Style, text::{Line, Span}, widgets::{Block, Borders, Sparkline, SparklineBar, Widget}};
use crate::ui::colors::{GREEN, ORANGE, RED};

use crate::app::models::CPU;

pub struct CPUWidget<'a> {
    cpus: &'a Vec<CPU>,
    scroll_offset: usize,
    viewport_height: u16
}

impl<'a> CPUWidget<'a> {
    pub fn new(cpus: &'a Vec<CPU>, scroll_offset: usize, viewport_height: u16) -> Self {
        Self { 
            cpus,
            scroll_offset,
            viewport_height
        }
    }
}

impl<'a> Widget for CPUWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        
        let rows_per_core = 5;
        let core_start_index = self.scroll_offset / rows_per_core;
        let visible_cores = self.viewport_height / rows_per_core as u16;
        let core_end_index = (core_start_index + visible_cores as usize).min(self.cpus.len());
        
        let visible_cpus = &self.cpus[core_start_index..core_end_index];
        
        let constraints: Vec<Constraint> = visible_cpus.iter().map(|_| 
            Constraint::Length(5)
        ).collect();
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)   
            .constraints(constraints)
            .split(area);
        
        for (index, cpu) in visible_cpus.iter().enumerate() {
            let sparkline = self.create_cpu_sparkline(cpu);
            let mut area = chunks[index];
            area.width = area.width.min(75);
            sparkline.render(area, buf);
        }
    }
}

impl<'a> CPUWidget<'a> {

    fn create_cpu_sparkline(&'a self, cpu: &'a CPU) -> Sparkline<'a> {
        let line = Line::from(vec![
            "Core: ".into(),
            cpu.core.as_str().into(),
            ". ".into(),
            cpu.usage_history.back().unwrap_or(&0).to_string().into(),
            "%".into()
        ]);

        let color = match cpu.usage_history.back() {
            Some(usage) => match usage {
                0..=50 => GREEN,
                51..=70 => ORANGE,
                71..=100 => RED,
                _ => GREEN
            },
            None => GREEN
        };
        Sparkline::default()
            .block(Block::new()
                .borders(Borders::ALL)
                .title(line))
            .data(&cpu.usage_history)
            .style(Style::default().fg(color))
    }
}
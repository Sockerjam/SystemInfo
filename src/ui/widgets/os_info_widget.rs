use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::Span, widgets::{Paragraph, Widget}};
use crate::app::models::OSInfo;

pub struct OsInfoWidget<'a> {
    os_info: &'a OSInfo
}

impl<'a> OsInfoWidget<'a> {
    pub fn new(os_info: &'a OSInfo) -> Self {
        Self {
            os_info
        }
    }
    
    fn create_paragraph(title: String) -> Paragraph<'static> {
        let span = Span::styled(
            title, 
            Style::default().fg(Color::Green));
        Paragraph::new(span).alignment(Alignment::Center)
    }
}

impl<'a> Widget for OsInfoWidget<'a> {

    fn render(self, area: Rect, buf: &mut Buffer) {
        
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1)
            ])
            .split(area);
        
        let system_name_title = format!("System: {}", self.os_info.system_name);
        let system_name_paragraph = OsInfoWidget::create_paragraph(system_name_title);
        
        let host_name_title = format!("CPU Arch: {}", self.os_info.cpu_arch);
        let host_name_paragraph = OsInfoWidget::create_paragraph(host_name_title);
        
        let os_version_title = format!("OS Version: {}", self.os_info.os_version);
        let os_version_paragraph = OsInfoWidget::create_paragraph(os_version_title);
        
        system_name_paragraph.render(chunks[0], buf);
        host_name_paragraph.render(chunks[1], buf);
        os_version_paragraph.render(chunks[2], buf);

    }
}
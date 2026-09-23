// src/ui/settings.rs
// The settings screen.
use super::selectable_line;
use super::text::{self, BOLD, DIM};
use crate::app::App;
use crate::settings::SettingsSection;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use strum::IntoEnumIterator;

const HINTS: &[(&str, &str)] =
    &[("Tab/S-Tab", "Section"), ("↑↓/jk", "Navigate"), ("←→/hl/Enter", "Change"), ("Esc", "Back"), ("?", "Help")];

pub fn render(frame: &mut Frame, app: &App, area: Rect) -> &'static [(&'static str, &'static str)] {
    let [sections_area, options_area] = Layout::horizontal([Constraint::Length(26), Constraint::Min(0)]).areas(area);

    let section_lines: Vec<Line> = SettingsSection::iter()
        .map(|section| selectable_line(format!("{:<22}", section.to_string()), section == app.settings_section))
        .collect();
    frame.render_widget(
        Paragraph::new(section_lines).block(Block::default().borders(Borders::RIGHT).title(" Sections ")),
        sections_area,
    );

    let options_area = options_area.inner(Margin { horizontal: 2, vertical: 0 });
    let width = options_area.width as usize;
    let config = app.config();
    let mut lines = vec![Line::from(Span::styled(app.settings_section.to_string(), BOLD)), Line::from("")];
    for (i, item) in app.settings_section.items().iter().enumerate() {
        lines.push(selectable_line(format!("{}: [{}]", item.label, (item.value)(config)), i == app.settings_cursor));
    }
    lines.push(Line::from(""));
    lines.extend(text::wrap_text(app.settings_section.description(), width, DIM));
    lines.push(Line::from(""));
    if let Some(path) = app.config_path() {
        lines.extend(text::wrap_text(&format!("Settings are saved to {}", path.display()), width, DIM));
    }
    frame.render_widget(Paragraph::new(lines), options_area);
    HINTS
}

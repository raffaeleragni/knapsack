use std::cmp::Ordering;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
};

use crate::app::{App, Item, Storage};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        AppFrame {}.render(area, buf);

        let outer = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .margin(2)
            .split(area);
        let occupied = self.sack.total();
        let perc_occupied = if self.capacity > 0 {
            occupied * 100 / self.capacity
        } else {
            0
        };
        let perc_occupied = match occupied.cmp(&self.capacity) {
            Ordering::Equal => perc_occupied.to_string().green(),
            Ordering::Greater => perc_occupied.to_string().red(),
            Ordering::Less => perc_occupied.to_string().yellow(),
        };
        let occupied_text = Text::from(vec![Line::from(vec![
            "Occupied: ".into(),
            occupied.to_string().blue(),
            " (".into(),
            perc_occupied,
            "%)".into(),
        ])]);
        Paragraph::new(occupied_text)
            .centered()
            .render(outer[0], buf);

        let columns = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outer[1]);
        self.inventory.render(columns[0], buf);
        self.sack.render(columns[1], buf);
    }
}

struct AppFrame;

impl Widget for &AppFrame {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        let title = Title::from(" Knapsack ".bold());
        let instructions = Title::from(Line::from(vec![" Quit ".into(), "<q> ".blue().bold()]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);
        block.render(area, buf);
    }
}

impl Widget for &Storage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(buf.area);
        let lines = self.items.iter().map(|i| i.into()).collect::<Vec<Span>>();
        let mut text = Text::default();
        text.extend(lines);
        Paragraph::new(text)
            .block(
                Block::new()
                    .title(Title::from(self.name.as_ref()).alignment(Alignment::Center))
                    .borders(Borders::ALL)
                    .border_set(border::THICK),
            )
            .render(area, buf);
    }
}

impl<'a> From<&Item> for Span<'a> {
    fn from(val: &Item) -> Self {
        let weight = val.weight.to_string();
        let text = format!("{:^0width$}", weight, width = val.size);
        Span::from(text).bold().style(
            Style::default()
                .fg(if !val.selected {
                    Color::White
                } else {
                    Color::Black
                })
                .bg(if !val.selected {
                    Color::DarkGray
                } else {
                    Color::Gray
                }),
        )
    }
}

use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{self, Span},
    widgets::{
        canvas::{self, Canvas, Circle, Map, MapResolution, Rectangle},
        Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph,
        Row, Sparkline, Table, Tabs, Wrap, Borders,
    },
    Frame,
};
use ratatui::prelude::*;

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Length(1),
        ])
        .split(frame.area());

    // let title_block = Block::default()
    //     .borders(Borders::ALL)
    //     .style(Style::default());

    // let title = Paragraph::new(Text::styled(
    //     "Create New Json",
    //     Style::default().fg(Color::Green),
    // ))
    // .block(title_block);

    let mut list_items = Vec::<ListItem>::new();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .title(
            Line::from("Countdown Timers")
            .left_aligned()
            .style(Style::default().fg(Color::Yellow)));

    // for key in app.pairs.keys() {
    //     list_items.push(ListItem::new(Line::from(Span::styled(
    //         format!("{: <25} : {}", key, app.pairs.get(key).unwrap()),
    //         Style::default().fg(Color::Yellow),
    //     ))));
    // }

    let list = List::new(list_items)
        .block(list_block);

    frame.render_widget(list, chunks[0]);
}

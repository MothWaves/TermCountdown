use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{self, Span},
    widgets::{
        canvas::{self, Canvas, Circle, Map, MapResolution, Rectangle},
        Axis, BarChart, Block, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem, Paragraph,
        Row, Sparkline, Table, Tabs, Wrap,
    },
    Frame,
};

use crate::app::App;

pub fn draw(frame: &mut Frame, app: &mut App) {
}

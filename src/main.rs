use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};


// Uses app.rs and ui.rs from this project.

// mod app;
// mod ui;
// use crate::{
//     app::App,
// };

pub fn main() -> Result<(), Box<dyn Error>> {

}

fn run() {
}

// pub fn match_key_press(key: KeyEvent, app: &mut App) {
//     if key.modifiers == KeyModifiers::CONTROL {
//         match key.code {
//             KeyCode::Char('c') => app.should_quit = true,
//             _ => {}
//         }
//     }
//     else {
//         match key.code {
//             KeyCode::Left | KeyCode::Char('h') => app.on_left(),
//             KeyCode::Up | KeyCode::Char('k') => app.on_up(),
//             KeyCode::Right | KeyCode::Char('l') => app.on_right(),
//             KeyCode::Down | KeyCode::Char('j') => app.on_down(),
//             // TODO Make note on how uppercase is differently registered from lowercase.
//             // KeyCode::Char('J') => {
//             //     app.should_quit = true;
//             //     app.log("UPPERCASE J");
//             // },
//             KeyCode::Esc => app.on_esc(),
//             KeyCode::Char('q') => app.should_quit = true,
//             _ => {}
//         }
//     }
// }

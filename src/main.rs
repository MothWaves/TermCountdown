use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};


// Uses app.rs and ui.rs from this project.

// mod app;
// mod ui;
// use crate::{
//     app::App,
// };

pub fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    static mut display_num: i32 = 0;
    loop {
        terminal.draw(render)?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char(c) => {
                    if c == 'q' {
                        break Ok(())
                    }
                }
                _ => {}
            }
        }
        unsafe {
            display_num += 1;
        }
    }
}

fn render<'a>(frame: &mut Frame) {
    unsafe {
        let display_text: 'a String = String::from(display_num);
    }
    frame.render_widget(display_text, frame.area())
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

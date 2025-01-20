use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};


// Uses app.rs and ui.rs from this project.

mod app;
mod ui;
use crate::{
    app::App,
};

pub fn main() -> Result<(), Box<dyn Error>> {
    // setup variables
    let tick_rate = Duration::from_millis(250);

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("TermCountdown");
    let app_result = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = app_result {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match_key_press(key.code, app);
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

pub fn match_key_press(key_code: KeyCode, mut app: App) {
    match key_code {
        KeyCode::Left | KeyCode::Char('h') => app.on_left(),
        KeyCode::Up | KeyCode::Char('k') => app.on_up(),
        KeyCode::Right | KeyCode::Char('l') => app.on_right(),
        KeyCode::Down | KeyCode::Char('j') => app.on_down(),
        KeyCode::Char(c) => app.on_key(c),
        _ => {}
    }
}

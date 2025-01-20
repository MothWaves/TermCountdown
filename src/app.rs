use std::option::Option;

pub struct Countdown {
}

pub struct App<'a> {
    pub title: &'a str,
    countdown: Option<Countdown>,
    pub should_quit: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App {
        App {
            title,
            countdown: Option::None,
            should_quit: false,
        }
    }
}

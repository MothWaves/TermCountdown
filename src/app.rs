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

    pub fn on_up(&mut self) {

    }

    pub fn on_down(&mut self) {

    }

    pub fn on_left(&mut self) {

    }

    pub fn on_right(&mut self) {
       
    }

    pub fn on_key(&mut self, c: char) {

    }

    pub fn on_esc(&mut self) {
        self.should_quit = true;
    }

    pub fn on_tick(&mut self) {

    }
}

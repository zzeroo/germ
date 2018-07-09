use config::Config;
use shell::Shell;
use ui::{Event, Ui};

pub struct App<U: Ui> {
    config: Config,
    shell: Shell,
    ui: U,
}

impl<U: Ui> App<U> {
    pub fn new(config: Config, shell: Shell, ui: U) -> Self {
        return App {
            config: config,
            shell: shell,
            ui: ui,
        };
    }

    pub fn run(mut self) -> Result<(), String> {
        'main: loop {
            for event in self.ui.events() {
                match event {
                    Event::Submit(command) => {
                        eprintln!("submitted: {}", command);
                    }
                    // break loop
                    Event::Exit => return Ok(()),
                }
            }

            match self.ui.draw() {
                Ok(()) => (),
                Err(e) => return Err(format!("error drawing ui:\n{}", e)),
            }
        }
    }
}

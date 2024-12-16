use color_eyre::eyre::Result;
use enigo::{Enigo, MouseControllable};
use std::{thread, time::Duration};
use tracing::{debug, info};

pub struct MouseMover {
    enigo: Enigo,
}

impl MouseMover {
    pub fn new() -> Self {
        Self { 
            enigo: Enigo::new() 
        }
    }

    pub fn move_mouse(&mut self) {
        debug!("Moving mouse slightly");
        self.enigo.mouse_move_relative(1, 0);
        self.enigo.mouse_move_relative(-1, 0);
    }

    pub fn run(&mut self, delay: u64) -> Result<()> {
        info!("Starting mouse movement prevention. Delay: {} seconds", delay);
        loop {
            self.move_mouse();
            thread::sleep(Duration::from_secs(delay));
        }
    }
}

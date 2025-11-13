use tcod::Console;
use tcod::console::Offscreen;

use crate::sidescreen::SideScreen;

pub struct Messages {
    list: Vec<String>,
    max_wnd: i32,
    current_amount: i32,
}

impl Messages {
    pub fn new() -> Self {
        let list = Vec::new();

        Messages {
            list,
            max_wnd: 4,
            current_amount: 0,
        }
    }
    pub fn add_message(&mut self, msg: String) {
        if self.current_amount >= 5 {
            self.list.remove(0);
        }
        self.list.push(msg);
        self.current_amount += 1;
    }

    pub fn show_messages(&self, scr: &SideScreen) {
        let mut s = &scr.screen;
        for i in 0..self.current_amount {
            s.print(
                3,
                scr.height - 1 - (self.current_amount - i) * 2,
                &self.list[i as usize],
            );
        }
    }
}

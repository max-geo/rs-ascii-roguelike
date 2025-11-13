use crate::Entity;
use crate::World;
use crate::comps::temp_comps::{Damage, Hp};

use tcod::colors::*;
use tcod::{
    Console,
    console::{Offscreen, Root, blit},
};

pub struct SideScreen {
    pub screen: Offscreen,

    pub start_x: i32,
    pub start_y: i32,

    pub width: i32,
    pub height: i32,

    pub state: char,
}

impl SideScreen {
    pub fn new(start_x: i32, start_y: i32, width: i32, height: i32, state: char) -> Self {
        let mut screen = Offscreen::new(width, height);
        screen.set_default_background(GREY);
        screen.set_default_foreground(YELLOW);

        //border
        for x in 0..width {
            screen.put_char(x, 0, '-', tcod::BackgroundFlag::Set);
            screen.put_char(x, height - 1, '-', tcod::BackgroundFlag::Set);
        }
        for y in 0..height {
            screen.put_char(0, y, '|', tcod::BackgroundFlag::Set);
            screen.put_char(width - 1, y, '|', tcod::BackgroundFlag::Set);
        }

        screen.put_char(0, 0, '+', tcod::BackgroundFlag::Set);
        screen.put_char(0, height - 1, '+', tcod::BackgroundFlag::Set);
        screen.put_char(width - 1, 0, '+', tcod::BackgroundFlag::Set);
        screen.put_char(width - 1, height - 1, '+', tcod::BackgroundFlag::Set);

        SideScreen {
            screen,
            start_x,
            start_y,
            width,
            height,
            state,
        }
    }

    pub fn draw_screen(&self, t: &mut Root, w: &World, player: Entity) {
        match self.state {
            'c' => self.draw_ch_sheet(w, player),
            _ => (),
        }

        blit(
            &self.screen,
            (0, 0),
            (self.width, self.height),
            t,
            (self.start_x, self.start_y),
            1.0,
            1.0,
        );
    }

    pub fn draw_ch_sheet(&self, wrld: &World, player: Entity) {
        let mut s = &self.screen;
        let w = self.width;
        let h = self.height;

        s.print(2, 2, "HP: ");
        s.print(6, 2, wrld.get_component::<Hp>(player).val.to_string());

        s.print(2, 4, "DMG: ");
        s.print(7, 4, wrld.get_component::<Damage>(player).val.to_string());
    }
    pub fn make_screen(&mut self, t: &mut Root) {}
}

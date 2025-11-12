use tcod::colors::*;
use tcod::{
    Console,
    console::{Offscreen, Root},
};

pub fn make_screen(t: &mut Root, width: i32, height: i32) -> Offscreen {
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

    screen
}

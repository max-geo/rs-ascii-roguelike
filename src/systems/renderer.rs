use crate::Entity;
use crate::inventory::make_screen;
use crate::world::*;
use crate::{Player, Position, Renderable};
use tcod::colors::*;
use tcod::console::{Console, Offscreen, Root, blit};
use tcod::image::*;

pub fn show_screen(terminal: &mut Root, side_screen: &mut Offscreen, w: &World) {
    terminal.set_default_background(BLACK);
    terminal.clear();

    let renderables: Vec<Entity> = w
        .get_entities::<Renderable>()
        .into_iter()
        .filter(|e| w.has_component::<Position>(*e))
        .collect();

    let mut player: Option<Entity> = None;

    for r in renderables {
        if w.has_component::<Player>(r) {
            player = Some(r);
            continue;
        }
        terminal.put_char(
            w.get_component::<Position>(r).x,
            w.get_component::<Position>(r).y,
            w.get_component::<Renderable>(r).glyph,
            tcod::BackgroundFlag::None,
        );
    }

    terminal.put_char(
        w.get_component::<Position>(player.unwrap()).x,
        w.get_component::<Position>(player.unwrap()).y,
        w.get_component::<Renderable>(player.unwrap()).glyph,
        tcod::BackgroundFlag::None,
    );

    // let img = Image::from_file("assets/image.png").unwrap();
    // blit(
    //     &img,
    //     (1.0_f32, 1.0_f32),
    //     0.0_f32,
    //     terminal,
    //     (30.0_f32, 30.0_f32),
    //     tcod::BackgroundFlag::Set,
    // );
    terminal.set_default_background(RED);
    terminal.rect(30, 30, 5, 5, false, tcod::BackgroundFlag::Set);

    let start_x = terminal.width() - 30;

    let start_y = terminal.height() - 90;

    blit(
        side_screen,
        (0, 0),
        (30, 90),
        terminal,
        (start_x, start_y),
        1.0,
        1.0,
    );

    terminal.flush();
}

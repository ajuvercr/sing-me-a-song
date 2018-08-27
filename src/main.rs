
extern crate piston_window;
extern crate ai_behavior;
extern crate sprite;
extern crate find_folder;
extern crate graphics;
extern crate rand;
extern crate gfx_device_gl;

mod config;
use config::Config;

mod sheet;
use sheet::Sheet;

// use std::rc::Rc;

use piston_window::*;
// use sprite::*;
// use ai_behavior::{
//     Action,
//     Sequence,
//     Wait,
//     WaitForever,
//     While,
// };

fn main() {
    let width = 1500.0;
    let height = 1000.0;
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: sprite", (width as u32, height as u32))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let mut glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

    let config = Config::default();
    
    let config = config.with_padding(0.0, 0.1);

    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("assets").unwrap();
    // let id;
    // let mut scene = Scene::new();
    // let tex = Rc::new(Texture::from_path(
    //         &mut window.factory,
    //         assets.join("rust.png"),
    //         Flip::None,
    //         &TextureSettings::new()
    //     ).unwrap());
    // let mut sprite = Sprite::from_texture(tex.clone());
    // sprite.set_position(config.width as f64 / 2.0, config.height as f64 / 2.0);

    // id = scene.add_child(sprite);

    // // Run a sequence of animations.
    // let seq = Sequence(vec![
    //     Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
    //     Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
    //     Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
    //     Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
    //     Wait(0.5),
    //     Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
    //     Action(Blink(1.0, 5)),
    //     While(Box::new(WaitForever), vec![
    //         Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.3)))),
    //         Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.3)))),
    //     ]),
    // ]);
    // scene.run(id, &seq);

    // // This animation and the one above can run in parallel.
    // let rotate = While(Box::new(WaitForever), vec![
    //     Action(Ease(EaseFunction::ExponentialInOut,
    //         Box::new(RotateTo(2.0, 360.0)))),
    //     Action(Ease(EaseFunction::ExponentialInOut,
    //         Box::new(RotateTo(2.0, 0.0))))
    //     ]);
    // scene.run(id, &rotate);

    // let mut x_scale = 1.0;
    // let mut y_scale = 1.0;

    let (c1, c2) = config.split_vert(0.5, 0.1);
    let mut sheets = vec![Sheet::new(c1)];//, Sheet::new(c2)];
    
    while let Some(e) = window.next() {
        //scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            //let c = c.scale(x_scale, y_scale);
        //    scene.draw(c.transform, g);
        //    rect.draw(dims, &c.draw_state, c.transform, g);
            sheets.iter().for_each(|m| m.draw(c, g, [0.0, 1.0, 0.0, 0.3], &mut glyphs));
        });
        

        // if let Some(e) = e.resize_args() {
        //     x_scale = e[0] as f64 / config.height;
        //     y_scale = e[1] as f64 / config.width;
        // }

        if let Some(e) = e.update_args() {
            sheets.iter_mut().for_each(|m| m.update(e.dt));
        }
    }
}
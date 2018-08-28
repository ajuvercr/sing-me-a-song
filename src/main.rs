
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
use graphics::rectangle::square;

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
    let mut glyphs = Glyphs::new(font, factory.clone(), TextureSettings::new()).unwrap();

    let texture = Texture::from_path(&mut factory.clone(), assets.join("note.png"), Flip::None, &TextureSettings::new()).unwrap();
    //let texture: u32 = texture;
    println!("texture: {:?}", texture.surface);
    let info = texture.surface.get_info().to_image_info(0);
    println!("texture: {:?}", texture.surface.get_info().to_image_info(0));
    let image = Image::new().rect([0.0,0.0,info.width as f64, info.height as f64]);

    let config = Config::default();
    
    let config = config.with_padding(0.0, 0.1);

    // let mut x_scale = 1.0;
    // let mut y_scale = 1.0;

    let (c1, c2) = config.split_vert(0.5, 0.1);
    let mut sheets = vec![Sheet::new(c1, &assets, factory.clone())];//, Sheet::new(c2)];
    
    while let Some(e) = window.next() {
        //scene.event(&e);

        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            //let c = c.scale(x_scale, y_scale);
        //    scene.draw(c.transform, g);
        //    rect.draw(dims, &c.draw_state, c.transform, g);
            sheets.iter().for_each(|m| m.draw(c, g, [0.0, 1.0, 0.0, 0.3], &mut glyphs));

            image.draw(&texture, &c.draw_state, c.transform, g);
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
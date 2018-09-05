

//use gfx_device_gl::{Factory, Resources};
use graphics::{Graphics, Context, ImageSize, Ellipse, color, Line, Rectangle};
use graphics::types::Color;

use piston_window::{Transformed, text};
use graphics::glyph_cache::rusttype::GlyphCache;
use gfx_device_gl::{Factory, Resources, CommandBuffer};
use piston_window::texture::CreateTexture;
use piston_window::*;


use rand;
use rand::Rng;
use std::path::PathBuf;

use config::Config;

pub struct Sheet<T> 
    where T: ImageSize + Clone + CreateTexture<Factory> {

    config: Config,
    sheet: Vec<Measure<T>>,
    sheet_config: SheetConfig<T>,

    note_x : f64,
}

pub struct SheetConfig<T>
    where T: ImageSize + Clone + CreateTexture<Factory> {
    textures: Vec<T>,
    images: Vec<Image>,
    scale: f64,
}

impl Sheet<Texture<Resources>> {
    pub fn new(config: Config, folder: &PathBuf, factory: Factory) -> Sheet<Texture<Resources>> {
        println!("new sheet: {:?}", config);
        let mut textures = Vec::new();
        let mut images = Vec::new();

        let texture_settings = TextureSettings::new();
        let path = folder.join("note0.png");
        let tex = Texture::from_path(&mut factory.clone(), path, Flip::None, &texture_settings).unwrap();
        let info = tex.surface.get_info().to_image_info(0);

        let height = config.height / config.lines;
        let scale = height / info.height as f64;

        let d_width = info.width as f64 * scale / 2.0;
        let d_height = height / 2.0;

        images.push(
            Image::new().rect([d_width/2.0, - (info.height as f64) + d_height, info.width as f64, info.height as f64])
        );
        textures.push(tex);

        (1..config.smallest_note + 1).for_each(|n| {
            let path = folder.join(format!("note{}.png", n));
            let tex = Texture::from_path(&mut factory.clone(), path, Flip::None, &texture_settings).unwrap();
            let info = tex.surface.get_info().to_image_info(0);
            images.push(
                Image::new().rect([d_width / 2.0, - (info.height as f64) + d_height, info.width as f64, info.height as f64])
            );
            textures.push(tex);
        });
        let sc = SheetConfig {textures, images, scale};
        let mut sheet = Vec::new();
        let mut x = 0.0;
        while x < config.width + config.measure_size {
            sheet.push(Measure::new(config.clone(), x, &sc));

            x += config.measure_size;
        }

        Sheet {
            note_x: 200.0,
            sheet, config, 
            sheet_config: sc,
        }
    }

    pub fn check(&mut self, note: char) -> bool {
        let mut measures = self.sheet.iter_mut();

        while let Some(m) = measures.next() {
            if m.x + self.config.measure_size < self.note_x {
                continue;
            }
            return m.check(note, self.note_x);
        }

        false
    }

    pub fn draw<G>(&self, c: Context, g: &mut G, color: Color, glyphs: &mut GlyphCache<Factory, Texture<Resources>>) 
        where G: Graphics<Texture = Texture<Resources>> {
            
            let rect = Rectangle::new(color);
            rect.draw(self.config.as_dims(), &c.draw_state, c.transform, g);

            let c = self.config.trans(&c);
            self.sheet.iter().for_each(|m| m.draw(c, g, glyphs));

            Line::new([1.0,0.0,0.0,1.0], 2.0).draw([self.note_x, 0.0, self.note_x, self.config.height], &c.draw_state, c.transform, g);
    }

    pub fn update(&mut self, dt: f64) {
        self.sheet.iter_mut().for_each(|m| m.update(dt));
        let replace = self.sheet.first().unwrap().is_out_screen();
        if replace {
            let x = self.sheet.last().unwrap().x;
            let x = x + self.config.measure_size;
            self.sheet.remove(0);
            self.sheet.push(
                Measure::new(self.config.clone(), x, &self.sheet_config)
            );
        }
    }
}

pub struct Measure<T>
    where T: ImageSize + Clone {
    notes: Vec<Note<T>>,
    pub x: f64,
    config: Config,
}

impl<T> Measure<T> 
    where T: ImageSize + Clone + CreateTexture<Factory> {
    pub fn new(config: Config, x: f64, sc: &SheetConfig<T>) -> Measure<T> 
        where T: ImageSize + Clone {

        let notes = Note::random(config.clone(), 0, 0.0, sc);

        Measure {
            notes: notes,
            x,
            config: config,
        }
    }

    fn check(&mut self, note_char: char, note_x: f64) -> bool {
        let mut out = false;

        for note in self.notes.iter_mut() {
            out = out || note.check(note_char, note_x - self.x);
        }

        out
    }

    pub fn draw<G>(&self, c: Context, g: &mut G, glyphs: &mut GlyphCache<Factory, T>)
        where G: Graphics<Texture = T> {
        
        let c = c.trans(self.x, 0.0);
        let line = Line::new(color::BLACK, 2.0);

        let mut l = 0.0;
        while l < self.config.lines {
            let height = l * self.config.height / (self.config.lines - 1.0);
            let dim = [0.0, height, self.config.measure_size, height];
            line.draw(dim, &c.draw_state, c.transform, g);
            l += 1.0;
        };

        let line = Line::new(color::BLACK, 1.0);
        let dim = [0.0,0.0,0.0, self.config.height];
        line.draw(dim, &c.draw_state, c.transform, g);

        self.notes.iter().for_each(|n| n.draw(c, g, glyphs));
    }

    pub fn update(&mut self, dt: f64) {
        self.notes.iter_mut().for_each(|n| n.update());
        self.x -= dt*self.config.speed;
    }

    pub fn is_out_screen(&self) -> bool {
        self.x + self.config.measure_size < self.config.x
    }
}

struct Note<T> 
    where T: ImageSize {
    config: Config,
    x: f64,
    note: usize,
    size: u32,
    tex: T,
    img: Image,
    scale: f64,
    special: bool,
}

static NOTES: [char; 7] = ['E', 'F', 'G', 'A', 'B', 'C', 'D'];

impl<T> Note<T> 
    where T: ImageSize + Clone + CreateTexture<Factory> {
    fn random(config: Config, depth: u32, x: f64, sc: &SheetConfig<T>) -> Vec<Note<T>> {
        if depth == config.smallest_note || rand::thread_rng().gen_range(0,2) < 1 {
            let note = rand::thread_rng().gen_range(0,7);
            let height = config.height / config.lines;
            let width = 1.5*height;
            return vec![Note {
                x: x*(config.measure_size - width), note, size: depth, config, 
                special: false,
                tex: sc.textures[depth as usize].clone(), img: sc.images[depth as usize].clone(), scale: sc.scale
            }];
        } else {
            let mut one = Note::random(config.clone(), depth + 1, x, sc);
            let mut other = Note::random(config.clone(), depth + 1, x + (2.0 as f64).powi(-1 * depth as i32 -1), sc);
            one.append(&mut other);
            return one;
        }
    }

    fn draw<G>(&self, c: Context, g: &mut G, glyphs: &mut GlyphCache<Factory,T>) 
        where G: Graphics<Texture = T> {

        let height = self.config.height / (self.config.lines + 1.0);
        let width = self.tex.get_width() as f64;
        let y = (8 - self.note) as f64 / 8.0 * self.config.height + height/2.0;

        let trans = c.trans(self.x, y).transform;
        let scale = trans.scale(self.scale, self.scale);

        if self.special {
            Ellipse::new([0.0, 1.0, 0.0, 1.0])
                .draw([0.0, -3.0*height/2.0, 2.0*height,2.0* height], &c.draw_state, trans, g);
            // Ellipse::new([0.0, 1.0, 0.0, 1.0])
            //     .draw([0.0, -width/2.0  - height / (2.0 * self.scale), width, width], &c.draw_state, scale, g);
        }

        self.img.draw(&self.tex, &c.draw_state, scale, g);

        text::Text::new_color([1.0, 0.0, 0.0, 1.0], 48).draw(
            &NOTES[self.note].to_string(),
            glyphs,
            &c.draw_state,
            trans.trans(0.0, 40.0),
            g
        ).map_err(|_| ()).unwrap();
    }

    fn check(&mut self, note: char, note_x: f64) -> bool {
        let width = self.tex.get_width() as f64 * self.scale;

        println!("{}", width);

        if NOTES[self.note] == note.to_ascii_uppercase() && self.x < note_x && self.x + width > note_x {
            self.special = true;
            return true && !self.special;
        }
        false
    }

    fn update(&mut self) {
        
    }
}
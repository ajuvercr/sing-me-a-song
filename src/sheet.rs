

//use gfx_device_gl::{Factory, Resources};
use graphics::{Graphics, Context, ImageSize, Ellipse, color, Line, Rectangle};
use graphics::types::Color;

use piston_window::{Transformed, text};
use graphics::glyph_cache::rusttype::GlyphCache;
use gfx_device_gl::Factory;
use piston_window::texture::CreateTexture;


use rand;
use rand::Rng;

use config::Config;

pub struct Sheet {
    config: Config,
    sheet: Vec<Measure>,

}

impl Sheet {
    pub fn new(config: Config) -> Sheet {
        println!("new sheet: {:?}", config);
        let mut sheet = Vec::new();
        let mut x = 0.0;
        while x < config.width + config.measure_size {
            sheet.push(Measure::new(config.clone(), x));

            x += config.measure_size;
        }
        Sheet {
            sheet, config
        }
    }

    pub fn draw<G,T>(&self, c: Context, g: &mut G, color: Color, glyphs: &mut GlyphCache<Factory,T>) 
        where G: Graphics<Texture = T>, T: ImageSize + CreateTexture<Factory> {
            
            let rect = Rectangle::new(color);
            rect.draw(self.config.as_dims(), &c.draw_state, c.transform, g);

            let c = self.config.trans(&c);
            self.sheet.iter().for_each(|m| m.draw(c, g, glyphs));
    }

    pub fn update(&mut self, dt: f64) {
        self.sheet.iter_mut().for_each(|m| m.update(dt));
        let replace = self.sheet.first().unwrap().is_out_screen();
        if replace {
            let x = self.sheet.last().unwrap().x;
            let x = x + self.config.measure_size;
            self.sheet.remove(0);
            self.sheet.push(
                Measure::new(self.config.clone(), x)
            );
        }
    }
}

pub struct Measure {
    notes: Vec<Note>,
    pub x: f64,
    config: Config,
}

impl Measure {
    pub fn new(config: Config, x: f64) -> Measure {
        let notes = Note::random(config.clone(), 0, 0.0);

        Measure {
            notes: notes,
            x,
            config: config,
        }
    }

    pub fn draw<G,T>(&self, c: Context, g: &mut G, glyphs: &mut GlyphCache<Factory,T>)
        where G: Graphics<Texture = T>, T: ImageSize + CreateTexture<Factory> {
        
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

struct Note {
    config: Config,
    x: f64,
    note: usize,
    size: u32,
}

static NOTES: [char; 7] = ['E', 'F', 'G', 'A', 'B', 'C', 'D'];

impl Note {
    fn random(config: Config, depth: u32, x: f64) -> Vec<Note> {
        if depth == config.smallest_note || rand::thread_rng().gen_range(0,2) < 1 {
            let note = rand::thread_rng().gen_range(0,7);
            let height = config.height / config.lines;
            let width = 1.5*height;
            return vec![Note {
                x: x*(config.measure_size - width), note, size: depth, config, 
            }];
        } else {
            let mut one = Note::random(config.clone(), depth + 1, x);
            let mut other = Note::random(config.clone(), depth + 1, x + (2.0 as f64).powi(-1 * depth as i32 -1));
            one.append(&mut other);
            return one;
        }
    }

    fn draw<G,T>(&self, c: Context, g: &mut G, glyphs: &mut GlyphCache<Factory,T>) 
        where G: Graphics<Texture = T>, T: ImageSize + CreateTexture<Factory> {
        let height = self.config.height / self.config.lines;
        let width = 1.5*height;

        let y = (8 - self.note) as f64 / 8.0 * self.config.height;

        let dims = [self.x, y - height/2.0, width, height];

        if self.size == 0 {
            let note = Ellipse::new_border(color::BLACK, 2.0);
            note.draw(dims, &c.draw_state, c.transform, g);
        } else {
            let note = Ellipse::new(color::BLACK);
            note.draw(dims, &c.draw_state, c.transform, g);

            if self.size > 1 {
                let note = Line::new(color::BLACK, 2.0);
                let x = self.x + width;
                let dims = [x, y, x, y-2.0*height];
                note.draw(dims, &c.draw_state, c.transform, g);

                if self.size > 2 {
                    let note = Line::new(color::BLACK, 4.0);
                    let y = y - 2.0*height;
                    let dims = [x, y, x + 40.0, y + 40.0];

                    note.draw(dims, &c.draw_state, c.transform, g);
                }
            }
        }

        text::Text::new_color([1.0, 0.0, 0.0, 1.0], 48).draw(
            &NOTES[self.note].to_string(),
            glyphs,
            &c.draw_state,
            c.transform.trans(self.x + width - 64.0, y+height+10.0),
            g
        ).map_err(|_| ()).unwrap();
    }

    fn update(&mut self) {
        
    }
}
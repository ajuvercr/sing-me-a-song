
use graphics::{Graphics, Context, ImageSize, Ellipse, color, Line, Rectangle};
use graphics::types::Color;

use piston_window::Transformed;

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

    pub fn draw<G,T>(&self, c: Context, g: &mut G, color: Color) 
        where G: Graphics<Texture = T>, T: ImageSize {
            
            let rect = Rectangle::new(color);
            rect.draw(self.config.as_dims(), &c.draw_state, c.transform, g);

            let c = self.config.trans(&c);
            self.sheet.iter().for_each(|m| m.draw(c, g));
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
        let mut notes = Vec::new();
        notes.push(Note::random(config.clone()));
        notes.push(Note::random(config.clone()));
        notes.push(Note::random(config.clone()));
        notes.push(Note::random(config.clone()));

        Measure {
            notes: notes,
            x,
            config: config,
        }
    }

    pub fn draw<G,T>(&self, c: Context, g: &mut G)
        where G: Graphics<Texture = T>, T: ImageSize {
        
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

        self.notes.iter().for_each(|n| n.draw(c, g));
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
    note: u32,
}

static NOTES: [char; 7] = ['E', 'F', 'G', 'A', 'B', 'C', 'D'];

impl Note {
    fn random(config: Config) -> Note {
        let note = rand::thread_rng().gen_range(0,7);
        let x = rand::thread_rng().gen_range(0.0, config.measure_size);
        Note {
            x,
            config,
            note,
        }
    }

    fn draw<G,T>(&self, c: Context, g: &mut G) 
        where G: Graphics<Texture = T>, T: ImageSize{
        let height = self.config.height / self.config.lines;

        let y = (8 - self.note) as f64 / 8.0 * self.config.height;

        let dims = [self.x, y - height/2.0, 1.5 * height, height];
        let note = Ellipse::new(color::BLACK);
        note.draw(dims, &c.draw_state, c.transform, g);
    }

    fn update(&mut self) {
        
    }
}
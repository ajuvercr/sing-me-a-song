
use graphics::{Context};
use piston_window::Transformed;

#[derive(Clone, Debug)]
pub struct Config {
    pub width: f64,
    pub height: f64, 
    pub measure_size: f64,
    pub speed: f64,
    pub lines: f64,
    pub x: f64,
    pub y: f64,
    // pub smallest_note : u32,
}

impl Config {
    pub fn default() -> Config {
        Config {
            width: 1000.0,
            height: 1000.0,
            measure_size: 400.0,
            speed: 500.0,
            lines: 5.0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn small() -> Config {
        Config {
            width: 1000.0,
            height: 200.0,
            measure_size: 400.0,
            speed: 400.0,
            lines: 5.0,
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn split_vert(&self, perc: f64, padding: f64) -> (Config, Config) {
        let padding = padding*self.height;
        let t_height = self.height - padding;
        let (h1, h2) = (t_height * perc, t_height * (1.0 - perc));
        let (mut c1, mut c2) = (self.clone(), self.clone());
        c1.height = h1;
        c2.height = h2;
        c2.y = self.y + h1 + padding;
        (c1, c2)
    }

    pub fn split_hor(&self, perc: f64, padding: f64) -> (Config, Config) {
        let padding = padding*self.width;
        let t_width = self.width - padding;

        let (w1, w2) = (t_width * perc, t_width * (1.0 - perc));
        let (mut c1, mut c2) = (self.clone(), self.clone());
        c1.width = w1;
        c2.width = w2;
        c2.x = self.x + w1 + padding;
        (c1, c2)
    }

    pub fn trans(&self, c: &Context) -> Context {
        c.trans(self.x, self.y)
    }

    pub fn with_padding(&self, x_padding: f64, y_padding: f64) -> Config {
        let x_pad = x_padding * self.width;
        let y_pad = y_padding * self.height;
        let mut out = self.clone();
        out.x = out.x + x_pad;
        out.y = out.y + y_pad;
        out.width = out.width - 2.0*x_pad;
        out.height = out.height - 2.0*y_pad;
        return out;
    }

    pub fn as_dims(&self) -> [f64; 4] {
        [
            self.x,
            self.y,
            self.width,
            self.height
        ]
    }
}
use num_complex::Complex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Wasmbrot {
    width: usize,
    height: usize,
    dwell: u64,
    points: Vec<Point>,
    z_points: Vec<Complex<f64>>,
    c_points: Vec<Complex<f64>>,
    orbit_lasts: Vec<Complex<f64>>,
    orbit_dwell: u64,
    colors: Vec<u8>,
}

#[wasm_bindgen]
impl Wasmbrot {
    pub fn new(width: usize, height: usize) -> Wasmbrot {
        Wasmbrot {
            width,
            height,
            dwell: 0,
            points: vec![Point::Unknown(0); width * height],
            z_points: Vec::with_capacity(width * height),
            c_points: Vec::with_capacity(width * height),
            orbit_lasts: Vec::with_capacity(width * height),
            orbit_dwell: 8,
            colors: Vec::with_capacity(4 * width * height),
        }
    }

    pub fn param(&mut self, left: f64, top: f64, pixel_width: f64, pixel_height: f64) {
        self.points.clear();
        self.z_points.clear();
        self.c_points.clear();
        self.orbit_lasts.clear();
        self.colors.clear();

        self.dwell = 0;
        self.orbit_dwell = 8;

        for idx in 0..self.width * self.height {
            self.points.push(Point::Unknown(0));

            self.colors.push(0);
            self.colors.push(0);
            self.colors.push(0);
            self.colors.push(0xff);

            let row = idx / self.width;
            let col = idx % self.width;

            let x = left + col as f64 * pixel_width;
            let y = top - row as f64 * pixel_height;

            let z = Complex::new(x, y);
            self.z_points.push(z);
            self.orbit_lasts.push(z);
            self.c_points.push(z);
        }
    }

    pub fn step(&mut self, step_size: u64) -> StepResult {
        self.dwell += step_size;

        let mut all_known = true;
        let mut new_colors = false;

        for idx in 0..(self.width * self.height) {
            if let Point::Unknown(dwell) = &mut self.points[idx] {
                all_known = false;

                let c = &self.c_points[idx];
                let z = &mut self.z_points[idx];
                let last = &mut self.orbit_lasts[idx];

                for _ in 0..step_size {
                    let re_squared = z.re * z.re;
                    let im_squared = z.im * z.im;

                    if re_squared + im_squared > 4.0 {
                        self.points[idx] = Point::NotRendered(*dwell);
                        new_colors = true;
                        break;
                    }

                    *dwell += 1;

                    let re_im = z.re * z.im;
                    z.re = re_squared - im_squared + c.re;
                    z.im = 2.0 * re_im + c.im;

                    if z == last {
                        self.points[idx] = Point::InSet;
                        break;
                    }

                    if *dwell > self.orbit_dwell {
                        *last = *z;
                    }
                }
            }
        }

        if self.dwell > self.orbit_dwell {
            self.orbit_dwell *= 2;
        }

        StepResult {
            all_known,
            new_colors,
        }
    }

    pub fn colorize(&mut self, color_dist: f64) {
        for idx in 0..self.width * self.height {
            if let Point::NotRendered(dwell) = self.points[idx] {
                self.points[idx] = Point::Rendered; // we just rendered it!

                let dwell = dwell as f64 / color_dist;

                let r = dwell;
                let g = dwell + std::f64::consts::FRAC_PI_4;
                let b = dwell + std::f64::consts::FRAC_PI_2;

                let r = r.sin();
                let r = (r * r * 255.0) as u8;

                let g = g.sin();
                let g = (g * g * 255.0) as u8;

                let b = b.sin();
                let b = (b * b * 255.0) as u8;

                self.colors[4 * idx] = r;
                self.colors[4 * idx + 1] = g;
                self.colors[4 * idx + 2] = b;
            }
        }
    }

    pub fn dwell(&self) -> u64 {
        self.dwell
    }

    pub fn colors(&self) -> *const u8 {
        self.colors.as_ptr()
    }
}

#[wasm_bindgen]
pub struct StepResult {
    pub all_known: bool,
    pub new_colors: bool,
}

#[derive(Clone, PartialEq)]
enum Point {
    Unknown(u64),
    InSet,
    NotRendered(u64),
    Rendered,
}

use colored::Colorize;
use rand_distr::Distribution;

pub fn read_float<H: rustyline::Helper>(
    rl: &mut rustyline::Editor<H>,
    prompt: &str,
    min: f64,
    max: f64,
) -> Result<f64, rustyline::error::ReadlineError> {
    loop {
        match rl.readline(prompt)?.parse::<f64>() {
            Ok(f) => {
                if f < min {
                    eprintln!(
                        "  {}",
                        format!(
                            "value outside range: got {}, must be ≥ {}",
                            f.to_string().bold(),
                            min.to_string().bold()
                        )
                        .bright_red()
                    );
                    continue;
                }
                if f > max {
                    eprintln!(
                        "  {}",
                        format!(
                            "value outside range: got {}, must be ≤ {}",
                            f.to_string().bold(),
                            max.to_string().bold()
                        )
                        .bright_red()
                    );
                    continue;
                }
                return Ok(f);
            }
            Err(err) => eprintln!("  {}", err.to_string().bright_red()),
        }
    }
}

pub struct Perturber {
    absolute: rand_distr::Normal<f64>,
    relative: rand_distr::Normal<f64>,
}

impl Perturber {
    pub fn new(abs: f64, rel: f64) -> Self {
        use rand_distr::Normal;
        Perturber {
            absolute: Normal::new(0., abs).unwrap(),
            relative: Normal::new(0., rel).unwrap(),
        }
    }
    pub fn perturb(&self, r: &mut impl rand::Rng, f: f64) -> f64 {
        f * (1. + self.relative.sample(r)) + self.absolute.sample(r)
    }
}

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Vec2 {
    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.x * b.x + a.y * b.y
    }
    pub fn dot_with(&self, other: &Self) -> f64 {
        Self::dot(self, other)
    }
    pub fn cross(a: &Self, b: &Self) -> f64 {
        a.x * b.y - b.x * a.y
    }
    pub fn cross_with(&self, other: &Self) -> f64 {
        Self::cross(self, other)
    }
    pub fn from_polar(r: f64, θ: f64) -> Self {
        Self {
            x: r * θ.cos(),
            y: r * θ.sin(),
        }
    }
    pub fn rot90(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
    pub fn normal_from_angle(θ: f64) -> Self {
        Self::from_polar(1., θ)
    }
    pub fn norm2(&self) -> f64 {
        Self::dot(self, self)
    }
    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }
    pub fn normal(&self) -> Self {
        *self / self.norm()
    }
    pub fn project_onto(&self, axis: &Self) -> Self {
        *axis * (Self::dot(self, axis) / axis.norm2())
    }
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}
impl std::ops::Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, s: f64) -> Self {
        Self {
            x: self.x / s,
            y: self.y / s,
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

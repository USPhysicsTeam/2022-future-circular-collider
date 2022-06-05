#[derive(Clone, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum ParseVecError {
    WrongNumComponents(usize),
    MalformedX(String, std::num::ParseFloatError),
    MalformedY(String, std::num::ParseFloatError),
}

impl std::str::FromStr for Vec2 {
    type Err = ParseVecError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        match parts[..] {
            [sx, sy] => match (sx.trim().parse::<f64>(), sy.trim().parse::<f64>()) {
                (Err(e), _) => Err(ParseVecError::MalformedX(sx.to_string(), e)),
                (_, Err(e)) => Err(ParseVecError::MalformedY(sy.to_string(), e)),
                (Ok(x), Ok(y)) => Ok(Vec2 { x, y }),
            },
            _ => Err(ParseVecError::WrongNumComponents(parts.len())),
        }
    }
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::fmt::Display for ParseVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ParseVecError::*;
        match self {
            WrongNumComponents(1) => {
                write!(f, "missing a comma (example: `3.0,5.0`)")
            }
            WrongNumComponents(n) => {
                write!(f, "too many components given (expecting 2, got {})", n)
            }
            MalformedX(s, e) => {
                write!(f, "invalid x-component {:?}, {}", s, e)
            }
            MalformedY(s, e) => {
                write!(f, "invalid y-component {:?}, {}", s, e)
            }
        }
    }
}

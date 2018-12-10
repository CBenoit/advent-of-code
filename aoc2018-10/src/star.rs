use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Default, Debug, Clone)]
pub struct Star {
    pub x: i32,
    pub y: i32,
    pub dx: i32,
    pub dy: i32,
}

impl Star {
    pub fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Star { x, y, dx, dy }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn speed(&self) -> (i32, i32) {
        (self.dx, self.dy)
    }
}

impl FromStr for Star {
    type Err = Box<std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref POS_RE: Regex =
                Regex::new(r"position=< *(?P<x>-?\d+), *(?P<y>-?\d+)>").unwrap();
            static ref SPEED_RE: Regex =
                Regex::new(r"velocity=< *(?P<dx>-?\d+), *(?P<dy>-?\d+)>").unwrap();
        }

        let caps = POS_RE
            .captures(input)
            .ok_or_else(|| format!("no position match from input '{}'.", input))?;

        let x = caps["x"].parse::<i32>()?;
        let y = caps["y"].parse::<i32>()?;

        let caps = SPEED_RE
            .captures(input)
            .ok_or_else(|| format!("no position match from input '{}'.", input))?;

        let dx = caps["dx"].parse::<i32>()?;
        let dy = caps["dy"].parse::<i32>()?;

        Ok(Star { x, y, dx, dy })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut star = Star::new(0, 0, 1, -1);
        assert_eq!(star.pos(), (0, 0));
        assert_eq!(star.speed(), (1, -1));
        star.update();
        assert_eq!(star.pos(), (1, -1));
        star.update();
        assert_eq!(star.pos(), (2, -2));
    }

    #[test]
    fn parse() {
        let input = "position=<-42528,  42920> velocity=< 4, -4>";
        let star = input.parse::<Star>().unwrap();
        assert_eq!(star.pos(), (-42528, 42920));
        assert_eq!(star.speed(), (4, -4));

        let input = "velocity=< 4, -4>       position=<-42528,  42920>";
        let star = input.parse::<Star>().unwrap();
        assert_eq!(star.pos(), (-42528, 42920));
        assert_eq!(star.speed(), (4, -4));

        let input = "velotsraiety=< 4, -4> narste=<-42528,  42920>";
        assert!(input.parse::<Star>().is_err());
    }
}

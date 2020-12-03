use crate::star::Star;
use std::collections::HashMap;
use std::fmt;
use std::cmp::{min, max};

#[derive(Clone, Debug)]
struct Bounds {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Bounds {
    fn width(&self) -> u32 {
        (self.max_x - self.min_x) as u32 + 1
    }

    fn height(&self) -> u32 {
        (self.max_y - self.min_y) as u32 + 1
    }

    fn convert_x(&self, x: i32) -> u32 {
        (x - self.min_x) as u32
    }

    fn convert_y(&self, y: i32) -> u32 {
        (y - self.min_y) as u32
    }
}

#[derive(Clone, Debug)]
pub struct Sky {
    pub stars: Vec<Star>,
    pub instant: u32,
}

impl Sky {
    pub fn new(stars: Vec<Star>) -> Sky {
        Sky { stars, instant: 0 }
    }

    pub fn update(&mut self) {
        for star in &mut self.stars {
            star.update();
        }
        self.instant += 1;
    }

    pub fn count_features(&self) -> u32 {
        let mut map = HashMap::new();

        for star in &self.stars {
            map.insert(star.pos(), true);
        }

        let mut count = 0;
        for star in &self.stars {
            let pos = star.pos();
            let side_positions = [(pos.0 + 1, pos.1), (pos.0, pos.1 + 1)];

            for side_pos in side_positions.iter() {
                if let Some(_) = map.get(&side_pos) {
                    count += 1;
                }
            }
        }

        count
    }

    fn bounds(&self) -> Bounds {
        let mut b = Bounds {
            min_x: self.stars[0].x,
            min_y: self.stars[0].y,
            max_x: self.stars[0].x,
            max_y: self.stars[0].y,
        };

        for star in &self.stars {
            b.min_x = min(star.x, b.min_x);
            b.min_y = min(star.y, b.min_y);
            b.max_x = max(star.x, b.max_x);
            b.max_y = max(star.y, b.max_y);
        }

        b
    }
}

impl From<String> for Sky {
    fn from(input: String) -> Self {
        let stars: Vec<Star> = input
            .lines()
            .map(|l| l.parse().expect("failed to parse star info from input"))
            .collect();

        Sky::new(stars)
    }
}

impl fmt::Display for Sky {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let b = self.bounds();

        writeln!(f, "Instant: {}", self.instant)?;
        writeln!(f, "Size: {}x{}", b.width(), b.height())?;
        writeln!(f, "Features: {}", self.count_features())?;

        let mut grid = vec![vec![b'.'; b.width() as usize]; b.height() as usize];
        for star in &self.stars {
            let x = b.convert_x(star.x);
            let y = b.convert_y(star.y);
            grid[y as usize][x as usize] = b'#';
        }

        for row in grid {
            writeln!(f, "{}", std::str::from_utf8(&row).unwrap())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let input = "position=<-42528,  42920> velocity=< 4, -4>
                     position=<-31857, -53217> velocity=< 3,  5>
                     position=< 32290, -21174> velocity=<-3,  2>
                     position=<-21141, -21171> velocity=< 2,  2>";

        let mut sky = Sky::from(input.to_string());
        assert_eq!(sky.instant, 0);
        assert_eq!(sky.stars.len(), 4);
        assert_eq!(sky.stars[0].pos(), (-42528, 42920));

        sky.update();
        assert_eq!(sky.instant, 1);
        assert_eq!(sky.stars.len(), 4);
        assert_eq!(sky.stars[0].pos(), (-42524, 42916));
    }

    #[test]
    fn sky_features() {
        let mut stars = Vec::new();
        stars.push(Star::new(0, 0, 0, 0));
        stars.push(Star::new(1, 0, 0, 0));
        stars.push(Star::new(0, 1, 0, 0));
        stars.push(Star::new(0, 2, 0, 0));
        stars.push(Star::new(5, 1, 0, 0));

        let sky = Sky::new(stars);
        assert_eq!(sky.count_features(), 3);
    }

    #[test]
    fn bounds() {
        let mut stars = Vec::new();
        stars.push(Star::new(0, -3, 0, 0));
        stars.push(Star::new(1, 0, 0, 0));
        stars.push(Star::new(0, 1, 0, 0));
        stars.push(Star::new(-2, 2, 0, 0));
        stars.push(Star::new(5, 1, 0, 0));

        let sky = Sky::new(stars);
        let b = sky.bounds();
        assert_eq!(b.min_x, -2);
        assert_eq!(b.min_y, -3);
        assert_eq!(b.max_x, 5);
        assert_eq!(b.max_y, 2);

        assert_eq!(b.width(), 8);
        assert_eq!(b.height(), 6);
    }
}

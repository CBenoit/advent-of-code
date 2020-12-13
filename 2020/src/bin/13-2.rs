//! https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Statement

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::convert::TryFrom;

    let input = std::fs::read_to_string("inputs/13")?;

    let buses: Vec<(_, _)> = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(offset, id)| {
            let id = id.parse::<i128>().ok()?;
            let offset = i128::try_from(offset).unwrap() % id;
            Some((id, offset))
        })
        .collect();

    let mut iter = buses.into_iter();

    let init = iter.next().unwrap();

    let (ni, x) = iter
        .fold(
            init,
            |(n1, a1), (n2, a2)| {
                let (m1, m2) = bezout(n1, n2);
                let x = (a1 * m2 * n2) + (a2 * m1 * n1);
                let n3 = n1 * n2;
                let m3 = x % n3;
                (n3, m3)
            }
        );

    println!("ni = {}, abs(x) = {}", ni, x.abs());

    Ok(())
}

/// Extended Euclidian division result gives us
/// a pair of Bézout coefficients
fn bezout(a: i128, b: i128) -> (i128, i128) {
    if b > a {
        let (y, x) = bezout(b, a);
        return (x, y);
    }

    let mut prev_r = a; // r0
    let mut r = b; // r1
    let mut prev_s = 1; // s0
    let mut s = 0; // s1
    let mut prev_t = 0; // t0
    let mut t = 1; // s1

    while r != 0 {
        let q = prev_r / r;
        let next_r = prev_r - q * r;
        let next_s = prev_s - q * s;
        let next_t = prev_t - q * t;

        prev_r = r;
        r = next_r;
        prev_s = s;
        s = next_s;
        prev_t = t;
        t = next_t;
    }

    (prev_s, prev_t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bezout_coefs() {
        let (a, b) = (240, 46);
        let pgcd = 2;
        let (x, y) = bezout(a, b);
        assert_eq!(x, -9);
        assert_eq!(y, 47);
        assert_eq!(a * x + b * y, pgcd);

        // preserve order
        let (y, x) = bezout(b, a);
        assert_eq!(x, -9);
        assert_eq!(y, 47);
    }
}

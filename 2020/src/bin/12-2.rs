type Dir = (i64, i64);
type Rot = ((i64, i64), (i64, i64));
type Dist = i64;

#[derive(Debug)]
enum Action {
    Turn(Rot),
    Forward(Dist),
    Move(Dir),
}

fn cos(angle: i64) -> i64 {
    match angle % 360 {
        0 => 1,
        90 | -270 => 0,
        180 | -180 => -1,
        270 | -90 => 0,
        _ => panic!("invalid angle: {}", angle),
    }
}

fn sin(angle: i64) -> i64 {
    match angle % 360 {
        0 => 0,
        90 | -270 => 1,
        180 | -180 => 0,
        270 | -90 => -1,
        _ => panic!("invalid angle: {}", angle),
    }
}

fn parse_actions(line: &str) -> Action {
    let val = line[1..].parse().unwrap();
    match &line[..1] {
        "N" => Action::Move((0, val)),
        "S" => Action::Move((0, -val)),
        "E" => Action::Move((val, 0)),
        "W" => Action::Move((-val, 0)),
        "L" => Action::Turn(((cos(-val), sin(-val)), (-sin(-val), cos(-val)))),
        "R" => Action::Turn(((cos(val), sin(val)), (-sin(val), cos(val)))),
        "F" => Action::Forward(val),
        invalid => panic!("invalid instruction: {}", invalid),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("inputs/12")?;
    let instructions: Vec<Action> = input.lines().map(parse_actions).collect();

    let mut waypoint = (10, 1); // waypoint is always relative to ship
    let mut ship = (0, 0);

    for action in instructions {
        match action {
            Action::Turn(rot) => {
                let (x, y) = (waypoint.0, waypoint.1);
                waypoint.0 = (rot.0).0 * x + (rot.0).1 * y;
                waypoint.1 = (rot.1).0 * x + (rot.1).1 * y;
            }
            Action::Forward(dist) => {
                let (dx, dy) = (waypoint.0 * dist, waypoint.1 * dist);
                ship.0 += dx;
                ship.1 += dy;
            }
            Action::Move(dir) => {
                waypoint.0 += dir.0;
                waypoint.1 += dir.1;
            }
        }
    }

    let man_dist = ship.0.abs() + ship.1.abs();

    println!("{}", man_dist);

    Ok(())
}


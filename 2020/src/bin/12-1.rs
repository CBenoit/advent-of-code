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

    let mut dir = (1, 0);
    let mut pos = (0, 0);

    for action in instructions {
        match action {
            Action::Turn(rot) => {
                let (x, y) = (dir.0, dir.1);
                dir.0 = (rot.0).0 * x + (rot.0).1 * y;
                dir.1 = (rot.1).0 * x + (rot.1).1 * y;
            }
            Action::Forward(dist) => {
                pos.0 += dir.0 * dist;
                pos.1 += dir.1 * dist;
            }
            Action::Move(dir) => {
                pos.0 += dir.0;
                pos.1 += dir.1;
            }
        }
    }

    let man_dist = pos.0.abs() + pos.1.abs();

    println!("{}", man_dist);

    Ok(())
}


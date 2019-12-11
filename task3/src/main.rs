use std::collections::HashMap;
use std::io::stdin;

#[derive(Debug, Hash, Copy, Eq, PartialEq, PartialOrd, Clone)]
struct Position(isize, isize);

impl Position {
    fn distance_from_origin(&self) -> isize {
        self.0.abs() + self.1.abs()
    }
}

enum Move {
    Up(usize),
    Right(usize),
    Down(usize),
    Left(usize),
}

impl Move {
    fn from_str(input: &str) -> Move {
        let steps: usize = input[1..input.len()].parse().unwrap();
        match &input[0..1] {
            "U" => Move::Up(steps),
            "R" => Move::Right(steps),
            "D" => Move::Down(steps),
            "L" => Move::Left(steps),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let stdin = stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let positions1 = position_set(&line);
    line.clear();
    stdin.read_line(&mut line).unwrap();
    let positions2 = position_set(&line);
    let mut closest_intersection_distance = std::isize::MAX;
    for position in positions1.keys() {
        if positions2.contains_key(&position) && positions1[position] + positions2[position] < closest_intersection_distance {
            closest_intersection_distance = positions1[position] + positions2[position];
        }
    }
    println!("{}", closest_intersection_distance);
}

fn position_set(line: &String) -> HashMap<Position, isize> {
    let mut positions = HashMap::new();
    let mut current_pos = Position(0, 0);
    let mut current_distance = 1;
    for processed_move in line.trim().split(",").map(Move::from_str) {
        let new_positions = process_move(processed_move, current_pos);
        current_pos = new_positions[new_positions.len() - 1];
        for pos in new_positions {
            positions.insert(pos, current_distance);
            current_distance += 1;
        }
    }
    positions.remove(&Position(0, 0));
    positions
}

fn process_move(processed_move: Move, current_pos: Position) -> Vec<Position> {
    let (change, steps): ((isize, isize), isize) = match processed_move {
        Move::Up(steps) => ((1, 0), steps as isize),
        Move::Right(steps) => ((0, 1), steps as isize),
        Move::Down(steps) => ((-1, 0), steps as isize),
        Move::Left(steps) => ((0, -1), steps as isize),
    };
    (1..steps + 1)
        .into_iter()
        .map(|step| {
            Position(
                current_pos.0 + change.0 * step,
                current_pos.1 + change.1 * step,
            )
        })
        .collect()
}

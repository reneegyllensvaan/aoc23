use std::collections::{HashMap, HashSet};

use utils::{Dir, Grid, Pos, PosUtils, SGrid, Vec2dUtils};

static INPUT_FILE: &str = "input/day16";
#[allow(dead_code)]
static EXAMPLE_INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

pub fn part1(input: &str) -> i64 {
    let mut grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();

    let mut seen = HashSet::new();
    traverse(&mut grid, (0, 0), Dir::Right, &mut seen);
    let seen = seen.into_iter().map(|v| v.0).collect::<HashSet<_>>();

    if cfg!(test) {
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if seen.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    seen.len() as i64
}

pub fn part2(input: &str) -> i64 {
    let grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();

    let traverse_from = |pos: Pos, dir: Dir| {
        let mut seen = HashSet::new();
        traverse(&mut grid.clone(), pos, dir, &mut seen);
        seen.into_iter().map(|v| v.0).collect::<HashSet<_>>().len() as i64
    };

    let h = grid.len();
    let w = grid[0].len();
    let mut result = 0;
    for row in 0..h {
        let a = traverse_from((row, 0), Dir::Right);
        let b = traverse_from((row, w - 1), Dir::Left);
        result = result.max(a).max(b);
    }
    for col in 0..w {
        let a = traverse_from((0, col), Dir::Down);
        let b = traverse_from((w - 1, col), Dir::Up);
        result = result.max(a).max(b);
    }

    result
}

pub fn part2_opt(input: &str) -> i64 {
    let grid: Grid<char> = input.lines().map(|l| l.chars().collect()).collect();

    let mut outgoing_scores = HashMap::<(Pos, Dir), i64>::new();
    let mut traverse_from = |pos: Pos, dir: Dir| {
        if let Some(v) = outgoing_scores.get(&(pos, dir.opposite())) {
            return *v;
        }
        let mut seen = HashSet::new();
        let mut exits = HashSet::new();
        traverse_track_exits(&mut grid.clone(), pos, dir, &mut seen, &mut exits);
        let out = seen.into_iter().map(|v| v.0).collect::<HashSet<_>>().len() as i64;
        for exit in exits {
            outgoing_scores.insert(exit, out);
        }
        out
    };

    let h = grid.len();
    let w = grid[0].len();
    let mut result = 0;
    for row in 0..h {
        let a = traverse_from((row, 0), Dir::Right);
        let b = traverse_from((row, w - 1), Dir::Left);
        result = result.max(a).max(b);
    }
    for col in 0..w {
        let a = traverse_from((0, col), Dir::Down);
        let b = traverse_from((w - 1, col), Dir::Up);
        result = result.max(a).max(b);
    }

    result
}

fn traverse(grid: &mut SGrid<char>, pos: Pos, dir: Dir, seen: &mut HashSet<(Pos, Dir)>) {
    if !seen.insert((pos, dir)) {
        return;
    }
    let tile = grid.at(pos);
    match tile {
        '|' if dir.horizontal() => {
            if let Some(next) = pos.go_in(Dir::Up, grid) {
                traverse(grid, next, Dir::Up, seen);
            }
            if let Some(next) = pos.go_in(Dir::Down, grid) {
                traverse(grid, next, Dir::Down, seen);
            }
        }
        '-' if dir.vertical() => {
            if let Some(next) = pos.go_in(Dir::Left, grid) {
                traverse(grid, next, Dir::Left, seen);
            }
            if let Some(next) = pos.go_in(Dir::Right, grid) {
                traverse(grid, next, Dir::Right, seen);
            }
        }
        '.' | '|' | '-' => {
            if let Some(next) = pos.go_in(dir, grid) {
                traverse(grid, next, dir, seen);
            }
        }
        '/' | '\\' => {
            let mut new_dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
            };
            if tile == '\\' {
                new_dir = new_dir.opposite();
            }
            if let Some(next) = pos.go_in(new_dir, grid) {
                traverse(grid, next, new_dir, seen);
            }
        }
        _ => panic!("unknown tile: {tile}"),
    }
}
fn traverse_track_exits(
    grid: &mut SGrid<char>,
    pos: Pos,
    dir: Dir,
    seen: &mut HashSet<(Pos, Dir)>,
    exits: &mut HashSet<(Pos, Dir)>,
) {
    if !seen.insert((pos, dir)) {
        return;
    }
    let tile = grid.at(pos);
    match tile {
        '|' if dir.horizontal() => {
            if let Some(next) = pos.go_in(Dir::Up, grid) {
                traverse_track_exits(grid, next, Dir::Up, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
            if let Some(next) = pos.go_in(Dir::Down, grid) {
                traverse_track_exits(grid, next, Dir::Down, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
        }
        '-' if dir.vertical() => {
            if let Some(next) = pos.go_in(Dir::Left, grid) {
                traverse_track_exits(grid, next, Dir::Left, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
            if let Some(next) = pos.go_in(Dir::Right, grid) {
                traverse_track_exits(grid, next, Dir::Right, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
        }
        '.' | '|' | '-' => {
            if let Some(next) = pos.go_in(dir, grid) {
                traverse_track_exits(grid, next, dir, seen, exits);
            } else {
                exits.insert((pos, dir));
            }
        }
        '/' | '\\' => {
            let mut new_dir = match dir {
                Dir::Up => Dir::Right,
                Dir::Left => Dir::Down,
                Dir::Down => Dir::Left,
                Dir::Right => Dir::Up,
            };
            if tile == '\\' {
                new_dir = new_dir.opposite();
            }
            if let Some(next) = pos.go_in(new_dir, grid) {
                traverse_track_exits(grid, next, new_dir, seen, exits);
            } else {
                exits.insert((pos, new_dir));
            }
        }
        _ => panic!("unknown tile: {tile}"),
    }
}

pub fn main(bench: bool) {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();

    let iters = 10;

    let fns: Vec<(&'static str, fn(&str) -> i64)> = vec![
        ("part1", part1),
        ("part2", part2),
        ("part2 (opt)", part2_opt),
    ];

    for (name, f) in &fns {
        println!("  {name}: {}", f(&input));
    }
    println!("");
    if bench {
        for (name, f) in &fns {
            let begin = std::time::Instant::now();
            for _ in 0..iters {
                f(&input);
            }
            let end = std::time::Instant::now();
            println!(
                "  {} {} in: {}us ({}us/iter)",
                iters,
                name,
                (end - begin).as_micros(),
                (end - begin).as_micros() / iters
            );
        }
    }
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(EXAMPLE_INPUT), 46);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(EXAMPLE_INPUT), 51);
    assert_eq!(part2_opt(EXAMPLE_INPUT), 51);
}

#[test]
fn test_part1_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part1(&input), 7034);
}

#[test]
fn test_part2_facit() {
    let input = std::fs::read_to_string(INPUT_FILE).unwrap();
    assert_eq!(part2(&input), 7759);
}

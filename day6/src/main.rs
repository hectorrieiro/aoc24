use std::{collections::HashSet, fs};

type Vector = [i32; 2];

struct Player {
    position: Vector,
    direction: Vector,
    visited_locations: HashSet<Vector>,
    positions: Vec<Vector>,
    directions: Vec<Vector>,
    game: Game,
}
#[derive(Clone)]
struct Game {
    width: i32,
    height: i32,
    obstacles: HashSet<Vector>,
}

impl Game {
    fn is_position_outside(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x >= self.width || y >= self.height
    }

    fn is_position_obstacle(&self, x: i32, y: i32) -> bool {
        let v = [x, y];
        self.obstacles.contains(&v)
    }
}

const CARET: u8 = '^'.to_ascii_lowercase() as u8;
const SHARP: u8 = '#'.to_ascii_lowercase() as u8;

impl Player {
    fn try_to_move(&mut self) -> Option<Vector> {
        let intended_position = [
            self.position[0] + self.direction[0],
            self.position[1] + self.direction[1],
        ];

        if self
            .game
            .is_position_obstacle(intended_position[0], intended_position[1])
        {
            return None;
        }
        self.position = intended_position;
        if !(self.is_outside()) {
            self.visited_locations.insert(self.position);
            self.positions.push(self.position);
            self.directions.push(self.direction);
        }

        Some(self.position)
    }

    fn is_outside(&self) -> bool {
        self.game
            .is_position_outside(self.position[0], self.position[1])
    }

    fn turn(&mut self) {
        self.direction = [-self.direction[1], self.direction[0]];
    }
}

fn load_from_file(filename: &str) -> Option<Player> {
    let data = fs::read_to_string(filename).unwrap();

    let mut p0 = [-1, -1];
    let mut obs = HashSet::<Vector>::new();
    let mut set = HashSet::<Vector>::new();
    let mut w = 0;
    let mut h = 0;
    for (i, line) in data.split_whitespace().enumerate() {
        if w == 0 {
            w = line.len() as i32;
        } else {
            if w != line.len() as i32 {
                return None;
            }
        }
        for (j, b) in line.bytes().enumerate() {
            match b {
                SHARP => _ = obs.insert([j as i32, i as i32]),
                CARET => p0 = [j as i32, i as i32],
                _ => continue,
            }
        }
        h += 1;
    }
    if p0 == [-1, -1] {
        return None;
    }
    set.insert(p0);

    let g = Game {
        height: h,
        width: w,
        obstacles: obs,
    };
    let p = Player {
        position: p0,
        direction: [0, -1],
        visited_locations: set,
        positions: vec![p0],
        directions: vec![[0, -1]],
        game: g,
    };
    Some(p)
}

fn gets_into_loop(p: &Player) -> bool {
    let self_iter = p.positions.iter();
    let mut idx = 0;
    if let Some(steps) = self_iter
        .zip(p.directions.iter())
        .position(|(&x, &v)| x == p.position && p.direction == v)
    {
        idx += steps;
        if idx == p.positions.len() - 1 {
            return false;
        }
        //println!("Found loop of length {:?}", p.positions.len()-1-idx);
        return true;
    }
    false
}

fn main() {
    let mut p = load_from_file("input.txt").unwrap();
    let initial_position = p.position;

    /////Part 1
    while !p.is_outside() {
        match p.try_to_move() {
            Some(_pos) => continue,
            None => p.turn(),
        }
    }
    println! {"Positions visited: {:?}", p.visited_locations.len()};


    ////Part 2
    let mut candidates = HashSet::<Vector>::new();
    let mut memory_pos = Vec::<Vector>::new();
    let mut memory_dir = Vec::<Vector>::new();
   

    for (&pos, &vel) in p.positions.iter().zip(p.directions.iter()) {
        memory_pos.push(pos);
        memory_dir.push(vel);
       
       let obstacle_pos = [pos[0] + vel[0], pos[1] + vel[1]];
        if memory_pos.contains(&obstacle_pos) || (obstacle_pos == initial_position) {
            continue;
        }
        let mut new_game = p.game.clone();
        if new_game.is_position_outside(obstacle_pos[0], obstacle_pos[1]) {
            continue;
        }        
        if !new_game.obstacles.insert(obstacle_pos) {
            continue;
        }

        let mut player2 = Player {
            position: pos,
            direction: vel,
            visited_locations: HashSet::<Vector>::new(), //IRRELEVANT NOW
            positions: memory_pos.clone(),
            directions: memory_dir.clone(),
            game: new_game,
        };

        while !player2.is_outside() {
            
            if gets_into_loop(&player2) {
                candidates.insert(obstacle_pos);
                break;
            }
            match player2.try_to_move() {
                Some(_pos) => continue,
                None => player2.turn(),
            }
        }
        
    }
    println!("{:?} candidates for obstacle.", candidates.len());
}

use std::{collections::HashSet, fs};

#[derive(Debug)]
struct Grid {
    points:Vec<u8>,
    width:usize,
    height:usize
}

fn subtract_u8(a:u8, b:u8) -> Option<u8> {
    if b>a {
        None
    } else {
        Some(a-b)
    }
}

impl Grid {
    fn from_lines_iterator<'a>(it:&'a mut impl Iterator<Item= &'a str>) -> Option<Self>{
        let mut grid = Self {
            points:Vec::<u8>::new(),
            width:0,
            height:0,
        };

        for l in it{
            grid.height +=1;
            let mut bytes:Vec<u8> = l.as_bytes().iter().map(|&c| (c as char).to_digit(10).unwrap() as u8).collect();
            if grid.width>0 {
                if bytes.len() != grid.width {return None;}
            } else {
                grid.width = bytes.len();
            }

            grid.points.append(&mut bytes);
        }

        Some(grid)
    }

    fn index_to_coord(&self, idx:&usize) -> [usize;2] {
        [idx%self.width, idx/self.height]
    }
    fn coord_to_index(&self, coord:&[usize; 2]) -> usize {
        coord[0] + coord[1]*self.width
    }
    fn get_starting_points(&self) -> Vec<[usize;2]> {
        let mut points = Vec::<[usize;2]>::new();
        self.points.iter().enumerate().for_each (|(i, &v)| {
            if v==0 {
                points.push(self.index_to_coord(&i))
            }
        });
        points
    }

    fn possible_directions(&self, coord:&[usize;2]) -> Vec<[i32;2]> {
        let mut dirs = Vec::<[i32;2]>::new();
        let idx = self.coord_to_index(&coord);
        if (coord[0] > 0) && (subtract_u8(self.points[idx-1],self.points[idx]) == Some(1)) {
            dirs.push([-1,0]);
        }
        if (coord[0] < self.width-1) && (subtract_u8(self.points[idx+1], self.points[idx])==Some(1)) {
            dirs.push([1,0]);
        }
        if (coord[1]>0) && (subtract_u8(self.points[idx-self.width],self.points[idx]) == Some(1)) {
            dirs.push([0,-1]);
        }
        if (coord[1]<self.height-1) && (subtract_u8(self.points[idx+self.width],self.points[idx])==Some(1)) {
            dirs.push([0,1]);
        }
        dirs
    }

    fn trail_walk(&self, p:&[usize;2], found_peaks:&mut HashSet<[usize;2]>, distinct_peaks:bool) -> Option<usize> {
        let mut score = 0usize;
        if self.points[self.coord_to_index(p)]==9 {
            if found_peaks.insert(p.clone()) || !distinct_peaks {
                println!("Found {:?}", p);
                return Some(1);
            } else {
                return None;
            }
            
        }
        let dirs = self.possible_directions(p);
        if dirs.len()==0 {return None;};
        
        for &d in dirs.iter() {
            let next_point = [(p[0] as i32+d[0]) as usize, (p[1] as i32+d[1]) as usize];
            if let Some(branch_score) = self.trail_walk(&next_point, found_peaks, distinct_peaks) {
                score += branch_score;
            }
        }
        
        Some(score)

    }
}

fn main() {
    
    let data = fs::read_to_string("input.txt").unwrap();
    let mut lines_it  = data.split_ascii_whitespace();
    let grid = Grid::from_lines_iterator(&mut lines_it).unwrap();
    let starting_points = grid.get_starting_points();
    let mut total_score = 0usize;
    for p in starting_points.iter() {
        println!("Starting at {:?}",p);
        let mut found_peaks = HashSet::<[usize;2]>::new();
        if let Some(head_score) = grid.trail_walk(&p, &mut found_peaks, false) {
            total_score += head_score;
        }
    }
    println!("Total score: {:?}", total_score);
}

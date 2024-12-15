use std::{collections::{HashMap, HashSet}, fs};

type AntennasMap = HashMap<u8, Vec<[usize;2]>>; 
type AntinodeMap = HashSet<[usize;2]>;

fn parse_file(data:&str, mapping:&mut AntennasMap) -> [usize;2] {
    let line_it = data.split_whitespace();
    let mut map_size = [0,0];
    for (j,line) in line_it.enumerate() {
        map_size[1] = map_size[1] + 1;
        map_size[0] = line.len();
        for (i,&c) in line.as_bytes().iter().enumerate() {
            if c.is_ascii_alphanumeric() {
                if let Some(v) = mapping.get_mut(&c) {
                    v.push([i,j]);
                } else {
                    mapping.insert(c, vec![[i,j]]);
                }
            }
        }
    }
    map_size
}

fn gcd(a:i32, b:i32) -> i32 {
    if a==b {
        return a;
    }
    let mut x;
    let mut y;

    if a>b {
        x = a;
        y = b;
    } else {
        x = b;
        y = a;
    }
    let mut q= x /y;
    let mut r:i32 = x % y;
    while r!=0 {
        x = y;
        y = r;
        q = x/y;
        r = x%y;
    }

    y.abs()
}

fn is_in_bounds(point:&[i32;2], map_size:&[usize;2]) -> bool {
    point[0] >= 0 && point[1] >=0 && point[0] < map_size[0] as i32  && point[1] < map_size[1] as i32 
}

fn find_nodes(antennas:&[[usize;2]], output_set:&mut AntinodeMap, map_size:&[usize;2]) {
    if antennas.len() <=1 {
        return;
    }
    let current_antenna = antennas[0];
    for pair in antennas[1..].iter() {
        let dist = [pair[0] as i32-current_antenna[0] as i32, pair[1] as i32-current_antenna[1] as i32];
        let candidate1 = [current_antenna[0] as i32 -dist[0] as i32 , current_antenna[1] as i32 -dist[1] as i32];
        let candidate2 = [pair[0] as i32 +dist[0] as i32,  pair[1] as i32+dist[1] as i32];
        if is_in_bounds(&candidate1, map_size) {
            output_set.insert([candidate1[0] as usize, candidate1[1] as usize]);
        };
        if is_in_bounds(&candidate2, map_size) {
            output_set.insert([candidate2[0] as usize, candidate2[1] as usize]);
        };
    };

    find_nodes(&antennas[1..], output_set, map_size);

}

fn find_resonant_antinodes(antennas:&[[usize;2]], output_set:&mut AntinodeMap, map_size:&[usize;2]) {
    if antennas.len() <=1 {
        return;
    }
    let current_antenna = antennas[0];
    for pair in antennas[1..].iter() {
        let dist = [pair[0] as i32-current_antenna[0] as i32, pair[1] as i32-current_antenna[1] as i32];
        let d = gcd(dist[0], dist[1]);
        let dist = [dist[0]/d, dist[1]/d];
        let mut point = [current_antenna[0] as i32, current_antenna[1] as i32];
        let mut k = 0i32;
        while is_in_bounds(&point, map_size) {
            output_set.insert([point[0] as usize, point[1] as usize]);
            k = k-1;
            point = [point[0]-dist[0], point[1]-dist[1]];
        }
        k = 0;
        point = [current_antenna[0] as i32, current_antenna[1] as i32];
        while is_in_bounds(&point, map_size) {
            output_set.insert([point[0] as usize, point[1] as usize]);
            k += 1;
            point = [point[0]+dist[0], point[1]+dist[1]];
        }
    };

    find_resonant_antinodes(&antennas[1..], output_set, map_size);

}

fn find_antinodes(antennas_map:&AntennasMap, antinodes:&mut AntinodeMap, map_size:&[usize;2]) {
    for (_freq, antennas) in antennas_map.iter() {
        if antennas.len() < 2 {
            continue;
        }
        find_nodes(&antennas.as_slice(), antinodes, map_size)
    }
}

fn find_antinodes_resonant(antennas_map:&AntennasMap, antinodes:&mut AntinodeMap, map_size:&[usize;2]) {
    for (_freq, antennas) in antennas_map.iter() {
        if antennas.len() < 2 {
            continue;
        }
        find_resonant_antinodes(&antennas.as_slice(), antinodes, map_size)
    }
}

fn print_map(map_size:&[usize;2], antennas:&AntennasMap, antinodes:&AntinodeMap) {
    
    let mut map = Vec::<Vec<u8>>::new();
    map.resize(map_size[0], Vec::<u8>::new());
    for j in 0..map_size[0] {
       
        map[j].resize(map_size[1], b'.');
        for i in 0..map_size[1] {
            
            if antinodes.contains(&[i,j]) {
                map[j][i] = b'#';
            }
            for (freq, points) in antennas {
                if points.contains(&[i,j]) {
                    map[j][i] = *freq;
                }
            }
            print!("{}", map[j][i] as char); 

        }
        println!{""};

    }

}

fn main() {
    let mut antinodes = HashSet::<[usize;2]>::new();
    let mut frequency_antennas_map = HashMap::<u8, Vec<[usize;2]>>::new();
    let filecontents = fs::read_to_string("input.txt").unwrap();
    let map_size = parse_file(&filecontents, &mut frequency_antennas_map);
    find_antinodes(&frequency_antennas_map, &mut antinodes, &map_size);
    println!("Found {:?} antinodes", antinodes.len());

    //part 2
    antinodes.clear();
    find_antinodes_resonant(&frequency_antennas_map, &mut antinodes, &map_size);
    println!("Found {:?} resonant antinodes.", antinodes.len());
    //println!("{:?}", antinodes);
    print_map(&map_size, &frequency_antennas_map, &antinodes);
    

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(3,6), 3);
        assert_eq!(gcd(6,3), 3);
        assert_eq!(gcd(5,5), 5);
        assert_eq!(gcd(-5,5), 5);
        
    }

   
}

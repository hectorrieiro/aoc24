use std::{fs, iter::zip};

fn count_appearances(x:i32, v:&Vec<i32>) ->i32 {
    v.iter().map(|n| if x==*n {1} else {0}).sum()
}

fn main() {
    let file = "part1.txt";
    println!("In file {file}");

    let contents = fs::read_to_string(file)
        .expect("Should have been able to read the file");
    let v:Vec<&str> = contents.split_terminator("\n").collect();
    let mut x:Vec<i32> = Vec::with_capacity(v.len());
    let mut y:Vec<i32> = Vec::with_capacity(v.len());
    for line in v {
        let mut iter = line.split_ascii_whitespace();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        x.push(a.parse().unwrap());
        y.push(b.parse().unwrap()); 
    }
    //println!("First column: {:?}", x);
    //println!("Second column: {:?}", y);
    x.sort();
    y.sort();
    //println!("First column sorted: {:?}", x);
    //println!("Second column sorted: {:?}", y);
    let zipper = zip(x.clone(),y.clone());
    let dist:i32 = zipper.map(|(x,y)| (x-y).abs()).sum();
    println!("Part 1 solution: {:?}", dist);
    let dist2:i32 = x.iter().map(|n| n*count_appearances(*n, &y)).sum();
    println!("Part 2 solution: {:?}", dist2);
    
}

use std::fs;
use core::iter::Iterator;
use std::collections::HashSet;

fn concat_digits(a:u64, b:u64) -> u64 {
    str::parse::<u64>((a.to_string() + b.to_string().as_str()).as_str()).unwrap()
}

fn inner_graph_walk<'a>(acc:u64, v:&[u64], set:&mut HashSet<u64> )  {
    if v.len()>0 {
        let next_element = v[0];
        inner_graph_walk(acc+next_element, &v[1..], set);
        inner_graph_walk(acc*next_element, &v[1..], set);
        inner_graph_walk(concat_digits(acc, next_element), &v[1..], set);
            
    } else {
        set.insert(acc);
    }
    
}

fn graph_walk(total:u64, numbers:&Vec<u64>) -> bool {
    let mut set = HashSet::<u64>::new();
    inner_graph_walk(0, numbers.as_slice(), &mut set);
    set.contains(&total)

    
}

fn process_line(line:&str) -> Option<u64> {
    //println!("{:?}", line);
    let mut line_iter = line.split(':');
    let total = line_iter.next().unwrap().parse::<u64>().unwrap();
    let number_tokens = line_iter.next().unwrap().split_whitespace();
    let numbers:Vec<u64> = number_tokens.map(|x| x.parse::<u64>().unwrap()).collect();
    //println!("{:?}", numbers);
    if graph_walk(total, &numbers) {
        return Some(total)
    }
    None
}

fn main() {
    let data_string = fs::read_to_string("input.txt").unwrap();
    let mut acc:u64 = 0;
    data_string.split('\n').for_each(|l| if let Some(sum) = process_line(l) {acc+=sum;});
    println!("Total: {:?}", acc);


}

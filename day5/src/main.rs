use std::cmp::Ordering;
use std::fs;
use std::sync::OnceLock;

//static mut SORTING_RULES:Box<Vec<(usize,usize)>> = Box{vec![]};
static SORTING_RULES: OnceLock<Vec<(usize, usize)>> = OnceLock::<Vec<(usize, usize)>>::new();

fn sorting(a: &usize, b: &usize) -> Ordering {
    let sorting_rules = SORTING_RULES.get().unwrap();
    for rule in sorting_rules.iter() {
        if rule.0 == *a && rule.1 == *b {
            return Ordering::Less;
        } else if rule.1 == *a && rule.0 == *b {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

fn main() {
    let filename = "input.txt";
    let file_data = fs::read_to_string(filename).unwrap();
    let mut line_it = file_data.lines();
    let mut rules: Vec<(usize, usize)> = Vec::new();
    //let mut rules_sect_after:Vec<u32> = Vec::new();

    while let Some(line) = line_it.next() {
        if line.is_empty() {
            break;
        };
        let mut rule_splits = line.split('|');

        let tup = (
            rule_splits.next().unwrap().parse::<usize>().unwrap(),
            rule_splits.next().unwrap().parse::<usize>().unwrap(),
        );

        rules.push(tup);
    }
    SORTING_RULES.set(rules.clone());

    let mut acc: usize = 0;
    let mut incorrect_acc: usize = 0;

    while let Some(line) = line_it.next() {
        let pages: Vec<usize> = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let mut correct = true;
        let mut pit = pages.iter();
        while let Some(page_number) = pit.next() {
            //get rules indexes that need the current value to be after something
            for rule in rules.iter() {
                if !(rule.1 == *page_number) {
                    continue;
                }
                let mut reit = pit.clone();
                if reit.any(|x| rule.0 == *x) {
                    correct = false;
                    break;
                }
            }
        }
        if correct {
            //println!("Line {:?} is correct", line);
            let idx = pages.len() / (2 as usize);
            acc += pages[idx];
        } else {
            let mut p2: Vec<usize> = pages.clone();
            println!("Before sorting: {:?}", p2);
            p2.sort_by(sorting);
            println!("After sorting: {:?}", p2);
            let idx = p2.len() / (2 as usize);
            incorrect_acc += p2[idx];
        }
    }

    println!("The sum is {:?}", acc);
    println!("Incorrect acc is {:?}", incorrect_acc)
}

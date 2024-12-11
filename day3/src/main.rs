use regex::Regex;
use std::fs;

fn main() {
    let ex = Regex::new(r"mul\((?<a>[\d]{1,3}),(?<b>[\d]{1,3})\)").unwrap();
    let filename = "replica.txt";
    let data = fs::read_to_string(filename).unwrap();
    //println!("{:?}",data);
    let caps:Vec<(i32,i32)> = ex.captures_iter(&data).map(|caps| {
        //println!("{:?}", caps);
        // The unwraps are okay because every capture group must match if the whole
        // regex matches, and in this context, we know we have a match.
        //
        // Note that we use `caps.name("y").unwrap().as_str()` instead of
        // `&caps["y"]` because the lifetime of the former is the same as the
        // lifetime of `hay` above, but the lifetime of the latter is tied to the
        // lifetime of `caps` due to how the `Index` trait is defined.
        let a:i32 = caps.name("a").unwrap().as_str().parse().unwrap();
        let b:i32 = caps.name("b").unwrap().as_str().parse().unwrap();
        
        (a,b)
    }).collect();
    let mut acc:i32 = 0;
    for tuple in caps.iter(){
        acc = acc + tuple.0*tuple.1;
    }
    println!("Total is {:?}", acc);

    // with dos and don'ts
    let sections_ex = Regex::new(r"(?s)((\A|do\(\)).*?don't\(\))|(do\(\).*)").unwrap();//|(do\(\).*?(don't\(\)))").unwrap();
    let mut new_acc = 0;
    let mut sections_iter = sections_ex.find_iter(&data).peekable();
    
    if !sections_iter.peek().is_some() {
        println!("Total between dos and don'ts: {:?}", acc);
        return;
    };
    for (i,section) in sections_iter.enumerate() {
        println!("Section {:?}:{:?}", i, section);
        //check the section is correct
        assert!((section.as_str().starts_with("do()")) | (section.range().start==0));
        //assert!((section.as_str().ends_with("don't()")));      
        let caps:Vec<(i32,i32)> = ex.captures_iter(section.as_str()).map(|c| {
                    // The unwraps are okay because every capture group must match if the whole
                    // regex matches, and in this context, we know we have a match.
                    //
                    // Note that we use `caps.name("y").unwrap().as_str()` instead of
                    // `&caps["y"]` because the lifetime of the former is the same as the
                    // lifetime of `hay` above, but the lifetime of the latter is tied to the
                    // lifetime of `caps` due to how the `Index` trait is defined.
                    let a:i32 = c.name("a").unwrap().as_str().parse().unwrap();
                    let b:i32 = c.name("b").unwrap().as_str().parse().unwrap();
                    (a,b)
                }).collect();
        for tuple in caps.iter(){
            new_acc = new_acc + tuple.0*tuple.1;
        }
        println!("");

    };

       

    println!("Total between dos and donts: {:?}", new_acc);
    



}

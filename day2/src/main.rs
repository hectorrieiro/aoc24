use std::fs;

fn diff(v: &Vec<i32>) -> Vec<i32> {
    let mut y = Vec::<i32>::with_capacity(v.len() - 1);
    for i in 1..v.len() {
        let d = v[i] - v[i - 1];
        y.push(d);
    }
    y
}

fn all_same_sign(v: &Vec<i32>) -> bool {
    let i: Vec<bool> = v.iter().map(|x| (*x) > 0).collect();

    for k in 1..i.len() {
        if i[k] != i[k - 1] {
            return false;
        }
    }

    return true;
}

fn within_margins(v: &Vec<i32>) -> bool {
    v.iter()
        .map(|x| ((*x).abs() >= 1) & ((*x).abs() <= 3))
        .all(|x| x)
}

fn is_safe(v: &Vec<i32>) -> bool {
    let d = diff(v);

    all_same_sign(&d) & within_margins(&d)
}
fn dampen_check(d: &Vec<i32>, v: &Vec<i32>) -> bool {
    for (k, di) in d.iter().enumerate() {
        let mut found_anomaly = false;
        if di.abs() < 1 {

            found_anomaly = true;
        }

        if di.abs() > 3 && (k==d.len()-1 || k==0) {
            found_anomaly=true;
        }

        if k < d.len() - 1 {
            if (di * d[k + 1]) < 0 {
               
                found_anomaly = true;
            }
        }
        if found_anomaly {
            println!("Found anomaly at diff position {:?}", k);
            let mut veccopy = v.clone();
            veccopy.remove(k);
            println!("Trying {:?}", veccopy);
            if is_safe(&veccopy) {
                return true;
            }
            let mut veccopy = v.clone();
            veccopy.remove(k+1);
            println!("Trying {:?}", veccopy);
            if is_safe(&veccopy) {
                return true;
            }
            if k < d.len() - 1 {
                veccopy = v.clone();
                veccopy.remove(k+2);
                println!("Trying {:?}", veccopy);
                if is_safe(&veccopy) {
                    return true;
                }
            }
            
        }
    }

    false
}
fn is_safe_dampened(v: &Vec<i32>) -> bool {
    if is_safe(v) {
        return true;
    }
    let d = diff(v);
    dampen_check(&d, &v)
}

// let mut found_error = false;
// let mut next:usize = 1;
// let mut i:usize = 0;
// //need to check the first value first
// let mut last_sign = 0;

// while i<v.len()-1 {
//     let current_sign = v[next]-v[i];
//     let same_sign =  current_sign*last_sign >= 0;
//     let within_margin = ((v[next]-v[i]).abs()<=3) & ((v[next]-v[i]).abs()>=1);
//     if !(same_sign & within_margin) {
//         if found_error {return false;} // not the first error
//         found_error = true;

//     } else {
//         i = i+1;
//     }
//     next = next + 1;
//     if next==v.len() {
//         break;
//     }
// }
// true

//7 9 8 11 12 10
fn main() {
    let filename = "task1.txt";
    let contents: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let mut count_safe: u32 = 0;
    let mut count_safe_dampened: u32 = 0;
    for line in contents {
        let measurements: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if is_safe(&measurements) {
            //println!("{:?} is safe", line);
            count_safe += 1;
        } else {
            //println!("{:?} is unsafe", line);
        }
        if is_safe_dampened(&measurements) {
            //println!("{:?} is safe damepend", line);
            count_safe_dampened += 1;
        } else {
            println!("{:?} is unsafe", line);
        }
    }

    println!("Number of safe reports: {:?}", count_safe);
    println!("Number of safe dampened reports: {:?}", count_safe_dampened);
}

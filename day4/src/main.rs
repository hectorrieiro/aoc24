use core::str;
use std::fs;

//fn get_matching_offsets(pattern_len:usize) -> Vec<Vec<u8>> {

//}

//fn matches(pattern:&[u8], values:&[u8]) -> bool {
//    false
//}

//fn grab_data(matrix: &[&u8,2], offsets:&[u8,2]) -> Option<Vec<u8>> {

//}

#[derive(Debug)]
struct Buffer {
    data: Vec<u8>,
    nrows: usize,
    ncols: usize,
}

impl Buffer {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            nrows: 0,
            ncols: 0,
        }
    }

    fn add_row(&mut self, data: &[u8]) -> Option<usize> {
        if self.nrows == 0 {
            self.data.extend(data);
            self.ncols = data.len();
            self.nrows += 1;
            return Some(self.nrows);
        } else {
            if data.len() != self.ncols {
                return None;
            } else {
                self.data.extend(data);
                self.nrows += 1;
                return Some(self.nrows);
            }
        }
    }

    fn get_element(&self, i: usize, j: usize) -> Option<u8> {
        let idx = self.ncols * i + j;

        match idx <= self.data.len() {
            false => None,
            true => Some(self.data[idx]),
        }
    }

    fn grab_indices(&self, indices: &[(i32, i32)]) -> Option<Vec<u8>> {
        let mut output: Vec<u8> = Vec::new();
        for (i, j) in indices {
            let iu = match usize::try_from(*i) {
                Ok(val) => val,
                _ => return None,
            };
            let ju = match usize::try_from(*j) {
                Ok(val) => val,
                _ => return None,
            };
            if ju >= self.ncols {
                return None;
            }
            let idx = iu * self.ncols + ju;
            let val = self.data.get(idx)?;
            output.push(*val);
        }
        Some(output)
    }

    //fn get(&self, row:usize, row:usize) ->Result<u8, >
}

fn get_offsets(i0: &(usize, usize), dir: &(i32, i32), n: usize) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = Vec::new();
    for i in 0..n {
        output.push((
            (i0.0 as i32) + dir.0 * (i as i32),
            (i0.1 as i32) + dir.1 * (i as i32),
        ));
    }
    output
}

fn main() {
    let input = fs::read_to_string("input1.txt").unwrap();

    let splits_iter = input.split_ascii_whitespace();

    #[allow(non_snake_case)]
    let mut buff = Buffer::new();
    for line in splits_iter {
        //println!("Line: {:?}", line);
        let r: Vec<u8> = line.bytes().collect();
        //println!("Bytes: {:?}", r);

        buff.add_row(r.as_slice())
            .expect("Error appending new row}");
    }

    //println!("Read matrix:\n{:?}", buff);

    //println!("{:?}", M);

    let pattern = "XMAS".as_bytes();
    let pattern_len = pattern.len();

    let mut pattern_count = 0u32;
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    //    let matching_offsets = get_matching_offsets(pattern_len);

    for i in 0..buff.nrows {
        for j in 0..buff.ncols {
            if buff.get_element(i, j).unwrap() != pattern[0] {
                //look for first letter of pattern
                continue;
            }
            for d in directions.iter() {
                let offsets = get_offsets(&(i, j), &d, pattern_len);

                let buf_data = buff.grab_indices(offsets.as_slice());
                match buf_data {
                    Some(bytes) => {
                        let matching = bytes.iter().zip(pattern.iter()).all(|(a, b)| a == b);
                        if matching {
                            pattern_count += 1;
                        }
                    }
                    None => continue,
                }
            }
            //            for v_offsets in matching_offsets {
            //                if let data = grab_data(M, v_offsets) else {
            //                    continue
            //                }

            //            }
        }
    }

    println!("Number of patterns found: {:?}", pattern_count);
    let valid_sequences = vec!["MSMS", "SMSM", "SMSM", "MMSS", "SSMM"];
    let mut pattern_count_task2 = 0u32;
    for i in 0..(buff.nrows as i32) {
        for j in 0..(buff.ncols as i32) {
            if buff.get_element(i as usize, j as usize).unwrap() != ('A'.to_ascii_uppercase() as u8)
            {
                //look for first letter of pattern
                continue;
            }
            if let Some(buf_data) = buff.grab_indices(
                vec![
                    (i - 1, j + 1),
                    (i + 1, j + 1),
                    (i - 1, j - 1),
                    (i + 1, j - 1),
                ]
                .as_slice(),
            ) {
                let s = str::from_utf8(buf_data.as_slice()).unwrap();
                if valid_sequences.iter().any(|&seq| {
                    println!("{:?}", seq);
                    println!("{:?}=={:?}=>{:?}", seq, s, seq == s);
                    seq == s
                }) {
                    pattern_count_task2 += 1;
                    println!("Found at ({:?},{:?}\t{:?})", i, j, s);
                }
            }
        }
    }

    println!(
        "Number of patterns found on task 2: {:?}",
        pattern_count_task2
    );
}

use std::{cmp::Ordering, fs};

#[derive(Clone, Copy)]
struct File {
    id: Option<usize>,
    position: usize,
    size: usize,
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.position < other.position {
            return Some(Ordering::Less);
        } else if self.position > other.position {
            return Some(Ordering::Greater);
        }
        return None;
    }
}

fn checksum_from_blocks(list: &Vec<File>) -> u64 {
    let mut acc = 0u64;
    list.iter().for_each(|&f| {
        if let Some(id) = f.id {
            for k in (f.position)..(f.position + f.size) {
                acc += (k as u64) * (id as u64);
            }
        }
    });
    acc
}

fn data_to_string(v: &[u8]) -> Vec<Option<usize>> {
    let mut is_data = true;
    let mut out = Vec::<Option<usize>>::new();
    let mut current_id = 0 as usize;
    for k in 0..v.len() {
        let val = (v[k] as char).to_digit(10).unwrap() as usize;
        if is_data {
            //s += current_id.to_string().repeat(val).as_str();
            let mut a = vec![Some(current_id)].repeat(val);
            out.append(&mut a);
            current_id += 1;
            is_data = !is_data;
        } else {
            let mut a: Vec<Option<usize>> = vec![None].repeat(val);
            out.append(&mut a);
            is_data = !is_data;
        }
    }
    out
}

fn blocks_to_string(v: &Vec<File>) -> String {
    let mut s = String::new();
    for &b in v.iter() {
        let c: char;
        if let Some(id) = b.id {
            c = char::from_u32(id as u32).unwrap();
        } else {
            c = '.';
        }
        s.push_str(c.to_string().repeat(b.size).as_str());
    }
    s
}

fn get_data_blocks(v: &[u8]) -> Vec<File> {
    let mut out = Vec::<File>::new();
    let mut is_data = true;
    let mut current_id = 0 as usize;
    let mut current_position = 0 as usize;
    for k in 0..v.len() {
        if is_data {
            out.push(File {
                id: Some(current_id),
                position: current_position,
                size: (v[k] as char).to_digit(10).unwrap() as usize,
            });
            current_id += 1;
            current_position += (v[k] as char).to_digit(10).unwrap() as usize;
            is_data = !is_data;
        } else {
            let size = (v[k] as char).to_digit(10).unwrap() as usize;
            if size > 0 {
                out.push(File {
                    id: None,
                    position: current_position,
                    size: size,
                });
            }

            current_position += size;
            is_data = !is_data;
        }
    }
    out
}

fn seek_forward(idx: usize, v: &[Option<usize>]) -> usize {
    let mut out_idx = idx;
    while let Some(_x) = v[out_idx] {
        out_idx += 1;
    }
    out_idx
}

fn seek_back(idx: usize, v: &[Option<usize>]) -> usize {
    let mut out_idx = idx;
    while v[out_idx] == None {
        out_idx -= 1;
    }
    out_idx
}

fn checksum(v: &Vec<Option<usize>>) -> u64 {
    let mut sum = 0u64;
    v.iter().enumerate().for_each(|(i, &x)| {
        if let Some(x) = x {
            sum += (i as u64) * (x as u64);
        }
    });
    sum
}

fn main() {
    let data = fs::read("input.txt").unwrap();
    let mut v = data_to_string(&data);
    //println!("{:?}", s);

    //println!("{:?}", String::from_utf8(v.clone()));
    let mut start_idx = 0usize; //.position(|&x| x==b'.');
    let mut end_idx = v.len() - 1; //.position(|&x| x!=b'.');
    while start_idx < end_idx {
        start_idx = seek_forward(start_idx, &v);
        end_idx = seek_back(end_idx, &v);
        if start_idx >= end_idx {
            break;
        }
        v[start_idx] = v[end_idx];
        v[end_idx] = None;
        //println!("{:?}", String::from_utf8(v.clone()));
    }
    //fs::write("defrag.txt", String::from_utf8(v.clone()).unwrap());
    //println!("{:?}", v);
    let cs = checksum(&v);
    println!("Final checksum is \n{:?}", cs);

    let mut v_defrag = data_to_string(&data);
    let mut data_blocks = get_data_blocks(&data);
    let mut start_idx = 0usize; //.position(|&x| x==b'.');
    let mut end_idx = v.len() - 1; //.position(|&x| x!=b'.');
    let mut k = data_blocks.len() - 1;
    while k > 0 {
        match data_blocks[k].id {
            None => {
                k = k - 1;
                continue;
            }
            Some(_id) => {
                if let Some(space_idx) = data_blocks
                    .iter()
                    .position(|&b| b.id == None && b.size >= data_blocks[k].size)
                {
                    if space_idx > k {
                        k = k - 1;
                        continue;
                    }
                    let mut block = data_blocks.remove(k);
                    if (data_blocks.len() > k + 1)
                        && (k > 0)
                        && (data_blocks[k - 1].id == None)
                        && (data_blocks[k].id == None)
                    {
                        let second_block = data_blocks.remove(k);
                        data_blocks[k - 1].size += (second_block.size + block.size);
                        k = k - 2;
                    } else if (k > 0) && (data_blocks[k - 1].id == None) {
                        data_blocks[k - 1].size += block.size;
                        k = k - 2;
                    } else if (data_blocks.len() > k + 1) && (data_blocks[k].id == None) {
                        data_blocks[k].size += block.size;
                        data_blocks[k].position = block.position;
                        if k > 0 {
                            k = k - 1;
                        }
                    } else {
                        let new_block = File {
                            position: block.position,
                            size: block.size,
                            id: None,
                        };
                        data_blocks.insert(k, new_block);
                        if k > 0 {
                            k = k - 1;
                        }
                    }
                    let mut empty_block = data_blocks.get_mut(space_idx).unwrap();
                    block.position = empty_block.position;
                    if empty_block.size > block.size {
                        empty_block.position += block.size;
                        empty_block.size -= block.size;
                        data_blocks.insert(space_idx, block);
                        k = k + 1;
                    } else {
                        data_blocks[space_idx] = block;
                    }
                } else {
                    k = k - 1;
                }
            }
        }
        //println!("{:?}", blocks_to_string(&data_blocks));
    }
    //println!("{:?}", blocks_to_string(&data_blocks));
    println!("Final checksum: {:?}", checksum_from_blocks(&data_blocks));
}

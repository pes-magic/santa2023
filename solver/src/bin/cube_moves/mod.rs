// Reference
// * https://github.com/merpig/RubiksProgram
// * https://cube.uubio.com

use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use once_cell::sync::Lazy;

static BFS_FOUR_CORNER: Lazy<(Vec<Option<usize>>, Vec<Option<usize>>)> =
    Lazy::new(|| bfs_four_corner_impl());

static BFS_FOUR_EDGE: Lazy<(Vec<Option<usize>>, Vec<Option<usize>>)> =
    Lazy::new(|| bfs_four_edge_impl());

static PERM_FOUR_CORNER: Lazy<(Vec<u32>, HashMap<u32, usize>)> =
    Lazy::new(|| gen_perm_map_four_faces_impl());

#[allow(dead_code)]
pub fn rev_move(s: &String) -> String {
    if s.starts_with("-") {
        s[1..].to_string()
    } else {
        format!("-{}", s)
    }
}

#[allow(dead_code)]
pub fn rot_move_f(s: &String, dim: usize) -> String {
    if s.starts_with("r") {
        format!("d{}", &s[1..])
    } else if s.starts_with("-r") {
        format!("-d{}", &s[2..])
    } else if s.starts_with("d") {
        format!("-r{}", dim - 1 - s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-d") {
        format!("r{}", dim - 1 - s[2..].parse::<usize>().unwrap())
    } else {
        s.to_string()
    }
}

#[allow(dead_code)]
pub fn rot2_move_f(s: &String, dim: usize) -> String {
    if s.starts_with("r") {
        format!("-r{}", dim - 1 - &s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-r") {
        format!("r{}", dim - 1 - &s[2..].parse::<usize>().unwrap())
    } else if s.starts_with("f") {
        format!("-f{}", dim - 1 - s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-f") {
        format!("f{}", dim - 1 - s[2..].parse::<usize>().unwrap())
    } else {
        s.to_string()
    }
}

#[allow(dead_code)]
pub fn rot_move_d(s: &String, dim: usize) -> String {
    if s.starts_with("r") {
        format!("f{}", &s[1..])
    } else if s.starts_with("-r") {
        format!("-f{}", &s[2..])
    } else if s.starts_with("f") {
        format!("-r{}", dim - 1 - s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-f") {
        format!("r{}", dim - 1 - s[2..].parse::<usize>().unwrap())
    } else {
        s.to_string()
    }
}

#[allow(dead_code)]
pub fn rot2_move_d(s: &String, dim: usize) -> String {
    if s.starts_with("r") {
        format!("-r{}", dim - 1 - &s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-r") {
        format!("r{}", dim - 1 - &s[2..].parse::<usize>().unwrap())
    } else if s.starts_with("f") {
        format!("-f{}", dim - 1 - s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-f") {
        format!("f{}", dim - 1 - s[2..].parse::<usize>().unwrap())
    } else {
        s.to_string()
    }
}

#[allow(dead_code)]
pub fn rot_move_r(s: &String, dim: usize) -> String {
    if s.starts_with("f") {
        format!("d{}", &s[1..])
    } else if s.starts_with("-f") {
        format!("-d{}", &s[2..])
    } else if s.starts_with("d") {
        format!("-f{}", dim - 1 - s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-d") {
        format!("f{}", dim - 1 - s[2..].parse::<usize>().unwrap())
    } else {
        s.to_string()
    }
}

#[allow(dead_code)]
pub fn rot2_move_r(s: &String, dim: usize) -> String {
    if s.starts_with("f") {
        format!("-f{}", dim - 1 - &s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-f") {
        format!("f{}", dim - 1 - &s[2..].parse::<usize>().unwrap())
    } else if s.starts_with("d") {
        format!("-d{}", dim - 1 - s[1..].parse::<usize>().unwrap())
    } else if s.starts_with("-d") {
        format!("d{}", dim - 1 - s[2..].parse::<usize>().unwrap())
    } else {
        s.to_string()
    }
}

#[allow(dead_code)]
pub fn rot_sequence_f(seq: &Vec<String>, dim: usize) -> Vec<String> {
    seq.iter().map(|s| rot_move_f(s, dim)).collect()
}

#[allow(dead_code)]
pub fn rot2_sequence_f(seq: &Vec<String>, dim: usize) -> Vec<String> {
    seq.iter().map(|s| rot2_move_f(s, dim)).collect()
}

#[allow(dead_code)]
pub fn rot_sequence_d(seq: &Vec<String>, dim: usize) -> Vec<String> {
    seq.iter().map(|s| rot_move_d(s, dim)).collect()
}

#[allow(dead_code)]
pub fn rot2_sequence_d(seq: &Vec<String>, dim: usize) -> Vec<String> {
    seq.iter().map(|s| rot2_move_d(s, dim)).collect()
}

#[allow(dead_code)]
pub fn rot_sequence_r(seq: &Vec<String>, dim: usize) -> Vec<String> {
    seq.iter().map(|s| rot_move_r(s, dim)).collect()
}

#[allow(dead_code)]
pub fn rot2_sequence_r(seq: &Vec<String>, dim: usize) -> Vec<String> {
    seq.iter().map(|s| rot2_move_r(s, dim)).collect()
}

#[derive(Debug, Clone)]
pub struct P3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl P3 {
    #[allow(dead_code)]
    pub fn new(x: usize, y: usize, z: usize) -> P3 {
        P3 { x, y, z }
    }
}

fn p3_to_side(p: &P3, dim: usize) -> char {
    if p.x == 0 {
        'L'
    } else if p.x == dim - 1 {
        'R'
    } else if p.y == 0 {
        'F'
    } else if p.y == dim - 1 {
        'B'
    } else if p.z == 0 {
        'D'
    } else if p.z == dim - 1 {
        'U'
    } else {
        panic!("Invalid p3")
    }
}

#[allow(dead_code)]
pub fn solve_white_middle_impl(current: &P3, solved: &P3, dim: usize) -> Vec<String> {
    let current_side = p3_to_side(current, dim);
    let row = dim - 2 - solved.z;
    let column = solved.x - 1;
    let mut res: Vec<String> = Vec::new();

    if solved.x == 1 {
        if dim - solved.z == 2 {
            if current_side == 'F' {
                res.push("f0".to_string());
            }
            // !D and !U
            else if current_side != 'D' && current_side != 'U' {
                if current.z != dim - 2 {
                    if current_side == 'B' {
                        res.push(format!("-f{}", dim - 1));
                    } else if current_side == 'R' {
                        res.push("r0".to_string());
                    } else if current_side == 'L' {
                        res.push(format!("-r{}", dim - 1));
                    } else {
                        panic!("invalid current_side");
                    }
                } else {
                    res.push(format!("-d{}", current.z));
                }
            } else {
                if current.x != solved.x {
                    if current_side == 'D' {
                        res.push("d0".to_string());
                    } else if current_side == 'U' {
                        res.push(format!("-d{}", dim - 1));
                    } else {
                        panic!("invalid current_side");
                    }
                } else {
                    res.push(format!("-r{}", dim - 2));
                }
            }
        } else {
            if current_side == 'F' {
                res.push(format!("-d{}", current.z));
            } else if current_side == 'B' {
                if current.z <= solved.z {
                    res.push(format!("d{}", current.z));
                } else {
                    res.push(format!("-f{}", dim - 1));
                }
            } else if current_side == 'L' {
                if current.y != dim - 2 {
                    res.push(format!("-r{}", dim - 1));
                } else if current.z != solved.z {
                    res.push(format!("-r{}", dim - 1));
                } else {
                    res.push(format!("d{}", current.z));
                }
            } else {
                res.push(format!("-f{}", current.y));
            }
        }
    } else {
        if row == 0 {
            if current_side == 'F' {
                if current.z < solved.z && current.x >= solved.x {
                    res.push(format!("r{}", dim - 1 - current.x));
                } else {
                    res.push(format!("d{}", current.z));
                }
            } else if current_side == 'B' {
                if current.x == solved.x && current.z != solved.z {
                    res.push(format!("r{}", dim - 2 - column));
                    res.push(format!("r{}", dim - 2 - column));
                } else {
                    res.push(format!("-f{}", dim - 1));
                };
            } else {
                if current.y == dim - 2 {
                    if current.z != dim - 1 {
                        res.push(format!("-f{}", dim - 2 - row));
                    } else {
                        if current.x != solved.x {
                            res.push(format!("-d{}", dim - 1));
                        } else {
                            res.push(format!("-r{}", dim - 2 - column));
                        };
                    }
                } else {
                    if current_side == 'D' {
                        res.push("d0".to_string());
                    } else if current_side == 'U' {
                        res.push(format!("-d{}", dim - 1));
                    } else if current_side == 'R' {
                        res.push("r0".to_string());
                    } else if current_side == 'L' {
                        res.push(format!("-r{}", dim - 1));
                    } else {
                        panic!("invalid current_side");
                    }
                }
            }
        } else {
            if current_side == 'F' {
                if current.z < solved.z {
                    res.push(format!("-d{}", current.z));
                } else {
                    res.push(format!("-d{}", current.z));
                    res.push(format!("-f{}", dim - 1 - current.x));
                    res.push(format!("d{}", current.z));
                }
            } else if current_side == 'B' {
                if current.z == solved.z {
                    if solved.x == current.x {
                        res.push(format!("-d{}", current.z));
                        res.push(format!("-f{}", dim - 1 - current.x));
                        res.push(format!("d{}", current.z));
                    } else {
                        res.push(format!("-d{}", current.z));
                        res.push("r0".to_string());
                        res.push(format!("d{}", current.z));
                    }
                } else {
                    res.push(format!("-f{}", dim - 1));
                }
            } else {
                if current_side != 'U' {
                    res.push(format!("f{}", current.y));
                } else {
                    if current.x != solved.x || current.y != solved.z {
                        res.push(format!("-d{}", dim - 1));
                    } else {
                        res.push(format!("d{}", dim - 1));
                        res.push(format!("d{}", solved.z));
                        res.push(format!("f{}", current.x));
                        res.push(format!("-d{}", solved.z));
                    }
                }
            }
        }
    }
    res
}

#[allow(dead_code)]
pub fn solve_yellow_middle_impl(current: &P3, solved: &P3, dim: usize) -> Vec<String> {
    let current_side = p3_to_side(current, dim);
    let middle = dim / 2;
    let mut res: Vec<String> = Vec::new();

    if current_side == 'B' {
        let check_middle = dim % 2 == 1 && current.z == middle;
        res.push(format!("-d{}", current.z));
        res.push("r0".to_string());
        if !check_middle {
            res.push("r0".to_string());
        }
        res.push(format!("d{}", current.z));
        if check_middle {
            res.push("r0".to_string());
        }
        res.push(format!("f{}", current.x));
        if check_middle {
            res.push("r0".to_string());
        }
        res.push(format!("-d{}", current.z));
        res.push("r0".to_string());
        if !check_middle {
            res.push("r0".to_string());
        }
        res.push(format!("d{}", current.z));
    } else {
        if current_side != 'R' {
            res.push(format!("f{}", current.y));
        } else {
            if current.y != dim - (solved.x + 1) || current.z != solved.z {
                res.push("r0".to_string());
            } else {
                res.push(format!("-f{}", current.y));
                res.push(format!("-d{}", current.z));
                if (current.y >= middle && current.z >= middle)
                    || (current.y < middle && current.z < middle)
                {
                    res.push(format!("-r{}", dim - 1));
                } else {
                    res.push(format!("r{}", dim - 1));
                }
                res.push(format!("f{}", current.y));
                if (current.y >= middle && current.z >= middle)
                    || (current.y < middle && current.z < middle)
                {
                    res.push(format!("r{}", dim - 1));
                } else {
                    res.push(format!("-r{}", dim - 1));
                }
                res.push(format!("d{}", current.z));
            }
        }
    }
    res
}

#[allow(dead_code)]
pub fn solve_blue_middle_impl(current: &P3, solved: &P3, dim: usize) -> Vec<String> {
    let current_side = p3_to_side(current, dim);
    let middle = dim / 2;
    let odd_cube = dim % 2 == 1;
    let mut res: Vec<String> = Vec::new();

    if odd_cube && solved.x == middle && solved.y == middle {
        match current_side {
            'L' => {
                res.push(format!("-f{}", current.y));
                res.push(format!("-r{}", dim - 1));
                res.push(format!("f{}", current.y));
                res.push(format!("r{}", dim - 1));
                res.push(format!("f{}", current.y));
            }
            'D' => {
                res.push(format!("-f{}", current.y));
                res.push(format!("-r{}", dim - 1));
                res.push(format!("f{}", current.y));
                res.push(format!("f{}", current.y));
                res.push(format!("r{}", dim - 1));
                res.push(format!("f{}", current.y));
            }
            'R' => {
                res.push(format!("f{}", current.y));
                res.push("r0".to_string());
                res.push(format!("-f{}", current.y));
                res.push("-r0".to_string());
                res.push(format!("-f{}", current.y));
            }
            _ => {
                panic!("Shouldn't be checking middle if already in solved location.");
            }
        }
    } else if current_side == 'U' {
        res.push(format!("f{}", current.y)); //1
        res.push(format!("d{}", dim - 1 - current.x)); //2
        res.push(format!("d{}", dim - 1 - current.x)); //2
        if odd_cube && current.y == middle {
            res.push(format!("r{}", dim - 1)); //2.1
        }
        res.push(format!("-f{}", current.y)); //3
        if odd_cube && current.y == middle {
            res.push(format!("-r{}", dim - 1)); //3.1
        }
        res.push(format!("d{}", dim - 1 - current.x)); //4
        res.push(format!("d{}", dim - 1 - current.x)); //4
    } else if current_side == 'R' {
        if (dim - current.z) == (solved.x + 1) && current.y == solved.y {
            res.push(format!("d{}", dim - 1 - solved.x)); // 6
            res.push(format!("d{}", dim - 1 - solved.x)); // 6
            if odd_cube && solved.y == middle {
                res.push(format!("r{}", dim - 1)); //6.1
            }
            res.push(format!("f{}", solved.y)); // 7
            if odd_cube && solved.y == middle {
                res.push(format!("-r{}", dim - 1)); //7.1
            }
            res.push(format!("d{}", dim - 1 - solved.x)); // 8
            res.push(format!("d{}", dim - 1 - solved.x)); // 8
            res.push(format!("-f{}", solved.y)); // 9
        } else {
            res.push("-r0".to_string()); //5
        }
    } else if current_side == 'L' {
        if current.z == solved.x && current.y == solved.y {
            res.push(format!("d{}", current.z)); // 6
            res.push(format!("d{}", current.z)); // 6
            if odd_cube && solved.y == middle {
                res.push("r0".to_string()); //6.1
            }
            res.push(format!("-f{}", solved.y)); // 7
            if odd_cube && solved.y == middle {
                res.push("-r0".to_string()); //7.1
            }
            res.push(format!("d{}", current.z)); // 8
            res.push(format!("d{}", current.z)); // 8
            res.push(format!("f{}", solved.y)); // 9
        } else {
            res.push(format!("-r{}", dim - 1)); //5
        }
    } else {
        if current.y != solved.y {
            res.push("d0".to_string());
        } else {
            res.push(format!("-f{}", solved.y));
            res.push("r0".to_string());
            res.push(format!("f{}", solved.y));
        }
    }

    res
}

#[allow(dead_code)]
pub fn solve_orange_middle_impl(current: &P3, solved: &P3, dim: usize) -> Vec<String> {
    let current_side = p3_to_side(current, dim);
    let middle = dim / 2;
    let odd_cube = dim % 2 == 1;
    let mut res: Vec<String> = Vec::new();

    // 4x4 Solver for Orange
    if dim % 2 == 0 {
        //console.log("4x4 orange middle solver here");
        match current_side {
            'L' => {
                if solved.x == 0 && solved.y == middle - 1 && solved.z == middle {
                    res.push(format!("-r{}", dim - 1));
                } else if solved.x == 0 && solved.y == middle - 1 && solved.z == middle - 1 {
                    res.push(format!("-f{}", current.y));
                    res.push("d0".to_string());
                    res.push("d0".to_string());
                    res.push(format!("f{}", current.y));
                } else if solved.x == 0 && solved.y == middle && solved.z == middle {
                    res.push(format!("-f{}", current.y));
                    res.push("d0".to_string());
                    res.push("d0".to_string());
                    res.push(format!("f{}", current.y));
                } else {
                    if solved.z == solved.y
                        || solved.z == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == solved.y
                    {
                        if current.z < middle - 1 && current.y < middle - 1 {
                            res.push(format!("r{}", dim - 1));
                            res.push("-d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push(format!("-r{}", dim - 1));
                        } else if current.z > middle && current.y < middle - 1 {
                            res.push("-d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                        } else if current.z > middle && current.y > middle {
                            res.push(format!("-r{}", dim - 1));
                            res.push("-d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push(format!("r{}", dim - 1));
                        } else if current.z < middle - 1 && current.y > middle {
                            res.push(format!("r{}", dim - 1));
                            res.push(format!("r{}", dim - 1));
                            res.push("-d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push(format!("r{}", dim - 1));
                            res.push(format!("r{}", dim - 1));
                        }
                    } else {
                        res.push("-d0".to_string());
                        res.push(format!("f{}", current.z));
                        res.push("d0".to_string());
                        res.push(format!("-f{}", current.y));
                        res.push("-d0".to_string());
                        res.push(format!("-f{}", current.z));
                        res.push("d0".to_string());
                        res.push(format!("f{}", current.y));
                    }
                }
            }
            'D' => {
                if solved.x == 0 && solved.y == middle - 1 && solved.z == middle {
                    res.push(format!("f{}", current.y));
                    res.push(format!("r{}", dim - 1));
                    res.push(format!("r{}", dim - 1));
                    res.push(format!("-f{}", current.y));
                } else if solved.x == 0 && solved.y == middle - 1 && solved.z == middle - 1 {
                    if current.x == middle && current.y == middle {
                        res.push(format!("-r{}", dim - 1));
                        res.push(format!("-f{}", middle - 1));
                        res.push("-d0".to_string());
                        res.push(format!("f{}", middle - 1));
                    } else {
                        res.push("d0".to_string());
                    }
                } else if solved.x == 0 && solved.y == middle && solved.z == middle {
                    if current.y == middle - 1 && current.x == middle - 1 {
                        res.push(format!("-f{}", middle));
                        res.push("-d0".to_string());
                        res.push(format!("f{}", middle));
                    } else {
                        res.push("d0".to_string());
                    }
                } else if solved.x == 0 && solved.y == middle && solved.z == middle - 1 {
                    if current.y == middle - 1 && current.x == middle {
                        res.push(format!("-f{}", middle));
                        res.push("-d0".to_string());
                        res.push(format!("f{}", middle));
                        res.push("d0".to_string());
                        res.push(format!("-f{}", middle));
                        res.push("d0".to_string());
                        res.push(format!("f{}", middle));
                    } else {
                        res.push("d0".to_string());
                    }
                } else {
                    if solved.x == solved.y
                        || solved.x == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == solved.y
                    {
                        // solve from green to orange middles
                        if current.x < middle - 1 && current.y < middle - 1 {
                            if solved.z < middle - 1 && solved.y < middle - 1 {
                                res.push(format!("r{}", dim - 1));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", solved.y));
                                res.push(format!("-r{}", dim - 1));
                            } else if solved.z > middle && solved.y < middle - 1 {
                                res.push("-d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", solved.y));
                            } else if solved.z > middle && solved.y > middle {
                                res.push(format!("-r{}", dim - 1));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push(format!("r{}", dim - 1));
                            } else if solved.z < middle - 1 && solved.y > middle {
                                res.push(format!("r{}", dim - 1));
                                res.push(format!("r{}", dim - 1));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push(format!("r{}", dim - 1));
                                res.push(format!("r{}", dim - 1));
                            }
                        } else {
                            res.push("d0".to_string());
                        }
                    } else {
                        if current.y == solved.y && current.x == (dim - 1) - solved.z {
                            res.push("-d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.x));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.x));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                        } else {
                            res.push("d0".to_string());
                        }
                    }
                }
            }
            'R' => {
                if solved.x == 0 && solved.y == middle - 1 && solved.z == middle {
                    res.push(format!("f{}", current.y));
                    res.push(format!("f{}", current.y));
                    res.push(format!("r{}", dim - 1));
                    res.push(format!("r{}", dim - 1));
                    res.push(format!("f{}", current.y));
                    res.push(format!("f{}", current.y));
                } else if (solved.x == 0 && solved.y == middle - 1 && solved.z == middle - 1)
                    || (solved.x == 0 && solved.y == middle && solved.z == middle - 1)
                    || (solved.x == 0 && solved.y == middle && solved.z == middle)
                {
                    res.push(format!("f{}", current.y));
                    res.push("d0".to_string());
                    res.push("d0".to_string());
                    res.push(format!("-f{}", current.y));
                } else {
                    res.push(format!("f{}", current.y));
                    res.push("d0".to_string());
                    res.push(format!("-f{}", current.y));
                }
            }
            _ => {
                panic!("Something broke");
            }
        }
    }
    // 5x5 Solver for Orange
    else if dim % 2 == 1 {
        //console.log(solved);
        // First row
        match current_side {
            'L' => {
                if solved.z >= middle - 1
                    && solved.z <= middle + 1
                    && solved.y >= middle - 1
                    && solved.y <= middle + 1
                {
                    // piece 1
                    if solved.x == 0 && solved.y == middle - 1 && solved.z == middle + 1 {
                        res.push(format!("-r{}", dim - 1));
                    }
                    //row 1 solved location
                    else if solved.y == middle - 1 {
                        res.push(format!("-f{}", current.y));
                        if middle != 0 && odd_cube {
                            res.push("-d0".to_string());
                            res.push(format!("f{}", current.y));
                        } else {
                            res.push("d0".to_string());
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                        }
                    }
                    //row 2 solved location
                    else if solved.y == middle {
                        res.push(format!("-f{}", current.y));
                        res.push("-d0".to_string());
                        res.push(format!("f{}", current.y));
                    }
                    //row 3 solved location
                    else if solved.y == middle + 1 {
                        if solved.z == middle + 1 {
                            res.push(format!("-f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                        }
                    }
                } else {
                    if solved.z == solved.y
                        || solved.z == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == solved.y
                    {
                        if current.z < middle - 1 && current.y < middle - 1 {
                            res.push(format!("r{}", dim - 1));
                            res.push("-d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push(format!("-r{}", dim - 1));
                        } else if current.z > middle && current.y < middle - 1 {
                            res.push("-d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                        } else if current.z > middle && current.y > middle {
                            res.push(format!("-r{}", dim - 1));
                            res.push("-d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push(format!("r{}", dim - 1));
                        } else if current.z < middle - 1 && current.y > middle {
                            res.push(format!("r{}", dim - 1));
                            res.push(format!("r{}", dim - 1));
                            res.push("-d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push(format!("r{}", dim - 1));
                            res.push(format!("r{}", dim - 1));
                        }
                    } else {
                        res.push("-d0".to_string());
                        res.push(format!("f{}", current.z));
                        res.push("d0".to_string());
                        res.push(format!("-f{}", current.y));
                        res.push("-d0".to_string());
                        res.push(format!("-f{}", current.z));
                        res.push("d0".to_string());
                        res.push(format!("f{}", current.y));
                    }
                }
            }
            'D' => {
                if solved.z >= middle - 1
                    && solved.z <= middle + 1
                    && solved.y >= middle - 1
                    && solved.y <= middle + 1
                {
                    if solved.x == 0 && solved.y == middle - 1 && solved.z == middle + 1 {
                        res.push(format!("f{}", current.y));
                        res.push(format!("r{}", dim - 1));
                        res.push(format!("r{}", dim - 1));
                        res.push(format!("-f{}", current.y));
                    }
                    // row 1
                    else if solved.y == middle - 1 {
                        if current.x < middle + 1 || current.y == middle - 1 {
                            res.push("d0".to_string());
                        } else {
                            res.push(format!("-r{}", dim - 1));
                            res.push(format!("f{}", current.y));
                            res.push(format!("r{}", dim - 1));
                            res.push(format!("-f{}", current.y));
                        }
                    }
                    // row 2 (rework)
                    else if solved.y == middle {
                        // first piece row 2
                        if solved.z == middle + 1 {
                            if current.y != middle - 1 {
                                res.push("d0".to_string());
                            } else {
                                res.push(format!("-f{}", middle));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", middle));
                            }
                        }
                        // last piece row 2
                        else if solved.z == middle - 1 {
                            if current.y != middle + 1 {
                                res.push("d0".to_string());
                            } else {
                                res.push(format!("-f{}", middle));
                                res.push(format!("f{}", current.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", middle));
                            }
                        }
                    }
                    // row 3
                    else if solved.y == middle + 1 {
                        // piece 1
                        if solved.z == middle + 1 {
                            if current.x == middle - 1 && current.y == middle - 1 {
                                res.push(format!("-f{}", middle + 1));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", middle + 1));
                            } else {
                                res.push("d0".to_string());
                            }
                        }
                        // piece 2
                        else if solved.z == middle {
                            if current.x == middle + 1 {
                                res.push(format!("f{}", middle + 1));
                                res.push(format!("f{}", middle + 1));
                                res.push("-r0".to_string());
                                res.push(format!("f{}", middle + 1));
                                res.push("d0".to_string());
                                res.push(format!("f{}", middle + 1));
                            } else {
                                res.push("-d0".to_string());
                            }
                        } else if solved.z == middle - 1 {
                            if current.x == middle - 1 && current.y == middle + 1 {
                                //04F' 05R' 04F 05R 04F'
                                res.push(format!("f{}", middle + 1));
                                res.push(format!("r{}", dim - 1));
                                res.push(format!("-f{}", middle + 1));
                                res.push(format!("-r{}", dim - 1));
                                res.push("d0".to_string());
                                res.push("d0".to_string());
                                res.push(format!("-f{}", middle + 1));
                                res.push("d0".to_string());
                                res.push("d0".to_string());
                                res.push(format!("f{}", middle + 1));
                            } else {
                                res.push("d0".to_string());
                            }
                        }
                    }
                } else {
                    if solved.z == solved.y
                        || solved.z == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == (dim - 1) - solved.y
                        || (dim - 1) - solved.z == solved.y
                    {
                        if current.x < middle - 1 && current.y < middle - 1 {
                            if solved.z < middle - 1 && solved.y < middle - 1 {
                                res.push(format!("r{}", dim - 1));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", solved.y));
                                res.push(format!("-r{}", dim - 1));
                            } else if solved.z > middle && solved.y < middle - 1 {
                                res.push("-d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", solved.y));
                            } else if solved.z > middle && solved.y > middle {
                                res.push(format!("-r{}", dim - 1));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push(format!("r{}", dim - 1));
                            } else if solved.z < middle - 1 && solved.y > middle {
                                res.push(format!("r{}", dim - 1));
                                res.push(format!("r{}", dim - 1));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", dim - 1 - solved.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", solved.y));
                                res.push("d0".to_string());
                                res.push(format!("f{}", dim - 1 - solved.y));
                                res.push(format!("r{}", dim - 1));
                                res.push(format!("r{}", dim - 1));
                            }
                        } else {
                            res.push("d0".to_string());
                        }
                    } else {
                        if current.y == solved.y && current.x == (dim - 1) - solved.z {
                            res.push("-d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.x));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.x));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                        } else {
                            res.push("d0".to_string());
                        }
                    }
                }
            }
            'R' => {
                if solved.z >= middle - 1
                    && solved.z <= middle + 1
                    && solved.y >= middle - 1
                    && solved.y <= middle + 1
                {
                    if solved.x == 0 && solved.y == middle - 1 && solved.z == middle + 1 {
                        res.push(format!("f{}", current.y));
                        res.push(format!("f{}", current.y));
                        res.push(format!("r{}", dim - 1));
                        res.push(format!("r{}", dim - 1));
                        res.push(format!("f{}", current.y));
                        res.push(format!("f{}", current.y));
                    }
                    // row 1
                    else if solved.y == middle - 1 {
                        if current.z != middle + 1 || current.y == middle - 1 {
                            res.push("r0".to_string());
                        } else {
                            res.push(format!("-r{}", dim - 1));
                            res.push(format!("f{}", current.y));
                            res.push(format!("f{}", current.y));
                            res.push(format!("r{}", dim - 1));
                            res.push(format!("f{}", current.y));
                            res.push(format!("f{}", current.y));
                        }
                    }
                    // row 2 (rework)
                    else if solved.y == middle {
                        res.push(format!("f{}", current.y));
                        res.push("d0".to_string());
                        res.push(format!("-f{}", current.y));
                    }
                    // row 3
                    else if solved.y == middle + 1 {
                        // piece 1
                        if solved.z == middle + 1 {
                            if current.y == middle - 1 && current.z == middle - 1 {
                                res.push(format!("f{}", middle + 1));
                                res.push(format!("f{}", middle + 1));
                                res.push("-r0".to_string());
                                res.push(format!("f{}", middle + 1));
                                res.push(format!("f{}", middle + 1));
                            } else {
                                res.push("r0".to_string());
                            }
                        }
                        // piece 2
                        else if solved.z == middle {
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                        } else if solved.z == middle - 1 {
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                        }
                    }
                } else {
                    res.push(format!("f{}", current.y));
                    res.push("d0".to_string());
                    res.push(format!("-f{}", current.y));
                }
            }
            _ => {
                panic!("Something broke");
            }
        }
    }

    res
}

#[allow(dead_code)]
pub fn solve_four_faces_impl(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    if dim % 2 == 1 {
        let half = dim / 2;
        let idx_offset = half * dim + half;
        let action = format!("f{}", half);
        if cur_state[idx_offset] != sol_state[idx_offset] {
            if cur_state[5 * dim * dim + idx_offset] == sol_state[idx_offset] {
                state = apply_action(&state, &actions[&action]);
                moves.push(action.clone());
            }
            if state[4 * dim * dim + idx_offset] == sol_state[idx_offset] {
                state = apply_action(&state, &actions[&action]);
                moves.push(action.clone());
            } else if state[2 * dim * dim + idx_offset] == sol_state[idx_offset] {
                let action = format!("-{}", action);
                state = apply_action(&state, &actions[&action]);
                moves.push(action.clone());
            }
        }
    }
    let (end_state, m) = solve_four_faces_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_four_faces_edge(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    (state, moves)
}

fn extract_from_front(current: &P3, max_coord: usize, min_coord: usize, dim: usize) -> Vec<String> {
    if current.z == max_coord {
        Vec::from([
            format!("r{}", dim - 1 - current.x),
            format!("f{}", dim - 1),
            "r0".to_string(),
            format!("-f{}", dim - 1),
            format!("-r{}", dim - 1 - current.x),
            "-r0".to_string(),
        ])
    } else if current.x == max_coord {
        Vec::from([
            format!("d{}", current.z),
            format!("f{}", dim - 1),
            "d0".to_string(),
            format!("-f{}", dim - 1),
            format!("-d{}", current.z),
            "-d0".to_string(),
        ])
    } else if current.z == min_coord {
        Vec::from([
            format!("-r{}", dim - 1 - current.x),
            format!("f{}", dim - 1),
            format!("-r{}", dim - 1),
            format!("-f{}", dim - 1),
            format!("r{}", dim - 1 - current.x),
            format!("r{}", dim - 1),
        ])
    } else if current.x == min_coord {
        Vec::from([
            format!("-d{}", current.z),
            format!("f{}", dim - 1),
            format!("-d{}", dim - 1),
            format!("-f{}", dim - 1),
            format!("d{}", current.z),
            format!("d{}", dim - 1),
        ])
    } else {
        Vec::new()
    }
}

fn move_from_middle_to_back(
    current: &P3,
    max_coord: usize,
    min_coord: usize,
    dim: usize,
    white_side: usize,
) -> Vec<String> {
    if current.x == max_coord && current.z == max_coord {
        // Top Right
        if white_side == 1 {
            Vec::from([
                format!("d{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("-d{}", dim - 1),
            ])
        } else {
            Vec::from([
                "r0".to_string(),
                format!("-f{}", dim - 1),
                "-r0".to_string(),
            ])
        }
    } else if current.x == min_coord && current.z == max_coord {
        // Top Left
        if white_side == 1 {
            Vec::from([
                format!("-d{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("d{}", dim - 1),
            ])
        } else {
            Vec::from([
                format!("r{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("-r{}", dim - 1),
            ])
        }
    } else if current.x == max_coord && current.z == min_coord {
        // Bottom Right
        if white_side == 5 {
            Vec::from([
                "d0".to_string(),
                format!("-f{}", dim - 1),
                "-d0".to_string(),
            ])
        } else {
            Vec::from([
                "-r0".to_string(),
                format!("-f{}", dim - 1),
                "r0".to_string(),
            ])
        }
    } else if current.x == min_coord && current.z == min_coord {
        // Bottom Left
        if white_side == 1 {
            Vec::from([
                "-d0".to_string(),
                format!("-f{}", dim - 1),
                format!("-d{}", dim - 1),
            ])
        } else {
            Vec::from([
                format!("-r{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("r{}", dim - 1),
            ])
        }
    } else {
        Vec::new()
    }
}

fn flip_piece_on_back(current: &P3, max_coord: usize, min_coord: usize, dim: usize) -> Vec<String> {
    if current.z == max_coord {
        //top
        Vec::from([
            format!("-d{}", dim - 1),
            "r0".to_string(),
            format!("f{}", dim - 1),
            format!("f{}", dim - 1),
            "-r0".to_string(),
            format!("d{}", dim - 1),
            format!("f{}", dim - 1),
        ])
    } else if current.x == max_coord {
        //right
        Vec::from([
            "r0".to_string(),
            "d0".to_string(),
            format!("f{}", dim - 1),
            format!("f{}", dim - 1),
            "-d0".to_string(),
            "-r0".to_string(),
            format!("f{}", dim - 1),
        ])
    } else if current.z == min_coord {
        //down
        Vec::from([
            "d0".to_string(),
            format!("-r{}", dim - 1),
            format!("f{}", dim - 1),
            format!("f{}", dim - 1),
            format!("r{}", dim - 1),
            "-d0".to_string(),
            format!("f{}", dim - 1),
        ])
    } else if current.x == min_coord {
        //left
        Vec::from([
            format!("-r{}", dim - 1),
            format!("-d{}", dim - 1),
            format!("f{}", dim - 1),
            format!("f{}", dim - 1),
            format!("d{}", dim - 1),
            format!("r{}", dim - 1),
            format!("f{}", dim - 1),
        ])
    } else {
        Vec::new()
    }
}

fn solve_from_back_to_front(
    current: &P3,
    solved: &P3,
    max_coord: usize,
    min_coord: usize,
    dim: usize,
) -> Vec<String> {
    if current.z == max_coord && solved.z == max_coord {
        Vec::from([
            format!("-r{}", dim - 1 - current.x),
            "f0".to_string(),
            "-r0".to_string(),
            "-f0".to_string(),
            format!("r{}", dim - 1 - current.x),
            "f0".to_string(),
            "r0".to_string(),
            "-f0".to_string(),
        ])
    } else if current.x == max_coord && solved.x == max_coord {
        Vec::from([
            format!("-d{}", current.z),
            "f0".to_string(),
            "-d0".to_string(),
            "-f0".to_string(),
            format!("d{}", current.z),
            "f0".to_string(),
            "d0".to_string(),
            "-f0".to_string(),
        ])
    } else if current.z == min_coord && solved.z == min_coord {
        Vec::from([
            format!("r{}", dim - 1 - current.x),
            "f0".to_string(),
            format!("r{}", dim - 1),
            "-f0".to_string(),
            format!("-r{}", dim - 1 - current.x),
            "f0".to_string(),
            format!("-r{}", dim - 1),
            "-f0".to_string(),
        ])
    } else if current.x == min_coord && solved.x == min_coord {
        Vec::from([
            format!("d{}", current.z),
            "f0".to_string(),
            format!("d{}", dim - 1),
            "-f0".to_string(),
            format!("-d{}", current.z),
            "f0".to_string(),
            format!("-d{}", dim - 1),
            "-f0".to_string(),
        ])
    } else {
        Vec::new()
    }
}

#[allow(dead_code)]
pub fn solve_front_edge_segments_impl(
    current: &P3,
    solved: &P3,
    dim: usize,
    white_side: usize,
) -> Vec<String> {
    let back = 3;
    let max_coord = dim - 1;
    let min_coord = 0;

    if current.y == min_coord {
        extract_from_front(current, max_coord, min_coord, dim)
    } else if current.y > min_coord && current.y < max_coord {
        move_from_middle_to_back(current, max_coord, min_coord, dim, white_side)
    } else if current.y == max_coord && white_side == back {
        flip_piece_on_back(current, max_coord, min_coord, dim)
    } else if current.z == solved.z && current.x == solved.x {
        solve_from_back_to_front(current, solved, max_coord, min_coord, dim)
    } else {
        Vec::from([format!("-f{}", dim - 1)])
    }
}

#[allow(dead_code)]
pub fn solve_back_edge_segments_impl(
    current: &P3,
    solved: &P3,
    dim: usize,
    yellow_side: usize,
) -> Vec<String> {
    let up = 1;
    let right = 2;
    let left = 4;
    let down = 5;
    let max_coord = dim - 1;
    let min_coord = 0;

    // Solves top back edge
    if solved.z == max_coord {
        // top right middle edge
        if current.y < max_coord && current.z == max_coord && current.x == max_coord {
            //a
            // if yellow side is on the right face (red center)
            if yellow_side == right {
                //a.1
                Vec::from([
                    format!("r{}", dim - 1),
                    format!("-f{}", dim - 1),
                    format!("-r{}", dim - 1),
                    format!("-f{}", current.y),
                    format!("r{}", dim - 1),
                    format!("-f{}", dim - 1),
                    format!("-r{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("f{}", current.y),
                ])
            }
            // if yellow side is on the upward face (blue center)
            else {
                //a.2
                Vec::from([
                    "r0".to_string(),
                    format!("-f{}", dim - 1),
                    "-r0".to_string(),
                    format!("f{}", dim - 1),
                    format!("d{}", dim - 1),
                    format!("-f{}", dim - 1),
                    format!("-d{}", dim - 1),
                ])
            }
        }
        // top back edge (wrong position)
        else if current.z == solved.z && current.y == solved.y {
            // b
            Vec::from([
                format!("r{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("-r{}", dim - 1),
                format!("-f{}", dim - 1 - current.x),
                format!("r{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("-r{}", dim - 1),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1 - current.x),
            ])
        }
        // back edge that isn't top
        else if current.y == max_coord && !(current.z == max_coord) {
            // c
            Vec::from([
                format!("d{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("-d{}", dim - 1),
            ])
        }
        // middle edge that isn't top right (just places piece on back edges)
        else if current.y != max_coord {
            // d
            if current.z == max_coord && current.x == min_coord {
                Vec::from([
                    format!("r{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("-r{}", dim - 1),
                    format!("-f{}", dim - 1),
                ])
            } else if current.z == min_coord && current.x == min_coord {
                Vec::from([
                    format!("-r{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("r{}", dim - 1),
                    format!("-f{}", dim - 1),
                ])
            } else if current.z == min_coord && current.x == max_coord {
                Vec::from([
                    "-r0".to_string(),
                    format!("-f{}", dim - 1),
                    "r0".to_string(),
                    format!("f{}", dim - 1),
                ])
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
    // Solves right back edge
    else if solved.x == max_coord {
        if current.y < max_coord && current.z == min_coord && current.x == max_coord {
            //a
            // if yellow side is on the down face (green center)
            if yellow_side == down {
                //a.1
                Vec::from([
                    format!("d{}", dim - 1),
                    format!("-f{}", dim - 1),
                    format!("-d{}", dim - 1),
                    format!("-f{}", current.y),
                    format!("d{}", dim - 1),
                    format!("-f{}", dim - 1),
                    format!("-d{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("f{}", current.y),
                    "-r0".to_string(),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                    "r0".to_string(),
                ])
            }
            // if yellow side is on the right face (red center)
            else {
                //a.2
                Vec::from([
                    "d0".to_string(),
                    format!("-f{}", dim - 1),
                    "-d0".to_string(),
                    format!("f{}", dim - 1),
                    "-r0".to_string(),
                    format!("-f{}", dim - 1),
                    "r0".to_string(),
                    "r0".to_string(),
                    format!("f{}", dim - 1),
                    "-r0".to_string(),
                ])
            }
        }
        // b // change coords + small mod to algo
        else if current.x == solved.x && current.y == solved.y {
            // b
            Vec::from([
                format!("d{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("-d{}", dim - 1),
                format!("-f{}", current.z),
                format!("d{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("-d{}", dim - 1),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1),
                format!("f{}", current.z),
                "-r0".to_string(),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1),
                "r0".to_string(),
            ])
        }
        // c // hard code those instead of generalizing
        else if current.y == max_coord {
            if current.z == min_coord {
                Vec::from([
                    "-r0".to_string(),
                    format!("-f{}", dim - 1),
                    "r0".to_string(),
                    "r0".to_string(),
                    format!("f{}", dim - 1),
                    "-r0".to_string(),
                ])
            } else if current.x == min_coord {
                Vec::from([
                    "-r0".to_string(),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                    "r0".to_string(),
                    "r0".to_string(),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                    "-r0".to_string(),
                ])
            } else {
                Vec::new()
            }
        }
        // d // change coords
        else if current.y != max_coord {
            if current.x == max_coord && current.z == max_coord {
                Vec::from([
                    format!("-f{}", dim - 1),
                    "r0".to_string(),
                    format!("-f{}", dim - 1),
                    "-r0".to_string(),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                ])
            } else if current.x == min_coord && current.z == max_coord {
                Vec::from([
                    format!("r{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("-r{}", dim - 1),
                    format!("-f{}", dim - 1),
                ])
            } else if current.x == min_coord && current.z == min_coord {
                Vec::from([
                    "-d0".to_string(),
                    format!("-f{}", dim - 1),
                    "d0".to_string(),
                    format!("f{}", dim - 1),
                ])
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
    // Solves bottom back edge
    else if solved.z == min_coord {
        if current.y < max_coord && current.z == min_coord && current.x == min_coord {
            //a
            // if yellow side is on the left face (orange center)
            if yellow_side == left {
                //a.1
                Vec::from([
                    "-r0".to_string(),
                    format!("-f{}", dim - 1),
                    "r0".to_string(),
                    format!("-f{}", current.y),
                    "-r0".to_string(),
                    format!("-f{}", dim - 1),
                    "r0".to_string(),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("f{}", current.y),
                    "d0".to_string(),
                    format!("d{}", dim - 1),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                    "-d0".to_string(),
                    format!("-d{}", dim - 1),
                ])
            }
            // if yellow side is on the down face (green center)
            else {
                //a.2
                Vec::from([
                    format!("-r{}", dim - 1),
                    format!("-f{}", dim - 1),
                    format!("r{}", dim - 1),
                    format!("f{}", dim - 1),
                    "-d0".to_string(),
                    format!("-f{}", dim - 1),
                    "d0".to_string(),
                    "d0".to_string(),
                    format!("f{}", dim - 1),
                    "-d0".to_string(),
                    format!("f{}", dim - 1),
                    "-r0".to_string(),
                    format!("f{}", dim - 1),
                    "r0".to_string(),
                    format!("f{}", dim - 1),
                    format!("f{}", dim - 1),
                ])
            }
        }
        // b // change coords + small mod to algo
        else if current.z == solved.z && current.y == solved.y {
            // b
            Vec::from([
                "-r0".to_string(),
                format!("-f{}", dim - 1),
                "r0".to_string(),
                format!("-f{}", current.x),
                "-r0".to_string(),
                format!("-f{}", dim - 1),
                "r0".to_string(),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1),
                format!("f{}", current.x),
                "d0".to_string(),
                format!("d{}", dim - 1),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1),
                "-d0".to_string(),
                format!("-d{}", dim - 1),
            ])
        } else if current.y == max_coord {
            // c
            Vec::from([
                "-d0".to_string(),
                format!("-f{}", dim - 1),
                "d0".to_string(),
                "d0".to_string(),
                format!("f{}", dim - 1),
                "-d0".to_string(),
            ])
        } else if current.y < max_coord {
            Vec::from([
                "d0".to_string(),
                "r0".to_string(),
                format!("-d{}", dim - 1),
                format!("-f{}", dim - 1),
                format!("d{}", dim - 1),
                "-r0".to_string(),
                "-d0".to_string(),
            ])
        } else {
            Vec::new()
        }
        // permutations of the original algo should work
    } else if solved.x == min_coord {
        let displace = Vec::from([
            "r0".to_string(),
            format!("f{}", dim - 1),
            "-r0".to_string(),
            format!("-f{}", dim - 1),
            format!("-d{}", dim - 1),
            format!("-f{}", dim - 1),
            format!("d{}", dim - 1),
            format!("f{}", dim - 1),
        ]);
        let solve = Vec::from([
            "-d0".to_string(),
            format!("-f{}", dim - 1),
            "d0".to_string(),
            format!("-f{}", current.y),
            "-d0".to_string(),
            format!("-f{}", dim - 1),
            "d0".to_string(),
            format!("f{}", dim - 1),
            format!("f{}", dim - 1),
            format!("f{}", current.y),
            format!("-r{}", dim - 1),
            "-r0".to_string(),
            format!("f{}", dim - 1),
            format!("f{}", dim - 1),
            "r0".to_string(),
            format!("r{}", dim - 1),
        ]);
        let replace = Vec::from([
            format!("-f{}", dim - 1),
            format!("-d{}", dim - 1),
            format!("f{}", dim - 1),
            format!("d{}", dim - 1),
            format!("f{}", dim - 1),
            "r0".to_string(),
            format!("-f{}", dim - 1),
            "-r0".to_string(),
        ]);
        let flip = Vec::from([
            "r0".to_string(),
            format!("-f{}", dim - 1),
            "-r0".to_string(),
            format!("f{}", dim - 1),
            format!("d{}", dim - 1),
            format!("-f{}", dim - 1),
            format!("d{}", dim - 1),
            format!("d{}", dim - 1),
            format!("f{}", dim - 1),
            format!("d{}", dim - 1),
            format!("f{}", dim - 1),
            format!("r{}", dim - 1),
            format!("f{}", dim - 1),
            format!("-r{}", dim - 1),
            format!("f{}", dim - 1),
            format!("f{}", dim - 1),
        ]);

        if current.y < max_coord && current.z == max_coord && current.x == max_coord {
            //a
            // if yellow side is on the left face (orange center)
            if yellow_side == right {
                //a.1
                let mut res = Vec::new();
                res.extend(displace);
                res.extend(solve);
                res.extend(replace);
                res
            }
            // if yellow side is on the down face (green center)
            else {
                //a.2
                flip
            }
        } else if current.y < max_coord && current.z == max_coord && current.x == min_coord {
            //a
            if yellow_side == up {
                //a.1
                // TODO: rotate only f{dim-1} face
                let mut res = Vec::new();
                for i in 0..dim - 1 {
                    res.push(format!("f{}", i));
                }
                res.extend(displace);
                res.extend(solve);
                res.extend(replace);
                for i in 0..dim - 1 {
                    res.push(format!("-f{}", i));
                }
                res
            } else {
                //a.2
                // TODO: rotate only f{dim-1} face
                let mut res = Vec::new();
                for i in 0..dim - 1 {
                    res.push(format!("f{}", i));
                }
                res.extend(flip);
                for i in 0..dim - 1 {
                    res.push(format!("-f{}", i));
                }
                res
            }
        } else if current.y < max_coord && current.z == min_coord && current.x == min_coord {
            //a
            if yellow_side == left {
                //a.1
                // TODO: rotate only f{dim-1} face
                let mut res = Vec::new();
                for i in 0..dim - 1 {
                    res.push(format!("f{}", i));
                    res.push(format!("f{}", i));
                }
                res.extend(displace);
                res.extend(solve);
                res.extend(replace);
                for i in 0..dim - 1 {
                    res.push(format!("-f{}", i));
                    res.push(format!("-f{}", i));
                }
                res
            } else {
                //a.2
                // TODO: rotate only f{dim-1} face
                let mut res = Vec::new();
                for i in 0..dim - 1 {
                    res.push(format!("f{}", i));
                    res.push(format!("f{}", i));
                }
                res.extend(flip);
                for i in 0..dim - 1 {
                    res.push(format!("-f{}", i));
                    res.push(format!("-f{}", i));
                }
                res
            }
        } else if current.y < max_coord && current.z == min_coord && current.x == max_coord {
            //a
            if yellow_side == down {
                //a.1
                // TODO: rotate only f{dim-1} face
                let mut res = Vec::new();
                for i in 0..dim - 1 {
                    res.push(format!("-f{}", i));
                }
                res.extend(displace);
                res.extend(solve);
                res.extend(replace);
                for i in 0..dim - 1 {
                    res.push(format!("f{}", i));
                }
                res
            } else {
                //a.2
                // TODO: rotate only f{dim-1} face
                let mut res = Vec::new();
                for i in 0..dim - 1 {
                    res.push(format!("-f{}", i));
                }
                res.extend(flip);
                for i in 0..dim - 1 {
                    res.push(format!("f{}", i));
                }
                res
            }
        } else if current.x == solved.x && current.y == solved.y {
            // b
            let mut res = displace;
            res.extend(Vec::from([
                "-d0".to_string(),
                format!("-f{}", dim - 1),
                "d0".to_string(),
                format!("-f{}", dim - 1 - current.z),
                "-d0".to_string(),
                format!("-f{}", dim - 1),
                "d0".to_string(),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1 - current.z),
                format!("-r{}", dim - 1),
                "-r0".to_string(),
                format!("f{}", dim - 1),
                format!("f{}", dim - 1),
                format!("r{}", dim - 1),
                "r0".to_string(),
                format!("-r{}", dim - 1),
                "d0".to_string(),
                "r0".to_string(),
                format!("-f{}", dim - 1),
                "-r0".to_string(),
                "-d0".to_string(),
                format!("r{}", dim - 1),
            ]));
            res
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}

fn gen_swap_last_two_center_edges(center_edge: usize, dim: usize) -> Vec<String> {
    let mut res = Vec::new();
    for i in 0..center_edge {
        res.push(format!("f{}", dim - 1 - i));
        res.push(format!("f{}", dim - 1 - i));
    }
    res.extend(Vec::from([
        format!("r{}", dim - 1),
        format!("r{}", dim - 1),
        "d0".to_string(),
        "d0".to_string(),
    ]));
    for i in 0..center_edge {
        res.push(format!("f{}", dim - 1 - i));
        res.push(format!("f{}", dim - 1 - i));
    }
    res.extend(Vec::from([
        "d0".to_string(),
        "d0".to_string(),
        format!("r{}", dim - 1),
        format!("r{}", dim - 1),
    ]));
    for i in 0..center_edge {
        res.push(format!("f{}", dim - 1 - i));
        res.push(format!("f{}", dim - 1 - i));
    }

    res.push("d0".to_string());
    res.push("d0".to_string());
    res
}

fn gen_flip_last_center_edge(center_edge: usize, dim: usize) -> Vec<String> {
    let mut res = Vec::new();

    for i in 0..center_edge {
        res.push(format!("f{}", i));
    }

    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));
    for i in 0..center_edge {
        res.push(format!("f{}", i));
    }
    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));

    for i in 0..center_edge {
        res.push(format!("-f{}", i));
    }
    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));

    for i in 0..center_edge {
        res.push(format!("f{}", i));
    }
    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));
    for i in 0..center_edge {
        res.push(format!("f{}", dim - 1 - i));
    }
    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));
    for i in 0..center_edge {
        res.push(format!("f{}", i));
    }
    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));

    for i in 0..center_edge {
        res.push(format!("-f{}", i));
    }

    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));
    // TODO: Could it be removed?
    for i in 0..dim {
        res.push(format!("-f{}", i));
    }
    for i in 0..center_edge {
        res.push(format!("-f{}", i));
    }
    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));
    for i in 0..center_edge {
        res.push(format!("-f{}", i));
    }
    res.push(format!("r{}", dim - 1));
    res.push(format!("r{}", dim - 1));
    res.push(format!("f{}", center_edge));
    res.push(format!("r{}", dim - 1));
    res.push(format!("-f{}", dim - 1));
    res.push("d0".to_string());
    res
}

#[allow(dead_code)]
pub fn solve_middle_edge_segments_impl(current: &P3, solved: &P3, dim: usize) -> Vec<String> {
    let max_coord = dim - 1;
    let min_coord = 0;
    let center_edge = dim / 2;

    // where blue and orange meet
    let first_edge = solved.x == min_coord && solved.z == max_coord;
    // where blue and red meet
    let second_edge = solved.x == max_coord && solved.z == max_coord;
    // where green and red meet
    let third_edge = solved.x == max_coord && solved.z == min_coord;

    // flip edge segment in place
    let flip = Vec::from([
        "r0".to_string(),
        format!("d{}", dim - 1),
        format!("-f{}", dim - 1),
        "-r0".to_string(),
        format!("-d{}", dim - 1),
    ]);
    let flip2 = Vec::from([
        "d0".to_string(),
        "-r0".to_string(),
        format!("-f{}", dim - 1),
        "-d0".to_string(),
        "r0".to_string(),
    ]);
    let flip3 = Vec::from([
        format!("-r{}", dim - 1),
        "-d0".to_string(),
        format!("-f{}", dim - 1),
        format!("r{}", dim - 1),
        "d0".to_string(),
    ]);

    let flip_first_center_edge = Vec::from([
        format!("f{}", dim - 1 - center_edge),
        "r0".to_string(),
        format!("-f{}", dim - 1),
        "-r0".to_string(),
        format!("-d{}", dim - 1),
        "-r0".to_string(),
        format!("d{}", dim - 1),
        "r0".to_string(),
        format!("-f{}", dim - 1 - center_edge),
    ]);

    let flip_second_center_edge = Vec::from([
        format!("f{}", dim - 1 - center_edge),
        "d0".to_string(),
        format!("-f{}", dim - 1),
        "-d0".to_string(),
        "r0".to_string(),
        "-d0".to_string(),
        "-r0".to_string(),
        "d0".to_string(),
        format!("-f{}", dim - 1 - center_edge),
    ]);

    let flip_third_center_edge = Vec::from([
        format!("f{}", dim - 1 - center_edge),
        format!("-r{}", dim - 1),
        format!("-f{}", dim - 1),
        format!("r{}", dim - 1),
        "d0".to_string(),
        format!("r{}", dim - 1),
        "-d0".to_string(),
        format!("-r{}", dim - 1),
        format!("-f{}", dim - 1 - center_edge),
    ]);

    let swap_last_two_center_edges = gen_swap_last_two_center_edges(center_edge, dim);

    let flip_last_center_edge = gen_flip_last_center_edge(center_edge, dim);

    // solves piece diagonally to solved location
    fn solve_up(depth: usize, dim: usize) -> Vec<String> {
        Vec::from([
            "f0".to_string(),
            format!("f{}", depth),
            "r0".to_string(),
            format!("d{}", dim - 1),
            format!("-f{}", dim - 1),
            "-r0".to_string(),
            format!("-d{}", dim - 1),
            "-f0".to_string(),
            format!("-f{}", depth),
        ])
    }
    fn solve_down(depth: usize, dim: usize) -> Vec<String> {
        Vec::from([
            format!("f{}", dim - 1 - depth),
            format!("f{}", dim - 1),
            "r0".to_string(),
            format!("d{}", dim - 1),
            format!("-f{}", dim - 1),
            "-r0".to_string(),
            format!("-d{}", dim - 1),
            format!("-f{}", dim - 1 - depth),
            format!("-f{}", dim - 1),
        ])
    }
    fn solve_up2(depth: usize, dim: usize) -> Vec<String> {
        Vec::from([
            "f0".to_string(),
            format!("f{}", depth),
            "d0".to_string(),
            "-r0".to_string(),
            format!("-f{}", dim - 1),
            "-d0".to_string(),
            "r0".to_string(),
            "-f0".to_string(),
            format!("-f{}", depth),
        ])
    }
    fn solve_down2(depth: usize, dim: usize) -> Vec<String> {
        Vec::from([
            format!("f{}", dim - 1 - depth),
            format!("f{}", dim - 1),
            "d0".to_string(),
            "-r0".to_string(),
            format!("-f{}", dim - 1),
            "-d0".to_string(),
            "r0".to_string(),
            format!("-f{}", dim - 1 - depth),
            format!("-f{}", dim - 1),
        ])
    }
    fn solve_up3(depth: usize, dim: usize) -> Vec<String> {
        Vec::from([
            "f0".to_string(),
            format!("f{}", depth),
            format!("-r{}", dim - 1),
            "-d0".to_string(),
            format!("-f{}", dim - 1),
            format!("r{}", dim - 1),
            "d0".to_string(),
            "-f0".to_string(),
            format!("-f{}", depth),
        ])
    }
    fn solve_down3(depth: usize, dim: usize) -> Vec<String> {
        Vec::from([
            format!("f{}", dim - 1 - depth),
            format!("f{}", dim - 1),
            format!("-r{}", dim - 1),
            "-d0".to_string(),
            format!("-f{}", dim - 1),
            format!("r{}", dim - 1),
            "d0".to_string(),
            format!("-f{}", dim - 1 - depth),
            format!("-f{}", dim - 1),
        ])
    }

    fn solve_last_edge(depth: usize, dim: usize) -> Vec<String> {
        Vec::from([
            format!("f{}", dim - 1 - depth),
            "d0".to_string(),
            "d0".to_string(),
            format!("f{}", depth),
            format!("r{}", dim - 1),
            format!("r{}", dim - 1),
            format!("-f{}", depth),
            format!("r{}", dim - 1),
            format!("r{}", dim - 1),
            format!("f{}", dim - 1 - depth),
            format!("f{}", dim - 1 - depth),
            "d0".to_string(),
            "d0".to_string(),
            format!("-f{}", dim - 1 - depth),
            "d0".to_string(),
            "d0".to_string(),
            format!("f{}", dim - 1 - depth),
            "d0".to_string(),
            "d0".to_string(),
            format!("r{}", dim - 1),
            format!("r{}", dim - 1),
            format!("f{}", dim - 1 - depth),
            format!("f{}", dim - 1 - depth),
            format!("r{}", dim - 1),
            format!("r{}", dim - 1),
        ])
    }

    if dim > 4 && dim % 2 == 1 && current.y == center_edge {
        if first_edge {
            if current.x == min_coord && current.z == min_coord {
                Vec::from([
                    format!("-f{}", center_edge),
                    "d0".to_string(),
                    "d0".to_string(),
                    format!("f{}", center_edge),
                ])
            } else if current.x == max_coord && current.z == min_coord {
                ["d0".to_string(), "d0".to_string()].to_vec()
            } else if current.x == max_coord && current.z == max_coord {
                Vec::from([
                    format!("f{}", center_edge),
                    "r0".to_string(),
                    "r0".to_string(),
                    format!("-f{}", center_edge),
                ])
            } else {
                flip_first_center_edge
            }
        } else if second_edge {
            if current.x == min_coord && current.z == min_coord {
                ["d0".to_string(), "d0".to_string()].to_vec()
            } else if current.x == max_coord && current.z == min_coord {
                Vec::from([
                    format!("f{}", center_edge),
                    "d0".to_string(),
                    "d0".to_string(),
                    format!("-f{}", center_edge),
                ])
            } else {
                flip_second_center_edge
            }
        } else if third_edge {
            if current.x == max_coord && current.z == min_coord {
                flip_third_center_edge
            } else {
                swap_last_two_center_edges
            }
        } else {
            flip_last_center_edge
        }
    } else {
        if first_edge {
            if current.x == min_coord && current.z == max_coord {
                if current.y >= center_edge {
                    solve_down(dim - current.y - 1, dim)
                } else {
                    solve_up(current.y, dim)
                }
            } else if current.x == max_coord && current.z == max_coord {
                if current.y == solved.y {
                    flip
                } else {
                    if current.y >= center_edge {
                        solve_up(dim - current.y - 1, dim)
                    } else {
                        solve_down(current.y, dim)
                    }
                }
            } else if current.x == min_coord && current.z == min_coord {
                Vec::from([
                    "d0".to_string(),
                    "d0".to_string(),
                    "r0".to_string(),
                    "r0".to_string(),
                ])
            } else {
                Vec::from(["r0".to_string(), "r0".to_string()])
            }
        } else if second_edge {
            if current.x == max_coord && current.z == max_coord {
                if current.y >= center_edge {
                    solve_down2(dim - current.y - 1, dim)
                } else {
                    solve_up2(current.y, dim)
                }
            } else if current.x == max_coord && current.z == min_coord {
                if current.y == solved.y {
                    flip2
                } else {
                    if current.y >= center_edge {
                        solve_up2(dim - current.y - 1, dim)
                    } else {
                        solve_down2(current.y, dim)
                    }
                }
            } else {
                Vec::from(["d0".to_string(), "d0".to_string()])
            }
        } else if third_edge {
            if current.x == max_coord && current.z == min_coord {
                if current.y == (max_coord - 1) {
                    solve_down3(dim - current.y - 1, dim)
                } else {
                    solve_up3(current.y, dim)
                }
            } else if current.x == min_coord && current.z == min_coord {
                if current.y == solved.y {
                    flip3
                } else {
                    if current.y == (max_coord - 1) {
                        solve_up3(dim - current.y - 1, dim)
                    } else {
                        solve_down3(current.y, dim)
                    }
                }
            } else {
                Vec::new()
            }
        } else {
            solve_last_edge(current.y, dim)
        }
    }
}

pub fn create_actions(
    allowed_moves: &HashMap<String, Vec<i16>>,
) -> HashMap<String, Vec<(usize, usize)>> {
    let mut actions = HashMap::new();
    for (k, v) in allowed_moves.iter() {
        let mut action_k: Vec<(usize, usize)> = Vec::new();
        for (idx, i) in v.iter().enumerate() {
            if idx == *i as usize {
                continue;
            }
            action_k.push((idx, *i as usize));
        }
        actions.insert(k.clone(), action_k);
    }
    actions
}

pub fn apply_action(state: &Vec<usize>, action: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_state = state.clone();
    for (i, j) in action.iter() {
        new_state[*i] = state[*j];
    }
    new_state
}

fn calc_permutation_idx_four_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<u32, usize>,
    dim: usize,
    pos: usize,
) -> usize {
    let target_idx = target_idx_four_corner(dim, pos);
    let mut p = Vec::new();
    for i in 0..target_idx.len() {
        if cur_state[target_idx[i]] == sol_state[target_idx[0]] {
            p.push(0);
        } else if cur_state[target_idx[i]] == sol_state[target_idx[4]] {
            p.push(1);
        } else if cur_state[target_idx[i]] == sol_state[target_idx[8]] {
            p.push(2);
        } else {
            p.push(3);
        }
    }
    perm_map[&pack_perm_for_four_faces(&p)]
}

fn calc_permutation_idx_four_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<u32, usize>,
    dim: usize,
    y: usize,
    x: usize,
) -> usize {
    let target_idx = target_idx_four_edge(dim, y, x);
    let mut p = Vec::new();
    for i in 0..target_idx.len() {
        if cur_state[target_idx[i]] == sol_state[target_idx[0]] {
            p.push(0);
        } else if cur_state[target_idx[i]] == sol_state[target_idx[4]] {
            p.push(1);
        } else if cur_state[target_idx[i]] == sol_state[target_idx[8]] {
            p.push(2);
        } else {
            p.push(3);
        }
    }
    perm_map[&pack_perm_for_four_faces(&p)]
}

fn solve_four_faces_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_four_faces();
    let (prev_state, prev_action) = bfs_four_corner();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..=dim / 2 - 1).rev() {
        let perm_idx = calc_permutation_idx_four_corner(&state, sol_state, &perm_map, dim, pos);
        let mut state_idx = perm_idx;
        let moves = sequence_moves_four_corner(dim, pos);
        let mut action_idxes = Vec::new();
        while let Some(next_index) = prev_state[state_idx] {
            action_idxes.push(prev_action[state_idx].unwrap());
            state_idx = next_index;
        }
        for action_idx in action_idxes.iter() {
            let action = &moves[*action_idx];
            for act in action.iter().rev() {
                let inv_act = rev_move(act);
                if !actions.contains_key(inv_act.as_str()) {
                    println!("{}", inv_act);
                }
                let action = &actions[inv_act.as_str()];
                state = apply_action(&state, action);
                moves_str.push(inv_act);
            }
        }
    }
    (state, moves_str)
}

fn solve_four_faces_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_four_faces();
    let (prev_state, prev_action) = bfs_four_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let perm_idx = calc_permutation_idx_four_edge(&state, sol_state, &perm_map, dim, y, x);

            let mut state_idx = perm_idx;

            let moves = sequence_moves_four_edge(dim, y, x);

            let mut action_idxes = Vec::new();
            let mut qu = std::collections::VecDeque::new();
            while let Some(next_index) = prev_state[state_idx] {
                action_idxes.push(prev_action[state_idx].unwrap());
                state_idx = next_index;
                qu.push_back(state_idx);
            }
            for action_idx in action_idxes.iter() {
                let action = &moves[*action_idx];
                for act in action.iter().rev() {
                    let inv_act = if act.starts_with("-") {
                        act[1..].to_string()
                    } else {
                        format!("-{}", act)
                    };
                    if !actions.contains_key(inv_act.as_str()) {
                        println!("{}", inv_act);
                    }
                    let action = &actions[inv_act.as_str()];
                    state = apply_action(&state, action);
                    moves_str.push(inv_act);
                }
            }
        }
    }

    (state, moves_str)
}

// ====================================================

fn pack_perm_for_four_faces(perm: &Vec<usize>) -> u32 {
    let mut res = 0;
    for i in 0..perm.len() {
        res |= (perm[i] as u32) << (2 * i);
    }
    res
}

#[allow(dead_code)]
fn unpack_perm_for_four_faces(packed_perm: u32) -> Vec<usize> {
    let mut res = Vec::new();
    let mut p = packed_perm;
    for _ in 0..16 {
        res.push(p as usize % 4);
        p /= 4;
    }
    res
}

fn apply_action_to_packed_perm(packed_perm: u32, action: &Vec<usize>) -> u32 {
    let mut res = 0;
    for i in 0..16 {
        res |= ((packed_perm >> (2 * action[i] as u32)) % 4) << (2 * i);
    }
    res
}

#[allow(dead_code)]
pub fn target_idx_four_corner(dim: usize, pos: usize) -> Vec<usize> {
    target_idx_corner(dim, pos, &vec![0, 2, 4, 5])
}

#[allow(dead_code)]
pub fn target_idx_four_edge(dim: usize, pos_y: usize, pos_x: usize) -> Vec<usize> {
    target_idx_edge(dim, pos_y, pos_x, &vec![0, 2, 4, 5])
}

#[allow(dead_code)]
pub fn target_idx_corner(dim: usize, pos: usize, faces: &Vec<usize>) -> Vec<usize> {
    let rev_pos = dim - 1 - pos;
    let mut res = Vec::new();
    for f in faces {
        res.push(f * dim * dim + pos * dim + pos);
        res.push(f * dim * dim + pos * dim + rev_pos);
        res.push(f * dim * dim + rev_pos * dim + rev_pos);
        res.push(f * dim * dim + rev_pos * dim + pos);
    }
    res
}

#[allow(dead_code)]
pub fn target_idx_corner_distinct(dim: usize, pos: usize, faces: &Vec<usize>) -> Vec<usize> {
    let rev_pos = dim - 1 - pos;
    let mut res = Vec::new();
    for f in faces {
        res.push(f * dim * dim + pos * dim + pos);
        res.push(f * dim * dim + pos * dim + rev_pos);
        res.push(f * dim * dim + rev_pos * dim + rev_pos);
        res.push(f * dim * dim + rev_pos * dim + pos);
    }
    res
}

#[allow(dead_code)]
pub fn target_idx_edge(dim: usize, pos_y: usize, pos_x: usize, faces: &Vec<usize>) -> Vec<usize> {
    let rev_pos_x = dim - 1 - pos_x;
    let rev_pos_y = dim - 1 - pos_y;
    let mut res = Vec::new();
    for f in faces {
        res.push(f * dim * dim + pos_y * dim + pos_x);
        res.push(f * dim * dim + pos_x * dim + rev_pos_y);
        res.push(f * dim * dim + rev_pos_y * dim + rev_pos_x);
        res.push(f * dim * dim + rev_pos_x * dim + pos_y);
    }
    res
}

#[allow(dead_code)]
pub fn target_idx_edge_distinct(
    dim: usize,
    pos_y: usize,
    pos_x: usize,
    faces: &Vec<usize>,
) -> Vec<usize> {
    let rev_pos_x = dim - 1 - pos_x;
    let rev_pos_y = dim - 1 - pos_y;
    let mut res = Vec::new();
    for f in faces {
        res.push(f * dim * dim + pos_y * dim + pos_x);
        res.push(f * dim * dim + pos_x * dim + rev_pos_y);
        res.push(f * dim * dim + rev_pos_y * dim + rev_pos_x);
        res.push(f * dim * dim + rev_pos_x * dim + pos_y);
    }
    res
}

#[allow(dead_code)]

pub fn calc_face_rot(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    dim: usize,
    face: usize,
) -> usize {
    let pos = dim / 2 - 1;
    let rev_pos = dim - 1 - pos;
    let target_idx = [
        (face * dim + pos) * dim + pos,
        (face * dim + pos) * dim + rev_pos,
        (face * dim + rev_pos) * dim + rev_pos,
        (face * dim + rev_pos) * dim + pos,
    ];
    for i in 0..3 {
        if cur_state[target_idx[i]] == sol_state[target_idx[0]] {
            return i;
        }
    }
    3
}

#[allow(dead_code)]
pub fn sequence_moves_four_corner(dim: usize, pos: usize) -> Vec<Vec<String>> {
    let rev_pos = dim - 1 - pos;
    let f_rev_minus = format!("-f{}", rev_pos);
    let f_rev_plus = format!("f{}", rev_pos);
    let f_minus = format!("-f{}", pos);
    let f_plus = format!("f{}", pos);
    let l0_p = format!("-r{}", dim - 1);
    let l0_m = format!("r{}", dim - 1);
    let u0_p = format!("-d{}", dim - 1);
    let u0_m = format!("d{}", dim - 1);
    let mut res = Vec::from([
        Vec::from(["-r0".to_string()]),
        Vec::from(["r0".to_string()]),
        Vec::from([l0_m.clone()]),
        Vec::from([l0_p.clone()]),
        Vec::from(["-d0".to_string()]),
        Vec::from(["d0".to_string()]),
        Vec::from([u0_m.clone()]),
        Vec::from([u0_p.clone()]),
    ]);
    for (plus, minus) in &[
        ("r0".to_string(), "-r0".to_string()),
        ("d0".to_string(), "-d0".to_string()),
        (l0_p.clone(), l0_m.clone()),
        (u0_p.clone(), u0_m.clone()),
    ] {
        for (first, last) in &[
            (f_rev_minus.clone(), f_rev_plus.clone()),
            (f_minus.clone(), f_plus.clone()),
        ] {
            res.extend([
                Vec::from([first.clone(), plus.clone(), last.clone()]),
                Vec::from([first.clone(), plus.clone(), plus.clone(), last.clone()]),
                Vec::from([first.clone(), minus.clone(), last.clone()]),
            ])
        }
    }
    for (plus, minus) in &[
        ("r0".to_string(), "-r0".to_string()),
        ("d0".to_string(), "-d0".to_string()),
    ] {
        for (first, last) in &[
            (f_rev_minus.clone(), f_rev_plus.clone()),
            (f_minus.clone(), f_plus.clone()),
        ] {
            res.extend([
                Vec::from([
                    first.clone(),
                    first.clone(),
                    plus.clone(),
                    last.clone(),
                    last.clone(),
                ]),
                Vec::from([
                    first.clone(),
                    first.clone(),
                    plus.clone(),
                    plus.clone(),
                    last.clone(),
                    last.clone(),
                ]),
                Vec::from([
                    first.clone(),
                    first.clone(),
                    minus.clone(),
                    last.clone(),
                    last.clone(),
                ]),
            ])
        }
    }
    res
}

#[allow(dead_code)]
pub fn sequence_moves_four_edge(dim: usize, y: usize, x: usize) -> Vec<Vec<String>> {
    assert!(y != x);
    let rev_x = dim - 1 - x;
    let rev_y = dim - 1 - y;

    let f_rev_x_minus = format!("-f{}", rev_x);
    let f_rev_x_plus = format!("f{}", rev_x);
    let f_x_minus = format!("-f{}", x);
    let f_x_plus = format!("f{}", x);
    let f_rev_y_minus = format!("-f{}", rev_y);
    let f_rev_y_plus = format!("f{}", rev_y);
    let f_y_minus = format!("-f{}", y);
    let f_y_plus = format!("f{}", y);
    let r0_p = "r0".to_string();
    let d0_p = "d0".to_string();
    let l0_p = format!("-r{}", dim - 1);
    let u0_p = format!("-d{}", dim - 1);
    let mut res_plus = Vec::from([
        Vec::from([r0_p.clone()]),
        Vec::from([l0_p.clone()]),
        Vec::from([d0_p.clone()]),
        Vec::from([u0_p.clone()]),
    ]);
    for plus in &[r0_p.clone(), d0_p.clone(), l0_p.clone(), u0_p.clone()] {
        for (first, second, rot) in &[
            (f_rev_y_minus.clone(), f_rev_x_minus.clone(), plus.clone()),
            (f_rev_y_minus.clone(), f_x_minus.clone(), rev_move(plus)),
            (f_y_plus.clone(), f_x_plus.clone(), plus.clone()),
            (f_y_plus.clone(), f_rev_x_plus.clone(), rev_move(plus)),
            (f_y_minus.clone(), f_rev_x_minus.clone(), rev_move(plus)),
            (f_y_minus.clone(), f_x_minus.clone(), plus.clone()),
            (f_rev_y_plus.clone(), f_x_plus.clone(), rev_move(plus)),
            (f_rev_y_plus.clone(), f_rev_x_plus.clone(), plus.clone()),
        ] {
            res_plus.push(Vec::from([
                first.clone(),
                rot.clone(),
                second.clone(),
                rev_move(rot),
                rev_move(first),
                rot.clone(),
                rev_move(second),
            ]));
        }
    }
    for (plus0, plus1) in &[(r0_p.clone(), l0_p.clone()), (d0_p.clone(), u0_p.clone())] {
        for (first, second, rot0, rot1) in &[
            (
                f_rev_y_minus.clone(),
                f_rev_x_minus.clone(),
                plus0.clone(),
                plus1.clone(),
            ),
            (
                f_rev_y_minus.clone(),
                f_x_minus.clone(),
                rev_move(plus0),
                rev_move(plus1),
            ),
            (
                f_y_plus.clone(),
                f_x_plus.clone(),
                plus0.clone(),
                plus1.clone(),
            ),
            (
                f_y_plus.clone(),
                f_rev_x_plus.clone(),
                rev_move(plus0),
                rev_move(plus1),
            ),
            (
                f_y_minus.clone(),
                f_rev_x_minus.clone(),
                rev_move(plus0),
                rev_move(plus1),
            ),
            (
                f_y_minus.clone(),
                f_x_minus.clone(),
                plus0.clone(),
                plus1.clone(),
            ),
            (
                f_rev_y_plus.clone(),
                f_x_plus.clone(),
                rev_move(plus0),
                rev_move(plus1),
            ),
            (
                f_rev_y_plus.clone(),
                f_rev_x_plus.clone(),
                plus0.clone(),
                plus1.clone(),
            ),
        ] {
            res_plus.push(Vec::from([
                first.clone(),
                rot0.clone(),
                rot1.clone(),
                second.clone(),
                rev_move(rot0),
                rev_move(rot1),
                rev_move(first),
                rot0.clone(),
                rot1.clone(),
                rev_move(second),
            ]));
        }
    }
    for plus in &[r0_p.clone(), d0_p.clone(), l0_p.clone(), u0_p.clone()] {
        for (first, second, rot) in &[
            (f_rev_y_minus.clone(), f_rev_x_minus.clone(), plus.clone()),
            (f_y_plus.clone(), f_x_plus.clone(), plus.clone()),
            (f_y_minus.clone(), f_rev_x_minus.clone(), rev_move(plus)),
            (f_rev_y_plus.clone(), f_x_plus.clone(), rev_move(plus)),
        ] {
            res_plus.push(Vec::from([
                first.clone(),
                first.clone(),
                rot.clone(),
                second.clone(),
                second.clone(),
                rev_move(rot),
                rev_move(first),
                rev_move(first),
                rot.clone(),
                rev_move(second),
                rev_move(second),
            ]));
        }
    }
    for (plus0, plus1) in &[(r0_p.clone(), l0_p.clone()), (d0_p.clone(), u0_p.clone())] {
        for (first, second, rot0, rot1) in &[(
            f_rev_y_minus.clone(),
            f_rev_x_minus.clone(),
            plus0.clone(),
            plus1.clone(),
        )] {
            res_plus.push(Vec::from([
                first.clone(),
                first.clone(),
                rot0.clone(),
                rot1.clone(),
                second.clone(),
                second.clone(),
                rev_move(rot0),
                rev_move(rot1),
                rev_move(first),
                rev_move(first),
                rot0.clone(),
                rot1.clone(),
                rev_move(second),
                rev_move(second),
            ]));
        }
    }
    let mut res = Vec::new();
    for r in res_plus.iter() {
        res.push(r.clone());
        res.push(r.iter().map(|s| rev_move(s)).rev().collect());
    }
    res
}

#[allow(dead_code)]
pub fn generate_four_corner_move_for_bfs(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = create_actions(&allowed_moves);
    for pos in (1..=dim / 2 - 1).rev() {
        let target_idx = target_idx_four_corner(dim, pos);
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let mut cur_result = Vec::new();
        let sequences = sequence_moves_four_corner(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            for i in 1..sequence.len() {
                state = apply_action(&state, &actions[&sequence[i]]);
            }

            for face in &[1, 3] {
                for i in 1..dim - 1 {
                    for j in 1..dim - 1 {
                        assert!(state[face * dim * dim + i * dim + j] / (dim * dim) == *face);
                    }
                }
            }
            for face in &[0, 2, 4, 5] {
                for i in pos + 1..=dim / 2 {
                    let rev_i = dim - 1 - i;
                    assert!(state[face * dim * dim + i * dim + i] / (dim * dim) == *face);
                    assert!(state[face * dim * dim + i * dim + rev_i] / (dim * dim) == *face);
                    assert!(state[face * dim * dim + rev_i * dim + i] / (dim * dim) == *face);
                    assert!(state[face * dim * dim + rev_i * dim + rev_i] / (dim * dim) == *face);
                }
            }
            let mut cur_move = Vec::new();
            for idx in target_idx.iter() {
                cur_move.push(idx_map[&state[*idx]]);
            }
            cur_result.push((cur_move, sequence.len()));
        }
        if result.is_none() {
            result = Some(cur_result);
        } else {
            assert!(result == Some(cur_result));
        }
    }
    println!("[");
    for r in result.unwrap() {
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
}

#[allow(dead_code)]
pub fn generate_four_edge_move_for_bfs(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx = target_idx_four_edge(dim, y, x);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = sequence_moves_four_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                for i in 1..sequence.len() {
                    state = apply_action(&state, &actions[&sequence[i]]);
                }

                for face in &[1, 3] {
                    for i in 1..dim - 1 {
                        for j in 1..dim - 1 {
                            assert!(state[face * dim * dim + i * dim + j] / (dim * dim) == *face);
                        }
                    }
                }
                if sequence.len() >= 2 {
                    for face in &[0, 2, 4, 5] {
                        for i in 1..dim - 1 {
                            for j in 1..dim - 1 {
                                let idx = face * dim * dim + i * dim + j;
                                if idx_map.contains_key(&idx) {
                                    continue;
                                }
                                let idx1 = face * dim * dim + j * dim + dim - 1 - i;
                                let idx2 = face * dim * dim + (dim - 1 - i) * dim + dim - 1 - j;
                                let idx3 = face * dim * dim + (dim - 1 - j) * dim + i;
                                assert!(
                                    state[idx] == idx
                                        || state[idx1] == idx
                                        || state[idx2] == idx
                                        || state[idx3] == idx
                                );
                            }
                        }
                    }
                }
                let mut cur_move = Vec::new();
                for idx in target_idx.iter() {
                    cur_move.push(idx_map[&state[*idx]]);
                }
                cur_result.push((cur_move, sequence.len()));
            }
            if result.is_none() {
                result = Some(cur_result);
            } else {
                assert!(result == Some(cur_result));
            }
        }
    }
    println!("[");
    for r in result.unwrap() {
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
}

fn gen_perm_map_four_faces_impl() -> (Vec<u32>, HashMap<u32, usize>) {
    let mut perm_list = Vec::new();
    let mut perm_map = HashMap::new();

    for pos0 in (0..16).combinations(4) {
        let mut remain0 = Vec::new();
        for i in 0..16 {
            if !pos0.contains(&i) {
                remain0.push(i);
            }
        }
        for pos1 in remain0.iter().combinations(4) {
            let mut remain1 = Vec::new();
            for j in 0..16 {
                if !pos0.contains(&j) && !pos1.contains(&&j) {
                    remain1.push(j);
                }
            }
            for pos2 in remain1.iter().combinations(4) {
                let mut p = vec![3; 16];
                for pos in pos0.iter() {
                    p[*pos] = 0;
                }
                for pos in pos1.iter() {
                    p[**pos] = 1;
                }
                for pos in pos2.iter() {
                    p[**pos] = 2;
                }
                let p = pack_perm_for_four_faces(&p);
                perm_map.insert(p, perm_list.len());
                perm_list.push(p);
            }
        }
    }

    (perm_list, perm_map)
}

fn gen_perm_map_four_faces() -> (Vec<u32>, HashMap<u32, usize>) {
    PERM_FOUR_CORNER.clone()
}

fn bfs_four_corner_impl() -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    if std::path::Path::new("bfs_four_corner.json").exists() {
        // ファイルが存在する場合、ファイルからデータを読み込む
        let contents = std::fs::read_to_string("bfs_four_corner.json").unwrap();
        let deserialized: (Vec<Option<usize>>, Vec<Option<usize>>) =
            serde_json::from_str(&contents).unwrap();
        return deserialized;
    }
    let (perm_list, perm_map) = gen_perm_map_four_faces();
    let mut priority_qu = BinaryHeap::new();
    // generated by `generate_four_corner_move_for_bfs`
    let actions = [
        (
            Vec::from([0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 12]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15, 12, 13, 14]),
            1,
        ),
        (
            Vec::from([1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 5, 6, 15, 8, 9, 10, 11, 12, 13, 4, 14]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 15, 5, 6, 14, 8, 9, 10, 11, 12, 13, 7, 4]),
            4,
        ),
        (
            Vec::from([0, 1, 2, 3, 14, 5, 6, 4, 8, 9, 10, 11, 12, 13, 15, 7]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 13, 5, 7, 8, 9, 10, 11, 6, 12, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 12, 13, 7, 8, 9, 10, 11, 5, 6, 14, 15]),
            4,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 6, 12, 7, 8, 9, 10, 11, 13, 5, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 11, 9, 10, 13, 8, 12, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 13, 9, 10, 12, 11, 8, 14, 15]),
            4,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 12, 9, 10, 8, 13, 11, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 15, 9, 11, 12, 13, 10, 14]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 14, 15, 11, 12, 13, 9, 10]),
            4,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 14, 11, 12, 13, 15, 9]),
            3,
        ),
        (
            Vec::from([10, 0, 2, 3, 4, 5, 6, 7, 8, 1, 9, 11, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([9, 10, 2, 3, 4, 5, 6, 7, 8, 0, 1, 11, 12, 13, 14, 15]),
            4,
        ),
        (
            Vec::from([1, 9, 2, 3, 4, 5, 6, 7, 8, 10, 0, 11, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 8, 2, 4, 5, 6, 7, 11, 9, 10, 3, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 11, 8, 4, 5, 6, 7, 3, 9, 10, 2, 12, 13, 14, 15]),
            4,
        ),
        (
            Vec::from([0, 1, 3, 11, 4, 5, 6, 7, 2, 9, 10, 8, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 6, 2, 4, 3, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 5, 6, 4, 2, 3, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            4,
        ),
        (
            Vec::from([0, 1, 3, 5, 4, 6, 2, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([4, 0, 2, 3, 7, 5, 6, 1, 8, 9, 10, 11, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([7, 4, 2, 3, 1, 5, 6, 0, 8, 9, 10, 11, 12, 13, 14, 15]),
            4,
        ),
        (
            Vec::from([1, 7, 2, 3, 0, 5, 6, 4, 8, 9, 10, 11, 12, 13, 14, 15]),
            3,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 5, 6, 8, 11, 9, 10, 4, 12, 13, 14, 15]),
            5,
        ),
        (
            Vec::from([0, 1, 2, 3, 8, 5, 6, 11, 4, 9, 10, 7, 12, 13, 14, 15]),
            6,
        ),
        (
            Vec::from([0, 1, 2, 3, 11, 5, 6, 4, 7, 9, 10, 8, 12, 13, 14, 15]),
            5,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 10, 5, 7, 8, 6, 9, 11, 12, 13, 14, 15]),
            5,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 9, 10, 7, 8, 5, 6, 11, 12, 13, 14, 15]),
            6,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 6, 9, 7, 8, 10, 5, 11, 12, 13, 14, 15]),
            5,
        ),
        (
            Vec::from([13, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 12, 14, 15]),
            5,
        ),
        (
            Vec::from([12, 13, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1, 14, 15]),
            6,
        ),
        (
            Vec::from([1, 12, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 0, 14, 15]),
            5,
        ),
        (
            Vec::from([0, 1, 15, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 3, 14]),
            5,
        ),
        (
            Vec::from([0, 1, 14, 15, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 2, 3]),
            6,
        ),
        (
            Vec::from([0, 1, 3, 14, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 2]),
            5,
        ),
    ];
    let start =
        pack_perm_for_four_faces(&Vec::from([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3]));
    let mut prev_state: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; perm_list.len()];
    priority_qu.push(Reverse((0, start)));
    cost[perm_map[&start]] = 0;
    while let Some(Reverse((cur_cost, state))) = priority_qu.pop() {
        let idx = perm_map[&state];
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_action_to_packed_perm(state, &perm);
            let next_idx = perm_map[&next_state];
            if action_cost + c < cost[next_idx] {
                cost[next_idx] = action_cost + c;
                prev_state[next_idx] = Some(idx);
                prev_action[next_idx] = Some(action_idx);
                priority_qu.push(Reverse((action_cost + c, next_state)));
            }
        }
    }
    let mut reachable = 0;

    let mut max_cost = 0;
    let mut max_idx = 0;
    for (idx, c) in cost.iter().enumerate() {
        if *c == std::i32::MAX {
            continue;
        }
        reachable += 1;
        if *c > max_cost {
            max_cost = *c;
            max_idx = idx;
        }
    }
    println!("reachable: {}", reachable);
    println!("max_cost: {} {:?}", max_cost, perm_list[max_idx]);

    let res = (prev_state, prev_action);
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("bfs_four_corner.json", serialized).ok();

    res
}

#[allow(dead_code)]
pub fn bfs_four_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    BFS_FOUR_CORNER.clone()
}

fn bfs_four_edge_impl() -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    if std::path::Path::new("bfs_four_edge.json").exists() {
        // ファイルが存在する場合、ファイルからデータを読み込む
        let contents = std::fs::read_to_string("bfs_four_edge.json").unwrap();
        let deserialized: (Vec<Option<usize>>, Vec<Option<usize>>) =
            serde_json::from_str(&contents).unwrap();
        return deserialized;
    }
    let (perm_list, perm_map) = gen_perm_map_four_faces();
    let mut priority_qu = BinaryHeap::new();
    // generated by `generate_four_edge_move_for_bfs`
    let actions = [
        (
            Vec::from([0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 6, 7, 4, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 10, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15, 12, 13, 14]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 12]),
            1,
        ),
        (
            Vec::from([3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            1,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 4, 14, 6, 8, 9, 10, 11, 12, 13, 15, 5]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 15, 7, 4, 8, 9, 10, 11, 12, 13, 6, 14]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 14, 6, 7, 4, 8, 9, 10, 11, 12, 5, 13, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 13, 5, 6, 8, 9, 10, 11, 12, 14, 4, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 3, 7, 2, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 4, 2, 5, 6, 7, 3, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 7, 1, 3, 5, 6, 2, 4, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 2, 6, 3, 7, 4, 5, 1, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 6, 12, 4, 8, 9, 10, 11, 15, 13, 14, 7]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 4, 5, 15, 8, 9, 10, 11, 6, 13, 14, 12]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 12, 4, 5, 6, 8, 9, 10, 11, 13, 7, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 6, 7, 13, 8, 9, 10, 11, 4, 12, 14, 15]),
            7,
        ),
        (
            Vec::from([3, 1, 2, 5, 0, 6, 7, 4, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([4, 1, 2, 0, 7, 3, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 5, 2, 3, 7, 4, 0, 6, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([6, 0, 2, 3, 5, 1, 7, 4, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 14, 9, 10, 8, 15, 12, 13, 11]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 11, 9, 10, 15, 13, 14, 8, 12]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 14, 10, 13, 11, 15, 12]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 13, 15, 12, 10, 14]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 12, 5, 6, 4, 8, 9, 10, 11, 15, 7, 13, 14]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 5, 6, 13, 8, 9, 10, 11, 4, 14, 15, 12]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 12, 6, 8, 9, 10, 11, 13, 14, 15, 7]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 7, 15, 8, 9, 10, 11, 6, 12, 13, 14]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 12, 8, 10, 11, 13, 14, 15, 9]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 9, 15, 10, 11, 8, 12, 13, 14]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 10, 12, 11, 15, 9, 13, 14]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 13, 9, 11, 10, 14, 15, 12]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 14, 4, 6, 7, 8, 9, 10, 11, 13, 5, 15, 12]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 13, 6, 7, 8, 9, 10, 11, 15, 12, 4, 14]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 6, 14, 7, 8, 9, 10, 11, 15, 12, 13, 5]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 15, 5, 7, 8, 9, 10, 11, 13, 14, 6, 12]),
            7,
        ),
        (
            Vec::from([1, 11, 2, 3, 4, 5, 6, 7, 0, 8, 9, 10, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([8, 0, 2, 3, 4, 5, 6, 7, 9, 10, 11, 1, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([3, 1, 2, 11, 4, 5, 6, 7, 9, 10, 0, 8, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([10, 1, 2, 0, 4, 5, 6, 7, 11, 8, 9, 3, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 12, 10, 13, 9, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 9, 13, 11, 8, 10, 12, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 12, 10, 11, 8, 15, 13, 14, 9]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 11, 15, 9, 10, 8, 13, 14, 12]),
            7,
        ),
        (
            Vec::from([0, 9, 1, 3, 4, 5, 6, 7, 2, 10, 11, 8, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 2, 8, 3, 4, 5, 6, 7, 11, 1, 9, 10, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 3, 9, 4, 5, 6, 7, 11, 8, 2, 10, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 10, 2, 4, 5, 6, 7, 9, 3, 11, 8, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 14, 8, 12, 11, 13, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 11, 8, 9, 13, 12, 14, 10, 15]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 14, 8, 9, 10, 12, 13, 15, 11]),
            7,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 15, 12, 13, 8, 14]),
            7,
        ),
        (
            Vec::from([3, 5, 1, 2, 4, 6, 0, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([6, 2, 3, 0, 4, 1, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 2, 3, 5, 0, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([4, 0, 1, 2, 5, 3, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([3, 0, 1, 9, 4, 5, 6, 7, 8, 10, 2, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 2, 10, 0, 4, 5, 6, 7, 8, 3, 9, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 9, 3, 0, 4, 5, 6, 7, 2, 8, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([3, 0, 8, 2, 4, 5, 6, 7, 9, 1, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 7, 3, 0, 4, 5, 2, 6, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([3, 0, 6, 2, 4, 5, 7, 1, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([3, 0, 1, 7, 2, 5, 6, 4, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 2, 4, 0, 7, 5, 6, 3, 8, 9, 10, 11, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 2, 3, 11, 4, 5, 6, 7, 8, 9, 0, 10, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([10, 0, 1, 2, 4, 5, 6, 7, 8, 9, 11, 3, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([3, 11, 1, 2, 4, 5, 6, 7, 0, 9, 10, 8, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([8, 2, 3, 0, 4, 5, 6, 7, 11, 9, 10, 1, 12, 13, 14, 15]),
            7,
        ),
        (
            Vec::from([1, 11, 2, 3, 7, 4, 14, 6, 0, 8, 9, 10, 12, 13, 15, 5]),
            10,
        ),
        (
            Vec::from([8, 0, 2, 3, 5, 15, 7, 4, 9, 10, 11, 1, 12, 13, 6, 14]),
            10,
        ),
        (
            Vec::from([3, 1, 2, 11, 14, 6, 7, 4, 9, 10, 0, 8, 12, 5, 13, 15]),
            10,
        ),
        (
            Vec::from([10, 1, 2, 0, 7, 13, 5, 6, 11, 8, 9, 3, 12, 14, 4, 15]),
            10,
        ),
        (
            Vec::from([0, 1, 3, 7, 2, 4, 5, 6, 11, 8, 12, 10, 13, 9, 14, 15]),
            10,
        ),
        (
            Vec::from([0, 1, 4, 2, 5, 6, 7, 3, 9, 13, 11, 8, 10, 12, 14, 15]),
            10,
        ),
        (
            Vec::from([0, 7, 1, 3, 5, 6, 2, 4, 12, 10, 11, 8, 15, 13, 14, 9]),
            10,
        ),
        (
            Vec::from([0, 2, 6, 3, 7, 4, 5, 1, 11, 15, 9, 10, 8, 13, 14, 12]),
            10,
        ),
        (
            Vec::from([0, 9, 1, 3, 5, 6, 12, 4, 2, 10, 11, 8, 15, 13, 14, 7]),
            10,
        ),
        (
            Vec::from([0, 2, 8, 3, 7, 4, 5, 15, 11, 1, 9, 10, 6, 13, 14, 12]),
            10,
        ),
        (
            Vec::from([0, 1, 3, 9, 12, 4, 5, 6, 11, 8, 2, 10, 13, 7, 14, 15]),
            10,
        ),
        (
            Vec::from([0, 1, 10, 2, 5, 6, 7, 13, 9, 3, 11, 8, 4, 12, 14, 15]),
            10,
        ),
        (
            Vec::from([3, 1, 2, 5, 0, 6, 7, 4, 9, 10, 14, 8, 12, 11, 13, 15]),
            10,
        ),
        (
            Vec::from([4, 1, 2, 0, 7, 3, 5, 6, 11, 8, 9, 13, 12, 14, 10, 15]),
            10,
        ),
        (
            Vec::from([1, 5, 2, 3, 7, 4, 0, 6, 14, 8, 9, 10, 12, 13, 15, 11]),
            10,
        ),
        (
            Vec::from([6, 0, 2, 3, 5, 1, 7, 4, 9, 10, 11, 15, 12, 13, 8, 14]),
            10,
        ),
        (
            Vec::from([3, 5, 1, 2, 4, 6, 0, 7, 14, 9, 10, 8, 15, 12, 13, 11]),
            10,
        ),
        (
            Vec::from([6, 2, 3, 0, 4, 1, 5, 7, 11, 9, 10, 15, 13, 14, 8, 12]),
            10,
        ),
        (
            Vec::from([1, 2, 3, 5, 0, 4, 6, 7, 8, 9, 14, 10, 13, 11, 15, 12]),
            10,
        ),
        (
            Vec::from([4, 0, 1, 2, 5, 3, 6, 7, 8, 9, 11, 13, 15, 12, 10, 14]),
            10,
        ),
        (
            Vec::from([3, 0, 1, 9, 12, 5, 6, 4, 8, 10, 2, 11, 15, 7, 13, 14]),
            10,
        ),
        (
            Vec::from([1, 2, 10, 0, 7, 5, 6, 13, 8, 3, 9, 11, 4, 14, 15, 12]),
            10,
        ),
        (
            Vec::from([1, 9, 3, 0, 4, 5, 12, 6, 2, 8, 10, 11, 13, 14, 15, 7]),
            10,
        ),
        (
            Vec::from([3, 0, 8, 2, 4, 5, 7, 15, 9, 1, 10, 11, 6, 12, 13, 14]),
            10,
        ),
        (
            Vec::from([1, 7, 3, 0, 4, 5, 2, 6, 12, 8, 10, 11, 13, 14, 15, 9]),
            10,
        ),
        (
            Vec::from([3, 0, 6, 2, 4, 5, 7, 1, 9, 15, 10, 11, 8, 12, 13, 14]),
            10,
        ),
        (
            Vec::from([3, 0, 1, 7, 2, 5, 6, 4, 8, 10, 12, 11, 15, 9, 13, 14]),
            10,
        ),
        (
            Vec::from([1, 2, 4, 0, 7, 5, 6, 3, 8, 13, 9, 11, 10, 14, 15, 12]),
            10,
        ),
        (
            Vec::from([1, 2, 3, 11, 14, 4, 6, 7, 8, 9, 0, 10, 13, 5, 15, 12]),
            10,
        ),
        (
            Vec::from([10, 0, 1, 2, 5, 13, 6, 7, 8, 9, 11, 3, 15, 12, 4, 14]),
            10,
        ),
        (
            Vec::from([3, 11, 1, 2, 4, 6, 14, 7, 0, 9, 10, 8, 15, 12, 13, 5]),
            10,
        ),
        (
            Vec::from([8, 2, 3, 0, 4, 15, 5, 7, 11, 9, 10, 1, 13, 14, 6, 12]),
            10,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 4, 11, 6, 5, 9, 10, 8, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 8, 7, 4, 11, 9, 10, 6, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 9, 4, 5, 6, 8, 10, 7, 11, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 6, 7, 10, 8, 4, 9, 11, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 6, 9, 4, 7, 8, 10, 11, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 4, 5, 8, 9, 6, 10, 11, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 11, 6, 7, 4, 8, 9, 5, 10, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 10, 5, 6, 8, 9, 11, 4, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([1, 14, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 15, 12, 13, 0]),
            11,
        ),
        (
            Vec::from([15, 0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 1, 12]),
            11,
        ),
        (
            Vec::from([0, 1, 3, 12, 4, 5, 6, 7, 8, 9, 10, 11, 15, 2, 13, 14]),
            11,
        ),
        (
            Vec::from([0, 1, 13, 2, 4, 5, 6, 7, 8, 9, 10, 11, 3, 14, 15, 12]),
            11,
        ),
        (
            Vec::from([0, 12, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 2]),
            11,
        ),
        (
            Vec::from([0, 2, 15, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 12, 13, 14]),
            11,
        ),
        (
            Vec::from([3, 1, 2, 14, 4, 5, 6, 7, 8, 9, 10, 11, 13, 0, 15, 12]),
            11,
        ),
        (
            Vec::from([13, 1, 2, 0, 4, 5, 6, 7, 8, 9, 10, 11, 15, 12, 3, 14]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 6, 11, 7, 5, 8, 9, 10, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 8, 5, 7, 9, 10, 11, 6, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 9, 5, 6, 4, 11, 8, 7, 10, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 5, 6, 10, 9, 4, 11, 8, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 9, 6, 7, 10, 11, 8, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 4, 5, 7, 8, 11, 6, 9, 10, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 11, 4, 6, 7, 9, 10, 5, 8, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 10, 6, 7, 11, 8, 9, 4, 12, 13, 14, 15]),
            11,
        ),
        (
            Vec::from([3, 14, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 15, 0]),
            11,
        ),
        (
            Vec::from([15, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 1, 14]),
            11,
        ),
        (
            Vec::from([3, 0, 1, 12, 4, 5, 6, 7, 8, 9, 10, 11, 13, 2, 14, 15]),
            11,
        ),
        (
            Vec::from([1, 2, 13, 0, 4, 5, 6, 7, 8, 9, 10, 11, 3, 12, 14, 15]),
            11,
        ),
        (
            Vec::from([1, 12, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 15, 13, 14, 2]),
            11,
        ),
        (
            Vec::from([3, 0, 15, 2, 4, 5, 6, 7, 8, 9, 10, 11, 1, 13, 14, 12]),
            11,
        ),
        (
            Vec::from([1, 2, 3, 14, 4, 5, 6, 7, 8, 9, 10, 11, 12, 0, 13, 15]),
            11,
        ),
        (
            Vec::from([13, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14, 3, 15]),
            11,
        ),
        (
            Vec::from([0, 1, 2, 3, 7, 4, 5, 6, 11, 8, 9, 10, 12, 13, 14, 15]),
            14,
        ),
        (
            Vec::from([0, 1, 2, 3, 5, 6, 7, 4, 9, 10, 11, 8, 12, 13, 14, 15]),
            14,
        ),
        (
            Vec::from([3, 0, 1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 15, 12, 13, 14]),
            14,
        ),
        (
            Vec::from([1, 2, 3, 0, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 12]),
            14,
        ),
    ];
    let start =
        pack_perm_for_four_faces(&Vec::from([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3]));
    let mut prev_state: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; perm_list.len()];
    priority_qu.push(Reverse((0, start)));
    cost[perm_map[&start]] = 0;
    while let Some(Reverse((cur_cost, state))) = priority_qu.pop() {
        let idx = perm_map[&state];
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_action_to_packed_perm(state, &perm);
            let next_idx = perm_map[&next_state];
            if action_cost + c < cost[next_idx] {
                cost[next_idx] = action_cost + c;
                prev_state[next_idx] = Some(idx);
                prev_action[next_idx] = Some(action_idx);
                priority_qu.push(Reverse((action_cost + c, next_state)));
            }
        }
    }
    let mut reachable = 0;

    let mut max_cost = 0;
    let mut max_idx = 0;
    for (idx, c) in cost.iter().enumerate() {
        if *c == std::i32::MAX {
            continue;
        }
        reachable += 1;
        if *c > max_cost {
            max_cost = *c;
            max_idx = idx;
        }
    }
    println!("reachable: {}", reachable);
    println!("max_cost: {} {:?}", max_cost, perm_list[max_idx]);

    let res = (prev_state, prev_action);
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("bfs_four_edge.json", serialized).ok();

    res
}

#[allow(dead_code)]
pub fn bfs_four_edge() -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    BFS_FOUR_EDGE.clone()
}

// Reference
// * https://github.com/merpig/RubiksProgram

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct P3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl P3 {
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

pub fn solve_green_middle_impl(current: &P3, solved: &P3, dim: usize) -> Vec<String> {
    let current_side = p3_to_side(current, dim);
    let middle = dim / 2;
    let max = middle + 1;
    let min = middle - 1;
    let mut res: Vec<String> = Vec::new();

    // Temp 4x4 solver
    if dim % 2 == 0 {
        if current_side == 'D' {
            if solved.x == middle - 1 && solved.y == middle - 1 && solved.z == 0 {
                res.push("d0".to_string());
            } else if solved.x == middle && solved.y == middle - 1 && solved.z == 0 {
                res.push(format!("-f{}", current.y));
                res.push("r0".to_string());
                res.push("r0".to_string());
                res.push(format!("f{}", current.y));
            } else if solved.x == middle - 1 && solved.y == middle && solved.z == 0 {
                res.push(format!("-f{}", middle));
                res.push("-r0".to_string());
                res.push(format!("f{}", middle));
            } else {
                if solved.x == solved.y
                    || solved.x == (dim - 1) - solved.y
                    || (dim - 1) - solved.x == (dim - 1) - solved.y
                    || (dim - 1) - solved.x == solved.y
                {
                    if solved.y < middle && solved.x < middle {
                        res.push("d0".to_string());
                    } else if solved.x > middle && solved.y < middle {
                        res.push(format!("-f{}", current.y));
                        res.push("r0".to_string());
                        res.push("r0".to_string());
                        res.push(format!("f{}", current.y));
                    } else if solved.x < middle && solved.y > middle {
                        res.push(format!("-f{}", current.y));
                        res.push("r0".to_string());
                        res.push(format!("f{}", current.y));
                    }
                } else {
                    res.push("-r0".to_string());
                    res.push("-d0".to_string());
                    res.push(format!("f{}", dim - 1 - current.x));
                    res.push("d0".to_string());
                    res.push(format!("f{}", current.y));
                    res.push("-d0".to_string());
                    res.push(format!("-f{}", dim - 1 - current.x));
                    res.push("d0".to_string());
                    res.push(format!("-f{}", current.y));
                    res.push("r0".to_string());
                }
            }
        }

        if current_side == 'R' {
            if solved.x == middle - 1 && solved.y == middle - 1 && solved.z == 0 {
                res.push(format!("f{}", current.y));
                res.push("d0".to_string());
                res.push("d0".to_string());
                res.push(format!("-f{}", current.y));
            } else if solved.x == middle && solved.y == middle - 1 && solved.z == 0 {
                if current.y == middle && current.z == middle {
                    res.push("d0".to_string());
                    res.push(format!("f{}", middle));
                    res.push("-d0".to_string());
                    res.push(format!("-f{}", middle));
                } else {
                    res.push("r0".to_string());
                }
            } else if solved.x == middle - 1 && solved.y == middle && solved.z == 0 {
                if current.y == middle && current.z == middle - 1 {
                    res.push("r0".to_string());
                    res.push(format!("-f{}", middle));
                    res.push("-r0".to_string());
                    res.push(format!("f{}", middle));
                } else {
                    res.push("r0".to_string());
                }
            } else if solved.x == middle && solved.y == middle && solved.z == 0 {
                if current.y == middle - 1 && current.z == middle {
                    res.push(format!("-f{}", middle));
                    res.push("-r0".to_string());
                    res.push(format!("f{}", middle));
                    res.push("r0".to_string());
                    res.push(format!("-f{}", middle));
                    res.push("r0".to_string());
                    res.push(format!("f{}", middle));
                } else {
                    res.push("r0".to_string());
                }
            } else {
                if solved.x == solved.y
                    || solved.x == (dim - 1) - solved.y
                    || (dim - 1) - solved.x == (dim - 1) - solved.y
                    || (dim - 1) - solved.x == solved.y
                {
                    if current.y < middle && current.z < middle {
                        if solved.x < middle && solved.y < middle {
                            res.push(format!("f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("d0".to_string());
                        } else if solved.x > middle && solved.y < middle {
                            res.push("-d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("d0".to_string());
                            res.push("d0".to_string());
                        } else if solved.x < middle && solved.y > middle {
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("-r0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                        } else if solved.x > middle && solved.y > middle {
                            res.push("-r0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("r0".to_string());
                            res.push("r0".to_string());
                            res.push("d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.y));
                            res.push("r0".to_string());
                            res.push("r0".to_string());
                            res.push(format!("f{}", dim - 1 - current.y));
                        }
                    } else {
                        res.push("r0".to_string());
                    }
                } else {
                    if current.y == solved.y && current.z == solved.x {
                        res.push("-r0".to_string());
                        res.push("-d0".to_string());
                        res.push(format!("f{}", dim - 1 - current.z));
                        res.push("d0".to_string());
                        res.push(format!("f{}", current.y));
                        res.push("-d0".to_string());
                        res.push(format!("-f{}", dim - 1 - current.z));
                        res.push("d0".to_string());
                        res.push(format!("-f{}", current.y));
                        res.push("r0".to_string());
                    } else {
                        res.push("r0".to_string());
                    }
                }
            }
        }
    } else if dim % 2 == 1 {
        match current_side {
            'D' => {
                if solved.x >= min && solved.x <= max && solved.y >= min && solved.y <= max {
                    if solved.y == min {
                        if solved.x == min {
                            res.push("d0".to_string());
                        } else if solved.x == middle {
                            if current.x < middle {
                                res.push(format!("f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", current.y));
                                res.push("d0".to_string());
                            } else if current.x > middle {
                                res.push(format!("f{}", current.y));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", current.y));
                                res.push("-d0".to_string());
                            } else {
                                res.push(format!("-f{}", current.y));
                                res.push("-r0".to_string());
                                res.push(format!("f{}", current.y));
                            }
                        } else {
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("-d0".to_string());
                        }
                    } else if solved.y == middle {
                        if solved.x == min {
                            if current.y == middle {
                                res.push(format!("f{}", min));
                                res.push("d0".to_string());
                                res.push("d0".to_string());
                                res.push(format!("-f{}", min));
                            } else {
                                res.push(format!("f{}", min));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", min));
                            }
                        } else {
                            res.push(format!("-f{}", current.y));
                            res.push("r0".to_string());
                            res.push(format!("f{}", current.y));
                        }
                    } else {
                        if solved.x == min {
                            res.push(format!("-f{}", current.y));
                            res.push("r0".to_string());
                            res.push(format!("f{}", current.y));
                        }
                    }
                } else {
                    if solved.x == solved.y
                        || solved.x == (dim - 1) - solved.y
                        || (dim - 1) - solved.x == (dim - 1) - solved.y
                        || (dim - 1) - solved.x == solved.y
                    {
                        if solved.y < middle && solved.x < middle {
                            res.push("d0".to_string());
                        } else if solved.x > middle && solved.y < middle {
                            res.push(format!("-f{}", current.y));
                            res.push("r0".to_string());
                            res.push(format!("f{}", current.y));
                        } else if solved.x < middle && solved.y > middle {
                            res.push(format!("-f{}", current.y));
                            res.push("r0".to_string());
                            res.push(format!("f{}", current.y));
                        }
                    } else {
                        res.push("-r0".to_string());
                        res.push("-d0".to_string());
                        res.push(format!("f{}", dim - 1 - current.x));
                        res.push("d0".to_string());
                        res.push(format!("f{}", current.y));
                        res.push("-d0".to_string());
                        res.push(format!("-f{}", dim - 1 - current.x));
                        res.push("d0".to_string());
                        res.push(format!("-f{}", current.y));
                        res.push("r0".to_string());
                    }
                }
            }
            'R' => {
                if solved.x >= middle - 1
                    && solved.x <= middle + 1
                    && solved.y >= middle - 1
                    && solved.y <= middle + 1
                {
                    if solved.y == min {
                        if solved.x == min {
                            res.push(format!("f{}", current.y));
                            res.push("d0".to_string());
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                        } else if solved.x == middle {
                            if current.z == max {
                                res.push("d0".to_string());
                                res.push(format!("f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", current.y));
                            } else {
                                res.push("r0".to_string());
                            }
                        } else {
                            if current.y == max && current.z == max {
                                res.push("d0".to_string());
                                res.push(format!("f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", current.y));
                            } else {
                                res.push("r0".to_string());
                            }
                        }
                    } else if solved.y == middle {
                        if solved.x == min {
                            if current.y == max {
                                res.push(format!("-f{}", middle));
                                res.push("r0".to_string());
                                res.push(format!("f{}", middle));
                            } else {
                                res.push("-r0".to_string());
                            }
                        } else {
                            if current.z == min {
                                res.push(format!("f{}", min));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", min));
                                res.push("r0".to_string());
                                res.push(format!("f{}", min));
                                res.push("d0".to_string());
                                res.push(format!("-f{}", min));
                            } else {
                                res.push("-r0".to_string());
                            }
                        }
                    } else {
                        if solved.x == min {
                            if current.z == min && current.y == min {
                                res.push(format!("-f{}", max));
                                res.push("-r0".to_string());
                                res.push(format!("f{}", max));
                            } else {
                                res.push("r0".to_string());
                            }
                        } else if solved.x == middle {
                            if current.y == max {
                                res.push(format!("-f{}", current.y));
                                res.push("-r0".to_string());
                                res.push(format!("f{}", current.y));
                                res.push("-r0".to_string());
                                res.push(format!("-f{}", current.y));
                                res.push("r0".to_string());
                                res.push(format!("f{}", current.y));
                            } else {
                                res.push("r0".to_string());
                            }
                        } else {
                            if current.y == max && current.z == min {
                                res.push(format!("f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("f{}", current.y));
                                res.push("d0".to_string());
                                res.push("d0".to_string());
                                res.push(format!("-f{}", current.y));
                            } else {
                                res.push("r0".to_string());
                            }
                        }
                    }
                } else {
                    if solved.x == solved.y
                        || solved.x == (dim - 1) - solved.y
                        || (dim - 1) - solved.x == (dim - 1) - solved.y
                        || (dim - 1) - solved.x == solved.y
                    {
                        if current.y < middle && current.z < middle {
                            if solved.x < middle && solved.y < middle {
                                res.push(format!("f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", current.y));
                                res.push("d0".to_string());
                            } else if solved.x > middle && solved.y < middle {
                                res.push("-d0".to_string());
                                res.push(format!("f{}", current.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", current.y));
                                res.push("d0".to_string());
                                res.push("d0".to_string());
                            } else if solved.x < middle && solved.y > middle {
                                res.push(format!("-f{}", dim - 1 - current.y));
                                res.push("-r0".to_string());
                                res.push(format!("f{}", dim - 1 - current.y));
                            } else if solved.x > middle && solved.y > middle {
                                res.push("-r0".to_string());
                                res.push(format!("f{}", dim - 1 - current.y));
                                res.push("-d0".to_string());
                                res.push(format!("-f{}", dim - 1 - current.y));
                                res.push("r0".to_string());
                                res.push("r0".to_string());
                                res.push("d0".to_string());
                                res.push(format!("-f{}", dim - 1 - current.y));
                                res.push("r0".to_string());
                                res.push("r0".to_string());
                                res.push(format!("f{}", dim - 1 - current.y));
                            }
                        } else {
                            res.push("r0".to_string());
                        }
                    } else {
                        if current.y == solved.y && current.z == solved.x {
                            res.push("-r0".to_string());
                            res.push("-d0".to_string());
                            res.push(format!("f{}", dim - 1 - current.z));
                            res.push("d0".to_string());
                            res.push(format!("f{}", current.y));
                            res.push("-d0".to_string());
                            res.push(format!("-f{}", dim - 1 - current.z));
                            res.push("d0".to_string());
                            res.push(format!("-f{}", current.y));
                            res.push("r0".to_string());
                        } else {
                            res.push("r0".to_string());
                        }
                    }
                }
            }
            _ => {
                panic!("Woops something broke. Only red and green should be unsolved.");
            }
        }
    }
    res
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

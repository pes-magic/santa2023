use std::collections::HashMap;

mod cube_moves;

fn generate_move_for_bfs_up_corner(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for pos in (1..=dim / 2 - 1).rev() {
        let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![0, 2, 4, 5]);
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let mut cur_result = Vec::new();
        let sequences = cube_moves::sequence_moves_four_corner(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            let mut rot = 0;
            let plus_rot = format!("-d{}", dim - 1);
            let minus_rot = format!("d{}", dim - 1);
            for i in 1..sequence.len() {
                state = cube_moves::apply_action(&state, &actions[&sequence[i]]);
            }
            for seq in sequence.iter() {
                if *seq == plus_rot {
                    rot = (rot + 1) % 4;
                } else if *seq == minus_rot {
                    rot = (rot + 3) % 4;
                }
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
            cur_result.push((cur_move, rot, sequence.len()));
        }
        if result.is_none() {
            result = Some(cur_result);
        } else {
            assert!(result == Some(cur_result));
        }
    }
    println!("[");
    let res = result.unwrap();
    for r in &res {
        println!("(Vec::from({:?}), {}, {}),", r.0, r.1, r.2)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("move_for_bfs_up_corner.json", serialized).ok();
}

fn generate_move_for_bfs_up_edge(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = cube_moves::sequence_moves_four_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                let mut rot = 0;
                let plus_rot = format!("-d{}", dim - 1);
                let minus_rot = format!("d{}", dim - 1);

                for i in 1..sequence.len() {
                    state = cube_moves::apply_action(&state, &actions[&sequence[i]]);
                }
                for seq in sequence.iter() {
                    if *seq == plus_rot {
                        rot = (rot + 1) % 4;
                    } else if *seq == minus_rot {
                        rot = (rot + 3) % 4;
                    }
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
                cur_result.push((cur_move, rot, sequence.len()));
            }
            if result.is_none() {
                result = Some(cur_result);
            } else {
                assert!(result == Some(cur_result));
            }
        }
    }
    let res = result.unwrap();
    println!("[");
    for r in &res {
        println!("(Vec::from({:?}), {}, {}),", r.0, r.1, r.2)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("move_for_bfs_up_edge.json", serialized).ok();
}

fn generate_sequence_filter_for_left_corner() -> Vec<usize> {
    [
        0, 1, 2, 3, 4, 5, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 32, 33, 34, 35, 36, 37,
    ]
    .to_vec()
    // let contents = std::fs::read_to_string("move_for_bfs_up_corner.json").unwrap();
    // let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    // let mut res = Vec::new();
    // for (idx, action) in actions.iter().enumerate() {
    //     if action.0[0] == 0 && action.0[1] == 1 && action.0[2] == 2 && action.0[3] == 3 {
    //         res.push(idx);
    //     }
    // }
    // println!("left_corner_filter: {:?}", res);
    // res
}
fn generate_sequence_filter_for_left_edge() -> Vec<usize> {
    [
        0, 1, 2, 3, 4, 5, 8, 9, 10, 11, 16, 17, 18, 19, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34,
        35, 36, 37, 38, 39, 44, 45, 46, 47, 52, 53, 54, 55, 104, 105, 106, 107, 108, 109, 110, 111,
        120, 121, 122, 123, 124, 125, 126, 127, 136, 137,
    ]
    .to_vec()
    // let contents = std::fs::read_to_string("move_for_bfs_up_edge.json").unwrap();
    // let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    // let mut res = Vec::new();
    // for (idx, action) in actions.iter().enumerate() {
    //     if action.0[0] == 0 && action.0[1] == 1 && action.0[2] == 2 && action.0[3] == 3 {
    //         res.push(idx);
    //     }
    // }
    // println!("left_edge_filter: {:?}", res);
    // res
}

fn sequence_moves_left_corner(dim: usize, pos: usize) -> Vec<Vec<String>> {
    let base = cube_moves::sequence_moves_four_corner(dim, pos);
    let valid_idxes = generate_sequence_filter_for_left_corner();
    let mut res = Vec::new();
    for idx in valid_idxes {
        res.push(base[idx].clone());
    }
    if pos == 1 {
        println!("left_corner_filter: {:?}", res);
    }
    res
}

fn sequence_moves_left_edge(dim: usize, y: usize, x: usize) -> Vec<Vec<String>> {
    let base = cube_moves::sequence_moves_four_edge(dim, y, x);
    let valid_idxes = generate_sequence_filter_for_left_edge();
    let mut res = Vec::new();
    for idx in valid_idxes {
        res.push(base[idx].clone());
    }
    if y == 1 && x == 2 {
        println!("left_edge_filter: {:?}", res);
    }
    res
}

fn generate_move_for_bfs_left_corner(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for pos in (1..=dim / 2 - 1).rev() {
        let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![2, 4, 5]);
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let mut cur_result = Vec::new();
        let sequences = sequence_moves_left_corner(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            let mut rot = 0;
            let plus_rot = format!("-r{}", dim - 1);
            let minus_rot = format!("r{}", dim - 1);
            for i in 1..sequence.len() {
                state = cube_moves::apply_action(&state, &actions[&sequence[i]]);
            }
            for seq in sequence.iter() {
                if *seq == plus_rot {
                    rot = (rot + 1) % 4;
                } else if *seq == minus_rot {
                    rot = (rot + 3) % 4;
                }
            }

            for face in &[0, 1, 3] {
                for i in 1..dim - 1 {
                    for j in 1..dim - 1 {
                        assert!(state[face * dim * dim + i * dim + j] / (dim * dim) == *face);
                    }
                }
            }
            for face in &[2, 4, 5] {
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
            cur_result.push((cur_move, rot, sequence.len()));
        }
        if result.is_none() {
            result = Some(cur_result);
        } else {
            assert!(result == Some(cur_result));
        }
    }
    println!("[");
    let res = result.unwrap();
    for r in &res {
        println!("(Vec::from({:?}), {}, {}),", r.0, r.1, r.2)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("move_for_bfs_left_corner.json", serialized).ok();
}

fn generate_move_for_bfs_left_edge(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![2, 4, 5]);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = sequence_moves_left_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                let mut rot = 0;
                let plus_rot = format!("-r{}", dim - 1);
                let minus_rot = format!("r{}", dim - 1);

                for i in 1..sequence.len() {
                    state = cube_moves::apply_action(&state, &actions[&sequence[i]]);
                }
                for seq in sequence.iter() {
                    if *seq == plus_rot {
                        rot = (rot + 1) % 4;
                    } else if *seq == minus_rot {
                        rot = (rot + 3) % 4;
                    }
                }

                for face in &[0, 1, 3] {
                    for i in 1..dim - 1 {
                        for j in 1..dim - 1 {
                            assert!(state[face * dim * dim + i * dim + j] / (dim * dim) == *face);
                        }
                    }
                }
                if sequence.len() >= 2 {
                    for face in &[2, 4, 5] {
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
                cur_result.push((cur_move, rot, sequence.len()));
            }
            if result.is_none() {
                result = Some(cur_result);
            } else {
                assert!(result == Some(cur_result));
            }
        }
    }
    let res = result.unwrap();
    println!("[");
    for r in &res {
        println!("(Vec::from({:?}), {}, {}),", r.0, r.1, r.2)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("move_for_bfs_left_edge.json", serialized).ok();
}

fn main() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_33/33/33"];
    generate_move_for_bfs_up_corner(&info, 33);
    generate_move_for_bfs_up_edge(&info, 33);
    generate_move_for_bfs_left_corner(&info, 33);
    generate_move_for_bfs_left_edge(&info, 33);
}

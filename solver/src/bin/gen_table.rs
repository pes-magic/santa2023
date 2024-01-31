use std::collections::HashMap;

use itertools::Itertools;

mod cube_moves;

fn generate_move_for_bfs_front_corner(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for pos in (1..=dim / 2 - 1).rev() {
        let target_idx =
            cube_moves::target_idx_corner_distinct(dim, pos, &(0..6).collect::<Vec<usize>>());
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let mut cur_result = Vec::new();
        let sequences = cube_moves::sequence_moves_first_two_corner(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            let mut rot = 0;
            let plus_rot = "f0".to_string();
            let minus_rot = "-f0".to_string();
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

            for face in 0..6 {
                for i in pos + 1..=dim / 2 {
                    let rev_i = dim - 1 - i;
                    assert_eq!(state[face * dim * dim + i * dim + i] / (dim * dim), face);
                    assert_eq!(
                        state[face * dim * dim + i * dim + rev_i] / (dim * dim),
                        face
                    );
                    assert_eq!(
                        state[face * dim * dim + rev_i * dim + i] / (dim * dim),
                        face
                    );
                    assert_eq!(
                        state[face * dim * dim + rev_i * dim + rev_i] / (dim * dim),
                        face
                    );
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
    std::fs::write("move_for_bfs_front_corner.json", serialized).ok();
}

fn generate_move_for_bfs_front_edge(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx =
                cube_moves::target_idx_edge_distinct(dim, y, x, &(0..6).collect::<Vec<usize>>());
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = cube_moves::sequence_moves_first_two_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                let mut rot = 0;
                let plus_rot = "f0".to_string();
                let minus_rot = "-f0".to_string();

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

                if sequence.len() >= 2 {
                    for face in 0..6 {
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
    std::fs::write("move_for_bfs_front_edge.json", serialized).ok();
}

fn generate_move_for_bfs_front_edge_diff(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 2 - y {
            let target_idx =
                cube_moves::target_idx_edge_distinct(dim, y, x, &(0..6).collect::<Vec<usize>>());
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let target_idx_dif =
                cube_moves::target_idx_edge_distinct(dim, y, x + 1, &(0..6).collect());
            let mut idx_map_dif = std::collections::HashMap::new();
            for i in 0..target_idx_dif.len() {
                idx_map_dif.insert(target_idx_dif[i], i);
            }

            let mut cur_result = Vec::new();
            let sequences = cube_moves::sequence_moves_first_two_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                let mut rot = 0;
                let plus_rot = "f0".to_string();
                let minus_rot = "-f0".to_string();

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

                if sequence.len() >= 2 {
                    for face in 0..6 {
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
                for idx in target_idx_dif.iter() {
                    cur_move.push(idx_map_dif[&state[*idx]]);
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
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("move_for_bfs_front_edge_diff.json", serialized).ok();
}

fn generate_sequence_filter_for_back_corner() -> Vec<usize> {
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 76, 77, 78, 79, 80, 81, 90, 91, 92,
        93, 94, 95, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 126, 127, 128, 129,
        130, 131,
    ]
    .to_vec()
    // let contents = std::fs::read_to_string("move_for_bfs_front_corner.json").unwrap();
    // let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    // let mut res = Vec::new();
    // for (idx, action) in actions.iter().enumerate() {
    //     if action.0[4] == 4 && action.0[5] == 5 && action.0[6] == 6 && action.0[7] == 7 {
    //         res.push(idx);
    //     }
    // }
    // println!("back_corner_filter: {:?}", res);
    // res
}

fn generate_sequence_filter_for_back_edge() -> Vec<usize> {
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93,
        94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130,
        131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 148, 149, 150,
        151, 156, 157, 158, 159, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
        177, 178, 179, 184, 185, 186, 187, 192, 193, 194, 195, 244, 245, 246, 247, 248, 249, 250,
        251, 260, 261, 262, 263, 264, 265, 266, 267, 276, 277, 282, 283, 284, 285, 286, 287, 304,
        305, 306, 307, 312, 313, 314, 315, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330,
        331, 332, 333, 334, 335, 340, 341, 342, 343, 348, 349, 350, 351, 392, 393, 394, 395, 396,
        397, 398, 399, 408, 409, 410, 411, 412, 413, 414, 415, 418, 419,
    ]
    .to_vec()
    // let contents = std::fs::read_to_string("move_for_bfs_front_edge.json").unwrap();
    // let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    // let mut res = Vec::new();
    // for (idx, action) in actions.iter().enumerate() {
    //     if action.0[4] == 4 && action.0[5] == 5 && action.0[6] == 6 && action.0[7] == 7 {
    //         res.push(idx);
    //     }
    // }
    // println!("back_edge_filter: {:?}", res);
    // res
}

fn sequence_moves_back_corner(dim: usize, pos: usize) -> Vec<Vec<String>> {
    let base = cube_moves::sequence_moves_first_two_corner(dim, pos);
    let valid_idxes = generate_sequence_filter_for_back_corner();
    let mut res = Vec::new();
    for idx in valid_idxes {
        res.push(base[idx].clone());
    }
    res
}

fn sequence_moves_back_edge(dim: usize, y: usize, x: usize) -> Vec<Vec<String>> {
    let base = cube_moves::sequence_moves_first_two_edge(dim, y, x);
    let valid_idxes = generate_sequence_filter_for_back_edge();
    let mut res = Vec::new();
    for idx in valid_idxes {
        res.push(base[idx].clone());
    }
    res
}

fn generate_move_for_bfs_back_corner(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for pos in (1..=dim / 2 - 1).rev() {
        let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![0, 2, 3, 4, 5]);
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let mut cur_result = Vec::new();
        let sequences = sequence_moves_back_corner(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            let mut rot = 0;
            let plus_rot = format!("-f{}", dim - 1);
            let minus_rot = format!("f{}", dim - 1);
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

            for face in &[1] {
                for i in 1..dim - 1 {
                    for j in 1..dim - 1 {
                        assert_eq!(state[face * dim * dim + i * dim + j] / (dim * dim), *face);
                    }
                }
            }
            for face in &[0, 2, 3, 4, 5] {
                for i in pos + 1..=dim / 2 {
                    let rev_i = dim - 1 - i;
                    assert_eq!(state[face * dim * dim + i * dim + i] / (dim * dim), *face);
                    assert_eq!(
                        state[face * dim * dim + i * dim + rev_i] / (dim * dim),
                        *face
                    );
                    assert_eq!(
                        state[face * dim * dim + rev_i * dim + i] / (dim * dim),
                        *face
                    );
                    assert_eq!(
                        state[face * dim * dim + rev_i * dim + rev_i] / (dim * dim),
                        *face
                    );
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
    std::fs::write("move_for_bfs_back_corner.json", serialized).ok();
}

fn generate_move_for_bfs_back_edge(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 3, 4, 5]);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = sequence_moves_back_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                let mut rot = 0;
                let plus_rot = format!("-f{}", dim - 1);
                let minus_rot = format!("f{}", dim - 1);

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

                for face in &[1] {
                    for i in 1..dim - 1 {
                        for j in 1..dim - 1 {
                            assert_eq!(state[face * dim * dim + i * dim + j] / (dim * dim), *face);
                        }
                    }
                }
                if sequence.len() >= 2 {
                    for face in &[0, 2, 3, 4, 5] {
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
    std::fs::write("move_for_bfs_back_edge.json", serialized).ok();
}

fn generate_move_for_bfs_back_edge_diff(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 2 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 3, 4, 5]);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let target_idx_dif =
                cube_moves::target_idx_edge_distinct(dim, y, x + 1, &vec![0, 2, 3, 4, 5]);
            let mut idx_map_dif = std::collections::HashMap::new();
            for i in 0..target_idx_dif.len() {
                idx_map_dif.insert(target_idx_dif[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = sequence_moves_back_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                let mut rot = 0;
                let plus_rot = format!("-f{}", dim - 1);
                let minus_rot = format!("f{}", dim - 1);

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

                for face in &[1] {
                    for i in 1..dim - 1 {
                        for j in 1..dim - 1 {
                            assert_eq!(state[face * dim * dim + i * dim + j] / (dim * dim), *face);
                        }
                    }
                }
                if sequence.len() >= 2 {
                    for face in &[0, 2, 3, 4, 5] {
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
                for idx in target_idx_dif.iter() {
                    cur_move.push(idx_map_dif[&state[*idx]]);
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
    std::fs::write("move_for_bfs_back_edge_diff.json", serialized).ok();
}

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

fn generate_move_for_bfs_up_edge_diff(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 2 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let target_idx_dif =
                cube_moves::target_idx_edge_distinct(dim, y, x + 1, &vec![0, 2, 4, 5]);
            let mut idx_map_dif = std::collections::HashMap::new();
            for i in 0..target_idx_dif.len() {
                idx_map_dif.insert(target_idx_dif[i], i);
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
                for idx in target_idx_dif.iter() {
                    cur_move.push(idx_map_dif[&state[*idx]]);
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
    std::fs::write("move_for_bfs_up_edge_diff.json", serialized).ok();
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

fn generate_move_for_bfs_left_edge_diff(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 2 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![2, 4, 5]);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let target_idx_dif =
                cube_moves::target_idx_edge_distinct(dim, y, x + 1, &vec![2, 4, 5]);
            let mut idx_map_dif = std::collections::HashMap::new();
            for i in 0..target_idx_dif.len() {
                idx_map_dif.insert(target_idx_dif[i], i);
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
                for idx in target_idx_dif.iter() {
                    cur_move.push(idx_map_dif[&state[*idx]]);
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
    std::fs::write("move_for_bfs_left_edge_diff.json", serialized).ok();
}

fn main() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_33/33/33"];
    generate_move_for_bfs_front_corner(&info, 33);
    generate_move_for_bfs_front_edge(&info, 33);
    generate_move_for_bfs_front_edge_diff(&info, 33);
    generate_move_for_bfs_back_corner(&info, 33);
    generate_move_for_bfs_back_edge(&info, 33);
    generate_move_for_bfs_back_edge_diff(&info, 33);
    generate_move_for_bfs_up_corner(&info, 33);
    generate_move_for_bfs_up_edge(&info, 33);
    generate_move_for_bfs_up_edge_diff(&info, 33);
    generate_move_for_bfs_left_corner(&info, 33);
    generate_move_for_bfs_left_edge(&info, 33);
    generate_move_for_bfs_left_edge_diff(&info, 33);
    cube_moves::generate_four_corner_move_for_bfs(&info, 33);
    cube_moves::generate_four_edge_move_for_bfs(&info, 33);
    cube_moves::generate_four_edge_move_diff(&info, 33);
    cube_moves::generate_first_two_corner_move_for_bfs(&info, 33);
    cube_moves::generate_first_two_edge_move_for_bfs(&info, 33);
    cube_moves::generate_first_two_edge_move_diff(&info, 33);
    cube_moves::generate_cube_edge_move_for_bfs(&info, 33);
    cube_moves::generate_cube_edge_first_even_move_for_bfs(&puzzle_info["cube_10/10/10"], 10);
    cube_moves::bfs_four_corner();
    cube_moves::bfs_four_edge();
    cube_moves::bfs_first_two_corner();
    cube_moves::bfs_first_two_edge();
}

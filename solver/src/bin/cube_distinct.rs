// Reference
// * https://www.kaggle.com/code/seanbearden/solve-all-nxnxn-cubes-w-traditional-solution-state
// * https://macozy.com/rubik/rce/rc_e.html
// Dependencies
// * https://github.com/dwalton76/rubiks-cube-NxNxN-solver

use itertools::Itertools;
use rand::seq::SliceRandom;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

mod cube_moves;

fn count_center_cube_rot(
    cur_state: &Vec<usize>,
    moves: &Vec<String>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> Vec<usize> {
    let mut res = vec![0; 6];

    if dim % 2 == 0 {
        return res;
    }

    let mut cur_state = cur_state.clone();
    for m in moves.iter() {
        if *m == format!("f{}", 0) {
            res[cur_state[dim * dim + dim * dim / 2] / (dim * dim)] += 1;
        }
        if *m == format!("-f{}", 0) {
            res[cur_state[dim * dim + dim * dim / 2] / (dim * dim)] += 3;
        }
        if *m == format!("f{}", dim - 1) {
            res[cur_state[3 * dim * dim + dim * dim / 2] / (dim * dim)] += 3;
        }
        if *m == format!("-f{}", dim - 1) {
            res[cur_state[3 * dim * dim + dim * dim / 2] / (dim * dim)] += 1;
        }
        if *m == format!("d{}", 0) {
            res[cur_state[dim * dim / 2] / (dim * dim)] += 1;
        }
        if *m == format!("-d{}", 0) {
            res[cur_state[dim * dim / 2] / (dim * dim)] += 3;
        }
        if *m == format!("d{}", dim - 1) {
            res[cur_state[5 * dim * dim + dim * dim / 2] / (dim * dim)] += 3;
        }
        if *m == format!("-d{}", dim - 1) {
            res[cur_state[5 * dim * dim + dim * dim / 2] / (dim * dim)] += 1;
        }
        if *m == format!("r{}", 0) {
            res[cur_state[2 * dim * dim + dim * dim / 2] / (dim * dim)] += 1;
        }
        if *m == format!("-r{}", 0) {
            res[cur_state[2 * dim * dim + dim * dim / 2] / (dim * dim)] += 3;
        }
        if *m == format!("r{}", dim - 1) {
            res[cur_state[4 * dim * dim + dim * dim / 2] / (dim * dim)] += 3;
        }
        if *m == format!("-r{}", dim - 1) {
            res[cur_state[4 * dim * dim + dim * dim / 2] / (dim * dim)] += 1;
        }
        cur_state = cube_moves::apply_action(&cur_state, &actions[m]);
    }
    res
}

fn resolve_center_and_inversion(
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
                state = cube_moves::apply_action(&state, &actions[&action]);
                moves.push(action.clone());
            }
            if state[4 * dim * dim + idx_offset] == sol_state[idx_offset] {
                state = cube_moves::apply_action(&state, &actions[&action]);
                moves.push(action.clone());
            } else if state[2 * dim * dim + idx_offset] == sol_state[idx_offset] {
                let action = format!("-{}", action);
                state = cube_moves::apply_action(&state, &actions[&action]);
                moves.push(action.clone());
            }
        }
    }

    // 4: 0
    // 5: 2
    //    *
    // 6: 2
    //    *
    // 7: 3
    //    * *
    //      *
    /*
      1 2 3 4
    1   x x
    2     x
    3

     */

    if dim >= 5 {
        let mut inv = vec![vec![true; (dim + 1) / 2]; (dim * 1) / 2];
        for y in 1..dim / 2 {
            for x in y + 1..(dim - y - 1).min((dim + 1) / 2) {
                let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
                let mut idx_map = HashMap::new();
                for (i, idx) in target_idx.iter().enumerate() {
                    idx_map.insert(sol_state[*idx], i);
                }
                let mut p = Vec::new();
                for idx in target_idx.iter() {
                    p.push(idx_map[&state[*idx]]);
                }
                let mut cnt = 0;
                for i in 0..p.len() {
                    for j in i + 1..p.len() {
                        if p[i] > p[j] {
                            cnt += 1;
                        }
                    }
                }
                if cnt % 2 == 1 {
                    inv[y][x] = false;
                }
            }
        }
        for i in 0..2 {
            let mut inv_cpy = inv.clone();
            if i == 1 {
                for j in 0..inv_cpy.len() {
                    for k in 0..inv_cpy[j].len() {
                        inv_cpy[j][k] = !inv_cpy[j][k];
                    }
                }
            }
            let mut ok = true;
            let mut need_action = Vec::new();
            println!("i: {}", i);
            for j in 0..inv_cpy.len() {
                for k in 0..inv_cpy[j].len() {
                    print!("{} ", if inv_cpy[j][k] { "o" } else { "x" });
                }
                println!("");
            }
            for j in (1..(dim - 1) / 2).rev() {
                if inv_cpy[j][j + 1..].iter().all(|&x| x) {
                    // do nothing
                } else if inv_cpy[j][j + 1..].iter().all(|&x| !x) {
                    need_action.push(j);
                    for k in 0..j {
                        inv_cpy[k][j] = !inv_cpy[k][j];
                    }
                    for k in j + 1..=(dim - 1) / 2 {
                        inv_cpy[j][k] = !inv_cpy[j][k];
                    }
                } else {
                    ok = false;
                }
            }
            if ok {
                if i == 1 {
                    state = cube_moves::apply_action(&state, &actions["r0"]);
                    moves.push("r0".to_string());
                    println!("r0");
                }
                for a in need_action {
                    let act = format!("f{}", a);
                    state = cube_moves::apply_action(&state, &actions[&act]);
                    println!("{}", act);
                    moves.push(act);
                }

                break;
            }
            assert!(i != 1);
        }
    }

    (state, moves)
}

fn calc_permutation_idx_impl(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    target_idx: &Vec<usize>,
    target_pos: &Vec<usize>,
) -> usize {
    let mut idx_map: HashMap<usize, usize> = HashMap::new();
    for i in 0..target_idx.len() {
        idx_map.insert(cur_state[target_idx[i]], i);
    }

    perm_map[&target_pos
        .iter()
        .map(|i| idx_map[&sol_state[target_idx[*i]]])
        .collect::<Vec<usize>>()]
}

fn calc_permutation_idx_front(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    target_idx: &Vec<usize>,
) -> usize {
    calc_permutation_idx_impl(
        cur_state,
        sol_state,
        perm_map,
        target_idx,
        &vec![4, 5, 6, 7],
    )
}

fn calc_permutation_idx_back(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    target_idx: &Vec<usize>,
) -> usize {
    calc_permutation_idx_impl(
        cur_state,
        sol_state,
        perm_map,
        target_idx,
        &vec![8, 9, 10, 11],
    )
}

fn calc_permutation_idx_up(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    target_idx: &Vec<usize>,
) -> usize {
    calc_permutation_idx_impl(
        cur_state,
        sol_state,
        perm_map,
        target_idx,
        &vec![0, 1, 2, 3],
    )
}

fn calc_permutation_idx_left(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    target_idx: &Vec<usize>,
) -> usize {
    calc_permutation_idx_impl(
        cur_state,
        sol_state,
        perm_map,
        target_idx,
        &vec![4, 5, 6, 7],
    )
}

fn calc_permutation_idx_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    dim: usize,
    pos: usize,
) -> usize {
    let rev_pos = dim - 1 - pos;
    let target_idx = [
        2 * dim * dim + pos * dim + pos,
        2 * dim * dim + pos * dim + rev_pos,
        2 * dim * dim + rev_pos * dim + pos,
        2 * dim * dim + rev_pos * dim + rev_pos,
        5 * dim * dim + pos * dim + rev_pos,
        5 * dim * dim + rev_pos * dim + rev_pos,
        5 * dim * dim + pos * dim + pos,
        5 * dim * dim + rev_pos * dim + pos,
    ];
    let mut idx_map: HashMap<usize, usize> = HashMap::new();
    for i in 0..8 {
        idx_map.insert(sol_state[target_idx[i]], i);
    }
    perm_map[&(0..8)
        .map(|i| idx_map[&cur_state[target_idx[i]]])
        .collect::<Vec<usize>>()]
}

fn calc_permutation_idx_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    dim: usize,
    pos_y: usize,
    pos_x: usize,
) -> usize {
    let target_idx = [
        2 * dim * dim + pos_y * dim + pos_x,
        2 * dim * dim + (dim - 1 - pos_x) * dim + pos_y,
        2 * dim * dim + pos_x * dim + (dim - 1 - pos_y),
        2 * dim * dim + (dim - 1 - pos_y) * dim + (dim - 1 - pos_x),
        5 * dim * dim + pos_x * dim + (dim - 1 - pos_y),
        5 * dim * dim + pos_y * dim + pos_x,
        5 * dim * dim + (dim - 1 - pos_y) * dim + (dim - 1 - pos_x),
        5 * dim * dim + (dim - 1 - pos_x) * dim + pos_y,
    ];
    let mut idx_map: HashMap<usize, usize> = HashMap::new();
    for i in 0..8 {
        idx_map.insert(sol_state[target_idx[i]], i);
    }
    perm_map[&(0..8)
        .map(|i| idx_map[&cur_state[target_idx[i]]])
        .collect::<Vec<usize>>()]
}

fn calc_rot0(cur_state: &Vec<usize>, sol_state: &Vec<usize>, dim: usize) -> usize {
    let pos = dim / 2 - 1;
    let rev_pos = dim - 1 - pos;
    let target_idx = [
        2 * dim * dim + pos * dim + pos,
        2 * dim * dim + pos * dim + rev_pos,
        2 * dim * dim + rev_pos * dim + rev_pos,
        2 * dim * dim + rev_pos * dim + pos,
    ];
    for i in 0..3 {
        if cur_state[target_idx[i]] == sol_state[target_idx[0]] {
            return i;
        }
    }
    3
}

fn calc_rot1(cur_state: &Vec<usize>, sol_state: &Vec<usize>, dim: usize) -> usize {
    let pos = dim / 2 - 1;
    let rev_pos = dim - 1 - pos;
    let target_idx = [
        5 * dim * dim + pos * dim + rev_pos,
        5 * dim * dim + rev_pos * dim + rev_pos,
        5 * dim * dim + rev_pos * dim + pos,
        5 * dim * dim + pos * dim + pos,
    ];
    for i in 0..3 {
        if cur_state[target_idx[i]] == sol_state[target_idx[0]] {
            return i;
        }
    }
    3
}

fn apply_perm_to_partial(p: &Vec<usize>, perm: &Vec<usize>) -> Vec<usize> {
    let mut tmp = vec![100; perm.len()];
    for (idx, v) in p.iter().enumerate() {
        tmp[*v] = idx;
    }
    let mut res = vec![p.len() - 1; p.len()];
    for (idx, v) in perm.iter().enumerate() {
        if tmp[*v] == 100 {
            continue;
        }
        res[tmp[*v]] = idx;
    }
    res
}

fn apply_perm_to_partial_inv(p: &Vec<usize>, perm: &Vec<usize>) -> Vec<usize> {
    let mut tmp = vec![100; perm.len()];
    for (idx, v) in p.iter().enumerate() {
        tmp[*v] = idx;
    }
    let mut res = vec![p.len() - 1; p.len()];
    for (idx, v) in perm.iter().enumerate() {
        if tmp[idx] == 100 {
            continue;
        }
        res[tmp[idx]] = *v;
    }
    res
}

fn gen_perm_map_impl(size: usize) -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    let mut perm_list = Vec::new();
    let mut perm_map = HashMap::new();

    for p in (0..size).permutations(4) {
        perm_map.insert(p.clone(), perm_list.len());
        perm_list.push(p);
    }

    (perm_list, perm_map)
}

fn gen_perm_map_front_corner() -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    gen_perm_map_impl(24)
}

fn gen_perm_map_back_corner() -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    gen_perm_map_impl(20)
}

fn gen_perm_map_up_corner() -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    gen_perm_map_impl(16)
}

fn gen_perm_map_left_corner() -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    gen_perm_map_impl(12)
}

fn bfs_one_face_impl(
    actions: &Vec<(Vec<usize>, usize, i32)>,
    perm_list: &Vec<Vec<usize>>,
    perm_map: &HashMap<Vec<usize>, usize>,
    starts: &Vec<(Vec<usize>, usize)>,
    check_rot: bool,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let mut priority_qu = BinaryHeap::new();
    let rot_num = if check_rot { 4 } else { 1 };
    let mut prev_state: Vec<Option<usize>> = vec![None; rot_num * perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; rot_num * perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; rot_num * perm_list.len()];
    for (start, rot) in starts.iter() {
        let r = (rot_num - 1).min(*rot);
        priority_qu.push(Reverse((0, start.clone(), r)));
        cost[rot_num * perm_map[start] + r] = 0;
    }
    while let Some(Reverse((cur_cost, state, rot))) = priority_qu.pop() {
        let idx = rot_num * perm_map[&state] + rot.min(rot_num - 1);
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, rot_diff, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_perm_to_partial(&state, &perm);
            let next_idx = rot_num * perm_map[&next_state] + (rot + rot_diff) % rot_num;
            if action_cost + c < cost[next_idx] {
                cost[next_idx] = action_cost + c;
                prev_state[next_idx] = Some(idx);
                prev_action[next_idx] = Some(action_idx);
                priority_qu.push(Reverse((
                    action_cost + c,
                    next_state,
                    (rot + rot_diff) % rot_num,
                )));
            }
        }
    }
    // let mut reachable = 0;

    // let mut max_cost = 0;
    // let mut max_idx = 0;
    // for (idx, c) in cost.iter().enumerate() {
    //     if *c == std::i32::MAX {
    //         continue;
    //     }
    //     reachable += 1;
    //     if *c > max_cost {
    //         max_cost = *c;
    //         max_idx = idx;
    //     }
    // }
    // println!("reachable: {}", reachable);
    // println!("max_cost: {} {:?}", max_cost, perm_list[max_idx / rot_num]);

    (prev_state, prev_action, cost)
}

fn bfs_front_impl(
    actions: &Vec<(Vec<usize>, usize, i32)>,
    check_rot: bool,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let (perm_list, perm_map) = gen_perm_map_front_corner();
    let starts = Vec::from([
        (vec![4, 5, 6, 7], 0),
        (vec![5, 6, 7, 4], 1),
        (vec![6, 7, 4, 5], 2),
        (vec![7, 4, 5, 6], 3),
    ]);
    bfs_one_face_impl(actions, &perm_list, &perm_map, &starts, check_rot)
}

fn bfs_front_first_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_front_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_front_impl(&actions, false)
}

fn bfs_front_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_front_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_front_impl(&actions, true)
}

fn bfs_front_edge() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_front_edge.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_front_impl(&actions, true)
}

fn bfs_back_impl(
    actions: &Vec<(Vec<usize>, usize, i32)>,
    check_rot: bool,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let (perm_list, perm_map) = gen_perm_map_back_corner();
    let starts = Vec::from([
        (vec![8, 9, 10, 11], 0),
        (vec![9, 10, 11, 8], 1),
        (vec![10, 11, 8, 9], 2),
        (vec![11, 8, 9, 10], 3),
    ]);
    bfs_one_face_impl(actions, &perm_list, &perm_map, &starts, check_rot)
}

fn bfs_back_first_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_back_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_back_impl(&actions, false)
}

fn bfs_back_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_back_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_back_impl(&actions, true)
}

fn bfs_back_edge() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_back_edge.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_back_impl(&actions, true)
}

fn bfs_up_impl(
    actions: &Vec<(Vec<usize>, usize, i32)>,
    check_rot: bool,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let (perm_list, perm_map) = gen_perm_map_up_corner();
    let starts = Vec::from([
        (vec![0, 1, 2, 3], 0),
        (vec![1, 2, 3, 0], 1),
        (vec![2, 3, 0, 1], 2),
        (vec![3, 0, 1, 2], 3),
    ]);
    bfs_one_face_impl(actions, &perm_list, &perm_map, &starts, check_rot)
}

fn bfs_up_first_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_up_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_up_impl(&actions, false)
}

fn bfs_up_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_up_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_up_impl(&actions, true)
}

fn bfs_up_edge() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_up_edge.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_up_impl(&actions, true)
}

fn bfs_left_impl(
    actions: &Vec<(Vec<usize>, usize, i32)>,
    check_rot: bool,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let (perm_list, perm_map) = gen_perm_map_left_corner();
    let starts = Vec::from([
        (vec![4, 5, 6, 7], 0),
        (vec![5, 6, 7, 4], 1),
        (vec![6, 7, 4, 5], 2),
        (vec![7, 4, 5, 6], 3),
    ]);
    bfs_one_face_impl(actions, &perm_list, &perm_map, &starts, check_rot)
}

fn bfs_left_first_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_left_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_left_impl(&actions, false)
}

fn bfs_left_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_left_corner.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_left_impl(&actions, true)
}

fn bfs_left_edge() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let contents = std::fs::read_to_string("move_for_bfs_left_edge.json").unwrap();
    let actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    bfs_left_impl(&actions, true)
}

fn construct_path_from_bfs(
    cur_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    prev_state: &Vec<Option<usize>>,
    prev_action: &Vec<Option<usize>>,
    cost_table: &Vec<i32>,
    moves: &Vec<Vec<String>>,
    cur_state_idx: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    let mut state_idx = cur_state_idx;
    let mut action_idxes = Vec::new();
    while let Some(next_index) = prev_state[state_idx] {
        action_idxes.push(prev_action[state_idx].unwrap());
        state_idx = next_index;
    }
    if cost_table[state_idx] != 0 {
        println!("{}", cost_table.len());
        println!("{}", state_idx);
        println!("cost: {}", cost_table[state_idx]);
    }
    assert!(cost_table[state_idx] == 0);
    for action_idx in action_idxes.iter() {
        let action = &moves[*action_idx];
        for act in action.iter().rev() {
            let inv_act = cube_moves::rev_move(act);
            if !actions.contains_key(&inv_act) {
                println!("{}", inv_act);
            }
            let action = &actions[&inv_act];
            state = cube_moves::apply_action(&state, action);
            moves_str.push(inv_act);
        }
    }
    (state, moves_str)
}

fn solve_front_first_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_front_corner();
    let (prev_state, prev_action, cost_table) = bfs_front_first_corner();
    let pos = dim / 2 - 1;
    let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &(0..6).collect());
    let moves = cube_moves::sequence_moves_first_two_corner(dim, pos);
    let state_idx = calc_permutation_idx_front(cur_state, sol_state, &perm_map, &target_idx);
    construct_path_from_bfs(
        &cur_state,
        actions,
        &prev_state,
        &prev_action,
        &cost_table,
        &moves,
        state_idx,
    )
}

fn solve_front_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_front_corner();
    let (prev_state, prev_action, cost_table) = bfs_front_corner();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..dim / 2 - 1).rev() {
        let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &(0..6).collect());
        let perm_idx = calc_permutation_idx_front(&state, sol_state, &perm_map, &target_idx);
        let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 1);
        let state_idx = perm_idx * 4 + rot0;
        let moves = cube_moves::sequence_moves_first_two_corner(dim, pos);
        let (last_state, m) = construct_path_from_bfs(
            &state,
            actions,
            &prev_state,
            &prev_action,
            &cost_table,
            &moves,
            state_idx,
        );
        state = last_state;
        moves_str.extend(m);
    }
    (state, moves_str)
}

fn solve_front_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (perm_list, perm_map) = gen_perm_map_front_corner();
    let (_, prev_action, cost_table) = bfs_front_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    let contents = std::fs::read_to_string("move_for_bfs_front_edge.json").unwrap();
    let perm_actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let contents = std::fs::read_to_string("move_for_bfs_front_edge_diff.json").unwrap();
    let perm_actions_diff: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let mut cnt = 0;
    for y in 1..dim / 2 {
        let all_target_pos = (y + 1..dim - 1 - y)
            .map(|x| cube_moves::target_idx_edge_distinct(dim, y, x, &(0..6).collect()))
            .collect::<Vec<_>>();
        let all_moves = (y + 1..dim - 1 - y)
            .map(|x| cube_moves::sequence_moves_first_two_edge(dim, y, x))
            .collect::<Vec<_>>();
        if all_moves.is_empty() {
            continue;
        }
        let act_diff =
            cube_moves::calc_sequence_list_diff(&all_moves[0], all_moves.last().unwrap());
        let mut rng = rand::thread_rng();
        let mut fix_order = (0..dim - 1 - y - (y + 1)).collect::<Vec<usize>>();
        fix_order.shuffle(&mut rng);

        loop {
            let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 1);

            let perms = (0..dim - 1 - y - (y + 1))
                .map(|x| {
                    4 * calc_permutation_idx_front(&state, sol_state, &perm_map, &all_target_pos[x])
                        + rot0
                })
                .collect::<Vec<_>>();

            if perms.iter().map(|p| cost_table[*p]).sum::<i32>() == 0 {
                break;
            }
            let mut best_action_idx = 0;
            let mut best_action_cost = std::i32::MAX;
            let mut best_apply_list = Vec::new();
            for i in fix_order.iter() {
                if cost_table[perms[*i]] == 0 {
                    continue;
                }
                best_action_idx = prev_action[perms[*i]].unwrap();
                best_action_cost = 0;
                best_apply_list.push(*i);
                break;
            }
            for i in best_action_idx..best_action_idx + 1 {
                let moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial_inv(&perm_list[*p / 4], &perm_actions[i].0)]
                            + (*p + 4 - perm_actions[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let no_moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial(&perm_list[*p / 4], &perm_actions_diff[i].0)]
                            + (*p + 4 - perm_actions_diff[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let mut apply_list = Vec::new();
                let mut cost_sum = perm_actions[i].2;
                for j in 0..moved_perms.len() {
                    let no_action_cost = cost_table[no_moved_perms[j]] - cost_table[perms[j]];
                    let action_cost =
                        cost_table[moved_perms[j]] - cost_table[perms[j]] + act_diff[i].0;
                    if action_cost < no_action_cost {
                        cost_sum += action_cost;
                        if apply_list.is_empty() {
                            cost_sum -= act_diff[i].0;
                        }
                        apply_list.push(j);
                    } else {
                        cost_sum += no_action_cost;
                    }
                }
                if apply_list.is_empty() {
                    continue;
                }
                if cost_sum < best_action_cost
                    || (cost_sum == best_action_cost && apply_list.len() > best_apply_list.len())
                {
                    best_action_idx = i;
                    best_action_cost = cost_sum;
                    best_apply_list = apply_list;
                }
            }
            assert!(best_action_cost != std::i32::MAX);
            assert!(!best_apply_list.is_empty());
            let base_action = &all_moves[best_apply_list[0]][best_action_idx];
            for (inv_idx, act) in base_action.iter().rev().enumerate() {
                let idx = base_action.len() - 1 - inv_idx;
                if act_diff[best_action_idx].1 & (1 << idx) != 0 {
                    for j in best_apply_list.iter() {
                        let cur_act = &all_moves[*j][best_action_idx][idx];
                        let inv_act = cube_moves::rev_move(cur_act);
                        if !actions.contains_key(inv_act.as_str()) {
                            println!("{}", inv_act);
                        }
                        let action = &actions[inv_act.as_str()];
                        state = cube_moves::apply_action(&state, action);
                        moves_str.push(inv_act);
                    }
                } else {
                    let inv_act = cube_moves::rev_move(act);
                    if !actions.contains_key(inv_act.as_str()) {
                        println!("{}", inv_act);
                    }
                    let action = &actions[inv_act.as_str()];
                    state = cube_moves::apply_action(&state, action);
                    moves_str.push(inv_act);
                }
            }
            cnt += 1;
            if cnt == 10000 {
                assert!(false);
            }
        }
    }

    // let mut ok = true;
    // for i in &[1] {
    //     for j in 1..dim - 1 {
    //         for k in 1..dim - 1 {
    //             if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k] {
    //                 ok = false
    //             }
    //         }
    //     }
    // }
    // if !ok {
    // for i in 0..6 {
    //     for j in 1..dim - 1 {
    //         for k in 1..dim - 1 {
    //             print!("{} ", state[i * dim * dim + j * dim + k]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
    // }
    // assert!(ok);

    (state, moves_str)
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

fn solve_back_first_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_back_corner();
    let (prev_state, prev_action, cost_table) = bfs_back_first_corner();
    let pos = dim / 2 - 1;
    let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![0, 2, 3, 4, 5]);
    let moves = sequence_moves_back_corner(dim, pos);
    let state_idx = calc_permutation_idx_back(cur_state, sol_state, &perm_map, &target_idx);
    construct_path_from_bfs(
        &cur_state,
        actions,
        &prev_state,
        &prev_action,
        &cost_table,
        &moves,
        state_idx,
    )
}

fn solve_back_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_back_corner();
    let (prev_state, prev_action, cost_table) = bfs_back_corner();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..dim / 2 - 1).rev() {
        let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![0, 2, 3, 4, 5]);
        let perm_idx = calc_permutation_idx_back(&state, sol_state, &perm_map, &target_idx);
        let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 3);
        let state_idx = perm_idx * 4 + rot0;
        let moves = sequence_moves_back_corner(dim, pos);
        let (last_state, m) = construct_path_from_bfs(
            &state,
            actions,
            &prev_state,
            &prev_action,
            &cost_table,
            &moves,
            state_idx,
        );
        state = last_state;
        moves_str.extend(m);
    }
    (state, moves_str)
}

fn solve_back_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (perm_list, perm_map) = gen_perm_map_back_corner();
    let (_, prev_action, cost_table) = bfs_back_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    let contents = std::fs::read_to_string("move_for_bfs_back_edge.json").unwrap();
    let perm_actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let contents = std::fs::read_to_string("move_for_bfs_back_edge_diff.json").unwrap();
    let perm_actions_diff: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let mut cnt = 0;
    for y in 1..dim / 2 {
        let all_target_pos = (y + 1..dim - 1 - y)
            .map(|x| cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 3, 4, 5]))
            .collect::<Vec<_>>();
        let all_moves = (y + 1..dim - 1 - y)
            .map(|x| sequence_moves_back_edge(dim, y, x))
            .collect::<Vec<_>>();
        if all_moves.is_empty() {
            continue;
        }
        let act_diff =
            cube_moves::calc_sequence_list_diff(&all_moves[0], all_moves.last().unwrap());
        let mut rng = rand::thread_rng();
        let mut fix_order = (0..dim - 1 - y - (y + 1)).collect::<Vec<usize>>();
        fix_order.shuffle(&mut rng);

        loop {
            let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 3);

            let perms = (0..dim - 1 - y - (y + 1))
                .map(|x| {
                    4 * calc_permutation_idx_back(&state, sol_state, &perm_map, &all_target_pos[x])
                        + rot0
                })
                .collect::<Vec<_>>();

            if perms.iter().map(|p| cost_table[*p]).sum::<i32>() == 0 {
                break;
            }
            let mut best_action_idx = 0;
            let mut best_action_cost = std::i32::MAX;
            let mut best_apply_list = Vec::new();
            for i in fix_order.iter() {
                if cost_table[perms[*i]] == 0 {
                    continue;
                }
                best_action_idx = prev_action[perms[*i]].unwrap();
                best_action_cost = 0;
                best_apply_list.push(*i);
                break;
            }
            for i in best_action_idx..best_action_idx + 1 {
                let moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial_inv(&perm_list[*p / 4], &perm_actions[i].0)]
                            + (*p + 4 - perm_actions[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let no_moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial(&perm_list[*p / 4], &perm_actions_diff[i].0)]
                            + (*p + 4 - perm_actions_diff[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let mut apply_list = Vec::new();
                let mut cost_sum = perm_actions[i].2;
                for j in 0..moved_perms.len() {
                    let no_action_cost = cost_table[no_moved_perms[j]] - cost_table[perms[j]];
                    let action_cost =
                        cost_table[moved_perms[j]] - cost_table[perms[j]] + act_diff[i].0;
                    if action_cost < no_action_cost {
                        cost_sum += action_cost;
                        if apply_list.is_empty() {
                            cost_sum -= act_diff[i].0;
                        }
                        apply_list.push(j);
                    } else {
                        cost_sum += no_action_cost;
                    }
                }
                if apply_list.is_empty() {
                    continue;
                }
                if cost_sum < best_action_cost
                    || (cost_sum == best_action_cost && apply_list.len() > best_apply_list.len())
                {
                    best_action_idx = i;
                    best_action_cost = cost_sum;
                    best_apply_list = apply_list;
                }
            }
            assert!(best_action_cost != std::i32::MAX);
            assert!(!best_apply_list.is_empty());
            let base_action = &all_moves[best_apply_list[0]][best_action_idx];
            for (inv_idx, act) in base_action.iter().rev().enumerate() {
                let idx = base_action.len() - 1 - inv_idx;
                if act_diff[best_action_idx].1 & (1 << idx) != 0 {
                    for j in best_apply_list.iter() {
                        let cur_act = &all_moves[*j][best_action_idx][idx];
                        let inv_act = cube_moves::rev_move(cur_act);
                        if !actions.contains_key(inv_act.as_str()) {
                            println!("{}", inv_act);
                        }
                        let action = &actions[inv_act.as_str()];
                        state = cube_moves::apply_action(&state, action);
                        moves_str.push(inv_act);
                    }
                } else {
                    let inv_act = cube_moves::rev_move(act);
                    if !actions.contains_key(inv_act.as_str()) {
                        println!("{}", inv_act);
                    }
                    let action = &actions[inv_act.as_str()];
                    state = cube_moves::apply_action(&state, action);
                    moves_str.push(inv_act);
                }
            }
            cnt += 1;
            if cnt == 10000 {
                assert!(false);
            }
        }
    }

    // let mut ok = true;
    // for i in &[1, 3] {
    //     for j in 1..dim - 1 {
    //         for k in 1..dim - 1 {
    //             if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k] {
    //                 ok = false
    //             }
    //         }
    //     }
    // }
    // if !ok {
    //     for i in 0..6 {
    //         for j in 1..dim - 1 {
    //             for k in 1..dim - 1 {
    //                 print!("{} ", state[i * dim * dim + j * dim + k]);
    //             }
    //             println!("");
    //         }
    //         println!("");
    //     }
    // }
    // assert!(ok);

    (state, moves_str)
}

fn solve_up_first_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_up_corner();
    let (prev_state, prev_action, cost_table) = bfs_up_first_corner();
    let pos = dim / 2 - 1;
    let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![0, 2, 4, 5]);
    let moves = cube_moves::sequence_moves_four_corner(dim, pos);
    let state_idx = calc_permutation_idx_up(cur_state, sol_state, &perm_map, &target_idx);
    construct_path_from_bfs(
        &cur_state,
        actions,
        &prev_state,
        &prev_action,
        &cost_table,
        &moves,
        state_idx,
    )
}

fn solve_up_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_up_corner();
    let (prev_state, prev_action, cost_table) = bfs_up_corner();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..dim / 2 - 1).rev() {
        let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![0, 2, 4, 5]);
        let perm_idx = calc_permutation_idx_up(&state, sol_state, &perm_map, &target_idx);
        let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 0);
        let state_idx = perm_idx * 4 + rot0;
        let moves = cube_moves::sequence_moves_four_corner(dim, pos);
        let (last_state, m) = construct_path_from_bfs(
            &state,
            actions,
            &prev_state,
            &prev_action,
            &cost_table,
            &moves,
            state_idx,
        );
        state = last_state;
        moves_str.extend(m);
    }
    (state, moves_str)
}

fn solve_up_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (perm_list, perm_map) = gen_perm_map_up_corner();
    let (_, prev_action, cost_table) = bfs_up_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    let contents = std::fs::read_to_string("move_for_bfs_up_edge.json").unwrap();
    let perm_actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let contents = std::fs::read_to_string("move_for_bfs_up_edge_diff.json").unwrap();
    let perm_actions_diff: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let mut cnt = 0;
    for y in 1..dim / 2 {
        let all_target_pos = (y + 1..dim - 1 - y)
            .map(|x| cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]))
            .collect::<Vec<_>>();
        let all_moves = (y + 1..dim - 1 - y)
            .map(|x| cube_moves::sequence_moves_four_edge(dim, y, x))
            .collect::<Vec<_>>();
        if all_moves.is_empty() {
            continue;
        }
        let act_diff =
            cube_moves::calc_sequence_list_diff(&all_moves[0], all_moves.last().unwrap());
        let mut rng = rand::thread_rng();
        let mut fix_order = (0..dim - 1 - y - (y + 1)).collect::<Vec<usize>>();
        fix_order.shuffle(&mut rng);

        loop {
            let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 0);

            let perms = (0..dim - 1 - y - (y + 1))
                .map(|x| {
                    4 * calc_permutation_idx_up(&state, sol_state, &perm_map, &all_target_pos[x])
                        + rot0
                })
                .collect::<Vec<_>>();

            if perms.iter().map(|p| cost_table[*p]).sum::<i32>() == 0 {
                break;
            }
            let mut best_action_idx = 0;
            let mut best_action_cost = std::i32::MAX;
            let mut best_apply_list = Vec::new();
            for i in fix_order.iter() {
                if cost_table[perms[*i]] == 0 {
                    continue;
                }
                best_action_idx = prev_action[perms[*i]].unwrap();
                best_action_cost = 0;
                best_apply_list.push(*i);
                break;
            }
            for i in best_action_idx..best_action_idx + 1 {
                let moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial_inv(&perm_list[*p / 4], &perm_actions[i].0)]
                            + (*p + 4 - perm_actions[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let no_moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial(&perm_list[*p / 4], &perm_actions_diff[i].0)]
                            + (*p + 4 - perm_actions_diff[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let mut apply_list = Vec::new();
                let mut cost_sum = perm_actions[i].2;
                for j in 0..moved_perms.len() {
                    let no_action_cost = cost_table[no_moved_perms[j]] - cost_table[perms[j]];
                    let action_cost =
                        cost_table[moved_perms[j]] - cost_table[perms[j]] + act_diff[i].0;
                    if action_cost < no_action_cost {
                        cost_sum += action_cost;
                        if apply_list.is_empty() {
                            cost_sum -= act_diff[i].0;
                        }
                        apply_list.push(j);
                    } else {
                        cost_sum += no_action_cost;
                    }
                }
                if apply_list.is_empty() {
                    continue;
                }
                if cost_sum < best_action_cost
                    || (cost_sum == best_action_cost && apply_list.len() > best_apply_list.len())
                {
                    best_action_idx = i;
                    best_action_cost = cost_sum;
                    best_apply_list = apply_list;
                }
            }
            assert!(best_action_cost != std::i32::MAX);
            assert!(!best_apply_list.is_empty());
            let base_action = &all_moves[best_apply_list[0]][best_action_idx];
            for (inv_idx, act) in base_action.iter().rev().enumerate() {
                let idx = base_action.len() - 1 - inv_idx;
                if act_diff[best_action_idx].1 & (1 << idx) != 0 {
                    for j in best_apply_list.iter() {
                        let cur_act = &all_moves[*j][best_action_idx][idx];
                        let inv_act = cube_moves::rev_move(cur_act);
                        if !actions.contains_key(inv_act.as_str()) {
                            println!("{}", inv_act);
                        }
                        let action = &actions[inv_act.as_str()];
                        state = cube_moves::apply_action(&state, action);
                        moves_str.push(inv_act);
                    }
                } else {
                    let inv_act = cube_moves::rev_move(act);
                    if !actions.contains_key(inv_act.as_str()) {
                        println!("{}", inv_act);
                    }
                    let action = &actions[inv_act.as_str()];
                    state = cube_moves::apply_action(&state, action);
                    moves_str.push(inv_act);
                }
            }
            cnt += 1;
            if cnt == 10000 {
                assert!(false);
            }
        }
    }

    // let mut ok = true;
    // for i in &[0, 2, 4, 5] {
    //     for j in 1..dim - 1 {
    //         for k in 1..dim - 1 {
    //             if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k] {
    //                 ok = false
    //             }
    //         }
    //     }
    // }
    // if !ok {
    //     for i in 0..6 {
    //         for j in 1..dim - 1 {
    //             for k in 1..dim - 1 {
    //                 print!("{} ", state[i * dim * dim + j * dim + k]);
    //             }
    //             println!("");
    //         }
    //         println!("");
    //     }
    // }
    // assert!(ok);

    (state, moves_str)
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
    res
}

fn sequence_moves_left_edge(dim: usize, y: usize, x: usize) -> Vec<Vec<String>> {
    let base = cube_moves::sequence_moves_four_edge(dim, y, x);
    let valid_idxes = generate_sequence_filter_for_left_edge();
    let mut res = Vec::new();
    for idx in valid_idxes {
        res.push(base[idx].clone());
    }
    res
}

fn solve_left_first_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_left_corner();
    let (prev_state, prev_action, cost_table) = bfs_left_first_corner();
    let pos = dim / 2 - 1;
    let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![2, 4, 5]);
    let moves = sequence_moves_left_corner(dim, pos);
    let state_idx = calc_permutation_idx_left(cur_state, sol_state, &perm_map, &target_idx);
    construct_path_from_bfs(
        &cur_state,
        actions,
        &prev_state,
        &prev_action,
        &cost_table,
        &moves,
        state_idx,
    )
}

fn solve_left_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_left_corner();
    let (prev_state, prev_action, cost_table) = bfs_left_corner();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..dim / 2 - 1).rev() {
        let target_idx = cube_moves::target_idx_corner_distinct(dim, pos, &vec![2, 4, 5]);
        let perm_idx = calc_permutation_idx_left(&state, sol_state, &perm_map, &target_idx);
        let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 4);
        let state_idx = perm_idx * 4 + rot0;
        let moves = sequence_moves_left_corner(dim, pos);
        let (last_state, m) = construct_path_from_bfs(
            &state,
            actions,
            &prev_state,
            &prev_action,
            &cost_table,
            &moves,
            state_idx,
        );
        state = last_state;
        moves_str.extend(m);
    }
    (state, moves_str)
}

fn solve_left_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (perm_list, perm_map) = gen_perm_map_left_corner();
    let (_, prev_action, cost_table) = bfs_left_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    let contents = std::fs::read_to_string("move_for_bfs_left_edge.json").unwrap();
    let perm_actions: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let contents = std::fs::read_to_string("move_for_bfs_left_edge_diff.json").unwrap();
    let perm_actions_diff: Vec<(Vec<usize>, usize, i32)> = serde_json::from_str(&contents).unwrap();
    let mut cnt = 0;
    for y in 1..dim / 2 {
        let all_target_pos = (y + 1..dim - 1 - y)
            .map(|x| cube_moves::target_idx_edge_distinct(dim, y, x, &vec![2, 4, 5]))
            .collect::<Vec<_>>();
        let all_moves = (y + 1..dim - 1 - y)
            .map(|x| sequence_moves_left_edge(dim, y, x))
            .collect::<Vec<_>>();
        if all_moves.is_empty() {
            continue;
        }
        let act_diff =
            cube_moves::calc_sequence_list_diff(&all_moves[0], all_moves.last().unwrap());
        let mut rng = rand::thread_rng();
        let mut fix_order = (0..dim - 1 - y - (y + 1)).collect::<Vec<usize>>();
        fix_order.shuffle(&mut rng);

        loop {
            let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 4);

            let perms = (0..dim - 1 - y - (y + 1))
                .map(|x| {
                    4 * calc_permutation_idx_left(&state, sol_state, &perm_map, &all_target_pos[x])
                        + rot0
                })
                .collect::<Vec<_>>();

            if perms.iter().map(|p| cost_table[*p]).sum::<i32>() == 0 {
                break;
            }
            let mut best_action_idx = 0;
            let mut best_action_cost = std::i32::MAX;
            let mut best_apply_list = Vec::new();
            for i in fix_order.iter() {
                if cost_table[perms[*i]] == 0 {
                    continue;
                }
                best_action_idx = prev_action[perms[*i]].unwrap();
                best_action_cost = 0;
                best_apply_list.push(*i);
                break;
            }
            for i in best_action_idx..best_action_idx + 1 {
                let moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial_inv(&perm_list[*p / 4], &perm_actions[i].0)]
                            + (*p + 4 - perm_actions[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let no_moved_perms = perms
                    .iter()
                    .map(|p| {
                        4 * perm_map
                            [&apply_perm_to_partial(&perm_list[*p / 4], &perm_actions_diff[i].0)]
                            + (*p + 4 - perm_actions_diff[i].1) % 4
                    })
                    .collect::<Vec<_>>();
                let mut apply_list = Vec::new();
                let mut cost_sum = perm_actions[i].2;
                for j in 0..moved_perms.len() {
                    let no_action_cost = cost_table[no_moved_perms[j]] - cost_table[perms[j]];
                    let action_cost =
                        cost_table[moved_perms[j]] - cost_table[perms[j]] + act_diff[i].0;
                    if action_cost < no_action_cost {
                        cost_sum += action_cost;
                        if apply_list.is_empty() {
                            cost_sum -= act_diff[i].0;
                        }
                        apply_list.push(j);
                    } else {
                        cost_sum += no_action_cost;
                    }
                }
                if apply_list.is_empty() {
                    continue;
                }
                if cost_sum < best_action_cost
                    || (cost_sum == best_action_cost && apply_list.len() > best_apply_list.len())
                {
                    best_action_idx = i;
                    best_action_cost = cost_sum;
                    best_apply_list = apply_list;
                }
            }
            assert!(best_action_cost != std::i32::MAX);
            assert!(!best_apply_list.is_empty());
            let base_action = &all_moves[best_apply_list[0]][best_action_idx];
            for (inv_idx, act) in base_action.iter().rev().enumerate() {
                let idx = base_action.len() - 1 - inv_idx;
                if act_diff[best_action_idx].1 & (1 << idx) != 0 {
                    for j in best_apply_list.iter() {
                        let cur_act = &all_moves[*j][best_action_idx][idx];
                        let inv_act = cube_moves::rev_move(cur_act);
                        if !actions.contains_key(inv_act.as_str()) {
                            println!("{}", inv_act);
                        }
                        let action = &actions[inv_act.as_str()];
                        state = cube_moves::apply_action(&state, action);
                        moves_str.push(inv_act);
                    }
                } else {
                    let inv_act = cube_moves::rev_move(act);
                    if !actions.contains_key(inv_act.as_str()) {
                        println!("{}", inv_act);
                    }
                    let action = &actions[inv_act.as_str()];
                    state = cube_moves::apply_action(&state, action);
                    moves_str.push(inv_act);
                }
            }
            cnt += 1;
            if cnt == 10000 {
                assert!(false);
            }
        }
    }

    // let mut ok = true;
    // for i in &[0, 1, 3, 4] {
    //     for j in 1..dim - 1 {
    //         for k in 1..dim - 1 {
    //             if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k] {
    //                 ok = false
    //             }
    //         }
    //     }
    // }
    // if !ok {
    //     for i in 0..6 {
    //         for j in 1..dim - 1 {
    //             for k in 1..dim - 1 {
    //                 print!("{} ", state[i * dim * dim + j * dim + k]);
    //             }
    //             println!("");
    //         }
    //         println!("");
    //     }
    // }
    // assert!(ok);

    (state, moves_str)
}

fn solve_green_middle_center(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = get_perm_map();
    let parity = cube_moves::calc_face_rot(cur_state, sol_state, dim, 0)
        + cube_moves::calc_face_rot(cur_state, sol_state, dim, 1)
        + cube_moves::calc_face_rot(cur_state, sol_state, dim, 3)
        + cube_moves::calc_face_rot(cur_state, sol_state, dim, 4);
    let (prev_state, prev_action) = bfs0(1 - parity % 2);
    let pos = dim / 2 - 1;
    let rev_pos = dim - 1 - pos;
    let mut state_idx = calc_permutation_idx_corner(cur_state, sol_state, &perm_map, dim, pos);
    let f_rev_minus = format!("-f{}", rev_pos);
    let f_rev_plus = format!("f{}", rev_pos);
    let f_minus = format!("-f{}", pos);
    let f_plus = format!("f{}", pos);
    let moves = [
        Vec::from(["-r0"]),
        Vec::from(["r0"]),
        Vec::from(["-d0"]),
        Vec::from(["d0"]),
        Vec::from([f_rev_minus.as_str(), "r0", f_rev_plus.as_str()]),
        Vec::from([f_rev_minus.as_str(), "r0", "r0", f_rev_plus.as_str()]),
        Vec::from([f_rev_minus.as_str(), "-r0", f_rev_plus.as_str()]),
        Vec::from([f_minus.as_str(), "r0", f_plus.as_str()]),
        Vec::from([f_minus.as_str(), "r0", "r0", f_plus.as_str()]),
        Vec::from([f_minus.as_str(), "-r0", f_plus.as_str()]),
    ];
    let mut action_idxes = Vec::new();
    while let Some(next_index) = prev_state[state_idx] {
        action_idxes.push(prev_action[state_idx].unwrap());
        state_idx = next_index;
    }
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for action_idx in action_idxes.iter() {
        let action = &moves[*action_idx];
        for act in action.iter() {
            let action = &actions[*act];
            state = cube_moves::apply_action(&state, action);
            moves_str.push(act.to_string());
        }
    }
    (state, moves_str)
}

fn solve_green_middle_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = get_perm_map();
    let (prev_state, prev_action) = bfs1();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..=dim / 2 - 2).rev() {
        let rev_pos = dim - 1 - pos;
        let perm_idx = calc_permutation_idx_corner(&state, sol_state, &perm_map, dim, pos);
        let rot0 = calc_rot0(&state, sol_state, dim);
        let rot1 = calc_rot1(&state, sol_state, dim);
        let mut state_idx = perm_idx * 16 + rot0 * 4 + rot1;
        let f_rev_minus = format!("-f{}", rev_pos);
        let f_rev_plus = format!("f{}", rev_pos);
        let f_minus = format!("-f{}", pos);
        let f_plus = format!("f{}", pos);
        let moves = [
            Vec::from(["-r0"]),
            Vec::from(["r0"]),
            Vec::from(["-d0"]),
            Vec::from(["d0"]),
            Vec::from([f_rev_minus.as_str(), "r0", f_rev_plus.as_str()]),
            Vec::from([f_rev_minus.as_str(), "r0", "r0", f_rev_plus.as_str()]),
            Vec::from([f_rev_minus.as_str(), "-r0", f_rev_plus.as_str()]),
            Vec::from([f_minus.as_str(), "r0", f_plus.as_str()]),
            Vec::from([f_minus.as_str(), "r0", "r0", f_plus.as_str()]),
            Vec::from([f_minus.as_str(), "-r0", f_plus.as_str()]),
        ];
        let mut action_idxes = Vec::new();
        while let Some(next_index) = prev_state[state_idx] {
            action_idxes.push(prev_action[state_idx].unwrap());
            state_idx = next_index;
        }
        for action_idx in action_idxes.iter() {
            let action = &moves[*action_idx];
            for act in action.iter() {
                if !actions.contains_key(*act) {
                    println!("{}", act);
                }
                let action = &actions[*act];
                state = cube_moves::apply_action(&state, action);
                moves_str.push(act.to_string());
            }
        }
    }
    (state, moves_str)
}

fn solve_green_middle_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (perm_list, perm_map) = get_perm_map();
    let (prev_state, prev_action) = bfs3();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            // let debug = y == 1 && x == 3;
            let debug = false;
            let rev_x = dim - 1 - x;
            let rev_y = dim - 1 - y;
            let perm_idx = calc_permutation_idx_edge(&state, sol_state, &perm_map, dim, y, x);
            let rot0 = calc_rot0(&state, sol_state, dim);
            let rot1 = calc_rot1(&state, sol_state, dim);
            let mut perm_map = std::collections::HashMap::new();
            perm_map.insert(2 * dim * dim + y * dim + x, 0);
            perm_map.insert(2 * dim * dim + rev_x * dim + y, 1);
            perm_map.insert(2 * dim * dim + x * dim + rev_y, 2);
            perm_map.insert(2 * dim * dim + rev_y * dim + rev_x, 3);
            perm_map.insert(5 * dim * dim + x * dim + rev_y, 4);
            perm_map.insert(5 * dim * dim + y * dim + x, 5);
            perm_map.insert(5 * dim * dim + rev_y * dim + rev_x, 6);
            perm_map.insert(5 * dim * dim + rev_x * dim + y, 7);

            if debug {
                println!("perm: {:?}", perm_list[perm_idx]);
                println!("  {}", perm_map[&state[2 * dim * dim + y * dim + x]]);
                println!(
                    "{}   {}",
                    perm_map[&state[2 * dim * dim + rev_x * dim + y]],
                    perm_map[&state[2 * dim * dim + x * dim + rev_y]]
                );
                println!(
                    "  {}",
                    perm_map[&state[2 * dim * dim + rev_y * dim + rev_x]]
                );
                println!("  {}", perm_map[&state[5 * dim * dim + x * dim + rev_y]]);
                println!(
                    "{}   {}",
                    perm_map[&state[5 * dim * dim + y * dim + x]],
                    perm_map[&state[5 * dim * dim + rev_y * dim + rev_x]]
                );
                println!("  {}", perm_map[&state[5 * dim * dim + rev_x * dim + y]]);
                println!("rot0: {}", rot0);
                println!(
                    "{:4} {:4}",
                    state[2 * dim * dim + 15 * dim + 15],
                    state[2 * dim * dim + 15 * dim + 17]
                );
                println!("");
                println!(
                    "{:4} {:4}",
                    state[2 * dim * dim + 17 * dim + 15],
                    state[2 * dim * dim + 17 * dim + 17]
                );
                println!("rot1: {}", rot1);
                println!(
                    "{:4} {:4}",
                    state[5 * dim * dim + 15 * dim + 15],
                    state[5 * dim * dim + 15 * dim + 17]
                );
                println!("");
                println!(
                    "{:4} {:4}",
                    state[5 * dim * dim + 17 * dim + 15],
                    state[5 * dim * dim + 17 * dim + 17]
                );
            }
            let mut state_idx = perm_idx * 16 + rot0 * 4 + rot1;
            let f_rev_x_minus = format!("-f{}", rev_x);
            let f_rev_x_plus = format!("f{}", rev_x);
            let f_x_minus = format!("-f{}", x);
            let f_x_plus = format!("f{}", x);
            let f_rev_y_minus = format!("-f{}", rev_y);
            let f_rev_y_plus = format!("f{}", rev_y);
            let f_y_minus = format!("-f{}", y);
            let f_y_plus = format!("f{}", y);

            let moves = [
                Vec::from(["-r0"]),
                Vec::from(["r0"]),
                Vec::from(["-d0"]),
                Vec::from(["d0"]),
                Vec::from([
                    "r0",
                    f_rev_x_minus.as_str(),
                    "-r0",
                    f_rev_y_minus.as_str(),
                    "r0",
                    f_rev_x_plus.as_str(),
                    "-r0",
                    f_rev_y_plus.as_str(),
                ]),
                Vec::from([
                    f_rev_y_minus.as_str(),
                    "r0",
                    f_rev_x_minus.as_str(),
                    "-r0",
                    f_rev_y_plus.as_str(),
                    "r0",
                    f_rev_x_plus.as_str(),
                    "-r0",
                ]),
                Vec::from([
                    "-r0",
                    f_rev_x_minus.as_str(),
                    "r0",
                    f_y_minus.as_str(),
                    "-r0",
                    f_rev_x_plus.as_str(),
                    "r0",
                    f_y_plus.as_str(),
                ]),
                Vec::from([
                    f_y_minus.as_str(),
                    "-r0",
                    f_rev_x_minus.as_str(),
                    "r0",
                    f_y_plus.as_str(),
                    "-r0",
                    f_rev_x_plus.as_str(),
                    "r0",
                ]),
                Vec::from([
                    "-d0",
                    f_x_plus.as_str(),
                    "d0",
                    f_rev_y_plus.as_str(),
                    "-d0",
                    f_x_minus.as_str(),
                    "d0",
                    f_rev_y_minus.as_str(),
                ]),
                Vec::from([
                    f_rev_y_plus.as_str(),
                    "-d0",
                    f_x_plus.as_str(),
                    "d0",
                    f_rev_y_minus.as_str(),
                    "-d0",
                    f_x_minus.as_str(),
                    "d0",
                ]),
                Vec::from([
                    "d0",
                    f_x_plus.as_str(),
                    "-d0",
                    f_y_plus.as_str(),
                    "d0",
                    f_x_minus.as_str(),
                    "-d0",
                    f_y_minus.as_str(),
                ]),
                Vec::from([
                    f_y_plus.as_str(),
                    "d0",
                    f_x_plus.as_str(),
                    "-d0",
                    f_y_minus.as_str(),
                    "d0",
                    f_x_minus.as_str(),
                    "-d0",
                ]),
            ];

            let mut action_idxes = Vec::new();
            let mut qu = std::collections::VecDeque::new();
            if debug {
                println!("start: {} -> {:?}", state_idx, prev_state[state_idx])
            }
            while let Some(next_index) = prev_state[state_idx] {
                action_idxes.push(prev_action[state_idx].unwrap());
                state_idx = next_index;
                qu.push_back(state_idx);
            }
            for action_idx in action_idxes.iter() {
                let action = &moves[*action_idx];
                for act in action.iter() {
                    if !actions.contains_key(*act) {
                        println!("{}", act);
                    }
                    let action = &actions[*act];
                    state = cube_moves::apply_action(&state, action);
                    moves_str.push(act.to_string());
                }
                if debug {
                    let idx = qu.pop_front().unwrap();
                    let perm_idx = idx / 16;
                    let rot0 = (idx % 16) / 4;
                    let rot1 = idx % 4;
                    println!("perm: {:?}", perm_list[perm_idx]);
                    println!("  {}", perm_map[&state[2 * dim * dim + y * dim + x]]);
                    println!(
                        "{}   {}",
                        perm_map[&state[2 * dim * dim + rev_x * dim + y]],
                        perm_map[&state[2 * dim * dim + x * dim + rev_y]]
                    );
                    println!(
                        "  {}",
                        perm_map[&state[2 * dim * dim + rev_y * dim + rev_x]]
                    );
                    println!("  {}", perm_map[&state[5 * dim * dim + x * dim + rev_y]]);
                    println!(
                        "{}   {}",
                        perm_map[&state[5 * dim * dim + y * dim + x]],
                        perm_map[&state[5 * dim * dim + rev_y * dim + rev_x]]
                    );
                    println!("  {}", perm_map[&state[5 * dim * dim + rev_x * dim + y]]);
                    println!("rot0: {}", rot0);
                    println!(
                        "{:4} {:4}",
                        state[2 * dim * dim + 15 * dim + 15],
                        state[2 * dim * dim + 15 * dim + 17]
                    );
                    println!("");
                    println!(
                        "{:4} {:4}",
                        state[2 * dim * dim + 17 * dim + 15],
                        state[2 * dim * dim + 17 * dim + 17]
                    );
                    println!("rot1: {}", rot1);
                    println!(
                        "{:4} {:4}",
                        state[5 * dim * dim + 15 * dim + 15],
                        state[5 * dim * dim + 15 * dim + 17]
                    );
                    println!("");
                    println!(
                        "{:4} {:4}",
                        state[5 * dim * dim + 17 * dim + 15],
                        state[5 * dim * dim + 17 * dim + 17]
                    );
                }
            }
        }
    }

    (state, moves_str)
}

fn solve_center(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    if dim % 2 == 0 {
        return (cur_state.clone(), Vec::new());
    }
    let half = dim / 2;
    let action_list = [
        format!("f{}", half),
        format!("r{}", half),
        format!("d{}", half),
        format!("-f{}", half),
        format!("-r{}", half),
        format!("-d{}", half),
    ];
    fn valid_state(state: &Vec<usize>, sol_state: &Vec<usize>, dim: usize) -> bool {
        let half = dim / 2;
        let idx_offset = half * dim + half;
        for i in 0..6 {
            if state[i * dim * dim + idx_offset] != sol_state[i * dim * dim + idx_offset] {
                return false;
            }
        }
        true
    }
    let mut best_end_state = Vec::new();
    let mut best_move = Vec::new();
    let mut best_cost = 100000000;
    if valid_state(cur_state, sol_state, dim) {
        return (cur_state.clone(), Vec::new());
    }
    let mut state = cur_state.clone();
    for (idx0, act0) in action_list.iter().enumerate() {
        state = cube_moves::apply_action(&state, &actions[act0]);
        if valid_state(&state, sol_state, dim) && best_cost > 1 {
            best_end_state = state.clone();
            best_move.clear();
            best_move.push(act0.clone());
            best_cost = 1;
        }
        for (idx1, act1) in action_list.iter().enumerate() {
            if (idx1 + 3) % 6 == idx0 {
                continue;
            }
            state = cube_moves::apply_action(&state, &actions[act1]);
            if valid_state(&state, sol_state, dim) && best_cost > 2 {
                best_end_state = state.clone();
                best_move.clear();
                best_move.push(act0.clone());
                best_move.push(act1.clone());
                best_cost = 2;
            }
            for (idx2, act2) in action_list.iter().enumerate() {
                if (idx2 + 3) % 6 == idx1 {
                    continue;
                }
                state = cube_moves::apply_action(&state, &actions[act2]);
                if valid_state(&state, sol_state, dim) && best_cost > 3 {
                    best_end_state = state.clone();
                    best_move.clear();
                    best_move.push(act0.clone());
                    best_move.push(act1.clone());
                    best_move.push(act2.clone());
                    best_cost = 3;
                }
                state = cube_moves::apply_action(&state, &actions[&cube_moves::rev_move(act2)]);
            }
            state = cube_moves::apply_action(&state, &actions[&cube_moves::rev_move(act1)]);
        }

        state = cube_moves::apply_action(&state, &actions[&cube_moves::rev_move(act0)]);
    }
    (best_end_state, best_move)
}

fn solve_front_face(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_center(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_front_first_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_front_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_front_edge(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);

    (state, moves)
}

fn solve_back_face(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_back_first_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_back_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_back_edge(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);

    (state, moves)
}

fn solve_up_face(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_up_first_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_up_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_up_edge(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);

    (state, moves)
}

fn solve_left_face(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_left_first_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_left_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_left_edge(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);

    (state, moves)
}

fn solve_green_middle2(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_green_middle_center(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_green_middle_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_green_middle_edge(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);

    // for i in 0..6 {
    //     println!("{}:", i);
    //     for j in 1..=31 {
    //         for k in 1..=31 {
    //             print!(
    //                 "{:4}({:4}) ",
    //                 state[i * dim * dim + j * dim + k],
    //                 sol_state[i * dim * dim + j * dim + k]
    //             );
    //         }
    //         println!("");
    //     }
    // }

    // for (i, j) in middle_check_order_green(dim) {
    //     let target_index = (5 * dim + i) * dim + j;
    //     checked[i * dim + j] = true;
    //     if state[target_index] == sol_state[target_index] {
    //         continue;
    //     }
    //     let mut cur_index = 0;
    //     for k in movable_pos[target_index].iter() {
    //         if *k < 2 * dim * dim {
    //             continue;
    //         }
    //         if 3 * dim * dim <= *k && *k < 5 * dim * dim {
    //             continue;
    //         }
    //         if 5 * dim * dim <= *k && checked[*k - 5 * dim * dim] {
    //             continue;
    //         }
    //         if state[*k] == sol_state[target_index] {
    //             cur_index = *k;
    //             break;
    //         }
    //     }
    //     if cur_index == 0 {
    //         panic!("source index not found");
    //     }
    //     let target_p3 = index_to_p3(target_index, dim);
    //     let mut perm = (0..6 * dim * dim).collect::<Vec<usize>>();
    //     while cur_index != target_index {
    //         let cur_p3 = index_to_p3(cur_index, dim);
    //         let m = cube_moves::solve_green_middle_impl(&cur_p3, &target_p3, dim);
    //         if m.is_empty() {
    //             break;
    //         }
    //         for act in m.iter() {
    //             let action = &actions[act];
    //             state = cube_moves::apply_action(&state, action);
    //             perm = cube_moves::apply_action(&perm, action);
    //             cur_index = allowed_moves_inv[act][cur_index] as usize;
    //             moves.push(act.clone());
    //         }
    //     }
    //     println!("Applyed perm:");
    //     for k in 0..6 {
    //         let mut ok = true;
    //         for kk in 1..dim - 1 {
    //             for kkk in 1..dim - 1 {
    //                 let idx = k * dim * dim + kk * dim + kkk;
    //                 if idx != perm[idx] {
    //                     ok = false;
    //                 }
    //             }
    //         }
    //         if ok {
    //             continue;
    //         }
    //         println!("{}:", k);
    //         for kk in 1..dim - 1 {
    //             for kkk in 1..dim - 1 {
    //                 let idx = k * dim * dim + kk * dim + kkk;
    //                 let a = perm[idx] / (dim * dim);
    //                 let b = perm[idx] / dim % dim;
    //                 let c = perm[idx] % dim;
    //                 if idx == perm[idx] {
    //                     print!("( ,   ,   )")
    //                 } else {
    //                     print!("({}, {:2}, {:2})", a, b, c);
    //                 }
    //             }
    //             println!("");
    //         }
    //     }
    //     println!("");
    //     println!("Current");
    //     for k in 0..6 {
    //         let mut ok = true;
    //         for kk in 1..dim - 1 {
    //             for kkk in 1..dim - 1 {
    //                 let idx = k * dim * dim + kk * dim + kkk;
    //                 if idx / (dim * dim) != state[idx] / (dim * dim) {
    //                     ok = false;
    //                 }
    //             }
    //         }
    //         if ok {
    //             continue;
    //         }
    //         println!("{}:", k);
    //         for kk in 1..dim - 1 {
    //             for kkk in 1..dim - 1 {
    //                 let idx = k * dim * dim + kk * dim + kkk;
    //                 let a = state[idx] / (dim * dim);
    //                 let b = state[idx] / dim % dim;
    //                 let c = state[idx] % dim;
    //                 if k == a {
    //                     print!("o ")
    //                 } else {
    //                     print!("x ");
    //                 }
    //             }
    //             println!("");
    //         }
    //     }
    //     println!("");
    // }
    (state, moves)
}

fn move_translation(dim: usize) -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::new();
    m.insert("U".to_string(), format!("-d{}", dim - 1));
    m.insert("R".to_string(), "r0".to_string());
    m.insert("B".to_string(), format!("-f{}", dim - 1));
    m.insert("F".to_string(), "f0".to_string());
    m.insert("L".to_string(), format!("-r{}", dim - 1));
    m.insert("D".to_string(), "d0".to_string());

    if dim > 3 {
        m.insert("Uw".to_string(), format!("-d{}.-d{}", dim - 2, dim - 1));
        m.insert("Rw".to_string(), "r0.r1".to_string());
        m.insert("Bw".to_string(), format!("-f{}.-f{}", dim - 2, dim - 1));
        m.insert("Fw".to_string(), "f0.f1".to_string());
        m.insert("Lw".to_string(), format!("-r{}.-r{}", dim - 2, dim - 1));
        m.insert("Dw".to_string(), "d0.d1".to_string());
    }

    if dim >= 4 {
        m.insert("2Uw".to_string(), format!("-d{}.-d{}", dim - 2, dim - 1));
        m.insert("2Rw".to_string(), "r0.r1".to_string());
        m.insert("2Bw".to_string(), format!("-f{}.-f{}", dim - 2, dim - 1));
        m.insert("2Fw".to_string(), "f0.f1".to_string());
        m.insert("2Lw".to_string(), format!("-r{}.-r{}", dim - 2, dim - 1));
        m.insert("2Dw".to_string(), "d0.d1".to_string());

        let width_max = dim / 2;
        for i in 3..=width_max {
            m.insert(
                format!("{}Uw", i),
                format!("-d{}.{}", dim - i, m[&format!("{}Uw", i - 1)]),
            );
            m.insert(
                format!("{}Rw", i),
                format!("{}.r{}", m[&format!("{}Rw", i - 1)], i - 1),
            );
            m.insert(
                format!("{}Bw", i),
                format!("-f{}.{}", dim - i, m[&format!("{}Bw", i - 1)]),
            );
            m.insert(
                format!("{}Fw", i),
                format!("{}.f{}", m[&format!("{}Fw", i - 1)], i - 1),
            );
            m.insert(
                format!("{}Lw", i),
                format!("-r{}.{}", dim - i, m[&format!("{}Lw", i - 1)]),
            );
            m.insert(
                format!("{}Dw", i),
                format!("{}.d{}", m[&format!("{}Dw", i - 1)], i - 1),
            );
        }
    }
    let keys: Vec<String> = m.keys().cloned().collect();
    for key in keys {
        let value = m[&key].clone();
        m.insert(format!("{}2", key), format!("{}.{}", value, value));

        let prime = if value.contains("-") {
            value.replace("-", "")
        } else {
            value
                .split('.')
                .map(|i| format!("-{}", i))
                .collect::<Vec<String>>()
                .join(".")
        };

        m.insert(format!("{}'", key), prime);
    }
    m
}

fn calc_dim(l: usize) -> usize {
    for i in 1..50 {
        if i * i == l / 6 {
            return i;
        }
    }
    panic!("Invalid length of state");
}

/*
N0;N1;N2;N3;
N4;N5;N9;N7;
N8;N6;N10;N11;
N12;N13;N14;N15;

N16;N17;N18;N19;
N20;N25;N21;N23;
N24;N26;N22;N27;
N28;N29;N30;N31;

N32;N33;N34;N35;
N36;N38;N42;N39;
N40;N37;N41;N43;
N44;N45;N46;N47;

N48;N49;N50;N51;
N52;N57;N53;N55;
N56;N58;N54;N59;
N60;N61;N62;N63;

N64;N65;N66;N67;
N68;N74;N73;N71;
N72;N70;N69;N75;
N76;N77;N78;N79;

N80;N81;N82;N83;
N84;N89;N90;N87;
N88;N85;N86;N91;
N92;N93;N94;N95
*/

fn state2ubl(state: &str) -> String {
    let u_dict: HashMap<char, char> = [
        ('A', 'U'),
        ('B', 'F'),
        ('C', 'R'),
        ('D', 'B'),
        ('E', 'L'),
        ('F', 'D'),
    ]
    .iter()
    .cloned()
    .collect();
    let state_split: Vec<&str> = state.split(';').collect();
    let dim = calc_dim(state_split.len());
    let dim_2 = dim.pow(2);
    let mut s = String::new();
    for f in state_split.iter() {
        s.push(u_dict[&f.chars().next().unwrap()]);
    }
    s[0..dim_2].to_string()
        + &s[2 * dim_2..3 * dim_2]
        + &s[dim_2..2 * dim_2]
        + &s[5 * dim_2..]
        + &s[4 * dim_2..5 * dim_2]
        + &s[3 * dim_2..4 * dim_2]
}

fn checker_state2cbl(init_state: &str, dim: usize) -> String {
    let dim2 = dim.pow(2);
    let u_dict = HashMap::from([
        ("B", "A"),
        ("C", "B"),
        ("D", "C"),
        ("E", "D"),
        ("F", "E"),
        ("A", "F"),
    ]);
    let mut state = init_state.split(";").collect::<Vec<&str>>();
    for i in 0..state.len() {
        let x = i % dim2 / dim;
        let y = i % dim2 % dim;
        if (x + y) % 2 == 1 {
            state[i] = u_dict[&state[i]];
        }
    }
    state2ubl(&state.join(";"))
}

fn distinct_state2ubl(init_state: &str, dim: usize) -> String {
    let dim2 = dim.pow(2);
    let arr = ["A", "B", "C", "D", "E", "F"];
    let state = init_state
        .split(";")
        .map(|s| arr[s[1..].parse::<usize>().unwrap() / dim2])
        .collect::<Vec<&str>>();
    state2ubl(&state.join(";"))
}

fn solve_cube_by_rule(
    puzzle: &solver::Puzzle,
    move_map: &HashMap<String, String>,
    current_solution: &String,
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) -> Option<String> {
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let actions = cube_moves::create_actions(&allowed_moves);
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);

    let current_solution = current_solution
        .split(".")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let rotate = count_center_cube_rot(
        &(0..6 * dim * dim).collect(),
        &current_solution,
        &actions,
        dim,
    );

    // println!("{:?}", sol_state);
    let mut allowed_moves_inv = HashMap::new();
    for (k, v) in allowed_moves.iter() {
        if k.starts_with("-") {
            allowed_moves_inv.insert(k[1..].to_string(), v);
        } else {
            allowed_moves_inv.insert(format!("-{}", k), v);
        }
    }
    let mut state = init_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_front_face(&state, &sol_state, &actions, dim);
    state = end_state;
    moves.extend(m);
    println!(
        "end up: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    let (end_state, m) = solve_back_face(&state, &sol_state, &actions, dim);
    state = end_state;
    moves.extend(m);
    println!(
        "end back: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    let (end_state, m) = resolve_center_and_inversion(&state, &sol_state, &actions, dim);
    state = end_state;
    moves.extend(m);
    println!(
        "end resolve inversion: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    for y in 1..dim / 2 {
        for _ in 0..y - 1 {
            print!("  ");
        }
        for x in y + 1..dim - y - 1 {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
            let mut idx_map = HashMap::new();
            for (i, idx) in target_idx.iter().enumerate() {
                idx_map.insert(sol_state[*idx], i);
            }
            let mut p = Vec::new();
            for idx in target_idx.iter() {
                p.push(idx_map[&state[*idx]]);
            }
            let mut cnt = 0;
            for i in 0..p.len() {
                for j in i + 1..p.len() {
                    if p[i] > p[j] {
                        cnt += 1;
                    }
                }
            }
            if cnt % 2 == 1 {
                print!("x ");
            } else {
                print!("o ");
            }
        }
        println!("");
    }

    let (end_state, m) = solve_up_face(&state, &sol_state, &actions, dim);
    state = end_state;
    moves.extend(m);
    println!(
        "end up: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );

    let (end_state, m) = solve_left_face(&state, &sol_state, &actions, dim);
    for y in 1..dim / 2 {
        for _ in 0..y - 1 {
            print!("  ");
        }
        for x in y + 1..dim - y - 1 {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
            let mut idx_map = HashMap::new();
            for (i, idx) in target_idx.iter().enumerate() {
                idx_map.insert(sol_state[*idx], i);
            }
            let mut p = Vec::new();
            for idx in target_idx.iter() {
                p.push(idx_map[&state[*idx]]);
            }
            let mut cnt = 0;
            for i in 0..p.len() {
                for j in i + 1..p.len() {
                    if p[i] > p[j] {
                        cnt += 1;
                    }
                }
            }
            if cnt % 2 == 1 {
                print!("x ");
            } else {
                print!("o ");
            }
        }
        println!("");
    }

    // let (end_state, m) = solve_orange_middle(
    //     &state,
    //     &sol_state,
    //     &allowed_moves_inv,
    //     &actions,
    //     &movable_pos,
    //     dim,
    // );
    state = end_state;
    moves.extend(m);
    println!(
        "end left: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    {
        let mut inversion = 0;
        for i in 0..state.len() {
            for j in i + 1..state.len() {
                if state[i] > state[j] {
                    inversion += 1;
                }
            }
        }
        println!("inversion: {}", inversion % 2);
    }
    {
        let mut inversion = 0;
        for i in 0..sol_state.len() {
            if i % (dim * dim) == (dim * dim) / 2 {
                continue;
            }
            for j in i + 1..sol_state.len() {
                if j % (dim * dim) == (dim * dim) / 2 {
                    continue;
                }
                if sol_state[i] > sol_state[j] {
                    inversion += 1;
                }
            }
        }
        println!("inversion2: {}", inversion % 2);
    }
    // for k in 0..6 {
    //     for i in 0..dim {
    //         for j in 0..dim {
    //             // if state[k * dim * dim + i * dim + j] == k * dim * dim + i * dim + j {
    //             //     print!("o ");
    //             // } else {
    //             //     print!("x ");
    //             // }
    //             print!("{:4} ", state[k * dim * dim + i * dim + j]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    // for k in 0..6 {
    //     for i in 0..dim {
    //         for j in 0..dim {
    //             if state[k * dim * dim + i * dim + j] == k * dim * dim + i * dim + j {
    //                 print!("o ");
    //             } else {
    //                 print!("x ");
    //             }
    //             // print!("{:4} ", state[k * dim * dim + i * dim + j]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    for y in 1..dim / 2 {
        for _ in 0..y - 1 {
            print!("  ");
        }
        for x in y + 1..dim - y - 1 {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
            let mut idx_map = HashMap::new();
            for (i, idx) in target_idx.iter().enumerate() {
                idx_map.insert(sol_state[*idx], i);
            }
            let mut p = Vec::new();
            for idx in target_idx.iter() {
                p.push(idx_map[&state[*idx]]);
            }
            let mut cnt = 0;
            for i in 0..p.len() {
                for j in i + 1..p.len() {
                    if p[i] > p[j] {
                        cnt += 1;
                    }
                }
            }
            if cnt % 2 == 1 {
                print!("x ");
            } else {
                print!("o ");
            }
        }
        println!("");
    }

    let rotate2 = count_center_cube_rot(&init_state, &moves, &actions, dim);
    for i in 0..6 {
        println!("rot {}: {}", i, (rotate[i] + rotate2[i]) % 4);
    }

    let (end_state, m) = solve_green_middle2(&state, &sol_state, &actions, dim);
    state = end_state;
    moves.extend(m);
    println!(
        "end green: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );

    {
        for i in 0..6 {
            let final_rot = cube_moves::calc_face_rot(&state, &sol_state, dim, i);
            let rot_action = [
                format!("-d{}", dim - 1),
                "f0".to_string(),
                "r0".to_string(),
                format!("-f{}", dim - 1),
                format!("-r{}", dim - 1),
                "d0".to_string(),
            ];
            if final_rot != 0 {
                for _ in 0..4 - final_rot {
                    state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
                }
            }
            for j in 1..dim - 1 {
                for k in 1..dim - 1 {
                    if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k]
                    {
                        println!("Could not resolve faces");
                        for k in 0..6 {
                            for i in 0..dim {
                                for j in 0..dim {
                                    // if state[k * dim * dim + i * dim + j] == k * dim * dim + i * dim + j {
                                    //     print!("o ");
                                    // } else {
                                    //     print!("x ");
                                    // }
                                    print!("{:4} ", state[k * dim * dim + i * dim + j]);
                                }
                                println!("");
                            }
                            println!("");
                        }
                        return None;

                        // println!(
                        //     "{} {}",
                        //     state[i * dim * dim + j * dim + k],
                        //     sol_state[i * dim * dim + j * dim + k]
                        // );
                    }
                }
            }

            if final_rot != 0 {
                for _ in 0..final_rot {
                    state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
                }
            }
        }
    }

    let (end_state, m) = cube_moves::solve_cube_edges(&state, &sol_state, &actions, dim);
    state = end_state;
    moves.extend(m);
    // for k in 0..6 {
    //     for i in 0..dim {
    //         for j in 0..dim {
    //             // if state[k * dim * dim + i * dim + j] == k * dim * dim + i * dim + j {
    //             //     print!("o ");
    //             // } else {
    //             //     print!("x ");
    //             // }
    //             print!("{:4} ", state[k * dim * dim + i * dim + j]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    println!(
        "end edges: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );

    // let mut rot0 = calc_rot0(&state, &sol_state, dim);
    // let mut rot1 = calc_rot1(&state, &sol_state, dim);
    // while rot0 % 4 != 0 {
    //     state = cube_moves::apply_action(&state, &actions["r0"]);
    //     rot0 += 1;
    // }
    // while rot1 % 4 != 0 {
    //     rot1 += 1;
    //     state = cube_moves::apply_action(&state, &actions["d0"]);
    // }

    for y in 1..dim / 2 {
        for _ in 0..y - 1 {
            print!("  ");
        }
        for x in y + 1..dim - y - 1 {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
            let mut ok = true;
            let rot0 = cube_moves::calc_face_rot(&state, &sol_state, dim, 0);
            for i in 0..4 {
                if sol_state[target_idx[i]] != state[target_idx[(i + rot0) % 4]] {
                    ok = false;
                }
            }
            let rot2 = cube_moves::calc_face_rot(&state, &sol_state, dim, 2);
            for i in 0..4 {
                if sol_state[target_idx[i + 4]] != state[target_idx[(i + rot2) % 4 + 4]] {
                    ok = false;
                }
            }
            let rot4 = cube_moves::calc_face_rot(&state, &sol_state, dim, 4);
            for i in 0..4 {
                if sol_state[target_idx[i + 8]] != state[target_idx[(i + rot4) % 4 + 8]] {
                    ok = false;
                }
            }
            let rot5 = cube_moves::calc_face_rot(&state, &sol_state, dim, 5);
            for i in 0..4 {
                if sol_state[target_idx[i + 12]] != state[target_idx[(i + rot5) % 4 + 12]] {
                    ok = false;
                }
            }
            if !ok {
                print!("x ");
            } else {
                print!("o ");
            }
        }
        println!("");
    }

    // for k in 0..6 {
    //     for i in 0..dim {
    //         for j in 0..dim {
    //             // if state[k * dim * dim + i * dim + j] == k * dim * dim + i * dim + j {
    //             //     print!("o ");
    //             // } else {
    //             //     print!("x ");
    //             // }
    //             print!("{:4} ", state[k * dim * dim + i * dim + j]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    // Edges can be solved in a shorter steps by the solver.

    // for k in 0..6 {
    //     for i in 0..dim {
    //         for j in 0..dim {
    //             if state[k * dim * dim + i * dim + j] == sol_state[k * dim * dim + i * dim + j] {
    //                 print!("o ");
    //             } else {
    //                 print!("x ");
    //             }
    //             // print!("{:4} ", state[k * dim * dim + i * dim + j]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    // println!("Solved {}", moves.len());
    // for k in 0..6 {
    //     for i in 0..dim {
    //         for j in 0..dim {
    //             // if state[k * dim * dim + i * dim + j] == k * dim * dim + i * dim + j {
    //             //     print!("o ");
    //             // } else {
    //             //     print!("x ");
    //             // }
    //             print!("{:4} ", state[k * dim * dim + i * dim + j]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }

    let mut position_map = vec![0; 6 * dim * dim];
    for i in 0..6 {
        position_map[i * dim * dim] = 9 * i;
        for j in 1..dim - 1 {
            position_map[i * dim * dim + j] = 9 * i + 1;
        }
        position_map[i * dim * dim + dim - 1] = 9 * i + 2;
        for j in 1..dim - 1 {
            position_map[i * dim * dim + j * dim] = 9 * i + 3;
            for k in 1..dim - 1 {
                position_map[i * dim * dim + j * dim + k] = 9 * i + 4;
            }
            position_map[i * dim * dim + j * dim + dim - 1] = 9 * i + 5;
        }
        position_map[i * dim * dim + (dim - 1) * dim] = 9 * i + 6;
        for j in 1..dim - 1 {
            position_map[i * dim * dim + (dim - 1) * dim + j] = 9 * i + 7;
        }
        position_map[i * dim * dim + (dim - 1) * dim + dim - 1] = 9 * i + 8;
    }

    let cur_state = solver::list_to_state(&state, &piece_list);
    if let Some(last_move) = solve_cube_by_solver(
        &cur_state,
        &puzzle.solution_state,
        move_map,
        allowed_moves,
        dim,
    ) {
        for m in last_move.split(".") {
            state = cube_moves::apply_action(&state, &actions[m]);
        }

        moves.push(last_move);

        let mut final_rot = vec![0; 6];
        let mut valid_final = true;
        for i in 0..6 {
            final_rot[i] = cube_moves::calc_face_rot(&state, &sol_state, dim, i);
            let val = position_map[state[i * dim * dim + dim + 1]];
            if val != i * 9 + 4 {
                valid_final = false;
                println!("invalid center");
            }
            for j in 1..dim - 1 {
                for k in 1..dim - 1 {
                    if position_map[state[i * dim * dim + j * dim + k]] != val {
                        valid_final = false;
                        println!("different color");
                    }
                }
            }
            let rot_action = [
                format!("-d{}", dim - 1),
                "f0".to_string(),
                "r0".to_string(),
                format!("-f{}", dim - 1),
                format!("-r{}", dim - 1),
                "d0".to_string(),
            ];
            if final_rot[i] != 0 {
                for _ in 0..4 - final_rot[i] {
                    state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
                }
            }
            for j in 1..dim - 1 {
                for k in 1..dim - 1 {
                    if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k]
                    {
                        valid_final = false;
                        // println!(
                        //     "{} {}",
                        //     state[i * dim * dim + j * dim + k],
                        //     sol_state[i * dim * dim + j * dim + k]
                        // );
                    }
                }
            }

            if final_rot[i] != 0 {
                for _ in 0..final_rot[i] {
                    state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
                }
            }
            let val0 = position_map[state[i * dim * dim + 1]];
            let val1 = position_map[state[i * dim * dim + dim]];
            let val2 = position_map[state[i * dim * dim + dim + dim - 1]];
            let val3 = position_map[state[i * dim * dim + (dim - 1) * dim + 1]];
            for j in 1..dim - 1 {
                if position_map[state[i * dim * dim + j]] != val0 {
                    valid_final = false;
                }
                if position_map[state[i * dim * dim + j * dim]] != val1 {
                    valid_final = false;
                }
                if position_map[state[i * dim * dim + j * dim + dim - 1]] != val2 {
                    valid_final = false;
                }
                if position_map[state[i * dim * dim + (dim - 1) * dim + j]] != val3 {
                    valid_final = false;
                }
            }
        }
        if final_rot.iter().sum::<usize>() % 2 == 1 {
            valid_final = false;
            println!("invalid parity");
        }
        if !valid_final {
            println!("Invalid final state");
            for i in 0..6 {
                for j in 0..dim {
                    for k in 0..dim {
                        print!("{:3} ", state[i * dim * dim + j * dim + k]);
                    }
                    println!("");
                }
            }
            return None;
        }
        if valid_final {
            let (end_state, m) = bfs_cube_face_rot(&state, &sol_state, &actions, dim);
            if end_state != sol_state {
                println!("Could not resolve face rotation");
                return None;
            }
            moves.extend(m);
            // println!("Current move: {}", moves.join("."));
            // let mut state_for_solver = Vec::new();
            // for i in 0..6 {
            //     for j in &[0, 1, dim - 1] {
            //         for k in &[0, 1, dim - 1] {
            //             state_for_solver.push(
            //                 position_map[state[i * dim * dim + j * dim + k]]
            //                     + position_map[state[i * dim * dim + j * dim + k]] / 9 * 6,
            //             );
            //         }
            //     }
            //     for j in 0..6 {
            //         state_for_solver.push((15 * i + 9 + j + 15 * final_rot[j]) % (6 * 15));
            //     }
            // }
            // println!("Solver state: {:?}", state_for_solver);
        }

        Some(moves.join("."))
    } else {
        None
    }
}

fn solve_cube_by_solver(
    init_state: &String,
    sol_state: &String,
    move_map: &HashMap<String, String>,
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) -> Option<String> {
    let standard_cube = sol_state[0..2 * dim.pow(2) - 1] == vec!["A"; dim.pow(2)].join(";");
    let checker_cube = sol_state.starts_with("A;B;A;B;A;B;A;B;A");

    let state = if standard_cube {
        state2ubl(init_state)
    } else if checker_cube {
        // TODO: Support the case where A is even
        checker_state2cbl(init_state, dim)
    } else {
        distinct_state2ubl(init_state, dim)
    };
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let relative_path = std::path::Path::new(&project_dir).join("../rubiks-cube-NxNxN-solver");

    std::env::set_current_dir(&relative_path).unwrap();

    let output = std::process::Command::new("./rubiks-cube-solver.py")
        .arg("--state")
        .arg(state)
        .output()
        .expect("Failed to execute command");

    std::env::set_current_dir(&project_dir).unwrap();

    // 出力を文字列に変換
    let output_str = String::from_utf8_lossy(&output.stdout);

    // 出力を行ごとに分割
    let outputs: Vec<&str> = output_str.split('\n').collect();

    // 出力の最後の行をチェック
    let last_line = outputs.last().unwrap_or(&"");
    let mut sol = "";

    if last_line.starts_with("Solution:") {
        sol = last_line.split(": ").nth(1).unwrap_or("");
    } else {
        // "Solution:"を含む行を後ろから検索
        for line in outputs.iter().rev() {
            if line.contains("Solution:") {
                sol = line
                    .split("Solution: ")
                    .nth(1)
                    .unwrap_or("")
                    .split("2023-")
                    .next()
                    .unwrap_or("");
                break;
            }
        }
    }
    if sol.is_empty() {
        println!("Could not solved by solver");
        println!("{:?}", outputs);
        return None;
    }
    for m in sol.split(" ") {
        if !move_map.contains_key(m) {
            println!("invalid move: `{}`", m);
        }
    }
    // join M[m] for m in sol.split(" ") with "."
    let mut mmoves = sol
        .split(" ")
        .map(|m| move_map[m].clone())
        .collect::<Vec<String>>()
        .join(".");

    let mut new_state = init_state.to_string();
    for m in mmoves.split(".") {
        let new_state_vec: Vec<&str> = new_state.split(";").collect();
        // reorder new_state_vec with moves[m] and join with ";"
        new_state = allowed_moves[m]
            .iter()
            .map(|i| new_state_vec[*i as usize])
            .collect::<Vec<&str>>()
            .join(";")
            .to_string();
    }

    let base_move = ["r", "d", "f", "-r", "-d", "-f"]
        .iter()
        .map(|&c| {
            (0..dim)
                .map(move |i| format!("{}{}", c, i))
                .collect::<Vec<String>>()
                .join(".")
        })
        .collect::<Vec<String>>();
    let mut manipulations: Vec<String> = Vec::new();
    manipulations.push("".to_string());
    for i1 in base_move.iter() {
        manipulations.push(i1.to_string());
    }
    for i1 in base_move.iter() {
        for i2 in base_move.iter() {
            manipulations.push(format!("{}.{}", i1, i2));
        }
    }
    for i1 in base_move.iter() {
        for i2 in base_move.iter() {
            for i3 in base_move.iter() {
                manipulations.push(format!("{}.{}.{}", i1, i2, i3));
            }
        }
    }
    for i1 in base_move.iter() {
        for i2 in base_move.iter() {
            for i3 in base_move.iter() {
                for i4 in base_move.iter() {
                    manipulations.push(format!("{}.{}.{}.{}", i1, i2, i3, i4));
                }
            }
        }
    }

    for init_moves in manipulations.iter() {
        let mut temp_state = new_state.clone();
        if init_moves.len() > 0 {
            for m in init_moves.split(".") {
                let temp_state_vec: Vec<&str> = temp_state.split(";").collect();
                // reorder new_state_vec with moves[m] and join with ";"
                temp_state = allowed_moves[m]
                    .iter()
                    .map(|i| temp_state_vec[*i as usize])
                    .collect::<Vec<&str>>()
                    .join(";")
                    .to_string();
            }
        }

        if temp_state == *sol_state {
            if init_moves.len() > 0 {
                if mmoves.len() > 0 {
                    mmoves.push_str(".");
                }
                mmoves.push_str(&init_moves);
            }
            break;
        }
    }
    // println!("solution: {}", mmoves);
    Some(mmoves)
}

fn solve_cube(
    puzzle: &solver::Puzzle,
    current_solution: &String,
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) -> Option<String> {
    let move_map = move_translation(dim);
    let init_state = &puzzle.initial_state;
    let sol_state = &puzzle.solution_state;

    let checker_cube = sol_state.starts_with("A;B;A;B;A;B;A;B;A");
    let distinct_cube = sol_state.starts_with("N0");

    if dim % 2 == 0 && checker_cube {
        return None;
    }
    if !distinct_cube {
        return None;
    }
    if dim <= 3 {
        solve_cube_by_solver(init_state, sol_state, &move_map, allowed_moves, dim)
    } else {
        solve_cube_by_rule(puzzle, &move_map, current_solution, allowed_moves, dim)
    }
}

fn solve() {
    let puzzle_info = solver::load_puzzle_info();
    let puzzles = solver::load_puzzle();
    let submission = solver::load_submission();
    let mut submission_df: HashMap<usize, String> = HashMap::new();
    for sub in submission.iter() {
        submission_df.insert(sub[0].parse::<usize>().unwrap(), sub[1].clone());
    }
    let mut allowed_moves: HashMap<String, HashMap<String, Vec<i16>>> = HashMap::new();
    for (k, v) in puzzle_info.iter() {
        allowed_moves.insert(k.to_string(), v.clone());
    }
    for (id, row) in puzzles.iter().enumerate() {
        if !row.puzzle_type.starts_with("cube") {
            continue;
        }
        let dim = row
            .puzzle_type
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let moves = &puzzle_info[&row.puzzle_type];
        if let Some(result) = solve_cube(&row, &submission_df[&id], &moves, dim) {
            println!("solved id: {}", id);
            // println!("solution: {}", result);
            let result = solver::cancel_moves_in_cube(&result);
            // validation
            let mut state = row.initial_state.clone();
            for m in result.split(".") {
                let state_vec: Vec<&str> = state.split(";").collect();
                // reorder new_state_vec with moves[m] and join with ";"
                state = moves[m]
                    .iter()
                    .map(|i| state_vec[*i as usize])
                    .collect::<Vec<&str>>()
                    .join(";")
                    .to_string();
            }

            // let (piece_map, piece_list) = solver::gen_piece_map(&row.solution_state);
            // let sol_state = solver::state_to_list(&state, &piece_map);
            // for k in 0..6 {
            //     for i in 0..dim {
            //         for j in 0..dim {
            //             // if sol_state[k * dim * dim + i * dim + j] == k * dim * dim + i * dim + j {
            //             //     print!("o ");
            //             // } else {
            //             //     print!("x ");
            //             // }
            //             print!("{:4} ", sol_state[k * dim * dim + i * dim + j]);
            //         }
            //         println!("");
            //     }
            //     println!("");
            // }
            // println!("sol_state: {:?}", row.solution_state);
            // println!("state    : {}", state);
            // for (i, s) in state.split(";").enumerate() {
            //     print!("{:3} ", s[1..].parse::<usize>().unwrap() % dim.pow(2));
            //     if i % dim == dim - 1 {
            //         println!("");
            //     }
            //     if i % dim.pow(2) == dim.pow(2) - 1 {
            //         println!("");
            //     }
            // }
            if row.solution_state != state {
                println!("failed id: {}", id);
                // println!("expected: {}", row.solution_state);
                // println!("actual  : {}", state);
            } else {
                // assert_eq!(row.solution_state, state);
                let mmoves_length = result.split(".").collect::<Vec<&str>>().len();
                let best_moves_length = submission_df[&id].split(".").collect::<Vec<&str>>().len();
                println!("find: {}, best: {}", mmoves_length, best_moves_length);
                if mmoves_length < best_moves_length {
                    println!("solution: {}", result);
                    submission_df.insert(id, result);
                }
            }
        } else {
            // println!("failed id: {}", id);
        }
    }
    let mut wtr = csv::Writer::from_path("../submission_latest2.csv").unwrap();
    // ヘッダーを書き出す
    wtr.write_record(&["id", "moves"]).unwrap();

    // データ行を書き出す
    for i in 0..submission_df.len() {
        wtr.write_record(&[&i.to_string(), &submission_df[&i]])
            .unwrap();
    }

    wtr.flush().unwrap();
}

#[allow(dead_code)]
fn create_action(
    allowed_moves: &HashMap<String, Vec<i16>>,
) -> (Vec<Vec<(usize, usize)>>, Vec<String>) {
    let mut action: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut action_str: Vec<String> = Vec::new();
    for (k, v) in allowed_moves.iter() {
        let mut action_k: Vec<(usize, usize)> = Vec::new();
        for (idx, i) in v.iter().enumerate() {
            if idx == *i as usize {
                continue;
            }
            action_k.push((idx, *i as usize));
        }
        action.push(action_k);
        action_str.push(k.to_string());
    }
    (action, action_str)
}

#[allow(dead_code)]
fn attempt() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_7/7/7"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["r1"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = cube_moves::apply_action(&state, &action_map["d6"]);
    state = cube_moves::apply_action(&state, &action_map["-r1"]);
    // state = cube_moves::apply_action(&state, &action_map["-r3"]);
    // state = cube_moves::apply_action(&state, &action_map["-r0"]);
    for i in 0..6 {
        for j in 1..6 {
            for k in 1..6 {
                print!("{:3} ", state[i * 49 + j * 7 + k]);
            }
            println!("");
        }
        println!("");
    }
    println!("{:?}", state);
}

#[allow(dead_code)]
fn attempt1() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_7/7/7"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["r2"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = cube_moves::apply_action(&state, &action_map["f0"]);
    state = cube_moves::apply_action(&state, &action_map["f6"]);
    for _ in 0..4 {
        state = cube_moves::apply_action(&state, &action_map["-d2"]);
        state = cube_moves::apply_action(&state, &action_map["r0"]);
        state = cube_moves::apply_action(&state, &action_map["r0"]);
    }
    state = cube_moves::apply_action(&state, &action_map["-d2"]);
    state = cube_moves::apply_action(&state, &action_map["-f0"]);
    state = cube_moves::apply_action(&state, &action_map["-f6"]);
    state = cube_moves::apply_action(&state, &action_map["-r2"]);
    for i in 0..6 {
        for j in 0..7 {
            for k in 0..7 {
                print!("{:3} ", state[i * 49 + j * 7 + k]);
            }
            println!("");
        }
        println!("");
    }
    println!("{:?}", state);
}

fn sequence_moves_for_face_rot(dim: usize) -> Vec<Vec<String>> {
    let base_sequence1 = Vec::from([
        format!("-d{}", dim - 1),
        "-r0".to_string(),
        format!("-r{}", dim - 1),
        "f0".to_string(),
        format!("f{}", dim - 1),
        "d0".to_string(),
        format!("d{}", dim - 1),
        "-r0".to_string(),
        "-d0".to_string(),
        format!("-d{}", dim - 1),
        "-f0".to_string(),
        format!("-f{}", dim - 1),
        "r0".to_string(),
        format!("r{}", dim - 1),
    ]);
    let base_sequence2 = Vec::from([
        format!("-r{}", dim - 1),
        "r0".to_string(),
        format!("d{}", dim - 1),
        format!("r{}", dim - 1),
        "-r0".to_string(),
        format!("d{}", dim - 1),
        format!("d{}", dim - 1),
        format!("-r{}", dim - 1),
        "r0".to_string(),
        format!("d{}", dim - 1),
        format!("r{}", dim - 1),
        "-r0".to_string(),
        format!("d{}", dim - 1),
        format!("d{}", dim - 1),
    ]);

    let mut sequence1_single = vec![base_sequence1];
    let mut sequence_one_face = sequence1_single.clone();
    for _ in 0..3 {
        sequence1_single = sequence1_single
            .iter()
            .map(|s| cube_moves::rot_sequence_d(&s, dim))
            .collect();
        sequence_one_face.extend(sequence1_single.clone());
    }
    sequence_one_face.push(base_sequence2);
    let mut sequence_two_faces = sequence_one_face.clone();
    sequence_two_faces.extend(
        sequence_one_face
            .iter()
            .map(|s| cube_moves::rot2_sequence_r(&s, dim)),
    );
    let mut res = sequence_two_faces.clone();
    res.extend(
        sequence_two_faces
            .iter()
            .map(|s| cube_moves::rot_sequence_r(&s, dim)),
    );
    res.extend(
        sequence_two_faces
            .iter()
            .map(|s| cube_moves::rot_sequence_f(&s, dim)),
    );
    res
}

#[allow(dead_code)]
fn attempt3() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_7/7/7"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = (0..6 * 7 * 7).collect::<Vec<usize>>();
    // let idx = (0..6).map(|i| i * 49 + 24).collect::<Vec<usize>>();
    let idx = cube_moves::target_idx_edge(7, 2, 3, &(0..6).collect::<Vec<usize>>());
    let mut inversion = 0;
    for i in 0..idx.len() {
        for j in i + 1..idx.len() {
            if state[idx[i]] > state[idx[j]] {
                inversion += 1;
            }
        }
    }
    // let mut perms = (0..6).permutations(6).collect::<Vec<Vec<usize>>>();
    // let mut perm_map = HashMap::new();
    // for (i, p) in perms.iter().enumerate() {
    //     perm_map.insert(p.clone(), i);
    // }
    // let mut priority_qu = BinaryHeap::new();
    // priority_qu.push(Reverse((
    //     0,
    //     (0..6 * 7 * 7).map(|i| i / 49).collect::<Vec<usize>>(),
    // )));
    println!("inversion: {}", inversion);
    // let mut reach = 0;
    // let mut cost = vec![vec![std::i32::MAX; perms.len()]; 6];
    // while let Some(Reverse((cost, p))) = priority_qu.pop() {
    //     let p_idx = perm_map[&p];
    // }
    for _ in 0..5 {
        state = cube_moves::apply_action(&state, &action_map["f0"]);
        state = cube_moves::apply_action(&state, &action_map["d6"]);
        state = cube_moves::apply_action(&state, &action_map["-f0"]);
        state = cube_moves::apply_action(&state, &action_map["d6"]);
    }
    // state = cube_moves::apply_action(&state, &action_map["r3"]);
    // let mut inversion = 0;
    // for i in 0..idx.len() {
    //     for j in i + 1..idx.len() {
    //         if state[idx[i]] > state[idx[j]] {
    //             inversion += 1;
    //         }
    //     }
    // }
    // println!("inversion: {}", inversion);
    // state = cube_moves::apply_action(&state, &action_map["f3"]);
    // let mut inversion = 0;
    // for i in 0..idx.len() {
    //     for j in i + 1..idx.len() {
    //         if state[idx[i]] > state[idx[j]] {
    //             inversion += 1;
    //         }
    //     }
    // }
    // println!("inversion: {}", inversion);
    // state = cube_moves::apply_action(&state, &action_map["-d3"]);
    let mut inversion = 0;
    for i in 0..idx.len() {
        for j in i + 1..idx.len() {
            if state[idx[i]] > state[idx[j]] {
                inversion += 1;
            }
        }
    }
    println!("inversion: {}", inversion);
    for i in 0..6 {
        for j in 0..7 {
            for k in 0..7 {
                print!("{:3} ", state[i * 49 + j * 7 + k]);
            }
            println!("");
        }
        println!("");
    }
}

#[allow(dead_code)]
fn generate_cube_face_move_for_bfs(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = Vec::new();
    let actions = cube_moves::create_actions(&allowed_moves);
    let sol_state = (0..6 * dim * dim).collect::<Vec<usize>>();

    let sequences = sequence_moves_for_face_rot(dim);
    for sequence in sequences.iter() {
        let mut state = allowed_moves[&sequence[0]]
            .iter()
            .map(|v| *v as usize)
            .collect::<Vec<usize>>();
        for i in 1..sequence.len() {
            state = cube_moves::apply_action(&state, &actions[&sequence[i]]);
        }

        for face in 0..6 {
            for i in 0..dim {
                assert_eq!(state[face * dim * dim + i], sol_state[face * dim * dim + i]);
                assert_eq!(
                    state[face * dim * dim + (dim - 1) * dim + i],
                    sol_state[face * dim * dim + (dim - 1) * dim + i]
                );
            }
            for j in 1..dim - 1 {
                assert_eq!(
                    state[face * dim * dim + j * dim],
                    sol_state[face * dim * dim + j * dim]
                );
                assert_eq!(
                    state[face * dim * dim + j * dim + dim - 1],
                    sol_state[face * dim * dim + j * dim + dim - 1]
                );
            }
            for i in 1..dim - 1 {
                for j in 1..dim - 1 {
                    assert_eq!(state[face * dim * dim + i * dim + j] / (dim * dim), face);
                }
            }
        }
        let mut final_rot = vec![0; 6];
        for (i, rot_action) in [
            format!("-d{}", dim - 1),
            "f0".to_string(),
            "r0".to_string(),
            format!("-f{}", dim - 1),
            format!("-r{}", dim - 1),
            "d0".to_string(),
        ]
        .iter()
        .enumerate()
        {
            final_rot[i] = cube_moves::calc_face_rot(&state, &sol_state, dim, i);
            if final_rot[i] != 0 {
                for _ in 0..4 - final_rot[i] {
                    state = cube_moves::apply_action(&state, &actions[rot_action]);
                }
            }
            for j in 1..dim - 1 {
                for k in 1..dim - 1 {
                    assert_eq!(
                        state[i * dim * dim + j * dim + k],
                        sol_state[i * dim * dim + j * dim + k]
                    );
                }
            }

            if final_rot[i] != 0 {
                for _ in 0..final_rot[i] {
                    state = cube_moves::apply_action(&state, &actions[rot_action]);
                }
            }
        }
        // final_rot = final_rot.iter().map(|v| (4 - v) % 4).collect();
        result.push((final_rot, sequence.len()));
    }

    // println!("[");
    // for r in &result {
    //     println!("(Vec::from({:?}), {}),", r.0, r.1)
    // }
    // println!("]");
    let serialized = serde_json::to_string(&result).unwrap();
    std::fs::write("generate_cube_face_move_for_bfs.json", serialized).ok();
}

fn bfs_cube_face_rot(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    cube_actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut priority_qu = BinaryHeap::new();
    let contents = std::fs::read_to_string("generate_cube_face_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let mut prev_action: Vec<Option<usize>> = vec![None; 4096];
    let mut prev_state: Vec<Option<usize>> = vec![None; 4096];
    let mut cost: Vec<i32> = vec![std::i32::MAX; 4096];
    let mut init_rot = vec![0; 6];
    for i in 0..6 {
        init_rot[i] = cube_moves::calc_face_rot(&cur_state, &sol_state, dim, i);
    }
    fn rot_to_idx(rot: &Vec<usize>) -> usize {
        let mut res = 0;
        for i in 0..6 {
            res = 4 * res + rot[i];
        }
        res
    }
    priority_qu.push(Reverse((0, init_rot.clone())));
    cost[rot_to_idx(&init_rot)] = 0;

    while let Some(Reverse((cur_cost, state))) = priority_qu.pop() {
        let cur_idx = rot_to_idx(&state);
        let c = cost[cur_idx];
        if cur_cost > c {
            continue;
        }
        if cur_idx == 0 {
            let mut cur_rot_idx = 0;
            let mut sol_actions = Vec::new();
            while let Some(action_idx) = prev_action[cur_rot_idx] {
                sol_actions.push(action_idx);
                cur_rot_idx = prev_state[cur_rot_idx].unwrap();
            }

            let action_strings = sequence_moves_for_face_rot(dim);
            let mut end_state = cur_state.clone();
            let mut sol_moves = Vec::new();

            for action_idx in sol_actions.iter().rev() {
                for act in action_strings[*action_idx].iter() {
                    sol_moves.push(act.clone());
                    end_state = cube_moves::apply_action(&end_state, &cube_actions[act]);
                }
                let mut cur_face_rot = vec![0; 6];
                for i in 0..6 {
                    cur_face_rot[i] = cube_moves::calc_face_rot(&end_state, &sol_state, dim, i);
                }
            }
            return (end_state, sol_moves);
        }
        for (action_idx, (add, action_cost)) in actions.iter().enumerate() {
            let next_rot: Vec<usize> = (0..6).map(|i| (state[i] + add[i]) % 4).collect();
            let next_rot_idx = rot_to_idx(&next_rot);
            if action_cost + c < cost[next_rot_idx] {
                cost[next_rot_idx] = action_cost + c;
                prev_action[next_rot_idx] = Some(action_idx);
                prev_state[next_rot_idx] = Some(cur_idx);
                priority_qu.push(Reverse((action_cost + c, next_rot)));
            }
        }
    }
    (Vec::new(), Vec::new())
}

fn get_perm_map() -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    let mut perm_map: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut perm_list: Vec<Vec<usize>> = Vec::new();
    for p in (0..8).permutations(8) {
        perm_map.insert(p.clone(), perm_list.len());
        perm_list.push(p);
    }
    (perm_list, perm_map)
}

fn bfs0(parity: usize) -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let (perm_list, perm_map) = get_perm_map();
    let mut priority_qu = BinaryHeap::new();
    let actions = [
        (Vec::from([2, 0, 3, 1, 4, 5, 6, 7]), 1), // rotate top
        (Vec::from([1, 3, 0, 2, 4, 5, 6, 7]), 1), // rotate top (rev)
        (Vec::from([0, 1, 2, 3, 6, 4, 7, 5]), 1), // ratate btm
        (Vec::from([0, 1, 2, 3, 5, 7, 4, 6]), 1), // rotate btm (rev)
        (Vec::from([5, 1, 0, 3, 4, 7, 6, 2]), 3), // rotate right
        (Vec::from([7, 1, 5, 3, 4, 2, 6, 0]), 4), // rotate right2
        (Vec::from([2, 1, 7, 3, 4, 0, 6, 5]), 3), // rotate right (rev)
        (Vec::from([0, 3, 2, 6, 1, 5, 4, 7]), 3), // rotate left
        (Vec::from([0, 6, 2, 4, 3, 5, 1, 7]), 4), // rotate left2
        (Vec::from([0, 4, 2, 1, 6, 5, 3, 7]), 3), // rotate left (rev)
    ];
    let mut start = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let mut prev_state: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; perm_list.len()];
    for i in 0..4 {
        for j in 0..4 {
            if (i + j) % 2 == parity {
                priority_qu.push(Reverse((0, start.clone())));
            }
            let idx = perm_map[&start];
            cost[idx] = 0;
            start = cube_moves::apply_perm(&start, &actions[0].0);
        }
        start = cube_moves::apply_perm(&start, &actions[2].0);
    }
    while let Some(Reverse((cur_cost, state))) = priority_qu.pop() {
        let idx = perm_map[&state];
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, action_cost)) in actions.iter().enumerate() {
            let next_state = cube_moves::apply_perm(&state, &perm);
            let next_idx = perm_map[&next_state];
            if action_cost + c < cost[next_idx] {
                cost[next_idx] = action_cost + c;
                prev_state[next_idx] = Some(idx);
                prev_action[next_idx] = Some(action_idx);
                priority_qu.push(Reverse((action_cost + c, next_state)));
            }
        }
    }
    let mut max_cost = 0;
    let mut max_idx = 0;
    for (idx, c) in cost.iter().enumerate() {
        if *c > max_cost {
            max_cost = *c;
            max_idx = idx;
        }
    }
    println!("max_cost: {} {:?}", max_cost, perm_list[max_idx]);
    (prev_state, prev_action)
}

fn bfs1() -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let (perm_list, perm_map) = get_perm_map();
    let mut priority_qu = BinaryHeap::new();
    let actions = [
        (Vec::from([2, 0, 3, 1, 4, 5, 6, 7]), 1, 0, 1), // rotate top
        (Vec::from([1, 3, 0, 2, 4, 5, 6, 7]), 3, 0, 1), // rotate top (rev)
        (Vec::from([0, 1, 2, 3, 6, 4, 7, 5]), 0, 1, 1), // ratate btm
        (Vec::from([0, 1, 2, 3, 5, 7, 4, 6]), 0, 3, 1), // rotate btm (rev)
        (Vec::from([5, 1, 0, 3, 4, 7, 6, 2]), 3, 0, 3), // rotate right
        (Vec::from([7, 1, 5, 3, 4, 2, 6, 0]), 2, 0, 4), // rotate right2
        (Vec::from([2, 1, 7, 3, 4, 0, 6, 5]), 1, 0, 3), // rotate right (rev)
        (Vec::from([0, 3, 2, 6, 1, 5, 4, 7]), 3, 0, 3), // rotate left
        (Vec::from([0, 6, 2, 4, 3, 5, 1, 7]), 2, 0, 4), // rotate left2
        (Vec::from([0, 4, 2, 1, 6, 5, 3, 7]), 1, 0, 3), // rotate left (rev)
    ];
    let mut start = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let mut prev_state: Vec<Option<usize>> = vec![None; 16 * perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; 16 * perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; 16 * perm_list.len()];
    for i in 0..4 {
        for j in 0..4 {
            priority_qu.push(Reverse((0, start.clone(), j, i)));
            let idx = 16 * perm_map[&start] + 4 * j + i;
            cost[idx] = 0;
            start = cube_moves::apply_perm(&start, &actions[0].0);
        }
        start = cube_moves::apply_perm(&start, &actions[2].0);
    }
    while let Some(Reverse((cur_cost, state, rot0, rot1))) = priority_qu.pop() {
        let idx = 16 * perm_map[&state] + 4 * rot0 + rot1;
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, act_rot0, act_rot1, action_cost)) in actions.iter().enumerate() {
            let next_state = cube_moves::apply_perm(&state, &perm);
            let next_rot0 = (rot0 + act_rot0) % 4;
            let next_rot1 = (rot1 + act_rot1) % 4;
            let next_idx = 16 * perm_map[&next_state] + 4 * next_rot0 + next_rot1;
            if action_cost + c < cost[next_idx] {
                cost[next_idx] = action_cost + c;
                prev_state[next_idx] = Some(idx);
                prev_action[next_idx] = Some(action_idx);
                priority_qu.push(Reverse((action_cost + c, next_state, next_rot0, next_rot1)));
            }
        }
    }
    // let mut max_cost = 0;
    // let mut max_idx = 0;
    // let mut reachable = 0;
    // for (idx, c) in cost.iter().enumerate() {
    //     if *c == std::i32::MAX {
    //         continue;
    //     }
    //     reachable += 1;
    //     if *c > max_cost {
    //         max_cost = *c;
    //         max_idx = idx;
    //     }
    // }
    // println!("reachable: {}", reachable);
    // println!(
    //     "max_cost: {} {:?} {} {}",
    //     max_cost,
    //     perm_list[max_idx / 16],
    //     max_idx / 4 % 4,
    //     max_idx % 4
    // );
    (prev_state, prev_action)
}

fn bfs3() -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let (perm_list, perm_map) = get_perm_map();
    let mut priority_qu = BinaryHeap::new();
    let actions = [
        (Vec::from([1, 3, 0, 2, 4, 5, 6, 7]), 1, 0, 1), // rotate top
        (Vec::from([2, 0, 3, 1, 4, 5, 6, 7]), 3, 0, 1), // rotate top (rev)
        (Vec::from([0, 1, 2, 3, 5, 7, 4, 6]), 0, 1, 1), // rotate btm
        (Vec::from([0, 1, 2, 3, 6, 4, 7, 5]), 0, 3, 1), // ratate btm (rev)
        (Vec::from([0, 1, 6, 3, 4, 5, 7, 2]), 0, 0, 8), // rotate btm_right
        (Vec::from([0, 1, 7, 3, 4, 5, 2, 6]), 0, 0, 8), // rotate btm_right (inv)
        (Vec::from([0, 5, 2, 3, 4, 7, 6, 1]), 0, 0, 8), // rotate btm_left
        (Vec::from([0, 7, 2, 3, 4, 1, 6, 5]), 0, 0, 8), // rotate btm_left (rev)
        (Vec::from([6, 1, 0, 3, 4, 5, 2, 7]), 0, 0, 8), // rotate top_right
        (Vec::from([2, 1, 6, 3, 4, 5, 0, 7]), 0, 0, 8), // rotate top_right
        (Vec::from([5, 0, 2, 3, 4, 1, 6, 7]), 0, 0, 8), // rotate top_left
        (Vec::from([1, 5, 2, 3, 4, 0, 6, 7]), 0, 0, 8), // rotate top_left
    ];
    let mut start = Vec::from([0, 1, 2, 3, 4, 5, 6, 7]);
    let mut prev_state: Vec<Option<usize>> = vec![None; 16 * perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; 16 * perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; 16 * perm_list.len()];
    for i in 0..4 {
        for j in 0..4 {
            priority_qu.push(Reverse((0, start.clone(), j, i)));
            let idx = 16 * perm_map[&start] + 4 * j + i;
            cost[idx] = 0;
            start = cube_moves::apply_perm(&start, &actions[0].0);
        }
        start = cube_moves::apply_perm(&start, &actions[2].0);
    }
    while let Some(Reverse((cur_cost, state, rot0, rot1))) = priority_qu.pop() {
        let idx = 16 * perm_map[&state] + 4 * rot0 + rot1;
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, act_rot0, act_rot1, action_cost)) in actions.iter().enumerate() {
            let next_state = cube_moves::apply_perm(&state, &perm);
            let next_rot0 = (rot0 + act_rot0) % 4;
            let next_rot1 = (rot1 + act_rot1) % 4;
            let next_idx = 16 * perm_map[&next_state] + 4 * next_rot0 + next_rot1;
            if action_cost + c < cost[next_idx] {
                cost[next_idx] = action_cost + c;
                prev_state[next_idx] = Some(idx);
                prev_action[next_idx] = Some(action_idx);
                priority_qu.push(Reverse((action_cost + c, next_state, next_rot0, next_rot1)));
            }
        }
    }
    // let mut max_cost = 0;
    // let mut max_idx = 0;
    // let mut reachable = 0;
    // let mut printed = false;
    // for (idx, c) in cost.iter().enumerate() {
    //     if *c == std::i32::MAX {
    //         if idx % 16 == 0 {
    //             if !printed {
    //                 println!("{:?}", perm_list[idx / 16])
    //             }
    //             printed = true;
    //         }
    //         continue;
    //     }
    //     reachable += 1;
    //     if *c > max_cost {
    //         max_cost = *c;
    //         max_idx = idx;
    //     }
    // }
    // println!("reachable: {}", reachable);
    // println!(
    //     "max_cost: {} {:?} {} {}",
    //     max_cost,
    //     perm_list[max_idx / 16],
    //     max_idx / 4 % 4,
    //     max_idx % 4
    // );
    (prev_state, prev_action)
}

fn main() {
    solve();
}

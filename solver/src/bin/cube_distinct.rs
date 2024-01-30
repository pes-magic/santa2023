// Reference
// * https://github.com/merpig/RubiksProgram
// * https://www.kaggle.com/code/seanbearden/solve-all-nxnxn-cubes-w-traditional-solution-state
// Dependencies
// * https://github.com/dwalton76/rubiks-cube-NxNxN-solver

use itertools::Itertools;
use petgraph::visit::GraphBase;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

mod cube_moves;
use cube_moves::P3;

fn index_to_p3(idx: usize, dim: usize) -> P3 {
    let face = idx / dim.pow(2);
    let row = idx % dim.pow(2) / dim;
    let col = idx % dim;
    let end = dim - 1;
    match face {
        0 => P3::new(col, end - row, end),       // U
        1 => P3::new(col, 0, end - row),         // F
        2 => P3::new(end, col, end - row),       // R
        3 => P3::new(end - col, end, end - row), // B
        4 => P3::new(0, end - col, end - row),   // L
        5 => P3::new(col, row, 0),               // D
        _ => panic!("Invalid face"),
    }
}

fn calc_movable(actions: &HashMap<String, Vec<i16>>) -> Vec<Vec<usize>> {
    let piece_num = actions.values().next().unwrap().len();
    let mut movable = vec![Vec::new(); piece_num];
    for i in 0..piece_num {
        let mut qu = std::collections::VecDeque::new();
        movable[i].push(i);
        qu.push_back(i);
        let mut visited = vec![false; piece_num];
        visited[i] = true;
        while let Some(v) = qu.pop_front() {
            for action in actions.values() {
                let dst = action[v] as usize;
                if !visited[dst] {
                    visited[dst] = true;
                    qu.push_back(dst);
                    movable[i].push(dst);
                }
            }
        }
    }
    movable
}

fn find_edge_pairs(actions: &HashMap<String, Vec<i16>>) -> Vec<usize> {
    let piece_num = actions.values().next().unwrap().len();
    let mut groups = vec![0 as i128; piece_num];
    let mut idx = 0;
    for (k, v) in actions.iter() {
        if k.starts_with("-") {
            continue;
        }
        for (j, val) in v.iter().enumerate() {
            if j as i16 == *val {
                continue;
            }
            groups[j] |= 1 << idx;
        }
        idx += 1;
    }
    let mut val_groups: HashMap<i128, Vec<usize>> = HashMap::new();
    for (i, v) in groups.iter().enumerate() {
        if val_groups.contains_key(v) {
            val_groups.get_mut(v).unwrap().push(i);
        } else {
            val_groups.insert(*v, vec![i; 1]);
        }
    }
    let mut res = vec![piece_num; piece_num];
    for v in val_groups.values() {
        if v.len() == 2 {
            res[v[0]] = v[1];
            res[v[1]] = v[0];
        }
    }
    res
}

fn solve_while_middle(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();

    for i in 1..dim - 1 {
        for j in 1..dim - 1 {
            let target_index = (dim + i) * dim + j;
            if state[target_index] == sol_state[target_index] {
                continue;
            }
            let mut cur_index = 0;
            for k in movable_pos[target_index].iter() {
                if dim * dim <= *k && *k <= target_index {
                    continue;
                }
                if state[*k] == sol_state[target_index] {
                    cur_index = *k;
                    break;
                }
            }
            if cur_index == 0 {
                panic!("source index not found");
            }
            let target_p3 = index_to_p3(target_index, dim);
            while cur_index != target_index {
                let cur_p3 = index_to_p3(cur_index, dim);
                let m = cube_moves::solve_white_middle_impl(&cur_p3, &target_p3, dim);
                for act in m.iter() {
                    let action = &actions[act];
                    state = cube_moves::apply_action(&state, action);
                    cur_index = allowed_moves_inv[act][cur_index] as usize;
                    moves.push(act.clone());
                }
            }
        }
    }
    (state, moves)
}

fn solve_yellow_middle(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();

    for i in 1..dim - 1 {
        for j in (1..dim - 1).rev() {
            let target_index = (3 * dim + i) * dim + j;
            if state[target_index] == sol_state[target_index] {
                continue;
            }
            let mut cur_index = 0;
            for k in movable_pos[target_index].iter() {
                if dim * dim <= *k && *k < 2 * dim * dim {
                    continue;
                }
                if 3 * dim * dim <= *k && *k < 3 * dim * dim + i * dim {
                    continue;
                }
                if 3 * dim * dim + i * dim + j <= *k && *k < 3 * dim * dim + (i + 1) * dim {
                    continue;
                }
                if state[*k] == sol_state[target_index] {
                    cur_index = *k;
                    break;
                }
            }
            if cur_index == 0 {
                panic!("source index not found");
            }
            let target_p3 = index_to_p3(target_index, dim);
            while cur_index != target_index {
                let cur_p3 = index_to_p3(cur_index, dim);
                let m = cube_moves::solve_yellow_middle_impl(&cur_p3, &target_p3, dim);
                for act in m.iter() {
                    let action = &actions[act];
                    state = cube_moves::apply_action(&state, action);
                    cur_index = allowed_moves_inv[act][cur_index] as usize;
                    moves.push(act.clone());
                }
            }
        }
    }
    (state, moves)
}

fn solve_blue_middle(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();

    for i in (1..dim - 1).rev() {
        for j in 1..dim - 1 {
            let target_index = i * dim + j;
            if state[target_index] == sol_state[target_index] {
                continue;
            }
            let mut cur_index = 0;
            for k in movable_pos[target_index].iter() {
                if dim * dim <= *k && *k < 2 * dim * dim {
                    continue;
                }
                if 3 * dim * dim <= *k && *k < 4 * dim * dim {
                    continue;
                }
                if (i + 1) * dim <= *k && *k < dim * dim {
                    continue;
                }
                if i * dim <= *k && *k < i * dim + j {
                    continue;
                }
                if state[*k] == sol_state[target_index] {
                    cur_index = *k;
                    break;
                }
            }
            if cur_index == 0 {
                panic!("source index not found");
            }
            let target_p3 = index_to_p3(target_index, dim);
            while cur_index != target_index {
                let cur_p3 = index_to_p3(cur_index, dim);
                let m = cube_moves::solve_blue_middle_impl(&cur_p3, &target_p3, dim);
                for act in m.iter() {
                    let action = &actions[act];
                    state = cube_moves::apply_action(&state, action);
                    cur_index = allowed_moves_inv[act][cur_index] as usize;
                    moves.push(act.clone());
                }
            }
        }
    }
    (state, moves)
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

fn solve_orange_middle(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let mut checked = vec![false; dim * dim];

    for (i, j) in middle_check_order_orange(dim) {
        let target_index = (4 * dim + i) * dim + j;
        checked[i * dim + j] = true;
        if state[target_index] == sol_state[target_index] {
            continue;
        }
        let mut cur_index = 0;
        for k in movable_pos[target_index].iter() {
            if *k < 2 * dim * dim {
                continue;
            }
            if 3 * dim * dim <= *k && *k < 4 * dim * dim {
                continue;
            }
            if 4 * dim * dim <= *k && *k < 5 * dim * dim && checked[*k - 4 * dim * dim] {
                continue;
            }
            if state[*k] == sol_state[target_index] {
                cur_index = *k;
                break;
            }
        }
        if cur_index == 0 {
            panic!("source index not found");
        }
        let target_p3 = index_to_p3(target_index, dim);
        while cur_index != target_index {
            let cur_p3 = index_to_p3(cur_index, dim);
            let m = cube_moves::solve_orange_middle_impl(&cur_p3, &target_p3, dim);
            for act in m.iter() {
                let action = &actions[act];
                state = cube_moves::apply_action(&state, action);
                cur_index = allowed_moves_inv[act][cur_index] as usize;
                moves.push(act.clone());
            }
        }
    }
    (state, moves)
}

fn calc_permutation_idx_up(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    target_idx: &Vec<usize>,
) -> usize {
    let mut idx_map: HashMap<usize, usize> = HashMap::new();
    for i in 0..target_idx.len() {
        idx_map.insert(cur_state[target_idx[i]], i);
    }

    perm_map[&(0..4)
        .map(|i| idx_map[&sol_state[target_idx[i]]])
        .collect::<Vec<usize>>()]
}

fn calc_permutation_idx_left(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<Vec<usize>, usize>,
    target_idx: &Vec<usize>,
) -> usize {
    let mut idx_map: HashMap<usize, usize> = HashMap::new();
    for i in 0..target_idx.len() {
        idx_map.insert(cur_state[target_idx[i]], i);
    }

    perm_map[&(4..8)
        .map(|i| idx_map[&sol_state[target_idx[i]]])
        .collect::<Vec<usize>>()]
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

fn gen_perm_map_up_corner() -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    let mut perm_list = Vec::new();
    let mut perm_map = HashMap::new();

    for p in (0..16).permutations(4) {
        perm_map.insert(p.clone(), perm_list.len());
        perm_list.push(p);
    }

    (perm_list, perm_map)
}

fn gen_perm_map_left_corner() -> (Vec<Vec<usize>>, HashMap<Vec<usize>, usize>) {
    let mut perm_list = Vec::new();
    let mut perm_map = HashMap::new();

    for p in (0..12).permutations(4) {
        perm_map.insert(p.clone(), perm_list.len());
        perm_list.push(p);
    }

    (perm_list, perm_map)
}

fn bfs_up_impl(
    actions: &Vec<(Vec<usize>, usize, i32)>,
    check_rot: bool,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let (perm_list, perm_map) = gen_perm_map_up_corner();
    let mut priority_qu = BinaryHeap::new();
    // generated by `generate_four_corner_move_for_bfs`
    let rot_num = if check_rot { 4 } else { 1 };
    let mut prev_state: Vec<Option<usize>> = vec![None; rot_num * perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; rot_num * perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; rot_num * perm_list.len()];
    for (start, rot) in &[
        (vec![0, 1, 2, 3], 0),
        (vec![1, 2, 3, 0], 1),
        (vec![2, 3, 0, 1], 2),
        (vec![3, 0, 1, 2], 3),
    ] {
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
    println!("max_cost: {} {:?}", max_cost, perm_list[max_idx / rot_num]);

    (prev_state, prev_action, cost)
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
    let mut priority_qu = BinaryHeap::new();
    // generated by `generate_four_corner_move_for_bfs`
    let rot_num = if check_rot { 4 } else { 1 };
    let mut prev_state: Vec<Option<usize>> = vec![None; rot_num * perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; rot_num * perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; rot_num * perm_list.len()];
    for (start, rot) in &[
        (vec![4, 5, 6, 7], 0),
        (vec![5, 6, 7, 4], 1),
        (vec![6, 7, 4, 5], 2),
        (vec![7, 4, 5, 6], 3),
    ] {
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
    println!("max_cost: {} {:?}", max_cost, perm_list[max_idx / rot_num]);

    (prev_state, prev_action, cost)
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
    for pos in (1..=dim / 2 - 2).rev() {
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
    let (_, perm_map) = gen_perm_map_up_corner();
    let (prev_state, prev_action, cost_table) = bfs_up_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![0, 2, 4, 5]);
            let moves = cube_moves::sequence_moves_four_edge(dim, y, x);
            let perm_idx = calc_permutation_idx_up(&state, sol_state, &perm_map, &target_idx);
            let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 0);

            let state_idx = perm_idx * 4 + rot0;

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
    }

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
    println!("length: {}", cost_table.len());

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
    println!("length: {}", cost_table.len());
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..=dim / 2 - 2).rev() {
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
    let (_, perm_map) = gen_perm_map_left_corner();
    let (prev_state, prev_action, cost_table) = bfs_left_edge();
    println!("length: {}", cost_table.len());

    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx = cube_moves::target_idx_edge_distinct(dim, y, x, &vec![2, 4, 5]);
            let moves = sequence_moves_left_edge(dim, y, x);
            let perm_idx = calc_permutation_idx_left(&state, sol_state, &perm_map, &target_idx);
            let rot0 = cube_moves::calc_face_rot(&state, sol_state, dim, 4);

            let state_idx = perm_idx * 4 + rot0;

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
    }

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
    let (prev_state, prev_action) = bfs0(parity % 2);
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

#[allow(dead_code)]
fn solve_up_face(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_up_first_corner(cur_state, sol_state, actions, dim);
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

#[allow(dead_code)]
fn solve_left_face(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_left_first_corner(cur_state, sol_state, actions, dim);
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

#[allow(dead_code)]
fn solve_green_middle2(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_green_middle_center(cur_state, sol_state, actions, dim);
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

fn middle_check_order_orange(dim: usize) -> Vec<(usize, usize)> {
    let arr = middle_check_order_green(dim);
    let mut res = Vec::new();
    // rotate 90
    for v in arr.iter() {
        let x = 2 * v.0 as i32 - (dim as i32 - 1);
        let y = 2 * v.1 as i32 - (dim as i32 - 1);
        let rx = y;
        let ry = -x;
        res.push((
            (rx + (dim as i32 - 1)) as usize / 2,
            (ry + (dim as i32 - 1)) as usize / 2,
        ));
    }
    res
}

fn middle_check_order_green(dim: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut mark = vec![vec![false; dim]; dim];
    let in_left = (dim - 2) / 2;
    let in_right = (dim + 1) / 2;
    for i in in_left..=in_right {
        for j in in_left..=in_right {
            res.push((i, j));
            mark[i][j] = true;
        }
    }
    for i in 1..in_left {
        res.push((i, i));
        mark[i][i] = true;
        res.push((i, dim - 1 - i));
        mark[i][dim - 1 - i] = true;
        res.push((dim - 1 - i, i));
        mark[dim - 1 - i][i] = true;
        res.push((dim - 1 - i, dim - 1 - i));
        mark[dim - 1 - i][dim - 1 - i] = true;
    }
    for s in 1..in_left {
        for i in in_left - s..=in_right + s {
            for j in in_left - s..=in_right + s {
                if mark[i][j] {
                    continue;
                }
                res.push((i, j));
                mark[i][j] = true;
            }
        }
    }
    res
}

fn solve_front_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    edge_pairs: &Vec<usize>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let mut checked = vec![false; 6 * dim * dim];

    for target_index in front_edge_check_order(dim) {
        let pair_index = edge_pairs[target_index];
        if state[target_index] == sol_state[target_index]
            && state[pair_index] == sol_state[pair_index]
        {
            checked[target_index] = true;
            checked[pair_index] = true;
            continue;
        }
        let mut cur_index = 0;
        for k in movable_pos[target_index].iter() {
            if checked[*k] {
                continue;
            }
            if state[*k] == sol_state[target_index]
                && state[edge_pairs[*k]] == sol_state[pair_index]
            {
                cur_index = *k;
                break;
            }
        }
        if cur_index == 0 {
            panic!("source index not found");
        }
        let target_p3 = index_to_p3(target_index, dim);
        while cur_index != target_index {
            let cur_p3 = index_to_p3(cur_index, dim);
            let mut white_side = cur_index / dim.pow(2);
            if white_side < 2 {
                white_side = 1 - white_side;
            }
            let m =
                cube_moves::solve_front_edge_segments_impl(&cur_p3, &target_p3, dim, white_side);
            if m.is_empty() {
                break;
            }
            for act in m.iter() {
                let action = &actions[act];
                state = cube_moves::apply_action(&state, action);
                cur_index = allowed_moves_inv[act][cur_index] as usize;
                moves.push(act.clone());
            }
        }
        checked[target_index] = true;
        checked[pair_index] = true;
    }
    (state, moves)
}

fn solve_back_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    edge_pairs: &Vec<usize>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let mut checked = vec![false; 6 * dim * dim];

    for target_index in front_edge_check_order(dim) {
        let pair_index = edge_pairs[target_index];
        checked[target_index] = true;
        checked[pair_index] = true;
    }
    for target_index in back_edge_check_order(dim) {
        let pair_index = edge_pairs[target_index];
        if state[target_index] == sol_state[target_index]
            && state[pair_index] == sol_state[pair_index]
        {
            checked[target_index] = true;
            checked[pair_index] = true;
            continue;
        }
        let mut cur_index = 0;
        for k in movable_pos[target_index].iter() {
            if checked[*k] {
                continue;
            }
            if state[*k] == sol_state[target_index]
                && state[edge_pairs[*k]] == sol_state[pair_index]
            {
                cur_index = *k;
                break;
            }
        }
        if cur_index == 0 {
            panic!("source index not found");
        }
        let target_p3 = index_to_p3(target_index, dim);
        while cur_index != target_index {
            let cur_p3 = index_to_p3(cur_index, dim);
            let mut yellow_side = cur_index / dim.pow(2);
            if yellow_side < 2 {
                yellow_side = 1 - yellow_side;
            }
            let m =
                cube_moves::solve_back_edge_segments_impl(&cur_p3, &target_p3, dim, yellow_side);
            if m.is_empty() {
                break;
            }
            for act in m.iter() {
                let action = &actions[act];
                state = cube_moves::apply_action(&state, action);
                cur_index = allowed_moves_inv[act][cur_index] as usize;
                moves.push(act.clone());
            }
        }
        checked[target_index] = true;
        checked[pair_index] = true;
    }
    (state, moves)
}

fn solve_middle_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    allowed_moves_inv: &HashMap<String, &Vec<i16>>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    movable_pos: &Vec<Vec<usize>>,
    edge_pairs: &Vec<usize>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let mut checked = vec![false; 6 * dim * dim];

    for target_index in middle_edge_check_order(dim) {
        let pair_index = edge_pairs[target_index];
        if state[target_index] == sol_state[target_index]
            && state[pair_index] == sol_state[pair_index]
        {
            checked[target_index] = true;
            checked[pair_index] = true;
            continue;
        }
        let mut cur_index = 0;
        for k in movable_pos[target_index].iter() {
            if checked[*k] {
                continue;
            }
            if state[*k] == sol_state[target_index]
                && state[edge_pairs[*k]] == sol_state[pair_index]
            {
                cur_index = *k;
                break;
            }
        }
        if cur_index == 0 {
            panic!("source index not found");
        }
        let target_p3 = index_to_p3(target_index, dim);
        while cur_index != target_index {
            let cur_p3 = index_to_p3(cur_index, dim);
            let m = cube_moves::solve_middle_edge_segments_impl(&cur_p3, &target_p3, dim);
            if m.is_empty() {
                break;
            }
            for act in m.iter() {
                let action = &actions[act];
                state = cube_moves::apply_action(&state, action);
                cur_index = allowed_moves_inv[act][cur_index] as usize;
                moves.push(act.clone());
            }
        }
        checked[target_index] = true;
        checked[pair_index] = true;
    }
    (state, moves)
}

fn front_edge_check_order(dim: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in 1..dim - 1 {
        res.push((0, i));
    }
    for i in 1..dim - 1 {
        res.push((i, dim - 1));
    }
    for i in 1..dim - 1 {
        res.push((dim - 1, i));
    }
    for i in 1..dim - 1 {
        res.push((i, 0));
    }
    res.iter()
        .map(|(i, j)| (dim + i) * dim + j)
        .collect::<Vec<usize>>()
}

fn back_edge_check_order(dim: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in (1..dim - 1).rev() {
        res.push((0, i));
    }
    for i in 1..dim - 1 {
        res.push((i, 0));
    }
    for i in (1..dim - 1).rev() {
        res.push((dim - 1, i));
    }
    for i in 1..dim - 1 {
        res.push((i, dim - 1));
    }
    res.iter()
        .map(|(i, j)| (3 * dim + i) * dim + j)
        .collect::<Vec<usize>>()
}

fn middle_edge_check_order(dim: usize) -> Vec<usize> {
    let mut res = Vec::new();
    for i in (1..dim - 1).rev() {
        res.push(i * dim);
    }
    for i in (1..dim - 1).rev() {
        res.push(i * dim + dim - 1);
    }
    for i in 1..dim - 1 {
        res.push((5 * dim + i) * dim + dim - 1);
    }
    for i in 1..dim - 1 {
        res.push((5 * dim + i) * dim);
    }
    res
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
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) -> Option<String> {
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let actions = cube_moves::create_actions(&allowed_moves);
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
    // println!("{:?}", sol_state);
    let movable_pos = calc_movable(&allowed_moves);
    let mut allowed_moves_inv = HashMap::new();
    for (k, v) in allowed_moves.iter() {
        if k.starts_with("-") {
            allowed_moves_inv.insert(k[1..].to_string(), v);
        } else {
            allowed_moves_inv.insert(format!("-{}", k), v);
        }
    }
    let edge_pairs = find_edge_pairs(allowed_moves);
    let mut state = init_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_while_middle(
        &init_state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!(
        "end white: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    let (end_state, m) = solve_yellow_middle(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!(
        "end yellow: {}",
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
        for x in 0..y - 1 {
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
        for x in 0..y - 1 {
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
        for x in 0..y - 1 {
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
    let (end_state, m) = solve_green_middle2(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!(
        "end green: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    for i in 1..dim / 2 {
        let heuristic_table = heuristic_table_cube_edge();
        println!("try {}", i);
        let (end_state, m) =
            beam_search_cube_edge(&state, &sol_state, &heuristic_table, &actions, dim, i);
        state = end_state;
        moves.extend(m);
    }
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
        for x in 0..y - 1 {
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

    // let (end_state, m) = solve_front_edge(
    //     &state,
    //     &sol_state,
    //     &allowed_moves_inv,
    //     &actions,
    //     &movable_pos,
    //     &edge_pairs,
    //     dim,
    // );
    // state = end_state;
    // moves.extend(m);
    // println!(
    //     "end front edge: {}",
    //     solver::cancel_moves_in_cube(&moves.join("."))
    //         .split(".")
    //         .collect::<Vec<&str>>()
    //         .len()
    // );

    // {
    //     let mut final_rot = vec![0; 6];
    //     for i in 0..6 {
    //         let mut valid_final = true;
    //         final_rot[i] = cube_moves::calc_face_rot(&state, &sol_state, dim, i);
    //         let rot_action = [
    //             format!("-d{}", dim - 1),
    //             "f0".to_string(),
    //             "r0".to_string(),
    //             format!("-f{}", dim - 1),
    //             format!("-r{}", dim - 1),
    //             "d0".to_string(),
    //         ];
    //         if final_rot[i] != 0 {
    //             for _ in 0..4 - final_rot[i] {
    //                 state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
    //             }
    //         }
    //         for j in 1..dim - 1 {
    //             for k in 1..dim - 1 {
    //                 if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k]
    //                 {
    //                     valid_final = false;
    //                 }
    //             }
    //         }

    //         if final_rot[i] != 0 {
    //             for _ in 0..final_rot[i] {
    //                 state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
    //             }
    //         }
    //         println!("{} {}", i, valid_final);
    //     }
    // }

    // let (end_state, m) = solve_back_edge(
    //     &state,
    //     &sol_state,
    //     &allowed_moves_inv,
    //     &actions,
    //     &movable_pos,
    //     &edge_pairs,
    //     dim,
    // );
    // state = end_state;
    // moves.extend(m);
    // println!(
    //     "end back edge: {}",
    //     solver::cancel_moves_in_cube(&moves.join("."))
    //         .split(".")
    //         .collect::<Vec<&str>>()
    //         .len()
    // );
    // {
    //     let mut final_rot = vec![0; 6];
    //     for i in 0..6 {
    //         let mut valid_final = true;
    //         final_rot[i] = cube_moves::calc_face_rot(&state, &sol_state, dim, i);
    //         let rot_action = [
    //             format!("-d{}", dim - 1),
    //             "f0".to_string(),
    //             "r0".to_string(),
    //             format!("-f{}", dim - 1),
    //             format!("-r{}", dim - 1),
    //             "d0".to_string(),
    //         ];
    //         if final_rot[i] != 0 {
    //             for _ in 0..4 - final_rot[i] {
    //                 state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
    //             }
    //         }
    //         for j in 1..dim - 1 {
    //             for k in 1..dim - 1 {
    //                 if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k]
    //                 {
    //                     valid_final = false;
    //                 }
    //             }
    //         }

    //         if final_rot[i] != 0 {
    //             for _ in 0..final_rot[i] {
    //                 state = cube_moves::apply_action(&state, &actions[&rot_action[i]]);
    //             }
    //         }
    //         println!("{} {}", i, valid_final);
    //     }
    // }

    // let (end_state, m) = solve_middle_edge(
    //     &state,
    //     &sol_state,
    //     &allowed_moves_inv,
    //     &actions,
    //     &movable_pos,
    //     &edge_pairs,
    //     dim,
    // );
    // state = end_state;
    // moves.extend(m);
    // println!(
    //     "end middle edge {}",
    //     solver::cancel_moves_in_cube(&moves.join("."))
    //         .split(".")
    //         .collect::<Vec<&str>>()
    //         .len()
    // );

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
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) -> Option<String> {
    let move_map = move_translation(dim);
    let init_state = &puzzle.initial_state;
    let sol_state = &puzzle.solution_state;

    let checker_cube = sol_state.starts_with("A;B;A;B;A;B;A;B;A");
    let distinct_cube = sol_state.starts_with("N0");
    if dim != 33 {
        return None;
    }

    if dim % 2 == 0 && checker_cube {
        return None;
    }
    if !distinct_cube {
        return None;
    }
    if dim <= 3 {
        solve_cube_by_solver(init_state, sol_state, &move_map, allowed_moves, dim)
    } else {
        solve_cube_by_rule(puzzle, &move_map, allowed_moves, dim)
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
        if dim <= 3 {
            continue;
        }
        if dim % 2 == 0 {
            continue;
        }

        let moves = &puzzle_info[&row.puzzle_type];
        if let Some(result) = solve_cube(&row, &moves, dim) {
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

            let (piece_map, piece_list) = solver::gen_piece_map(&row.solution_state);
            let sol_state = solver::state_to_list(&state, &piece_map);
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
    let mut wtr = csv::Writer::from_path("../submission_latest.csv").unwrap();
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

    let mut state = info["r1"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = cube_moves::apply_action(&state, &action_map["-d6"]);
    state = cube_moves::apply_action(&state, &action_map["r3"]);
    state = cube_moves::apply_action(&state, &action_map["d6"]);
    state = cube_moves::apply_action(&state, &action_map["-r1"]);
    state = cube_moves::apply_action(&state, &action_map["-d6"]);
    state = cube_moves::apply_action(&state, &action_map["-r3"]);
    state = cube_moves::apply_action(&state, &action_map["d6"]);
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
fn attempt2() {
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
    state = cube_moves::apply_action(&state, &action_map["-d6"]);
    state = cube_moves::apply_action(&state, &action_map["r2"]);
    state = cube_moves::apply_action(&state, &action_map["r3"]);
    state = cube_moves::apply_action(&state, &action_map["d6"]);
    state = cube_moves::apply_action(&state, &action_map["-r1"]);
    state = cube_moves::apply_action(&state, &action_map["-d6"]);
    state = cube_moves::apply_action(&state, &action_map["-r2"]);
    state = cube_moves::apply_action(&state, &action_map["-r3"]);
    state = cube_moves::apply_action(&state, &action_map["d6"]);
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

    let movable_pos = calc_movable(&info);
    for m in &movable_pos {
        println!("{:?}", m);
    }
}

fn attempt3() {
    let dim = 7;
    let y = 1;
    let x = 2;
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

    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_7/7/7"];
    println!("{:?}", info.keys().sorted());
    let (action, action_str) = create_action(&info);
    println!("{:?}", action_str.iter().sorted());
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }
    println!("{:?}", action_map.keys().sorted());
    for mvs in moves.iter() {
        let mut state = info[mvs[0]]
            .iter()
            .map(|v| *v as usize)
            .collect::<Vec<usize>>();
        for i in 1..mvs.len() {
            println!("{}", mvs[i]);
            state = cube_moves::apply_action(&state, &action_map[mvs[i]]);
        }
        for i in 0..6 {
            for j in 1..6 {
                for k in 1..6 {
                    print!("{:3} ", state[i * 49 + j * 7 + k]);
                }
                println!("");
            }
            println!("");
        }
    }
}

fn target_idx_cube_edge(dim: usize, pos: usize) -> Vec<usize> {
    let rev_pos = dim - 1 - pos;
    let half = dim / 2;
    let mut res = Vec::new();
    for i in 0..6 {
        for x in &[pos, half, rev_pos] {
            res.push(i * dim * dim + x);
        }
        for y in &[pos, half, rev_pos] {
            res.push(i * dim * dim + y * dim);
        }
        for y in &[pos, half, rev_pos] {
            res.push(i * dim * dim + y * dim + dim - 1);
        }
        for x in &[pos, half, rev_pos] {
            res.push(i * dim * dim + (dim - 1) * dim + x);
        }
    }
    res
}

fn sequence_moves_for_cube_edge(dim: usize, pos: usize) -> Vec<Vec<String>> {
    let mut base_sequences = Vec::new();
    for first_rot_pos in &[pos, dim - 1 - pos] {
        base_sequences.extend(Vec::from([
            Vec::from([
                format!("-r{}", first_rot_pos),
                format!("d{}", dim - 1),
                format!("r{}", 0),
                format!("-d{}", dim - 1),
                format!("r{}", first_rot_pos),
            ]),
            Vec::from([
                format!("-r{}", first_rot_pos),
                format!("d{}", dim - 1),
                format!("r{}", 0),
                format!("r{}", 0),
                format!("-d{}", dim - 1),
                format!("r{}", first_rot_pos),
            ]),
            Vec::from([
                format!("-r{}", first_rot_pos),
                format!("d{}", dim - 1),
                format!("-r{}", 0),
                format!("-d{}", dim - 1),
                format!("r{}", first_rot_pos),
            ]),
            Vec::from([
                format!("-r{}", first_rot_pos),
                format!("-d{}", dim - 1),
                format!("r{}", dim - 1),
                format!("d{}", dim - 1),
                format!("r{}", first_rot_pos),
            ]),
            Vec::from([
                format!("-r{}", first_rot_pos),
                format!("-d{}", dim - 1),
                format!("r{}", dim - 1),
                format!("r{}", dim - 1),
                format!("d{}", dim - 1),
                format!("r{}", first_rot_pos),
            ]),
            Vec::from([
                format!("-r{}", first_rot_pos),
                format!("-d{}", dim - 1),
                format!("-r{}", dim - 1),
                format!("d{}", dim - 1),
                format!("r{}", first_rot_pos),
            ]),
        ]));
    }
    let mut one_face = base_sequences.clone();
    for _ in 0..3 {
        base_sequences = base_sequences
            .iter()
            .map(|s| cube_moves::rot_sequence_f(&s, dim))
            .collect();
        one_face.extend(base_sequences.clone());
    }
    one_face.push(Vec::from(["f0".to_string()]));
    let mut two_faces = one_face.clone();
    two_faces.extend(
        one_face
            .iter()
            .map(|s| cube_moves::rot2_sequence_r(&s, dim)),
    );
    let mut res = two_faces.clone();
    res.extend(
        two_faces
            .iter()
            .map(|s| cube_moves::rot_sequence_r(&s, dim)),
    );
    res.extend(
        two_faces
            .iter()
            .map(|s| cube_moves::rot_sequence_d(&s, dim)),
    );
    res
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
fn generate_cube_edge_move_for_bfs(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = cube_moves::create_actions(&allowed_moves);
    let sol_state = (0..6 * dim * dim).collect::<Vec<usize>>();
    for pos in (1..=dim / 2 - 1).rev() {
        let target_idx = target_idx_cube_edge(dim, pos);
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let mut cur_result = Vec::new();
        let sequences = sequence_moves_for_cube_edge(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            for i in 1..sequence.len() {
                state = cube_moves::apply_action(&state, &actions[&sequence[i]]);
            }

            for face in 0..6 {
                for i in 1..dim - 1 {
                    for j in 1..dim - 1 {
                        assert_eq!(state[face * dim * dim + i * dim + j] / (dim * dim), face);
                    }
                }
            }
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
                let final_rot = cube_moves::calc_face_rot(&state, &sol_state, dim, i);
                if final_rot != 0 {
                    for _ in 0..4 - final_rot {
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

                if final_rot != 0 {
                    for _ in 0..final_rot {
                        state = cube_moves::apply_action(&state, &actions[rot_action]);
                    }
                }
            }
            for i in 0..6 {
                let rev_pos = dim - 1 - pos;
                let half = dim / 2;
                for y in [0, dim - 1] {
                    for j in 1..dim - 1 {
                        if j == pos || j == rev_pos {
                            continue;
                        }
                        if state[i * dim * dim + y * dim + half] / dim % dim % (dim - 1) == 0 {
                            assert_eq!(
                                state[i * dim * dim + y * dim + half] / dim,
                                state[i * dim * dim + y * dim + j] / dim
                            );
                        } else {
                            if state[i * dim * dim + y * dim + half] % dim
                                != state[i * dim * dim + y * dim + j] % dim
                            {
                                println!("{:?}", sequence);
                                for a in 0..6 {
                                    for b in 0..dim {
                                        for c in 0..dim {
                                            print!("{:3} ", state[a * dim * dim + b * dim + c]);
                                        }
                                        println!("");
                                    }
                                    println!("");
                                }
                                println!("{} {} {}", i, y, j);
                            }

                            assert_eq!(
                                state[i * dim * dim + y * dim + half] % dim,
                                state[i * dim * dim + y * dim + j] % dim
                            );
                            assert_eq!(
                                state[i * dim * dim + y * dim + half] / dim.pow(2),
                                state[i * dim * dim + y * dim + j] / dim.pow(2)
                            );
                        }
                    }
                }
                for x in [0, dim - 1] {
                    for j in 1..dim - 1 {
                        if j == pos || j == rev_pos {
                            continue;
                        }
                        if state[i * dim * dim + half * dim + x] / dim % dim % (dim - 1) == 0 {
                            assert_eq!(
                                state[i * dim * dim + half * dim + x] / dim,
                                state[i * dim * dim + j * dim + x] / dim
                            );
                        } else {
                            assert_eq!(
                                state[i * dim * dim + half * dim + x] % dim,
                                state[i * dim * dim + j * dim + x] % dim
                            );
                            assert_eq!(
                                state[i * dim * dim + half * dim + x] / dim.pow(2),
                                state[i * dim * dim + j * dim + x] / dim.pow(2)
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
    let res = result.unwrap();
    println!("[");
    for r in &res {
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("generate_cube_edge_move_for_bfs.json", serialized).ok();
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

    println!("[");
    for r in &result {
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
    let serialized = serde_json::to_string(&result).unwrap();
    std::fs::write("generate_cube_face_move_for_bfs.json", serialized).ok();
}

fn heuristic_table_cube_edge() -> Vec<Vec<i32>> {
    let mut res = vec![vec![std::i32::MAX; 72]; 72];
    let mut priority_qu = BinaryHeap::new();
    let contents = std::fs::read_to_string("generate_cube_edge_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    for i in 0..72 {
        if i % 3 != 1 {
            continue;
        }
        res[i][i - 1] = 0;
        res[i][i + 1] = 0;
        priority_qu.push(Reverse((0, (i, i - 1))));
        priority_qu.push(Reverse((0, (i, i + 1))));
    }
    while let Some(Reverse((cur_cost, (center, edge)))) = priority_qu.pop() {
        let c = res[center][edge];
        if cur_cost > c {
            continue;
        }
        for (perm, action_cost) in actions.iter() {
            let mut next_center = 0;
            let mut next_edge = 0;
            for j in 0..72 {
                if perm[j] == center {
                    next_center = j;
                }
                if perm[j] == edge {
                    next_edge = j;
                }
            }
            if action_cost + c < res[next_center][next_edge] {
                res[next_center][next_edge] = action_cost + c;
                priority_qu.push(Reverse((action_cost + c, (next_center, next_edge))));
            }
        }
    }
    let mut max_cost = 0;
    let mut reachable = 0;
    for r in res.iter() {
        for c in r.iter() {
            if *c == std::i32::MAX {
                continue;
            }
            reachable += 1;
            if *c > max_cost {
                max_cost = *c;
            }
        }
    }
    println!("reachable: {}", reachable);
    println!("max_cost: {}", max_cost);
    res
}

fn cube_edge_to_perm(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    dim: usize,
    pos: usize,
) -> Vec<usize> {
    let target_idx = target_idx_cube_edge(dim, pos);
    println!("{:?}", target_idx);
    let mut idx_map = HashMap::new();
    for i in 0..target_idx.len() {
        idx_map.insert(sol_state[target_idx[i]], i);
    }
    target_idx.iter().map(|i| idx_map[&cur_state[*i]]).collect()
}

fn calc_heuristic_cube_edge(cur_perm: &Vec<usize>, heuristic_table: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for i in 0..cur_perm.len() / 3 {
        if heuristic_table[cur_perm[3 * i + 1]][cur_perm[3 * i]] == std::i32::MAX {
            println!("{:?} {}", cur_perm, i);
            println!(
                "{} {} {} {}",
                cur_perm[3 * i + 1],
                cur_perm[3 * i],
                3 * i + 1,
                3 * i
            );
            assert!(false);
        }
        res += heuristic_table[cur_perm[3 * i + 1]][cur_perm[3 * i]];
        res += heuristic_table[cur_perm[3 * i + 1]][cur_perm[3 * i + 2]];
    }
    res / 2
}

fn beam_search_cube_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    heuristic_table: &Vec<Vec<i32>>,
    cube_actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
    pos: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut priority_qu = BinaryHeap::new();
    let contents = std::fs::read_to_string("generate_cube_edge_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let mut prev_action: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut cost: HashMap<Vec<usize>, i32> = HashMap::new();
    let mut upper_cost = std::i32::MAX;
    let init_perm = cube_edge_to_perm(cur_state, sol_state, dim, pos);
    priority_qu.push(Reverse((
        calc_heuristic_cube_edge(&init_perm, heuristic_table),
        0,
        init_perm.clone(),
    )));
    cost.insert(init_perm.clone(), 0);

    let mut cnt = 0;
    let mut end_state = cur_state.clone();
    let mut sol_moves = Vec::new();

    while let Some(Reverse((heuristic_cost, cur_cost, state))) = priority_qu.pop() {
        let c = cost[&state];
        if cur_cost > c {
            continue;
        }
        if cur_cost + (heuristic_cost - cur_cost) / 2 > upper_cost {
            continue;
        }
        cnt += 1;
        let mut is_goal = true;
        for i in 0..state.len() / 3 {
            if state[i * 3].abs_diff(state[i * 3 + 1]) != 1
                || state[i * 3 + 2].abs_diff(state[i * 3 + 1]) != 1
            {
                is_goal = false;
                break;
            }
        }
        if is_goal {
            upper_cost = cur_cost;
            println!("upper_cost: {}", upper_cost);
            let mut cur_perm = state;
            let mut sol_actions = Vec::new();
            while let Some(action_idx) = prev_action.get(&cur_perm) {
                sol_actions.push(*action_idx);
                cur_perm = apply_perm_inv(&cur_perm, &actions[*action_idx].0);
            }
            let action_strings = sequence_moves_for_cube_edge(dim, pos);
            end_state = cur_state.clone();
            sol_moves.clear();
            for action_idx in sol_actions.iter().rev() {
                for act in action_strings[*action_idx].iter() {
                    sol_moves.push(act.clone());
                    end_state = cube_moves::apply_action(&end_state, &cube_actions[act]);
                }
            }
            // break;
            continue;
        }
        for (action_idx, (perm, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_perm(&state, &perm);
            if !cost.contains_key(&next_state) {
                cost.insert(next_state.clone(), std::i32::MAX);
            }
            if action_cost + c < cost[&next_state] {
                cost.insert(next_state.clone(), action_cost + c);
                prev_action.insert(next_state.clone(), action_idx);
                let h = calc_heuristic_cube_edge(&next_state, heuristic_table);
                priority_qu.push(Reverse((action_cost + c + h, action_cost + c, next_state)));
            }
        }
    }
    (end_state, sol_moves)
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
    };
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
            println!("{:?}", init_rot);
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
                println!("{:?}", cur_face_rot);
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

#[allow(dead_code)]
fn attempt4() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_5/5/5"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["-d4"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = cube_moves::apply_action(&state, &action_map["-r0"]);
    state = cube_moves::apply_action(&state, &action_map["-r4"]);
    state = cube_moves::apply_action(&state, &action_map["f0"]);
    state = cube_moves::apply_action(&state, &action_map["f4"]);
    state = cube_moves::apply_action(&state, &action_map["d0"]);
    state = cube_moves::apply_action(&state, &action_map["d4"]);
    state = cube_moves::apply_action(&state, &action_map["-r0"]);
    state = cube_moves::apply_action(&state, &action_map["-d0"]);
    state = cube_moves::apply_action(&state, &action_map["-d4"]);
    state = cube_moves::apply_action(&state, &action_map["-f0"]);
    state = cube_moves::apply_action(&state, &action_map["-f4"]);
    state = cube_moves::apply_action(&state, &action_map["r0"]);
    state = cube_moves::apply_action(&state, &action_map["r4"]);
    // state = cube_moves::apply_action(&state, &action_map["-r3"]);
    // state = cube_moves::apply_action(&state, &action_map["-r0"]);
    for i in 0..6 {
        for j in 0..5 {
            for k in 0..5 {
                print!("{:3} ", state[i * 25 + j * 5 + k]);
            }
            println!("");
        }
        println!("");
    }
    println!("{:?}", state);
}

#[allow(dead_code)]
fn attempt5() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["cube_5/5/5"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["-r4"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = cube_moves::apply_action(&state, &action_map["r0"]);
    state = cube_moves::apply_action(&state, &action_map["d4"]);
    state = cube_moves::apply_action(&state, &action_map["r4"]);
    state = cube_moves::apply_action(&state, &action_map["-r0"]);
    state = cube_moves::apply_action(&state, &action_map["d4"]);
    state = cube_moves::apply_action(&state, &action_map["d4"]);
    state = cube_moves::apply_action(&state, &action_map["-r4"]);
    state = cube_moves::apply_action(&state, &action_map["r0"]);
    state = cube_moves::apply_action(&state, &action_map["d4"]);
    state = cube_moves::apply_action(&state, &action_map["r4"]);
    state = cube_moves::apply_action(&state, &action_map["-r0"]);
    state = cube_moves::apply_action(&state, &action_map["d4"]);
    state = cube_moves::apply_action(&state, &action_map["d4"]);
    // state = cube_moves::apply_action(&state, &action_map["-r3"]);
    // state = cube_moves::apply_action(&state, &action_map["-r0"]);
    for i in 0..6 {
        for j in 0..5 {
            for k in 0..5 {
                print!("{:3} ", state[i * 25 + j * 5 + k]);
            }
            println!("");
        }
        println!("");
    }
    println!("{:?}", state);
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

fn apply_perm(state: &Vec<usize>, perm: &Vec<usize>) -> Vec<usize> {
    perm.iter().map(|i| state[*i]).collect::<Vec<usize>>()
}

fn apply_perm_inv(state: &Vec<usize>, perm: &Vec<usize>) -> Vec<usize> {
    let mut res = vec![0; perm.len()];
    for i in 0..perm.len() {
        res[perm[i]] = state[i];
    }
    res
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
            start = apply_perm(&start, &actions[0].0);
        }
        start = apply_perm(&start, &actions[2].0);
    }
    while let Some(Reverse((cur_cost, state))) = priority_qu.pop() {
        let idx = perm_map[&state];
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_perm(&state, &perm);
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
            start = apply_perm(&start, &actions[0].0);
        }
        start = apply_perm(&start, &actions[2].0);
    }
    while let Some(Reverse((cur_cost, state, rot0, rot1))) = priority_qu.pop() {
        let idx = 16 * perm_map[&state] + 4 * rot0 + rot1;
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, act_rot0, act_rot1, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_perm(&state, &perm);
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
    let mut max_cost = 0;
    let mut max_idx = 0;
    let mut reachable = 0;
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

fn bfs2() -> (Vec<Option<usize>>, Vec<Option<usize>>) {
    let (perm_list, perm_map) = get_perm_map();
    let mut priority_qu = BinaryHeap::new();
    let actions = [
        (Vec::from([1, 3, 0, 2, 4, 5, 6, 7]), 1), // rotate top
        (Vec::from([2, 0, 3, 1, 4, 5, 6, 7]), 1), // rotate top (rev)
        (Vec::from([0, 1, 2, 3, 5, 7, 4, 6]), 1), // rotate btm
        (Vec::from([0, 1, 2, 3, 6, 4, 7, 5]), 1), // ratate btm (rev)
        (Vec::from([0, 1, 6, 3, 4, 5, 7, 2]), 8), // rotate btm_right
        (Vec::from([0, 1, 7, 3, 4, 5, 2, 6]), 8), // rotate btm_right (inv)
        (Vec::from([0, 5, 2, 3, 4, 7, 6, 1]), 8), // rotate btm_left
        (Vec::from([0, 7, 2, 3, 4, 1, 6, 5]), 8), // rotate btm_left (rev)
        (Vec::from([6, 1, 0, 3, 4, 5, 2, 7]), 8), // rotate top_right
        (Vec::from([2, 1, 6, 3, 4, 5, 0, 7]), 8), // rotate top_right
        (Vec::from([5, 0, 2, 3, 4, 1, 6, 7]), 8), // rotate top_left
        (Vec::from([1, 5, 2, 3, 4, 0, 6, 7]), 8), // rotate top_left
    ];
    let mut prev_state: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut prev_action: Vec<Option<usize>> = vec![None; perm_list.len()];
    let mut cost: Vec<i32> = vec![std::i32::MAX; 16 * perm_list.len()];
    for v0 in (0..4).permutations(4) {
        for v1 in (4..8).permutations(4) {
            let mut start = Vec::new();
            start.extend(v0.iter());
            start.extend(v1.iter());
            priority_qu.push(Reverse((0, start.clone())));
            let idx = perm_map[&start];
            cost[idx] = 0;
        }
    }

    while let Some(Reverse((cur_cost, state))) = priority_qu.pop() {
        let idx = perm_map[&state];
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_perm(&state, &perm);
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
    let mut reachable = 0;
    let mut printed = false;
    for (idx, c) in cost.iter().enumerate() {
        if *c == std::i32::MAX {
            if idx % 16 == 0 {
                if !printed {
                    println!("{:?}", perm_list[idx / 16])
                }
                printed = true;
            }
            continue;
        }
        reachable += 1;
        if *c > max_cost {
            max_cost = *c;
            max_idx = idx;
        }
    }
    println!("reachable: {}", reachable);
    println!(
        "max_cost: {} {:?} {} {}",
        max_cost,
        perm_list[max_idx / 16],
        max_idx / 4 % 4,
        max_idx % 4
    );
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
            start = apply_perm(&start, &actions[0].0);
        }
        start = apply_perm(&start, &actions[2].0);
    }
    while let Some(Reverse((cur_cost, state, rot0, rot1))) = priority_qu.pop() {
        let idx = 16 * perm_map[&state] + 4 * rot0 + rot1;
        let c = cost[idx];
        if cur_cost > c {
            continue;
        }
        for (action_idx, (perm, act_rot0, act_rot1, action_cost)) in actions.iter().enumerate() {
            let next_state = apply_perm(&state, &perm);
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
    let mut max_cost = 0;
    let mut max_idx = 0;
    let mut reachable = 0;
    let mut printed = false;
    for (idx, c) in cost.iter().enumerate() {
        if *c == std::i32::MAX {
            if idx % 16 == 0 {
                if !printed {
                    println!("{:?}", perm_list[idx / 16])
                }
                printed = true;
            }
            continue;
        }
        reachable += 1;
        if *c > max_cost {
            max_cost = *c;
            max_idx = idx;
        }
    }
    println!("reachable: {}", reachable);
    println!(
        "max_cost: {} {:?} {} {}",
        max_cost,
        perm_list[max_idx / 16],
        max_idx / 4 % 4,
        max_idx % 4
    );
    (prev_state, prev_action)
}

fn main() {
    // bfs2();
    // attempt3();
    // attempt2();
    // attempt4();
    // attempt5();
    // attempt6();
    // heuristic_table_cube_edge();
    solve();
    // bfs0(0);
    // bfs0(1);
    // bfs1();
    // bfs3();
}

/*
   5
0     2
   1          A
   3
4     6
   7


5 0 2 1 3 4 6 7
0 5 2 1 3 4 6 7 // 1
0 1 5 2 3 4 6 7 // 1 + 2
0 2 2 5 3 4 6 7 // 1 + 2 + 1
*/

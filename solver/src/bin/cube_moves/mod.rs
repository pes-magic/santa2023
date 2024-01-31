// Reference
// * https://github.com/merpig/RubiksProgram
// * https://cube.uubio.com

use itertools::Itertools;
use rand::seq::SliceRandom;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::cube_moves;

static BFS_FOUR_CORNER: Lazy<(Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>)> =
    Lazy::new(|| bfs_four_corner_impl());

static BFS_FOUR_EDGE: Lazy<(Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>)> =
    Lazy::new(|| bfs_four_edge_impl());

static PERM_FOUR_CORNER: Lazy<(Vec<u32>, HashMap<u32, usize>)> =
    Lazy::new(|| gen_perm_map_four_faces_impl());

static BFS_FIRST_TWO_CORNER: Lazy<(Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>)> =
    Lazy::new(|| bfs_first_two_corner_impl());

static BFS_FIRST_TWO_EDGE: Lazy<(Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>)> =
    Lazy::new(|| bfs_first_two_edge_impl());

static PERM_FIRST_TWO_CORNER: Lazy<(Vec<u64>, HashMap<u64, usize>)> =
    Lazy::new(|| gen_perm_map_first_two_faces_impl());

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

fn solve_first_two_faces_impl(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
    try_num: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    let (end_state, m) = solve_first_two_faces_corner(&state, sol_state, actions, dim);
    state = end_state;
    moves.extend(m);
    let (end_state, m) = solve_first_two_faces_edge(&state, sol_state, actions, dim);
    {
        let mut best_state = end_state;
        let mut best_m = m;
        for _ in 0..try_num {
            let (end_state2, m2) = solve_first_two_faces_edge(&state, sol_state, actions, dim);
            if m2.len() < best_m.len() {
                best_state = end_state2;
                best_m = m2;
            }
        }
        state = best_state;
        moves.extend(best_m);
    }
    (state, moves)
}

#[allow(dead_code)]
pub fn solve_first_two_faces(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    if dim % 2 == 0 {
        return solve_first_two_faces_impl(&cur_state, sol_state, actions, dim, 100);
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
    fn evaluate(
        cur_state: &Vec<usize>,
        sol_state: &Vec<usize>,
        actions: &HashMap<String, Vec<(usize, usize)>>,
        dim: usize,
        offset_cost: i32,
    ) -> i32 {
        let (mid_state, m) = solve_first_two_faces_impl(&cur_state, sol_state, actions, dim, 1);
        let post_cost = solve_four_faces_impl(&mid_state, sol_state, actions, dim, 0)
            .1
            .len() as i32;
        m.len() as i32 + post_cost + offset_cost
    }
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
        best_cost = evaluate(cur_state, sol_state, actions, dim, 0);
    }
    let mut state = cur_state.clone();
    for (idx0, act0) in action_list.iter().enumerate() {
        state = apply_action(&state, &actions[act0]);
        if valid_state(&state, sol_state, dim) {
            let cost = evaluate(&state, sol_state, actions, dim, 1);
            if cost < best_cost {
                best_end_state = state.clone();
                best_move.clear();
                best_move.push(act0.clone());
                best_cost = cost;
            }
        }
        for (idx1, act1) in action_list.iter().enumerate() {
            if (idx1 + 3) % 6 == idx0 {
                continue;
            }
            state = apply_action(&state, &actions[act1]);
            if valid_state(&state, sol_state, dim) {
                let cost = evaluate(&state, sol_state, actions, dim, 2);
                if cost < best_cost {
                    best_end_state = state.clone();
                    best_move.clear();
                    best_move.push(act0.clone());
                    best_move.push(act1.clone());
                    best_cost = cost;
                }
            }
            for (idx2, act2) in action_list.iter().enumerate() {
                if (idx2 + 3) % 6 == idx1 {
                    continue;
                }
                state = apply_action(&state, &actions[act2]);
                if valid_state(&state, sol_state, dim) {
                    let cost = evaluate(&state, sol_state, actions, dim, 3);
                    if cost < best_cost {
                        best_end_state = state.clone();
                        best_move.clear();
                        best_move.push(act0.clone());
                        best_move.push(act1.clone());
                        best_move.push(act2.clone());
                        best_cost = cost;
                    }
                }
                state = apply_action(&state, &actions[&rev_move(act2)]);
            }
            state = apply_action(&state, &actions[&rev_move(act1)]);
        }

        state = apply_action(&state, &actions[&rev_move(act0)]);
    }
    let (end_state, m) = solve_first_two_faces_impl(&best_end_state, sol_state, actions, dim, 100);
    best_move.extend(m);
    (end_state, best_move)
}

#[allow(dead_code)]
pub fn solve_four_faces_impl(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
    try_num: usize,
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
    {
        let mut best_state = end_state;
        let mut best_m = m;
        for _ in 0..try_num {
            let (end_state2, m2) = solve_four_faces_edge(&state, sol_state, actions, dim);
            if m2.len() < best_m.len() {
                best_state = end_state2;
                best_m = m2;
            }
        }
        state = best_state;
        moves.extend(best_m);
    }
    (state, moves)
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

pub fn apply_perm<T>(state: &Vec<T>, perm: &Vec<usize>) -> Vec<T>
where
    T: Copy,
{
    perm.iter().map(|i| state[*i]).collect()
}

pub fn apply_perm_inv(state: &Vec<u8>, perm: &Vec<usize>) -> Vec<u8> {
    let mut res = vec![0; perm.len()];
    for i in 0..perm.len() {
        res[perm[i]] = state[i];
    }
    res
}

fn calc_permutation_idx_first_two_impl(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<u64, usize>,
    target_idx: &Vec<usize>,
) -> usize {
    let mut p = Vec::new();
    for i in 0..target_idx.len() {
        if cur_state[target_idx[i]] == sol_state[target_idx[4]] {
            p.push(0);
        } else if cur_state[target_idx[i]] == sol_state[target_idx[12]] {
            p.push(1);
        } else {
            p.push(2);
        }
    }
    perm_map[&pack_perm_for_first_two_faces(&p)]
}

fn calc_permutation_idx_first_two_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<u64, usize>,
    dim: usize,
    pos: usize,
) -> usize {
    let target_idx = target_idx_corner(dim, pos, &(0..6).collect());
    calc_permutation_idx_first_two_impl(cur_state, sol_state, perm_map, &target_idx)
}

fn calc_permutation_idx_first_two_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<u64, usize>,
    dim: usize,
    y: usize,
    x: usize,
) -> usize {
    let target_idx = target_idx_edge(dim, y, x, &(0..6).collect());
    calc_permutation_idx_first_two_impl(cur_state, sol_state, perm_map, &target_idx)
}

fn calc_permutation_idx_four_impl(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<u32, usize>,
    target_idx: &Vec<usize>,
) -> usize {
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

fn calc_permutation_idx_four_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    perm_map: &HashMap<u32, usize>,
    dim: usize,
    pos: usize,
) -> usize {
    let target_idx = target_idx_four_corner(dim, pos);
    calc_permutation_idx_four_impl(cur_state, sol_state, perm_map, &target_idx)
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
    calc_permutation_idx_four_impl(cur_state, sol_state, perm_map, &target_idx)
}

fn solve_first_two_faces_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_first_two_faces();
    let (prev_state, prev_action, cost_table) = bfs_first_two_corner();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    for pos in (1..=dim / 2 - 1).rev() {
        let perm_idx: usize =
            calc_permutation_idx_first_two_corner(&state, sol_state, &perm_map, dim, pos);
        let mut state_idx = perm_idx;
        let moves = sequence_moves_first_two_corner(dim, pos);
        let mut action_idxes = Vec::new();
        while let Some(next_index) = prev_state[state_idx] {
            action_idxes.push(prev_action[state_idx].unwrap());
            state_idx = next_index;
        }
        assert!(cost_table[state_idx] == 0);
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

fn solve_first_two_faces_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (perm_list, perm_map) = gen_perm_map_first_two_faces();
    let (_, prev_action, cost_table) = bfs_first_two_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    let contents = std::fs::read_to_string("generate_first_two_edge_move_for_bfs.json").unwrap();
    let perm_actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let contents = std::fs::read_to_string("generate_first_two_edge_move_diff.json").unwrap();
    let perm_actions_diff: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let mut cnt = 0;
    for y in 1..dim / 2 {
        let all_moves = (y + 1..dim - 1 - y)
            .map(|x| sequence_moves_first_two_edge(dim, y, x))
            .collect::<Vec<_>>();
        if all_moves.is_empty() {
            continue;
        }
        let act_diff = calc_sequence_list_diff(&all_moves[0], all_moves.last().unwrap());
        let mut rng = rand::thread_rng();
        let mut fix_order = (0..dim - 1 - y - (y + 1)).collect::<Vec<usize>>();
        fix_order.shuffle(&mut rng);

        loop {
            let perms = (y + 1..dim - 1 - y)
                .map(|x| {
                    calc_permutation_idx_first_two_edge(&state, sol_state, &perm_map, dim, y, x)
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
                        perm_map
                            [&apply_action_to_packed_perm_inv64(perm_list[*p], &perm_actions[i].0)]
                    })
                    .collect::<Vec<_>>();
                let no_moved_perms = perms
                    .iter()
                    .map(|p| {
                        perm_map[&apply_action_to_packed_perm_inv64(
                            perm_list[*p],
                            &perm_actions_diff[i].0,
                        )]
                    })
                    .collect::<Vec<_>>();
                let mut apply_list = Vec::new();
                let mut cost_sum = perm_actions[i].1;
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
                        let inv_act = rev_move(cur_act);
                        if !actions.contains_key(inv_act.as_str()) {
                            println!("{}", inv_act);
                        }
                        let action = &actions[inv_act.as_str()];
                        state = apply_action(&state, action);
                        moves_str.push(inv_act);
                    }
                } else {
                    let inv_act = rev_move(act);
                    if !actions.contains_key(inv_act.as_str()) {
                        println!("{}", inv_act);
                    }
                    let action = &actions[inv_act.as_str()];
                    state = apply_action(&state, action);
                    moves_str.push(inv_act);
                }
            }
            cnt += 1;
            if cnt == 10000 {
                assert!(false);
            }
        }
    }

    let mut ok = true;
    for i in &[1, 3] {
        for j in 1..dim - 1 {
            for k in 1..dim - 1 {
                if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k] {
                    ok = false
                }
            }
        }
    }
    if !ok {
        for i in 0..6 {
            for j in 1..dim - 1 {
                for k in 1..dim - 1 {
                    print!("{} ", state[i * dim * dim + j * dim + k]);
                }
                println!("");
            }
            println!("");
        }
    }
    assert!(ok);

    (state, moves_str)
}

fn solve_four_faces_corner(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (_, perm_map) = gen_perm_map_four_faces();
    let (prev_state, prev_action, _) = bfs_four_corner();
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

fn calc_sequence_list_diff(seq0: &Vec<Vec<String>>, seq1: &Vec<Vec<String>>) -> Vec<(i32, usize)> {
    let mut res = Vec::new();
    for i in 0..seq0.len() {
        let mut diff = 0;
        let mut mask = 0;
        for j in 0..seq0[i].len() {
            if seq0[i][j] != seq1[i][j] {
                diff += 1;
                mask |= 1 << j;
            }
        }
        res.push((diff, mask));
    }
    res
}

fn solve_four_faces_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let (perm_list, perm_map) = gen_perm_map_four_faces();
    let (_, prev_action, cost_table) = bfs_four_edge();
    let mut state = cur_state.clone();
    let mut moves_str = Vec::new();
    let contents = std::fs::read_to_string("generate_four_edge_move_for_bfs.json").unwrap();
    let perm_actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let contents = std::fs::read_to_string("generate_four_edge_move_diff.json").unwrap();
    let perm_actions_diff: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let mut cnt = 0;
    for y in 1..dim / 2 {
        let all_moves = (y + 1..dim - 1 - y)
            .map(|x| sequence_moves_four_edge(dim, y, x))
            .collect::<Vec<_>>();
        if all_moves.is_empty() {
            continue;
        }
        let act_diff = calc_sequence_list_diff(&all_moves[0], all_moves.last().unwrap());
        let mut rng = rand::thread_rng();
        let mut fix_order = (0..dim - 1 - y - (y + 1)).collect::<Vec<usize>>();
        fix_order.shuffle(&mut rng);

        loop {
            let perms = (y + 1..dim - 1 - y)
                .map(|x| calc_permutation_idx_four_edge(&state, sol_state, &perm_map, dim, y, x))
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
                        perm_map
                            [&apply_action_to_packed_perm_inv(perm_list[*p], &perm_actions[i].0)]
                    })
                    .collect::<Vec<_>>();
                let no_moved_perms = perms
                    .iter()
                    .map(|p| {
                        perm_map[&apply_action_to_packed_perm_inv(
                            perm_list[*p],
                            &perm_actions_diff[i].0,
                        )]
                    })
                    .collect::<Vec<_>>();
                let mut apply_list = Vec::new();
                let mut cost_sum = perm_actions[i].1;
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
                        let inv_act = rev_move(cur_act);
                        if !actions.contains_key(inv_act.as_str()) {
                            println!("{}", inv_act);
                        }
                        let action = &actions[inv_act.as_str()];
                        state = apply_action(&state, action);
                        moves_str.push(inv_act);
                    }
                } else {
                    let inv_act = rev_move(act);
                    if !actions.contains_key(inv_act.as_str()) {
                        println!("{}", inv_act);
                    }
                    let action = &actions[inv_act.as_str()];
                    state = apply_action(&state, action);
                    moves_str.push(inv_act);
                }
            }
            cnt += 1;
            if cnt == 10000 {
                assert!(false);
            }
        }
    }

    let mut ok = true;
    for i in 0..6 {
        for j in 1..dim - 1 {
            for k in 1..dim - 1 {
                if state[i * dim * dim + j * dim + k] != sol_state[i * dim * dim + j * dim + k] {
                    ok = false
                }
            }
        }
    }
    if !ok {
        for i in 0..6 {
            for j in 1..dim - 1 {
                for k in 1..dim - 1 {
                    print!("{} ", state[i * dim * dim + j * dim + k]);
                }
                println!("");
            }
            println!("");
        }
    }
    assert!(ok);

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

fn pack_perm_for_first_two_faces(perm: &Vec<usize>) -> u64 {
    let mut res = 0;
    for i in 0..perm.len() {
        res |= (perm[i] as u64) << (2 * i);
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

#[allow(dead_code)]
fn unpack_perm_for_first_two_faces(packed_perm: u64) -> Vec<usize> {
    let mut res = Vec::new();
    let mut p = packed_perm;
    for _ in 0..24 {
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

fn apply_action_to_packed_perm_inv(packed_perm: u32, action: &Vec<usize>) -> u32 {
    let mut res = 0;
    for i in 0..16 {
        res |= ((packed_perm >> (2 * i)) % 4) << (2 * action[i] as u32);
    }
    res
}

fn apply_action_to_packed_perm64(packed_perm: u64, action: &Vec<usize>) -> u64 {
    let mut res = 0;
    for i in 0..24 {
        res |= ((packed_perm >> (2 * action[i] as u64)) % 4) << (2 * i);
    }
    res
}

fn apply_action_to_packed_perm_inv64(packed_perm: u64, action: &Vec<usize>) -> u64 {
    let mut res = 0;
    for i in 0..24 {
        res |= ((packed_perm >> (2 * i)) % 4) << (2 * action[i] as u64);
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

fn sequence_moves_first_two_corner(dim: usize, pos: usize) -> Vec<Vec<String>> {
    let base = sequence_moves_four_corner(dim, pos);
    let mut res = base.clone();
    res.extend(base.iter().map(|v| rot_sequence_r(v, dim)));
    res.extend(base.iter().map(|v| rot_sequence_d(v, dim)));
    res
}

fn sequence_moves_first_two_edge(dim: usize, y: usize, x: usize) -> Vec<Vec<String>> {
    let base = sequence_moves_four_edge(dim, y, x);
    let mut res = base.clone();
    res.extend(base.iter().map(|v| rot_sequence_r(v, dim)));
    res.extend(base.iter().map(|v| rot_sequence_d(v, dim)));
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
    let res = result.unwrap();
    println!("[");
    for r in &res {
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("generate_four_corner_move_for_bfs.json", serialized).ok();
}

#[allow(dead_code)]
pub fn generate_first_two_corner_move_for_bfs(
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) {
    let mut result = None;
    let actions = create_actions(&allowed_moves);
    for pos in (1..=dim / 2 - 1).rev() {
        let target_idx = target_idx_corner(dim, pos, &(0..6).collect());
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let mut cur_result = Vec::new();
        let sequences = sequence_moves_first_two_corner(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            for i in 1..sequence.len() {
                state = apply_action(&state, &actions[&sequence[i]]);
            }

            for face in 0..6 {
                for i in pos + 1..=dim / 2 {
                    let rev_i = dim - 1 - i;
                    assert!(state[face * dim * dim + i * dim + i] / (dim * dim) == face);
                    assert!(state[face * dim * dim + i * dim + rev_i] / (dim * dim) == face);
                    assert!(state[face * dim * dim + rev_i * dim + i] / (dim * dim) == face);
                    assert!(state[face * dim * dim + rev_i * dim + rev_i] / (dim * dim) == face);
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
    std::fs::write("generate_first_two_corner_move_for_bfs.json", serialized).ok();
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
    let res = result.unwrap();
    println!("[");
    for r in &res {
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("generate_four_edge_move_for_bfs.json", serialized).ok();
}

#[allow(dead_code)]
pub fn generate_four_edge_move_diff(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 2 - y {
            let target_idx = target_idx_four_edge(dim, y, x);
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let target_idx_dif = target_idx_four_edge(dim, y, x + 1);
            let mut idx_map_dif = std::collections::HashMap::new();
            for i in 0..target_idx_dif.len() {
                idx_map_dif.insert(target_idx_dif[i], i);
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
                for idx in target_idx_dif.iter() {
                    cur_move.push(idx_map_dif[&state[*idx]]);
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
    let res = result.unwrap();
    println!("[");
    for r in &res {
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("generate_four_edge_move_diff.json", serialized).ok();
}

#[allow(dead_code)]
pub fn generate_first_two_edge_move_for_bfs(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 1 - y {
            let target_idx = target_idx_edge(dim, y, x, &(0..6).collect());
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = sequence_moves_first_two_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                for i in 1..sequence.len() {
                    state = apply_action(&state, &actions[&sequence[i]]);
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
                cur_result.push((cur_move, sequence.len()));
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
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("generate_first_two_edge_move_for_bfs.json", serialized).ok();
}

#[allow(dead_code)]
pub fn generate_first_two_edge_move_diff(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = create_actions(&allowed_moves);
    for y in 1..dim / 2 {
        for x in y + 1..dim - 2 - y {
            let target_idx = target_idx_edge(dim, y, x, &(0..6).collect());
            let mut idx_map = std::collections::HashMap::new();
            for i in 0..target_idx.len() {
                idx_map.insert(target_idx[i], i);
            }
            let target_idx_dif = target_idx_edge(dim, y, x + 1, &(0..6).collect());
            let mut idx_map_dif = std::collections::HashMap::new();
            for i in 0..target_idx_dif.len() {
                idx_map_dif.insert(target_idx_dif[i], i);
            }
            let mut cur_result = Vec::new();
            let sequences = sequence_moves_first_two_edge(dim, y, x);
            for sequence in sequences.iter() {
                let mut state = allowed_moves[&sequence[0]]
                    .iter()
                    .map(|v| *v as usize)
                    .collect::<Vec<usize>>();
                for i in 1..sequence.len() {
                    state = apply_action(&state, &actions[&sequence[i]]);
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
                cur_result.push((cur_move, sequence.len()));
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
        println!("(Vec::from({:?}), {}),", r.0, r.1)
    }
    println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("generate_first_two_edge_move_diff.json", serialized).ok();
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

fn gen_perm_map_first_two_faces_impl() -> (Vec<u64>, HashMap<u64, usize>) {
    let mut perm_list = Vec::new();
    let mut perm_map = HashMap::new();

    for pos0 in (0..24).combinations(4) {
        let mut remain0 = Vec::new();
        for i in 0..24 {
            if !pos0.contains(&i) {
                remain0.push(i);
            }
        }
        for pos1 in remain0.iter().combinations(4) {
            let mut p = vec![2; 24];
            for pos in pos0.iter() {
                p[*pos] = 0;
            }
            for pos in pos1.iter() {
                p[**pos] = 1;
            }
            let p = pack_perm_for_first_two_faces(&p);
            perm_map.insert(p, perm_list.len());
            perm_list.push(p);
        }
    }

    (perm_list, perm_map)
}

fn gen_perm_map_four_faces() -> (Vec<u32>, HashMap<u32, usize>) {
    PERM_FOUR_CORNER.clone()
}

fn gen_perm_map_first_two_faces() -> (Vec<u64>, HashMap<u64, usize>) {
    PERM_FIRST_TWO_CORNER.clone()
}

fn bfs_four_impl(
    actions: &Vec<(Vec<usize>, i32)>,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    let (perm_list, perm_map) = gen_perm_map_four_faces();
    let mut priority_qu = BinaryHeap::new();
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
    (prev_state, prev_action, cost)
}

fn bfs_four_corner_impl() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    if std::path::Path::new("bfs_four_corner.json").exists() {
        // ファイルが存在する場合、ファイルからデータを読み込む
        let contents = std::fs::read_to_string("bfs_four_corner.json").unwrap();
        let deserialized: (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) =
            serde_json::from_str(&contents).unwrap();
        return deserialized;
    }
    let contents = std::fs::read_to_string("generate_four_corner_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();

    let res = bfs_four_impl(&actions);
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("bfs_four_corner.json", serialized).ok();

    res
}

#[allow(dead_code)]
pub fn bfs_four_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    BFS_FOUR_CORNER.clone()
}

fn bfs_four_edge_impl() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    if std::path::Path::new("bfs_four_edge.json").exists() {
        // ファイルが存在する場合、ファイルからデータを読み込む
        let contents = std::fs::read_to_string("bfs_four_edge.json").unwrap();
        let deserialized: (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) =
            serde_json::from_str(&contents).unwrap();
        return deserialized;
    }
    let contents = std::fs::read_to_string("generate_four_edge_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();

    let res = bfs_four_impl(&actions);
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("bfs_four_edge.json", serialized).ok();

    res
}

#[allow(dead_code)]
pub fn bfs_four_edge() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    BFS_FOUR_EDGE.clone()
}

fn bfs_first_two_impl(
    actions: &Vec<(Vec<usize>, i32)>,
) -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    // set face1 (front) and face3 (back)
    let start = pack_perm_for_first_two_faces(&Vec::from([
        2, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2,
    ]));

    let (perm_list, perm_map) = gen_perm_map_first_two_faces();
    let mut priority_qu = BinaryHeap::new();
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
            let next_state = apply_action_to_packed_perm64(state, &perm);
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

    (prev_state, prev_action, cost)
}

fn bfs_first_two_corner_impl() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    if std::path::Path::new("bfs_first_two_corner.json").exists() {
        // ファイルが存在する場合、ファイルからデータを読み込む
        let contents = std::fs::read_to_string("bfs_first_two_corner.json").unwrap();
        let deserialized: (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) =
            serde_json::from_str(&contents).unwrap();
        return deserialized;
    }
    let contents = std::fs::read_to_string("generate_first_two_corner_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();

    let res = bfs_first_two_impl(&actions);
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("bfs_first_two_corner.json", serialized).ok();

    res
}

#[allow(dead_code)]
pub fn bfs_first_two_corner() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    BFS_FIRST_TWO_CORNER.clone()
}

fn bfs_first_two_edge_impl() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    if std::path::Path::new("bfs_first_two_edge.json").exists() {
        // ファイルが存在する場合、ファイルからデータを読み込む
        let contents = std::fs::read_to_string("bfs_first_two_edge.json").unwrap();
        let deserialized: (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) =
            serde_json::from_str(&contents).unwrap();
        return deserialized;
    }
    let contents = std::fs::read_to_string("generate_first_two_edge_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();

    let res = bfs_first_two_impl(&actions);
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("bfs_first_two_edge.json", serialized).ok();

    res
}

#[allow(dead_code)]
pub fn bfs_first_two_edge() -> (Vec<Option<usize>>, Vec<Option<usize>>, Vec<i32>) {
    BFS_FIRST_TWO_EDGE.clone()
}

// ======= EDGE SOLVER =========

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

fn target_idx_cube_edge_first_even(dim: usize) -> Vec<usize> {
    let pos = dim / 2 - 1;
    let rev_pos = dim - 1 - pos;
    let mut res = Vec::new();
    for i in 0..6 {
        for x in &[pos, rev_pos] {
            res.push(i * dim * dim + x);
        }
        for y in &[pos, rev_pos] {
            res.push(i * dim * dim + y * dim);
        }
        for y in &[pos, rev_pos] {
            res.push(i * dim * dim + y * dim + dim - 1);
        }
        for x in &[pos, rev_pos] {
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
        let r2p = format!("r{}", first_rot_pos);
        let r2m = format!("-r{}", first_rot_pos);
        let r3p = format!("r{}", dim - 1 - first_rot_pos);
        let r3m = format!("-r{}", dim - 1 - first_rot_pos);
        let top = format!("d{}", dim - 1);
        let another_pattern = Vec::from([
            r2p.clone(),
            top.clone(),
            top.clone(),
            r3p.clone(),
            top.clone(),
            top.clone(),
            r3m.clone(),
            top.clone(),
            top.clone(),
            r2m.clone(),
            top.clone(),
            top.clone(),
            r3p.clone(),
            top.clone(),
            top.clone(),
            r3m.clone(),
        ]);
        base_sequences.push(another_pattern.iter().rev().map(|s| rev_move(s)).collect());
        base_sequences.push(another_pattern);
    }
    let mut one_face = base_sequences.clone();
    for _ in 0..3 {
        base_sequences = base_sequences
            .iter()
            .map(|s| rot_sequence_f(&s, dim))
            .collect();
        one_face.extend(base_sequences.clone());
    }
    one_face.push(Vec::from(["f0".to_string()]));
    let mut two_faces = one_face.clone();
    two_faces.extend(one_face.iter().map(|s| rot2_sequence_r(&s, dim)));
    let mut res = two_faces.clone();
    res.extend(two_faces.iter().map(|s| rot_sequence_r(&s, dim)));
    res.extend(two_faces.iter().map(|s| rot_sequence_d(&s, dim)));
    res
}

#[allow(dead_code)]
pub fn generate_cube_edge_move_for_bfs(allowed_moves: &HashMap<String, Vec<i16>>, dim: usize) {
    let mut result = None;
    let actions = create_actions(&allowed_moves);
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
                state = apply_action(&state, &actions[&sequence[i]]);
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
                let final_rot = calc_face_rot(&state, &sol_state, dim, i);
                if final_rot != 0 {
                    for _ in 0..4 - final_rot {
                        state = apply_action(&state, &actions[rot_action]);
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
                        state = apply_action(&state, &actions[rot_action]);
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
    // println!("[");
    // for r in &res {
    //     println!("(Vec::from({:?}), {}),", r.0, r.1)
    // }
    // println!("]");
    let serialized = serde_json::to_string(&res).unwrap();
    std::fs::write("generate_cube_edge_move_for_bfs.json", serialized).ok();
}

#[allow(dead_code)]
pub fn generate_cube_edge_first_even_move_for_bfs(
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) {
    assert!(dim % 2 == 0 && dim >= 6);
    let mut result = Vec::new();
    let actions = create_actions(&allowed_moves);
    let sol_state = (0..6 * dim * dim).collect::<Vec<usize>>();
    let pos = dim / 2 - 1;
    {
        let target_idx = target_idx_cube_edge_first_even(dim);
        let mut idx_map = std::collections::HashMap::new();
        for i in 0..target_idx.len() {
            idx_map.insert(target_idx[i], i);
        }
        let sequences = sequence_moves_for_cube_edge(dim, pos);
        for sequence in sequences.iter() {
            let mut state = allowed_moves[&sequence[0]]
                .iter()
                .map(|v| *v as usize)
                .collect::<Vec<usize>>();
            for i in 1..sequence.len() {
                state = apply_action(&state, &actions[&sequence[i]]);
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
                let final_rot = calc_face_rot(&state, &sol_state, dim, i);
                if final_rot != 0 {
                    for _ in 0..4 - final_rot {
                        state = apply_action(&state, &actions[rot_action]);
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
                        state = apply_action(&state, &actions[rot_action]);
                    }
                }
            }
            for i in 0..6 {
                let rev_pos = dim - 1 - pos;
                for y in [0, dim - 1] {
                    for j in 1..dim - 1 {
                        if j == pos || j == rev_pos {
                            continue;
                        }
                        if state[i * dim * dim + y * dim + 1] / dim % dim % (dim - 1) == 0 {
                            assert_eq!(
                                state[i * dim * dim + y * dim + 1] / dim,
                                state[i * dim * dim + y * dim + j] / dim
                            );
                        } else {
                            assert_eq!(
                                state[i * dim * dim + y * dim + 1] % dim,
                                state[i * dim * dim + y * dim + j] % dim
                            );
                            assert_eq!(
                                state[i * dim * dim + y * dim + 1] / dim.pow(2),
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
                        if state[i * dim * dim + dim + x] / dim % dim % (dim - 1) == 0 {
                            assert_eq!(
                                state[i * dim * dim + dim + x] / dim,
                                state[i * dim * dim + j * dim + x] / dim
                            );
                        } else {
                            assert_eq!(
                                state[i * dim * dim + dim + x] % dim,
                                state[i * dim * dim + j * dim + x] % dim
                            );
                            assert_eq!(
                                state[i * dim * dim + dim + x] / dim.pow(2),
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
            result.push((cur_move, sequence.len()));
        }
    }
    // println!("[");
    // for r in &res {
    //     println!("(Vec::from({:?}), {}),", r.0, r.1)
    // }
    // println!("]");
    let serialized = serde_json::to_string(&result).unwrap();
    std::fs::write(
        "generate_cube_edge_first_even_move_for_bfs.json",
        serialized,
    )
    .ok();
}

fn heuristic_table_cube_edge() -> Vec<Vec<Vec<i32>>> {
    let mut res = vec![vec![vec![std::i32::MAX; 72]; 72]; 72];
    let mut priority_qu = BinaryHeap::new();
    let contents = std::fs::read_to_string("generate_cube_edge_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    for i in 0..24 {
        res[3 * i][3 * i + 1][3 * i + 2] = 0;
        res[3 * i + 2][3 * i + 1][3 * i] = 0;
        priority_qu.push(Reverse((0, (3 * i, 3 * i + 1, 3 * i + 2))));
        priority_qu.push(Reverse((0, (3 * i + 2, 3 * i + 1, 3 * i))));
    }
    while let Some(Reverse((cur_cost, (p0, p1, p2)))) = priority_qu.pop() {
        let c = res[p0][p1][p2];
        if cur_cost > c {
            continue;
        }
        for (perm, action_cost) in actions.iter() {
            let mut next_p0 = 0;
            let mut next_p1 = 0;
            let mut next_p2 = 0;
            for j in 0..72 {
                if perm[j] == p0 {
                    next_p0 = j;
                }
                if perm[j] == p1 {
                    next_p1 = j;
                }
                if perm[j] == p2 {
                    next_p2 = j;
                }
            }
            if action_cost + c < res[next_p0][next_p1][next_p2] {
                res[next_p0][next_p1][next_p2] = action_cost + c;
                priority_qu.push(Reverse((action_cost + c, (next_p0, next_p1, next_p2))));
            }
        }
    }
    let mut sum_cost = 0;
    let mut max_cost = 0;
    let mut reachable = 0;
    for r in res.iter() {
        for c in r.iter() {
            for d in c.iter() {
                if *d == std::i32::MAX {
                    continue;
                }
                reachable += 1;
                if *d > max_cost {
                    max_cost = *d;
                }
                sum_cost += *d;
            }
        }
    }
    println!("reachable: {}", reachable);
    println!("max_cost: {}", max_cost);
    println!("sum_cost: {}", sum_cost);
    res
}

fn heuristic_table_cube_edge_first_even() -> Vec<Vec<i32>> {
    let mut res = vec![vec![std::i32::MAX; 48]; 48];
    let mut priority_qu = BinaryHeap::new();
    let contents =
        std::fs::read_to_string("generate_cube_edge_first_even_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    for i in 0..24 {
        res[2 * i][2 * i + 1] = 0;
        res[2 * i + 1][2 * i] = 0;
        priority_qu.push(Reverse((0, (2 * i, 2 * i + 1))));
        priority_qu.push(Reverse((0, (2 * i + 1, 2 * i))));
    }
    while let Some(Reverse((cur_cost, (p1, p2)))) = priority_qu.pop() {
        let c = res[p1][p2];
        if cur_cost > c {
            continue;
        }
        for (perm, action_cost) in actions.iter() {
            let mut next_p1 = 0;
            let mut next_p2 = 0;
            for j in 0..perm.len() {
                if perm[j] == p1 {
                    next_p1 = j;
                }
                if perm[j] == p2 {
                    next_p2 = j;
                }
            }
            if action_cost + c < res[next_p1][next_p2] {
                res[next_p1][next_p2] = action_cost + c;
                priority_qu.push(Reverse((action_cost + c, (next_p1, next_p2))));
            }
        }
    }
    let mut sum_cost = 0;
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
            sum_cost += *c;
        }
    }
    println!("reachable: {}", reachable);
    println!("max_cost: {}", max_cost);
    println!("sum_cost: {}", sum_cost);
    res
}

fn cube_edge_to_perm(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    target_idx: &Vec<usize>,
    dim: usize,
) -> Vec<u8> {
    let mut idx_map = HashMap::new();
    for i in 0..target_idx.len() {
        idx_map.insert(sol_state[target_idx[i]], i as u8);
    }
    let mut res = Vec::new();
    for t in target_idx.iter() {
        if idx_map.contains_key(&cur_state[*t]) {
            res.push(idx_map[&cur_state[*t]]);
        } else {
            if t % dim == 0 || t % dim == dim - 1 {
                if !idx_map.contains_key(&cur_state[*t - dim]) {
                    println!("size:{}", cur_state.len() / 6);
                    println!("target_idx: {:?}", target_idx);
                    println!("cur_state: {:?}", cur_state);
                    println!("sol_state: {:?}", sol_state);
                }
                res.push(idx_map[&cur_state[*t - dim]]);
            } else {
                res.push(idx_map[&cur_state[*t - 1]]);
            }
        }
    }
    res
}

fn calc_heuristic_cube_edge(cur_perm: &Vec<u8>, heuristic_table: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut res = 0;
    for i in 0..cur_perm.len() / 3 {
        let p0 = cur_perm[3 * i] as usize;
        let p1 = cur_perm[3 * i + 1] as usize;
        let p2 = cur_perm[3 * i + 2] as usize;
        if heuristic_table[p0][p1][p2] == std::i32::MAX {
            println!("{:?} {}", cur_perm, i);
            println!("{} {} {} {} {} {}", p0, p1, p2, 3 * i, 3 * i + 1, 3 * i + 2);
            assert!(false);
        }
        res += heuristic_table[p0][p1][p2];
    }
    res / 2
}

fn calc_heuristic_cube_edge_first_even(cur_perm: &Vec<u8>, heuristic_table: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;
    for i in 0..cur_perm.len() / 2 {
        if heuristic_table[cur_perm[2 * i] as usize][cur_perm[2 * i + 1] as usize] == std::i32::MAX
        {
            println!("{:?} {}", cur_perm, i);
            println!(
                "{} {} {} {}",
                cur_perm[2 * i],
                cur_perm[2 * i + 1],
                2 * i,
                2 * i + 1,
            );
            assert!(false);
        }
        res += heuristic_table[cur_perm[2 * i] as usize][cur_perm[2 * i + 1] as usize];
    }
    res / 2
}

fn aster_cube_edge(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    heuristic_table: &Vec<Vec<Vec<i32>>>,
    cube_actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
    pos: usize,
) -> (Vec<usize>, Vec<String>) {
    let mut priority_qu = BinaryHeap::new();
    let contents = std::fs::read_to_string("generate_cube_edge_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let mut prev_action: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut cost: HashMap<Vec<u8>, i32> = HashMap::new();
    let mut upper_cost = 100; // Experiments show that the cost does not exceed 100
    let init_perm = cube_edge_to_perm(cur_state, sol_state, &target_idx_cube_edge(dim, pos), dim);
    priority_qu.push(Reverse((
        calc_heuristic_cube_edge(&init_perm, heuristic_table),
        0,
        init_perm.clone(),
    )));
    cost.insert(init_perm.clone(), 0);

    let mut cnt = 0;
    let mut end_state = Vec::new();
    let mut sol_moves = Vec::new();
    // let mut max_h = calc_heuristic_cube_edge(&init_perm, heuristic_table);

    while let Some(Reverse((heuristic_cost, cur_cost, state))) = priority_qu.pop() {
        let c = cost[&state];
        if cur_cost > c {
            continue;
        }
        if cur_cost + (heuristic_cost - cur_cost) / 2 >= upper_cost {
            continue;
        }
        // if heuristic_cost - cur_cost < max_h {
        //     max_h = heuristic_cost - cur_cost;
        //     println!("max_h: {}, state: {:?}", max_h, state);
        // }
        cnt += 1;
        if cnt == 10000 && sol_state.is_empty() {
            break;
        }
        if cnt == 100000 {
            break;
        }
        // let mut parity_check = true;
        // let mut parity_cnt = 0;
        // for i in 0..state.len() / 3 {
        //     let a = state[i * 3] / 3;
        //     let b = state[i * 3 + 1] / 3;
        //     let c = state[i * 3 + 2] / 3;
        //     if a != b && b != c {
        //         parity_check = false;
        //         break;
        //     }
        //     if a != b || b != c {
        //         parity_cnt += 1;
        //     }
        // }
        // if parity_check && parity_cnt <= 4 && parity_cnt % 2 == 0 {
        //     break;
        // }
        let mut is_goal = true;
        for i in 0..state.len() / 3 {
            if state[i * 3] / 3 != state[i * 3 + 1] / 3
                || state[i * 3 + 2] / 3 != state[i * 3 + 1] / 3
            {
                is_goal = false;
                break;
            }
        }
        if is_goal {
            upper_cost = cur_cost;
            println!("upper_cost: {} (attempt: {})", upper_cost, cnt);
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
                    end_state = apply_action(&end_state, &cube_actions[act]);
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

fn aster_cube_edge_first_even(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    cube_actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let heuristic_table = heuristic_table_cube_edge_first_even();
    let pos = dim / 2 - 1;
    let mut priority_qu = BinaryHeap::new();
    let contents =
        std::fs::read_to_string("generate_cube_edge_first_even_move_for_bfs.json").unwrap();
    let actions: Vec<(Vec<usize>, i32)> = serde_json::from_str(&contents).unwrap();
    let mut prev_action: HashMap<Vec<u8>, usize> = HashMap::new();
    let mut cost: HashMap<Vec<u8>, i32> = HashMap::new();
    let mut upper_cost = 100; // Experiments show that the cost does not exceed 100
    let init_perm = cube_edge_to_perm(
        cur_state,
        sol_state,
        &target_idx_cube_edge_first_even(dim),
        dim,
    );
    priority_qu.push(Reverse((
        calc_heuristic_cube_edge_first_even(&init_perm, &heuristic_table),
        0,
        init_perm.clone(),
    )));
    cost.insert(init_perm.clone(), 0);

    let mut cnt = 0;
    let mut end_state = Vec::new();
    let mut sol_moves = Vec::new();

    while let Some(Reverse((heuristic_cost, cur_cost, state))) = priority_qu.pop() {
        let c = cost[&state];
        if cur_cost > c {
            continue;
        }
        if heuristic_cost >= upper_cost {
            continue;
        }
        cnt += 1;
        if cnt == 200000 {
            break;
        }
        let mut is_goal = true;
        for i in 0..state.len() / 2 {
            if state[2 * i] / 2 != state[2 * i + 1] / 2 {
                is_goal = false;
                break;
            }
        }
        if is_goal {
            upper_cost = cur_cost;
            println!("upper_cost: {} (attempt: {})", upper_cost, cnt);
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
                    end_state = apply_action(&end_state, &cube_actions[act]);
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
                let h = calc_heuristic_cube_edge_first_even(&next_state, &heuristic_table);
                priority_qu.push(Reverse((action_cost + c + h, action_cost + c, next_state)));
            }
        }
    }
    (end_state, sol_moves)
}

#[allow(dead_code)]
pub fn solve_cube_edges(
    cur_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    fn sequence_to_resolve_parity(invalid_row: &Vec<usize>, dim: usize) -> Vec<String> {
        let mut sequence: Vec<String> = invalid_row.iter().map(|i| format!("r{}", i)).collect();
        sequence.push("f0".to_string());
        sequence.push(format!("f{}", dim - 1));
        for _ in 0..4 {
            sequence.extend(invalid_row.iter().map(|i| format!("-d{}", i)));
            sequence.push("r0".to_string());
            sequence.push("r0".to_string());
        }
        sequence.extend(invalid_row.iter().map(|i| format!("-d{}", i)));
        sequence.push("-f0".to_string());
        sequence.push(format!("-f{}", dim - 1));
        sequence.extend(invalid_row.iter().map(|i| format!("-r{}", i)));
        sequence
    }
    let mut state = cur_state.clone();
    let mut moves = Vec::new();
    if dim % 2 == 0 {
        println!("try first even");
        let (end_state, m) = aster_cube_edge_first_even(&state, &sol_state, &actions, dim);
        if end_state.is_empty() {
            println!("retry first even");
            let sequence: Vec<String> = sequence_to_resolve_parity(&vec![dim / 2 - 1], dim);
            for mv in sequence.iter() {
                state = cube_moves::apply_action(&state, &actions[mv]);
            }
            moves.extend(sequence);
            let (end_state, m) = aster_cube_edge_first_even(&state, &sol_state, &actions, dim);
            assert!(!end_state.is_empty());
            state = end_state;
            moves.extend(m);
        } else {
            state = end_state;
            moves.extend(m);
        }
    }
    let mut invalid_row = Vec::new();
    let heuristic_table = heuristic_table_cube_edge();
    for i in 1..(dim - 1) / 2 {
        println!("try {}", i);
        let (end_state, m) =
            aster_cube_edge(&state, &sol_state, &heuristic_table, &actions, dim, i);
        if end_state.is_empty() {
            invalid_row.push(i);
        } else {
            state = end_state;
            moves.extend(m);
        }
    }
    if !invalid_row.is_empty() {
        // resolve parity error
        let sequence: Vec<String> = sequence_to_resolve_parity(&invalid_row, dim);
        for mv in sequence.iter() {
            state = cube_moves::apply_action(&state, &actions[mv]);
        }
        moves.extend(sequence);

        for i in invalid_row {
            println!("retry {}", i);
            let (end_state, m) =
                aster_cube_edge(&state, &sol_state, &heuristic_table, &actions, dim, i);
            assert!(!end_state.is_empty());
            state = end_state;
            moves.extend(m);
        }
    }
    (state, moves)
}

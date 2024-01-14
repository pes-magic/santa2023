// Reference
// * https://github.com/merpig/RubiksProgram
// * https://www.kaggle.com/code/seanbearden/solve-all-nxnxn-cubes-w-traditional-solution-state
// Dependencies
// * https://github.com/dwalton76/rubiks-cube-NxNxN-solver

use std::collections::HashMap;

mod cube_moves;
use cube_moves::P3;

fn create_actions(
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

fn apply_action(state: &Vec<usize>, action: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_state = state.clone();
    for (i, j) in action.iter() {
        new_state[*i] = state[*j];
    }
    new_state
}

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
                    state = apply_action(&state, action);
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
                    state = apply_action(&state, action);
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
                    state = apply_action(&state, action);
                    cur_index = allowed_moves_inv[act][cur_index] as usize;
                    moves.push(act.clone());
                }
            }
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
                state = apply_action(&state, action);
                cur_index = allowed_moves_inv[act][cur_index] as usize;
                moves.push(act.clone());
            }
        }
    }
    (state, moves)
}

fn solve_green_middle(
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

    for (i, j) in middle_check_order_green(dim) {
        let target_index = (5 * dim + i) * dim + j;
        checked[i * dim + j] = true;
        if state[target_index] == sol_state[target_index] {
            continue;
        }
        let mut cur_index = 0;
        for k in movable_pos[target_index].iter() {
            if *k < 2 * dim * dim {
                continue;
            }
            if 3 * dim * dim <= *k && *k < 5 * dim * dim {
                continue;
            }
            if 5 * dim * dim <= *k && checked[*k - 5 * dim * dim] {
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
            let m = cube_moves::solve_green_middle_impl(&cur_p3, &target_p3, dim);
            if m.is_empty() {
                break;
            }
            for act in m.iter() {
                let action = &actions[act];
                state = apply_action(&state, action);
                cur_index = allowed_moves_inv[act][cur_index] as usize;
                moves.push(act.clone());
            }
        }
    }
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
                state = apply_action(&state, action);
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
                state = apply_action(&state, action);
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
                state = apply_action(&state, action);
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

    if dim >= 6 {
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

struct History {
    action: usize,
    parent: Option<usize>,
}

struct Node {
    state: Vec<usize>,
    id: usize,
    parent: Option<usize>,
    action: Option<usize>,
    cost: i32,
}

struct Candidate {
    parent: usize,
    action: usize,
    cost: i32,
    rand: u32,
}

fn compare_candidate(a: &Candidate, b: &Candidate) -> std::cmp::Ordering {
    if a.cost < b.cost {
        std::cmp::Ordering::Less
    } else if a.cost > b.cost {
        std::cmp::Ordering::Greater
    } else {
        if a.rand < b.rand {
            std::cmp::Ordering::Less
        } else if a.rand > b.rand {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn solve_cube_by_rule(
    puzzle: &solver::Puzzle,
    move_map: &HashMap<String, String>,
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) -> Option<String> {
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let actions = create_actions(&allowed_moves);
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
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
    println!("end white: {}", moves.len());
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
    println!("end yellow: {}", moves.len());
    let (end_state, m) = solve_blue_middle(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!("end blue: {}", moves.len());
    let (end_state, m) = solve_orange_middle(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!("end orange: {}", moves.len());
    let (end_state, m) = solve_green_middle(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!("end green: {}", moves.len());
    // Edges can be solved in a shorter steps by the solver.
    /*
    let (end_state, m) = solve_front_edge(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        &edge_pairs,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!("end front edge: {}", moves.len());
    let (end_state, m) = solve_back_edge(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        &edge_pairs,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!("end back edge: {}", moves.len());
    let (end_state, m) = solve_middle_edge(
        &state,
        &sol_state,
        &allowed_moves_inv,
        &actions,
        &movable_pos,
        &edge_pairs,
        dim,
    );
    state = end_state;
    moves.extend(m);
    println!("end middle edge {}", moves.len());
    */
    // println!("Solved {}", moves.len());
    // for k in 0..6 {
    //     for i in 0..dim {
    //         for j in 0..dim {
    //             print!("{} ", piece_list[state[k * dim * dim + i * dim + j]]);
    //         }
    //         println!("");
    //     }
    //     println!("");
    // }
    let cur_state = solver::list_to_state(&state, &piece_list);
    if let Some(last_move) = solve_cube_by_solver(
        &cur_state,
        &puzzle.solution_state,
        move_map,
        allowed_moves,
        dim,
    ) {
        moves.push(last_move);
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
    // println!("raw_solution {}", sol);
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
    // Already have optimal solution for N=2
    if dim <= 2 {
        return None;
    }
    if dim % 2 == 0 && checker_cube {
        return None;
    }
    if distinct_cube {
        return None;
    }
    if dim <= 10 {
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
            assert_eq!(row.solution_state, state);
            let mmoves_length = result.split(".").collect::<Vec<&str>>().len();
            let best_moves_length = submission_df[&id].split(".").collect::<Vec<&str>>().len();
            println!("find: {}, best: {}", mmoves_length, best_moves_length);
            if mmoves_length < best_moves_length {
                submission_df.insert(id, result);
            }
        } else {
            println!("failed id: {}", id);
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

fn main() {
    solve();
}

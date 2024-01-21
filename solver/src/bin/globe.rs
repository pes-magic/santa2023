// Reference

use std::collections::HashMap;

fn create_action(
    allowed_moves: &HashMap<String, Vec<i16>>,
) -> (Vec<Vec<(usize, usize)>>, Vec<String>) {
    let mut action: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut action_str: Vec<String> = Vec::new();
    for (k, v) in allowed_moves.iter() {
        if k.starts_with("-f") {
            continue;
        }
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

fn apply_action(state: &Vec<usize>, action: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_state = state.clone();
    for (i, j) in action.iter() {
        new_state[*i] = state[*j];
    }
    new_state
}

struct History {
    action: usize,
    parent: Option<usize>,
}

struct VecHistory {
    actions: Vec<usize>,
    parent: Option<usize>,
}

struct Node {
    state: Vec<usize>,
    id: usize,
    cost: i32,
}

struct VecNode {
    state: Vec<usize>,
    id: usize,
    cost: i32,
    step: i32,
}

fn in_range(pos: i32, r: (i32, i32)) -> bool {
    if r.0 <= r.1 {
        if r.0 <= pos && pos <= r.1 {
            return true;
        }
    } else {
        if r.0 <= pos || pos <= r.1 {
            return true;
        }
    }
    false
}

fn dir_in_outside(pos: i32, n: i32, r: (i32, i32)) -> i32 {
    let left = (r.0 - pos + n) % n;
    let right = (pos - r.1 + n) % n;
    if left < right {
        1
    } else {
        -1
    }
}

fn dir_range(pos: i32, n: i32, r: (i32, i32)) -> i32 {
    if in_range(pos, r) {
        0
    } else {
        dir_in_outside(pos, n, r)
    }
}

fn rev_sim(v: &Vec<i32>, target_range: &Vec<(i32, i32)>, upper_cost: i32) -> i32 {
    let mut res = 0;
    let n = v.len() as i32;
    let mut state = v.clone();
    let mut dir = vec![0; v.len()];
    for i in 0..v.len() {
        dir[i] = dir_range(i as i32, n, target_range[state[i] as usize]);
    }
    loop {
        if res > upper_cost {
            break;
        }
        let mut find = false;
        for i in 0..v.len() {
            let next = (i + 1) % v.len();
            if dir[i] == 1 && dir[next] == -1 {
                state.swap(i, next);
                res += 1;
                dir[i] = dir_range(i as i32, n, target_range[state[i] as usize]);
                dir[next] = dir_range(next as i32, n, target_range[state[next] as usize]);
                find = true;
            }
        }
        if find {
            continue;
        }
        for i in 0..v.len() {
            let next = (i + 1) % v.len();
            if dir[i] == 1 && dir[next] == 0 && state[i] != state[next] {
                state.swap(i, next);
                res += 1;
                dir[i] = dir_range(i as i32, n, target_range[state[i] as usize]);
                dir[next] = dir_range(next as i32, n, target_range[state[next] as usize]);
                find = true;
                break;
            }
        }
        if find {
            continue;
        }
        for i in 0..v.len() {
            let next = (i + 1) % v.len();
            if dir[i] == 0 && dir[next] == -1 && state[i] != state[next] {
                state.swap(i, next);
                res += 1;
                dir[i] = dir_range(i as i32, n, target_range[state[i] as usize]);
                dir[next] = dir_range(next as i32, n, target_range[state[next] as usize]);
                find = true;
                break;
            }
        }
        if find {
            continue;
        }
        for i in 0..v.len() {
            let next = (i + 1) % v.len();
            if (dir[i] == 1 || dir[next] == -1) && state[i] != state[next] {
                state.swap(i, next);
                res += 1;
                dir[i] = dir_range(i as i32, n, target_range[state[i] as usize]);
                dir[next] = dir_range(next as i32, n, target_range[state[next] as usize]);
                find = true;
                break;
            }
        }

        if !find {
            break;
        }
    }
    res
}

fn rev_sim_all(v: &Vec<i32>, width: usize) -> i32 {
    let mut cnt = vec![0; width];
    for i in 0..v.len() {
        cnt[v[i] as usize] += 1;
    }
    let mut end = 0;
    let mut range = vec![(0, 0); width];
    for i in 0..width {
        range[i].0 = end;
        range[i].1 = end + cnt[i] - 1;
        end += cnt[i];
    }
    let half1 = (v.len() / 2) as i32;
    let half2 = ((v.len() + 1) / 2) as i32;
    let upper_cost = half1 * (half1 + 1) / 2 + half2 * (half2 + 1) / 2 + 1;
    let mut res = rev_sim(v, &range, upper_cost);
    for _ in 1..v.len() {
        for j in 0..range.len() {
            range[j].0 = (range[j].0 + 1) % v.len() as i32;
            range[j].1 = (range[j].1 + 1) % v.len() as i32;
        }
        res = res.min(rev_sim(v, &range, res));
    }
    res
}

fn calc_cost(state: &Vec<usize>, width: usize, height: usize, piece_type_num: usize) -> i32 {
    let mut cost = 0;
    let half = piece_type_num / 2;

    for h in 0..height {
        let mut start = width;
        for i in 0..width {
            if state[h * width + i] / half != state[h * width + (i + width - 1) % width] / half {
                start = i;
                break;
            }
        }
        if start == width {
            continue;
        }
        let mut prev = state[h * width + start] / half;
        let mut length = 1;
        for i in 1..width {
            let cur = state[h * width + (start + i) % width] / half;
            if cur == prev {
                length += 1;
            } else {
                cost += (width - length).pow(2) as i32;

                prev = cur;
                length = 0;
            }
        }
        cost += (width - length).pow(2) as i32;
    }
    cost
}

fn calc_cost_for_first_move(
    state: &Vec<usize>,
    order_map: &Vec<i32>,
    width: usize,
    height: usize,
    piece_type_num: usize,
) -> i32 {
    let mut cost = 0;
    let half = piece_type_num / 2;

    for h in 0..height {
        for i in 0..width {
            if state[h * width + i] / half != h / (height / 2) {
                cost += 3;
            }
        }
    }
    let mut rev_cost = vec![0; height];
    for h in 0..height / 2 {
        let mut vec_a = Vec::new();
        let mut vec_b = Vec::new();
        for w in 0..width {
            if state[h * width + w] / half == 0 {
                vec_a.push(order_map[state[h * width + w]]);
            } else {
                vec_b.push(width as i32 - 1 - order_map[state[h * width + w]]);
            }
        }
        if !vec_a.is_empty() {
            rev_cost[h] += rev_sim_all(&vec_a, width);
        }
        if !vec_b.is_empty() {
            rev_cost[h] += 2 * rev_sim_all(&vec_b, width);
        }
    }
    for h in height / 2..height {
        let mut vec_a = Vec::new();
        let mut vec_b = Vec::new();
        for w in 0..width {
            if state[h * width + w] / half == 1 {
                vec_a.push(order_map[state[h * width + w]]);
            } else {
                vec_b.push(width as i32 - 1 - order_map[state[h * width + w]]);
            }
        }
        if !vec_a.is_empty() {
            rev_cost[h] += rev_sim_all(&vec_a, width);
        }
        if !vec_b.is_empty() {
            rev_cost[h] += 2 * rev_sim_all(&vec_b, width);
        }
    }
    let mut mx = 0;
    for i in 0..height / 2 {
        let add = rev_cost[i].max(rev_cost[height - 1 - i]);
        cost += 4 * add;
        mx = mx.max(add);
    }
    cost + 8 * mx
}

struct Candidate {
    parent: usize,
    action: usize,
    cost: i32,
    rand: u32,
}

struct VecCandidate {
    parent: usize,
    actions: Vec<usize>,
    cost: i32,
    step: i32,
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

fn compare_vec_candidate(a: &VecCandidate, b: &VecCandidate) -> std::cmp::Ordering {
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

fn pack_state(state: &Vec<usize>, div: usize) -> Vec<u64> {
    let mut res = Vec::new();
    let mut val = 0;
    for i in 0..state.len() {
        val = 2 * val + (state[i] / div) as u64;
        if i % 64 == 63 || i == state.len() - 1 {
            res.push(val);
            val = 0;
        }
    }
    res
}

fn beam_search(
    puzzle: &solver::Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    current_best: &String,
    width: usize,
    height: usize,
    _id: usize,
) -> Option<String> {
    const BEAM_WIDTH: usize = 10000;
    let current_step = current_best.split(".").collect::<Vec<&str>>().len();
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let (action, action_str) = create_action(&allowed_move);
    let mut action_map = std::collections::HashMap::new();
    for (idx, key) in action_str.iter().enumerate() {
        action_map.insert(key.clone(), idx);
    }
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
    let mut order_map = vec![width as i32; piece_list.len()];
    for h in 0..height {
        let mut prev = sol_state[h * width];
        let mut idx = 0;
        for w in 0..width {
            if sol_state[h * width + w] != prev {
                prev = sol_state[h * width + w];
                idx += 1;
            }
            order_map[sol_state[h * width + w]] = idx;
        }
    }

    let mut checked_state = std::collections::HashSet::new();
    let mut history = Vec::new();
    history.push(History {
        action: 0,
        parent: None,
    });
    let mut nodes = Vec::new();
    let mut next_nodes = Vec::new();
    nodes.push(Node {
        state: init_state.clone(),
        id: 0,
        cost: calc_cost(&init_state, width, height, piece_list.len()),
    });
    if nodes[0].cost == 0 {
        return Some("".to_string());
    }
    let mut solution: Option<String> = None;
    let mut current_best = current_step;
    let mut best_state = init_state.clone();
    for step in 1..current_step {
        if step >= current_best {
            break;
        }
        // println!("current_step: {} ({})", step, nodes[0].cost);
        // println!("best state: {:?}", nodes[0].state);
        let mut cands = Vec::new();
        for (idx, node) in nodes.iter().enumerate() {
            for (i, a) in action.iter().enumerate() {
                let new_state = apply_action(&node.state, a);

                let new_cost = calc_cost(&new_state, width, height, piece_list.len());

                if new_cost == 0 {
                    let mut sol = Vec::new();
                    sol.push(i);
                    let mut history_id = node.id;
                    while let Some(n) = history[history_id].parent {
                        sol.push(history[history_id].action);
                        history_id = n;
                    }
                    sol.reverse();

                    let mut last_state = new_state.clone();

                    let mut side = vec![0; height];
                    for h in 0..height {
                        side[h] = new_state[h * width] / (piece_list.len() / 2);
                    }

                    let mut wrong = 0;
                    for h in 0..height / 2 {
                        if side[h] != h / (height / 2) {
                            wrong += 1;
                        }
                    }
                    if wrong > 0 {
                        sol.push(action_map["f0"]);
                        last_state = apply_action(&last_state, &action[action_map["f0"]]);
                        for h in 0..height {
                            if (wrong <= height / 4 && side[h] != h / (height / 2))
                                || (wrong > height / 4 && side[h] == h / (height / 2))
                            {
                                for _ in 0..width / 2 {
                                    let act = format!("r{}", h);
                                    sol.push(action_map[&act]);
                                    last_state =
                                        apply_action(&last_state, &action[action_map[&act]]);
                                }
                            }
                        }
                        if wrong <= height / 4 {
                            sol.push(action_map["f0"]);
                            last_state = apply_action(&last_state, &action[action_map["f0"]]);
                        } else {
                            let act = format!("f{}", width / 2);
                            sol.push(action_map[&act]);
                            last_state = apply_action(&last_state, &action[action_map[&act]]);
                        }
                    }

                    // if sol.len() <= current_best {
                    //     println!("side: {:?}", side);
                    //     for h in 0..height {
                    //         let mut vec = Vec::new();
                    //         for w in 0..width {
                    //             vec.push(order_map[new_state[h * width + w]]);
                    //         }
                    //         print!("{} ", rev_sim_all(&vec, width));
                    //     }
                    //     println!("");
                    //     println!("find step: {}", sol.len());
                    // }
                    if sol.len() < current_best {
                        current_best = sol.len();
                        solution = Some(
                            sol.iter()
                                .map(|v| action_str[*v].clone())
                                .collect::<Vec<String>>()
                                .join("."),
                        );
                        best_state = last_state.clone();
                    }
                }

                cands.push(Candidate {
                    parent: idx,
                    action: i,
                    cost: new_cost,
                    rand: rand::random::<u32>(), // tie breaker
                });
            }
        }
        cands.sort_by(compare_candidate);
        for c in cands {
            if next_nodes.len() >= BEAM_WIDTH {
                break;
            }
            let new_state = apply_action(&nodes[c.parent].state, &action[c.action]);
            let packed_state = pack_state(&new_state, piece_list.len() / 2);
            if checked_state.contains(&packed_state) {
                continue;
            }
            checked_state.insert(packed_state);
            next_nodes.push(Node {
                state: new_state,
                id: history.len(),
                cost: c.cost,
            });
            history.push(History {
                action: c.action,
                parent: Some(nodes[c.parent].id),
            });
        }
        std::mem::swap(&mut nodes, &mut next_nodes);
        next_nodes.clear();
    }
    // println!("{:?}", best_state);
    for h in 0..height {
        let mut vec = Vec::new();
        for w in 0..width {
            vec.push(order_map[best_state[h * width + w]]);
        }
        print!("{} ", rev_sim_all(&vec, width));
    }
    println!("");

    solution
}

fn first_moves_with_manual(
    puzzle: &solver::Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    current_best: &String,
    width: usize,
    height: usize,
    _id: usize,
) -> Option<String> {
    const BEAM_WIDTH: usize = 100;
    let current_step = current_best.split(".").collect::<Vec<&str>>().len();
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let (action, action_str) = create_action(&allowed_move);
    let mut action_map = std::collections::HashMap::new();
    for (idx, key) in action_str.iter().enumerate() {
        action_map.insert(key.clone(), idx);
    }
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
    let mut order_map = vec![width as i32; piece_list.len()];
    for h in 0..height {
        let mut prev = sol_state[h * width];
        let mut idx = 0;
        for w in 0..width {
            if sol_state[h * width + w] != prev {
                prev = sol_state[h * width + w];
                idx += 1;
            }
            order_map[sol_state[h * width + w]] = idx;
        }
    }

    let mut checked_state = std::collections::HashSet::new();
    let mut history = Vec::new();
    history.push(VecHistory {
        actions: Vec::new(),
        parent: None,
    });
    let mut nodes = Vec::new();
    let mut next_nodes = Vec::new();
    nodes.push(VecNode {
        state: init_state.clone(),
        id: 0,
        cost: 0,
        step: 0,
    });
    let mut solution: Option<String> = None;
    let mut current_best = current_step as i32;
    let mut best_state = init_state.clone();
    let half = piece_list.len() / 2;
    for _ in 0..width * height / 2 {
        if nodes.is_empty() {
            break;
        }
        println!("current_step:  ({})", nodes[0].cost);
        // println!("best state:");
        // for h in 0..height {
        //     for w in 0..width {
        //         print!("{} ", nodes[0].state[h * width + w]);
        //     }
        //     println!("");
        // }

        // println!(
        //     "{:?}",
        //     history[nodes[0].id]
        //         .actions
        //         .iter()
        //         .map(|v| action_str[*v].clone())
        //         .collect::<Vec<String>>()
        // );
        let mut cands = Vec::new();
        for (idx, node) in nodes.iter().enumerate() {
            let mut chg_target = height / 2;
            for h in 0..height / 2 {
                for w in 0..width {
                    if node.state[h * width + w] >= half {
                        chg_target = h;
                        break;
                    }
                }
                if chg_target != height / 2 {
                    break;
                }
            }
            if chg_target == height / 2 {
                if node.cost < current_best {
                    current_best = node.cost;
                    let mut sol = Vec::new();
                    let mut history_id = node.id;
                    while let Some(n) = history[history_id].parent {
                        for a in history[history_id].actions.iter().rev() {
                            sol.push(*a);
                        }
                        history_id = n;
                    }
                    sol.reverse();

                    solution = Some(
                        sol.iter()
                            .map(|v| action_str[*v].clone())
                            .collect::<Vec<String>>()
                            .join("."),
                    );
                    best_state = node.state.clone();
                }
                continue;
            }
            let chg_target_rev = height - 1 - chg_target;
            for mv in 0..=width / 2 {
                let mut find = false;
                for dir in 0..2 {
                    if (mv == 0 || mv == width / 2) && dir == 1 {
                        continue;
                    }
                    let mut new_state = node.state.clone();
                    let mut actions = Vec::new();
                    let act_idx = action_map[&if dir == 0 {
                        format!("r{}", chg_target_rev)
                    } else {
                        format!("-r{}", chg_target_rev)
                    }];
                    for _ in 0..mv {
                        actions.push(act_idx);
                        new_state = apply_action(&new_state, &action[act_idx]);
                    }
                    for w in 0..width {
                        if new_state[chg_target * width + w] / half == 1
                            && new_state[chg_target_rev * width + (w + width - 1) % width] / half
                                == 0
                        {
                            let mut chg_num = 1;
                            for i in 2..width / 2 {
                                if new_state[chg_target * width + (w + i - 1) % width] / half == 1
                                    && new_state[chg_target_rev * width + (w + width - i) % width]
                                        / half
                                        == 0
                                {
                                    chg_num = i;
                                } else {
                                    break;
                                }
                            }
                            let mut next_state = new_state.clone();
                            let mut next_actions = actions.clone();
                            {
                                let act_idx = action_map[&format!("f{}", w)];
                                next_actions.push(act_idx);
                                next_state = apply_action(&next_state, &action[act_idx])
                            }
                            for _ in 0..chg_num {
                                let act_idx = action_map[&format!("-r{}", chg_target_rev)];
                                next_actions.push(act_idx);
                                next_state = apply_action(&next_state, &action[act_idx])
                            }
                            {
                                let act_idx = action_map[&format!("f{}", w)];
                                next_actions.push(act_idx);
                                next_state = apply_action(&next_state, &action[act_idx])
                            }
                            find = true;
                            if checked_state.contains(&next_state) {
                                continue;
                            }
                            let new_step = node.step + next_actions.len() as i32;
                            let new_cost = calc_cost_for_first_move(
                                &next_state,
                                &order_map,
                                width,
                                height,
                                piece_list.len(),
                            ) + new_step;
                            cands.push(VecCandidate {
                                parent: idx,
                                actions: next_actions,
                                cost: new_cost,
                                step: new_step,
                                rand: rand::random::<u32>(), // tie breaker
                            });
                        }

                        if new_state[chg_target * width + (w + width - 1) % width] / half == 1
                            && new_state[chg_target_rev * width + w] / half == 0
                        {
                            let mut chg_num = 1;
                            for i in 2..width / 2 {
                                if new_state[chg_target * width + (w + width - i) % width] / half
                                    == 1
                                    && new_state[chg_target_rev * width + (w + i - 1) % width]
                                        / half
                                        == 0
                                {
                                    chg_num = i;
                                } else {
                                    break;
                                }
                            }
                            let mut next_state = new_state.clone();
                            let mut next_actions = actions.clone();
                            {
                                let act_idx = action_map[&format!("f{}", (w + width / 2) % width)];
                                next_actions.push(act_idx);
                                next_state = apply_action(&next_state, &action[act_idx])
                            }
                            for _ in 0..chg_num {
                                let act_idx = action_map[&format!("r{}", chg_target_rev)];
                                next_actions.push(act_idx);
                                next_state = apply_action(&next_state, &action[act_idx])
                            }
                            {
                                let act_idx = action_map[&format!("f{}", (w + width / 2) % width)];
                                next_actions.push(act_idx);
                                next_state = apply_action(&next_state, &action[act_idx])
                            }
                            if checked_state.contains(&next_state) {
                                continue;
                            }
                            find = true;
                            let new_step = node.step + next_actions.len() as i32;
                            let new_cost = calc_cost_for_first_move(
                                &next_state,
                                &order_map,
                                width,
                                height,
                                piece_list.len(),
                            ) + new_step;
                            cands.push(VecCandidate {
                                parent: idx,
                                actions: next_actions,
                                cost: new_cost,
                                step: new_step,
                                rand: rand::random::<u32>(), // tie breaker
                            });
                        }
                    }
                }
            }
        }
        cands.sort_by(compare_vec_candidate);
        for c in cands {
            if next_nodes.len() >= BEAM_WIDTH {
                break;
            }
            let mut new_state = nodes[c.parent].state.clone();
            for a in &c.actions {
                new_state = apply_action(&new_state, &action[*a]);
            }
            if checked_state.contains(&new_state) {
                continue;
            }
            checked_state.insert(new_state.clone());
            next_nodes.push(VecNode {
                state: new_state,
                id: history.len(),
                cost: c.cost,
                step: c.step,
            });
            history.push(VecHistory {
                actions: c.actions,
                parent: Some(nodes[c.parent].id),
            });
        }
        std::mem::swap(&mut nodes, &mut next_nodes);
        next_nodes.clear();
    }
    println!("{:?}", best_state);
    for h in 0..height {
        let mut vec = Vec::new();
        for w in 0..width {
            vec.push(order_map[best_state[h * width + w]]);
        }
        print!("{} ", rev_sim_all(&vec, width));
    }
    println!("");
    solution
}

fn resolve_inversion_greedy(
    puzzle: &solver::Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    first_moves: &String,
    width: usize,
    height: usize,
) -> Option<String> {
    let piece_num = width * height;
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let (action, action_str) = create_action(&allowed_move);
    let mut action_map = std::collections::HashMap::new();
    for (idx, key) in action_str.iter().enumerate() {
        action_map.insert(key.clone(), idx);
    }
    let mut cur_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
    for m in first_moves.split(".") {
        cur_state = apply_action(&cur_state, &action[action_map[m]]);
    }
    let mut order_map = vec![width as i32; piece_list.len()];
    for h in 0..height {
        let mut prev = sol_state[h * width];
        let mut idx = 0;
        for w in 0..width {
            if sol_state[h * width + w] != prev {
                prev = sol_state[h * width + w];
                idx += 1;
            }
            order_map[sol_state[h * width + w]] = idx;
        }
    }
    let mut solution: Vec<usize> = Vec::new();
    loop {
        let mut inv_num = vec![0; height];
        let mut inv_chg = vec![0; piece_num];
        let mut exist_chg = vec![vec![false; 3]; height];
        for h in 0..height {
            let mut vec = Vec::new();
            for w in 0..width {
                vec.push(order_map[cur_state[h * width + w]]);
            }
            inv_num[h] = rev_sim_all(&vec, width);
            for i in 0..width {
                if vec[i] == vec[(i + width - 1) % width] {
                    exist_chg[h][1] = true;
                    continue;
                }
                vec.swap(i, (i + width - 1) % width);
                let r = rev_sim_all(&vec, width);
                inv_chg[h * width + i] = r - inv_num[h];
                exist_chg[h][(inv_chg[h * width + i] + 1) as usize] = true;
                vec.swap(i, (i + width - 1) % width);
            }
        }
        // println!("{:?}", inv_num);
        // println!("{:?}", cur_state);
        if inv_num.iter().all(|v| *v == 0) {
            for h in 0..height {
                for w in 0..width {
                    let mut ok = true;
                    for i in 0..width {
                        if sol_state[h * width + i] != cur_state[h * width + (i + w) % width] {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        if w <= width / 2 {
                            let act_idx = action_map[&format!("r{}", h)];
                            for _ in 0..w {
                                solution.push(act_idx);
                                cur_state = apply_action(&cur_state, &action[act_idx]);
                            }
                        } else {
                            let act_idx = action_map[&format!("-r{}", h)];
                            for _ in 0..width - w {
                                solution.push(act_idx);
                                cur_state = apply_action(&cur_state, &action[act_idx]);
                            }
                        }
                        break;
                    }
                }
            }
            println!("{:?}", cur_state);
            return Some(
                solution
                    .iter()
                    .map(|v| action_str[*v].clone())
                    .collect::<Vec<String>>()
                    .join("."),
            );
        }
        let mut need_move = vec![false; height / 2];
        for h in 0..height / 2 {
            if inv_num[h] > 0 || inv_num[height - 1 - h] > 0 {
                need_move[h] = true;
            }
        }
        let mut target_chg = vec![0; height];
        for h in 0..height / 2 {
            if !need_move[h] {
                continue;
            }
            let inv0 = inv_num[h];
            let inv1 = inv_num[height - 1 - h];
            if inv0 % 2 == inv1 % 2 {
                if inv0 == 0 {
                    target_chg[height - 1 - h] = -1;
                    if exist_chg[h][1] {
                        target_chg[h] = 0;
                    } else {
                        target_chg[h] = 1;
                    }
                } else if inv1 == 0 {
                    target_chg[h] = -1;
                    if exist_chg[height - 1 - h][1] {
                        target_chg[height - 1 - h] = 0;
                    } else {
                        target_chg[height - 1 - h] = 1;
                    }
                } else {
                    target_chg[h] = -1;
                    target_chg[height - 1 - h] = -1;
                }
            } else {
                if inv0 == 0 {
                    target_chg[height - 1 - h] = -1;
                    if exist_chg[h][1] {
                        target_chg[h] = 0;
                    } else {
                        return None;
                    }
                } else if inv1 == 0 {
                    target_chg[h] = -1;
                    if exist_chg[height - 1 - h][1] {
                        target_chg[height - 1 - h] = 0;
                    } else {
                        return None;
                    }
                } else {
                    if inv0 < inv1 {
                        if exist_chg[h][1] {
                            target_chg[h] = 0;
                            target_chg[height - 1 - h] = -1;
                        } else if exist_chg[height - 1 - h][1] {
                            target_chg[h] = -1;
                            target_chg[height - 1 - h] = 0;
                        } else {
                            target_chg[h] = -1;
                            target_chg[height - 1 - h] = -1;
                        }
                    } else {
                        if exist_chg[height - 1 - h][1] {
                            target_chg[h] = -1;
                            target_chg[height - 1 - h] = 0;
                        } else if exist_chg[h][1] {
                            target_chg[h] = 0;
                            target_chg[height - 1 - h] = -1;
                        } else {
                            target_chg[h] = -1;
                            target_chg[height - 1 - h] = -1;
                        }
                    }
                }
            }
        }
        let mut cost = vec![0; width];
        for src in 0..width {
            for h in 0..height {
                if !need_move[h.min(height - 1 - h)] {
                    continue;
                }
                let mut min_cost = width;
                for dst in 0..width {
                    if inv_chg[h * width + dst] != target_chg[h] {
                        continue;
                    }
                    min_cost = min_cost.min((dst + width - src) % width);
                    min_cost = min_cost.min((src + width - dst) % width);
                }
                assert!(min_cost < width);
                cost[src] += min_cost;
            }
        }
        let mut min_cost_idx = 0;
        let mut cur_best_cost = cost[0];
        for w in 1..width {
            if cost[w] < cur_best_cost {
                cur_best_cost = cost[w];
                min_cost_idx = w;
            }
        }
        let mut need_shift = vec![0; height];
        for h in 0..height {
            if !need_move[h.min(height - 1 - h)] {
                continue;
            }
            let mut min_cost = width;
            for dst in 0..width {
                if inv_chg[h * width + dst] != target_chg[h] {
                    continue;
                }
                let c0 = (dst + width - min_cost_idx) % width;
                let c1 = (min_cost_idx + width - dst) % width;
                if c0 < min_cost {
                    min_cost = c0;
                    need_shift[h] = c0 as i32;
                }
                if c1 < min_cost {
                    min_cost = c1;
                    need_shift[h] = -(c1 as i32);
                }
            }
        }
        for h in 0..height {
            if need_shift[h] > 0 {
                let act_idx = action_map[&format!("r{}", h)];
                for _ in 0..need_shift[h] {
                    solution.push(act_idx);
                    cur_state = apply_action(&cur_state, &action[act_idx]);
                }
            } else if need_shift[h] < 0 {
                let act_idx = action_map[&format!("-r{}", h)];
                for _ in 0..(-need_shift[h]) {
                    solution.push(act_idx);
                    cur_state = apply_action(&cur_state, &action[act_idx]);
                }
            }
        }
        {
            let act_idx = action_map[&format!("f{}", min_cost_idx)];
            solution.push(act_idx);
            cur_state = apply_action(&cur_state, &action[act_idx]);
        }
        for h in 0..height / 2 {
            if !need_move[h] {
                continue;
            }
            let act_idx = action_map[&format!("-r{}", height - 1 - h)];
            solution.push(act_idx);
            cur_state = apply_action(&cur_state, &action[act_idx]);
        }
        {
            let act_idx = action_map[&format!("f{}", min_cost_idx)];
            solution.push(act_idx);
            cur_state = apply_action(&cur_state, &action[act_idx]);
        }
        for _ in 0..2 {
            for h in 0..height / 2 {
                if !need_move[h] {
                    continue;
                }
                let act_idx = action_map[&format!("r{}", height - 1 - h)];
                solution.push(act_idx);
                cur_state = apply_action(&cur_state, &action[act_idx]);
            }
        }
        {
            let act_idx = action_map[&format!("f{}", (min_cost_idx + width / 2 - 1) % width)];
            solution.push(act_idx);
            cur_state = apply_action(&cur_state, &action[act_idx]);
        }
        for h in 0..height / 2 {
            if !need_move[h] {
                continue;
            }
            let act_idx = action_map[&format!("-r{}", height - 1 - h)];
            solution.push(act_idx);
            cur_state = apply_action(&cur_state, &action[act_idx]);
        }
        {
            let act_idx = action_map[&format!("f{}", (min_cost_idx + width / 2 - 1) % width)];
            solution.push(act_idx);
            cur_state = apply_action(&cur_state, &action[act_idx]);
        }
    }
}

fn solve_globe_impl_even(
    puzzle: &solver::Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    current_best: &String,
    width: usize,
    height: usize,
    id: usize,
) -> Option<String> {
    if let Some(first_move) =
        first_moves_with_manual(puzzle, allowed_move, current_best, width, height, id)
    {
        if let Some(second_move) =
            resolve_inversion_greedy(puzzle, allowed_move, &first_move, width, height)
        {
            return Some(format!("{}.{}", first_move, second_move));
        }
    }
    None
}

fn solve_globe_impl_odd(
    puzzle: &solver::Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    current_best: &String,
    width: usize,
    height: usize,
    id: usize,
) -> Option<String> {
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let (_, action_str) = create_action(&allowed_move);
    let mut action_map = std::collections::HashMap::new();
    for (idx, key) in action_str.iter().enumerate() {
        action_map.insert(key.clone(), idx);
    }
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);

    let mid = height / 2;
    let mut solution = Vec::new();

    for w in 0..width {
        let mut ok = true;
        for i in 0..width {
            if sol_state[mid * width + i] != init_state[mid * width + (i + w) % width] {
                ok = false;
                break;
            }
        }
        if ok {
            if w <= width / 2 {
                for _ in 0..w {
                    solution.push(format!("r{}", mid));
                }
            } else {
                for _ in 0..width - w {
                    solution.push(format!("-r{}", mid));
                }
            }
            break;
        }
    }

    let mut init_state_even = Vec::new();
    let mut sol_state_even = Vec::new();
    for h in 0..height {
        if h == mid {
            continue;
        }
        for w in 0..width {
            init_state_even.push(init_state[h * width + w]);
            sol_state_even.push(sol_state[h * width + w]);
        }
    }
    let mut action_to_even = HashMap::new();
    let mut action_from_even = HashMap::new();
    for w in 0..width {
        action_to_even.insert(format!("f{}", w), format!("f{}", w));
        action_from_even.insert(format!("f{}", w), format!("f{}", w));
    }
    for h in 0..mid {
        action_to_even.insert(format!("r{}", h), format!("r{}", h));
        action_to_even.insert(format!("-r{}", h), format!("-r{}", h));
        action_from_even.insert(format!("r{}", h), format!("r{}", h));
        action_from_even.insert(format!("-r{}", h), format!("-r{}", h));
    }
    for h in mid + 1..height {
        action_to_even.insert(format!("r{}", h), format!("r{}", h - 1));
        action_to_even.insert(format!("-r{}", h), format!("-r{}", h - 1));
        action_from_even.insert(format!("r{}", h - 1), format!("r{}", h));
        action_from_even.insert(format!("-r{}", h - 1), format!("-r{}", h));
    }
    let puzzle_even = solver::Puzzle {
        puzzle_type: puzzle.puzzle_type.clone(),
        initial_state: solver::list_to_state(&init_state_even, &piece_list),
        solution_state: solver::list_to_state(&sol_state_even, &piece_list),
        num_wildcards: puzzle.num_wildcards,
    };
    let mut allowed_move_even = HashMap::new();
    for (key, value) in allowed_move.iter() {
        if !action_to_even.contains_key(key) {
            continue;
        }
        let mut new_value = Vec::new();
        for h in 0..height {
            if h == mid {
                continue;
            }
            for w in 0..width {
                let mut nv = value[h * width + w];
                if nv as usize > mid * width {
                    nv -= width as i16;
                }
                new_value.push(nv);
            }
        }
        allowed_move_even.insert(action_to_even[key].clone(), new_value);
    }
    if let Some(second_move) = solve_globe_impl_even(
        &puzzle_even,
        &allowed_move_even,
        current_best,
        width,
        height - 1,
        id,
    ) {
        let first_move = solution.join(".");
        let second_move_odd = second_move
            .split(".")
            .map(|v| action_from_even[v].clone())
            .collect::<Vec<String>>()
            .join(".");
        if solution.is_empty() {
            return Some(second_move_odd);
        }
        return Some(format!("{}.{}", first_move, second_move_odd));
    }
    None
}

fn solve_globe_impl(
    puzzle: &solver::Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    current_best: &String,
    width: usize,
    height: usize,
    id: usize,
) -> Option<String> {
    if height % 2 == 0 {
        solve_globe_impl_even(puzzle, allowed_move, current_best, width, height, id)
    } else {
        solve_globe_impl_odd(puzzle, allowed_move, current_best, width, height, id)
    }
}

fn solve_globe() {
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
        if !row.puzzle_type.starts_with("globe") {
            continue;
        }

        let width = row
            .puzzle_type
            .split("/")
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
            * 2;
        let height = (row.puzzle_type.as_bytes()[6] as usize - '0' as usize) + 1;

        let moves = &puzzle_info[&row.puzzle_type];
        if let Some(result) = solve_globe_impl(&row, &moves, &submission_df[&id], width, height, id)
        {
            let mmoves_length = result.split(".").collect::<Vec<&str>>().len();
            let best_moves_length = submission_df[&id].split(".").collect::<Vec<&str>>().len();
            if mmoves_length < best_moves_length {
                println!("solved id: {}", id);
                println!(
                    "{} find: {}, best: {}",
                    row.puzzle_type, mmoves_length, best_moves_length
                );
                println!("solution: {}", result);
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

#[allow(dead_code)]
fn attempt() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["globe_3/4"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["f0"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["f0"]);
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["f3"]);
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["f3"]);
    // state = apply_action(&state, &action_map["-r3"]);
    // state = apply_action(&state, &action_map["-r0"]);
    println!("{:?}", state);
}

#[allow(dead_code)]
fn attemp1() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["globe_3/4"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["f0"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["f0"]);
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["f5"]);
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["f5"]);
    // state = apply_action(&state, &action_map["-r3"]);
    // state = apply_action(&state, &action_map["-r0"]);
    println!("{:?}", state);
}

#[allow(dead_code)]
fn attempt2() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["globe_3/4"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["f1"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["f1"]);
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["f4"]);
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["f4"]);
    // state = apply_action(&state, &action_map["-r3"]);
    // state = apply_action(&state, &action_map["-r0"]);
    println!("{:?}", state);
}

#[allow(dead_code)]
fn attempt3() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["globe_3/4"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["f0"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["-r2"]);
    state = apply_action(&state, &action_map["f0"]);
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["r2"]);
    state = apply_action(&state, &action_map["r3"]);
    state = apply_action(&state, &action_map["r2"]);
    state = apply_action(&state, &action_map["f3"]);
    state = apply_action(&state, &action_map["-r3"]);
    state = apply_action(&state, &action_map["-r2"]);
    state = apply_action(&state, &action_map["f3"]);
    // state = apply_action(&state, &action_map["-r3"]);
    // state = apply_action(&state, &action_map["-r0"]);
    println!("{:?}", state);
}

#[allow(dead_code)]
fn attemp4() {
    let puzzle_info = solver::load_puzzle_info();
    let info = &puzzle_info["globe_1/8"];
    let (action, action_str) = create_action(&info);
    let mut action_map = std::collections::HashMap::new();
    for idx in 0..action_str.len() {
        action_map.insert(action_str[idx].clone(), action[idx].clone());
    }

    let mut state = info["f2"]
        .iter()
        .map(|v| *v as usize)
        .collect::<Vec<usize>>();
    state = apply_action(&state, &action_map["-r1"]);
    state = apply_action(&state, &action_map["f2"]);
    println!("{:?}", state);
}

fn main() {
    // attempt();
    // attemp1();
    // attempt2();
    // attempt3();
    // attemp4();
    solve_globe();
}

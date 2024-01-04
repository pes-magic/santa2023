// Reference

use csv;
use std::collections::HashMap;

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

fn apply_action(state: &Vec<usize>, action: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_state = state.clone();
    for (i, j) in action.iter() {
        new_state[*i] = state[*j];
    }
    new_state
}

fn gen_cost_table(
    sol_state: &Vec<usize>,
    valid_moves: &Vec<Vec<(usize, usize)>>,
    piece_num: usize,
    piece_kind: usize,
) -> Vec<Vec<i32>> {
    let mut cost_table = vec![vec![100000000; piece_num]; piece_kind];
    let mut g = vec![Vec::new(); piece_num];
    for v in valid_moves.iter() {
        for (i, j) in v.iter() {
            g[*i].push(*j);
            g[*j].push(*i);
        }
    }
    for i in 0..piece_kind {
        let mut qu = std::collections::VecDeque::new();
        for (idx, v) in sol_state.iter().enumerate() {
            if *v == i {
                cost_table[i][idx] = 0;
                qu.push_back(idx);
            }
        }
        while let Some(v) = qu.pop_front() {
            for nv in g[v].iter() {
                if cost_table[i][v] + 1 < cost_table[i][*nv] {
                    cost_table[i][*nv] = cost_table[i][v] + 1;
                    qu.push_back(*nv);
                }
            }
        }
    }
    cost_table
}

struct History {
    action: usize,
    parent: Option<usize>,
}

struct Node {
    state: Vec<usize>,
    id: usize,
    cost: i32,
}

fn calc_cost(
    state: &Vec<usize>,
    sol_state: &Vec<usize>,
    cost_table: &Vec<Vec<i32>>,
    num_wildcards: i16,
) -> i32 {
    let mut dif = 0;

    let mut cost = 0;
    for (i, v) in state.iter().enumerate() {
        cost += cost_table[*v][i];
        if sol_state[i] != *v {
            dif += 1;
        }
    }
    if dif <= num_wildcards {
        0
    } else {
        cost
    }
}

fn calc_cost_dif(
    base_state: &Vec<usize>,
    sol_state: &Vec<usize>,
    cost_table: &Vec<Vec<i32>>,
    action: &Vec<(usize, usize)>,
    current_cost: i32,
    num_wildcards: i16,
) -> i32 {
    let mut cost = current_cost;
    let mut dif = 0;
    for (i, v) in base_state.iter().enumerate() {
        if sol_state[i] != *v {
            dif += 1;
        }
    }
    for (p0, p1) in action.iter() {
        cost -= cost_table[base_state[*p0]][*p0];
        cost += cost_table[base_state[*p1]][*p0];
        if base_state[*p0] != sol_state[*p0] {
            dif -= 1;
        }
        if base_state[*p1] != sol_state[*p0] {
            dif += 1;
        }
    }
    if dif <= num_wildcards {
        0
    } else {
        cost
    }
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

fn pack_state(state: &Vec<usize>) -> (u128, u128, u64) {
    let mut s0 = 0;
    let mut s1 = 0;
    let mut s2 = 0;
    for (idx, v) in state.iter().enumerate() {
        if idx < 80 {
            s0 = 3 * s0 + (*v as u128);
        } else if idx < 160 {
            s1 = 3 * s1 + (*v as u128);
        } else {
            s2 = 3 * s2 + (*v as u64);
        }
    }
    (s0, s1, s2)
}

fn beam_search(
    puzzle: &solver::Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    current_best: &String,
) -> Option<String> {
    let mut cross_point = Vec::new();
    if allowed_move.contains_key("r") && allowed_move.contains_key("l") {
        for (idx, i) in allowed_move["r"].iter().enumerate() {
            if idx != *i as usize && idx != allowed_move["l"][idx] as usize {
                cross_point.push(idx);
            }
        }
    }
    const BEAM_WIDTH: usize = 1000000;
    let current_step = current_best.split(".").collect::<Vec<&str>>().len();
    let piece_num = puzzle.initial_state.split(";").collect::<Vec<&str>>().len();
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.initial_state);
    let (action, action_str) = create_action(&allowed_move);
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
    let cost_table = gen_cost_table(&sol_state, &action, piece_num, piece_list.len());
    let mut checked_state = std::collections::HashSet::new();
    checked_state.insert(pack_state(&init_state));
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
        cost: calc_cost(&init_state, &sol_state, &cost_table, puzzle.num_wildcards),
    });
    if nodes[0].cost == 0 {
        return Some("".to_string());
    }
    for step in 1..current_step {
        println!("current_step: {} ({})", step, nodes[0].cost);
        let mut cands = Vec::new();
        for (idx, node) in nodes.iter().enumerate() {
            for (i, a) in action.iter().enumerate() {
                let new_cost = calc_cost_dif(
                    &node.state,
                    &sol_state,
                    &cost_table,
                    a,
                    node.cost,
                    puzzle.num_wildcards,
                );

                if new_cost == 0 {
                    let mut sol = Vec::new();
                    sol.push(i);
                    let mut history_id = node.id;
                    while let Some(n) = history[history_id].parent {
                        sol.push(history[history_id].action);
                        history_id = n;
                    }
                    sol.reverse();
                    return Some(
                        sol.iter()
                            .map(|v| action_str[*v].clone())
                            .collect::<Vec<String>>()
                            .join("."),
                    );
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
            let packed_state = pack_state(&new_state);
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
    None
}

fn solve_wreath() {
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
        if !row.puzzle_type.starts_with("wreath") {
            continue;
        }
        if row.puzzle_type.ends_with("100") {
            continue;
        }
        let moves = &puzzle_info[&row.puzzle_type];
        if let Some(result) = beam_search(&row, &moves, &submission_df[&id]) {
            println!("solved id: {}", id);
            println!("solution: {}", result);
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
    solve_wreath();
}

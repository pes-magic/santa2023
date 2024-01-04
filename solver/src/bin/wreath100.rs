// Reference

use csv;
use serde_json;
use std::collections::HashMap;

fn gen_piece_map(state: &str) -> (HashMap<&str, i8>, Vec<&str>) {
    let mut piece_map: HashMap<&str, i8> = HashMap::new();
    let mut piece_list: Vec<&str> = Vec::new();
    for f in state.split(";") {
        if !piece_map.contains_key(f) {
            piece_map.insert(f, piece_list.len() as i8);
            piece_list.push(f);
        }
    }
    (piece_map, piece_list)
}

fn state_to_list(state: &str, piece_map: &HashMap<&str, i8>) -> Vec<i8> {
    state.split(";").map(|f| piece_map[f]).collect::<Vec<i8>>()
}

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

fn action_id_to_str(action: u8) -> String {
    if action <= 50 {
        vec!["l"; action as usize].join(".")
    } else if action < 100 {
        vec!["-l"; 100 - action as usize].join(".")
    } else if action <= 150 {
        vec!["r"; action as usize - 100].join(".")
    } else {
        vec!["-r"; 200 - action as usize].join(".")
    }
}

fn apply_action(state: &Vec<usize>, action: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_state = state.clone();
    for (i, j) in action.iter() {
        new_state[*i] = state[*j];
    }
    new_state
}

// Load csv file from the given path.
fn load_csv(path: &str) -> Vec<Vec<String>> {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut records: Vec<Vec<String>> = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let mut record_vec: Vec<String> = Vec::new();
        for field in record.iter() {
            record_vec.push(field.to_string());
        }
        records.push(record_vec);
    }
    records
}

fn load_submission() -> Vec<Vec<String>> {
    load_csv("../submission.csv")
}

fn load_puzzle_info() -> HashMap<String, HashMap<String, Vec<i16>>> {
    let mut puzzle_info: HashMap<String, HashMap<String, Vec<i16>>> = HashMap::new();
    let records = load_csv("../raw_data/puzzle_info.csv");
    for record in records.iter() {
        let key = &record[0];
        // parse the second element of the record as json
        // Replace single quote in record[1] with double quote
        let record1 = record[1].replace("'", "\"");
        let mut json: HashMap<String, Vec<i16>> = serde_json::from_str(&record1).unwrap();
        let mut json_inv = HashMap::new();
        for (k, v) in json.iter() {
            // create inverse permutation of v
            let mut v_inv = vec![0; v.len()];
            for (i, j) in v.iter().enumerate() {
                v_inv[*j as usize] = i as i16;
            }
            json_inv.insert(format!("-{}", k), v_inv);
        }
        // merge json and json_inv
        for (k, v) in json_inv.iter() {
            json.insert(k.to_string(), v.clone());
        }
        puzzle_info.insert(key.to_string(), json);
    }
    puzzle_info
}

struct Puzzle {
    puzzle_type: String,
    solution_state: String,
    initial_state: String,
    num_wildcards: i16,
}

fn load_puzzle() -> Vec<Puzzle> {
    let mut puzzles: Vec<Puzzle> = Vec::new();
    let records = load_csv("../raw_data/puzzles.csv");
    for record in records.iter() {
        let puzzle_type = &record[1];
        let solution_state = &record[2];
        let initial_state = &record[3];
        let num_wildcards = &record[4].parse::<i16>().unwrap();
        puzzles.push(Puzzle {
            puzzle_type: puzzle_type.to_string(),
            solution_state: solution_state.to_string(),
            initial_state: initial_state.to_string(),
            num_wildcards: *num_wildcards,
        });
    }
    puzzles
}

struct History {
    action: u8,
    cost: i16,
    node_id: usize,
    parent: Option<usize>,
}

struct Node {
    state: Vec<i8>,
    history_id: usize,
}

fn calc_cost(
    state: &Vec<i8>,
    sol_state: &Vec<i8>,
    wreath_l: &Vec<usize>,
    wreath_r: &Vec<usize>,
    idx_a: i8,
    idx_b: i8,
    num_wildcards: i16,
) -> i32 {
    let mut dif = 0;

    for (i, v) in state.iter().enumerate() {
        if sol_state[i] != *v {
            dif += 1;
        }
    }
    if dif <= num_wildcards {
        return 0;
    }
    let mut cost = 0;
    let mut checked = vec![false; state.len()];
    for i in 0..wreath_l.len() {
        if checked[wreath_l[i]] || checked[wreath_l[(i + 25) % 100]] {
            continue;
        }
        if state[wreath_l[i]] == idx_b && state[wreath_l[(i + 25) % 100]] == idx_b {
            cost += i.min(100 - i) as i32;
            checked[wreath_l[i]] = true;
            checked[wreath_l[(i + 25) % 100]] = true;
        }
    }
    for i in 0..wreath_r.len() {
        if checked[wreath_r[i]] || checked[wreath_r[(i + 100 - 26) % 100]] {
            continue;
        }
        if state[wreath_r[i]] == idx_a && state[wreath_r[(i + 100 - 26) % 100]] == idx_a {
            cost += i.min(100 - i) as i32;
            checked[wreath_r[i]] = true;
            checked[wreath_r[(i + 26) % 100]] = true;
        }
    }
    for i in 0..wreath_l.len() {
        if checked[wreath_l[i]] {
            continue;
        }
        if state[wreath_l[i]] != idx_b {
            continue;
        }
        let base = i.min(100 - i) as i32;
        let base2 = (i as i32 - 75).abs();
        cost += (base.min(base2)).max(25) * 2;
    }
    for i in 0..wreath_r.len() {
        if checked[wreath_r[i]] {
            continue;
        }
        if state[wreath_r[i]] != idx_a {
            continue;
        }
        let base = i.min(100 - i) as i32;
        let base2 = (i as i32 - 74).abs();
        cost += (base.min(base2)).max(26) * 2;
    }
    cost
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Candidate {
    parent_node: usize,
    action: u8,
    cost: i16,
    heuristic: i32,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // other
        //     .cost
        //     .cmp(&self.cost)
        //     .then_with(|| other.heuristic.cmp(&self.heuristic))
        other
            .heuristic
            .cmp(&self.heuristic)
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn pack_state(state: &Vec<i8>) -> (u128, u128, u64) {
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

fn wreath_cycle(action: &Vec<i16>) -> Vec<usize> {
    let mut idx = 0;
    let mut idx_list = Vec::new();
    loop {
        idx_list.push(idx);
        idx = action[idx] as usize;
        if idx == 0 {
            break;
        }
    }
    idx_list
}

fn move_wreath(
    state: &Vec<i8>,
    wreath_l: &Vec<usize>,
    wreath_r: &Vec<usize>,
    action: u8,
) -> Vec<i8> {
    let mut new_state = state.clone();
    if action % 100 != 0 {
        if action < 100 {
            for i in 0..wreath_l.len() {
                new_state[wreath_l[i]] = state[wreath_l[(i + action as usize) % 100]];
            }
        } else {
            for i in 0..wreath_r.len() {
                new_state[wreath_r[i]] = state[wreath_r[(i + action as usize) % 100]];
            }
        }
    }
    new_state
}

fn is_candidate_left(
    state: &Vec<i8>,
    wreath_l: &Vec<usize>,
    wreath_r: &Vec<usize>,
    shift: usize,
    idx_a: i8,
    idx_b: i8,
) -> bool {
    if state[wreath_l[shift]] == idx_b && state[wreath_l[(shift + 25) % 100]] == idx_b {
        return true;
    }
    if state[wreath_r[26]] == idx_a {
        if state[wreath_l[shift]] == idx_a && state[wreath_l[(shift + 25) % 100]] != idx_a {
            return true;
        }
    }
    if state[wreath_r[100 - 26 * 2]] == idx_a {
        if state[wreath_l[(shift + 25) % 100]] == idx_a && state[wreath_l[shift]] != idx_a {
            return true;
        }
    }
    if state[wreath_l[(shift + 50) % 100]] == idx_b
        && (state[wreath_l[(shift + 25) % 100]] != idx_b
            || state[wreath_l[(shift + 75) % 100]] != idx_b)
    {
        return true;
    }
    if state[wreath_l[(shift + 75) % 100]] == idx_b
        && (state[wreath_l[(shift + 50) % 100]] != idx_b || state[wreath_l[shift]] != idx_b)
    {
        return true;
    }

    false
}

fn is_candidate_right(
    state: &Vec<i8>,
    wreath_l: &Vec<usize>,
    wreath_r: &Vec<usize>,
    shift: usize,
    idx_a: i8,
    idx_b: i8,
) -> bool {
    if state[wreath_r[shift]] == idx_a && state[wreath_r[(shift + 100 - 26) % 100]] == idx_a {
        return true;
    }
    if state[wreath_l[75]] == idx_b {
        if state[wreath_r[shift]] == idx_b && state[wreath_r[(shift + 100 - 26) % 100]] != idx_b {
            return true;
        }
    }
    if state[wreath_l[50]] == idx_b {
        if state[wreath_r[(shift + 100 - 26) % 100]] == idx_b && state[wreath_r[shift]] != idx_b {
            return true;
        }
    }
    if state[wreath_r[(shift + 100 - 2 * 26) % 100]] == idx_a
        && (state[wreath_r[(shift + 100 - 3 * 26) % 100]] != idx_a
            || state[wreath_r[(shift + 100 - 26) % 100]] != idx_a)
    {
        return true;
    }
    if state[wreath_r[(shift + 26) % 100]] == idx_a
        && (state[wreath_r[(shift + 2 * 26) % 100]] != idx_a || state[wreath_r[shift]] != idx_a)
    {
        return true;
    }
    false
}

fn beam_search(
    puzzle: &Puzzle,
    allowed_move: &HashMap<String, Vec<i16>>,
    current_best: &String,
) -> Option<String> {
    let wreath_l = wreath_cycle(&allowed_move["l"]);
    let wreath_r = wreath_cycle(&allowed_move["r"]);
    let mut updated = false;
    let mut best_step = current_best.split(".").collect::<Vec<&str>>().len() as i16;
    let mut best_moves = current_best.clone();
    let piece_num = puzzle.initial_state.split(";").collect::<Vec<&str>>().len();
    let (piece_map, piece_list) = gen_piece_map(&puzzle.initial_state);
    let (action, action_str) = create_action(&allowed_move);
    let init_state = state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = state_to_list(&puzzle.solution_state, &piece_map);
    let mut checked_state = std::collections::HashMap::new();
    checked_state.insert(pack_state(&init_state), 0);
    let mut history = Vec::new();
    history.push(History {
        action: 0,
        cost: 0,
        node_id: 0,
        parent: None,
    });
    let idx_a = piece_map["A"];
    let idx_b = piece_map["B"];
    let idx_c = piece_map["C"];
    let mut nodes = Vec::new();
    nodes.push(Node {
        state: init_state.clone(),
        history_id: 0,
    });

    let mut cands = vec![std::collections::BinaryHeap::<Candidate>::new(); 1000];
    cands[0].push(Candidate {
        parent_node: 0,
        action: 100,
        cost: 0,
        heuristic: 0,
    });
    for it in 0..1000000 {
        if it % 10000 == 0 {
            println!("it: {}", it);
        }
        for cand_step in 0..cands.len() {
            if let Some(cand) = cands[cand_step].pop() {
                if cand_step == cands.len() - 1 {
                    println!("{} {} {}", cand_step, cand.cost, cand.heuristic);
                }
                let new_state = move_wreath(
                    &nodes[cand.parent_node].state,
                    &wreath_l,
                    &wreath_r,
                    cand.action,
                );
                let mut cur_cost = cand.cost;
                if cand.parent_node != 0 {
                    let mut history_id = nodes[cand.parent_node].history_id;
                    let mut sum = (cand.action % 100).min(100 - cand.action % 100) as i16;
                    while let Some(n) = history[history_id].parent {
                        sum += (history[history_id].action % 100)
                            .min(100 - history[history_id].action % 100)
                            as i16;
                        history_id = n;
                        if sum > cur_cost {
                            break;
                        }
                    }
                    cur_cost = cur_cost.min(sum);
                }
                if cur_cost >= best_step {
                    continue;
                }
                let key = pack_state(&new_state);
                if checked_state.contains_key(&key) {
                    let history_id = checked_state[&key];
                    if history[history_id].cost < cand.cost {
                        continue;
                    }

                    if history[history_id].cost > cand.cost {
                        history[history_id].cost = cand.cost;
                        history[history_id].action = cand.action;
                        history[history_id].parent = Some(nodes[cand.parent_node].history_id);
                    }
                } else {
                    checked_state.insert(key, history.len());
                    history.push(History {
                        action: cand.action,
                        cost: cand.cost,
                        node_id: nodes.len(),
                        parent: Some(nodes[cand.parent_node].history_id),
                    });
                    nodes.push(Node {
                        state: new_state.clone(),
                        history_id: history.len() - 1,
                    });
                }
                let mut dif = 0;
                for i in 0..new_state.len() {
                    if sol_state[i] != new_state[i] {
                        dif += 1;
                    }
                }
                if dif <= puzzle.num_wildcards {
                    let mut sol = Vec::new();
                    let mut history_id = nodes[cand.parent_node].history_id;
                    while let Some(n) = history[history_id].parent {
                        sol.push(history[history_id].action);
                        history_id = n;
                    }
                    sol.reverse();
                    best_step = cur_cost;
                    best_moves = sol
                        .iter()
                        .map(|v| action_id_to_str(*v))
                        .collect::<Vec<String>>()
                        .join(".")
                        + "."
                        + &action_id_to_str(cand.action);
                    assert_eq!(
                        best_step,
                        best_moves.split(".").collect::<Vec<&str>>().len() as i16
                    );
                    println!("best_step: {}", best_step);
                    println!("best_moves: {}", best_moves);
                    updated = true;
                    continue;
                }
                if cand_step + 1 >= cands.len() {
                    continue;
                }
                let current_parent_node = history[checked_state[&key]].node_id;
                if cand.action >= 100 {
                    for i in 1..wreath_l.len() {
                        if is_candidate_left(&new_state, &wreath_l, &wreath_r, i, idx_a, idx_b) {
                            let nnext_state =
                                move_wreath(&new_state, &wreath_l, &wreath_r, i as u8);
                            let heuristic = calc_cost(
                                &nnext_state,
                                &sol_state,
                                &wreath_l,
                                &wreath_r,
                                idx_a,
                                idx_b,
                                puzzle.num_wildcards,
                            );
                            cands[cand_step + 1].push(Candidate {
                                parent_node: current_parent_node,
                                action: i as u8,
                                cost: cand.cost + i.min(100 - i) as i16,
                                heuristic: heuristic,
                            });
                        }
                    }
                } else {
                    for i in 1..wreath_r.len() {
                        if is_candidate_right(&new_state, &wreath_l, &wreath_r, i, idx_a, idx_b) {
                            let nnext_state =
                                move_wreath(&new_state, &wreath_l, &wreath_r, i as u8 + 100);
                            let heuristic = calc_cost(
                                &nnext_state,
                                &sol_state,
                                &wreath_l,
                                &wreath_r,
                                idx_a,
                                idx_b,
                                puzzle.num_wildcards,
                            );
                            cands[cand_step + 1].push(Candidate {
                                parent_node: current_parent_node,
                                action: i as u8 + 100,
                                cost: cand.cost + i.min(100 - i) as i16,
                                heuristic: heuristic,
                            });
                        }
                    }
                }
            }
        }
    }
    if updated {
        Some(best_moves)
    } else {
        None
    }
}

fn solve_wreath() {
    let puzzle_info = load_puzzle_info();
    let puzzles = load_puzzle();
    let submission = load_submission();
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
        if !row.puzzle_type.ends_with("100") {
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

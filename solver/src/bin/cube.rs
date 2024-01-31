// Reference
// * https://www.kaggle.com/code/seanbearden/solve-all-nxnxn-cubes-w-traditional-solution-state
// Dependencies
// * https://github.com/dwalton76/rubiks-cube-NxNxN-solver

use std::collections::HashMap;

mod cube_moves;

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

fn solve_cube_edges(
    cur_state: &Vec<usize>,
    actions: &HashMap<String, Vec<(usize, usize)>>,
    current_solution: &String,
    temporary_solution: &Vec<String>,
    dim: usize,
) -> (Vec<usize>, Vec<String>) {
    let sol_state: Vec<usize> = (0..6 * dim * dim).collect();
    let mut state = sol_state.clone();
    let cur_moves = current_solution.split(".").collect::<Vec<&str>>();
    for m in cur_moves.iter().rev() {
        let action = &actions[&cube_moves::rev_move(&m.to_string())];
        state = cube_moves::apply_action(&state, action);
    }
    for m in temporary_solution.iter() {
        let action = &actions[m];
        state = cube_moves::apply_action(&state, action);
    }
    let (_, moves) = cube_moves::solve_cube_edges(&state, &sol_state, &actions, dim);
    let mut state = cur_state.clone();
    for m in moves.iter() {
        state = cube_moves::apply_action(&state, &actions[m]);
    }
    (state, moves)
}

fn solve_cube_by_solver_with_center_rot(
    puzzle: &solver::Puzzle,
    allowed_moves: &HashMap<String, Vec<i16>>,
    dim: usize,
) -> Option<String> {
    if dim % 2 == 0 {
        return solve_cube_by_solver(
            &puzzle.initial_state,
            &puzzle.solution_state,
            allowed_moves,
            dim,
        );
    }
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let actions = cube_moves::create_actions(&allowed_moves);
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
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
    fn evaluate(
        cur_state: &Vec<usize>,
        sol_state: &String,
        allowed_moves: &HashMap<String, Vec<i16>>,
        piece_list: &Vec<&str>,
        dim: usize,
    ) -> Option<String> {
        let init_state = cur_state
            .iter()
            .map(|i| piece_list[*i])
            .collect::<Vec<&str>>()
            .join(";");
        solve_cube_by_solver(&init_state, sol_state, allowed_moves, dim)
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

    let mut best_move: Option<String> = None;
    let mut best_cost = 100000000;
    if valid_state(&init_state, &sol_state, dim) {
        if let Some(cur_move) = evaluate(
            &init_state,
            &puzzle.solution_state,
            allowed_moves,
            &piece_list,
            dim,
        ) {
            let len = cur_move.split(".").collect::<Vec<&str>>().len();
            if len < best_cost {
                best_move = Some(cur_move);
                best_cost = len;
            }
        }
    }
    let mut cur_state = init_state.clone();
    for (idx0, act0) in action_list.iter().enumerate() {
        cur_state = cube_moves::apply_action(&cur_state, &actions[act0]);
        if valid_state(&cur_state, &sol_state, dim) {
            if let Some(cur_move) = evaluate(
                &cur_state,
                &puzzle.solution_state,
                allowed_moves,
                &piece_list,
                dim,
            ) {
                let len = cur_move.split(".").collect::<Vec<&str>>().len();
                if len + 1 < best_cost {
                    best_move = Some(format!("{}.{}", act0, cur_move));
                    best_cost = len + 1;
                }
            }
        }
        for (idx1, act1) in action_list.iter().enumerate() {
            if (idx1 + 3) % 6 == idx0 {
                continue;
            }
            cur_state = cube_moves::apply_action(&cur_state, &actions[act1]);
            if valid_state(&cur_state, &sol_state, dim) {
                if let Some(cur_move) = evaluate(
                    &cur_state,
                    &puzzle.solution_state,
                    allowed_moves,
                    &piece_list,
                    dim,
                ) {
                    let len = cur_move.split(".").collect::<Vec<&str>>().len();
                    if len + 2 < best_cost {
                        best_move = Some(format!("{}.{}.{}", act0, act1, cur_move));
                        best_cost = len + 2;
                    }
                }
            }
            for (idx2, act2) in action_list.iter().enumerate() {
                if (idx2 + 3) % 6 == idx1 {
                    continue;
                }
                cur_state = cube_moves::apply_action(&cur_state, &actions[act2]);
                if valid_state(&cur_state, &sol_state, dim) {
                    if let Some(cur_move) = evaluate(
                        &cur_state,
                        &puzzle.solution_state,
                        allowed_moves,
                        &piece_list,
                        dim,
                    ) {
                        let len = cur_move.split(".").collect::<Vec<&str>>().len();
                        if len + 3 < best_cost {
                            best_move = Some(format!("{}.{}.{}.{}", act0, act1, act2, cur_move));
                            best_cost = len + 3;
                        }
                    }
                }
                cur_state =
                    cube_moves::apply_action(&cur_state, &actions[&cube_moves::rev_move(act2)]);
            }
            cur_state = cube_moves::apply_action(&cur_state, &actions[&cube_moves::rev_move(act1)]);
        }
        cur_state = cube_moves::apply_action(&cur_state, &actions[&cube_moves::rev_move(act0)]);
    }
    best_move
}

fn solve_cube_by_rule(
    puzzle: &solver::Puzzle,
    allowed_moves: &HashMap<String, Vec<i16>>,
    current_solution: &String,
    dim: usize,
) -> Option<String> {
    let (piece_map, piece_list) = solver::gen_piece_map(&puzzle.solution_state);
    let actions = cube_moves::create_actions(&allowed_moves);
    let init_state = solver::state_to_list(&puzzle.initial_state, &piece_map);
    let sol_state = solver::state_to_list(&puzzle.solution_state, &piece_map);
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
    let (end_state, m) = cube_moves::solve_first_two_faces(&state, &sol_state, &actions, dim);
    state = end_state;
    moves.extend(m);

    println!(
        "end first two: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    let (end_state, m) = cube_moves::solve_four_faces_impl(&state, &sol_state, &actions, dim, 100);
    state = end_state;
    moves.extend(m);
    println!(
        "end four: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
    let (end_state, m) = solve_cube_edges(&state, &actions, &current_solution, &moves, dim);
    state = end_state;
    moves.extend(m);
    println!(
        "end edge: {}",
        solver::cancel_moves_in_cube(&moves.join("."))
            .split(".")
            .collect::<Vec<&str>>()
            .len()
    );
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
    if let Some(last_move) =
        solve_cube_by_solver(&cur_state, &puzzle.solution_state, allowed_moves, dim)
    {
        moves.push(last_move);
        Some(moves.join("."))
    } else {
        None
    }
}

fn solve_cube_by_solver(
    init_state: &String,
    sol_state: &String,
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

    // println!("raw_solution {}", sol);
    // join M[m] for m in sol.split(" ") with "."
    let move_map = move_translation(dim);
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
    current_solution: &String,
    dim: usize,
) -> Option<String> {
    let sol_state = &puzzle.solution_state;

    let checker_cube = sol_state.starts_with("A;B;A;B;A;B;A;B;A");
    let distinct_cube = sol_state.starts_with("N0");
    if dim % 2 == 0 && checker_cube {
        return None;
    }
    if distinct_cube {
        return None;
    }
    if dim <= 5 {
        solve_cube_by_solver_with_center_rot(puzzle, allowed_moves, dim)
    } else {
        solve_cube_by_rule(puzzle, allowed_moves, current_solution, dim)
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

        // Already have optimal solution for N=2
        // N=3 is solved by DeepCubeA* solver
        if dim <= 3 {
            continue;
        }

        let moves = &puzzle_info[&row.puzzle_type];
        if let Some(result) = solve_cube(&row, &moves, &submission_df[&id], dim) {
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

fn main() {
    solve();
}

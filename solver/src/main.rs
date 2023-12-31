use csv;
use serde_json;
use std::collections::HashMap;

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

fn solve_cubes() {
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
        if row.puzzle_type.split("_").collect::<Vec<&str>>()[0] == "cube" {
            let dim = row
                .puzzle_type
                .split("/")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .parse::<usize>()
                .unwrap();
            if dim == 2 {
                continue;
            }
            let moves = &puzzle_info[&row.puzzle_type];
            let move_map = move_translation(dim);
            let init_state = &row.initial_state;
            let sol_state = &row.solution_state;

            let standard_cube = sol_state[0..2 * dim.pow(2) - 1] == vec!["A"; dim.pow(2)].join(";");
            let checker_cube = dim % 2 == 1 && sol_state[0..17] == *"A;B;A;B;A;B;A;B;A";

            if checker_cube {
                let state = if standard_cube {
                    state2ubl(init_state)
                } else {
                    checker_state2cbl(init_state, dim)
                };
                println!("Starting {}", id);
                println!("{}", state);
                let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
                let relative_path =
                    std::path::Path::new(&project_dir).join("../rubiks-cube-NxNxN-solver");

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
                println!("raw_solution {}", sol);
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
                    new_state = moves[m]
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
                            temp_state = moves[m]
                                .iter()
                                .map(|i| temp_state_vec[*i as usize])
                                .collect::<Vec<&str>>()
                                .join(";")
                                .to_string();
                        }
                    }

                    if temp_state == *sol_state {
                        println!("solved id: {}", id);
                        if init_moves.len() > 0 {
                            if mmoves.len() > 0 {
                                mmoves.push_str(".");
                            }
                            mmoves.push_str(&init_moves);
                        }
                        break;
                    }
                }
                println!("solution: {}", mmoves);

                // validation
                let mut state = row.initial_state.clone();
                for m in mmoves.split(".") {
                    let state_vec: Vec<&str> = state.split(";").collect();
                    // reorder new_state_vec with moves[m] and join with ";"
                    state = moves[m]
                        .iter()
                        .map(|i| state_vec[*i as usize])
                        .collect::<Vec<&str>>()
                        .join(";")
                        .to_string();
                }

                println!("sol_state: {}", sol_state);
                println!("state    : {}", state);
                assert_eq!(*sol_state, state);
                let mmoves_length = mmoves.split(".").collect::<Vec<&str>>().len();
                let best_moves_length = submission_df[&id].split(".").collect::<Vec<&str>>().len();
                println!("find: {}, best: {}", mmoves_length, best_moves_length);
                if mmoves_length < best_moves_length {
                    submission_df.insert(id, mmoves);
                }
            }
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
    solve_cubes();
}

use csv;
use serde_json;
use std::collections::HashMap;

pub fn calc_dim(l: usize) -> usize {
    for i in 1..50 {
        if i * i == l / 6 {
            return i;
        }
    }
    panic!("Invalid length of state");
}

// Load csv file from the given path.
pub fn load_csv(path: &str) -> Vec<Vec<String>> {
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

pub fn load_submission() -> Vec<Vec<String>> {
    load_csv("../submission.csv")
}

pub fn load_puzzle_info() -> HashMap<String, HashMap<String, Vec<i16>>> {
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

pub struct Puzzle {
    pub puzzle_type: String,
    pub solution_state: String,
    pub initial_state: String,
    pub num_wildcards: i16,
}

pub fn load_puzzle() -> Vec<Puzzle> {
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

pub fn gen_piece_map(state: &str) -> (HashMap<&str, usize>, Vec<&str>) {
    let mut piece_map: HashMap<&str, usize> = HashMap::new();
    let mut piece_list: Vec<&str> = Vec::new();
    for f in state.split(";") {
        if !piece_map.contains_key(f) {
            piece_map.insert(f, piece_list.len());
            piece_list.push(f);
        }
    }
    (piece_map, piece_list)
}

pub fn state_to_list(state: &str, piece_map: &HashMap<&str, usize>) -> Vec<usize> {
    state
        .split(";")
        .map(|f| piece_map[f])
        .collect::<Vec<usize>>()
}

pub fn list_to_state(list: &Vec<usize>, piece_list: &Vec<&str>) -> String {
    list.iter()
        .map(|i| piece_list[*i])
        .collect::<Vec<&str>>()
        .join(";")
}

pub fn cancel_moves_in_cube(solution: &String) -> String {
    fn get_side(m: &str) -> &str {
        if m.starts_with("-") {
            get_side(&m[1..])
        } else {
            &m[0..1]
        }
    }
    fn get_sign(m: &str) -> usize {
        if m.starts_with("-") {
            3
        } else {
            1
        }
    }
    fn get_index(m: &str) -> usize {
        if m.starts_with("-") {
            get_index(&m[1..])
        } else {
            m[1..].parse::<usize>().unwrap()
        }
    }
    let mut arr = Vec::new();
    let moves = solution.split(".").collect::<Vec<&str>>();
    let mut idx = 0;
    let mut reduce = 0;
    let cost = [0, 1, 2, 1];
    let mut has_clear = false;
    while idx < moves.len() {
        let m = moves[idx];
        let side = get_side(m);
        let index = get_index(m);
        let mut cnt = vec![0; 33];
        cnt[index] = (cnt[get_index(m)] + get_sign(m)) % 4;
        let mut next_idx = idx + 1;
        while next_idx < moves.len() {
            let next_m = moves[next_idx];
            let next_side = get_side(next_m);
            if next_side != side {
                break;
            }
            let next_index = get_index(next_m);
            cnt[next_index] = (cnt[next_index] + get_sign(next_m)) % 4;
            next_idx += 1;
        }
        let sum_cost = cnt
            .iter()
            .map(|v| cost[*v])
            .reduce(|acc, e| acc + e)
            .unwrap();
        if sum_cost == 0 {
            has_clear = true;
        }
        if sum_cost < next_idx - idx {
            for (i, v) in cnt.iter().enumerate() {
                if *v == 1 {
                    arr.push(format!("{}{}", side, i));
                } else if *v == 2 {
                    arr.push(format!("{}{}", side, i));
                    arr.push(format!("{}{}", side, i));
                } else if *v == 3 {
                    arr.push(format!("-{}{}", side, i));
                }
            }
            reduce += next_idx - idx - sum_cost;
        } else {
            for i in idx..next_idx {
                arr.push(moves[i].to_string());
            }
        }
        idx = next_idx;
    }
    if reduce > 0 {
        let res = arr.join(".").to_string();
        if has_clear {
            cancel_moves_in_cube(&res)
        } else {
            res
        }
    } else {
        solution.to_string()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_cancel_moves_in_cube() {
        use super::cancel_moves_in_cube;
        assert_eq!(cancel_moves_in_cube(&"r0.-r0.r1".to_string()), "r1");
        assert_eq!(cancel_moves_in_cube(&"r0.r0.r0".to_string()), "-r0");
        assert_eq!(cancel_moves_in_cube(&"-r0.-r0.-r0".to_string()), "r0");
        assert_eq!(
            cancel_moves_in_cube(&"r0.r2.r1.r3.f0.-r0.-r2.-r1.-r3".to_string()),
            "r0.r2.r1.r3.f0.-r0.-r2.-r1.-r3"
        );
        assert_eq!(
            cancel_moves_in_cube(&"f1.r0.f0.-f0.-r0.f1".to_string()),
            "f1.f1"
        );
    }
}

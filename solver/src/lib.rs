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

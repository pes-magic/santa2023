"""Evaluation metric for Santa 2023.

https://www.kaggle.com/code/metric/santa-2023-metric
"""

import argparse
import pandas as pd


parser = argparse.ArgumentParser(description="Check puzzle info with given solution.")

parser.add_argument("file", type=str, help="Path of the solution file.")

args = parser.parse_args()

puzzle = pd.read_csv("./raw_data/puzzles.csv")
solution = pd.read_csv(args.file)
solution2 = pd.read_csv("submission.csv")

puzzle_type = puzzle["puzzle_type"]
state = puzzle["solution_state"]
wildcards = puzzle["num_wildcards"]
piece_num = [len(set(s.split(";"))) for s in state]
size = [len(s.split(";")) for s in state]
step = [len(s.split(".")) for s in solution["moves"]]
step2 = [len(s.split(".")) for s in solution2["moves"]]

df = pd.DataFrame(
    {
        "id": puzzle["id"].values,
        "puzzle_type": puzzle_type,
        "piece_num": piece_num,
        "size": size,
        "num_wildcards": wildcards,
        "step": step,
        "step2": step2,
    }
)
df.to_csv("puzzle_info.csv")

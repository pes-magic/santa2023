import subprocess
import pandas as pd
import matplotlib.pyplot as plt
from io import StringIO


def show_commit_history(csv_file_path):
    # Gitコマンドを使用して、CSVファイルのコミットハッシュと日時を取得
    cmd = f'git log --pretty=format:"%h|%cd" --date=short -- {csv_file_path}'
    commit_log = subprocess.check_output(cmd, shell=True).decode().split("\n")

    # 各コミットのデータを保持するためのリスト
    commit_data = []

    # 各コミットに対して処理を行う
    for entry in commit_log:
        try:
            commit_hash, commit_date = entry.split("|")
            # 特定のコミットでのCSVファイルの内容を取得
            cmd = f"git show {commit_hash}:{csv_file_path}"
            csv_content = subprocess.check_output(cmd, shell=True).decode()

            # CSV内容をPandas DataFrameに読み込む
            df = pd.read_csv(StringIO(csv_content))

            # 各カテゴリのstep2の合計を計算
            globe_total = df[df["puzzle_type"].str.startswith("globe")]["step2"].sum()
            wreath_total = df[df["puzzle_type"].str.startswith("wreath")]["step2"].sum()
            cube_total = df[df["puzzle_type"].str.startswith("cube")]["step2"].sum()

            # コミットのデータをリストに追加（コミットハッシュ + 日付）
            commit_label = f"{commit_hash} ({commit_date})"
            commit_data.append((commit_label, commit_date, globe_total, wreath_total, cube_total))
        except subprocess.CalledProcessError:
            # エラーが発生した場合（例：ファイルが存在しないコミット）
            continue

    # データをDataFrameに変換し、日付でソート
    commit_df = pd.DataFrame(commit_data, columns=["Commit", "Date", "Globe", "Wreath", "Cube"])
    commit_df.sort_values(by=["Date"], inplace=True)

    # 積み上げ棒グラフの作成
    commit_df.set_index("Commit").plot(kind="bar", stacked=True, figsize=(10, 6))
    plt.xlabel("Commit Hash and Date")
    plt.ylabel("Total Step")
    plt.title("Total Step by Commit and Puzzle Type")
    plt.xticks(rotation=45)
    # グラフをPNGファイルとして保存
    plt.savefig("graph/commit_graph.png", bbox_inches="tight")


def show_puzzle_type_step(csv_file_path):
    # Gitコマンドを使用して、最新のコミットのCSVファイルの内容を取得
    cmd = f"git show HEAD:{csv_file_path}"
    csv_content = subprocess.check_output(cmd, shell=True).decode()

    # CSV内容をPandas DataFrameに読み込む
    df = pd.read_csv(StringIO(csv_content))

    # 最初に登場するpuzzle_typeごとのインデックスを取得
    first_occurrence = df.groupby("puzzle_type").first().reset_index()
    order = first_occurrence.sort_values(by="id").index

    # 'puzzle_type'と'piece_num'ごとにグループ化し、step2の合計値を計算
    grouped_df = df.groupby(["puzzle_type", "piece_num"]).sum()["step2"].unstack(fill_value=0)

    # puzzle_typeの登場順にDataFrameを並べ替え
    grouped_df = grouped_df.iloc[order]

    # 積み上げ棒グラフの作成
    grouped_df.plot(kind="bar", stacked=True, figsize=(10, 6))
    plt.xlabel("Puzzle Type")
    plt.ylabel("Total Steps")
    plt.title("Total Steps by Puzzle Type and Piece Num")
    plt.legend(title="Piece Kind")
    # グラフをPNGファイルとして保存
    plt.savefig("graph/graph_puzzle_type.png", bbox_inches="tight")


# CSVファイルのパス
csv_file_path = "puzzle_info.csv"

show_commit_history(csv_file_path)
show_puzzle_type_step(csv_file_path)

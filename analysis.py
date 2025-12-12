# %%
from pathlib import Path

import numpy as np
import pandas as pd
import ultraplot as uplt

data_dir = Path("./data")

df = []
for file in data_dir.glob("*.csv"):
    df.append(pd.read_csv(file))
df = pd.concat(df, ignore_index=True)

print(df.head(2))
# %%
df["epsilon"].unique()
# %%
epsilons = np.unique(df["epsilon"])
colors = uplt.Colormap("538")(np.linspace(0, 1, epsilons.size))
actions = ["C", "D"]


fig, ax = uplt.subplots()
for group, dfi in df.groupby("player epsilon".split()):
    player, epsilon = group
    rounds = np.unique(dfi["round"])
    trials = np.unique(dfi["trial"])
    payoff = dfi["payoff"].values.reshape(max(rounds), max(trials) + 1)
    mu = payoff.mean(1)
    idx = int(np.argmin(abs(epsilons - epsilon)))
    color = colors[idx]
    lsi = "-" if "1" in player else "--"

    strategy = dfi["action"].values.reshape(max(rounds), max(trials) + 1)
    from collections import Counter

    played_c = np.zeros(len(strategy))
    for t, strat in enumerate(strategy):
        binned = Counter(strat)
        z = sum(binned.values())
        binned = {k: v / z for k, v in binned.items()}
        played_c[t] = binned.get("C", 0)
    ax.plot(played_c, color=colors[idx], ls=lsi)


ax.format(xlabel="Rounds", ylabel="Fraction C", ylim=(-0.05, 1.05))
h = [uplt.pyplot.Line2D([], [], color=c, label=e) for c, e in zip(colors, epsilons)]
ax.legend(handles=h, ncols=1, loc="r", frameon=0, title="$\epsilon$")

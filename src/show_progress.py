import contextlib
import json
import math
import os
import pprint

from cycler import cycler
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd


here_path = os.path.dirname(__file__)
leaderboard_path = os.path.join(here_path, '..', 'data', 'leaderboard.json')

with open(leaderboard_path, mode='r', encoding='utf-8') as file:
    player_stats = json.load(file)

player_stats = player_stats['members']
n_members = len(player_stats)

# List of dict
#   name
#   stars: {day: [timestamp, timestamp]}
simple_stats = []
for member in player_stats.values():
    print(json.dumps(member, indent=2))
    star_stats = {}
    for day, stars in member['completion_day_level'].items():
        tmp = []
        tmp.append(stars['1']['get_star_ts'])
        with contextlib.suppress(KeyError):
            tmp.append(stars['2']['get_star_ts'])
        star_stats[int(day)] = tmp

    simple_stats.append({
        'name': member['name'],
        'stars': star_stats
    })
pprint.pp(simple_stats)

# Print sum of stars over time.
simpler_stats = {}
simplest_stats = {}
fix, ax = plt.subplots()
for member in simple_stats:
    stars = []
    for timestamps in member['stars'].values():
        stars.extend(timestamps)
    stars.sort()
    simpler_stats[member['name']] = stars
    for star in stars:
        simplest_stats[star] = member['name']
    ax.plot(stars, range(len(stars)), '-*', label=member['name'])
pprint.pp(simplest_stats)
ax.legend(loc='upper left')
# plt.show(block=False)

members = [member['name'] for member in simple_stats]
members.sort()

stats_df = pd.DataFrame(
    index=members,
    columns=[f'Day {math.floor(star/2) + 1} Star {star%2 + 1}' for star in range(50)]
)

# Fill the dataframe
for member in simple_stats:
    name = member['name']
    for day, stars in member['stars'].items():
        stats_df.loc[name][f'Day {day} Star 1'] = stars[0]
        with contextlib.suppress(IndexError):
            stats_df.loc[name][f'Day {day} Star 2'] = stars[1]


# Compute score add at each star acquisition.
scores_df = pd.DataFrame(index=members)
for column, data in stats_df.items():
    sorted = data.sort_values(ascending=True)
    scored = pd.Series(
        [i for i in range(len(sorted), 0, -1)],
        index=sorted.index,
    )
    scored[sorted.isnull()] = 0
    scores_df[column] = scored

# print(scores_df)

fig, ax = plt.subplots()
ax.set_prop_cycle(
    cycler(linestyle=['-', '--'], marker=['*', '*'])
    * cycler(color=['b', 'g', 'r', 'c', 'm', 'y', 'k'])
)
member_score_dfs = {}

# Set up a mapping for the order of events.
timestamps = stats_df.values.flatten()
timestamps = [ts for ts in timestamps if not np.isnan(ts)]
timestamps.sort()
# print(timestamps)
events = {ts: idx for idx, ts in enumerate(timestamps)}
# pprint.pp(events)

for member in members:
    df = pd.DataFrame(
        scores_df.loc[member].values,
        columns=["plus"],
        index=stats_df.loc[member],
    )
    df.sort_index(inplace=True)
    df["Score"] = df['plus'].cumsum()
    member_score_dfs[member] = df
    # Make the x axis event-based.
    x_vals = []
    for ts in df.index:
        try:
            x_vals.append(events[ts])
        except KeyError:
            # nan
            x_vals.append(ts)
    ax.plot(x_vals, df['Score'], label=member)
ax.legend()
plt.show()

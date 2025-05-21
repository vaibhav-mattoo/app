# Algorithm Overview

- have a scoring system based on frequency of occurance of commands and length of commands and time of recency of commands at granularity of weeks.
- take the prefix at every space in the command and add scores to these prefixes (length handled by scoring mechanism)
- When we enter a new command, updates to the scoring system must occur asynchronously at entering a new command in a shell, and store the last access time and based on the last access time and the current time difference we assign a multiplier to the score.
- If 100 commands in the previous day and 5 commands in the last hour, might want to migrate to 100*1 + 5*4 instead of 101*4.
- storage per command class in data structure.
  - last access time
  - frequency
  - length
  - score
- Based on the number of occurances of commands exceeding some big constant, to prevent integer overflows or overtraining we divide all scores by some constant and if the new scores for some commands falls below some limit then we remove it from the suggestions list.
- let user pick the alias, later might have learning based system to suggest an alias, need to ensure that the alias does not already exist or clash with another command.
- Need to initialize with some set of common aliases.
- Need a TUI and a CLI for the app.
- Modes needed for adding, already available aliases (where we can delete aliases).

# Playing the Game

[How to play](https://www.youtube.com/watch?v=Ol3Id7xYsY4)

The program won't let you play an invalid move. Bots (minimax or random) will move without user input.

Quickstart:

```bash
$ make
Compiling simplified_othello v0.1.0 (PATH)
Finished release [optimized] target(s) in 1.03s

The othello command was added to this folder
$ ./othello human minimax

Player 1 (X) score: 2
Player 2 (O) score: 2

3:| . ? . .
2:| ? O X .
1:| . X O ?
0:| . . ? .
   ---------
    0 1 2 3

Player 1 (X) move:
Enter row (or ?): ?
Row: 0, Col: 2
Row: 1, Col: 3
Row: 2, Col: 0
Row: 3, Col: 1
Enter row (or ?): 3
Enter col (or ?): ?
Row: 3, Col: 1
Enter col (or ?): -2
You must enter positive numerical numerical values.
Enter col (or ?): what
You must enter positive numerical numerical values.
Enter col (or ?): 1
[Selected] Row: 3, Col: 1

Player 1 (X) score: 4
Player 2 (O) score: 1

3:| ? X ? .
2:| . X X .
1:| ? X O .
0:| . . . .
   ---------
    0 1 2 3

Player 2 (O) move:
[Selected] Row: 3, Col: 0

Player 1 (X) score: 3
Player 2 (O) score: 3

3:| O X . .
2:| ? O X .
1:| . X O ?
0:| . . ? .
   ---------
    0 1 2 3

Player 1 (X) move:
Enter row (or ?):
```

## Compiling

### With Make

```sh
make
./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. On my 5GHz 6-Core it takes 5 minutes to complete
```

#### Compiling with Random Player Option

Enables usage of the `random` player which picks the first valid move it randomly generates. Useful for testing.

```sh
make with_random
./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. On my 5GHz 6-Core it takes 5 minutes to complete
```

### With Cargo

```sh
cargo build --release && mv target/release/othello .
./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. On my 5GHz 6-Core it takes 5 minutes to complete
```

#### Compiling with Random Player Option

Enables usage of the `random` player which picks the first valid move it randomly generates. Useful for testing.

```sh
cargo build --release --features with_random && mv target/release/othello .
./othello <player> <player> [SIZE] [SIZE] # I wouldn't suggest over 5x5. On my 5GHz 6-Core it takes 5 minutes to complete
```

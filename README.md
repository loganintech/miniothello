# Playing the Game

[How to play](https://www.youtube.com/watch?v=Ol3Id7xYsY4)

The program won't let you play an invalid move. Bots (minimax or random) will move without user input.

```
Player 1 (X) score: 2
Player 2 (O) score: 2

3:| . . . .
2:| . O X .
1:| . X O .
0:| . . . .
   ---------
    0 1 2 3

Player 1 (X) move:
Enter row: 3
Enter col: 0
Invalid move.
Enter row: 1
Enter col: 3
[Selected] Row: 1, Col: 3

Player 1 (X) score: 4
Player 2 (O) score: 1

3:| . . . .
2:| . O X .
1:| . X X X
0:| . . . .
   ---------
    0 1 2 3

Player 2 (O) move:
[Selected] Row: 0, Col: 3
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

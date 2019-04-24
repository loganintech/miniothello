import sys
import subprocess

count = {
    0: 0,
    1: 0,
    2: 0,
}

for i in range(1000):
    res = subprocess.run(["./othello", "random", "minimax", sys.argv[1] if len(sys.argv) > 1 else 4, sys.argv[2] if len(sys.argv) > 2 else 4])
    count[res.returncode] += 1

print(count)

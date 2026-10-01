#!/usr/bin/env python3
"""Generate a large synthetic alignment for benchmarking phyloevolve."""
import random
import sys

random.seed(42)
n_seqs = int(sys.argv[1]) if len(sys.argv) > 1 else 5000
seq_len = int(sys.argv[2]) if len(sys.argv) > 2 else 5000
out_path = sys.argv[3] if len(sys.argv) > 3 else "bench.fasta"

bases = "ACGT"
with open(out_path, "w") as f:
    for i in range(n_seqs):
        f.write(f">seq{i}\n")
        f.write("".join(random.choice(bases) for _ in range(seq_len)) + "\n")

print(f"wrote {n_seqs} sequences x {seq_len}bp to {out_path}")

# Coverage/CRAP after

Status: complete
Evidence mode: Ran

After behavior-preserving decomposition and review corrections, the focused
parser suite passed 36/36. Targeted coverage was 687/741 lines (92.713%),
738/763 regions (96.723%), and 35/37 functions. The named-function region floor
was 80.0%, above the 75% gate. Maximum deduplicated CRAP was 16.352 for
`parse_required_branch`, below the ceiling of 30.

Commands exited zero:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --test infile_chaninp_parser_contract --lcov --output-path /tmp/chaninp-fq02-terminal.lcov
cargo llvm-cov --workspace --test infile_chaninp_parser_contract --json --no-clean --output-path /tmp/chaninp-fq02-terminal.json
cargo crap --workspace --lcov /tmp/chaninp-fq02-terminal.lcov --min 0 --format json --output /tmp/chaninp-fq02-terminal-crap.json
```

Raw evidence:

- `coverage-after.json` SHA-256
  `8aadb0d7555d672d12bce520d35b24d521af54fb4dc808db088e1a5468ded876`
- `lcov.info` SHA-256
  `a651390da3e7ddabea9d14df6681c6351d78fc67bc28084af8605e6afb3a2990`
- `crap-after.json` SHA-256
  `857b1751d9089e8030f2664b22f2785b8982dd9b7ee3b1a63939ac8ad632e13b`

The terminal run is anchored to scaffold HEAD
`3caf5fbe2c24e4f6963ddf9667a0907a74dc9a38` plus the package worktree:
source SHA-256
`f7857a4cbd5a0bdb5f7ade1bf4e2d8871811988791f79dcb77fe5af33b59646d`
(30,065 bytes) and focused-test SHA-256
`bb7475b308acd1364e7c8037fc4495a321ec1de8d46abb78a0fdf4c62f620c9e`
(31,436 bytes). Timings were 1.37 seconds clean, 22.43 seconds LCOV,
0.54 seconds JSON, and 0.91 seconds CRAP. Raw sizes are 1,065,734 bytes,
208,912 bytes, and 2,779,116 bytes respectively.

Uncovered arms are defensive/type-impossible after preceding structural
validation: compiler-generated closure fallback, negative-to-unsigned
conversion after normalization, empty token arms after nonempty collection,
post-normalization invariant checks, finite checks shadowed by numeric parsing,
and a nondeterministic read-race `NotFound` arm. No exclusions were removed
from the denominator.

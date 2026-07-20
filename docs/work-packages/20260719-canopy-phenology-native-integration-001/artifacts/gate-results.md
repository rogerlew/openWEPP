# Gate Results

Evidence mode: `Ran`

Status: `PASS — exact terminal receipt independently verified twice`

## Exact Terminal Evidence

- Exact head: `7053a9b834bfe4e369d57c5b12ddcececf9c4f1e`
- Intent plan:
  `1134bc7f9388af74ba1bd162113016ead748a99bd5d6ccf8f7e5f53894e0ba71`
- Terminal plan:
  `c1d3fa141ad6c9625218a2ed099f10f93c1a65f1157986f80cd6cefc4d80a42c`
- Receipt:
  `ed12971db7acb98e5a8ac3bd79452853fe25429a68f6bea08d4a5aab0f5d68af`
- Receipt path: `/tmp/c13r.json`
- Artifact root: `/tmp/c03e`
- Result: 15 passed; 0 failed, blocked, skipped, or retried; unavailable
  inventory empty.
- Trust class: `LOCAL_UNTRUSTED`; the executor self-verification and two
  subsequent independent `verify-receipt` processes all returned `PASS` with
  the same receipt ID.
- Reconciliation: no added or removed paths and no risk escalation.
- Authenticated environment:
  `ddcb915642236e6e1904f78ae1208ab24cdb6921f5b205f5e1b422108bf782a1`.

## Required Gate Results

| Gate | Result |
|---|---|
| A0 science-contract admission | PASS; 39 contracts and 18 science surfaces admitted |
| Authority-suite anti-evasion | PASS |
| Dependency policy | PASS |
| Formatting | PASS |
| Documentation lint | PASS; 29 files, 0 errors or warnings |
| Gate-policy schema | PASS |
| Placeholder scan | PASS |
| Required A3 authority | CONFORMS |
| Workspace Clippy | PASS |
| Workspace doctests | PASS |
| Native canopy management A1 | CONFORMS; 71/71 tests |
| Native canopy plant A1 | CONFORMS; 19/19 tests |
| Native canopy runtime A1 | CONFORMS; 624/624 tests in 155.5 seconds |
| Full-workspace Nextest | PASS; 2,185/2,185 tests, 18 slow, 5 configured skips, 1,100.1 seconds |
| Global adjudicated CRAP | PASS; 2 raw, 2 adjudicated, 0 actionable, 17 touched files, closure eligible |

The fresh global coverage run independently passed 2,185/2,185 tests with 5
configured skips in 908.8 seconds. Its production-source manifest contains 249
sources. Report hashes are:

- source manifest:
  `e073235870e4efb05425a91fee600015774d2b3465c6bc31fc16a7d757695bb5`
- LCOV: `36cd734fe1d9f6c33b13b3d03082228a0dd6cb736a32c59e47648fb835d21422`
- CRAP JSON:
  `d54d68cff6fcd579e0141fec38c7635e7d481811fab2cb0a0edaa95ab3bde401`

## Diagnostic History

Earlier receipts and stopped preflights remain diagnostic, not closure
evidence. C10 stopped before node execution because its intent omitted the
allowlisted Rust/Cargo environment. C11 emitted a verified non-pass receipt
after reusing snapshot-poisoned Cargo binaries. C12 passed all 15 nodes at
`37e3223b`, but its evidence was invalidated when mandatory line-count
governance required two behavior-preserving module splits. The first C13
launch stopped before node execution on a preserved C12 output collision; the
unchanged C13 relaunch used empty executor roots and produced the exact passing
receipt above.

# Gate Results

Evidence class: `Ran`

Exact current bytes passed:

- Full workspace: `cargo nextest run --workspace --profile full --no-fail-fast`
  with explicit Nix LLVM 21.1.8 `LLVM_COV`/`LLVM_PROFDATA`; run
  `b0e2a993-ece3-4c75-a02f-e9381cd895bb`, 2,999/2,999 PASS, 33 skipped,
  924.772 s. The preceding run was 2,998/2,999 solely because
  `llvm-tools-preview` was absent; focused reprovisioned test run
  `04d4c469-0e1e-4d4f-a0df-368e2795fea5` passed before the clean full rerun.
  A second uninterrupted evidence-custody rerun completed with exit code 0;
  its raw transcript is `terminal-evidence/full-workspace.log`.
- Workspace all-target Clippy with `-D warnings`: PASS.
- Workspace doctests: PASS.
- `cargo deny check`: PASS (one informational unmatched MIT-0 allowance).
- Formatting and `git diff --check`: PASS.
- Authority anti-evasion: PASS.
- AUTH11 required-suite guards: 3/3 PASS, run
  `b48684b6-c97b-47fc-95f7-3a56f4211079`.
- Vegetation authority contract: 27/27 PASS.
- Vegetation crate: 259/259 PASS.

The package-required heavy runner failed twice before execution because its
fixed model was at capacity. `heavy-runner-fallback.md` records the authorized
direct local fallback; no gate was waived or narrowed.

## Seven Child-3 benchmark surfaces

All seven selected nonzero tests and passed on the corrected litter physics:

1. strict all-open projection: 1/1 PASS;
2. one open tile public endpoint: 1/1 PASS;
3. single-rank covered frozen oracle: 1/1 PASS;
4. multirank covered frozen oracle: 1/1 PASS;
5. mixed open/covered public endpoint: 1/1 PASS;
6. complete strict public envelope target: 10/10 PASS;
7. actual-byte rollback injection matrix: 1/1 PASS.

Raw exact-command transcripts for all seven surfaces are preserved under
`terminal-evidence/bench-*.log`; `terminal-evidence/README.md` maps each file
to its selector and result.

Independent exact-byte review verdicts are preserved in
`fresh-science-review.md` and `fresh-rust-review.md`.

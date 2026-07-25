# Tooling Defect 01: Read-Only Fixture Mode Propagation

Evidence class: Ran.

Attempt:
`/home/workdir/openWEPP-quality-history/20260724-order3-local-attempt-S67HdA`.

Admission:
`10c5d4ee8e3b370fc04b290c22e5d41b6484e27286e17a165563f54ba041c487`.

Observed failure: the recursively read-only execution snapshot preserved
read-only mode bits when tests copied fixture trees into their private
temporary roots. Valid fixture mutation then failed with `Permission denied`.
At termination, instrumented `full` had run 1,118/2,279 tests: 1,064 passed,
54 failed, 31 skipped, and 1,161 were not run after the second signal.

Representative failures:

- `hphys0208_sat_perturbation_changes_coupled_wb13_publications` could not
  rewrite its temporary soil fixture;
- `erosion_multi_ofe_p102_wave1_chain_routes_sediment` could not write its
  temporary variant soil.

Impact: attempt invalid; no coverage merge, CRAP report, or quality evidence ID
was produced.

Correction boundary: preserve writable mode bits on non-measurement fixture
files while removing write permission from every snapshot directory, Git
metadata file, production source, Rust test source, Cargo/config/tool input,
collector, registry, and historical ledger. This keeps source/control identity
immutable and lets copied fixture files remain writable in test-owned temp
roots.

Correction verification:

- snapshot root, source/control inputs, and Git metadata are read-only;
- fixture source files retain writable mode bits and copied fixtures remain
  writable in test-owned temporary roots;
- direct tracked/untracked byte identity detects fixture mutation without Git
  LFS clean filters or writes under frozen `.git`;
- the ignored `.venv` evaluator symlink target is explicitly identity-bound;
- a frozen-snapshot probe changed only that target, observed a changed identity,
  restored the target and original identity, and restored the read-only root;
- reviewer A: `PASS`;
- reviewer B: `PASS`;
- Python compilation, collector self-test, focused Nextest `5/5`,
  warnings-denied focused Clippy, and diff hygiene: `PASS`.

Retry policy disposition: correction accepted. One retry is authorized through
`transition` in a fresh durable attempt root.

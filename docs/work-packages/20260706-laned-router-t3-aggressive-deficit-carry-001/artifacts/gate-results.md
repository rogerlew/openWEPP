# LANED-T3-AGG — Gate results

Status: **EXECUTED** (2026-07-07; canonical-command reconciliation + review
fixes re-gated same day per QA-M1). Evidence mode: **Ran** — every gate below
was invoked this session; the command and verbatim result line are recorded
per gate. Gates marked (post-review) ran on the tree INCLUDING the
review-disposition fixes (C-M1 guard, C-L1 doc+test, QA-L1 comment, QA-M2
contract tables).

| Gate | Command | Result |
|---|---|---|
| Format | `cargo fmt --check` | **PASS** (post-review) |
| Lint (canonical form, QA-M1) | `cargo clippy --workspace --all-targets -- -D warnings` | **PASS** (post-review) — zero warnings under `-D warnings` (the pre-review run used the non-`-D` form and is superseded by this one) |
| Policy | `cargo deny check` | **PASS** — `advisories ok, bans ok, licenses ok, sources ok` |
| Full suite (canonical form, QA-M1) | `cargo nextest run --workspace --profile full` | **PASS** (post-review) — verbatim summary recorded below; supersedes the pre-review non-profile run (`1424/1424`) |
| Focused | `cargo test -p openwepp-hillslope-orchestrator --lib rev30` / `hybrid_rejects` | **PASS** (post-review) — 6/6 rev-30 vectors (incl. the C-L1 all-dry-drop pin) + 2/2 rejection vectors (incl. the C-M1 hour-partition pin) |
| Plain-path invariance | H2637 `OPENWEPP_LANED_ACTIVE=1` (rebuilt post-review binary, `-p openwepp-runner --bins`, mtime/hash verified) | **PASS** (post-review) — `H2637.pass.parquet` sha256 `21c54bf2…` identical to the rev-27/29 record |
| H2637 aggressive closure | full-year run, all four rev-27 day-closure hard-fails LIVE | **PASS** — exit 0 on 731 days / 610 routed (incl. former failure coordinates lane 17 day 54); supply `7.3e-16`, cascade `3.26e-13`, seam `1.65e-14`, identity `3.30e-13` (all ≤ 1e-9 rel); post-review rerun BIT-IDENTICAL (parquet `a5fb9233…`, outlet `371322.66377028974` — the C-M1 guard is pass-through at the production 900 s cadence) |
| Determinism | 3 aggressive runs (+1 post-review) | **PASS** — books identical to the last bit across all runs |
| Deficit-carry exercise | instrumented diagnostic run (temporary eprintln, reverted) | **PASS** — 6 carry events fired (`3.06e-9 … 1.23e-5 m²`), all absorbed; end-of-window fail-closed never triggered; books identical to the clean binary |
| Line-count governance (QA-L2) | `wc -l` vs `crates/AGENTS.md` thresholds | **PASS** — touched files below the 2000-line WARN: `cascade.rs` 1247, `kinematic_wave.rs` 1858 (approaching WARN — noted for the next split pass), `laned_active.rs` 892 |

Post-review canonical full-suite result (verbatim):

```
Summary [ 595.323s] 1426 tests run: 1426 passed (4 slow), 4 skipped
```

(1424 at execution + the two review vectors: the C-M1 hour-partition
rejection pin and the C-L1 all-dry sub-noise-drop pin.)

Not run / not claimed:

- **Timing acceptance**: no timing gate was named for this package (it is a
  defect-closure package); the recorded endpoint (`38.28/38.32/38.04 s` vs
  `37.9 s` plain) is EVIDENCE, and its honest disposition — prize not
  realized at current implicit-solve cost — is in `fix-evidence.md`.
- **Case-4 hybrid oracle ladder / fidelity-tolerance ratification**: parent
  package's OPEN gates (unchanged by this package; deliberately out of
  scope per the package Included/Excluded).
- **Synthetic solver-level terminal-deficit vector**: not achieved (scan
  evidence in `fix-evidence.md`); the seam is pinned on both sides by unit
  vectors and the real class is exercised by the instrumented H2637 run.

Build-provenance caution recorded for future workers: `cargo build
--release` at the workspace level does NOT relink `openwepp-cli-hill`
(non-default bin member) — the first evidence attempt silently ran a stale
rev-29 binary (caught because its books were bit-identical to the strict
record). Use `cargo build --release -p openwepp-runner --bins` for evidence
binaries and verify the binary mtime/hash before timing.

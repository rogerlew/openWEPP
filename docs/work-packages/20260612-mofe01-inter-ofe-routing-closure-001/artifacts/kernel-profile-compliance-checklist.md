# kernel profile compliance checklist

Status: checked through M-E3

Evidence mode: Ran + Static

## M-E3 checklist

- Production edits: yes; persistent OFE lane state model, multi-OFE runner
  daily lifecycle wiring, and manifest dynamic-state policy tokens.
- Science-contract edits: none.
- Test edits: yes; focused M-E3 orchestrator writeback tests.
- Typed errors in production: extended `OfeLaneSequenceError` for persistent
  lane count/order replacement mismatches; runner maps persistent lifecycle
  failures to typed CLI runtime-surface failures.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: aggregate WB13/WAT publication preserved; dynamic
  per-OFE state is shadow persistence only.

Validation:

- `cargo fmt --check`: PASS.
- `cargo test -p openwepp-hillslope-orchestrator mofe01_me3 -- --nocapture`:
  PASS.
- `cargo test -p openwepp-runner mofe01 -- --nocapture`: PASS.
- `cargo test --test mofe01_per_ofe_state_contract -- --nocapture`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- Work-package docs lint: PASS.
- Required H1/H6/H9/H11 runtime smoke: PASS.
- Single-OFE anchor comparison: PASS.
- Local owcmp H1/H6/H9/H11 command execution: PASS; expected semantic FAIL
  remains at the unchanged aggregate-publication boundary.

## M-E2 checklist

- Production edits: yes; orchestrator scheduler sequential OFE lane executor
  and public exports.
- Science-contract edits: none.
- Test edits: yes; focused M-E2 orchestrator writeback tests.
- Typed errors in production: added `OfeLaneSequenceError` for invalid lane
  count/order, invalid transfer values, daily sum mismatch, scheduler failure,
  and lane execution failure.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: unchanged; aggregate WB13/WAT publication
  preserved.

Validation:

- `cargo fmt --check`: PASS.
- `cargo test -p openwepp-hillslope-orchestrator mofe01_me2 -- --nocapture`:
  PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.
- Final H1-H36 replay/comparison: PASS runtime execution; expected `owcmp`
  semantic FAIL remains at the unchanged aggregate-publication boundary.

## M-E1 checklist

- Production edits: yes; scheduler data-model types, runner static slices, and
  publication manifest provenance.
- Science-contract edits: none.
- Test edits: yes; focused M-E1 runner tests and stale WATBAL version-pin
  repairs in two HPHYS authority tests.
- Typed errors in production: added `PerOfeDailyWaterBalanceError` variants for
  invalid record and transfer source/recipient mismatches.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: aggregate WB13/WAT publication preserved; no
  dynamic per-OFE WAT publication flip.

Validation:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS.
- `bash tools/release/check_authority_suite_antievasion.sh`: PASS.

## M-E0 checklist

- Production edits: none.
- Science-contract edits: yes; `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
  `SC-SYSTEM-001` amended for per-OFE dynamic-state authority.
- Test edits: yes; `mofe01_per_ofe_state_contract` added and M-B authority
  smoke test date brittleness fixed.
- Typed errors in production: unchanged.
- `unwrap`/`expect` in production: none introduced. The new panic helper is
  test-only.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged; M-E0 only installs the contract
  and red-test boundary for M-E1.

Validation:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo deny check`: PASS.
- M-E0 authority test: PASS.
- M-B authority smoke test: PASS.
- Full M-E0 target: FAIL by design on missing per-OFE state collection,
  transfer payloads, and publication-policy manifest gate.
- Full Rust closure loop: BLOCKED until M-E1 satisfies the intentional red
  test.

## M-D checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged in code; M-D only declares the
  architecture needed before M-E production edits.

Validation:
- Full Rust closure loop was not rerun for M-D because no production Rust,
  science-contract, dependency, or test files were edited.
- Package docs lint is recorded in `gate-results.md`.

## M-C2 checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime state/publication paths: unchanged because current architecture has
  no real per-OFE daily WB state surface to retain or publish.

Validation:
- Full Rust closure loop was not rerun for M-C2 because no production Rust,
  science-contract, dependency, or test files were edited.
- Focused existing M-B carry tests passed.
- M-C2 output comparison and publication audit were run separately and are
  recorded in `m-c2-per-ofe-daily-state-scope-evidence.md`.

## M-C checklist

- Production edits: none.
- Typed errors: unchanged.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: none introduced.
- Kernel math: unchanged.
- Runtime publication paths: unchanged because current aggregate-only WB13/WAT
  surface cannot support real per-OFE publication semantics.

Validation:
- Full Rust closure loop was not rerun for M-C because no production Rust,
  contract, test, or dependency files were edited.
- M-C output comparison and publication audit were run separately and are
  recorded in `m-c-wat-publication-closure-evidence.md`.

## M-B checklist

- Typed errors: preserved. Missing active frost topology and invalid aggregate-vs-array carry remain typed failures.
- `unwrap`/`expect` in production: none introduced.
- Unsafe: none introduced.
- Bounded canonicalization: top-layer saturation excess is relocated to the explicit current saturation carry under M-B contract authority; no silent default or mask was added.
- Kernel math: limited to contract-pinned inter-OFE routing/carry plumbing and saturation excess conservation.
- Runtime carry paths: updated to purge stale aggregate carry before MOFE
  hourly-array execution and preserve separated `UpStrmQ`/`SubRIn` lineage in
  runtime state. WAT publication remains aggregate-only and is held in M-C.

Validation:
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.

## M-A

M-A made no production kernel/runtime edits.

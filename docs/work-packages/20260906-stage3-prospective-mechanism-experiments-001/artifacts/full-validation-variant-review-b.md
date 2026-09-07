# Full validation build-variant QA review B

Static: authority/configuration/source inspection only, 2026-09-06. No Cargo
build, list, test or measurement executed by this reviewer.

## Findings and decision

No authority blocker to executing `cargo nextest run --release --workspace
--profile full` with `CARGO_PROFILE_RELEASE_LTO=false`, a recorded 64 MiB stack,
unchanged canonical full filters/groups, and no external total-run timeout.
This is full-profile optimized-build correctness evidence, not debug-full
evidence or normal thin-LTO release qualification. Completion/results remain
unestablished until the entire required inventory executes.

- `testing-and-gate-strategy.md:242` requires immediate campaign-strength full
  correctness; lines270–290 prohibit narrower coverage and require explained
  command selection. Neither provision requires the default-debug Cargo build.
- `local-ci-gate-selection.md:20` labels its argv **Typical commands**;
  line27 names full, and lines128–129 forbid replacement by quick/domain.
  Cargo `--release` does not change the selected Nextest `full` profile.
- Kickoff line226, package line37 and the experiment protocol require full
  correctness strength plus all affected science/mechanism gates, not a
  specifically unoptimized build. Speed alone would not override an explicit
  build-mode requirement if another applicable obligation imposed one.

## Required evidence limits

Record the prospective command-plan revision and exact source/dirty identity,
toolchain, Cargo/Nextest configurations, argv, environment, stack override,
test inventory, excluded/ignored inventory, start/end and final report.
Confirm build-mode conditionals do not remove any required test. Preserve full
test groups, retries and existing per-test timeout: `.config/nextest.toml:120`
still sets 90 seconds × 8. No external timeout does not mean tests are unlimited.

Default release differs from debug in debug assertions and integer-overflow
checks. Source inspection finds production `debug_assert*` in LSE exact-dyadic
arithmetic, vegetation occupancy, kernel symbol indexing and runner publication/
execution. Execute default-debug owning-crate/focused checks for the affected
invariants and record their actual scope. They must not be described as a
default-debug full-workspace pass. Explicit test assertions still execute in
release tests; optimization is not a justification for ignoring a failure.

The repository release profile normally uses thin LTO (`Cargo.toml:909`);
the proposed false override is a distinct build identity. A 64 MiB stack also
limits transfer to a default-stack execution. Neither variant establishes
timing/memory comparability or qualifies a different measured binary.

Keep the interrupted/malformed-environment debug attempt as its actual
incomplete/invalid execution, not a pass superseded in place. A complete
optimized run supplies new evidence for the named full-correctness obligation.
All explicit authority, ignored mechanism/oracle, compile-negative custody,
restart/output/closure, format, warnings-denied lint and other applicable gates
remain separate and mandatory; full profile has existing manual-family
exclusions and does not execute ignored tests automatically.

QA: GO for this prospective full-profile build variant with these recorded
limits. No scientific admission, full-suite pass or release qualification is
claimed by this review.

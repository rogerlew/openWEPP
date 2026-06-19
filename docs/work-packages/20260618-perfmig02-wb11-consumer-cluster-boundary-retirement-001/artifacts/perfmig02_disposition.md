# PERFMIG02 Disposition

Static: synthesized code review, gate table, timing, identity, and package acceptance criteria.

Ran: focused tests, H2637 endpoint, and transition-boundary bench.

## Verdict

`EXECUTED-REDIRECT: final-code endpoint flat/negative and strict apply-boundary attribution failed`.

PERFMIG02 should not be recorded as `CONTINUE` because the package's non-negotiable acceptance included a
measured endpoint improvement and a measured `apply_indexed` cost drop for the retired materialization
boundary. Both failed on the final clippy-clean code.

- materialize-all apply: `104.752336 us/payload`
- PERFMIG02 skip-six apply: `105.460510 us/payload`
- delta: `+0.708174 us/payload`, or about `+0.167101 s` projected over H2637 OFE-days

The endpoint did not improve:

- PERFMIG01: `669.97 s`, `228144 KB`
- PERFMIG02 final run 1: `672.14 s`, `227636 KB`
- PERFMIG02 final run 2: `675.00 s`, `228152 KB`
- delta: `+2.17 s` to `+5.03 s` (`+0.32%` to `+0.75%`)

Identity is preserved on the exercised surfaces:

- HBP byte-identical;
- WAT byte-identical;
- PASS Arrow-equal with metadata ignored;
- PERFMIG01-final and PERFMIG02 manifests agree on HBP/loss/plot/WAT checksums.

## What This Means

The widen-and-retire strategy did not convert in this rung. The conservative six-symbol logical
materialization retirement does not lower apply cost once stale logical removal is included, and the final
endpoint is flat/negative. This is the package's REDIRECT signal.

## Next Action

Do not run another writeback-only rung. The next perf package should pivot to a deep single-phase
array-native migration for WB11/WB12 that captures dense read + compute + write together, per the PERFARCH03
floor.

The current final-code ratio is `73.70x` to `74.01x` versus the `9.12 s` legacy no-UI anchor.

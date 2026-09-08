# Independent baseline oracle results

Ran: 2026-09-08 PDT in detached A-source-05 worktree
`/tmp/openwepp-residual-jacobian-FC2T1v/J`; no candidate derivative existed.

- Selector self-tests: PASS 1/1 (cubic basin, constant/roundoff rejection,
  kink rejection, deliberately wrong derivative rejection).
- Authentic covered Potential+FixedFinal transaction/oracle: PASS 1/1.
- Active-stem corpus: `/tmp/openwepp-rj-active-stem-corpus-v1.json`, SHA-256
  `a05ca09031c28f863bc8dd8f3cc8d605459a683abc2af6e0f37dd84c713068c0`,
  509,132 bytes, 8 records (4 Potential, 4 FixedFinal), N=2/S=2/D=25,
  4,636 exact tagged binary64 values; both dry-stem areas positive throughout.
- Full native encode/decode equality and base replay: PASS.
- Exact-zero undeclared-row mask and elementary sensible/longwave signs: PASS
  for the inspected probes.
- Frozen v33 oracle admission: **0/16 active dry-stem columns**.

The first failed affected entry was row 9/column 9. Its nine centered estimates
were `[-8326586.9648, -8326586.9241, -8326586.5108, -8326586.5075,
-8326586.6175, -8326586.6150, -8326586.5896, -8326586.5896,
-8326586.5964]`; no consecutive triple met the frozen `[2.5,5.5]` second-order
ratio. Each of the 16 authentic columns lacked at least one mandatory affected-row
basin and was frozen unsupported before J. The protocol and tolerances were not
changed after observation.

Commands:

```text
nix develop --offline -c cargo test -p openwepp-land-surface-energy solver_stem_jacobian_tests::v33_oracle_selector_self_tests_precede_candidate_code -- --exact --nocapture
OPENWEPP_RESIDUAL_CORPUS_PATH=/tmp/openwepp-rj-active-stem-corpus-v1.json nix develop --offline -c cargo test -p openwepp-land-surface-energy solver::tests::covered_v8_block_matches_frozen_joint_solution -- --exact --nocapture
```

This is an oracle-resolution boundary, not evidence that an analytic derivative
is wrong. Phase A did not admit a column, so candidate implementation and all
downstream comparisons are ineligible.

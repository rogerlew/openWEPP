# Ratification Evidence

Status: EXECUTED-HOLD-CASE4-HYBRID-LADDER

## Case-4 Hybrid Oracle Ladder

Ran:

```
cargo nextest run -p openwepp-hillslope-orchestrator ofe_routing --profile quick
```

First execution with the new retained ratification vector active failed:

```
FAIL openwepp-hillslope-orchestrator
  ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle

hybrid peak error at ladder step 0: 0.2278616770287263
(ref 0.008334954309852328); ladder
[0.2278616770287263, 0.1546442906262445, 0.1020328027401588]
```

Interpretation:
- Required peak tolerance from `SC-OFEROUTE-001` rev 24/26 Case-4 acceptance:
  `<= 0.05` relative error at every rung.
- Observed hybrid errors: `22.8%`, `15.5%`, `10.2%`.
- The ladder improves with refinement, but every rung fails the named tolerance.

Disposition:
- Ratification gate FAIL.
- The vector is retained as
  `case4_hybrid_manning_ladder_meets_iwagaki_oracle` and marked ignored with a
  reason so normal CI remains green while the ratification hold is explicit.
- Current reproducibility evidence is the ignored-only command:

  ```
  cargo nextest run -p openwepp-hillslope-orchestrator 'ofe_routing::d10b_reconciliation_tests::case4_hybrid_manning_ladder_meets_iwagaki_oracle' --profile quick --run-ignored ignored-only
  ```

  This failed 1/1 in `150.896 s` with the same ladder; see
  `artifacts/case4-hybrid-ignored-ratification.log`.
- No hybrid selector promotion is authorized by this package.

## Fidelity-Tolerance Adjudication

Status: BLOCKED by Case-4 hybrid ladder failure.

The package cannot ratify H2637 fidelity tolerances while the required Case-4
hybrid oracle ladder fails the current contract tolerance. Existing evidence
remains diagnostic only:
- Parent I1 pure-recession per-bin L1 at `dt=900`: about `0.40-0.43`.
- T3-AGG H2637 aggressive outlet delta vs plain active: `-0.84%`.
- T3 strict H2637 outlet delta vs plain active: about `-0.24%` from the parent
  records (`373,581.06` vs `374,463.08 m3`).

## H2637 Active Hybrid Timing

Status: PASS as timing/solve-cost evidence; not sufficient for ratification.

The H2637 timing/profile run is still useful as solve-cost evidence, but it
cannot lift the Case-4 ratification hold by itself.

Ran by `comparator_suite_runner`; full evidence in
`artifacts/verification-h2637-timing.md`.

Summary:
- Build: `cargo build --release -p openwepp-runner --bins`, exit `0`.
- Binary: `target/release/openwepp-cli-hill`, sha256
  `6e7a1c56ef9b74b6f37a790c98be5f2bfc9119fa7fc40027d953c9e05ae7ae9e`.
- Run: `OPENWEPP_LANED_ACTIVE=1 OPENWEPP_LANED_ACTIVE_IMPLICIT=1
  OPENWEPP_LANED_SHADOW_PROFILE=1 /usr/bin/time -v taskset -c 4
  target/release/openwepp-cli-hill --run-dir <scratch> --run-file
  p2637.run.toml --output-dir <scratch>/output`
- Exit: `0`.
- User/sys/wall: `36.61 s` / `0.02 s` / `0:36.65`.
- `laned_active_profile`:
  - `solver_steps=5806728`
  - `solver_steps_implicit=1146432`
  - `implicit_equilibrium_map_evaluations=274681460`
  - `implicit_branch_evaluations=37241376`
  - `alpha_evaluations=88421812`
  - `solver_cfl_ns=14859981755`
  - `solver_step_ns=5473027693`

Comparison to T3-AGG:
- Rev-30 aggressive endpoint: `38.28 / 38.32 / 38.04 s` user.
- Rev-31 warm-seed/counter endpoint: `36.61 s` user.
- Outcome: modest positive timing evidence (~3.8-4.5% faster than the prior
  aggressive runs), while showing the implicit solve remains expensive
  (`274.7M` map evaluations).

# Terminal Verification A

Status: complete

Evidence mode: Static + Ran

Verdict: **PASS-WITH-NOTES**

## Conclusion

No scientific, Rust, fixture-custody, materiality, conservation, or validation
blocker was found. The 21K correction satisfies Verifier A's acceptance scope:
canonical v125 authority defines the exact operand, production carries one
private value to the real density consumer, bulk and multilayer behavior and
the offline replay are proven, receipt-bound materiality is finite and within
the contract tolerances, protected fixture and output boundaries are intact,
accepted review findings are dispositioned, and the terminal gate receipts
reconcile.

Two closure-prose notes remain:

1. The closure-ready base-to-worktree union is still exactly 86 authorized
   paths, but after prompt archival its current split is 77 tracked paths plus
   nine untracked paths, not the 78-plus-eight pre-archival split stated in
   `exact-diff-reconciliation.md`. The difference is the byte-identical move
   from the scaffolded tracked active prompt to the untracked archived path.
   This is a nonblocking accounting wording issue; no path is outside the
   declared write set.
2. This artifact establishes only Terminal Verification A. At this snapshot,
   `verification_agent_b.md` remains queued, so package/catalog statements that
   both terminal verifiers pass become truthful only after independent
   Verification B completes with a passing verdict. This is a lifecycle
   prerequisite, not an implementation defect.

## Canonical Authority And Exact Production Handoff

- `SC-SNOWFREEZE-001.md` has `contract_version: 125` and binds
  `INV-SNOWFREEZE-092`, `OBL-SNOWFREEZE-P-065`, and
  `TOL-SNOWFREEZE-017`. The accepted daily value is
  `sum(max(hourly melt_raw_m, 0)) + rain_retained + rain_released`, in metres
  water equivalent, evaluated before runoff and counted once. The contract
  explicitly rejects the retired state-loss-plus-routed, routed-only,
  state-loss-plus-rain, raw-rain, signed-daily-melt, and retained-store aliases.
- `SnowCouplingOutcome::wet_compaction_liquid_input_m` is private. Active
  finalization computes it from positive hourly applied/capped CoE melt and the
  two snow-contact-rain components before signed-melt redistribution and
  released-rain routing. The existing typed boundary rejects a non-finite or
  negative result. Inactive coupling supplies exactly zero.
- `resolve_typed_snow_density_outcome` passes only
  `snow_coupling.wet_compaction_liquid_input_m` to
  `SnowDensityRuntimeInputs::liquid_for_compaction_m`. Repository-wide searches
  found no active Rust consumer of the retired duplicate expression.
- The unchanged density implementation converts that one scalar to
  `kg m^-2` once for the bulk path and once for the selected multilayer path;
  the multilayer implementation partitions that total among layers. The
  contract integration target exercises real bulk and multilayer outcomes and
  reconstructs the diagnostic input independently from hourly applied melt and
  contact rain.
- The offline CoE boundary emits
  `gross_positive_generated_melt_m` from verbose hourly
  `coe_melt_applied_m`. Its density replay requires finite, nonnegative gross
  melt, retained-rain, and released-rain columns and uses only their sum for
  compaction. State loss and routed melt remain diagnostic columns; changing
  them does not change replay density, while changing the authoritative source
  does.

## Independent Materiality And Custody Check

The retained identities match the package records:

- execution receipt SHA-256:
  `1cd4aa5fb2110eb0445f57de846e2b65b224e7b0704e00a9d6cff1e3d4ca220a`;
- materiality result SHA-256:
  `25c8150f95d1be81afa7597d93dc271f8df5d82e062c558b231dd1695afab05a`;
- release binary SHA-256:
  `1934000cd3c2534350af7ab1678325906762798e94dbe245b3895b910bf1382a`;
- materiality tool SHA-256:
  `e1ab0dbcb179f037a252e0ed502e05c3c8b8b439bcc598d3d190e37acc0e4020`;
- terminal command log/result SHA-256:
  `123291a0e067186a6f8278e67bb83831a1c7a702fb540f26871cdb59775d2a9f`
  and
  `1bf9e174dd777e811e3e1999d19355bd704c891aa2d650b4565aba76766ccb0e`.

All five current trace files independently hash to the values in the execution
receipt. The receipt labels four lanes `CANONICAL` and the scaled Snowbird lane
`DEVELOPMENT_ONLY`, with all lane return codes zero. The result reports
`PASS` under these fail-closed limits:

| Acceptance quantity | Observed maximum/count | Limit |
| --- | ---: | ---: |
| Operand reconstruction | `8.353e-17 m` | `1e-12 m` |
| Upstream mass delta | `2.443e-15 m` | `1e-9 m` |
| Stage-3 incoming-liquid closure | `3e-17 m` | `1e-9 m` |
| Density-process closure | `2.274e-13 kg m^-3` | `1e-9 kg m^-3` |
| Layer SWE/depth closure | `4.441e-16 / 8.882e-16 m` | `1e-9 m` |
| Canonical changed driver/density days | `24,046 / 22,392` | at least `1 / 1` |

The separately reported `0.002363 m` Stage-3 disposition delta is correctly
classified as a density-mediated response, not used to waive upstream mass
invariance. The materiality tool applies acceptance before atomic publication
and rechecks workspace, tool, binary, receipt, and trace identities. The
reviews' requested executable negative-injection test remains explicitly
deferred as nonblocking hardening debt; the current source-marker regression,
fail-closed implementation, and fresh accepted receipt are all present.

Canonical Snowbird SHA-256 remains
`10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7`.
The precipitation-only derivative is
`c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb`.
The materializer check and the 14,245-row Rust custody test prove exact decimal
factor `1.2155576`, `0.1 mm` half-up rounding, and unchanged non-precipitation
tokens. Canonical `p8.cli`, `09_snow_density.rs`, and `Cargo.lock` have no
base-to-worktree diff.

## Reviews, Gates, Scope, And Handoff

- Both scientific/Rust reviews are final `GO`. Review B's initial quick-profile
  timeout HOLD is retained, then resolved by the exact-source `-j 2` quick run.
  Every accepted or deferred finding appears in `review-disposition.md`; none
  is undispositioned.
- The structured terminal receipt contains 18 commands, all with status
  `PASS` and exit zero. It records focused `8/8`, offline `2/2`, helper `1/1`,
  quick `2181/2181`, frost `358/358`, Critical full `2270/2270`, doctest,
  format, warnings-denied Clippy, dependency policy, assurance, fixture,
  anti-evasion, and AUTH11 results. The failed high-contention quick attempt is
  preserved separately and is not mislabeled as a pass.
- Current line counts reproduce as `927`, `2579`, `2723`, `969`, `1209`, and
  `428` for the five touched Rust modules and new integration target. The two
  reconciliation modules are correctly in the warning band; none reaches the
  mandatory 3,000-line threshold.
- The exact current union is 86 paths and remains inside the amended package
  envelope. Protected runtime schemas, observations, phase/energy/radiation/
  canopy/frost surfaces, density constants/cap/defaults, canonical climate,
  `Cargo.lock`, and the density algorithm file remain untouched.
- The archived kickoff prompt is byte-identical to the scaffolded active
  prompt at SHA-256
  `a863c62df3b18bb82a7de9d5a38ecf4364d1cfbfb2ae591bbfb9480fa9f1f69e`;
  no active prompt remains.
- 21L admission is truthful only as corrected-state rebaselining and
  attribution work. The roadmap and handoff require canonical lanes for
  acceptance, restrict scaled Snowbird to input sensitivity, preserve forcing,
  snowfall, storage, loss, and energy operands separately, and authorize no
  tuning or early-melt correction from 21K.

## Fresh Bounded Verification Runs

Ran from `/home/workdir/openWEPP` against the closure-ready source:

```text
cargo nextest run --test snow_wet_compaction_operand_authority --no-fail-fast
  PASS 8/8; run e1012b67-5a6a-4353-99a8-52df28ed67fe
cargo nextest run -p openwepp-runner -E 'test(boundary_requires_finite_nonnegative_authoritative_source_columns) | test(replay_uses_generated_melt_and_contact_rain_once)'
  PASS 2/2; run 3c56da80-a73a-4bc2-8a19-cc1cce02a0e3
cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(helper_sums_positive_melt_and_contact_rain_and_fails_closed)'
  PASS 1/1; run f8efaae0-b553-4844-a9a6-e9cc8a457a2b
cargo nextest run --test auth11_required_suite_obligation_guards_contract
  PASS 3/3; run 1fdb3439-c0f3-4119-bec6-33a2f6dd8b4f
cargo fmt --all -- --check
git diff --check
target/release/openwepp-assurance validate --all
.venv/bin/python docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/materialize_snowbird_development_cli.py --check
bash tools/release/check_authority_suite_antievasion.sh
markdown-doc lint --path docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001 --format plain
markdown-doc validate --path docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001 --format plain
```

All commands exited zero. Markdown lint covered 28 files with zero errors and
zero warnings; schema validation covered the same 28 files with zero errors.

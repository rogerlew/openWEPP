# Component-temperature dependency-replay contract cycle

Evidence mode: `Static + Ran + Expected-red`

Base commit under review: `a28c55c2d0f06e0c4aab58642f1009f70f82b3d3`.
The rejected production candidate reviewed in `implementation_review_a.md` and
`implementation_review_b.md` has been fully reverted. This corrected,
uncommitted contract-first increment amends
`SC-LANDSURFACEENERGY-001` to revision 31, adds
`INV-LANDSURFACEENERGY-164` and `OBL-LANDSURFACEENERGY-C-020`, and changes no
production Rust, numerical solver, process equation, tolerance, output, wire
format, or package authority.

## Bound candidate

Revision 31 permits exact dependency replay only for component-temperature
probes within one validated represented-snow Jacobian sweep. The corrected
contract fixes stable node IDs, complete direct-edge families, independently
enumerated edge/hash vectors, inclusive transitive closure, conservative
unknown-edge ineligibility, and exact source order. It expressly includes both source-
distinct wet evaluations/finalizations, upper-to-every-lower routing,
reciprocal longwave, the routed/final mismatch guard, terminal ground release
and stemflow, leaf/CI/hydraulic descendants, shared rows, normalization, and
complete result assembly. Complete and replay evaluation must consume one
shared canonical node/tail implementation; duplicated physics arithmetic is
not authorized.

The CTDR-RRB-003 direct-read correction makes every previously implicit
source-real direct edge explicit: wet probes directly reach first routing
finalization; routing preparation reaches the second wet evaluation; the
longwave column reaches every occupancy output; ground release and stemflow
reach final output; lower/ground output reaches all three shared nodes; and
shared heat/vapor each reach shared tolerance. The focused authority assertion
binds those relationships independently of transitive reachability.

One immutable sweep base is bound exactly to generation, all validated inputs,
caps/frozen values, current trial, authentic map/solve/iteration/sweep and graph
identity. Every signed probe gets a fresh consumed-on-success-or-error
capability bound to coordinate, sign, perturbation, exact probe and actual
stencil. Typed identities/borrows or compact seals minted at validation provide
exact custody; `Debug` strings, length-only digests, whole-probe cloning and
repeated hot-path whole-input scans are not authority. Ordinary ineligibility
selects the canonical complete evaluator before replay; private integrity
mismatch fails typed; a post-start node error returns directly without
complete-evaluator fallback.

The corrected normative fallibility/crossability matrix requires source-real
paired replay-versus-forced-complete first-error and byte-rollback evidence for
every canonically crossable typed-error surface, currently
`occ.leaf.current`. `occ.leaf.maximum` remains fallible but noncrossable unless
an authentic successful-base/admitted-probe counterexample is established.
Other fallible-but-noncrossable families require a guard-by-guard successful-
base implication proof plus authentic boundary/branch successes and exact
fields. Infallible nodes require exact source-order field parity, never invented
errors. A differential corpus catches any naturally occurring error; mutation
or fault-injection hooks are forbidden.

For a full interior centered sweep with two occupancies and six soil nodes, the
required per-sweep accounting is exactly 58 ordered logical probes: 14 existing
ground/soil identity-anchor syntheses, 16 component dependency replays, and 28
complete evaluations. The graph is defined for arbitrary validated occupancy
and soil cardinality; the common fixture cannot be encoded as production
topology logic. Separately scoped run aggregation records every potential/final,
centered/inward and completed/failed source-real sweep and never treats the
single fixture identity as a release-run total. A short-circuit class exists
only if the unchanged solver has a real non-error early-ending sweep path; an
impossible always-zero class is forbidden. Map, solve, iteration and sweep
identities come from their distinct real lifecycle positions and are never
copied proxies.

Analytic/automatic derivatives, graph coloring, simultaneous perturbation,
sparse Jacobian/LU, changed pivoting, cross-sweep/iteration/map/retry caches,
approximation, fallback, and hardcoded two-occupancy/six-soil behavior remain
forbidden. The forced-complete oracle, complete direct-edge oracle, corrected
fallibility/crossability matrix, full-solve bit equality, rollback, authentic
probe counters, and release keep/revert rule are mandatory implementation gates.

## Reproducible contract-first evidence

Ran the active structural-parser fixture:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract rust_structural_item_parser
```

Result: `PASS`, 2/2. Disabled/test-only decoys, all seven nested required-item
decoys, and the unconditional positive control behaved as required. The lexer
fixtures include `'}'`, `'\''`, `b'}'`, and `b'\''`, while named, static and
placeholder lifetimes remain unmasked. A dispatcher fixture deliberately
contains a cfg-only call, same-named local, bare reference, and `if false` call
to demonstrate that token occurrence is not connectivity evidence.

Ran the focused authority assertion:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo nextest run --test land_surface_energy_balance_authority_contract version_thirty_one_binds_component_temperature_dependency_replay
```

Result: `PASS`, 1/1.

Ran the structural production-seam assertion:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo test --test land_surface_energy_balance_authority_contract revision_31_component_temperature_dependency_replay_structural_seam_is_expected_red -- --exact --nocapture
```

Result after correction: `EXPECTED_RED`. The hardened test masks comments,
cooked strings, arbitrary-hash raw/raw-byte strings (including embedded
quotes), and escaped/unescaped character and byte-character literals without
masking lifetimes. It parses real Rust item bodies, walks arbitrarily stacked and
multiline attributes, and rejects every top-level item gated by `cfg` or
`cfg_attr`, including visibility-qualified `cfg(any())` and
`cfg(any(test, feature = ...))` decoys. An active parser self-test proves those
and literal/nested decoys cannot satisfy the seam. Its absent production set is:
`CoveredComponentTemperatureDependencyGraph`,
`ValidatedCoveredComponentReplaySweepBase`,
`ValidatedCoveredComponentProbeReplay`,
`CoveredComponentDependencyReplayAudit`,
`covered_component_temperature_probe_residuals`,
`begin_covered_component_dependency_replay_audit`, and
`take_covered_component_dependency_replay_audit`. This source assertion proves
only unconditional top-level item presence/absence. It cannot prove dispatcher
invocation, control-flow reachability, graph/evidence consumption, counter
provenance, or numerical behavior. Empty, skeleton, token-only, dead-code, or
disconnected declarations could turn it green and remain insufficient for
implementation readiness. Only the executable real-dispatcher counters and
forced-complete node/residual/Jacobian/full-solve oracle can close those claims.

Ran the complete integration target:

```text
env RUST_MIN_STACK=67108864 nix develop -c cargo test --test land_surface_energy_balance_authority_contract -- --nocapture
```

Result: `PASS_WITH_EXPECTED_RED`: 24 passed, exactly the named revision-31
item-presence seam failed, and zero were ignored. No stale current-revision pin or
unrelated assertion failed. The build emitted one unrelated pre-existing
dead-code warning in the orchestrator terminal request.

Ran strict contract gates:

```text
.venv/bin/python tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md
nix develop -c cargo fmt --all -- --check
```

Result: `PASS`; 15 Binding Exposure Index rows are fully consolidated, unit
compliance reports no findings, and formatting is clean. Scoped
`git diff --check` also passes.

## Frozen baseline and prospective retention

Exact baseline/candidate command:

```text
timeout 1800 taskset -c 0 env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile -- --ignored --exact --nocapture --test-threads=1
```

Frozen baseline evidence:
`artifacts/terminal-heavy-gates/component_dependency_replay_baseline_3run.log`.
The pre/post-run Rust source manifest is
`78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`.
All runs used binary
`/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/release/deps/openwepp_runner-fc552493dc3c6cc2`,
SHA-256 `9a91c82f1799382014c3a561e79130b5f5b665bef0667a4bdff613c91d8e573f`.
The `STAGE3_LANED_RELEASE_PROBE` JSON fields
`(run_wall_us, physical_phase_wall_us.potential, rss_kib)` were
`(4926758,354838,70696)`, `(4903570,353374,54624)`, and
`(4896095,353431,59364)`. Sorted-middle medians are total `4903570 us` and
potential `353431 us`; candidate ceilings are respectively `4803570 us` and
`253431 us`.

Candidate qualification builds once, proves one unchanged source manifest and
binary hash across three identical CPU-0 commands, and uses the same JSON fields
and sorted-middle median. Every run retains exact science and `48/56/20/32/4`,
complete per-sweep/run aggregation with a qualifying authentic
`58/14/16/28` sweep, full-solve bits, and `rss_kib <= 65536`. Any failed
conjunct fully reverts v31 production.

## Executed candidate and mandatory reversion

Ran 2026-09-04: a fresh graph-driven implementation closed every accepted
implementation-review finding through seven immutable correction cuts. The
final reviewed 16-path implementation manifest was
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`.
Both independent implementation reviewers returned `APPROVE`; focused replay
passed `14/14`, full LSE passed `154/154`, feature-enabled compiler-negative
evidence passed `2/2` with the intended ownership/lifetime diagnostics, the
authentic replay-versus-forced runner and complete HBP/WAT/PASS byte oracle
passed, and all applicable checks, Clippy, formatting, diff, and line-count
gates passed.

The binding exact CPU-0 release command then failed closed on candidate run 1.
The source manifest stayed
`039a312502a5e6ef442b1e81ac78b988141199f6283fedcc86518ba78ba61abc`;
the executed binary was `openwepp_runner-ce7ba1c0f7527921`, SHA-256
`f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`.
The real release aggregation contained no authentic completed `N=2,S=6`
`58/14/16/28` sweep, so the required assertion failed before a
`STAGE3_LANED_RELEASE_PROBE` record was emitted. Run 1 exited `101`; runs 2 and
3 were not run because one failed conjunct mandates immediate full reversion.
Raw evidence is
`artifacts/terminal-heavy-gates/component_dependency_replay_candidate_3run.log`.

All v31 production, runner, test-support, and extracted test files were then
fully reverted. The structural seam is again expected red on exactly the seven
absent declarations, LSE is again `140/140`, LSE/orchestrator/runner all-target
checks pass, and residue, formatting, and diff checks pass. The current
modified/untracked Rust manifest is
`2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`.
It differs from the frozen numeric baseline only because the authority test
received 23 authorized contract/parser patches after that baseline. Reversing
those patches in memory reproduces `78d756...bbbe` exactly; production authority
was not reverted. Revision 31 remains unimplemented and its release-retention
result is `FAIL_REVERTED`.

Final independent Verifiers A and B audited the raw run structure and binary,
the complete revert, exact seven-symbol expected red, post-revert LSE `140/140`,
179-path Rust manifest `2813f6e8...ee0d`, frozen-manifest forensic
reconstruction, four sequencing defects, package status consistency, and HOLD
boundary. Both approve terminal `HOLD` with no remaining findings. Neither
approval changes `FAIL_REVERTED` into production retention or qualification.

## Ordered manifest

The handoff manifest covers, in this exact order:

1. `docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md`
2. `docs/specifications/science-contracts/index.md`
3. `tests/integration/land_surface_energy_balance_authority_contract.rs`
4. `tests/integration/snow_stage3_shared_carrier_authority_contract.rs`
5. `tests/integration/stage3_native_vegetation_laned_throughput_recovery.rs`
6. `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/component-temperature-dependency-replay/contract_ref.md`
7. `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/component-temperature-dependency-replay/readiness-matrix.md`
8. `docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001/artifacts/science-contracts/component-temperature-dependency-replay/disposition.md`

Recipe:

```text
sha256sum <the eight paths above in the listed order> | sha256sum
```

The exact digest is reported in the task handoff rather than embedded here,
because this file is itself a member of the hashed manifest.

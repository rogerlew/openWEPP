# Component-temperature dependency-replay finding disposition

Evidence mode: `Static + Ran`

Contract-review inputs: `review_agent_a.md`, `review_agent_b.md` against
five-file manifest
`4faf1cdd6ebd618d12a0a23501cf4f9df85d536f61b4dad74dd70ec4e452ed13`.
Implementation-review inputs: `implementation_review_a.md` and
`implementation_review_b.md` against the later eight-file authority manifest
`23d90c29038eb836b1ce8f2105588f44d382f2ae5b726a7bc55467b842ac058b` and
the captured pre-revert production candidate. Every finding is accepted. The
earlier contract-review findings below are closed by authority/test corrections;
every implementation-review finding remains open until a fresh implementation
supplies executable closure and passes independent review. The rejected
production increment has been fully reverted.

| finding_id | source | severity | decision | status | action_taken | artifact_ref | rationale |
|---|---|---|---|---|---|---|---|
| `CTDR-A-001` | agent_a | high | accepted | closed | Added normative stable node IDs, source-aligned node taxonomy, complete direct-edge generator, schema version/hash, inclusive closure, conservative unknown-edge handling, duplicated wet regions, longwave, routing, leaf/CI/hydraulic/shared/residual/output descendants and exact order. | `SC-LANDSURFACEENERGY-001.md#component-temperature-jacobian-dependency-replay-amendment` | Graph construction and replay reachability are independently reproducible for arbitrary topology. |
| `CTDR-A-002` | agent_a | high | accepted | closed | Split immutable whole-sweep base evidence from fresh non-Clone, single-use signed-probe capabilities and bound generation/input/caps/frozen/trial/iteration/sweep/graph/coordinate/sign/perturbation/probe/stencil identity, expiry and zero-mutation drop. | canonical amendment; `readiness-matrix.md` | One base can serve all probes without allowing stale cross-boundary reuse. |
| `CTDR-A-003` | agent_a | high | accepted | closed | Added eligibility/integrity/mismatch/error outcome table: behavior-identical complete evaluation only before replay, direct typed integrity failure, direct post-start error, exact first-error source order and no recovery fallback. | canonical amendment | Ordinary selection and forbidden error recovery are no longer conflated. |
| `CTDR-A-004` | agent_a | medium | accepted | closed | Aligned contract/index evidence level to `static+independent_oracle+contract_vectors`; used schema-valid `maps-to-existing-INV` in BEI/profile and separately stated revision 31 introduced new IDs. | contract front matter, BEI/profile; index; authority test | `new-INV` is an ID-introduction fact, not a permitted BEI classification value. |
| `CTDR-A-005` | agent_a | medium | accepted | closed | Bound symmetric three-run CPU-0 command/environment, unchanged pre/post source and binary identities, exact JSON fields, sorted-middle medians, candidate ceilings, per-run identity/RSS and full-revert rule. | canonical amendment; `contract_ref.md`; baseline log | Baseline and candidate now use one uniquely executable comparison protocol. |
| `B-01` | agent_b | high | accepted | closed | Recorded source manifest `78d756...bbbe`, binary path/hash `9a91c8...73f`, three raw tuples, exact JSON paths, medians `4903570/353431 us`, and ceilings `4803570/253431 us` under the exact release command. | `artifacts/terminal-heavy-gates/component_dependency_replay_baseline_3run.log`; canonical amendment; `contract_ref.md` | The parent supplied the previously missing frozen current-source numeric baseline; source manifest matches before/after all runs. |
| `B-02` | agent_b | high | accepted | closed | Restricted `58/14/16/28` to one named fully centered `N=2,S=6` sweep; added generic admitted-sign formulas, sweep reset/seal semantics and separately reset run aggregation for solve/stencil and every source-real lifecycle class. | canonical amendment; readiness matrix | Fixture accounting is no longer misrepresented as a whole-release count, and an unreachable short-circuit class is not fabricated. |
| `B-03` | agent_b | high | accepted | closed | BEI and profile now use `maps-to-existing-INV`; the test extracts the BEI row and asserts its exact cell plus the separate new-ID note. | canonical BEI/profile; authority test | This follows the science-contract schema while preserving that v31 introduced INV-164/C-020. |
| `B-04` | agent_b | high | accepted | closed | Parent reconciled three stale runtime version assertions to revision 31 in their owning integration files. | `tests/integration/snow_stage3_shared_carrier_authority_contract.rs`; `tests/integration/stage3_native_vegetation_laned_throughput_recovery.rs` | Static inspection confirms all three named assertions now require `contract_version: 31`; both parent-owned files are members four and five of the corrected eight-file manifest. |
| `B-05` | agent_b | high | accepted | closed | Added distinct first and second wet evaluation/finalization nodes, all lower routing, route-match guard, terminal release/stemflow descendants, complete edge families and stable source order. The feasibility correction now applies source-real paired errors only to crossable surfaces, implication plus authentic success vectors to noncrossable fallible surfaces, and exact fields without invented errors to infallible surfaces. | canonical amendment | The oracle audits value completeness and every naturally reachable first-error chronology without synthetic hooks. |
| `B-06` | agent_b | medium | accepted | closed | Replaced concatenated raw-substring checks with comment/cooked/raw/character/byte-character literal masking that preserves lifetimes, Rust item-body parsing, arbitrary stacked/multiline `cfg`/`cfg_attr` rejection through item visibility/modifier prefixes, and live adversarial parser tests. The seam is explicitly limited to unconditional top-level item presence. | `land_surface_energy_balance_authority_contract.rs` | Comments, literals, disabled/test-only and nested items cannot satisfy the source classifier; disconnected/skeleton items are explicitly left to executable readiness gates. |
| `CTDR-RRA-001` | corrected_rereview_a | high | accepted | closed | Hardened the item parser to reject any top-level declaration carrying `cfg` or `cfg_attr`, irrespective of attribute stacking, layout, expression, visibility or function modifiers; added active fixtures for `cfg(any())`, multiline `cfg(any(test, feature = ...))`, `cfg_attr`, embedded-quote raw-string and arbitrary-hash raw-byte-string decoys plus an unconditional positive control. | `land_surface_energy_balance_authority_contract.rs` | The intended production seam must be unconditional and lexically real; no false/disabled/test-only or literal declaration can satisfy structural readiness. |
| `CTDR-RRA-002` | corrected_rereview_a | high | accepted | closed | Expanded manifest custody to include both parent-edited stale-pin integration files in exact positions four and five. | `contract_ref.md#ordered-manifest`; readiness matrix | The corrected digest now covers every file changed to make revision-31 authority assertions consistent. |
| `CTDR-RRB-001` | corrected_rereview_b | high | accepted | closed | Added lifetime-safe character and byte-character literal parsing for unescaped, escaped, hexadecimal and Unicode forms; active adversarial fixtures bind `'}'`, `'\''`, `b'}'`, `b'\''`, named/static/placeholder lifetimes and all seven cfg-gated nested required items. | `land_surface_energy_balance_authority_contract.rs` | Literal braces can no longer corrupt body/top-level depth, and cfg-only nested item decoys cannot satisfy the item-presence seam. |
| `CTDR-RRB-002` | corrected_rereview_b | high | accepted | closed | Removed token-based dispatcher-call and helper-consumption assertions. Narrowed expected-red to unconditional top-level presence for seven declarations; added normative text that skeleton/disconnected symbols cannot establish implementation readiness and bound connectivity/consumption to executable authentic counters plus the forced-complete oracle. The adversarial dispatcher retains cfg-only, local, bare-reference and dead-call tokens to expose the rejected inference. | canonical amendment; `land_surface_energy_balance_authority_contract.rs`; `contract_ref.md`; readiness matrix | Source token occurrence cannot prove AST call shape or control/data-flow reachability; the contract now matches the classifier's defensible evidence class. |
| `CTDR-RRB-003` | corrected_rereview_b | high | accepted | closed | Added all ten source-real direct edges omitted from the normative table: wet-probe to first finalization, prepare to second wet, column longwave to occupancy output, both terminal routing results to final output, lower/ground output to each shared node, and shared heat/vapor to shared tolerance. Added focused authority assertions for every relationship. | canonical direct-edge generator; `land_surface_energy_balance_authority_contract.rs`; `contract_ref.md`; readiness matrix | Transitive reachability cannot substitute for a complete versioned direct-edge authority; the corrected table now matches direct source reads at the existing stable-node granularity. |

## Pre-revert implementation-review findings

| finding_id | source | severity | decision | status | required closure before retention | authority disposition |
|---|---|---|---|---|---|---|
| `CTDR-IMPL-A-001` | implementation_review_a | high | accepted | open-until-reimplementation | Refactor complete and replay evaluation to consume one shared canonical node/tail implementation; prove no mirrored physical, tolerance, residual, branch or output arithmetic remains. | Added as an explicit INV-164/C-020 obligation; no implementation closure is claimed. |
| `CTDR-IMPL-A-002` | implementation_review_a | high | accepted | open-until-reimplementation | Implement every normative direct edge and compare an independently enumerated exact edge set plus golden hash for generic and `N=2,S=6` topologies; reachability-only sampling is insufficient. | Complete direct-edge oracle is now normative. |
| `CTDR-IMPL-A-003` | implementation_review_a | high | accepted | open-until-reimplementation | Bind exact generation/all-input/caps/frozen/graph/trial/coordinate/sign/perturbation/probe/stencil custody and the successful base evaluation without length-only or expensive hot-path proxy checks. | Typed borrow/generation or compact validation-point seal rules now replace `Debug`, length-only, clone and repeated-scan proxies. |
| `CTDR-IMPL-A-004` | implementation_review_a | high | accepted | open-until-reimplementation | Supply the corrected source-real fallibility/crossability oracle, natural-error catch-all, exact field parity and rollback/no-fallback evidence for both potential and fixed-final solves. | The impossible every-node poison mandate is removed; the replacement matrix is authority, but executable evidence remains absent. |
| `CTDR-IMPL-A-005` | implementation_review_a | high | accepted | open-until-reimplementation | Source map/solve/iteration/sweep identities independently from their real lifecycle, reconcile every real completed/failed/inward sweep, and represent short-circuit only if the unchanged solver has such a path. | Copied identity proxies and impossible always-zero lifecycle classes are explicitly forbidden. |
| `CTDR-IB-001` | implementation_review_b | critical | accepted | open-until-reimplementation | Exercise real dispatcher connectivity and corrected C-020 source-real errors, beginning/custody rollback, dense Jacobian/pivot/norm/trajectory and full potential/final solve bit parity with no fallback. | Same-v31 feasibility correction narrows impossible poisons without weakening natural-error, rollback or numerical equality. |
| `CTDR-IB-002` | implementation_review_b | high | accepted | open-until-reimplementation | Make replay decisions from the complete normative node graph and prove exact direct-edge completeness independently, rather than using the graph as a label for a separately hand-coded coarse replay mask. | Graph-driven source-order execution and exact edge testing remain mandatory. |
| `CTDR-IB-003` | implementation_review_b | high | accepted | open-until-reimplementation | Enforce exact base/probe custody including generation, caps, frozen state and authentic map identity; eliminate digest-length and copied-ordinal checks. | Exact no-proxy custody and distinct lifecycle identity requirements now govern reimplementation. |
| `CTDR-IB-004` | implementation_review_b | high | accepted | open-until-reimplementation | Seal and aggregate every source-real sweep with per-record bucket/stencil/lifecycle reconciliation, both solve classes and emitted full-solve oracle status; do not fabricate unreachable states. | Audit semantics now name only authentic completed/failed plus any proven real short-circuit. |
| `CTDR-IB-005` | implementation_review_b | high | accepted | open-until-reimplementation | Remove per-sweep/per-probe string-map/closure/`Debug` hashing, whole-trial scanning and probe cloning overhead, then pass the frozen same-source/same-binary three-run timing, science and RSS gate. | Frozen baseline/ceilings remain unchanged; efficient exact typed custody is required and no candidate result is claimed. |
| `CTDR-IB-006` | implementation_review_b | medium | accepted | open-until-reimplementation | Split graph, custody, parity, error/rollback, audit and release tests into cohesive modules/fixtures before file-size or reviewability limits are approached. | Fresh implementation review must confirm focused ownership and maintainable test structure. |

## Gate disposition

Contract-review finding disposition: `PASS`; all historical contract-review
findings are closed. Implementation-review finding disposition: `HOLD`; all 11
pre-revert implementation findings are accepted and open until reimplementation.

The historical preimplementation readiness state was `HOLD` pending fresh
production work and its gates. That work was subsequently completed and
reviewed, then failed its release conjunct and was reverted. Authority
correction does not itself close a retained-production finding or authorize
partial retention of the rejected implementation.

## Fresh implementation and release disposition — 2026-09-04

The fresh candidate closed every finding above through seven immutable review
cuts. Final implementation Review A and B both approved ordered 16-path manifest
`edc3f0b94c393e537b0115403548b779754f33308e53a26b1041932e9915be71`.
Those closures are valid evidence about the rejected candidate; none authorizes
retaining it after a failed release conjunct.

| finding | final candidate status | release/revert disposition |
|---|---|---|
| `CTDR-IMPL-A-001..005`, `CTDR-IB-001..006`, and correction findings through `R8-001` | `CLOSED` by exact graph-walker/copy semantics, O(1) typed custody, source-real error/rollback, per-guard proof, full dense/backtracking/owner parity, authentic runner bytes, and feature-enabled lint/compiler evidence | Candidate approved for measurement only. |
| release aggregate authentic `N=2,S=6` `58/14/16/28` obligation | `FAILED` on exact candidate run 1 before JSON emission | Exit `101`; runs 2/3 not run; complete v31 production/test/runner increment reverted. |
| post-revert production seam | `EXPECTED_RED` on exactly seven absent declarations | LSE `140/140`, all-target checks, fmt/diff/residue pass; no partial replay mechanism retained. |
| historical-source reconciliation | `PASS_WITH_EXPLAINED_DELTA` | Current manifest `2813f6e8...ee0d` includes 23 authorized post-baseline authority-test patches. Reversing them in memory reproduces frozen baseline `78d756...bbbe`; restored production hosts match archived pre-v31 identities. |

Current implementation/release disposition: `FAIL_REVERTED`. Revision 31
remains authoritative but unimplemented. Final independent Verifiers A and B
both approve terminal package `HOLD` with no remaining findings; neither grants
release, performance, exact-workspace, or completion status.

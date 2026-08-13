# Gate Results

Status: `EXECUTING / V3 authority intake PASS / focused remediation active`

Evidence mode: `Ran`

| Gate | Result | Evidence |
|---|---|---|
| Milestone 0 A0/model/oracle | PASS | pre-implementation artifact and comparator logs |
| Focused compile | PASS | `cargo check` for the three affected crates |
| Focused clippy | PASS | affected crates with `-D warnings` |
| Boundary scaffold | PASS | 7 tests in `vegetation_boundary_authority_contract` |
| E01--E22 public transaction | FAIL | science review B-CRITICAL-001--005 |
| independent five-ledger closure/rollback | FAIL | science review B-HIGH-006 |
| exact numerical/guard posture | FAIL | science review B-CRITICAL-002/003 and B-HIGH-007 |
| Critical/full workspace/benchmarks | NOT RUN | illegitimate while material focused gates fail |
| dual terminal verification | NOT RUN | terminal bytes are not closure-eligible |

## Remediation Continuation

The rows above preserve checkpoint results, including failures. New command
results append below; no retry replaces historical evidence.

| Gate | Result | Evidence |
|---|---|---|
| checkpoint identity and clean tree | PASS | `git rev-parse HEAD` returned `c064206883bd26848a93bd4b9b104b7f5b647344`; `git status --short --branch` reported clean `main...origin/main` |
| instruction discovery for full continuation write set | PASS | `tools/agents/find-agents --for ...` returned root plus `crates/AGENTS.md`, `tests/AGENTS.md`, and `docs/work-packages/AGENTS.md` as applicable |
| restored A0 suite | PASS | `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`: 12 passed |
| implementation/transaction suite | PASS | `cargo nextest run --test c3_vegetation_implementation_contract --profile quick`: 10 passed, including the real public transaction and 13 injected rollback phases |
| vegetation crate | PASS | `cargo nextest run -p openwepp-vegetation --profile quick`: 2 passed, including exact four-node/cap hydraulic oracle vectors |
| BGC crate | PASS | `cargo nextest run -p openwepp-biogeochemistry --profile quick`: 3 passed for proportional competition, species/layer separation, unused authorization, and duplicate receipt rejection |
| hillslope orchestrator crate | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --profile quick`: 490 passed in 149.178 s; three existing slow OFE-routing tests |
| focused checks | PASS | all four required package-scoped `cargo check` commands passed |
| focused strict Clippy | PASS after correction | all four required package-scoped commands passed; an earlier BGC test-only float-comparison failure was corrected rather than erased |
| science admission | PASS after correction | first retry rejected missing impact binding for `vegetation_diagnostic.rs`; bounded catalog bindings were added for it and the typed resource DTO; retry returned `A0_ADMITTED contracts=44 science_surfaces=2` |
| authority anti-evasion | PASS | `check_authority_suite_antievasion.sh` passed |
| AUTH11 obligation guards | PASS | 3 passed |
| SC unit compliance | PASS | both `SC-VEGETATION-001.md` and `SC-BIOGEOCHEM-001.md` returned no findings |
| independent oracle regeneration | PASS | 11 selected values matched at relative/absolute tolerance `1e-12`; fixture SHA-256 `339537846d2cad3b5f03f55cd1946b545a35a49aca9d92f6cd952e9bde83e964` |
| formatting/diff hygiene | PASS | `cargo fmt --all -- --check` and `git diff --check` passed |

## Hold-boundary continuation

| Gate | Result | Evidence |
|---|---|---|
| coverage-corrected radiation | PASS | vegetation crate 7/7 after exact tile-local `area/C_s`, `f_t` aggregation, and multirank boundary correction |
| implementation transaction suite | PASS after containment update | 10/10; ambiguous partial-cover E04 now returns typed unsupported before candidate execution |
| repeat science review | HOLD | independent targeted adjudication confirms a canonical E04 heterogeneous-tile state/aggregation omission |
| repeat Rust review | FAIL / accepted findings remain | numerical diagnostics, multirank final-liquid routing, independent owner reconstruction, all-owner commit API, duplicate request identity, and line-count work remain open |
| heavy/full/benchmark gates | NOT RUN | prohibited because focused science review cannot pass without constitutive authority |
| terminal verification and prompt archive | NOT RUN | no terminal-complete bytes exist |
| Markdown package lint | PASS after invocation correction | initial invocation omitted `--path` and failed argument parsing; `markdown-doc lint --path <package>` then validated 46 files with 0 errors and 0 warnings |
| final HOLD admission/lint reconciliation | PASS | admission returned `A0_ADMITTED contracts=44 science_surfaces=3`; both SC unit lints passed; package Markdown lint validated 47 files; `git diff --check` passed |

## Accepted review-remediation rerun

The first fresh science and Rust reviews returned FAIL and remain preserved in
their remediation review artifacts. All material findings were accepted and
corrected before this rerun.

| Gate | Result | Evidence |
|---|---|---|
| restored A0 suite | PASS | 12 passed |
| implementation/transaction suite | PASS | 10 passed, including strict digest and serialized 13-phase rollback evidence |
| vegetation crate | PASS | 6 passed, including Brent/singularity, condensation capacity, two-rank boundary routing, and stem-only optics poisons |
| BGC crate | PASS | 3 passed, including deterministic order, species/layer separation, and exact receipt rejection |
| hillslope orchestrator crate | PASS | 490 passed in 148.030 s; three known slow routing tests |
| focused strict Clippy | PASS | all four affected crates passed `--all-targets -- -D warnings` |
| science admission | PASS after binding correction | first run rejected the new kernel-contract manifest surface; impact-map generation 20 added its SC/A1 binding; rerun returned `A0_ADMITTED contracts=44 science_surfaces=3` |
| authority anti-evasion | PASS | authority suite anti-evasion script passed |
| AUTH11 obligation guards | PASS | 3 passed |
| SC unit compliance | PASS | both vegetation and BGC contracts returned no findings |
| formatting/diff hygiene | PASS | `cargo fmt --all -- --check` and `git diff --check` passed |

## 2026-08-12 Stage-B V2 Intake

The historical failures and HOLD evidence above remain preserved. The separate
contract-first authority package released the missing topology authority at
commit `817b082d01d194cde61b1cf284bd85e40e44afc9`.

| Gate | Result | Evidence |
|---|---|---|
| V2 definition identity | PASS | both canonical copies SHA-256 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3` |
| independent V2 oracle regeneration | PASS | regenerated fixture byte-identical; SHA-256 `c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1` |
| shared transaction contract identity | PASS | SHA-256 `c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8` |
| science admission | PASS | 45 contracts, zero science surfaces, receipt `464b2675f17f75a6a9e92c6de0a70dae76ef03ca092c23f29d2ad965d62be628` |
| contract unit checks | PASS | vegetation and vegetation-transaction contracts found no findings |
| authority anti-evasion | PASS | source-level suite guard passed |
| AUTH11 | PASS | 3/3 |
| A0 vegetation authority | PASS | 14/14 |
| formatting, Markdown, diff hygiene | PASS | 49 package files and catalog had zero Markdown findings; format and diff checks passed |

## 2026-08-12 V2 Identity Increment

| Gate | Result | Evidence |
|---|---|---|
| embedded executable model | PASS | exact released `OPENWEPP_C3_WOODY_V2`, SHA-256 `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3` |
| V1 immutability/nonexecution | PASS | V1 registry remains SHA-256 `003107043e8eb5bda6d9d6476e3ea01690815e3280ac98daf169317ce4d09157`; V1 fails the V2 executable identity gate |
| vegetation compile/tests | PASS | `cargo check -p openwepp-vegetation`; crate quick suite `10/10` |
| diff hygiene | PASS | `git diff --check` |

## 2026-08-12 Occupancy-State Primitive

| Gate | Result | Evidence |
|---|---|---|
| exact V2 lane schema | PASS | all 15 frozen fields, typed occupancy/root identities, strict domains, root order/cardinality, and transaction continuity |
| canonical serialization | PASS | authority bytes reproduced exactly; SHA-256 `5cb16721125b5352e4aadb861b5e928f13ce05ff32f34533798648ebc2c4bd4b` |
| vegetation compile/tests | PASS | crate quick suite `21/21` |
| strict Clippy/format/diff | PASS | `-D warnings`, format check, and diff check |

The implementation integration suite then passed `9/10` and rejected its old
V1 whole-transaction fixture at the V2 model-digest gate. This is expected
fail-closed transition evidence, not a fixture to relabel: a genuine V2
occupancy-state fixture and migration path must replace it before the suite can
pass again.

## 2026-08-12 Typed Identity and Pure-Kernel Increment

| Gate | Result | Evidence |
|---|---|---|
| resource identity | PASS | typed stratum/tile/occupancy/layer identity, explicit stand-ground water basis, duplicate and mixed-transaction rejection |
| kernel-contract crate | PASS | quick suite `48/48` |
| E04 drainage operands | PASS | initial and second drainage exposed separately; closure uses both; evaporation, condensation, and stemflow tests |
| E20 offset identity | PASS | exact `Nlit=Cfall/CNleaf_litter` and `Nret=Cfall/CNleaf-Nlit`; insufficient donor fails before mutation |
| vegetation crate | PASS | quick suite `14/14` |
| diagnostic compile | PASS | occupancy-keyed requests compete proportionally over shared layer supply |
| diff hygiene | PASS | `git diff --check` |

## 2026-08-12 Milestone 1 V2 State Integration and Migration

| Gate | Result | Evidence |
|---|---|---|
| V2 state structure | PASS | shared stratum state contains no liquid or hydraulic numerical lane; exact typed occupancy map owns all 15 V2 fields |
| exact state identity | PASS | exact two-tile/two-stratum set, missing/extra/wrong/duplicate occupancy, missing/extra stratum, root order/cardinality, model/config/state identity, unit spelling, and initial/prior transaction tests |
| canonical digest | PASS | every occupancy field, occupancy identity, root order, configuration digest, and transaction marker changes the whole-state digest; fixed fixture SHA-256 `70d05bcda1e31aa82e9444cf73b032f20a47f6894c663ca07103bf36a0a7d77a` |
| V1 state isolation | PASS | V1 bytes fail the V2 parser; shared liquid and scalar hydraulic warm starts remain only in `V1StratumState` |
| V1 migration | PASS | zero store, exact single-occupancy coverage conversion, exhaustive nonzero multi-tile unresolved report, missing/invalid caller lanes, invalid V1 liquid, and V1 identity tests |
| RHESSys migration | PASS | exhaustive deterministic occupancy-field report; no lane synthesis or executable hash |
| public transaction boundary | PASS / fail-closed | complete V2 validation precedes a typed E04 implementation-incomplete error; no candidate, request, mutation, or commit is produced |
| vegetation crate | PASS | `cargo nextest run -p openwepp-vegetation --profile quick`: 38 passed |
| implementation suite | PASS | `cargo nextest run --test c3_vegetation_implementation_contract --profile quick`: 11 passed |
| four focused checks | PASS | kernel-contract, vegetation, BGC, and hillslope-orchestrator `cargo check` commands passed |
| strict Clippy first attempt | FAIL | vegetation and downstream hillslope Clippy exposed duplicated/dangling attributes left by removal of inaccessible V1 transaction blocks; kernel-contract and BGC passed |
| strict Clippy correction | PASS | dangling attributes removed; vegetation and hillslope-orchestrator reruns passed `--all-targets -- -D warnings` |
| formatting/diff hygiene | PASS after format correction | first format check identified one integration-test wrap; `cargo fmt --all` applied it, then format and diff checks passed |
| A0 vegetation authority | PASS | 14/14 |
| authority anti-evasion | PASS | source-level authority-suite guard passed |
| AUTH11 obligation guards | PASS | 3/3 |
| package Markdown | PASS | 49 files, zero errors and zero warnings |

Milestone 1 does not claim E04 routing, E11--E15 capped coupling, owner
candidate construction, closure, or commit. Heavy gates remain ineligible.

## 2026-08-12 Increment 2A Internal Tile-Column Engine

Historical failures and retries above remain unchanged. This increment also
retains its own edit-loop failures rather than replacing them with final passes.

| Gate | Result | Evidence |
|---|---|---|
| first vegetation quick run | FAIL, 47/48 | replicated-store poison used `store0=3`, which was out of the accepted occupancy-result domain and returned the earlier typed domain guard rather than the closure error asserted by the test |
| replicated-store poison correction | PASS | changed the poison to replicate one valid lane's full `0.15 kg m-2 tile-ground` store into every lane; the distinct lower/tile-b beginning stores now reach and fail independent closure as intended |
| first strict vegetation Clippy | FAIL | `execute_tile_column` exceeded the 100-line lint and exact float comparisons triggered `float_cmp` |
| Clippy correction | PASS | decomposed column/per-occupancy acceptance and used exact `to_bits` identity for the candidate-store operand; `cargo clippy -p openwepp-vegetation --all-targets -- -D warnings` passed |
| private-module visibility check | FAIL then PASS | making the developmental module private after the first pass produced dead-code and unreachable-public errors under strict Clippy; restored the intentionally callable callback seam while `execute_candidate` and commit remain fail-closed, then reran strict Clippy |
| V2 fixture configuration identity | FAIL then PASS | placeholder configuration digest exposed expected SHA-256 `9b31228cc957031a1d71e9ffb6978591e63619da9fd4a1a9e75499aacbc62d1d`; fixture updated and exact identity test passed |
| V2 fixture state identity | FAIL then PASS | placeholder state digest exposed expected SHA-256 `30b0df3945d3b63a45b05f7234bf4d0a37d3130aa5f542c9cb7528cbac1b4327`; state and configuration cross-binding updated and strict validation passed |
| internal topology/routing vectors | PASS | empty/single/two-rank columns, throughfall, initial/second drainage, condensation, stemflow bypass, tile isolation/order, conditional area, one-time weighting, fixed-cap back-conversion plumbing, local/column/stand closure, producer-residual poison, and rollback |
| vegetation crate | PASS | `cargo nextest run -p openwepp-vegetation --profile quick`: 51/51 |
| implementation suite | PASS | `cargo nextest run --test c3_vegetation_implementation_contract --profile quick`: 11/11; public V2 execution remains explicitly fail-closed |
| A0 vegetation authority | PASS | `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`: 14/14 |
| AUTH11 obligation guards | PASS | `cargo nextest run --test auth11_required_suite_obligation_guards_contract`: 3/3 |
| four focused checks | PASS | kernel-contract, vegetation, BGC, and hillslope-orchestrator `cargo check` commands passed |
| four affected strict Clippy gates | PASS | kernel-contract, vegetation, BGC, and hillslope-orchestrator passed `--all-targets -- -D warnings` |
| authority anti-evasion | PASS | `bash tools/release/check_authority_suite_antievasion.sh` |
| science admission | PASS | base `c064206883bd26848a93bd4b9b104b7f5b647344`, 45 contracts, 3 science surfaces, authority SHA-256 `49f331d85841ee60f361e5ac6937007829d544a5a0c4a2225dca88097e856913` |
| formatting/diff hygiene | PASS | `cargo fmt --all -- --check` and `git diff --check` |
| package Markdown | PASS | `markdown-doc lint --path <active-package> --format plain`: 49 files, 0 errors, 0 warnings |

Heavy/full-workspace, benchmarks, science review, and terminal verification
were not run: the exact E11--E15 occupancy solver and public candidate remain
fail-closed, so those gates are not yet legitimate.

## 2026-08-12 Increment 2B Authority Audit and HOLD Checkpoint

Prior failures and successful Increment 2A evidence remain unchanged.

| Gate | Result | Evidence |
|---|---|---|
| exact authority/code audit | HOLD | two independent read-only audits confirmed missing leaf/stem E01 reduction/partition, local surface-wind, hydraulic path/state mapping, beta-one residual semantics, and exact V2 fixture authority |
| inferred radiation attempt | REJECTED / REMOVED | a draft reused historical combined-area clumping, area-weighted optics, and absorptivity partition; review found no canonical authorization and all draft bytes were removed before acceptance |
| potential solver attempt | NOT IMPLEMENTED | sequential energy/hydraulic composition would be the prohibited one-pass endpoint; no potential/input/diagnostic solver files were retained |
| typed water boundary | PASS | complete transaction/owner/occupancy/layer/stand-basis request batches, canonical ordering, duplicate/mixed/amount guards, exact authorization correspondence, stand-to-tile conversion, and tolerance poisons |
| first strict Clippy after resource module | FAIL | dead private cap helpers/constants plus three test style lints were exposed; no warning was suppressed |
| strict Clippy correction | PASS | cap APIs documented and public for later typed final-pass use; style/conversion findings corrected; `cargo clippy -p openwepp-vegetation --all-targets -- -D warnings` passed |
| vegetation quick suite | PASS | `cargo nextest run -p openwepp-vegetation --profile quick`: 58/58 |
| four focused checks | PASS | kernel-contract, vegetation, BGC, and hillslope-orchestrator package checks passed |
| four affected strict Clippy gates | PASS | all four affected crates passed `--all-targets -- -D warnings` |
| A0 vegetation authority | PASS | `vegetation_boundary_authority_contract`: 14/14 |
| implementation contract | PASS | `c3_vegetation_implementation_contract`: 11/11; public path remains fail-closed |
| AUTH11 and anti-evasion | PASS | AUTH11 3/3; source-level authority-suite guard passed |
| science admission | PASS | base `c064206883bd26848a93bd4b9b104b7f5b647344`, 45 contracts, 3 science surfaces, authority SHA-256 `49f331d85841ee60f361e5ac6937007829d544a5a0c4a2225dca88097e856913` |
| formatting/diff/package Markdown | PASS | format and diff checks passed; 50 package files had 0 Markdown errors and 0 warnings |
| public candidate containment | PASS / fail-closed | no radiation, demand, request, candidate, or mutation is emitted by `execute_candidate()` |

Heavy/full-workspace, benchmarks, focused science review, and terminal
verification remain ineligible because exact potential authority and fixtures
are absent.

## 2026-08-12 V3 Authority Intake

| Gate | Result | Evidence |
|---|---|---|
| V3 authority predecessor | PASS | terminal authority commit `94a4c99dc1228aa0399c01f4cc9590742960028f` |
| V3 definition identity | PASS | SHA-256 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852` |
| V3 independent fixture identity | PASS | SHA-256 `1210e41f13aeffd2e099f9c812b8c5da6109ee9e23c6f51f045af9684a7ae109` |
| authority package reviews | PASS | dual science GO and dual terminal PASS, no unresolved material finding |
| implementation lifecycle | PASS | existing package resumed; V1/V2 HOLD and Increment 2A evidence preserved |

The exact potential authority and fixtures now exist. Focused implementation
may resume; heavy gates remain ineligible until the complete public V3 path and
all seven accepted Review-B findings have focused passing evidence.

## 2026-08-12 V3 Identity, State, Migration, and Radiation Increment

| Gate | Result | Evidence |
|---|---|---|
| executable model registry | PASS | V3 crate registry is byte-identical to released authority at SHA-256 `7768657ca3d03603b66f5cd6677f032ee630fdd46d6ffadf214c713065f73852`; V1/V2 digests remain unchanged and non-executable |
| V3 state schema | PASS | exact scalar `root_node_potential_mm`; old layer vector rejected by strict runtime parser; lane digest `75830208c3ff83948dc646ca218f1d64fcefc94dc3412e789f2e0634075ea04e` |
| V2-to-V3 migration | PASS | nonempty bitwise-identical finite roots migrate; unequal, empty, nonfinite, and `+0/-0` vectors return `AmbiguousV2LayerRootWarmStarts` per occupancy without normalization |
| V2 migration intake | PASS | historical array-of-pairs encoding parses strictly; duplicate occupancy identities, invalid V3 configurations, wrong topology/layer identity, and forged model identity fail closed |
| V3 consumed configuration | PASS | removed `rd_leaf_n_rate`; historical V1/V2 configuration bytes fail the V3 strict parser |
| named V3 configuration/state fixtures | PASS after digest correction | Rust-canonical configuration SHA-256 `fcb8e1d0cbbf206c7439d5e8f06e1037f0860a5ec2556170b52b3d41bea54e8e` and state SHA-256 `141b485649b63cd0b8d62bf113225d8e44ad1eea1020e8628ecc89e84562d59a` validate and cross-bind |
| V3 whole-column radiation kernel | PASS | exact mixed leaf/stem optical reduction, one-time clumping, owner absorption partition, sunlit leaf identity, ordered two-rank directional transport, and closure/identity poisons; 5/5 focused radiation tests |
| V3 wind/respiration primitives | PASS | friction-velocity canopy-surface wind and single-source Atkin `Rd25`/temperature/debit primitives have focused positive and domain tests |
| public candidate containment | PASS / fail-closed | complete V3 identity/state/forcing validation precedes typed capped-transaction implementation-incomplete failure; no request or candidate is emitted |
| vegetation crate | PASS | quick suite `73/73` |
| implementation contract | PASS | quick suite `11/11` |
| A0 vegetation authority | PASS | quick suite `17/17` |
| four affected checks | PASS | kernel-contract, vegetation, BGC, and hillslope-orchestrator |
| strict affected Clippy | PASS | kernel-contract, vegetation, BGC, and hillslope-orchestrator all targets, warnings denied |
| AUTH11 and anti-evasion | PASS | AUTH11 `3/3`; source-level authority-suite guard passed |
| science-contract unit lint | PASS | `SC-VEGETATION-001` and `SC-BIOGEOCHEM-001` |
| formatting/diff hygiene | PASS | `cargo fmt --all -- --check`; `git diff --check` |

This increment closes V3 executable identity, state schema, offline migration,
and the pure whole-column radiation/wind/respiration preparation surfaces. It
does not claim the coupled potential demand, capped finalization, or a public
candidate.

## 2026-08-12 V3 Column-Radiation and Stage-A Driver Increment

| Gate | Result | Evidence |
|---|---|---|
| whole-column radiation adapter | PASS | exact topology occupancy identity, conditional area, VIS/NIR, direct/diffuse, and leaf/stem owner results are prepared before occupancy calculation; 6/6 focused tests |
| candidate transaction identity propagation | PASS | the explicit `TransactionId` now enters every column occupancy input; radiation preparation does not fabricate transaction identity |
| Stage-A numerical driver first review | HOLD then corrected / infrastructure only | first review found convergence-control, residual-sign, nested-error, singular-identity, diagnostic-finiteness, layer-identity, public-surface, and test-evidence defects; the driver was made crate-private and corrected before any constitutive adapter or public use |
| Stage-A driver smoke tests | PASS | distinct-class six-residual solve, initially converged state, and exact zero-demand branch; exact independent constitutive fixture remains pending |
| vegetation crate | PASS | quick suite `81/81` before review correction; rerun below records current exact bytes |
| strict vegetation Clippy | PASS | all targets, warnings denied |
| formatting/diff hygiene | PASS | `cargo fmt --all -- --check`; `git diff --check` |
| Stage-A correctness rereview | PASS | crate-private infrastructure only; no remaining material finding after convergence, error-precedence, finiteness, residual-sign, layer-identity, and overflow corrections |
| Stage-A QA rereview | PASS | bounded infrastructure increment approved; exact fixture and production adapter remain explicitly pending |
| post-review vegetation crate | PASS | quick suite `82/82` |

The Stage-A driver accepts only a callback that returns a complete nested
gas/energy/hydraulic evaluation for every trial state. The production adapter
from configuration, forcing, occupancy radiation, interception, and soil
forcing is not yet implemented, so this is not E11--E15 acceptance evidence,
does not emit a potential request batch, and does not lift the public
fail-closed branch.

## 2026-08-12 Typed Request Boundaries

| Gate | Result | Evidence |
|---|---|---|
| potential water request orchestration | PASS / constitutive seam pending | centralized complete input validation, exact radiation/area binding, sequential transaction, one-time tile-to-stand conversion, configured root-layer completeness, typed batch, rollback; focused `4/4`; correctness rereview PASS |
| typed E19 mineral-N boundary | PASS / shared-state consumer pending | internal retranslocation first; exact stratum owner/layer/species identity; authorization correspondence; proportional finalized use; unused authorization preserved; focused `7/7` |
| constitutive evaluator first review | HOLD / remediation active | positive oracle vectors passed, but reviewers accepted nested diagnostic, runtime-input provenance, configured-dimension, retained-state, adapter, validation, and numerical-evidence defects; module remains disconnected from compilation/public request path while remediation proceeds |
| public candidate containment | PASS / fail-closed | no request pass or constitutive evaluator is reachable from `execute_candidate()` |
| focused vegetation after request boundaries | PASS | quick suite `93/93`; strict all-target Clippy, formatting, and diff hygiene pass |

No E11--E15 or Milestone 2/3 completion claim is made until the corrected
constitutive evaluator, production adapter, exact nested failure vectors, and
authorization-capped second pass all pass independent review.

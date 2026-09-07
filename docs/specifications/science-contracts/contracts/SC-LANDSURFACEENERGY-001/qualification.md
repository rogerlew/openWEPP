[Parent contract](../SC-LANDSURFACEENERGY-001.md)

## Dependencies
| Target | Required when | Boundary/obligation | Reading extent |
|---|---|---|---|
| interface.md#interface | every task | universal scope, owners, failure and qualification | whole mechanism chapter |
| dependency-replay.md#dependency-replay | scientific replay/coverage/result claims | graph, custody, errors and forced-complete proof | whole mechanism chapter |
| nonlinear-solve.md#nonlinear-solve | solver scientific result claims | complete ordered solver | whole mechanism chapter |
| ../../../../work-packages/20260906-stage3-prospective-mechanism-experiments-001/prompts/active/kickoff.md#4-experimental-design-one-baseline-two-independent-treatments | EXP-R executable identity/capture checks | frozen execution requirements; not scientific authority | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |
| ../../../../work-packages/20260906-stage3-prospective-mechanism-experiments-001/artifacts/worker-handoff.md#stage-3-current-continuation | EXP-R execution authorization/status | owner pause and frozen execution posture | named section plus destination scope; retain applicable single-file whole-contract/frozen-kickoff requirements |

<a id="qualification"></a>
# Qualification
Binding qualification and capture requirements, not evidence of successful adoption or current execution authorization. Historical revision-31 thresholds/results remain unchanged. EXP-R changes only its stated gates; PC1 and SG1 retain their exact experimental scope. The frozen prospective kickoff remains binding and owner-paused; format migration does not resume it.

Production retention uses this exact command for baseline and candidate:

```text
timeout 1800 taskset -c 0 env RUST_MIN_STACK=67108864 CARGO_PROFILE_RELEASE_LTO=false nix develop -c cargo test --release -p openwepp-runner --lib hillslope::tests::stage3_laned_release_one_ofe_positive_baseline_profile -- --ignored --exact --nocapture --test-threads=1
```

The frozen pre-v31 Rust source manifest is
`78d756be1fa11ed85ee92b7d19e6c04427b01b122efaf7804d1b55d60536bbbe`
before and after all three runs. The unchanged binary is
`/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/release/deps/openwepp_runner-fc552493dc3c6cc2`,
SHA-256 `9a91c82f1799382014c3a561e79130b5f5b665bef0667a4bdff613c91d8e573f`.
From each `STAGE3_LANED_RELEASE_PROBE` JSON record, ordered
`(run_wall_us, physical_phase_wall_us.potential, rss_kib)` values are
`(4926758,354838,70696)`, `(4903570,353374,54624)`, and
`(4896095,353431,59364)`. Sort each timing field independently and select the
middle integer: baseline medians are `run_wall_us=4903570` and
`physical_phase_wall_us.potential=353431`.

Build the candidate once; record one unchanged candidate source-manifest hash
before/after three repetitions and one unchanged candidate binary path/hash;
then run the identical command/environment three times. The same sorted-middle
median requires candidate `run_wall_us <= 4803570` and
`physical_phase_wall_us.potential <= 253431`, at least `100000 us` improvement
in both exact JSON fields. Every candidate run must retain
source `0.8488061229561478`, outlet `0.8471105124736579`, storage
`0.0016956104824910018`, clamp `0`, exact `48/56/20/32/4` workload counts, the
complete run-level sweep aggregation with a qualifying `58/14/16/28` named
sweep, full-solve bit equality, and JSON `rss_kib <= 65536`. If either median
ceiling fails, any identity differs, aggregation is incomplete, candidate
source/binary identity changes, or any RSS exceeds the bound, fully revert the
revision-31 production increment; retain no partial replay, cache, or fallback.

This amendment introduces no dimensional symbol, unit, conversion, constant,
parameter, tolerance, equation, output, publication field, or wire format.
Calibration and identifiability remain `CALIBRATION_NOT_APPLICABLE`.

| Profile surface | Binding |
| --- | --- |
| state surface | One immutable sweep base plus fresh non-Clone signed-probe capabilities, exact generation/input/caps/frozen/graph/trial/map/solve/iteration/sweep/coordinate/sign/perturbation/probe/stencil custody without costly proxy checks, and no persisted result or cross-boundary cache. |
| algorithm step | Construct/admit canonical probes unchanged; for one component-temperature coordinate replay reachable nodes through the one shared canonical evaluator node/tail implementation in complete-evaluator order and reuse only proven-unreachable successful nodes; assemble the existing ordered residual vector and finite difference unchanged. |
| branch/guard | Exact one-coordinate difference and every custody/hash join are mandatory; ordinary ineligibility chooses complete evaluation before replay, integrity violation fails typed, and any post-start error returns directly without fallback. |
| invariant guard map | `INV-LANDSURFACEENERGY-164` -> private graph/evidence types, canonical replay walker, forced-complete oracle, exact direct-edge graph tests, normative fallibility/crossability matrix, exact bucket counters, source-real error/rollback corpus, authentic release gate. |
| alias/unit/constant/tolerance | No new dimensional values, aliases, conversions, constants, parameters, tolerances, numerical normalization, or derivative rule. |
| calibration | `CALIBRATION_NOT_APPLICABLE`: no parameter, observation, objective, empirical evidence, or identifiability claim changes. |
| test vector | `OBL-LANDSURFACEENERGY-C-020`: exact forced-complete node/residual/Jacobian/full-solve parity; complete direct-edge oracle; source-real crossable first-error/rollback vectors; noncrossable implication and authentic boundary-success vectors; infallible exact-field parity; lifetime/integrity/custody; real two-occupancy/six-soil reciprocal-longwave, duplicated wet-routing and terminal-descendant fixture; truthful scoped/aggregate counters; exact release gate. |
| binding exposure | `LSE-V31-COMPONENT-TEMPERATURE-DEPENDENCY-REPLAY`, active, `maps-to-existing-INV`; revision 31 introduced new IDs `164/C-020`; dual review/verification required. |
| change log | 2026-09-04, contract 31: corrected contract-first feasibility after full production revert; exact same-sweep component-temperature dependency replay only, one shared canonical evaluator implementation, source-real error/rollback obligations, unchanged dense solver, trajectory and outputs. |

<a id="prospective-stencil-aware-dependency-replay-experiment"></a>
## Prospective Stencil-Aware Dependency-Replay Experiment

`EXP-STAGE3-20260906-R` is a non-versioned experimental qualification binding
under `INV-LANDSURFACEENERGY-164` and `OBL-LANDSURFACEENERGY-C-020`. Authority:
explicit owner direction dated 2026-09-06, retained in
`docs/work-packages/20260906-stage3-prospective-mechanism-experiments-001/prompts/active/kickoff.md`.
Evidence: `[DIRECT][Static]` authorization, not experimental outcome evidence.

Freeze baseline A and R = A + one reviewed same-sweep dependency-replay delta;
R is never F + replay. Historical source/index/binary identity is not an
admission requirement. Historical revision-31 rules and FAIL/HOLD results remain
unchanged, including the old fully-centered existential and 64 MiB retention
gate. For this new experiment only, those two gates and old fixed timing
ceilings are replaced by the following stencil-aware qualification and the
package's separately reviewed relative timing/memory protocol. Report existing
engineering-budget comparisons separately, including baseline failures.
Retained production remains HOLD; no production promotion or hidden production
selector is authorized.

INV-164/C-020 scientific obligations remain binding: one shared canonical
physics implementation, the complete direct-edge graph oracle and normative
fallibility/crossability matrix, immutable same-sweep base and fresh single-use
probe custody, exact forced-complete node/residual/Jacobian/full-solve parity,
source-real first typed error and rollback, and unchanged real-consumer outputs.
Unknown dependencies select complete evaluation before replay; integrity or
post-start errors return directly without fallback. No cross-sweep/iteration/
map/retry result cache, derivative redesign or duplicated physics is permitted.

Independently enumerate expected signed probes from actual coordinate stencils
and independently derived evaluator classification, using canonical coordinate
order, authenticated topology, bounds and graph eligibility. A centered
coordinate contributes its two signed probes; an inward coordinate contributes
its actual one signed probe; RejectedBeforeProbe contributes no unattempted
probe. Do not copy producer counters or producer bucket labels into expected
results. Match ordered probe identities, starts, completions and first errors
to those expectations. Count identity anchors, component replay and complete
evaluation disjointly; logical = anchor + replay + complete applies to actual
starts, while completion/error reconciliation independently proves what ended.
Preserve distinct authentic map/solve/iteration/sweep identities and actual
reachable lifecycle cardinality. Reconcile every started sweep and dropped
record; missing or unexplained dropped records invalidate coverage evidence.

Require nonzero completed component replay on the real primary workload;
eligibility alone is insufficient. Neither an all-centered sweep nor a
hardcoded 54/14/16/24 pattern is required by this protocol. Existing lawful
centered/interior and boundary tests remain intact; do not manipulate physical
state, perturbations or bounds to manufacture coverage. Before timing require
forced-complete residual/Jacobian/full-solve, boundary/error/rollback and real
output parity. Detailed oracle/audit work runs separately from performance and
memory intervals; A and R have matched optional audit posture and bounded
compact timing counters. New named tests bind these prospective claims without
weakening historical assertions. Textual authority tests prove no execution.

Change log: 2026-09-06, non-versioned EXP-STAGE3-20260906-R qualification binding;
no equation, domain, tolerance, stopping criterion, Jacobian arithmetic,
adaptive cadence, event localization, physical process, ownership, restart,
publication, exact-one transfer, or fail-closed guard change.
`CALIBRATION_NOT_APPLICABLE`: no dimensional parameter or calibration claim.

<a id="exp-stage3-20260906-r-pc1-proof-closed-maximum-leaf-eligibility"></a>
### EXP-STAGE3-20260906-R-PC1: proof-closed maximum-leaf eligibility

This separately identified supplement narrows only the prospective R experiment
above, under the existing `EXP-STAGE3-20260906-R` Binding Exposure Index entry
and `INV-LANDSURFACEENERGY-163/164`, `OBL-LANDSURFACEENERGY-C-018/020`.
Evidence: `[INFERENCE][Static]`, exact-operand implication from the Covered Leaf
Maximum-Demand Exact-Reuse Amendment and the shared canonical maximum helper;
not a new physiological bound or a claim inferred from a finite search.
Historical revision-31 scientific requirements and results are unchanged.

Before minting a signed replay capability, a sun- or shade-temperature probe
must additionally establish that the affected leaf's beta in the immutable
successful sweep base is bit-identical to binary64 `1.0`. The canonical
single-coordinate temperature probe leaves that beta unchanged. The affected
current leaf still executes at the probe's temperature in the existing source
order and may return its existing typed error. If it succeeds, the shared
canonical maximum helper at its existing source position returns that same
successful current state under INV-163: every maximum-call operand is identical
to the current call. Thus no independent maximum-leaf domain crossing is
assumed away. The unaffected leaf's current and maximum operands are unchanged;
its successful base results may be reused only with the existing graph proof.

Wet- and stem-temperature probes retain eligibility only with proof that every
leaf-call operand, including beta, leaf temperature, canopy humidity, gas
environment, conductances, biochemical inputs and constants, is bit-identical
to its successful base call. This preserves their existing eligibility where
that proof holds; it does not authorize ignoring a routed dependency or merely
assuming a successful base implies success after a changed operand.

| Trigger | Required result |
| --- | --- |
| Sun/shade affected beta is exact binary64 one and all existing eligibility/custody proofs hold | Replay; affected current call executes, then the unchanged canonical maximum helper uses the successful probe-current state. |
| Affected beta differs, or any required implication is unproved | Select the unchanged complete evaluator before capability creation; no new physical-domain rejection, normalization, error retry, or alternate solver. |
| Wet/stem leaf operands are unchanged and all existing eligibility/custody proofs hold | Existing replay selection remains available. |
| Existing current-leaf, other reachable-node, or private-integrity error | Preserve its existing first-error/source-order and rollback requirements; never retry complete evaluation after replay starts. |

The normative route/wet/longwave/hydraulic/route-match/lower-boundary implication
proofs and authentic boundary vectors remain required, as do graph completeness,
current-leaf crossable-error vectors, custody, every-field/residual/Jacobian/
potential/fixed-final trajectory parity, exact output and rollback. A report
of zero maximum-only witnesses in 198 bounded trials is diagnostic search
evidence, not an implication proof and not grounds to waive any gate.

Before this narrower selector changes behavior, R-only contract-derived runtime
tests and dual prospective review must bind: exact one versus adjacent binary64
beta values; affected sun/shade and occupancy positions; successful probe-current
to maximum equality through the shared helper; unchanged wet/stem operands;
pre-capability complete selection; and unchanged first error/no-retry/rollback.
An independent signed-stencil oracle must derive the narrower classifications
from actual base beta bits and the reviewed operand/graph proofs, not producer
labels. Nonzero completed real-workload replay remains mandatory; no fixed
component replay count replaces actual eligibility. Textual tests alone cannot
discharge these obligations.

Change log: 2026-09-06, `EXP-STAGE3-20260906-R-PC1` strengthens experimental
reuse admission, not science validity. No new unit, parameter, tolerance,
equation, derivative, solver, physical bound, custody, publication or restart
surface; `CALIBRATION_NOT_APPLICABLE`. Production retention remains HOLD.

<a id="exp-stage3-20260906-r-sg1-shared-wet-routing-guard-assurance"></a>
### EXP-STAGE3-20260906-R-SG1: shared wet-routing guard assurance

This separately identified supplement changes only the prospective EXP-R
assurance classification for `route.prepare[o]`, `route.wet[o]`,
`route.finalize[o]`, `occ.wet[o]`, and `occ.liquid[o]`, including affected
instances of those five families in lower occupancies. It is subordinate to
the existing `EXP-STAGE3-20260906-R` Binding Exposure Index entry and
INV-164/C-020. No other node family's implication obligation is replaced.
Historical C-020 noncrossability requirements and PC1 text are preserved;
their general wet-routing noncrossability assertion has NOT been proved or
passed by this experiment. This is an explicit experimental assurance-gate
substitution, not blanket C-020 qualification or production promotion.

Evidence: `[INFERENCE][Static]`, deterministic identical-call and source-order
equivalence under the shared canonical evaluator. Successful base evaluation
and admitted temperature bounds alone do not establish a pressure margin for
the canonical saturation denominator `pressure - 0.378 * es(temperature)`.
No physical pressure bound, new normalization or claim of an authentic crossing
witness follows from this observation. The existing equations, domains and
typed guards remain unchanged.

For an instance of the five named families actually recomputed by replay,
assurance shall prove that replay and forced complete evaluation consume
bit-identical signed-probe operands and branches through the SAME canonical
function body at the SAME source-order position. The canonical result, or
first existing typed error, is propagated unchanged. Preserve top-to-bottom
prepare -> first wet -> first finalization and the later source-distinct second
wet/finalization calls; neither duplication nor coalescing of their roles is
authorized. This exact result-or-error proof replaces only the unsupported
successful-base-implies-probe-success requirement for those recomputed nodes.

Every copied/skipped node still requires its actual successful base result,
bit-identical complete-call operands and branch, and complete dependency-graph
proof. This includes every earlier skipped fallible guard needed to preserve
first-error precedence. A previous success without exact operand identity is
insufficient. Unknown or unproved reuse selects unchanged complete evaluation
before replay capability creation; an integrity or post-start error returns
directly, with no complete retry, alternate solver or publication.

PC1 maximum-leaf admission and implication, current-leaf source-real errors,
longwave, hydraulic, route-match and lower-boundary implication proofs remain
unchanged. All graph, custody, canonical stencils, exact node/residual/Jacobian/
potential/fixed-final trajectory and real-consumer/output requirements remain
binding. This substitution does not waive actual success/error/rollback gates.

Before adoption and admission, R-only contract-derived tests and source-proof
artifacts must receive independent dual review. They must bind each named
recomputed family to its canonical call/operands and source order, distinguish
copied from recomputed nodes, cover authentic wet/dry/zero/exact-capacity and
upper-to-lower routing successes with exact fields, and retain the unmodified
natural-error differential catch-all. Every naturally occurring source-real
error requires identical first typed error, no later work or complete fallback,
and byte-exact beginning/custody rollback. Claim crossability only with an
authentic successful-base plus admitted canonical-probe paired witness; no
synthetic fault seam or global parameter search substitutes for the proof.

Change log: 2026-09-06, `EXP-STAGE3-20260906-R-SG1` explicitly substitutes
shared-call result-or-error equivalence for the experimental noncrossability
assurance of the five named recomputed wet-routing families only. No equation,
domain, physical bound, tolerance, derivative, solver, ownership, restart or
publication change; `CALIBRATION_NOT_APPLICABLE`. Retained production HOLD.

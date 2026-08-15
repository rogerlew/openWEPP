# Review Finding Disposition

Evidence class: `Static + Ran`

All authority-review findings were accepted. None was rejected, deferred, or
moved to follow-up.

| Finding family | Disposition | Correction |
|---|---|---|
| LSE identity and condensation basis | Accepted / closed | Exact surface/source/source-ID mapping and OFE-ground DTO basis. |
| Kernel profile, guards, units, calibration | Accepted / closed | Obligations, invariant/guard/evidence table, machine registry, readiness matrix. |
| Actual WB14 and duplicate depression custody | Accepted / closed | Shared production transition, zero legacy depression capacity, persistent post-infiltration retention. |
| Cadence and restart continuation | Accepted / closed | Exact 48-step per-OFE continuation embedded in state/digest/lineage. |
| Precipitation/canopy duplication | Accepted / closed | Mutually exclusive open raw rain and covered accepted canopy release. |
| Enthalpy priority and retained LSE energy | Accepted / closed | Conservative mixing, exact tile/source attribution, tile-basis energy receipt. |
| Cross-tile redistribution | Accepted / closed | Excess retains only against its exact tile/source key. |
| Multi-OFE area basis | Accepted / closed | `basis_ofe_id` re-keying and once-only `A_u/A_d` mass/energy conversion. |
| Evidence overstatement | Accepted / closed | Independent cadence vector added; runtime parity reserved for implementation. |

Final contract/profile reviewer: `PASS`.

Final hydrology/ownership reviewer: `PASS`.

## Runtime implementation findings

| Finding | Disposition | Remediation status |
|---|---|---|
| Actual soil-liquid and soil-thermal infiltration recipients absent | Accepted | Remediated; focused PASS |
| Producer-self-referential and partial closure | Accepted | Remediated; focused PASS |
| Strict restart combinations under-validated | Accepted | Remediated; focused PASS |
| Production OFE/lane/area/day identity unbound | Accepted | Remediated; focused PASS |
| Canonical typed error identity and rollback context absent | Accepted | Remediated; focused PASS |
| Actual LSE/soil-thermal receiver candidates and rollback hashes discarded | Accepted | Remediated; focused PASS |
| Public mutable/forgeable resource candidate | Accepted | Remediated; focused PASS |
| Canonical persistence bytes differ from digest representation | Accepted | Remediated; focused PASS |
| Unified digest uses legacy 64-bit token rather than canonical soil bytes | Accepted | Remediated; focused PASS |
| `runoff.rs` exceeds mandatory line threshold; WARN files undispositioned | Accepted | Remediated; focused PASS |
| Complete WB14 interval transition duplicated | Accepted | Remediated; focused PASS |

No runtime finding is rejected, deferred, or moved to follow-up. Exact-byte
Rust and hydrology re-reviews and terminal verification remain pending.

## Re-review round 1

| Finding | Disposition | Remediation status |
|---|---|---|
| B-REMEDIATION-HIGH-001: actual receiver ending equations lack independent reconstruction | Accepted | Remediated; focused PASS |
| B-REMEDIATION-HIGH-002: canonical E001--E011 runtime payload is incomplete | Accepted | Remediated; focused PASS |
| A-REMEDIATION-HIGH-001: mutable arbitration can forge proportional authorization | Accepted | Remediated; focused PASS |
| A-REMEDIATION-MEDIUM-004: invalid public state can emit canonical bytes | Accepted | Remediated; focused PASS |
| A-REMEDIATION-MEDIUM-005: ingress/unified candidates remain mutable and duplicated | Accepted | Remediated; focused PASS |

All findings are in-scope implementation defects. Their corrections change no
authority, model identity, production selection, or package write envelope.
No finding is closed until final exact-byte re-review passes.

## Final re-review

| Finding | Disposition | Remediation status |
|---|---|---|
| B-FINAL-HIGH-001: extra/nonfinite thermal layers and forged rollback owner identity accepted | Accepted | Remediated; focused PASS |
| B-FINAL-HIGH-002: E004/E007/E011 omit available offending identity context | Accepted | Remediated; focused PASS |
| A-FINAL-HIGH-001: independent soil aggregate omits valid residual/frozen storage | Accepted | Remediated; focused PASS |

These are bounded implementation defects in the existing bridge and receiver
validator. The correction seals independent LSE and soil-thermal receiver
expectations, narrows the Child-3 rollback set to its actual three owners,
reconstructs production aggregate soil water with residual water over unfrozen
depth, and includes typed owner/OFE/tile context when known. No new authority or
package is indicated. Fresh exact-byte review remains pending.

| Finding | Disposition | Remediation status |
|---|---|---|
| B-FINAL-PASS-HIGH-001 / A-FINAL-PASS-HIGH-001: E011 substitutes the first configured receiver and hydrology owner for the actual offending receiver or rollback row | Accepted | Remediated; focused PASS |

The first canonical structural offender now supplies typed owner, OFE and tile
context. Missing rows use the exact expected owner/identity; malformed or extra
rows retain their actual identity. Two-row thermal and wrong LSE/soil-thermal
rollback poisons assert the payload rather than only the error code.

| Finding | Disposition | Remediation status |
|---|---|---|
| B-TERMINAL-HIGH-001 / A-TERMINAL-HIGH-001: later independent thermal expectation mismatch reports first LSE receiver | Accepted | Remediated; focused PASS |
| A-TERMINAL-HIGH-002: missing non-terminal rollback row reports shifted following owner | Accepted | Remediated; focused PASS |

Expectation preflight now reports the soil-thermal owner and the exact first
actual mismatch, or expected missing row. Rollback sequence validation detects
a deletion before treating the shifted row as malformed, so a missing first
LSE row identifies the expected LSE owner. Equal-length wrong rows continue to
identify their actual wrong owner.

| Finding | Disposition | Remediation status |
|---|---|---|
| B-RELEASE-HIGH-001: public bridge admits represented frozen/thawing and snow-retained-liquid-only state | Accepted | Remediated; focused PASS |
| A-RELEASE-HIGH-002: finite same-store demand accumulation can overflow and produce non-proportional authorization | Accepted | Remediated; focused PASS |

The public bridge now returns contextual E004 before authorization or callback
for snow runtime/carry, retained snow liquid, frost runtime/carry, or positive
production-layer frozen depth/water. The poison matrix proves callback
non-invocation and byte-identical production state. Demand accumulation and
every proportional-allocation intermediate now fail closed on nonfinite
results; retained arbitration reconstruction independently repeats the guard.

## Closure review and evidence findings

| Finding | Disposition | Remediation status |
|---|---|---|
| Rust closure review: exact workspace all-feature Clippy failed on three package-owned integration-test lints | Accepted | Remediated; exact workspace Clippy PASS |
| Rust closure review: exact-head full-workspace PASS absent | Accepted | Remediated; 2,783/2,783 PASS at `74d512f44` |
| Heavy attempt 3: Stage-0 source scan aliases admitted LSE crate name with protected meteorology module | Accepted | Remediated; exact module-path guard and full-suite PASS |

The hydrology closure review at `ab703c83a` returned PASS with no material
finding. The three closure findings above are evidence and test-governance
defects only; their correction changes no constitutive source, authority,
model identity, candidate, owner, or production selector behavior.

## Terminal Rust review findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-RUST-HIGH-001: nonterminal independent thermal deletion reports shifted actual receiver | Accepted | Remediated at `82bfdc3a0`; focused PASS |
| A-TERMINAL-RUST-HIGH-002: duplicated closure tolerance scale can overflow and accept wrong finite values | Accepted | Remediated at `82bfdc3a0`; focused PASS |

Deletion attribution now first identifies a missing member of the unique
expected sequence; present replacements and reorders continue to retain the
actual offending identity. Checked unit-aware arithmetic rejects nonfinite or
nonzero-underflowed conversion, sum, difference, scale and tolerance values
before a closure predicate can pass. No tolerance, source operand, authority,
model identity or production selector changed.

## Terminal arithmetic-precedence re-review finding

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-REREVIEW-HIGH-001: checked-close indeterminacy collapses into E010/E011 and receiver aggregation remains unchecked | Accepted | Remediated at `3b9e5ed13`; focused PASS |

Checked comparison is now propagated as a tri-state at every caller. An
arithmetic failure returns contextual E003 with available identity and rollback
hashes; only a finite `Some(false)` result returns the appropriate closure code.
All cited receiver divisions and accumulations use the same checked arithmetic.

## Final ingress-precedence review finding

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-FINAL-HIGH-001: independent E010 closure preempts producer attribution/routing E009 | Accepted | Remediated at `47f959b43`; focused PASS |

Ingress candidate validation now preserves the canonical branch order: E003
domain/arithmetic preflight, immutable producer reconstruction and attribution
as E009, then independent closure as E010. The three branches have distinct
public poisons and exact rollback assertions.

## Multi-record closure review findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-CLOSURE-HIGH-001: earlier finite E010 hides later-store E003 | Accepted | Remediated at `ee240618c`; focused PASS |
| A-TERMINAL-CLOSURE-HIGH-002: later producer E009 mismatch falls back to first store context | Accepted | Remediated at `ee240618c`; focused PASS |

The arithmetic preflight now scans the complete candidate without executing or
short-circuiting on finite closure comparisons. Producer comparison localizes
the exact first structural mismatch across every record, ledger and WB14 row;
no configured-first fallback remains.

## Aggregate projection and deletion review findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-CLOSURE2-HIGH-001: preflight omits checked per-OFE aggregate comparison | Accepted | Remediated at `86ddb8aa2`; focused PASS |
| A-TERMINAL-CLOSURE2-HIGH-002: positional producer comparison misattributes nonterminal deletion | Accepted | Remediated at `86ddb8aa2`; focused PASS |

Preflight and final closure now consume the same projected per-key and per-OFE
maps. Producer sequence comparison is membership-aware before positional
replacement/reorder handling for every canonical record class.

## Independent parcel-join review findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-CLOSURE3-HIGH-001: final closure omits per-source-parcel enthalpy comparison | Accepted | Remediated at `636dd36be`; focused PASS |
| A-TERMINAL-CLOSURE3-HIGH-002: routed failure context uses origin rather than current basis OFE | Accepted | Remediated at `636dd36be`; focused PASS |
| A-TERMINAL-CLOSURE3-MEDIUM-003: store arithmetic duplicated between preflight and final validation | Accepted | Remediated at `636dd36be`; focused PASS |

Per-source mass and enthalpy, OFE aggregate enthalpy, routed destination
identity, and store arithmetic now flow through shared projections and a shared
comparison disposition. Replacement/reorder controls retain the actual row.

## Constitutive mixed-enthalpy and source-identity findings

| Finding | Disposition | Remediation status |
|---|---|---|
| B/A-TERMINAL-CLOSURE4-CRITICAL-001: production replaces canonical interval `h_mix` with source-specific temperature | Accepted | Remediated at `e19bcdbcf`; focused PASS |
| A-TERMINAL-CLOSURE4-HIGH-002: zero frozen source identity/cardinality fails open | Accepted | Remediated at `e19bcdbcf`; focused PASS |
| A-TERMINAL-CLOSURE4-HIGH-003: multi-tile aggregate failure fabricates first-tile context | Accepted | Remediated at `e19bcdbcf`; focused PASS |

Canonical interval mixing is restored without tolerance or model change. Raw
source identity and enthalpy remain independently frozen; post-mix attribution
uses the single accepted `h_mix`. Aggregate failures expose only identities
that are actually known.

## Chronological closure and source-support findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-CLOSURE5-CRITICAL-001: independent closure collapses chronological `h_mix,b` to one whole-OFE mixture | Accepted | Remediated at `c4114fc8c`; focused PASS |
| A/B-TERMINAL-CLOSURE5-HIGH-002: frozen source support hardcoded and identity depends on caller order | Accepted | Remediated at `c4114fc8c`; focused PASS |

Independent closure now reconstructs the canonical chronological support
partition and per-window mixture from actual frozen source operands. Source
identity is canonicalized independently of caller order; production physics is
unchanged.

## Window identity, routed expectation and domain findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-CLOSURE6-CRITICAL-001: join key drops window identity and routed expectation copies actual receipt | Accepted | Remediated at `b5453e7d8`; focused PASS |
| A-TERMINAL-CLOSURE6-HIGH-002: support/mass/temperature domain violations miss global E003 precedence | Accepted | Remediated at `b5453e7d8`; focused PASS |

Window/disposition identity is explicit. Expected local and multi-hop routed
segments are independently derived from frozen inputs/topology. Domain
preflight covers every frozen and actual support/amount/temperature before
producer or closure comparison.

## Independent partition and recipient-identity findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-CLOSURE7-CRITICAL-001: expected nonrouted partition consumes actual receipts | Accepted | Remediated at `c3fdeca50`; focused PASS |
| A-TERMINAL-CLOSURE7-HIGH-002: exact current/recipient tile identity absent | Accepted | Remediated at `c3fdeca50`; focused PASS |
| A/B-TERMINAL-CLOSURE7-HIGH-003: routed descendant retains pre-route kind | Accepted | Remediated at `c3fdeca50`; focused PASS |
| A/B-TERMINAL-CLOSURE7-HIGH-004: raw Q not joined to mass and specific enthalpy | Accepted | Remediated at `c3fdeca50`; focused PASS |
| A-TERMINAL-CLOSURE7-MEDIUM-005: closure-module WARN lacks split intent | Accepted | Remediated in line-count governance |

Expected nonrouted and routed partitions are wholly independent of actual
receipts. Complete current/recipient identity, canonical routed kind and raw
mass/enthalpy relations are first-class closure seams.

## Persistent endpoint join finding

| Finding | Disposition | Remediation status |
|---|---|---|
| B-TERMINAL-CLOSURE8-CRITICAL-001: independently replayed final stores and WB14 continuation are discarded before persistent-state/restart join | Accepted | Remediated at `862f26bb7`; focused PASS |

The receipt-free replay now returns exact expected ending store and continuation
values. Independent validation joins them to the persistent owner, accepted
transaction lineage and strict restart digest before the candidate can pass.

## Partition taxonomy and routed-order findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-CLOSURE8-HIGH-002: partition structural mutations are misclassified E003 and continuation cross-field bounds are incomplete | Accepted | Remediated at `6e203beec`; focused PASS |
| A-TERMINAL-CLOSURE8-MEDIUM-003: duplicated unnamed constants/order definitions and missing multi-kind routed vector create drift risk | Accepted | Remediated at `6e203beec`; focused PASS |

Arithmetic/domain validation now exhaustively precedes membership-aware
producer identity errors. Shared named physical constants remove transcription
drift while the independent algorithm remains separate; a mixed-kind routed
overlap vector freezes chronological ordering and mixture behavior.

## Ending context and exact identity/vector findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-CLOSURE9-HIGH-001: ending-state aggregate/structural failures fabricate first-OFE context | Accepted | Remediated at `aacf181d7`; focused PASS |
| B-TERMINAL-CLOSURE10-MEDIUM-001 / A-TERMINAL-CLOSURE9-MEDIUM-002: canonical parcel order/source IDs remain duplicated and mixed-route outputs are not frozen | Accepted | Remediated at `aacf181d7`; focused PASS |
| A-TERMINAL-CLOSURE9-MEDIUM-003: 2,324-line ingress test module incorrectly marked PASS without WARN rationale/split intent | Accepted | Remediated in line-count governance; no runtime effect |

Aggregate context now uses typed absence rather than an invented OFE, and
membership failures identify the exact available or missing key. Canonical
non-arithmetic identity/order is shared, while a bit-frozen, nondegenerate
fixture protects the independent physical implementations from coordinated
ordering drift.

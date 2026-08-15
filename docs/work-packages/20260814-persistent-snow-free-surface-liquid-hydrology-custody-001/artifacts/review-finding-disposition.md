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

## Cardinality-aware offender-context findings

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-CLOSURE10-HIGH-001: equal-length ending replacement reports missing expected rather than actual replacement identity | Accepted | Remediated at `2dfd0af64`; focused PASS |
| A-TERMINAL-CLOSURE10-MEDIUM-002: complete large-file WARN inventory is absent | Accepted | Remediated in line-count governance; no runtime effect |

The ending join now distinguishes missing, excess, replacement and reorder by
cardinality and sequence direction. Tests bind exact context availability and
rollback hashes at every structural position.

## Line-count inventory finding

| Finding | Disposition | Remediation status |
|---|---|---|
| B-TERMINAL-CLOSURE12-MEDIUM-001: duplicate `runoff.rs` rows carry conflicting dispositions | Accepted | Remediated by removing the obsolete row; Markdown lint PASS |

## Exact-head terminal-review findings at `87b187b19`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-87B-HIGH-001: proportional authorization can underflow and bias the final canonical request | Accepted | Checked arithmetic and tiny-positive poison implemented; focused PASS |
| A-TERMINAL-87B-HIGH-002: public receiver validation accepts incomplete or identity-invalid envelopes | Accepted | Expected topology is frozen and hashed; empty/missing/duplicate/reordered/rekeyed poisons implemented; focused PASS |
| A/B-TERMINAL-87B-HIGH-003: receiver and record failures escape canonical taxonomy, context, hashes, and precedence | Accepted | E003/E009/E010 contextual paths and global arithmetic preflight implemented; focused PASS |
| A-TERMINAL-87B-MEDIUM-004: exact-head terminal evidence absent | Accepted | Pending fresh exact-byte reviews and heavy rerun after this remediation commit |

No `87b187b19` finding is rejected, deferred, or moved to another package.

## Exact-head terminal-review findings at `2b713d659`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-2B7-HIGH-001: canonical-last tiny request bypasses checked proportional arithmetic | Accepted | Every row is checked before remainder assignment; both caller orders pass focused poisons |
| A-TERMINAL-2B7-HIGH-002: frame and serialized-byte failures discard canonical context | Accepted | Attachment errors are transparent/context-completed; noncanonical bytes retain parsed owner/transaction/hash; focused PASS |
| A-TERMINAL-2B7-HIGH-003: receiver E011 can preempt later E003 | Accepted | Global receiver numeric preflight precedes structural/closure checks; mixed poison PASS |
| A/B-TERMINAL-2B7-MEDIUM-004: receiver hashes use ambiguous unframed concatenation | Accepted | One tagged, length-framed, cardinality-framed encoder drives all receiver hashes; collision poisons PASS |
| B-TERMINAL-2B7-HIGH-005: unified public entry/protocol errors remain generic | Accepted | E002/E005/E006 contextual failures with rollback hashes implemented; focused PASS |
| A/B-TERMINAL-2B7-EVIDENCE-006: exact-head heavy and terminal-diff evidence incomplete | Accepted | Historical whitespace corrected; fresh exact-head heavy run pending after remediation commit |

No `2b713d659` finding is rejected, deferred, or moved to another package.

## Exact-head terminal-review findings at `0e5262b4b`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-0E5-HIGH-001: canonical-last remainder violates exact proportional authorization and creates key-order priority | Accepted | Remainder substitution removed; every row stores the checked exact `D_i*S/D_sum`; three-equal-demand bit-vector and reverse-order poison PASS |
| A/B-TERMINAL-0E5-HIGH-002: unified snapshot, binding, partition, authorization and protocol paths expose generic or misclassified errors | Accepted | Canonical E002/E003/E005/E006 failures now carry phase, exact available identity and rollback hashes; focused PASS |
| A/B-TERMINAL-0E5-HIGH-003: structural E011 can preempt nonfinite E003 | Accepted | Global request, protocol and sealed-receiver numeric preflight runs before structural validation; mixed-precedence poisons PASS |
| A-TERMINAL-0E5-HIGH-004: attempted receiver hashes omit thermal operands and beginning hydrology snapshot | Accepted | Framed hashes now bind ground-heat credit, infiltration enthalpy, ending temperature and beginning-snapshot identity; mutation poisons PASS |
| A-TERMINAL-0E5-MEDIUM-005: first invalid attachment reports a false beginning-owner hash | Accepted | First attachment now reports `beginning_owner_sha256=None`; existing-owner replacement retains the real beginning hash; focused PASS |
| A-TERMINAL-0E5-EVIDENCE-006: interrupted-run narrative, terminal diff and line-count inventory are incomplete | Accepted | Failed wrapper commands truthfully recorded; historical EOF whitespace removed; all touched Rust files at or above 2,000 lines inventoried; complete heavy rerun remains pending after fresh PASS reviews |

No `0e5262b4b` finding is rejected, deferred, or moved to another package.

## Exact-head terminal-review findings at `dd8127b04`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-DD8-HIGH-001: rounded proportional rows can jointly exceed immutable supply | Accepted | SC-SURFACELIQUID v6 binds one symmetric common binary64 representability scale; no last-key repair; ordinary-scale counterexample, reverse order and exact `F=A` ending PASS |
| A-TERMINAL-DD8-HIGH-002: raw attempted hashes trust stale declared digests or are absent | Accepted | Complete raw configuration/state fields and parser bytes are framed; stale-digest NaN payload and whitespace collisions reject with distinct attempted hashes; beginning hash is actual accepted state or absent |
| A/B-TERMINAL-DD8-HIGH-003: public error taxonomy and precedence are inconsistent | Accepted | Public owner and unified paths enforce E002 identity, E003 arithmetic/domain, E005 cardinality and E006 finite bounds; mixed-defect poisons PASS |
| A-TERMINAL-DD8-HIGH-004: finite receiver equation mismatch is incorrectly E011 | Accepted | Independent soil, thermal and LSE equation mismatch is contextual E010; complete-owner/rollback mismatch remains E011 |
| A-TERMINAL-DD8-HIGH-005: canonical conversion loses exact later-row context | Accepted | Existing canonical context is preserved; structural preflight identifies the actual later offending key; no first-row fallback remains |
| A-TERMINAL-DD8-EVIDENCE-006: exact-head heavy evidence and line-count inventory incomplete | Accepted | Inventory reconciled below; heavy rerun remains pending after fresh PASS review |

No `dd8127b04` finding is rejected, deferred, or moved to another package.

## Exact-head terminal-review findings at `fe6cc4bd5`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-FE6-HIGH-001: unified request/protocol precedence differs from canonical order | Accepted | Identity/structure and numeric passes now enforce E002, E003, E005, E006 order; mixed poisons PASS |
| A-TERMINAL-FE6-HIGH-002: receiver E011 can preempt derived E003 | Accepted | Receipt aggregation/conversion and receiver arithmetic dry-run before expectation/topology/rollback E011; mixed poisons PASS |
| A-TERMINAL-FE6-HIGH-003: public rollback hashes absent or trust declared state digest | Accepted | Beginning hashes use actual raw accepted state; attempted request/protocol/candidate framing is complete; unified mapping retains both hashes |
| A-TERMINAL-FE6-MEDIUM-004: redundant caller-order `D_sum` changes overflow context | Accepted | Removed; complete-key-order checked sum is sole arithmetic; reverse-order overflow context PASS |
| A-TERMINAL-FE6-MEDIUM-005: restart `W>W_max` is E006 | Accepted | Classified as domain E003 with identity-before-domain poison |
| A/B-TERMINAL-FE6-MEDIUM-006: later/cardinality binding and ingress context is fabricated or incomplete | Accepted | Exact later/excess record identity or typed absence; ingress always retains configured owner; multi-OFE and direct poisons PASS |
| A-TERMINAL-FE6-EVIDENCE-007: exact-head heavy evidence absent | Accepted | Pending after fresh PASS review |

No `fe6cc4bd5` finding is rejected, deferred, or moved to another package.

## Exact-head terminal-review findings at `2e32a8a0e`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-2E3-HIGH-001: full-infiltration mass/depth round-trip can create a negative source remainder and the independent replay lacks a raw-mass join | Accepted | Remediated: exact full-infiltration identity preserves raw source mass; independent source/OFE raw-mass joins and bit-frozen poisons PASS |
| A-TERMINAL-2E3-HIGH-002: mixed public failures do not consistently observe canonical E001 through E011 precedence | Accepted | Remediated: unified and ingress boundaries enforce explicit E002 through E008 order; mixed precedence poisons PASS |
| A-TERMINAL-2E3-EVIDENCE-003: exact-head heavy evidence is absent after material source corrections | Accepted | Pending fresh PASS reviews and complete external-target heavy rerun |

No `2e32a8a0e` finding is rejected, deferred, or moved to another package. The
independent hydrology/ownership review of the same bytes returned PASS, but it
does not override the Rust correctness HOLD.

## Exact-head terminal-review findings at `f249431d4`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-F249-HIGH-001: configuration/state record validation can return an earlier E003 before a later E002 identity defect | Accepted | Remediated: whole-set identity preflight precedes every record-domain pass and outer public validation; cross-row/order poisons PASS |
| A-TERMINAL-F249-MEDIUM-002: receiver checked aggregation is duplicated between preflight and final construction | Accepted | Remediated: one shared checked aggregation result serves preflight and final operand freezing; drift/poison tests PASS |

No `f249431d4` finding is rejected, deferred, or moved to another package. The
fresh hydrology/ownership review returned PASS; the interrupted full-workspace
attempt is not terminal evidence.

## Exact-head terminal-review finding at `7b208bb26`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-7B2-MEDIUM-001: public unified execution and finalization do not apply global precedence across independent input sets | Accepted | Remediated: category-wide identity/domain/cardinality/bound preflights span all public inputs; cross-set permutation poisons PASS |

No `7b208bb26` finding is rejected, deferred, or moved to another package. The
fresh hydrology/ownership review returned PASS; an accidental broad run with
584 passes and three SIGINT results is retained as interrupted non-evidence.

## Exact-head terminal-review findings at `c9524729a`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-C952-HIGH-001: child enthalpies do not use authoritative parent Q and canonical-last subtraction remainders; independent replay duplicates the drift | Accepted | Remediated: exact parent-Q splitting at every stage, independent frozen-operand reconstruction and one-ULP E010 poison PASS |
| A/B-TERMINAL-C952-HIGH-002: condensation T/h E009 omits available OFE/tile/surface/source context | Accepted at higher hydrology severity | Remediated: credit-contextual E009 carries complete identity and raw rollback hashes; both records/fields PASS |
| A-TERMINAL-C952-MEDIUM-003: nonfinite production-lane area attaches and is later reported as E002 | Accepted | Remediated: all-lane identity then domain preflight rejects NaN/±infinity as contextual E003; position/precedence poisons PASS |

No `c9524729a` finding is rejected, deferred, or moved to another package.

## Exact-head terminal-review findings at `10b914da1`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-10B-HIGH-001: resource and ingress public seams allow configuration/state E003 to mask caller-supplied E002 identity | Accepted | Remediated in worktree: whole-envelope E001/E002 preflight and seven mixed-poison permutations PASS |
| B-TERMINAL-10B-MAJOR-001: receiver closure errors omit applicable configured surface/source identity | Accepted | Remediated in worktree: authoritative receiver identity mapping is frozen, digest-bound and propagated; context/digest poisons PASS |

No `10b914da1` finding is rejected, deferred, or moved to another package.
Fresh review is pending on the corrected exact bytes.

## Exact-head terminal-review findings at `73f22169a`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-73F-HIGH-001: arithmetic scan can stop at parcel E010 before later E003 | Accepted | Remediated: identity-only joins are nonterminal during arithmetic preflight; composed poison PASS |
| A-TERMINAL-73F-HIGH-002: derived receiver E003 retains hydrology beginning digest | Accepted | Remediated: unique owner/kind rollback or typed absence; LSE/thermal/hydrology/missing/duplicate tests PASS |
| A/B-TERMINAL-73F-HIGH-003: nested snow/albedo/layer/frost/carry domains bypass E003 | Accepted | Remediated: complete shared production winter validator and exhaustive category matrix PASS |

No `73f22169a` finding is rejected, deferred, or moved to another package.
Fresh review is pending on the corrected exact bytes.

## Exact-head terminal-review findings at `83e1ee296`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-83E-HIGH-001: partition E009 can preempt later projection E003 in candidate validation | Accepted | Remediated: arithmetic-only preflight precedes partition identity; late-overflow plus reorder poison PASS |
| A-TERMINAL-83E-HIGH-002: LSE/thermal E003 owner is paired with hydrology beginning digest | Accepted | Remediated: unique owner/kind rollback hash or typed absence in sealed and post-ingress paths; focused PASS |
| B-TERMINAL-83E-HIGH-001: negative/nonfinite snow scalars bypass positivity-only snow predicate | Accepted | Remediated: exhaustive finite/nonnegative E003 preflight before E004; 80-case public poison PASS |

No `83e1ee296` finding is rejected, deferred, or moved to another package.
Fresh review is pending on the corrected exact bytes.

## Exact-head terminal-review findings at `a5c2243e6`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-A5C-HIGH-001: ingress candidate validation allows closure E003 to mask whole-input E001/E002 | Accepted | Remediated: shared identity-only public preflight precedes closure arithmetic; mixed poison PASS |
| A-TERMINAL-A5C-HIGH-002: temporal mass children lack canonical-last remainder and closure duplicates the drift | Accepted | Remediated: exact parent remainder plus frozen-parent raw mass reconstruction; five-window and one-ULP poisons PASS |
| A-TERMINAL-A5C-MEDIUM-003: exact-head line-count evidence is stale | Accepted | Remediated: complete corrected-worktree inventory and extracted helper/test modules recorded |
| B-TERMINAL-A5C-MEDIUM-001: sealed LSE numeric E003 reports hydrology owner | Accepted | Remediated: exact ground-surface request owner and applicable configured context with exact hashes; focused PASS |

No `a5c2243e6` finding is rejected, deferred, or moved to another package.
Fresh review is pending on the corrected exact bytes.

## Exact-head terminal-review findings at `85358c9b2`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-853-HIGH-001: standalone receiver failures can publish hydrology rollback provenance for LSE/soil-thermal owners or drop the implicated owner | Accepted | Remediated in worktree: complete rollback rows are digest-bound and one canonical owner-kind/owner-ID resolver supplies exact beginning hash or typed absence; E003/E010/E011 poisons PASS |
| A-TERMINAL-853-HIGH-002: malformed frost counts, indices, order and membership can fall through as E004 | Accepted | Remediated in worktree: complete structural winter validation and state/runtime poison matrix PASS |
| A-TERMINAL-853-HIGH-003: unified entry preflights only outer identity before E003 checks and callback execution | Accepted | Remediated in worktree: complete input-only ingress identity preflight precedes request/winter arithmetic and callback; mixed poisons PASS |

No `85358c9b2` finding is rejected, deferred, or moved to another package. The
fresh hydrology/ownership review returned PASS; its interrupted broader run is
retained as non-evidence.

## Exact-head terminal-review findings at `5d298ca1c`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-5D2-HIGH-001: standalone sealing accepts incomplete/substituted rollback owner sets and can attribute missing thermal to LSE | Accepted | Remediated in worktree: exactly one canonical rollback row is required for each LSE, hydrology and soil-thermal owner; focused PASS |
| A/B-TERMINAL-5D2-HIGH-002: frost fine/shadow membership is not reciprocal | Accepted | Remediated in worktree: persisted and runtime fine/shadow containers require exact reciprocal membership, order and count; focused PASS |
| A/B-TERMINAL-5D2-HIGH-003: unified E002 preflight omits request/source mapping and complete attempted-input provenance | Accepted | Remediated in worktree: configured source binding precedes E003 and callback execution; attempted hashes bind ingress, WB14 and soil-source mappings; rollback reports the computed snapshot; focused PASS |
| A-TERMINAL-5D2-LOW-004: two review artifacts add a blank line at EOF | Accepted | Remediated; terminal base diff hygiene will be rerun |

No `5d298ca1c` finding is rejected, deferred or moved to another package.

## Exact-head terminal-review findings at `73299b981`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-732-HIGH-001: soil request OFE is not bound to the configured production lane | Accepted | Remediated in worktree: exact OFE/lane-index/lane-ID/layer binding precedes E003 and callback; equal-layer cross-OFE poison PASS |
| A/B-TERMINAL-732-HIGH-002: standalone sealing accepts equal-but-unbound LSE rollback lineage | Accepted | Remediated in worktree: the sole public constructor consumes independent receiver expectations and binds all three exact beginning digests; forged equal-LSE poison PASS |
| A-TERMINAL-732-MEDIUM-003: several unified public failures omit complete attempted-input provenance | Accepted | Remediated in worktree: complete attempted framing is computed once and threaded through source-map, winter, exact-one and authorization failures; sensitivity poisons PASS |
| A-TERMINAL-732-TEST-004: frost layer indices beyond production cardinality lack a poison | Accepted | Remediated in worktree: shadow and fine layer indices beyond production cardinality return E003 before E004; poisons PASS |

No `73299b981` finding is rejected, deferred or moved to another package.

## Exact-head terminal-review findings at `e33f4cdd4`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-E33-HIGH-001: arbitrary callback errors escape the canonical public failure envelope | Accepted | Remediated in worktree: all callback variants canonicalize at ResourceCandidate with actual beginning and complete attempted hashes; poison matrix PASS |
| A-TERMINAL-E33-HIGH-002: receiver expectations are validated only after callback execution | Accepted | Remediated in worktree: complete expectation identity/topology/lineage validation precedes authorization and callback; callback-zero matrix PASS |
| A-TERMINAL-E33-MEDIUM-003: attempted-input hash omits receiver expectations and caller expected snapshot | Accepted | Remediated in worktree: v3 framing binds every raw expectation field, ordered topology, canonical digest, actual snapshot and caller expected snapshot; mutation matrix PASS |

No `e33f4cdd4` finding is rejected, deferred or moved to another package. The
fresh hydrology/ownership review of the same bytes returned PASS, but it does
not override the Rust correctness HOLD.

## Exact-head terminal-review findings at `fc65b2819`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-FC65-HIGH-001: surface-liquid callback variants retain wrong boundary phase/context | Accepted | Remediated in worktree: raw and canonical variants rebind to ResourceCandidate with known transaction and complete hashes while retaining applicable row identity; matrix PASS |
| A-TERMINAL-FC65-HIGH-002: nested land-surface error taxonomy is collapsed to E003 | Accepted | Remediated in worktree: exhaustive centralized mapping preserves E001/E002/E003/E004/E010/E011 classes; 19-case matrix PASS |
| A/B-TERMINAL-FC65-HIGH-003: v3 attempted framing discards raw malformed configuration/state attempt | Accepted | Remediated in worktree: raw configuration/state attempts are framed-joined with unified v3; stale-digest finite/nonfinite mutation poisons PASS |
| A-TERMINAL-FC65-HIGH-004: pre-callback receiver expectations omit configured infiltration thermal layer | Accepted | Remediated in worktree: exact configured infiltration layer must be first in every expected tile before callback; wrong/deleted/replaced poisons PASS |
| A-TERMINAL-FC65-MEDIUM-005: duplicated error-taxonomy translation has drifted | Accepted | Remediated in worktree: one boundary-aware taxonomy serves receiver and unified callback paths; strict Clippy PASS |

No `fc65b2819` finding is rejected, deferred or moved to another package.

## Exact-head terminal-review findings at `fb89e5a55`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-FB89-HIGH-001: full configuration E003 validation precedes request identity E002 | Accepted | Remediated in worktree: schema/identity-only configuration preflight precedes complete request E002; full numeric validation follows with raw+v3 attempt join; dual poison PASS |
| A-TERMINAL-FB89-MEDIUM-002: combined LSE error variants prevent semantically complete canonical taxonomy | Accepted | Remediated in worktree: LSE owns typed topology/water error classes and exhaustive class(); all production constructors migrated and real semantic vectors PASS |

No `fb89e5a55` finding is rejected, deferred or moved to another package.

## Exact-head terminal-review findings at `3ac61997d`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-3AC-HIGH-001: complete unified E002 identity envelope remains split around config/state E003 | Accepted | Remediated: configuration/state schema and identity, request identity, ingress identity, configured-source mapping, and outer transaction/snapshot identity all precede full E003 numeric validation; eight cross-input poisons prove E002, complete raw-plus-unified attempted hashes, and zero callback execution |
| A/B-TERMINAL-3AC-HIGH-002: standalone sealing accepts an entirely empty D/A/F protocol | Accepted | Remediated: independent receiver expectations require at least one exact ground request identity for every expected `(OFE,tile)` and reject missing/extra coverage; `WaterProtocol::validate()` retains exact request/authorization/use cardinality; empty, missing, extra, and zero-amount-complete vectors PASS |
| A-TERMINAL-3AC-MEDIUM-003: real LSE negative D/A/F/credit operands construct E003 rather than E006 | Accepted | Remediated: water-specific finite/bound helpers preserve nonfinite E003 and classify negative request/authorization/use/credit plus zero credit as E006 through the real public callback translation |
| A-TERMINAL-3AC-RISK-004: typed public error-shape compatibility and duplicated thermodynamic constants need explicit disposition | Accepted for disposition | Dispositioned: the typed public error-shape change is an intentional package-authorized source break; no compatibility fallback is admitted because it would restore ambiguous taxonomy. LSE and direct-runtime constants remain bit-identical (`4218.0`, `273.15`); duplication is retained as a documented maintenance risk because centralization would broaden scientific-authority coupling outside this correction |

No `3ac61997d` finding is rejected, deferred or moved to another package.

## Exact-head terminal-review findings at `413c0c32a`

| Finding | Disposition | Remediation status |
|---|---|---|
| A/B-TERMINAL-413-HIGH-001: configuration/state declared digest E002 can be masked by numeric E003 | Accepted | Remediated: public attachment, snapshot, standalone ingress and unified entry use staged structural identity, contextual cross-input identity, declared-digest identity and numeric-domain validation; isolated configuration/state stale-digest-plus-NaN vectors return E002 with callback zero and unchanged bytes |
| A-TERMINAL-413-HIGH-002: ingress-identity attempted hash omits raw configuration/state bytes | Accepted | Remediated: ingress E002 contextualization joins the raw snapshot attempt with the unified projection; same stale digest and ingress defect with distinct raw NaN payloads produce distinct attempted hashes |

No `413c0c32a` finding is rejected, deferred or moved to another package.

## Exact-head terminal-review findings at `15a110ece`

| Finding | Disposition | Remediation status |
|---|---|---|
| B-TERMINAL-15A-HIGH-001: attachment lane numeric E003 can mask later state identity/digest E002 | Accepted | Remediated: frame cross-input identity and production-lane numeric validation are separate; configuration/frame/state structure and declared digests precede lane-domain E003; lane-NaN × state-key/stale-digest vectors return E002 with complete hashes and unchanged attachment/lane bytes |

The Rust review returned PASS with no material finding. No `15a110ece` finding
is rejected, deferred or moved to another package.

## Exact-head terminal-review findings at `4360daef1`

| Finding | Disposition | Remediation status |
|---|---|---|
| A-TERMINAL-436-MEDIUM-001: contract-critical configuration/state/frame identity validation remains substantially duplicated | Accepted | Remediated: configuration/state constructors and full validation reuse the canonical structural/digest preflights then domain-only passes; attachment and independent receiver validation share one typed first-mismatch projection while retaining path-specific completion; exhaustive mismatch and identity-plus-domain anti-drift vectors PASS |

The hydrology review returned PASS with no material finding. No `4360daef1`
finding is rejected, deferred or moved to another package.

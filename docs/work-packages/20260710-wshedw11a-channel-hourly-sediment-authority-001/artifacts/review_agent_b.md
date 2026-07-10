# Review Agent B

Status: EXECUTED
Evidence mode: Ran + Static — Ran: `tools/check_sc_binding_exposure.py` lint (PASS, exit 0), `git diff` / `git diff --word-diff` of the amendment, `git status --porcelain` scope check, `ls` verification of cited vendored reference files, `grep` cross-reference checks into SC-SED-001 and SC-INFILE-HBP-001. Static: full read of SC-ROUTE-001 rev 51, the authoring procedure, the artifact spec, science-contracts AGENTS.md, and the W11A package.md. No external science PDFs were opened; source fidelity is out of this review's lens.
Reviewer: Claude subagent (independent review B), 2026-07-10

## Conformance summary (verified before findings)

- Ran: `.venv/bin/python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-ROUTE-001.md` → `PASS ... 7 binding exposure row(s) fully consolidated`, exit 0.
- Ran: `git status --porcelain` over `crates/`, `src/`, `tools/`, `Cargo.*` shows no code changes; the working tree touches only `SC-ROUTE-001.md` and `index.md`. No Rust, no HBP schema, no impoundment code — package Excluded Scope respected on the write surface.
- Ran: word-diff proves INV-ROUTE-005 clauses (a)-(d) are byte-identical; only (e) was rewritten, and the rewrite is a tightening (the carried per-hour inlet array gains a live consumer; the closing prohibition now binds "on both lanes"). INV-ROUTE-007/009/010 and guard-map rows 001-014 are untouched. No weakened invariant found.
- Ran: all newly cited anchor files exist on disk (`references/vendorable/creams/312-ch3.pdf|.md`, `references/vendorable/kineros/703.pdf|.md`, `references/vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf`, `references/50201000/chap14.pdf`).
- Ran: cross-references are accurate — `SC-SED-001#GAP-SED-008` really does scope the serialized `S_h` as total-mass with the day-level class blend (SC-SED-001.md:608); `SC-INFILE-HBP-001` §3a carries `hourly_sediment_mass_kg[24]` (kg) and §8.5 the integral-closure intake rule.
- Static: derivation order conforms — chap13/CREAMS/ARS-77 primary, `REF-ROUTE-GULLY-STATE` explicitly labeled "Secondary static-code provenance" with the pinned baseline SHA, and `REF-ROUTE-JIMF2023-CARRY` explicitly graded "maintainer-intent evidence (graded corroboration, not physics authority)". Every new invariant has a guard-map row with enforcement path and failure behavior. Front matter (`contract_version: 51`, `last_reviewed: 2026-07-10`) is consistent with the registry row and the revision-history entry, and the revision-history row accurately enumerates the change set.

## Findings

### B-1 (High) — Interval-lane activation is unreachable for inlets with upstream channel dependencies under a strict reading of INV-ROUTE-005(a)/(c)

Reference: SC-ROUTE-001.md:127 (INV-ROUTE-005 (a)/(c)), :594-602 (Activation), :137-138 (INV-ROUTE-015/016 upstream-channel ingress), :612 (operand table row "Upstream-channel sediment ingress").

INV-ROUTE-005(a) requires "no upstream dependency element lacks **channel-hourly surfaces**," and (c) fail-closes "an hourly hillslope contributor feeding an inlet with dependency nodes that do not yet carry channel-hourly surfaces." The W11A lane makes channels emit **per-interval egress on the `dtchr` grid** — not hourly surfaces — and the amendment never states that same-grid interval egress satisfies the (a)/(c) dependency-authority requirement. Strictly read: branch (a) can never hold at an inlet with a channel dependency; the Activation rule ("every contributing element satisfies the INV-ROUTE-005(a) all-hourly authority") therefore can never hold for any downstream channel; the upstream-channel same-interval ingress provisions of INV-ROUTE-015/016 are dead letters. The pre-rev-51 (e) text ("until channels themselves carry hourly surfaces") anticipated an *hourly* channel surface; W11A delivered an *interval* one and did not reconcile (a)/(c).

Impact: the multi-channel network case — the core of W11 — either fails closed everywhere or requires the implementer to decide that interval egress "counts as" channel-hourly authority. That is precisely the executor science choice the package exit criterion prohibits.

Proposed disposition: amend INV-ROUTE-005(a)/(c) (or the Activation section, with a pointer from (a)/(c)) to state explicitly that an upstream channel or dependency node **on the active interval lane, publishing same-grid per-interval per-class egress, satisfies the dependency-authority requirement**, and that this is the only non-hourly dependency form that does. Also state what an interval-lane channel presents to a *non*-activated downstream consumer, or forbid that topology explicitly.

### B-2 (Medium) — Activation predicate is narrative-only binding authority; modality unstated; the anti-partial sentence literally contradicts the authorized event-scalar configuration

Reference: SC-ROUTE-001.md:594-602 (Activation), :127 (INV-ROUTE-005(e)), :179 (BEI-ROUTE-007 notes).

Three connected defects. (1) The activation predicate (`ipeak >= 3` AND all-hourly per INV-ROUTE-005(a)) lives only in addendum narrative. INV-ROUTE-005(e) and INV-ROUTE-015 both condition on the lane being "active"/"inactive" without defining the predicate; BEI-ROUTE-007's note claims "the INV-ROUTE-005(e) conditional selects between the event-scalar and interval lanes," but 005(e) contains no selection predicate — the binding selector is un-invarianted narrative (hidden authority; the lint cannot see it because the addendum row maps to INVs that merely *reference* an undefined state). (2) Modality: "activates only when..." states a necessary condition; no text makes activation **mandatory** when the predicate holds. A literal implementation that never activates the lane satisfies every invariant, silently no-op'ing the whole amendment. (3) "Partial activation (interval water with event-scalar sediment on the same channel, or vice versa) is invalid" — read literally, this invalidates every `ipeak >= 3` run without hourly authority (wave-routed interval water + event-scalar sediment), which is exactly the configuration the immediately preceding sentence authorizes ("All other configurations remain on the INV-ROUTE-005(e) event-scalar lane").  The intended meaning is clearly "an *eligible/activated* channel may not run half the lane," but the text says otherwise.

Impact: lane selection — the amendment's central binding decision — is under-determined and internally contradictory at the letter level; implementers and future lint tooling cannot bind to it.

Proposed disposition: promote the activation predicate into invariant text (INV-ROUTE-015 or a new clause of 005(e)) as a biconditional with explicit modality ("the interval lane is active if and only if `ipeak >= 3` and the inlet's INV-ROUTE-005(a) authority holds [as amended per B-1]; remaining on the event-scalar solve when the predicate holds is invalid"), and reword the anti-partial sentence to scope it to eligible/activated channels.

### B-3 (Medium) — INV-ROUTE-017 cites the wrong gap row for the no-re-erodible-bed-store rule

Reference: SC-ROUTE-001.md:139 ("deposition does not create a re-erodible bed store (GAP-ROUTE-013)"), :692-693.

GAP-ROUTE-013 is the end-of-grid storage-disposition decision; the no-re-erodible-bed-store limitation is **GAP-ROUTE-012**. INV-ROUTE-020(c) cites GAP-ROUTE-013 correctly; INV-ROUTE-017 does not.

Impact: a broken cross-reference inside a hard-fail invariant sends the implementer to the wrong labeled decision.

Proposed disposition: change the citation in INV-ROUTE-017 to GAP-ROUTE-012.

### B-4 (Medium) — Unit-system bridge for the interval lane is undeclared; new W11A symbols missing from Variables and Units

Reference: SC-ROUTE-001.md:95-118 (Variables and Units), :137-142, :293-295 (TOL-ROUTE-006..008), :604-613 (operand table); authoring procedure "Symbol Alias and Unit Governance Workflow" rules 2-5.

The interval lane straddles two unit systems with no declared crossing: `q1(it)` and the zero-flow floor are SI (`m^3 s^-1`), inlet/lateral masses and TOL-ROUTE-006/007 are kg, `dtchr` is seconds — while the Chapter-13 segment solve the lane invokes is declared in English units (`qt/qu/ql` in `ft^3 s^-1`, `qsed` in `lb ft^-1 s^-1`, TOL-ROUTE-005 in `lb ft^-1 s^-1`). "Interval inlet sediment flux = interval inlet class mass / `dtchr`" produces kg/s feeding a solve whose inlet flux operand is lb-based; the kg-denominated TOL-ROUTE-006 closure over an lb/ft/s solve implies a conversion point that is nowhere named, directional, or provenance-backed — the unit-governance workflow makes that fail-closed-until-declared. Separately, the amendment introduces symbols with no Variables-and-Units rows: `W_current`/`W_f`/`W_i`, `omega`, `t_star`, `(dW/dt)_i`, `e_m`, `rho_soil`, `d_ch`, `ntchr`, `wera`/`werb`, and the consumed `V_h`/`S_h` (units only recoverable from an anchor description). `rho_soil` is also semantically unpinned (bulk vs particle density; which soil-input parameter).

Impact: the implementer must invent the SI/English bridge and the widening-law units — an executor choice, and a classic 10x/unit-smell breeding ground the program has been burned by before.

Proposed disposition: add Variables-and-Units rows for the new symbols (with CREAMS-declared units for the widening family and an explicit `rho_soil` definition/source), and declare the named directional conversion(s) at the interval-operand boundary or explicitly re-declare the interval solve's working unit system.

### B-5 (Medium) — INV-ROUTE-019 geometry-mass consistency clause is binding but untestable as written

Reference: SC-ROUTE-001.md:141, :166 (guard row).

"Boundary-detached mass must be consistent with eroded geometry volume × soil bulk density (`d_ch = e_m / rho_soil`, Eq. [I-131] lineage)" carries no tolerance ID, and the INV-ROUTE-019 guard row enumerates only TOL-ROUTE-006/007/008 residuals plus suspended-pool carry — the geometry-mass check has no enforcement path, failure threshold, or test vector (the nine vector obligations do not exercise it).

Impact: a hard-fail invariant clause with no defined pass/fail criterion either goes unenforced or gets an implementer-invented threshold.

Proposed disposition: either assign it a tolerance (e.g. a TOL-ROUTE-009 per-interval geometry-mass residual) plus a guard-map mention and a test vector, or demote the clause to a stated derivation rule (the detached-mass *definition* is `eroded volume × rho_soil`) so it is constructive rather than checked.

### B-6 (Medium) — Widening-clock per-interval re-anchoring is an unlabeled refinement; binary erosion-time operand unbounded

Reference: SC-ROUTE-001.md:140 (INV-ROUTE-018), :636-646 (Widening Clock), :613 (erosion-time operand); contrast :138 (INV-ROUTE-016's explicit "labeled refinement" language).

The CREAMS law [I-133]-[I-135] is anchored at nonerodible-layer contact (fixed `W_i`, `t_i`, initial rate `(dW/dt)_i`). The addendum re-anchors every interval (`W_i := W_current`, `t - t_i := dtchr`, rate basis recomputed "at the interval hydraulics"). This is fully specified — implementable without ambiguity, and it usefully eliminates any persistent `(W_i, t_i)` store — but it is a *semantic modification* of the anchored law (the composed trajectory equals the anchored one only under constant coefficients), and unlike INV-ROUTE-016 it carries no labeled-refinement flag. Similarly, the erosion-time operand is binary (full `dtchr` when shear exceeds `taucr`, else zero), replacing the legacy fractional `timsh = tb·(1 - taucr/tau)`; at coarse `dtchr` this systematically overweights marginal-shear intervals, and no vector or note bounds the coarse-grid limit.

Impact: the amendment's honesty posture ("labeled refinement... never a return to the single event-peak solve") is complete for the solve form but incomplete for the widening clock; a source-fidelity reviewer or future auditor will read INV-ROUTE-018 as claiming direct CREAMS conformance it does not have.

Proposed disposition: add one sentence to INV-ROUTE-018 labeling the per-interval re-anchoring and the binary erosion-time operand as the interval-lane refinement of the anchored CREAMS law (with the same recorded-fallback posture), and state explicitly that no persistent `(W_i, t_i)` state exists beyond `W_current`.

### B-7 (Medium) — Test vector 1 (single-interval equivalence) is not implementable as stated

Reference: SC-ROUTE-001.md:650-653.

"Reproduces the event-scalar solve run at `qe = Q` with mass/duration operands matched — per-class egress equal within TOL-ROUTE-006." Which operands are "matched" is undefined: is the comparator's `durrof` forced to `dtchr`? Its `qsed_top` to interval mass / `dtchr`? Critically, the event-scalar solve's shear-time operand is the triangular surrogate (`tb = 2·rundur`, fractional `timsh`), which INV-ROUTE-018 bans on the interval lane — for a geometry-mutating interval the two solves cannot agree to `1e-9 kg` unless the comparator's internal operands are forced to the interval lane's values, at which point the vector tests the solve against itself.

Impact: the first implementation gate cannot be built without an executor decision about what the comparator computes — against the package exit criterion.

Proposed disposition: specify the comparator construction exactly (e.g. "the event-scalar solve with `durrof := dtchr`, `qsed_top := interval class mass / dtchr`, and erosion time := `dtchr`, i.e. the shared solve core minus interval sequencing"), or narrow the vector to a non-widening, below-critical-shear geometry regime where the surrogate's operand difference is provably inert.

### B-8 (Low) — "WS18-WS31 lanes" is an undefined lane name

Reference: SC-ROUTE-001.md:138 (INV-ROUTE-016), :163 (guard row), :622-623 (sequencing step 1b); contrast revision rows v20-v33 (WSHEDIMPL18-31) and GAP-ROUTE-009 ("WSHEDIMPL20-37").

The contract's own record names the migration amendments WSHEDIMPL18-31 and the runtime lanes WS20/WS21/WS23/WS24 (`ws20_*`/`ws21_*`/`ws24_*` symbols). "WS18-WS31" resolves to no defined artifact set in the contract (it echoes the package.md's shorthand, which itself wobbles between WS18-WS26 and WS18-WS31). Proposed disposition: rename to "the WSHEDIMPL18-41 migrated segment-solve lanes (WS20/WS21 runtime families)" or equivalent defined term.

### B-9 (Low) — "the INV-ROUTE-007 floor" slightly overstates the pre-existing constant's scope

Reference: SC-ROUTE-001.md:142 (INV-ROUTE-020(a)), :129 (INV-ROUTE-007), :361 (WS11 step 5).

INV-ROUTE-007 applies `1e-12` to the published outlet peak `qpo` for duration closure — not to per-interval `q1(it)`. Reuse is dimensionally coherent (both `m^3 s^-1`) and clause (b) frames it honestly as constant reuse ("the existing routed-closure constant"), but clause (a)'s "the INV-ROUTE-007 floor" implies a pre-existing per-interval floor that does not exist. Proposed disposition: reword (a) to "the routed-closure constant of INV-ROUTE-007 (`qpo` floor), applied here per interval."

### B-10 (Low) — Projection rule edge cases: zero-mass relative tolerance, unwritten formula, and grid-coverage ownership

Reference: SC-ROUTE-001.md:295 (TOL-ROUTE-008), :137 (INV-ROUTE-015), :142 (INV-ROUTE-020(d)).

(1) TOL-ROUTE-008 is `<= 1e-12` **relative** per contribution — undefined when `Σ_h S_h = 0`; needs an absolute floor or zero-mass carve-out. (2) "Exact interval overlap (hour-uniform within each hour)" is unambiguous in intent for non-divisible `dtchr` (mass_i = `Σ_h S_h · overlap(interval_i, hour_h) / 3600 s`, grid anchored at 00:00), but the formula and the midnight anchor are never written — one line closes it. (3) INV-ROUTE-020(d) asserts the grid "covers exactly 86400 s," but no rule or guard owns the case where the configured `dtchr` does not divide 86400 (the "normalized" grid is water-routing lineage the sediment contract merely assumes). Proposed disposition: write the projection formula and anchor, add the zero-mass carve-out, and either cite where grid normalization is guaranteed or add a typed guard for non-covering grids.

### B-11 (Low) — Editorial: new non-ASCII character, trailing whitespace, case-nomenclature collision, uneven guard-code pinning

Reference: SC-ROUTE-001.md:141, :253, :621-623, :162-167, :672-675.

Ran: the `×` in INV-ROUTE-019 ("volume × soil bulk density") is the file's first-ever multiplication sign (old count 0, new count 1) — the file otherwise uses `*`; replace for consistency with the repo's conservative non-ASCII posture (Σ has precedent; × does not). Line 253's new invalid-state bullet ends with trailing whitespace. Sequencing step 1b's "detachment/deposition cases I-IV" collides with the §13.2 runoff Case I-IV nomenclature (INV-ROUTE-003) — the segment-solve branches are elsewhere named case12/case34; disambiguate. Guard-map rows: INV-ROUTE-015 pins `WKERNEL-WS10-CHANNEL-E-003` family, rows 016-020 name no codes, while test vector 9 pins the `E-001..003` family for all five failure classes — pin uniformly or state the family once.

### B-12 (Low) — REF-ROUTE-HECRAS-QUS is a user-manual web capture standing as load-bearing authority

Reference: SC-ROUTE-001.md:80; authority rows for INV-ROUTE-015/016/017.

The anchor itself concedes "the online 1D Sediment Transport Technical Reference Manual remains the formal citable source" while the vendored capture is a user manual. As a `[DIRECT]` anchor cited by three hard-fail invariants, this is weaker than the derivation-order intent. The substance it supports (quasi-steady-sequence class behavior) is uncontroversial and co-anchored (ARS-77, CREAMS), so this is low severity. Proposed disposition: vendor or pin the TRM citation, or annotate the anchor as class-corroboration grade.

### B-13 (Low) — Impoundment-dependency posture on the interval lane is unstated

Reference: SC-ROUTE-001.md:594-602; package.md Excluded Scope ("Impoundment sediment routing").

Inlets fed by `ws10_impoundment_*` nodes are presumably fail-closed via INV-ROUTE-005(c) or lane-inactive, but the addendum never says so, and the package explicitly excludes impoundment sediment. One sentence in Activation ("inlets with impoundment dependency nodes remain on the existing INV-ROUTE-005 branches; the interval lane does not activate for them") removes the inference.

## Recommendation

GO-WITH-AMENDMENTS — the authority acquisition, BEI conservation (lint PASS, Ran), evidence grading, and fail-closed posture are sound and nothing pre-existing is weakened, but B-1/B-2 must be fixed before the W11 handoff because as written the multi-channel interval lane is either unreachable or requires exactly the executor science choice the package exit criterion prohibits.

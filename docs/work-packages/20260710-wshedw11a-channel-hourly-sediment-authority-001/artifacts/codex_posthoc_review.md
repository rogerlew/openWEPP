# Codex Post-Hoc Review

Recommendation: `REOPEN`

Blocking defect: `WSHED-W11A-POSTHOC-001` — the interval lane does not yet
define a unique hydraulic/erosion operand realization, and the Rust lane named
as the governing WEPP-adapted realization contains baseline-divergent geometry
updates that can violate the contract's constructive geometry-mass rule. W11
therefore still has executor science choices despite the package's contrary
handoff claim.

## Evidence header

- `Static`: reviewed commit `dea1d62599ecbe764f04672701f79d0caec50cec`,
  the v50-to-v51 `SC-ROUTE-001` amendment, the W11A authority/disposition,
  both reviews, both verifications, the final disposition, and the W11 handoff.
  Reviewed the ten contract-derived vectors against the current WS20/WS21/WS26
  Rust interfaces and the pinned baseline.
- `Ran`: `git show`/line-numbered source inspection; confirmed the baseline at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; inspected baseline
  `dcap.for` and the migrated Rust detachment/segment-routing lanes; ran the
  binding-exposure checker (PASS, seven rows). Authorized read-only citation
  spot-checks used `pdftotext`, rendered-scan inspection, and line-numbered
  reads of Chapter 13, CREAMS Chapter 3, KINEROS ARS-77, and the HEC-RAS
  manual. No build, simulation, comparator, or production test was run; this
  was a docs/authority review.

## Severity-ranked findings

### High — H1: interval hydraulic operands do not determine one segment solve

`INV-ROUTE-016` says segment discharges are derived from routed interval
discharge and interval-projected lateral inflow
(`SC-ROUTE-001.md:145`). The addendum supplies outlet `q1(it)`, mentions
already-superposed inlet `qin(it)`, and separately supplies lateral forcing
(`SC-ROUTE-001.md:631-638`), but gives no equation selecting the upper-boundary
discharge or reconciling `qin`, `q1`, lateral inflow, and routed water-storage
change.

That omission is material. The current core does not consume interval `qin` or
`qlat`; it derives upper and lateral flow from outlet `qpo` and an event-peak
fraction (`02_ws20_segment_routing.rs:52-99`, `:992-1025`). On an unsteady
water-routing interval, `q1` need not equal `qin + lateral inflow` because
storage changes. An executor must therefore choose among retaining the event
peak fraction, using `qin` as the top discharge, forcing a profile from `q1`,
or introducing a storage-aware reconciliation. Those choices produce different
shear, transport capacity, detachment, and geometry.

None of vectors 1-10 distinguishes these alternatives
(`SC-ROUTE-001.md:712-753`). Vector 1 pins duration, inlet sediment flux, and
erosion budget, but not the hydraulic profile construction. This violates the
package exit criterion and the handoff's “no executor science choices” claim
(`w11-handoff.md:9-12,24-36`).

Required amendment: define the per-interval upper/lateral/lower discharge
operands, including the treatment of routed storage change, and add an
anti-alias vector in which event peak partition, `qin(it)`, `q1(it)`, and
projected lateral flow yield different candidate profiles.

### High — H2: the mandated migrated lane contradicts pinned-baseline geometry closure

The widening-law primitive adjudication is sound: linear detachment/widening,
the `1.0176` exponential, fitted `f(x_b)`, and `timpot`/`timex` match pinned
`dcap.for`. However, v51 goes further and says that realization is implemented
by the migrated WSHEDIMPL18-41 lanes (`SC-ROUTE-001.md:147,684-687`) while
`INV-ROUTE-019` defines detached mass constructively from mutated geometry
(`:148`). Two current Rust terminals do not match the cited baseline:

1. In the capped widening path, baseline caps `dct`, reconstructs capped
   `eros`, and derives width/depth from that capped erosion
   (`/workdir/wepp-forest_260430_baseline/src/dcap.for:238-261`). Rust caps only
   `dct` and returns the uncapped width with unchanged depth
   (`01_ws22_ws23_ws26_detachment.rs:376-386`). Its test locks in widening under
   the cap (`:1470-1512`).
2. When incision reaches the layer but boundary shear cannot widen, baseline
   sets `timsh = timpot` and decrements `depmid`
   (`dcap.for:210-215,173-190`). Rust computes class detachment but returns
   unchanged `depmid` (`01_ws22_ws23_ws26_detachment.rs:303-319`), and its test
   locks that state in (`:1543-1582`).

These are state/mass differences, not cosmetic parity deltas. The contract
currently tells W11 both to reuse the migrated realization and to obey the
pinned-baseline constructive geometry-mass identity. The original reviews did
not inspect these terminal semantics; verification only confirmed that the
accepted text edits existed.

Required amendment: state explicitly that these migrated-lane terminals must
be corrected to pinned `dcap.for` behavior before interval reuse, or adjudicate
and justify a different realization. Extend vector 10 with capped and
post-contact/subcritical-boundary-shear cases, independently reconstructing
detached mass from the resulting geometry.

### Medium — M1: the interval erosion clock lacks its normalization operand

V51 bans the event triangular surrogate `tb = 2*rundur` and assigns an erosion
budget of `dtchr` (`SC-ROUTE-001.md:147,638,704-707`). The current shared core,
however, sets `tb = 2 * event_duration`
(`02_ws20_segment_routing.rs:382-410,913-934`), and `dcap` uses `tb` both to
derive `timsh` and to normalize detachment flux by `tb * wflow`
(`dcap.for:154,173,235-245`; Rust `01_ws22_ws23_ws26_detachment.rs:252,273,376-380`).

Vector 1 pins `durrof := dtchr` and “erosion-time budget := dtchr” but never
states what replaces `tb` in the flux/mass denominator
(`SC-ROUTE-001.md:714-722`). Simply passing `event_duration = dtchr` preserves
the prohibited `2*dtchr` path; replacing every `tb` with `dtchr` changes both
clock and normalization. The implementation needs separate, named operands for
interval erosion exposure and mass/flux normalization, with a constructive
closure equation.

### Medium — M2: the accepted layer-contact fix still contains an undefined symbol and density convention

The accepted A-2 fix binds
`timpot = depmid * rho_soil / d_i` (`SC-ROUTE-001.md:147,689-694`), but `d_i`
is absent from Variables and Units (`:95-124`). CREAMS gives
`d_ch = e_m / rho_soil`; baseline `di` is the mass erosion rate
`excess * Kch * (effsh - taucr)` (`dcap.for:163-168`). An executor must guess
whether `d_i` aliases `e_m`, `D`, or `d_ch`.

The same rows call `rho_soil` “weight density” (`SC-ROUTE-001.md:123,148`),
whereas CREAMS calls it in-place soil mass density. Because the contract uses
it to construct sediment mass, the amendment should define `d_i` explicitly
and state the lbm/lbf convention (preferably “in-place bulk mass density,” with
baseline `wtdsoi` numeric provenance).

### Medium — M3: source-anchor summaries overclaim parts of their sources

- HEC-RAS supports a sequence of steady profiles, shared hydraulic/sediment
  computational increments, and persistent bed-change state. It also states
  that cross sections are **not** updated every computational increment;
  changes accumulate until a minimum-bed-change threshold is exceeded (vendored
  HEC-RAS manual, PDF p. 178). `REF-ROUTE-HECRAS-QUS` instead says bed geometry
  updates after each increment (`SC-ROUTE-001.md:80`), repeated in
  `authority-matrix.md:194-196` and summarized as per-increment bed carry in
  `final-disposition.md:25-34`. Narrow the anchor to state advancement/carry;
  use Chapter 13 and pinned WEPP lineage—not HEC cross-section refresh—as the
  geometry-update authority.
- KINEROS directly says a zero upper-boundary transport capacity invokes
  deposition mode on the same water grid (`703.md:1065-1075`). It does not say
  that all incoming lateral mass necessarily deposits. The all-mass dry-reach
  rule is a defensible contract inference and is already mixed-graded at
  `SC-ROUTE-001.md:241`; narrow the direct anchor wording at `:79` accordingly.
- Chapter 13 and the rendered CREAMS scan otherwise support the quoted
  continuity, geometry-carry, and widening equations. The CREAMS anchor's
  conclusion that event-scalar collapse was a compute-cost decision “not a
  physics claim” (`SC-ROUTE-001.md:77`) is interpretive, however, and should be
  mixed-graded rather than wholly `[DIRECT]`.

These corrections do not defeat the quasi-steady-sequence model class, which
is adequately corroborated. They correct source fidelity and evidence grading.

### Low — L1: post-fix review records retain stale or overbroad claims

- `contract-disposition.md:17` says nine anchors while listing eight and still
  says nine vectors after vector 10 was added; `gate-results.md:16` also says
  nine. The final disposition correctly says ten (`final-disposition.md:19`).
- `contract-disposition.md:27` describes deposit-at-grid-end as the ratified
  disposition without limiting it to the unsteady fallback, although the fixed
  contract applies zero-by-construction on the quasi-steady lane.
- `gate-results.md:20` says all verification notes were addressed, while
  `final-disposition.md:61-62` explicitly leaves verification-B note 4 for a
  future pass.

These do not independently invalidate v51, but they show that the post-fix
verification checked the contract more carefully than the package-wide record
reconciliation.

## Review and verification adjudication

The dual reviews were genuinely independent enough for this cycle. Review A
performed source-fidelity/lineage work; Review B performed contract
conformance, diff, BEI, and cross-reference work. Their overlapping activation
and widening findings arose through different lenses, and both were adequately
critical. The verifiers were independent closure checks, though neither
re-audited the migrated lane's terminal behavior or the full post-fix artifact
set.

The A-8 rejection holds. `closed` is established contract-corpus vocabulary for
retained, explicitly risk-accepted limitations (including GAP-ROUTE-001 and
GAP-ROUTE-004), and no `accepted-labeled` promotability value exists in the
current corpus. The corrected rejection rationale properly rests on that
precedent rather than the unrelated lifecycle-index registry rule.

The activation repair is also sound: `INV-ROUTE-015` supplies a mandatory
biconditional, `INV-ROUTE-005(a)` defines active upstream interval egress as
dependency authority, and the addendum evaluates the acyclic dependency in
topological order (`SC-ROUTE-001.md:134,144,601-627`). BEI-ROUTE-007 exposes
the new invariant family without losing the older binding rows (`:176-186`).

## W11 implementability and vector disposition

Vectors 2-9 are directionally implementable after the blocking operand map is
fixed. Vectors 1 and 10 are not sufficient as written:

- Vector 1 does not pin the `qin/q1/lateral/storage` hydraulic profile or the
  detachment normalization denominator.
- Vector 10 covers ordinary layer contact but not the two baseline-divergent
  capped/subcritical terminal branches.

Add those cases before W11 resumes. Until then, the authority is not sufficient
to lift `WSHED-W11-HOLD-001` truthfully.

## Final recommendation

`REOPEN` for `WSHED-W11A-POSTHOC-001`.

Required closure amendments are bounded:

1. specify the interval hydraulic profile and water-storage reconciliation;
2. specify separate erosion-exposure and flux/mass-normalization operands;
3. define `d_i` and the soil-density convention;
4. reconcile the two migrated `dcap` terminals with pinned baseline authority
   and add independent geometry-mass vectors;
5. narrow the HEC-RAS/KINEROS anchor summaries and reconcile stale package
   artifacts.

No contract text was edited in this review pass.

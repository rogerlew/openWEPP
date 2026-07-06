# MOFEFID-D10B - GAP-005 Source-Authority Reconciliation

Status: **SCAFFOLDED** (2026-07-06, Claude Code). Campaign:
[MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D, §6.1
D10 hold-lift. Contract focus: `SC-OFEROUTE-001#GAP-OFEROUTE-005` and the
Case-4 residual of `SC-OFEROUTE-001#INV-OFEROUTE-011`.

## Objective

Execute the follow-on that D10 named as its first actionable closure: a
source-authority reconciliation that binds (a) the TVD limiter / CFL /
dissipation form, (b) the lateral-source and OFE boundary/handoff treatment,
and (c) the Iwagaki Case-4 friction mapping into named, contract-recorded
authority and tolerances — then either lands the resulting contract-first
correction or closes in a legitimate HOLD with a proven boundary.

A non-HOLD closure of this package lifts the last named blocker on the §6.1
D15 rerun (`INV-OFEROUTE-011` / `GAP-OFEROUTE-005` blocks active routed-water
publication). If D10B closes in a legitimate HOLD, D15 remains blocked and the
hold must narrow D10's boundary to the next actionable authority surface.

## Reconciliation Thesis

D10 proved the primaries in hand (Garcia-Navarro 1992, Mingham 2001,
Iwagaki 1955) cannot bind a production correction **as long as the
acceptance oracle is implementation parity with the enhanced-WEPP code**.
Under the clean-room posture (operator, 2026-07-04, recorded in
`docs/planning/mofe-water-balance-sequencing.md` §3), that oracle is
permanently unclosable: openWEPP deliberately does not obtain Papanicolaou's
implementation, and R-63's printed numerics text does not fully specify it.

D10B's central move is therefore a **contract re-anchoring, not a source
hunt**. It decomposes `GAP-OFEROUTE-005` into three surfaces, each closable
from clean-room-legal authority:

1. **Scheme correctness (limiter/CFL/dissipation).** openWEPP owns its
   TVD-MacCormack scheme. The binding authority is the published TVD family
   (Mingham 2001 and Garcia-Navarro 1992 in hand; Davis 1984 and Tseng 2010
   named in R-63's own citation chain and acquirable), plus convergence
   evidence against the physics oracle in (2) — not fidelity to R-63's
   printed equation text where that text contradicts its own cited sources.
2. **Case-4 physics acceptance (oracle re-anchoring).** The authoritative
   Case-4 oracle becomes the Iwagaki 1955 primary itself: the
   characteristics (analytic) solution of the KWE for Iwagaki's published
   flume configuration plus his published experimental hydrographs, with the
   friction closure taken from the primary (Manning `n = 0.009`) via a
   definitional, named-source Manning-to-Darcy mapping. Acceptance is
   law-shaped: grid-refinement convergence to the characteristics solution
   within named tolerances. The digitized enhanced-WEPP Figure-4 model trace
   **demotes to an ADR-0017-class comparator flag** — recorded, expected to
   differ, never acceptance.
3. **OFE handoff conservation (reclassification).** R-63 specifies nothing
   about inter-OFE hydrograph discretization; the sampled handoff is
   openWEPP-owned machinery. Its correctness authority is the conservation
   invariant (program hard gate), not any external text. The H2637
   resolution sensitivity is adjudicated — and if a defect is confirmed,
   corrected — under that gate.

Evidence already on record supporting the demotion in (2): D10's own
refinement sweep moves the solver **away** from the digitized trace
(`NS_trace` 0.262677 → 0.193296 → 0.101244 as cells double twice;
`iwagaki-case4-evidence.md`), the signature of a reference that is not the
converged solution of the governing PDE under the configured friction
closure. Whether the residual divergence is oracle mismatch, friction
mapping, or a real scheme defect is exactly what the re-anchored harness
discriminates.

## Findings Carried In (static, hypothesis-grade until harness-adjudicated)

Recorded by Claude Code during scaffold review (2026-07-06, static reads
only; no simulations run):

- **Printed limiter (11c) vs its own citations.** The production solver
  implements R-63 eq. (11c) literally
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:470-478`):
  `phi = min(2r, 1)` for `r < 0`, else `0`. That branch applies full
  dissipation (`phi = 0`) in smooth/monotone regions (`r >= 0`, including
  `r ≈ 1`) and amplified dissipation (`phi < 0`, so `1 - phi > 1`) at
  extrema — inverted relative to the standard Davis-symmetric-TVD
  convention (`phi(r) = max(0, min(2r, 1))`, dissipation vanishing as
  `r → 1`) defined by the sources R-63 itself cites for the scheme (Davis
  1984; Mingham 2001). First-order smearing everywhere is consistent with
  the recorded Case-4 signature (late `t_peak` 37.0 s vs 25.98 s, slow
  10-90% rise, at every resolution), but this is a hypothesis until the
  Leg-B harness adjudicates it.
- **Additional scheme adjudication points.** One-sided single-ratio limiter
  (eq. 11d, `kinematic_wave.rs:449-459`) vs the two-sided per-face form in
  the Davis family; the pre-step-frozen `alpha` evaluation (D14 OPT-1
  precondition) vs within-step update (Tseng 2010 is the paper's named
  source for the applied KWE computation); boundary cells receive
  `TVD_i = 0` (`kinematic_wave.rs:567-573`), a candidate conservation-leak
  site for the Leg-C ledger.
- **Handoff aliasing hypothesis (Leg C).** A point-sampled inter-OFE
  hydrograph re-forced into the downstream OFE is not flux-conservative
  between samples; aliasing there would explain the non-monotone
  `(sample_dt, max_dt)` dependence of the H2637 run-level cascade
  conservation aggregate (6.0% / 10.0% / 22.1%, D10
  `h2637-resolution-evidence.md`). The seam-decomposed mass ledger decides;
  the candidate correction class if confirmed is flux-integrated
  (interval-averaged) conservative transfer.
- **D10's rejected limiter-flip trial is not dispositive here.** It was
  adjudicated against the digitized-trace `NS_trace` (the oracle this
  package demotes) and against focused `ofe_routing` tests that may pin
  current behavior rather than encode laws. D10B must audit which failing
  tests encode contract laws vs pinned behavior before treating them as
  blocking (see Required Artifacts: `behavior-pinned-test-audit.md`).

These are findings and candidate mechanisms, not prescribed fixes. The
executor owns disposition under the acquired/held authority and the
re-anchored harness evidence.

## Correction Authority Envelope

### Defect IDs and Observed Violations

- `GAP-OFEROUTE-005` (EXECUTED-HOLD-SOURCE-AUTHORITY at rev 18): Iwagaki
  shock sampled-hydrograph / resolution sensitivity; hold names the three
  unbound surfaces this package reconciles.
- `INV-OFEROUTE-011` Case-4 residual: the only remaining D-val blocker.
- H2637 shadow reproduction: run-level cascade conservation diagnostics
  resolution-sensitive and non-monotone on the steep 19-OFE fixture.

### In-Scope Authority

- `SC-OFEROUTE-001` (rev 23 at scaffold time) numerical-method, D-val,
  `INV-OFEROUTE-011`, and `GAP-OFEROUTE-005` rows.
- Iwagaki 1955 primary (R-74, in hand): flume geometry, slopes, lateral
  supplies, Manning `n = 0.009`, characteristics solution, experimental
  hydrographs.
- Garcia-Navarro 1992 (R-81, in hand), Mingham 2001 (R-82, in hand).
- Acquisition targets from R-63's own reference list, under
  `references/` copyright governance:
  - Davis, S. F. (1984). *TVD finite difference schemes and artificial
    viscosity* (Rep. 84). Hampton, VA: NASA Langley Research Center.
    (US-government report; rights classification per governance.)
  - Tseng, M. (2010). Kinematic wave computation using an efficient
    implicit method. *Journal of Hydroinformatics*, 12(3), 329-338.
- Lighthill & Whitham 1955 (in hand) for kinematic shock fitting
  (Rankine-Hugoniot for the kinematic flux) in the characteristics oracle.
- A named in-library hydraulics source for the definitional Manning-`n` to
  Darcy-`f` identity used only in the Case-4 D-val configuration.
- The program conservation hard gate (correctness re-anchoring scheme) as
  the authority class for Leg C.
- Current Lane D implementation surfaces: `ofe_routing::kinematic_wave`,
  `ofe_routing::cascade`, `ofe_routing::infiltration`, `ofe_routing::dval`,
  the D-val example, and D-val harness scripts/tests.

### Authorized Edit Classes

- Amend `SC-OFEROUTE-001` (rev 24+) to record: the re-anchored Case-4
  acceptance oracle and named tolerances; the digitized-trace demotion to
  comparator flag; the limiter/CFL/dissipation binding with named authority;
  the handoff reclassification under the conservation gate; the
  Manning-`n`-to-`f` mapping for the Case-4 configuration; and the
  resulting `GAP-OFEROUTE-005` disposition.
- Acquire and register the named references (bibliography row, rights
  classification, no vendored long excerpts).
- Build the characteristics-solution oracle harness (tool/example tier, not
  production numerics) and contract-derived tests: Case-4 convergence
  sweeps against the oracle, TV non-increase checks, seam-decomposed
  conservation ledger on the 19-OFE H2637 fixture.
- Correct the pure Lane D solver/cascade/handoff implementation if and only
  if all seven DC gates pass under the reconciled authority.
- Audit and, where proven behavior-pinning rather than law-encoding, amend
  focused `ofe_routing` tests — each such amendment individually justified
  in the test audit artifact.

### Protected Boundaries

- **Clean-room:** no acquisition or use of Papanicolaou implementation
  material beyond the published paper and its supplemental. Where the paper
  is silent, the contract records an openWEPP-owned decision with named
  family authority — never inferred implementation detail.
- No production/default activation; no D15 flip; no D16 policy.
- No change to the D11 rev-20/21 production friction operand path: the
  Manning-`n`-to-`f` mapping enters only the Case-4 D-val configuration.
- No D12 melt-limb or D13 erosion-shape scope.
- No watershed/channel routing changes.
- No tuned, surrogate, proxy, or heuristic stand-in numerics; `k_o` scans
  remain diagnostic-only, never authority.
- No pre-filled expected values in evidence artifacts; metrics are recorded
  verbatim from executed runs.

## Scope

### Included

- Acquisition/registration of Davis 1984 and Tseng 2010 (or a recorded
  acquisition hand-off to the operator if the execution environment lacks
  connectivity — see Phase D10B-S1).
- Limiter/CFL/dissipation adjudication: printed (11c) vs the
  Davis/Mingham/Tseng family; one-sided vs two-sided ratios; `alpha` update
  timing; boundary-cell dissipation treatment.
- Characteristics-solution oracle for Iwagaki Case 4 built from the R-74
  primary (+ Lighthill-Whitham shock fitting), with the `n = 0.009`
  friction closure mapped via the named definitional identity.
- Contract amendment re-anchoring `INV-OFEROUTE-011` Case-4 acceptance and
  demoting the digitized enhanced-WEPP trace to comparator flag.
- Seam-decomposed conservation ledger for the OFE handoff on H2637;
  adjudication (and correction if the seven gates pass) under the
  conservation hard gate.
- Behavior-pinned-test audit for the focused `ofe_routing` tests that
  blocked D10's trial.
- Re-run of the D10 evidence surfaces (Case-4 sweeps, H2637 resolution
  sweeps) under the re-anchored harness; named tolerances recorded from
  primary-derived uncertainty, not from current solver output.
- Dual review, disposition, dual verification, line-count governance,
  final package status, and the §6.1 D14-refresh flag if the accepted
  correction changes handoff policy, solver resolution, or per-step cost
  shape.

### Excluded

- Cases 1-3 and Zone 1/Zone 2 taxonomy (closed by D9).
- D11 production friction operand authority (closed rev 20/21; untouched).
- D12 melt-limb (closed rev 22), D13 erosion shape (closed rev 23/53).
- D14 re-profiling (only the refresh *flag* is in scope), D15 activation,
  D16 default policy.
- Any enhanced-WEPP implementation-parity acceptance claim.

## Dependencies

- `SC-OFEROUTE-001` rev 23.
- D10 package and artifacts (hold, source audit, Case-4/H2637 evidence,
  resolution-control harness):
  `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/`.
- D9 Case-4 handoff:
  `docs/work-packages/20260705-mofefid-d9-dval-disposition-001/artifacts/case4-d10-handoff.md`.
- Clean-room provenance record:
  `docs/planning/mofe-water-balance-sequencing.md` §3.
- MOFEFID strategy §6.1 (D10 hold-lift row; D14-refresh ordering rule).
- References in hand: `references/copyrighted/Iwagaki1955_runoff_characteristics_DPRI10.pdf`,
  `references/copyrighted/Papanicolaou2018.md` (+ PDF and supplemental),
  `references/copyrighted/10.1061@ASCE0733-94291992118@101359.pdf`
  (Garcia-Navarro 1992), `references/copyrighted/mingham2001.pdf`,
  `references/copyrighted/Lighthill_Whitham_1955_Kinematic_Waves.pdf`.

## Intended Write Set

Primary:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/`
- `docs/work-packages/README.md`
- `references/annotated_bibliography.md` plus acquired reference files under
  `references/` per rights governance.
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1 row status on
  closure.

Conditional, only under the reconciled authority and the seven DC gates:

- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `crates/openwepp-hillslope-orchestrator/examples/dval_case.rs`
- `tools/dval/` (existing comparator plus the new characteristics-oracle
  harness)
- Focused `ofe_routing` tests amended under the behavior-pinned-test audit;
  D-val / H2637 fixtures directly needed to prove the correction or hold.

Protected:

- Production routed-water activation files beyond read-only inspection.
- D11 rev-20/21 operand builder surfaces.
- HBP/pass/watershed output schemas.

## Phase Plan

1. **D10B-S0 - Intake and baseline.** Read core/conditional authority and
   the D10 artifacts; re-run the D10 Case-4 baseline and resolution
   controls and the H2637 shadow sweep unchanged to confirm the recorded
   pre-change metrics still reproduce; build the seam-decomposed
   conservation ledger (solver interior vs boundary cells vs inter-OFE
   handoff) on the H2637 fixture.
2. **D10B-S1 - Acquisition and scheme adjudication (Leg A).** Acquire
   Davis 1984 and Tseng 2010 under copyright governance; if the execution
   environment lacks connectivity, record the exact acquisition list in the
   worker handoff for the operator and proceed on the in-hand primaries,
   recording whether Mingham 2001 + Garcia-Navarro 1992 suffice to bind and
   which points remain Davis/Tseng-confirmatory. Adjudicate the printed
   (11c) branch, ratio form, `alpha` update timing, CFL posture, and
   boundary dissipation against the family authority.
3. **D10B-S2 - Contract-first amendment (Leg B + reclassification).** Amend
   `SC-OFEROUTE-001` before any production edit with the source-authority
   structure and test obligations: re-anchored Case-4 oracle, proposed
   tolerance basis, digitized-trace demotion, limiter/CFL/dissipation binding,
   Manning-`n`-to-`f` mapping for the Case-4 configuration, handoff
   reclassification under the conservation gate, and updated
   `GAP-OFEROUTE-005` row. Final tolerance ratification must cite the S3/S4
   oracle-constructibility and convergence evidence, or the package must hold
   with the unproven tolerance as the named boundary.
4. **D10B-S3 - Oracle harness and contract-derived tests.** Build the
   characteristics-solution oracle and convergence harness; add TV
   non-increase and seam-conservation checks; execute the
   behavior-pinned-test audit; record the pre-implementation gate. The new
   tests must fail (or the hold must be executable) under the observed
   defect before any correction lands.
5. **D10B-S4 - Correction or legitimate HOLD.** If the seven DC gates pass,
   land the solver/cascade/handoff correction(s) and validate: Case-4
   grid-refinement convergence to the oracle within named tolerances, TV
   non-increase, and resolution-convergent monotone-bounded H2637 cascade
   conservation within a named tolerance. Otherwise stop behind a
   hold-legitimacy audit naming the boundary.
6. **D10B-S5 - Evidence and closure.** Required gates, artifacts, dual
   review and verification, disposition, package status, strategy/registry
   updates, and the D14-refresh flag decision for the D15 rerun handoff.

## Exit Criteria

- `GAP-OFEROUTE-005` is closed by the reconciliation (with or without a
  production correction) only when the final contract authority and tolerance
  evidence are ratified, or held behind a newly proven boundary that is
  narrower than D10's — a hold that merely restates "primaries do not bind
  implementation parity" is not legitimate here, because this package's
  thesis removes implementation parity from the acceptance surface.
- `INV-OFEROUTE-011` Case-4 status is unambiguous under the re-anchored
  oracle: pass within named tolerances, held behind a named residual
  boundary, or superseded by the revised acceptance criterion.
- The digitized enhanced-WEPP Figure-4 trace is recorded as comparator flag
  only, with the divergence-under-refinement evidence cited.
- The limiter/CFL/dissipation form in production is bound to named
  authority; every departure from R-63's printed text is recorded with its
  source justification.
- The OFE handoff has a recorded authority class (conservation hard gate),
  a seam-decomposed ledger, and either a correction with
  resolution-convergent H2637 conservation evidence or a named residual
  boundary.
- The Case-4 friction closure is the primary's `n = 0.009` via the named
  definitional mapping; no `k_o` tuning enters authority; the D11
  production operand path is untouched.
- Behavior-pinned-test audit dispositions every focused test that blocked
  D10's trial.
- Any final Case-4 tolerance, TVD-family binding, or handoff conservation
  tolerance ratified in `SC-OFEROUTE-001` cites the S3/S4 evidence that proved
  the oracle/harness constructible and executable; provisional S2 obligations
  cannot be presented as final acceptance authority.
- If handoff policy, solver resolution, or per-step cost shape changed, the
  worker handoff flags the §6.1 D14 endpoint-timing refresh as a D15-rerun
  precondition.
- Dual review findings dispositioned; line-count governance recorded for
  every touched `.rs` file; no production/default activation claimed.

## Required Gates

Selection follows `docs/standards/local-ci-gate-selection.md` where
relevant, but D10B cannot close without recording:

- `git diff --check`
- Markdown lint for touched docs.
- Contract/profile/BEI checks for changed `SC-OFEROUTE-001` surfaces.
- Characteristics-oracle Case-4 convergence sweeps (commands + logs).
- H2637 shadow resolution sweep + seam-conservation ledger commands.
- Focused Rust tests for touched `ofe_routing` / D-val surfaces.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`
- Source-level anti-evasion guards if required-case bindings, D-val
  fixtures, or authority-suite posture are touched.

If heavy gates are delegated, record subagent output and log paths in
`artifacts/gate-results.md`.

## Conservation / Output Acceptance

Leg C adjudicates a conservation-sensitive seam. Before any
solver/cascade/handoff edit, record the operand-lineage table (source
terms, boundary inflows, outlet hydrographs, storage, sample interval,
substep controls, units, authoritative vs diagnostic status). Acceptance
requires independent mass/closure reconstruction and rejected alias
formulas — never one-sided bounds or self-consistency alone. Case-4
acceptance requires convergence-trend evidence across at least three
resolutions, not a single-resolution match.

## DC Conversion Rule

If the reconciled authority supports a reproducible root cause inside the
envelope, D10B must proceed through contract amendment, contract-derived
tests, pre-implementation gate, production correction, validation, review,
and disposition. It may not close as `HOLD` because more investigation is
possible. The seven gates: reproduction, named mechanism, ownership,
authority, safety, testability, validation.

## HOLD Legitimacy

D10B may close in `HOLD` only behind one of:

- a demonstrated contradiction between the acquired family primaries that
  prevents binding the scheme form;
- a characteristics-oracle construction blocker inside the Iwagaki primary
  (e.g., the primary's published configuration is insufficient to pose the
  Case-4 problem without material openWEPP does not have);
- a confirmed defect whose correction is proven to belong to another owner
  (named contract + row).

Each hold must name the boundary, cite evidence, list the in-envelope route
considered, and state why it cannot close inside D10B. D10's boundary
("primaries do not bind implementation parity") is consumed by this
package's re-anchoring and cannot be re-cited as the hold.

## Subagent Authorization

This package explicitly authorizes spawning/delegating to
`rust_code_reviewer`, `rust_qa_reviewer`, `explorer`, and
`comparator_suite_runner` subagents for read-only review, verification,
source/harness inspection, primary/source-evidence review, and heavy
D-val/full-gate execution. `comparator_suite_runner` is REQUIRED for heavy
Case-4/H2637 sweeps and full workspace nextest when available; if
unavailable, record command-level evidence and run locally only where
package governance permits substitution. Write access is read-only unless a
later operator explicitly assigns a bounded write set.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/correction-authority-envelope.md`
- `artifacts/source-acquisition-record.md`
- `artifacts/limiter-adjudication-evidence.md`
- `artifacts/oracle-reanchoring-evidence.md`
- `artifacts/friction-mapping-evidence.md`
- `artifacts/seam-conservation-ledger.md`
- `artifacts/behavior-pinned-test-audit.md`
- `artifacts/contract-implementation-evidence.md`
- `artifacts/contract-test-implementation-evidence.md`
- `artifacts/pre-implementation-contract-gate.md`
- `artifacts/iwagaki-case4-evidence.md`
- `artifacts/h2637-resolution-evidence.md`
- `artifacts/numerics-convergence-evidence.md`
- `artifacts/conservation-output-lineage.md`
- `artifacts/implementation-test-evidence.md`
- `artifacts/hold-legitimacy-audit.md` (only if HOLD is claimed)
- `artifacts/kernel-profile-compliance-checklist.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/line-count-governance-checklist.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`
- `artifacts/worker-handoff.md`
- `artifacts/disposition.md`

## Progress

- [x] 2026-07-06: Package scaffolded by Claude Code from the D10 hold, the
  D15 preflight, and the ROADMAP §M / strategy §6.1 D10 hold-lift row.
- [ ] D10B-S0 intake and baseline.
- [ ] D10B-S1 acquisition and scheme adjudication.
- [ ] D10B-S2 contract-first amendment.
- [ ] D10B-S3 oracle harness and contract-derived tests.
- [ ] D10B-S4 correction or legitimate HOLD.
- [ ] D10B-S5 evidence, review, verification, and closure.

## Surprises & Discoveries

(recorded during execution)

## Decision Log

- Decision: D10B reconciles `GAP-OFEROUTE-005` by re-anchoring the Case-4
  acceptance oracle to the Iwagaki primary and reclassifying the OFE
  handoff under the conservation hard gate, instead of seeking further
  Papanicolaou implementation material.
  Rationale: the clean-room posture makes implementation parity permanently
  unclosable; the operator's recorded rule for such questions is to bound
  them and keep them visible. The re-anchoring converts D10's hold boundary
  into closable surfaces with in-hand or acquirable clean-room authority.
  Date/Author: 2026-07-06 / Claude Code (scaffold).

## Outcomes & Retrospective

(recorded at closure)

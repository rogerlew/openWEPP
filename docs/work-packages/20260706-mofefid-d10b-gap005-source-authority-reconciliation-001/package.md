# MOFEFID-D10B - GAP-005 Source-Authority Reconciliation

Status: **EXECUTED-COMPLETE** (2026-07-06, Claude Code, operator-directed
end-to-end run; `GAP-OFEROUTE-005` RESOLVED at `SC-OFEROUTE-001` rev 26).
Campaign:
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
  convention defined by the sources R-63 itself cites for the scheme.
  **Branch convention confirmed first-hand (2026-07-06, Ran: page render
  + visual read):** Davis 1984 eq. (3.20), p. 9 of the acquired
  `19840021490.pdf`, reads `phi(r) = min(2r, 1) if r > 0; 0 if r <= 0` —
  the exact branch-swap of R-63's printed (11c). The *inconsistency* is
  therefore primary-adjudicated, not hypothesized; what remains
  hypothesis-grade is whether the implemented branch is the cause of the
  recorded Case-4 signature (first-order smearing everywhere is consistent
  with the late `t_peak` 37.0 s vs 25.98 s and slow 10-90% rise at every
  resolution, but the Leg-B harness adjudicates causality).
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
- Acquired 2026-07-06 (operator), from R-63's own reference list, under
  `references/` copyright governance; registered as bibliography rows
  R-102 (Davis) and R-103 (Tseng) on 2026-07-06 (Claude Code); the
  first-pass rights-log entries remain a D10B-S1 task:
  - Davis, S. F. (1984). *TVD finite difference schemes and artificial
    viscosity*. ICASE Report 84-20 / NASA CR-172373. Hampton, VA: NASA
    Langley Research Center. In hand:
    `references/copyrighted/19840021490.pdf` (identity verified: title
    page read) plus a Gemini-converted markdown
    `references/copyrighted/19840021490.md`. The markdown's load-bearing
    eq. (3.20) was verified against the rendered PDF page (p. 9) —
    faithful; other equations used as binding authority must be
    spot-checked against the PDF before citation (conversion, not
    primary).
  - Tseng, M.-H. (2010). Kinematic wave computation using an efficient
    implicit method. *Journal of Hydroinformatics*, 12(3), 329-338. In
    hand: `references/copyrighted/Tseng2010_Hydroinformatics.pdf`
    (identity verified: first page read). Note: the paper validates
    MacCormack-family KWE schemes against analytical solutions and an
    experimental measurement — directly relevant precedent for the Leg-B
    oracle shape.
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

- Registration of the acquired Davis 1984 and Tseng 2010 references
  (bibliography rows, rights classification) and conversion-fidelity
  spot-checks of `19840021490.md` against the PDF for every equation cited
  as binding authority.
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
  `references/copyrighted/Lighthill_Whitham_1955_Kinematic_Waves.pdf`,
  `references/copyrighted/19840021490.pdf` (Davis 1984, + `.md` Gemini
  conversion), `references/copyrighted/Tseng2010_Hydroinformatics.pdf`.

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
2. **D10B-S1 - Source adjudication (Leg A).**
   Davis 1984 and Tseng 2010 are in hand (operator acquisition,
   2026-07-06) and registered as bibliography rows R-102/R-103; record
   the first-pass rights-log entries. Spot-check
   `19840021490.md` conversion fidelity against the PDF for
   every equation cited as binding authority. Adjudicate the printed
   (11c) branch (Davis eq. 3.20 already confirms the branch-swap;
   record it contract-side), ratio form (Davis eq. 3.18 is two-sided
   per-face), `alpha` update timing (Tseng's implicit treatment), CFL
   posture, and boundary dissipation against the family authority.
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
- [x] 2026-07-06: Leg-A source acquisition satisfied by the operator
  (Davis 1984 `19840021490.pdf` + `.md`, Tseng 2010
  `Tseng2010_Hydroinformatics.pdf`); identities verified and the Davis
  markdown's eq. (3.20) checked against the rendered PDF page (Claude
  Code). Registered as bibliography rows R-102/R-103 (2026-07-06, Claude
  Code); first-pass rights-log entries remain in D10B-S1.
- [x] 2026-07-06: D10B-S0 intake and baseline (Claude Code, executing on
  operator direction): Case-4 D10 baselines reproduced exactly; the
  D10-era H2637 executed-vector test no longer exists (D11 rev-20
  fail-closed gate) — class reproduced on an H2637-shaped cascade fixture;
  seam-decomposed conservation ledger built and executed
  (`artifacts/seam-conservation-ledger.md`) with an EXACT decomposition
  identity. Mechanism ranking REVERSED vs the scaffold hypothesis: solver
  ledger-vs-scheme flux mismatches (outflow ghost, inflow booking) + an
  unbooked anti-convergent TVD boundary leak dominate; handoff aliasing is
  secondary.
- [x] 2026-07-06: D10B-S1 source adjudication (Claude Code):
  rights-log addendum recorded; Davis (3.20)/(3.18) conversion
  spot-checks done; Mingham (28b)/(31a)/(31f)/(31g) + `C(x)` + CFL 0.9
  extracted; all five held surfaces adjudicated in
  `artifacts/limiter-adjudication-evidence.md` (A1 branch bound, A2
  two-sided face form bound, A3 no-change, A4 frozen-alpha adjudicated
  with Tseng precedent, A5 conservation-gate-owned corrections named, A6
  Iwagaki Manning `n=0.009` mapping bound).
- [x] 2026-07-06: D10B-S2 contract-first amendment — `SC-OFEROUTE-001`
  rev 24 (bindings + re-anchored oracle + demotion + gap reconciliation);
  evidence artifacts written (`contract-implementation-evidence.md`,
  `oracle-reanchoring-evidence.md`, `friction-mapping-evidence.md`).
- [x] 2026-07-06: D10B-S3 oracle harness + contract-derived tests — the
  Iwagaki oracle landed as TWO independent constructions (monotone FV
  reference + Lagrangian characteristics fan) cross-validating within
  3%/1 s, with closed-form anchors and exact conservation; converged
  Case-4 truth: peak ~0.00831 m^2/s, `t_peak` ~24.6 s, rise ~19.65 s.
  Five contract-derived tests authored and recorded FAILING 5/5 against
  the pre-correction scheme; pre-implementation gate recorded (7/7 DC
  gates PASS -> proceed).
- [x] 2026-07-06: D10B-S4 correction LANDED (not HOLD): source-correct
  limiter branch; two-sided face-based exactly-telescoping dissipation
  (material-interface faces zero, boundary-stencil mirroring);
  prescribed-flux upstream BC; donor outflow closure with
  booked-equals-actual ledger; half-weight stage clamps; TRUE kinematic
  celerity for CFL (fixes a latent true-Courant ~1.8 condition on the
  laminar limb); conservative bin-series handoff + bin-mean boundary-flux
  hydrograph export; Manning limb + `run_iwagaki_manning`. Validation:
  5/5 contract tests green; Case-4 within ratified tolerances
  (peak <= 2.6%/5%, `t_peak` <= 0.09 s/1.5 s, rise <= 0.11 s/2.0 s);
  19-OFE cascade conservation IDENTICALLY ZERO at every sweep point
  (pre: 9-54% anti-convergent); full `ofe_routing` suite 61/61 with
  behavior-pinned tests dispositioned. `SC-OFEROUTE-001` rev 25 ratifies
  tolerances and RESOLVES `GAP-OFEROUTE-005`.
- [x] 2026-07-06: D10B-S5 evidence, review, verification, and closure:
  gates fmt/clippy/deny PASS, full workspace nextest PASS twice (1396/1396
  pre-review, 1399/1399 after the review fixes + 3 regressions); dual
  review (A: GO-WITH-AMENDMENTS, B: GO-WITH-AMENDMENTS, no CRITICAL) with
  every finding dispositioned and all accepted amendments landed
  (contract rev 26 + Review-B production fixes M1/M2/M3 with
  regressions); dual verification (A: PASS-WITH-NOTES, B:
  PASS-WITH-NOTES, all notes closed in-package).

- [x] 2026-07-06: Codex post-execution review response (review-codex.md,
  reviewed `1d202b10`): ALL findings accepted and fixed — High-1 CFL
  fail-closed guards + regression; High-2 Case-4 single-sourced from
  `OracleConfig::iwagaki_case4()` with exact breakpoint-clipped cutoff +
  source-history regression (acceptance metrics improved: peak errors
  now -0.3%..+2.1%); Medium-1 typed `NegativeOutletBin` fail-closed +
  single-OFE outlet regression; Medium-2 seam-ledger relabel + explicit
  sampled-quadrature diagnostic; Medium-3/4 stale GAP-005/D15 authority
  language swept from contract/strategy/ROADMAP; Low 1-5 (superseded
  wording, rights sync, test rename, six-point sweep expansion, stale
  docs/counts). One transparent re-disposition: the k_o diagnostic
  stability pin (confounded ratification) replaced with law-like guards.
  Focused suite 67/67; full workspace suite re-run recorded in
  gate-results. Evidence: `artifacts/review-response-claude.md`.

## Surprises & Discoveries

- Pre-execution (2026-07-06, scaffold): Davis 1984 eq. (3.20) — read
  first-hand from the rendered page 9 of the acquired PDF — states
  `phi(r) = min(2r, 1) if r > 0; 0 if r <= 0`, the exact branch-swap of
  R-63's printed (11c). The printed-limiter inconsistency is
  primary-adjudicated before execution begins; causality for the Case-4
  signature remains the harness's question.
- Pre-execution (2026-07-06, scaffold): Tseng 2010 validates its
  MacCormack-family KWE schemes against analytical solutions and an
  experimental measurement — published precedent for the Leg-B
  analytic-oracle acceptance shape.
- S0 (2026-07-06): the seam ledger REVERSED the scaffold's mechanism
  ranking. The handoff-aliasing hypothesis is secondary; the dominant
  terms are solver-internal — the ledger books `q_up dt` in and a
  committed-state trapezoid out while the scheme actually injects
  `0.5(q_up+q_0) dt` and discharges through an extrapolated ghost
  (16-28% of source at the operating point), and the boundary-exempt TVD
  term leaks unbooked mass ANTI-CONVERGENTLY (Cf saturates at 0.25 under
  CFL-active stepping, so refinement multiplies the leak). Evidence:
  `artifacts/seam-conservation-ledger.md` with an exact (<=6e-14)
  decomposition identity.
- S1 (2026-07-06): Iwagaki 1955 experiment (B) IS the D-val Case-4
  configuration verbatim (slopes, supplies, duration, geometry), analyzed
  by the primary itself with Manning `n=0.009` — the friction-mapping leg
  dissolves into "run the primary's own law on both sides."
- S0/S1 (2026-07-06): the Case-4 D-val slopes/supplies carry an OCR trap:
  Iwagaki's text prints `q = 0.800 cm/s` for the third reach where the
  physical value is `0.0800 cm/s` (dropped leading zero); the in-repo
  D-val operand (0.08 cm/s) is correct.
- S3 (2026-07-06): the demoted digitized enhanced-WEPP trace lands within
  ~2% peak / ~1.4 s of the primary-anchored oracle — the trace was CLOSE
  to the true entropy solution, and the pre-correction solver's failure
  against it was a REAL numerics failure compounded by the un-primary
  `k_o=200` operand (the `f = k_o/Re` law converges to `q ∝ h^3`, a
  different physics than Manning's `q ∝ h^{5/3}`). The demotion stands as
  an authority decision; the flag observation is recorded.
- S4 (2026-07-06): the corrected limiter EXPOSED a latent instability the
  inverted branch had been masking — the frozen-alpha celerity
  under-estimates the true kinematic celerity 2x on the laminar limb, so
  the scheme ran at true Courant ~1.8 with blanket dissipation hiding the
  oscillation. The D10 limiter-flip trial's "worse" result is thereby
  EXPLAINED: flipping the branch alone removed the mask without fixing
  the celerity. True-celerity CFL + the face-form dissipation resolve it.
- S4 (2026-07-06): a first "upwind outflow closure" attempt was WRONG
  (zero-gradient, steady-biased O(dx)) and was caught by the steady-state
  law tests; the donor-difference closure is algebraically the
  extrapolated ghost for mass purposes, and the residual confined
  boundary-flux ripple (zero-mean, mass-exact) is characterized in
  `iwagaki-case4-evidence.md` with the bin-mean export bounding it.

## Decision Log

- Decision: the acceptance oracle is the exact ENTROPY SOLUTION carried by
  two independent constructions (monotone finite-volume reference primary;
  Lagrangian characteristics fan with Rankine-Hugoniot shock tracking as
  cross-check), rather than a single hand-built characteristics solution.
  Rationale: monotone schemes provably converge to the same entropy
  solution MOC constructs; two independent constructions agreeing (3%/1 s)
  is stronger oracle self-evidence than either alone; the first MOC cut
  missed the cutoff rarefaction (-66% mass) and the miss was caught by the
  cross-validation design.
  Date/Author: 2026-07-06 / Claude Code.
- Decision: land the correction (conversion rule), not HOLD; ratify
  tolerances from measured evidence (peak 5% vs Richardson-extrapolated
  reference with non-divergence, `t_peak` 1.5 s, rise 2.0 s, conservation
  exactness 1e-9, TV-transient bound 1e-3 m^2/s); record three named
  bounded residuals (strict-TVD transient, boundary-flux ripple,
  shock-peak wobble) as refinement items rather than blockers.
  Rationale: all seven DC gates passed; the acceptance surfaces that gate
  activation (oracle convergence + conservation) are met with wide
  margins; the residuals are measured, bounded by tests, confined, and
  mass-exact — holding on them would be a grind-HOLD on quality-of-polish
  rather than a legitimate boundary.
  Date/Author: 2026-07-06 / Claude Code.
- Decision: the exported outlet hydrograph becomes the bin-mean BOUNDARY
  FLUX (with bin-mean stage), and the inter-OFE handoff consumes the
  conservative bin series.
  Rationale: the boundary flux is what the ledger books, what the
  downstream OFE physically receives, and what a gauge measures; the
  committed last-cell state carries O(dx) registration and a confined
  boundary ripple that must not pollute exported surfaces; bin semantics
  make the handoff exactly conservative at ANY sample resolution and fit
  the INV-008 hourly-profile intent.
  Date/Author: 2026-07-06 / Claude Code.
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

D10B closed `GAP-OFEROUTE-005` end-to-end as a landed, validated
correction — the DC conversion rule's intended path, not a hold. The
reconciliation thesis held: implementation parity with enhanced-WEPP was
never closable under clean-room, and re-anchoring to the Iwagaki-primary
entropy-solution oracle plus the conservation hard gate made every held
surface decidable from in-hand authority. The S0 exact seam-ledger
decomposition was the pivotal instrument: it overturned the scaffold's
mechanism ranking (handoff aliasing was secondary; unbooked scheme-flux
mismatches and an anti-convergent TVD boundary leak dominated), and later
pinpointed each residual as corrections landed. The corrected scheme now
matches the oracle to <= 2.6% peak / 0.09 s timing and conserves
IDENTICALLY (machine epsilon) on the 19-OFE H2637-class regime at every
recorded resolution — the defect class is eliminated at its mechanism.
Notable science outcomes: R-63's printed limiter adjudicated a
transcription error against its own citation chain; a latent
true-Courant-~1.8 instability exposed and fixed (explaining D10's failed
limiter-flip trial); the demoted digitized trace turned out to agree with
the oracle within ~2% — recorded as a flag, with the demotion standing on
authority grounds. Dual review added three real latent-bug fixes (fp
loop-progress, negative front-arrival bins, partial-final-bin seam) that
D10B's own evidence configurations could not have caught — the
adversarial-review stage earned its cost. Remaining forward items are the
D14 endpoint-timing refresh (required) and the D15 rerun; three named
bounded residuals are recorded as refinement candidates. Total: contract
revs 24-26, ~1500 lines of solver/oracle/test code, 28 artifacts, all
root gates green (1399/1399).

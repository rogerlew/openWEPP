# AUTH03 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: Level-4 constitutive gate bootstrap — `tests/integration/auth03_level4_constitutive_gate_contract.rs`,
the suite specs/fixtures/registry under `docs/specifications/external-authority/`
and `tests/fixtures/constitutive/`, and the AUTH03 contract addenda. Reviewed
against the AUTH01/02/04 framework and the over-drainage defect documented in
`docs/governance/correctness-reanchoring-keep-condemn-map.md`.

Evidence classes: **Static** (read source/contract/fixture) and **Ran** (command
executed by this reviewer).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — Framework infrastructure (AUTH01/02/04) is real and well-formed

Static. The suite framework is genuine, reusable scaffolding: a registry
(`docs/specifications/external-authority/registry.yaml`), per-suite specs with a
structured schema (`authority_level`, `sc_invariant_refs`, `external_citations`,
`fixtures`, `tolerances`, `gate_lane`, `failure_class`), contract-invariant
linkage (`INV-SOIL-014`, `INV-WATBAL-006`), and AUTH04 CI lane wiring (required
lane, hard-fail → non-zero exit, periodic/manual routing). The AUTH03 fail-closed
guard tests (missing/non-finite θ_fc → `HS-RUNTIME-E-004`/`-010`; missing θ in
percolation → `HKERNEL-WB11-PERC-E-001`) are genuine and correct. These findings
concern the constitutive *gates*, not the framework around them.

## F-2 — The AUTH03 gates pass on the unresolved over-drainage code

Ran (reviewer): `cargo test --test auth03_level4_constitutive_gate_contract` →
4 passed / 0 failed. Static: `git diff --stat ec31875..HEAD -- crates/` is empty
— the AUTH01–04 series changed no production kernel/runtime code. The
soil-water over-drainage defect (H1 candidate Total-Soil ~72 mm vs legacy
~645 mm, soil at ~6% of porosity-store) is therefore still present at HEAD, and
the new required Level-4 gates pass over it. AUTH03's decision is `GO`.

## F-3 — The FC/WP gate validates fixtures against themselves; the model is never compared to an authority

Static. `auth03_fc_wp_fixture_vectors_preserve_constitutive_ordering_and_storage_closure`
reads hand-authored JSON fixtures and asserts (a) `porosity ≥ θ_fc ≥ θ_wp ≥ 0`
on the fixture values and (b) `Σ(θ_fc·dg)·1000` equals the fixture's own
`expected.profile_fc_store_mm`. In `nominal_case.json`, `θ_fc = {0.31, 0.29}`,
`dg = {0.20, 0.30}`, and `expected.profile_fc_store_mm = 149.0`
(`0.31·0.20·1000 + 0.29·0.30·1000 = 149`). The assertion checks `149 == 149`.

- No retention-curve / matric-potential computation exists in the codebase
  (Ran: `grep -rinE "van.?genuchten|retention|matric|pressure_head|brooks.?corey"`
  over `crates/` returns no constitutive computation). The `−33 kPa` authority is
  a label in the fixture; nothing computes θ at −33 kPa from the declared PTF.
- The model's FC computation (`02_soil_slope.rs legacy_correct_layer_moisture`,
  the `cpm` path) is never executed against an authority in this gate.
  `build_hillslope_runtime_surface_from_soil` is called only in the fail-closed
  guard test (F-1), never to compute and compare the model's FC.

Consequence: the gate cannot observe the model's FC being physically implausible
(e.g., FC at ~10% of porosity for a high-rock-fragment soil); it asserts that a
hand-authored fixture is internally arithmetically consistent.

## F-4 — The FC suite cites legacy as an external authority

Static. `cas_l4_soil_fc_minus33_001.md` `external_citations` includes
`EXT-SOIL-FC-LEGACY-001` → `/workdir/wepp-forest_260430_baseline/src/watbal.for`
(commit `dac3c95…`), "Baseline FC threshold usage." The Level-4 tier is defined
as external/physical authority for constitutive correctness; legacy WEPP is the
dependency the re-anchoring removes from acceptance authority (ADR-0011; governance
map §3). The FC suite lists it as a citation alongside the WEPP User Summary.

## F-5 — The relax-to-FC gate exercises a local kernel branch, not systemic relaxation

Static. `auth03_relax_to_fc_kernel_vectors_cover_cutoff_and_positive_branch` runs
`Wb11HydrologyKernel` percolation on a single-layer synthetic state seeded with
fixture-supplied `theta_m`, `fc_m`, `ul_m`, `ssc_m_s`. It asserts the cutoff
(`θ = fc → pei = 0`, `D = 0`; `near_fc_cutoff.json` supplies these expected
values) and the positive branch (`θ > fc → pei > 0`, `D ≥ 0`). It does not
exercise multi-day relaxation toward FC, and `fc_m` is fixture-supplied, not the
model's computed FC for a real soil. The positive-branch expectations are
`Option`-gated (`if … == Some(true)`), so a fixture omitting them skips that
assertion.

## F-6 — Exit-criterion 3 is not yet substantiated by the gate contents

Static. AUTH03 exit criterion 3 reads "Residual adjudication no longer depends on
parity-only acceptance logic: pass." Per F-3/F-4/F-5, the constitutive gates
adjudicate fixture self-consistency and a synthetic kernel branch, and the FC
suite cites legacy as authority (F-4). The gates do not yet compute a physical
authority independent of the fixtures, nor compare the model's outputs to one.

## F-7 — Evaluative standard for "teeth" (assessment criterion, not a remedy)

Static. The standard by which a constitutive gate is judged to have acceptance
authority is whether it red-lights a known defect. The over-drainage / low-FC
defect is documented and reproducible (governance map §1; H1 72-vs-645). The
AUTH03 gates pass over it (F-2). By that standard the gates do not yet
demonstrate acceptance authority for the constitutive families they are named
for; the gap is the absence of (i) an authority computed independently of the
fixture and (ii) a model-to-authority comparison on real soils.

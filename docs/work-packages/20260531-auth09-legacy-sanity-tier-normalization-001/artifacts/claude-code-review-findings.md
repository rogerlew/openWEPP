# AUTH09 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: legacy/sanity authority-tier normalization — `correctness-authority-model.md`,
`external-authority/suite-schema.md`, `registry.yaml`, the renamed
`cas_l3_subhyd_solwpv_fcdep_branch_001` suite, and (as ratified by this tier
normalization) the AUTH07 `cas_l5_soil_fc_direct_theta_minus33_cohort_001` suite.
Responds to the AUTH08A review findings.

Evidence: **Static** (read model/schema/registry/suite/test/fixture) and **Ran**
(commands executed by this reviewer).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — The legacy/sanity tier is added coherently (AUTH08A findings resolved)

Static. `correctness-authority-model.md` now defines the gate classes with the
hard invariant gates as a separate blocking class (`A1`: closure/bounds/domain,
required/blocking) and an external-authority Level ladder of Level-3 legacy/sanity
(`A2`, non-blocking investigation) → Level-4 constitutive (`A3`, blocking) →
Level-5 measured (`A4`) → Level-6 solver (`A5`). The WB19 branch suite is retiered
to `cas_l3_subhyd_solwpv_fcdep_branch_001`, `authority_level: 3`,
`gate_lane: periodic`, `failure_class: investigation`. This resolves AUTH08A F-2
(tier inversion — no longer Level-5/measured), F-3 (conflicting signals — id, level,
and content now agree on Level-3 legacy/sanity), and F-4 (missing tier — Level-3 now
explicit in the model and schema). Legacy/sanity sits at the floor of the
external-authority ladder, below Level-4 constitutive, and does not outrank the
A1 hard gates; the earlier concern that legacy would be ranked above conservation
is not present. A normative legacy-comparator demotion/retirement policy
(coverage-based, no "match legacy column X" gates) was added.

## F-2 — The FC=θ(−33 kPa) constitutive law is tiered non-blocking

Static. The AUTH07 suite `cas_l5_soil_fc_direct_theta_minus33_cohort_001`
performs the independent check that prior Level-4 suites did not: in
`auth07_fc_authority_cohort_contract.rs`,
`authority_fc_store_mm = Σ(fc_measured · thickness)` — the declared −33 kPa field
capacity, independent of the model's `cpm`/`coca`/`sm20c` pipeline — is compared
to `model_fc_store_mm = wb13_profile_fc_store_mm`. This is the correct authority.
It is classified `authority_level: 5` (measured), `gate_lane: periodic`,
`failure_class: investigation` → non-blocking. By the model's own definitions,
Level-4 (`A3`) is "constitutive physics laws not adjudicable by conservation
alone" (blocking) and Level-5 (`A4`) is "system-level behavior vs empirical
observations" (non-blocking). `model FC == θ(−33 kPa)` is a per-soil constitutive
identity, not system-level validation; it matches the Level-4 definition. As
tiered, the framework measures the field-capacity discrepancy and blocks nothing
on it, and no blocking Level-4 FC constitutive gate exists. AUTH09 is the
tier-normalization authority that ratified this placement.

## F-3 — The FC cohort test asserts the discrepancy is present (regression-pin, not a gate)

Static + Ran. `auth07_profile_fc_authority_cohort_threshold_and_rock_bucket_classification`
asserts `result.exceeds_threshold == case.expect_exceeds_threshold` (and the rock
bucket), not `model FC == authority`. `exceeds_threshold` is
`relative_error > max_relative_error_threshold` (threshold `0.35`).
`expect_exceeds_threshold` is hand-declared per case; the `valid_9002_reference`
case sets it to `true`, i.e. the test asserts the model FC is >35% from the
−33 kPa authority. The suite passes by confirming the discrepancy
(Ran: `cargo test --test auth07_fc_authority_cohort_contract` → 2 passed); a fix
that brought the model FC to the authority would flip `exceeds_threshold` to
`false` and fail this case. As an acceptance mechanism the test has inverted
teeth — it pins the current discrepancy. As investigation evidence it is
legitimate, and it independently corroborates the −33 kPa worked example: the
fixture encodes that a 9002 model FC exceeds 35% error against the declared
authority.

## F-4 — Minor: incomplete rename and a fixture-label inconsistency

Static. The rename to `cas_l3_*` is partial. The renamed suite doc
`docs/specifications/external-authority/suites/cas_l3_subhyd_solwpv_fcdep_branch_001.md:34`
still references `tests/fixtures/constitutive/cas_l4_subhyd_solwpv_fcdep_branch_001/…`
as its fixture `source_path`, and the fixture directory retains the `cas_l4_` name —
so the coherent-naming objective claimed for AUTH08A F-3 is not fully realized.
Separately, the cohort fixture case `h1_high_rock_authority` declares
`expected_rock_bucket: "low"` despite its name.

## F-5 — Scope and gates

Static. AUTH09 changes governance/docs/tests/fixture-metadata only (commit
`34b4e55` touches no `crates/`). Ran (reviewer, HEAD `34b4e55`):
`cargo fmt --check` → exit 0; `cargo deny check` → exit 0; `cargo clippy
--workspace --all-targets -- -D warnings` → exit 0; `cargo test --workspace` →
exit 0. Decision: `GO`.

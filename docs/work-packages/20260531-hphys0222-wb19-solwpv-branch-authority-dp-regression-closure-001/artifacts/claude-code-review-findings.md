# HPHYS0222 Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: WB19 `solwpv` branch-authority correction — the kernel change in
`crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`,
the `SC-WATBAL-001` addendum + `INV-WATBAL-009`, and the
`auth08_wb19_solwpv_fcdep_branch_constitutive_contract.rs` gate. Assessed against
the over-drainage root-cause analysis in the AUTH05 artifact
`claude-code-fc-authority-worked-example.md`.

Evidence: **Static** (read source/contract) and **Ran** (command executed).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — The fix is correct, narrow, and well-scoped

Static. The kernel change is 3 lines (`kernel_phases.rs`, 2 insertions / 1
deletion): the WB19 saturated-depth mutation guard changes from
`!solwpv_mode_is_2006` (`solwpv != 2006`) to `solwpv_mode_lt_2006`
(`solwpv < 2006`). Effect: `fcdep = fcdep - q/watyld` is no longer applied for
`solwpv >= 2006`, matching the `watbal.for` branch logic recorded in the new
`SC-WATBAL-001` "HPHYS0222 WB19 `solwpv` Branch-Authority Correction Addendum"
and `INV-WATBAL-009`. The prior guard applied the mutation for both `< 2006`
and `> 2006` formats; restricting it to `< 2006` is a legitimate, surgical
correction. Ran: `auth08_wb19_solwpv_fcdep_branch_constitutive_contract` → 2
passed.

## F-2 — The change is downstream of the documented over-drainage root cause

Static. The dominant over-drainage driver established by the −33 kPa authority
analysis is field capacity ~2× too low (H1 `ProfileFCStore` 107 mm vs physics
authority 223 mm; the `cpm` rock double-count in
`02_soil_slope.rs:legacy_correct_layer_moisture`). HPHYS0222 modifies WB19
saturated-depth bookkeeping (`fcdep`/`unsdep`), not the FC the profile drains
toward. It can remove a secondary water-removal path for modern-format soils but
cannot raise the field-capacity floor; the soil continues to relax to a
half-value FC.

## F-3 — A behavioral change to the test cohort was landed unmeasured

Static. `solwpv >= 2006` covers the 9001/9002-format soils that constitute the
`unpalatable-rind` 39-hillslope cohort (e.g. H1 is 9002), so this change alters
WB19 behavior for the cohort. The disposition states parity "was intentionally
out of scope for this package and has not been rerun post-fix," deferring the
39-hillslope semantic comparison to a follow-on package. Consequence: a
behavioral change whose sign and magnitude on the monitored residuals
(`Dp`/`latqcc`/`Total-Soil`/`SoilWaterTotal`) are unknown was committed under
`HOLD`; and because measurement is deferred, subsequent residual movement cannot
be attributed cleanly to this change versus later ones. The change plausibly
intersects the over-drainage families (it removes a saturated-depth reduction for
the cohort soils), which is the case where same-package measurement matters most.

## F-4 — The branch gate is filed at Level-4 but its authority is legacy + hand-authored fixtures

Static. The suite `cas_l4_subhyd_solwpv_fcdep_branch_001` is labeled Level-4
("l4") external-authority constitutive and is required/hard-fail under
`INV-WATBAL-009`. Its authority is the `watbal.for` branch rule (recorded in the
SC-WATBAL addendum) plus hand-authored `expected` values in
`solwpv_fcdep_branch_cases.json` (`status_code`, `q_m`, `watyld`, `fcdep_m`,
`unsdep_m`). The test asserts the kernel reproduces those expected outputs per
`solwpv` case. This is an algorithm/branch-conformance check anchored to legacy
behavior, not a physics-derived external authority. Per the re-anchoring policy
(`docs/governance/correctness-reanchoring-keep-condemn-map.md` §2–§3) legacy is a
sanity-check tier; classifying a legacy-conformance branch gate as Level-4
constitutive continues the AUTH03/AUTH05 pattern of populating the physics tier
with checks that do not derive from physics.

## F-5 — Gates and commit state

Ran (reviewer, from `/home/workdir/openWEPP`, HEAD `2694f9b`): `cargo fmt --check`
→ exit 0; `cargo deny check` → exit 0;
`cargo clippy --workspace --all-targets -- -D warnings` → exit 0;
`cargo test --workspace` → exit 0. Static: HPHYS0222 is committed (`2694f9b`),
decision `HOLD`.

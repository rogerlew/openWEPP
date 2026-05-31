# AUTH08A Review Findings — Claude Code

Reviewer: Claude Code
Date (UTC): 2026-05-31
Scope: re-tiering of the WB19 `solwpv` FCDEP branch gate
(`cas_l4_subhyd_solwpv_fcdep_branch_001`) — suite doc, `registry.yaml`, and
`suite-schema.md`. Responds to HPHYS0222 review finding F-4.

Evidence: **Static** (read suite/registry/schema).

This artifact records observations and evidence only. It does not propose an
implementation approach or a disposition.

---

## F-1 — The gate was correctly de-blocked (the right action on HPHYS0222 F-4)

Static. The suite is moved from `gate_lane: required` / `failure_class: hard-fail`
to `gate_lane: periodic` / `failure_class: investigation`, and retitled "WB19
`solwpv` FCDEP Branch Legacy-Conformance Suite," in both
`docs/specifications/external-authority/suites/cas_l4_subhyd_solwpv_fcdep_branch_001.md`
and `registry.yaml`. The suite note records it as "Legacy-anchored
branch-conformance suite; non-blocking investigation lane pending independent
constitutive authority." A legacy-anchored branch check should not block
acceptance as if it were a physics gate; this de-blocking is the correct
response to HPHYS0222 F-4.

## F-2 — The assigned authority tier is inverted

Static. The suite is set to `authority_level: 5`. `suite-schema.md` line 40
defines level 5 as "measured/system validation; default non-blocking unless
promoted" — the strongest external-authority tier. A legacy-conformance branch
check is the weakest authority class (a sanity check), so it is filed at the
measured-data tier. The non-blocking behavior is already produced by
`gate_lane: periodic` + `failure_class: investigation` (F-1); `authority_level:
5` was therefore not required to de-block and is an independent mis-assignment
that claims measured/system-validation authority for a legacy-anchored check.

## F-3 — The suite now carries three conflicting tier signals

Static. `suite_id: cas_l4_subhyd_solwpv_fcdep_branch_001` encodes level 4 in the
identifier; `authority_level: 5` encodes measured/system validation; the title
and notes encode "legacy-conformance." The three do not agree on the suite's
tier.

## F-4 — Structural: the taxonomy has no legacy/sanity tier

Static. The re-anchoring policy
(`docs/governance/correctness-reanchoring-keep-condemn-map.md` §2–§3) demotes
legacy to a sanity-check role, but the authority tiers run conservation/bounds →
analytic → component-physics (4) → measured (5) → independent solver (6), with no
tier below 4 for legacy-conformance / sanity. Legacy-anchored checks therefore
have no correct destination, and AUTH08A's de-tiering landed at level 5
(measured) for lack of a lower slot. The durable correction is a legacy/sanity
tier below level 4 with a default non-blocking lane, with legacy-anchored checks
routed there; re-tiering this single suite again would not resolve the gap for
the next legacy-conformance gate.

## F-5 — Scope and gates

Static. Governance/docs only (commit `fe8d344` touches no `crates/`). Ran
(reviewer, HEAD `fe8d344`): `cargo fmt --check` → exit 0; `cargo deny check` →
exit 0; `cargo clippy --workspace --all-targets -- -D warnings` → exit 0;
`cargo test --workspace` → exit 0.

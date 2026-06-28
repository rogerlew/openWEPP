# SNOWDENSITY-10.3.18 Cross-SNOTEL Mechanism Rubric Diagnostic

Status: complete.

Objective: apply the `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050` snow/frost
fidelity rubric across the five SNOTEL SWE/depth/density climates plus the bound
`cancov_forest` SWE/depth/density paired set, scoring the activated bundle,
current direct-runtime opt-in candidates, archival rejected candidates, legacy,
and PySnobal flag profiles. This is diagnostic-only evidence and makes
**NO promotion/activation decision**.

## Authority

- `docs/planning/snow-frost-fidelity-strategy.md` §10.2/§10.3, including
  10.3.17 non-promotion folded into this model list as an opt-in profile.
- `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-050`,
  `GAP-SNOWFREEZE-002`, and `REF-SNOWFREEZE-FROST-OBS`.
- `ADR-0017`: legacy/PySnobal are flags, not targets.
- `tests/fixtures/snotel_observed/` and
  `tests/fixtures/cancov_forest/observations/`.
- `tools/snowfreeze_observed/snotel_density_three_way.py` as the rubric lineage
  from SNOWFROST-FIDELITY-H.

No contract amendment is expected because this package consumes existing
evaluation authority and adds no new cross-SNOTEL gate authority.

## Scope

In scope:

- Run current direct-production WAT for the supported model mechanisms:
  `legacy_baseline`, `activated_bundle`, `harder_pomeroy_partition`,
  `open_sublimation_stage_a_10_3_16`, and
  `shallow_pack_guard_10_3_17`.
- Include rejected/archival candidates in the model list:
  `spring_densification_10_3_11` and `winter_thaw_state_loss_10_3_7`, with
  unavailable cells where the current selector path no longer supports a real
  direct-runtime rerun.
- Include PySnobal as a SNOTEL H flag profile where available; mark cancov cells
  unavailable rather than manufacturing a bridge.
- Decompose residuals directly into SWE, depth, and density.
- Report per-site x per-model x per-signature rubric cells, mechanism
  improvements by climate regime, humid-New-England representativeness, and a
  ranked next global lever read.

Out of scope:

- No production/default/cap/schema/fixture/frost change.
- No parser/runfile/user selector change.
- No site calibration, fixture fitting, or observed-row-conditioned runtime
  behavior.
- No promotion or activation decision.
- No treating legacy or PySnobal agreement as a target.

## Plan

1. Read strategy, contract, SNOTEL/cancov fixtures, SNOWFROST-FIDELITY-H lineage,
   and prior 10.3.17 disposition.
2. Add `tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py`.
3. Add a focused source-level integration guard.
4. Run the diagnostic against real direct-production WAT for supported models.
5. Record matrix artifacts, reviews, gate results, and disposition.

## Gates

- The diagnostic emits a per-model x per-site x per-signature matrix with
  forcing-robust (`R`) and forcing-limited (`L`) cells preserved.
- Absolute SWE/depth magnitude cells are report-only and do not carry verdicts.
- Legacy/PySnobal are scored as ADR-0017 flag profiles only.
- Whole package remains diagnostic-only with no protected-boundary edits.
- Unsupported rejected candidates remain explicit unavailable/archival profiles;
  they are not silently dropped.

## Execution Log

- [x] Required reading: strategy §10.2/§10.3, work-package/science-contract
  governance, `INV-SNOWFREEZE-050`, SNOTEL/cancov fixture docs, ADR-0017, and H
  rubric tooling.
- [x] Scaffolded package and diagnostic tool.
- [x] Ran diagnostic.
- [x] Ran focused validation.
- [x] Recorded reviews, gates, and disposition.

## Disposition

`DIAGNOSTIC-COMPLETE-NO-PROMOTION-DECISION`.

The ranked supported next-lever read is `harder_pomeroy_partition`, but only as a
diagnostic investigation rank. Prior 10.3.5c non-SNOTEL snow-control regression
evidence remains binding against promotion. The 10.3.17 shallow-pack guard stays
non-promoted; 10.3.16 sublimation is worse in this cross-corpus profile; and
humid-New-England cancov residuals are not representative of the mountain SNOTEL
activated-bundle fail signature set.

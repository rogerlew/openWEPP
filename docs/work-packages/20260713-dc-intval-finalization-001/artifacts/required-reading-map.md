# Required Reading Map

Status: `CURRENT`

Evidence class: **Ran + Static**.

## Initial instruction chain

- `AGENTS.md` — repository governance.
- `docs/work-packages/AGENTS.md` — package execution and closure rules.
- `tests/AGENTS.md` — integration-test and authority-suite gates.
- `docs/specifications/science-contracts/AGENTS.md` — contract and physics
  authority boundaries.
- `docs/defect_closure_execplans.md` — iterative defect-closure contract.
- `docs/standards/local-ci-gate-selection.md` — focused/heavy gate selection.
- `docs/work-packages/20260713-dc-intval-finalization-001/package.md` and active
  kickoff prompt — execution authority.
- `docs/work-packages/20260713-integrated-validation-campaign-001/package.md`,
  scenario matrix, gate results, and reading map — terminal restart matrix.
- `docs/work-packages/20260713-dc-intval-release-nextest-isolation-001` and
  `docs/work-packages/20260713-dc-intval-authority-provenance-001` dispositions
  — predecessor failure sequence.
- the seven active required suite documents under
  `docs/specifications/external-authority/suites/` named in `package.md` —
  binding behavior and fixture authority.

## Instruction discovery

Ran `tools/agents/find-agents --for` over the package, `Cargo.toml`, AUTH11,
and all five missing target paths. The applicable chains are root plus
`docs/work-packages/AGENTS.md` for package evidence, and root plus
`tests/AGENTS.md` for integration tests.

Add mechanism-specific source, contract, fixture, and nested instruction paths
here before any later production or contract expansion.

## INTVAL-EROSION-TOE-001 mechanism expansion

- `crates/AGENTS.md` and the `find-agents` chain for
  `direct_runtime/erosion_continuity.rs` — production Rust constraints.
- `docs/specifications/science-contract-authoring-procedure.md` and
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`
  — contract-first kernel behavior gate.
- `docs/specifications/science-contracts/contracts/SC-SED-001.md` — EROD16
  proximate erosion routing authority.
- `/workdir/wepp-forest_260430_baseline/src/profil.for` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` — terminal-station `slen` and
  `xstar` normalization authority.
- `tests/integration/erod16_wave1_continuity_fixture_conservation.rs` — existing
  EROD16 contract-derived integration surface.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_wave1_continuity.rs`
  — focused `profil.for` normalization vector and red/green regression owner.
- the candidate-2 stability report and first failing p34 run — runtime
  reproduction, not scientific authority.

## Candidate-3 stability-family expansion

- `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` and pinned
  `grow.for:529-601`, `init1.for:175-183`, `infile.for:539-541` — perennial
  root-cap ordering and zero-cap authority.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs` —
  current validation, root update, and local contract tests.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`, especially
  `INV-PERC-017` and `REF-PERC-LEGACY-HOURLY-FIN` — all-positive same-pass
  ingress authority.
- pinned `watbal_hourly.for:464-524` — no positive-epsilon omission of hourly
  `xfin`.
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`,
  pinned `input.for:668-675`, and pinned `perc.for:186-213` — exact-zero
  restrictive conductivity input and impermeable-boundary authority.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs` and
  `src/tests/tests_mod/direct_runtime_r4mo.rs` — producer, effective-
  conductivity, closure, and focused regression surfaces.
- candidate-3 `logs/07-release-candidate*` — family inventory and ordered
  reproduction, not science authority.

## INTVAL-FROST-THAW-CLEAR-001 mechanism expansion

- `crates/AGENTS.md` and the `find-agents` chain for the runner frost helper
  and runner tests — production/test Rust constraints.
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
  especially `INV-SNOWFREEZE-012` and the FDHP01 `frwatc` handoff rules —
  post-thaw fine-to-coarse liquid-state authority.
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md` — aggregate
  liquid storage includes `st + thetdr * unfrozen_depth` and cannot debit the
  residual store.
- pinned `frwatc.for` fine-to-coarse (`wbtofs=0`) handoff and `watcon`
  reconstruction — post-handoff target provenance.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs`
  and `01_frost_and_layer_helpers.rs` — current outcome/target call chain.
- focused p13/p14/p25/p27/p40/p43/p45/p49 release-CLI reruns — common runtime
  reproduction, not scientific authority.

## INTVAL-EROSION-CLASS-FRACTION-001 mechanism expansion

- `docs/specifications/science-contracts/contracts/SC-SED-001.md`, especially
  `INV-SED-017`, `TOL-SED-005`, and `TOL-SED-006` — enrichment composition,
  total-load authority, and publication bounds.
- pinned `enrich.for:300-385` — do-30 normalization, absolute `1e-15` class
  floor, `sedmax` cap, and label-50 redistribution lineage.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_enrichment.rs`
  — current straight-line port and bounded reproportion loop.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime_wave1_continuity.rs`
  — existing enrichment direction, zero-deposition, and inflow tests.
- candidate-5 OR-H0081/H0204 workdirs and GDB operand captures — runtime
  mechanism evidence, not scientific authority.

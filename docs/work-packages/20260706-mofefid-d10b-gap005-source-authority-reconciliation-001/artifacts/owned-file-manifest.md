# Owned File Manifest (D10B)

Status: executed

## Contract
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` (rev 24 + rev 25)

## Production (`ofe_routing`, shadow-tier subsystem)
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` (scheme corrections, ledger, celerity, BinRecorder, Manning limb, diagnostics)
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` (conservative handoff, ledger volumes, per-OFE mass exposure)
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs` (`run_iwagaki_manning`, diagnostics, test dispositions)
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing.rs` (module wiring)

## Validation tier (new)
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/iwagaki_oracle.rs`
- `crates/openwepp-hillslope-orchestrator/src/ofe_routing/d10b_reconciliation_tests.rs`
- `crates/openwepp-hillslope-orchestrator/examples/cascade_seam_ledger.rs`
- `crates/openwepp-hillslope-orchestrator/examples/iwagaki_oracle_dump.rs`
- `crates/openwepp-hillslope-orchestrator/examples/steady_probe.rs`

## References / governance
- `references/annotated_bibliography.md` (R-102, R-103)
- `references/rights_classification_first_pass_2026-05-11.md` (addendum)
- `references/copyrighted/19840021490.pdf` + `.md`, `Tseng2010_Hydroinformatics.pdf` (operator-acquired)

## Package + registries
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/**`
- `docs/work-packages/README.md`, `docs/ROADMAP.md`,
  `docs/planning/mofe-fidelity-campaign-strategy.md` (status rows)

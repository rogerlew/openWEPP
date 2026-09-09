Static + Ran: v34 received dual prospective PASS, then isolated implementation,
numerical/consumer admission and measurements completed. Engineering disposition
is REJECTED_IMPLEMENTATION; production HOLD. The planned sequence below is
preserved prospective history; current results are in worker-handoff.md.

Canonical design: SC-LANDSURFACEENERGY-001/inactive-jacobian.md. Review findings
were required to resolve before expected-red tests and J implementation.

Exact primary write set: LSE entry, new inactive-jacobian.md, binding-index.md,
history.md, lifecycle index.md, and existing
tests/integration/land_surface_energy_balance_authority_contract.rs. Existing
package artifacts record evidence; no production crate implementation edits.

Exact planned isolated J write set, relative to a fresh sibling J source copy:

- crates/openwepp-land-surface-energy/src/solver_inactive_jacobian.rs (new)
- crates/openwepp-land-surface-energy/src/solver_inactive_jacobian_tests.rs (new)
- crates/openwepp-land-surface-energy/src/solver_inactive_jacobian_oracle.rs (new; independent baseline-only reference and accepted operand reconstruction, separated from candidate author)
- crates/openwepp-land-surface-energy/src/solver.rs (private module registration)
- crates/openwepp-land-surface-energy/src/error.rs (typed integrity error)
- crates/openwepp-land-surface-energy/src/numerics.rs (private column provider)
- crates/openwepp-land-surface-energy/src/solver_covered_solve.rs (column assembly)
- crates/openwepp-land-surface-energy/src/solver_litter_phase.rs (V3 provider)
- crates/openwepp-land-surface-energy/src/solver_residual_corpus_capture.rs (direct-column method counts and bounded test helpers)
- crates/openwepp-runner/src/hillslope/tests03/controlled_mechanism_experiments.rs (actual runner parity/cost entry)

Discover nearest instructions at actual absolute J paths before edits. Private
provider defaults to canonical FD for existing generic callers. Capability
is minted only from current successful base and consumed within that sweep;
complete-column assembly precedes unchanged row adjustment. No Ax, public runtime
selector, unknown elimination, physics factoring or controller changes.

A observation correction write set remains the capture module and runner test
file only, with off/on complete solver outcomes and source-real failures. Preserve
frozen cut2 bytes and logs first, then label the correction as a separate cut.
Owner requests continuation through implemented J and measured actual-runner cost,
not target selection or capture-only disposition. Full acceptance stays current.

Prospective accepted-operand evidence split (observation only, untimed): extend
the J-only exact write set with src/solver_accepted_operand_audit.rs, src/lib.rs
(cfg(test/test-support) registration), src/transaction.rs,
src/transaction_v3_bridge.rs and src/transaction_v3.rs in the LSE crate. The three
transaction edits are cfg-only hooks after accepted operand/receipt construction;
Covered and litter hooks follow existing validation, while V3's constructed raw
energy set is independently validated by the audit (no existing whole-set V3
validation is implied). No physics, validation decision, return value or custody change.
Separate runner ignored acceptance entry brackets this bounded audit. Reuse
independent raw-operand energy/water/litter reconstruction validators, not
normalized residuals or output hash equality. Every observed record is checked;
whole-run checked counts/failures and bounded first/last identities bind coverage.
Overflow/error invalidates audit evidence without altering ordinary solver
outcome. No observer work runs in timed series. Report actual accepted physical
candidate construction, not a claim every candidate was final publication or
independent full soil-interior conservation. Existing real runtime consumer and
within-run publication/custody tests remain required. No general tracing system.

Dual-reviewed local-cost extension: J-only cfg(test)
src/solver_inactive_jacobian_cost.rs plus solver.rs registration. Frozen corpus
classification occurs outside timing. Each repetition prepares the same fresh
base; report inclusive base-plus-columns timing separately from disjoint complete
column spans. A pays canonical FD work, actual source displacement denominator
and complete rows, not J-only checks. J column spans include eligibility,
capability and scaled assembly. Retain row adjustment at the same location.
Complete matrix checksums and method counts prevent partial-work admission.
Use two warmup batches per arm, one common premeasurement calibrated repetition
count, then 30 balanced AB/BA pairs. Each measured column batch in both arms
must last at least 100 ms; otherwise preserve the invalid attempt and restart
with a larger frozen count. Never silently cap and accept short samples.
Only column-only acceleration may combine with a column-only runtime fraction;
base-inclusive ratios are labeled separately. Numerical and actual consumer
admission precede measurements. This is test-only, not a runtime selector.
First-source review clarifies that the implemented disjoint span charges
allocation, complete selected columns, full-matrix checksums, row adjustment
and deallocation. Report a charged-local-obligation ratio, not exclusive FD
construction acceleration. A matching whole-run fraction is UNMEASURED; do not
multiply by a narrow probe fraction or treat participation counts as time.
Before calibration, independently compare every case's consumed matrix/RHS
bits. Sequence-bound batch checksums prevent cancellation; aggregate checksum
equality is anti-elision evidence, never the mathematical parity oracle.

Reproduction helpers in this package: runtime-environment.json records minimal
nonsecret common execution environment; admit_identity_v34.py reuses the existing
collector without its F-specific frame-counter exception. Its exact identity
shortcut is not scientific admission and a mismatch requires v34 adjudication.
test_admit_identity_v34.py has three passing integrity tests. Existing collector
and analysis scripts remain unchanged; their historical standalone performance
predicate is not substituted for this package's canonical disposition rules.
source-a-cut2b.json binds the existing complete A source index, archive, exact
executable and refreshed sidecar. Every current retained source member was
rechecked before admission; one baseline admission process passed. For each
subsequent J series, independently reconcile every emitted direct_columns record
against admitted J counts as well as the existing collector's per-arm LSE and
carrier counts. The historical collector preserves this field but does not
validate it itself. Missing, malformed or changed counts invalidate that series.

Validation-composition correction after the first full-workspace compile stop:
copy unchanged primary tests/integration/support/sc_contract_text.rs and
tools/sc_contract_directory.py into J. Explicit J/.venv symlink reuses
/workdir/openWEPP/.venv (Python3.12.14), an external environment, not source.
Four missing historical provenance documents are copied byte-identically from
the retained predecessor package; j-directory-recovery-manifest.json records
paths/hashes. No existing document, parser behavior or canonical rule changes.

Restore directory-aware routing in six existing J tests/integration readers:
vegetation_boundary_authority_contract.rs,
surface_liquid_hydrology_custody_authority_contract.rs,
stage3_native_vegetation_laned_throughput_recovery.rs,
snow_stage3_terminal_receiver_authority_contract.rs,
solver_architecture_authority_contract.rs and
snow_stage3_shared_carrier_authority_contract.rs. Reuse the existing canonical
bridge and anchored-table recognition; preserve every scientific assertion.
Current LSE version pins become34, not stale31/32. Explicitly pinned historical
Git-object readers remain unchanged. This test-only integration must pass its
focused checks and full retry; it is not grounds to waive either gate. Earlier
source cuts/bundles remain preserved; amended test-source custody is required.
The same three current-version pins are reconciled in the primary copies of
stage3_native_vegetation_laned_throughput_recovery.rs and
snow_stage3_shared_carrier_authority_contract.rs (32→34 only); their canonical
reader routing already exists there. No primary production Rust is changed.

Reproducible implementation handoff extension: preserve
artifacts/reproduction/j-prototype-cut3.patch as an exact A-cut2b to J-cut3 diff
for the explicitly declared LSE/runner Rust write set. This is detached
experimental source evidence, not a patch against repository HEAD and not
production activation. Bind the existing A source identity and full J source
archive; authority/test-reader composition remains available in that full
archive. Verify patch application against copied exact A inputs and byte-match
the selected J outputs. No reconstructed historical index or new custody service.

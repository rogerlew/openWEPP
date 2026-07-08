# Execute WSHED-W7R Resume

Scope: resume
`docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/`
on current `main` after Lane D Tier 1 landed. This is the next authoritative
queue item before Lane D active baseflow export closure and before Lane D
watershed HBP hourly water/sediment consumption.

Execution mode: package-end-to-end. Do not stop at analysis unless a declared
hard blocker is reached.

Standing user authorization for openWEPP subagent delegation is expected in the
session:
`For openWEPP work-package execution, I explicitly authorize Codex to`
`spawn/delegate to subagents whenever the active work package, AGENTS.md, or`
`package governance requires or authorizes review, verification, comparator`
`execution, or parallel agent work.`

## Task

Resume WSHED-W7 and close the sediment-active watershed fixture/publication
package if current main can now supply a real nonzero sediment HBP producer.

First actionable item: prove on current release binaries whether the historical
`WSHED-W7-HOLD-001` producer-side blocker is gone. Use the W7DC01 substrate or
another real multi-OFE sediment-active source candidate and record current-main
HBP pass evidence for `tdet`, `tdep`, `sedcon_*`, and the paired minor-1 hourly
water/sediment surfaces if present.

If the producer proof passes, continue W7:

1. Adopt or generate one committed full watershed fixture with actual nonzero
   sediment response from production hillslope pass artifacts.
2. Run the public watershed supervisor on the accepted full fixture with
   `--jobs 1` and an appropriate parallel `--jobs N`.
3. Compare all required watershed parquet outputs for schema and row identity.
4. Prove public typed dispatch/publication consumes `WatershedPublicationFrame`.
5. Independently reconstruct sediment-sensitive publication operands and reject
   daily-scalar, zero-fill, parser-only, inventory-only, and self-consistency
   aliases.
6. Update artifacts, reviews, verification, gate results, disposition, roadmap,
   and package catalog.

If the current-main producer proof fails, do not fake fixture sediment and do
not execute `WSHED-W7DC01` as written. Record
`EXECUTED-HOLD-CURRENT-MAIN-HBP-SEDIMENT-PRODUCER` with evidence proving the
fresh blocker, why it is outside W7's watershed-consumer envelope, and the
first follow-on package/action.

## Required Reading

Core:

- `AGENTS.md`
- `crates/AGENTS.md`
- `tests/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/kernel-work-package-preparation.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `docs/architecture/watershed-runtime-architecture-specification.md`
- `docs/decisions/0032-watershed-runtime-ratification.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/package.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/artifacts/disposition.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260702-wshedw7-sediment-active-watershed-fixture-publication-closure-001/artifacts/required-reading-map.md`
- `docs/work-packages/20260702-wshedw7dc01-hillslope-sediment-production-hold-lift-001/package.md`
- `docs/work-packages/20260703-erosion-sediment-continuity-port-001/artifacts/increment-2c-entry-gate.md`
- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/final-disposition.md`
- `docs/work-packages/20260708-laned-router-tier1-local-numerics-001/artifacts/worker-handoff.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md`

Watershed fixture/publication context:

- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/package.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/review-disposition.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/scaling-matrix-evidence.md`
- `docs/work-packages/20260702-wshedw6-publication-large-watershed-scaling-001/artifacts/publication-operand-lineage.md`
- `tests/fixtures/watershed/carnivorous-adobo/README.md`
- `tests/fixtures/watershed/onshore-xenophobia/README.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`,
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`,
  and `docs/specifications/science-contracts/index.md` before any `SC-*`
  contract edit.
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`,
  `SC-IMPOUND-001.md`, `SC-RUNOFFPART-001.md`, and `SC-WATBAL-001.md` if the
  accepted fixture or publication path touches those surfaces.
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` and pinned
  baseline source only if the work becomes a legacy migration/parity package.

## Current Context

- Tier 1 local numerics landed at `SC-OFEROUTE-001` rev 47 and met the active
  H2637 timing target. Its only hold is the unratified Hirsch `Re^0.45`
  approximation envelope, which does not block WSHED-W7R.
- `WSHED-W7DC01` is historical/superseded as a package. Its diagnosed
  producer-side cause was the old multi-OFE EROD14/Wave-2 path. Later erosion
  work chained Wave-1 per OFE, produced real multi-OFE sediment on a W7DC01
  substrate, and deleted EROD14/Wave-2 as production authority.
- WSHED-W7R owns current-main verification before W7 fixture adoption resumes.
- Lane D active baseflow export closure is queued after WSHED-W7R. Do not
  implement baseflow, Lane D hourly HBP water/sediment watershed consumption,
  W8 channel-balance operands, W9 `NoEvent`, W10 `chan.inp` default authority,
  or watershed CQR in this package unless a package amendment is made before
  implementation.

## Constraints

- No subsets, representative slices, scratch-only evidence, or local `/wc1`
  paths as persistent acceptance artifacts.
- No surrogate sediment fills, proxy equations, manually edited pass sediment,
  fixture-only values, or compatibility wrappers.
- No production physics changes without canonical contract authority and
  contract-derived tests first.
- Preserve the W2-W6 architecture: typed run plan, bounded hillslope worker
  pool, typed pass inventory, typed network frame, direct watershed dispatch,
  and typed publication.
- Preserve `--jobs 1` as deterministic default; explicit `--jobs N` is only
  the CPU-scaling comparison.
- Producer-only, parser-only, inventory-only, counter-only, shadow-only, and
  direct-runtime-internal proof cannot close W7. The public watershed CLI and
  real output writer must consume the sediment-active typed path.

## Required Artifacts

Create or update package-local artifacts as execution proceeds:

- `artifacts/current-main-producer-proof.md`
- `artifacts/fixture-inventory.md`
- `artifacts/fixture-provenance.md`
- `artifacts/operand-lineage.md`
- `artifacts/publication-consumer-proof.md`
- `artifacts/serial-parallel-identity.md`
- `artifacts/sediment-reconstruction.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Run and record at minimum:

- Current-main release-binary provenance:
  `cargo build --release -p openwepp-runner --bins`, binary path, mtime/size or
  hash, and exact run commands.
- Current-main HBP producer proof on a real multi-OFE sediment-active candidate.
- Full accepted watershed fixture execution with `--jobs 1`.
- Full accepted watershed fixture execution with an appropriate parallel
  `--jobs N`.
- Parquet schema/row identity across required public outputs.
- Nonzero sediment public-output proof.
- Independent sediment reconstruction and anti-tautology/alias rejection.
- Source guards proving publication remains on `WatershedPublicationFrame`.
- Focused watershed CLI/publication tests added or updated by the package.
- `git diff --check`.
- Markdown-doc lint for touched docs.
- Contract/profile/BEI checks for any touched `SC-*` contract.
- `cargo fmt --check`.
- `cargo clippy --workspace --all-targets -- -D warnings`.
- `cargo nextest run --workspace --profile full`.
- `cargo deny check`.
- Source-level anti-evasion guards if any required-case binding, cohort
  fixture, or external-authority suite posture is touched:
  `bash tools/release/check_authority_suite_antievasion.sh` and
  `cargo nextest run --test auth11_required_suite_obligation_guards_contract`.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegating to `comparator_suite_runner`, `fixture_inventory_agent`,
`rust_code_reviewer`, `rust_qa_reviewer`, and `science_contract_reviewer`
subagents for W7R fixture discovery, release-binary runs, serial/parallel
output identity, review, and verification. Expected outputs are compact
metrics/findings plus artifact or log paths. Write access is bounded to
package artifacts for fixture/comparator agents; review and verification roles
are read-only unless the operator explicitly assigns implementation fixes.

## Commit/Handback

If complete: commit the W7 resume implementation, fixture/provenance, and
evidence together; report SHA, status, gates, and whether Lane D active
baseflow export closure is now unblocked.

If held: do not partially adopt a sediment-active watershed fixture. Record the
exact hold condition, evidence, why it is outside or not safely closeable in
W7R, and the first actionable follow-on package/action.

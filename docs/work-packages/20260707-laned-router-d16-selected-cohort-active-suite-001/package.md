# LANED Router D16 Selected-Cohort Active Suite

Status: EXECUTED-HOLD-ACTIVE-RUN

## Objective

Resolve the D16 selected-cohort active plain-vs-hybrid suite evidence hold at
the current mesh by constructing package-local, source-authorized active
openWEPP run directories for the selected cohort, running active plain and
explicit hybrid for each member, and adjudicating the timing, closure, and
publication-surface deltas.

This package does not flip the hybrid default selector unless every
production-facing gate is explicitly met inside this package. Final outcome:
no selector flip; the selected suite stopped at the Minnesota row-crop active
plain run because the active Rev-21 operand guard failed closed on positive LAI
with missing/non-positive typed `canhgt`.

## Rationale

The preceding Disturbed route-coefficient source-acquisition package closed the
missing-input authority blocker: WEPPpy Disturbed can now emit native
`ow-lanuse-1` management files with explicit `routing_coefficients`. It held
only because the selected active plain-vs-hybrid suite was not executed.

This package uses that new producer authority to convert the selected owcmp
roots from inventory-only evidence into package-local active run evidence where
their persisted `landuse.parquet` rows provide the disturbed class binding.

## Required Reading

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/contracts/openwepp-management-lanuse-authority-contract.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`
- `tools/owcmp/AGENTS.md`
- `tools/owcmp/specification.md`
- `tools/owcmp/suites/*.json`
- `tests/fixtures/AGENTS.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/package.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/final-disposition.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260707-laned-router-d16-hybrid-default-promotion-001/artifacts/timing-and-fidelity.md`
- `/home/workdir/wepppy/AGENTS.md`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/route_coefficients.py`
- `/home/workdir/wepppy/wepppy/nodb/mods/disturbed/disturbed.py`
- `/home/workdir/wepppy/wepppy/wepp/management/managements.py`

## Scope

Included:

- Select H2637 plus one representative hillslope from each current owcmp
  inventory root:
  - Minnesota corn: `/wc1/runs/al/algebraic-radium`
  - N Idaho single-OFE: `/wc1/runs/un/unpalatable-rind`
  - WA Cascades MOFE: `/wc1/runs/ar/arboreal-dendrite`
- Use `landuse/landuse.parquet` as the source binding for each external
  hillslope's `wepp_id`, disturbed class, management file, and run-root
  provenance.
- Use WEPPpy Disturbed's native management API to generate package-local
  native `ow-lanuse-1` management files with explicit route coefficients.
- Generate package-local openWEPP hillslope TOML runfiles for selected external
  hillslopes from source sidecars.
- Run active plain and explicit hybrid for each member.
- Record wall/user/sys timing, closure counters, route-coefficient provenance,
  output hashes, and plain-vs-hybrid deltas.
- Adjudicate whether the selected-cohort evidence resolves the D16 suite hold
  and whether a later D16/default-promotion package is unblocked.

Excluded:

- Broad no-env `OPENWEPP_LANED_ACTIVE` default activation.
- Durable `tools/owcmp/suites/` posture changes unless the evidence proves a
  reusable suite should be committed.
- New route-coefficient values, bridges from legacy WEPP fields, surrogate
  physics, mesh-resolution policy changes, or hybrid solver changes.
- Mutating `/wc1/runs/*` roots; generated inputs live under this package only.

## Authority Envelope

The package may generate native external-member management files only when all
of these are true:

- The selected root has `landuse/landuse.parquet` with `wepp_id`, `_map`,
  `man_fn`, and `disturbed_class`.
- `_map` is `disturbed`.
- The selected `wepp_id` has source sidecars under `<root>/wepp/runs/`.
- The route coefficients come from WEPPpy Disturbed's explicit native
  coefficient table/API, not from legacy row/ridge/random-roughness fields.
- The generated file is package-local and records source path, class, and
  coefficient provenance.

If those conditions fail for any selected member, do not invent a replacement.
Record an `EXECUTED-HOLD-*` status with exact evidence.

## Selected Cohort

Initial selected members:

| Member | Source | Selection rule |
|---|---|---|
| H2637 | Prior D16 active H2637 fixture | Reuse the package-local native H2637 active fixture from the default-promotion package as the high-runoff reference. |
| Minnesota corn | `/wc1/runs/al/algebraic-radium` | Select the lowest `wepp_id` whose `disturbed_class` is `agriculture crops` (`p4` in the current root). |
| N Idaho single-OFE | `/wc1/runs/un/unpalatable-rind` | Select the lowest `wepp_id` whose `disturbed_class` is `forest` (`p1` in the current root). |
| WA Cascades MOFE | `/wc1/runs/ar/arboreal-dendrite` | Select the lowest `wepp_id` whose `disturbed_class` is `forest` (`p1` in the current root). |

## Phase Plan

1. **S0 Scaffold and authority map.** Create the package, update catalog
   pointers, and record required reading.
2. **S1 Cohort materialization.** Build package-local active run directories,
   native management files, TOML runfiles, and provenance manifest.
3. **S2 Active execution.** Build the exact release runner, then run active
   plain and explicit hybrid for every selected member.
4. **S3 Adjudication.** Compare timing, closure metrics, output hashes, HBP and
   pass parquet summaries, and classify deltas against `SC-OFEROUTE-002`
   promotion posture.
5. **S4 Review and verification.** Complete review, disposition, verification,
   gate table, line-count governance, final disposition, and worker handoff.

## Subagent Authorization

Subagent authorization: this package explicitly authorizes
spawning/delegation to comparator/timing, science-authority review, package QA,
and verification subagents for selected-cohort run verification, delta review,
gate review, and package disposition review. Expected outputs are package-local
`artifacts/review-*.md`, `artifacts/verification-*.md`, and compact
command/comparator evidence. Write access is read-only unless a worker is
explicitly assigned a bounded package-artifact fix.

## Required Artifacts

- `artifacts/required-reading-map.md`
- `artifacts/cohort-selection.md`
- `artifacts/materialization.md`
- `artifacts/active-suite-runs.md`
- `artifacts/plain-vs-hybrid-deltas.md`
- `artifacts/timing-and-closure.md`
- `artifacts/implementation.md` or `artifacts/hold-legitimacy-audit.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

- `git diff --check`
- Markdown/doc lint for touched docs.
- `tools/owcmp/owcmp env --manifest` for the three existing owcmp inventory
  manifests.
- Package-local native management generation for every selected external
  member.
- Active plain run for every selected member.
- Explicit active hybrid run for every selected member.
- Closure evidence from each run manifest.
- Plain-vs-hybrid output-hash and metric delta evidence for each selected
  member.
- H2637-class timing run with active routed path enabled.
- Contract/profile/BEI checks if `SC-*` contracts are touched.
- Anti-evasion guards if required-case bindings, durable cohort fixtures, or
  external-authority suite posture are touched.
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

## Closure Outcomes

- `EXECUTED-COMPLETE-D16-SUITE`: all selected members run active plain and
  explicit hybrid, direct evidence resolves the selected-cohort suite hold, and
  remaining promotion posture is explicitly classified.
- `EXECUTED-HOLD-COHORT-MATERIALIZATION`: source-authorized active inputs cannot
  be generated for one or more selected members.
- `EXECUTED-HOLD-ACTIVE-RUN`: active plain or hybrid fails for valid generated
  inputs.
- `EXECUTED-HOLD-FIDELITY-TOLERANCE`: selected-cohort suite executes, but
  publication-surface deltas remain outside current promotion authority.

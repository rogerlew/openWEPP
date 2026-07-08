# Srivastava Groundwater/Baseflow Authority

Status: `QUEUED`
Package ID: `20260708-groundwater-baseflow-srivastava-authority-001`
Queue row: `M-T2A`
Owner: Codex
Scaffold date: `2026-07-08`
Evidence mode: `Static scaffold; no contract or implementation executed`

## Objective

Author the contract-first authority package for WEPP groundwater/baseflow before
any new openWEPP implementation. The package must bind the Srivastava
groundwater/baseflow lineage to canonical openWEPP `SC-*` authority, distinguish
groundwater-reservoir baseflow from lateral subsurface export and channel
`cbase`, and define obligations for both single-OFE and Lane D MOFE execution.

This package is authority-only unless execution explicitly amends a science
contract. It must not implement production physics, compatibility wrappers,
surrogate baseflow formulas, or silent fallback behavior.

## Authority Posture

Primary code authority for this package is:

- `/workdir/wepp-forest_260430_baseline` at
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`

The relevant Anurag Srivastava groundwater/baseflow code surfaces include
`gwcoeff.txt` parsing in `main.for`, groundwater storage/baseflow/deep-seepage
bookkeeping in `contin.for`, hillslope-pass propagation in `wshpas.for` and
`wshdrv.for`, watershed/channel consumption in `wshchr.for` and `wshcqi.for`,
and water-balance publication behavior in `watbalprint.for`.

Primary literature authority is Srivastava (2013) dissertation. Companion
authorities are Srivastava et al. (2013), Srivastava et al. (2017), and Dun et
al. (2009). `references/copyrighted/Srivastava2013.pdf` is the 2013 ASABE
paper, not the dissertation PDF.

## Required Reading

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `references/annotated_bibliography.md` entries R-21, R-22, R-22A, and R-70
- `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md`
- `docs/specifications/wepp-input-files/specs/gwcoeff.spec.md`
- this package's `artifacts/required-reading-map.md`

Authority sources:

- `/workdir/wepp-forest/references/Srivastava_Diss2013_14.pdf`
- `references/copyrighted/Srivastava2013.pdf`
- `references/copyrighted/Srivastava2017_ToASABE_wepp_streamflow.pdf`
- `references/copyrighted/dun2009.pdf`

Baseline code:

- `/workdir/wepp-forest_260430_baseline/src/main.for`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpas.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `docs/specifications/science-contracts/index.md`

On demand:

- `/workdir/wepp-forest_260430_baseline/src/wshred.for`
- `/workdir/wepp-forest_260430_baseline/src/inidat.for`
- `/workdir/wepp-forest_260430_baseline/src/wshinp.for`
- Current `/workdir/wepp-forest/src/*` only for non-normative comparison to
  the pinned baseline code authority.

## Scope

### Included

- Package-local scaffold, artifacts, prompt, and catalog/roadmap pointers.
- Authority-source inventory mapping each PDF and baseline source file to the
  contract decision it can support.
- Baseline code map for:
  - `gwcoeff.txt` parse and `lr_bf` branch selection.
  - `igwstrd`, `bfcoeff`, `dscoeff`, and `bftharea` units and domains.
  - Daily groundwater storage, recharge from deep percolation, baseflow, and
    deep seepage update order.
  - Hillslope-pass serialization of groundwater baseflow/deep seepage.
  - Watershed/channel consumption and threshold-area behavior.
  - Separation from `chan.inp` `cbase` unit-area baseflow.
- Contract design for a new or amended groundwater/baseflow `SC-*` authority.
- Single-OFE and Lane D MOFE implementation obligations for the later M-T2B
  package.
- Fail-closed policy for missing, malformed, or mixed groundwater/baseflow
  authority.
- Review, verification, disposition, and worker handoff.

### Excluded

- No production Rust implementation.
- No openWEPP runtime default change.
- No Lane D active routed surface-water change.
- No watershed HBP hourly water/sediment consumption change.
- No parameter fitting, calibration, or inferred coefficient defaults.
- No surrogate, provisional, empirical stand-in, or heuristic production
  groundwater/baseflow formulas.
- No wepppy edits.

## Intended Write Set

Package and catalog:

- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `references/annotated_bibliography.md`

Contract authority, only if executing this package beyond scaffold:

- New `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
  or an explicitly justified alternate contract name.
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md`
  only for parser-to-process linkage.
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`,
  `SC-OFEROUTE-001.md`, `SC-ROUTE-001.md`, and `SC-INFILE-CHANINP-001.md`
  only for boundary notes or cross-contract obligations.

Protected:

- No Rust production source files in M-T2A unless the package is explicitly
  amended before execution.
- No fixture, external-authority suite posture, or required-case binding edits.

## Phase Plan

### Phase A - Intake and Source Inventory

1. Record `git status --short --branch` and identify unrelated dirty files.
2. Confirm `/workdir/wepp-forest_260430_baseline` exists at
   `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
3. Verify all local PDF authorities exist and identify the exact document type
   for each source.
4. Complete `artifacts/authority-source-inventory.md`.

### Phase B - Baseline Code Map

1. Map `gwcoeff.txt` parsing and `lr_bf` branch selection.
2. Map daily groundwater storage, baseflow, and deep-seepage update equations
   from `contin.for`.
3. Map hillslope-pass and watershed/channel consumption surfaces.
4. Record namespace separation from `chan.inp` `cbase`.
5. Complete `artifacts/baseline-code-map.md`.

### Phase C - Contract Design

1. Decide whether authority belongs in a new `SC-GWBASEFLOW-001` contract or
   an amendment to an existing contract.
2. Draft required state variables, equations, units, timing, guards, invariants,
   branch/guard map, test-vector obligations, and Binding Exposure Index
   surfaces.
3. Explicitly define:
   - lateral subsurface export (`latqcc`);
   - groundwater-reservoir baseflow;
   - deep seepage;
   - channel `cbase` unit-area baseflow;
   - single-OFE behavior;
   - Lane D MOFE aggregation and active-router boundary behavior.
4. Complete `artifacts/contract-design.md`.

### Phase D - Disposition

1. If contract authority is complete, hand off M-T2B implementation with the
   exact contract target and first test vectors.
2. If authority cannot close, record `EXECUTED-HOLD-*` with the missing source,
   conflict, or scope boundary and the first actionable follow-on.

## Subagent Authorization

This package explicitly authorizes subagent spawning/delegation for read-only
science authority review and verification. Authorized roles:

- review: inspect literature/code authority mapping, contract placement, and
  no-surrogate-physics posture.
- verification: independently check source-path existence, quoted code-line
  mappings, and package gate claims.

Expected outputs are package-local `artifacts/review-*.md` and
`artifacts/verification-*.md`. Write access is bounded to this package's
artifact directory unless the operator explicitly expands scope.

## Required Artifacts

- `artifacts/README.md`
- `artifacts/required-reading-map.md`
- `artifacts/authority-source-inventory.md`
- `artifacts/baseline-code-map.md`
- `artifacts/contract-design.md`
- `artifacts/gate-results.md`
- `artifacts/review-*.md`
- `artifacts/verification-*.md`
- `artifacts/disposition.md`
- `artifacts/final-disposition.md`
- `artifacts/worker-handoff.md`

## Gates

Required for scaffold:

- `git diff --check`
- Markdown/doc lint for touched docs.
- Path-existence check for local PDF and baseline-code authorities.

Required for execution:

- Contract/profile/BEI checks required by touched `SC-*` contracts.
- Markdown/doc lint for all touched package, bibliography, roadmap, and
  contract docs.
- Source-line evidence for every baseline code claim.
- Review and verification disposition.

Not required unless implementation scope is explicitly added:

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo nextest run --workspace --profile full`
- `cargo deny check`

## Exit Criteria

`SCAFFOLDED`:

- Package directory, prompts, artifacts, bibliography, roadmap, and catalog
  pointers exist.
- Local PDF references and baseline code authority are named.
- No implementation or contract amendment is claimed.

`EXECUTED-COMPLETE-AUTHORITY`:

- The groundwater/baseflow `SC-*` authority is created or amended
  contract-first.
- Authority source inventory and baseline code map are complete.
- Review and verification findings are dispositioned.
- Worker handoff gives M-T2B exact implementation obligations.

`EXECUTED-HOLD-*`:

- Authority cannot be safely closed in this envelope.
- Hold audit names the exact missing/conflicting source, evidence, why it is
  outside or not safely closeable here, and the first actionable follow-on.

## Final Outcome

Queued scaffold. Execution has not started.

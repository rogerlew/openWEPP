# PL10b Blind Authoring Protocol

Status: `complete`
Evidence mode: `Static + Ran`

## Protocol

1. Phase 1 authority authoring is implementation-blind.
2. Allowed Phase 1 corpus:
   - `references/50201000/*`
   - `/workdir/wepp-forest_260430_baseline/src/*`
   - openWEPP governance/spec docs (`docs/specifications/**`, work-package docs)
3. Forbidden Phase 1 corpus:
   - openWEPP production/test Rust implementation files (`crates/**`, `tests/**`).
4. Contract amendment is authored and reviewed against baseline/procedure
   authority before any implementation conformance read.
5. Phase 2+ (test authoring and conformance execution) may read implementation
   surfaces.

## Attestation

Static:
- `SC-PLANT-001` PL10b amendment content (algorithm/guards/invariants/test
  vectors) was authored from legacy baseline + contract docs without reading
  openWEPP implementation code.

Ran:
- Baseline-source discovery commands executed against
  `/workdir/wepp-forest_260430_baseline/src` before implementation-read phase.
- openWEPP implementation files were first read only after blind-authority
  amendment completion.

## Phase Boundary Record

| Phase | Boundary condition | Result |
|---|---|---|
| Phase 1 | Blind authority draft complete in `SC-PLANT-001` and index update committed in working tree | `met` |
| Phase 2 | Contract-derived tests authored from contract assertions | `met` |
| Phase 3 | Implementation conformance run executed (ignored PL10b gate tests) | `met` |
| Phase 4 | Gap reconciliation + queue/dependency patches + disposition finalized | `met` |

## Source-Class Manifest (Phase 1)

| Source | Class | Role |
|---|---|---|
| `docs/specifications/science-contract-authoring-procedure.md` | procedure authority | review/disposition and governance gate requirements |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | procedure authority | required kernel-contract schema/checklist |
| `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | canonical contract | amendment target |
| `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` | canonical contract | date/cardinality domain authority |
| `/workdir/wepp-forest_260430_baseline/src/infile.for` | legacy baseline | annual/perennial payload read semantics |
| `/workdir/wepp-forest_260430_baseline/src/tilage.for` | legacy baseline | slot/crop transition-control assignment semantics |
| `/workdir/wepp-forest_260430_baseline/src/cutgrz.for` | legacy baseline | perennial harvest progression semantics |
| `/workdir/wepp-forest_260430_baseline/src/ptgrp.for` | legacy baseline | grazing window/cycle progression semantics |
| `/workdir/wepp-forest_260430_baseline/src/ptgra.for` | legacy baseline | annual event-day precedence semantics |
| `/workdir/wepp-forest_260430_baseline/src/decomp.for` | legacy baseline | residue transition controls (`jdburn/jdcut/jdmove`) |
| `/workdir/wepp-forest_260430_baseline/src/inidat.for` | legacy baseline | zero-sentinel initialization authority |

# Comparator Confidence-Tier Policy

Status: complete
Date: 2026-05-20 UTC
Evidence mode: `Static`
Ran evidence: none in this kickoff execution

This policy operationalizes ADR-0011 for ARCH-01 subsystem implementation gating.

## Tier Definitions

### Tier A (Higher Confidence)

- Surface class: single OFE and daily water-balance surfaces (`[DIRECT][Static]`).
- Comparator role: promotion-gating acceptance signal (`[DIRECT][Static]`).
- Delta handling: unexplained deltas block promotion until a disposition packet is completed (`[DIRECT][Static]`).

### Tier B (Lower Confidence)

- Surface class: hourly and watershed-integrated surfaces (`[DIRECT][Static]`).
- Comparator role: investigation signal (`[DIRECT][Static]`).
- Delta handling: open investigation and decomposition; do not auto-reject by comparator mismatch alone (`[DIRECT][Static]`).

## Required Disposition Metadata

Each comparator review record must include:

- `tier`
- `surface_id`
- `delta_signature`
- `first_divergence_surface`
- `first_divergence_timestep`
- `investigation_owner`
- `decision` (`accept`, `defer`, `investigate`, `block`)
- `evidence_mode` (`Static`, `Ran`, or mixed)

## Triage Workflow

1. Classify the divergence surface into Tier A or Tier B (`[INFERENCE][Static]`).
2. Check relevant invariants first (closure, physical bounds, contract constraints) (`[INFERENCE][Static]`).
3. For Tier A, require either:
   - root-cause resolution, or
   - explicit risk acceptance with a blocking follow-up gate (`[INFERENCE][Static]`).
4. For Tier B, create an investigation record and continue only if Tier-A invariants and contracts remain satisfied (`[INFERENCE][Static]`).

## Promotion Gate Rules

- Any unresolved Tier-A delta is a promotion blocker.
- Tier-B deltas are non-blocking by default but must remain visible in work-package disposition and follow-on plans.
- Comparator output is never a standalone universal oracle; contract and invariant compliance remains primary authority.

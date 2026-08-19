# Admit Root-Zone Hydraulic Owner V1

Status: `HOLD / explicit root-path source and unsaturated conductivity authority missing`

Date: `2026-08-19`

Package ID: `20260819-root-zone-hydraulic-owner-authority-001`

Plan class: `Critical contract-first kernel boundary authority`

## Objective

Admit `OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1` without changing V10 equations,
historical model bytes, restart wire, selectors, defaults, or outputs.

## Intended write set

This package, its implementation successor scaffold, the draft root-zone
contract and lifecycle registries. Production Rust is excluded until authority
release.

## Exact disposition

The requested candidate design is scientifically coherent, but the repository
cannot supply the required explicit per-occupancy/per-layer root-tissue path.
The only immutable root geometry is `lateral_root_length_m`, already consumed
as `dxroot`; aliasing it to `z3` is forbidden. The current subsurface
`conductivity_m_s` is also saturated/base conductivity rather than the required
current unsaturated soil-root conductivity.

This is a prompt-authorized authority contradiction, not a package-size or
implementation-effort HOLD. See `artifacts/final-disposition.md`.

## Lift criteria

1. Admit a non-defaulted input/configuration source for every required
   `(occupancy, OFE, layer)` root-tissue path.
2. Admit the exact unsaturated conductivity relation and parameter custody, or
   provide evidence that a different live field already has that semantics.
3. Freeze deterministic binary64 power semantics and independent vectors.
4. Complete the requested three reviews and two terminal verifiers before
   release.

## Delegation

Subagent authorization: this package explicitly authorizes spawning/delegating
to read-only soil-hydraulic, plant-hydraulic, Rust/numerical reviewers and two
read-only terminal verifiers after the two authority blockers are lifted.

# Science Contract Canonical Location

Status: Active
Last updated: 2026-05-20

## Canonical Path (Normative)

Canonical openWEPP science contracts must be stored in:

- `docs/specifications/science-contracts/contracts/SC-<DOMAIN>-<NNN>.md`

Canonical registry and lifecycle index must be stored at:

- `docs/specifications/science-contracts/index.md`

This directory is the single source of truth for `SC-*` authority files.

## Non-Canonical Artifacts

Work-package artifacts may include contract drafts, review packets, disposition
records, and verification evidence, but these are evidence surfaces rather than
authority location.

Required practice:

1. `disposition.md` must reference the canonical contract path and commit SHA
   under review.
2. A contract is not promotable if edits exist only in work-package artifacts
   and are not reflected in the canonical `SC-*` file.

## Layout

- `index.md`: science-contract registry with lifecycle metadata.
- `contracts/`: canonical `SC-*` markdown files.

## Naming

- Contract IDs: `SC-<DOMAIN>-<NNN>`
- Invariant IDs: `INV-<DOMAIN>-<NNN>`
- Stable reference form: `SC-<DOMAIN>-<NNN>#INV-<DOMAIN>-<NNN>`

# Reference Vendoring and Copyright Policy

- **Status:** Active
- **Date:** 2026-05-11

## Purpose

Define how openWEPP stores scientific references while respecting copyright and
maintaining reproducible provenance.

## Core rules

1. openWEPP prefers vendoring reference artifacts in-repo when redistribution
   is allowed.
2. Copyrighted/restricted artifacts must be kept as local cache only and must
   not be committed.
3. Every reference used for architecture or kernel work must be listed in
   `references/annotated_bibliography.md`.

## Rights classification

Use one of these classes per reference artifact:

- `redistributable`: explicitly allowed to redistribute (for example
  public-domain government works, permissive open-license documents, or
  documented rights-holder permission).
- `restricted`: redistribution not allowed or not yet confirmed.

Default classification is `restricted` until confirmed otherwise.

## Storage mapping

- `references/vendorable/` (tracked): `redistributable` artifacts only.
- `references/copyrighted/` (gitignored): `restricted` artifacts.
- `references/annotated_bibliography.md` (tracked): required for both classes.

## Bibliography maintenance requirement

Maintain `references/annotated_bibliography.md` as the canonical ledger:

- citation
- local path
- reference quality
- kernel/contract mapping
- notes/caveats
- rights/distribution status

When copying from another repo (for example `wepp-forest`), update repo-local
paths and distribution status during import.

Legacy imported entries that do not yet include explicit rights/distribution
status are allowed temporarily but must be backfilled as part of subsequent
reference edits.

## Intake checklist

1. Add or update bibliography entry first.
2. Classify rights (`redistributable` or `restricted`).
3. Place artifact in the matching directory.
4. For `restricted`, verify the file is ignored by `.gitignore`.
5. For `redistributable`, include source/license note in commit message or
   linked artifact note.

## Enforcement posture

- Committing restricted files is a policy violation.
- If rights status is ambiguous, keep the file in `references/copyrighted/`
  and proceed with metadata-only tracking until resolved.

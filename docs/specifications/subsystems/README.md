# Subsystem Specifications

This directory contains canonical, stable subsystem specifications for
openWEPP.

## Purpose

- Define module/subsystem boundaries and responsibilities.
- Define normative interfaces, invariants, and failure semantics that
  implementation work must satisfy.
- Keep stable contract surfaces outside work-package artifact trees.

## Canonicality Rule

- `docs/work-packages/.../artifacts/` hold draft artifacts and evidence.
- `docs/specifications/subsystems/<subsystem>/` holds canonical subsystem
  specifications after disposition.
- When a work package closes, it must record draft-to-canonical promotion
  mapping in its disposition artifact.

## Current Subsystems

- [observability/README.md](observability/README.md)
- [runner/README.md](runner/README.md)

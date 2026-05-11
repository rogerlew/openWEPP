# Legacy Source Attribution and Contributors Policy

- **Status:** Active
- **Date:** 2026-05-11
- **Derived from:** `/workdir/wepp-forest/docs/contracts/f90-migration-header-schema.md`

## Purpose

Define mandatory attribution/governance metadata for openWEPP Rust modules that
port or mirror legacy WEPP source units.

## Scope

This policy applies to Rust source files that are:

1. direct or near-direct ports of legacy `.for` / `.inc` units
2. clean-room replacements tied to sanitized specs and vectors

Purely new Rust orchestration code with no legacy unit mapping is out of scope.

## Required attribution block location

For in-scope Rust files, place the attribution block in module-level inner docs
at file top (`//! ...`).

## Source classes and required fields

### 1) WEPP Core Public-Domain migrations (non-clean-room)

Use when porting public-domain WEPP units without clean-room isolation.

Required fields:

```rust
//! SPDX-License-Identifier: CC0-1.0
//! Origin-Class: WEPP-Core-Public-Domain
//! Migration-Method: direct-port-fixed-to-rust
//! Replaces: src/<unit>.for
//! Contract-Spec: docs/specs/source/<unit>_spec.md
//! Original-Author(s): <from legacy source; use not-stated-in-source when absent>
//! Contributors: <names>
```

### 2) Numerical Recipes replacement migrations (clean-room)

Use only for clean-room replacement routines.

Required fields:

```rust
//! SPDX-License-Identifier: CC0-1.0
//! Origin-Class: Numerical-Recipes-Replacement
//! Migration-Method: clean-room-from-sanitized-spec
//! Replaces: src/<unit>.for
//! Clean-Room-Status: active
//! Spec-Source: docs/cleanroom/specs/<unit>_spec.md
//! Vector-Source: docs/cleanroom/vectors/<unit>_vectors.csv
//! Provenance-Note: docs/cleanroom/provenance/<unit>_provenance.md
```

## Author and contributor governance rules

### `Original-Author(s)` rules

1. Pull names from legacy labels such as `Author(s)` or `Author`.
2. If absent, set `Original-Author(s): not-stated-in-source`.

### `Contributors` rules

1. Include names from legacy attribution lines such as `Recoded by`,
   `Modified by`, `Added by`, `Updated by`, `Revised by`, `Verified by`.
2. Include anyone, or anything, whose substantive work is present in the unit.
3. Always include `Roger Lew` for non-clean-room migrations.
4. Deduplicate repeated names.

`Contributors` is one egalitarian field; do not split by role/substrate.

## Include-contract (`.inc`) migration note

When a legacy include contract is ported into Rust modules, add this field:

```rust
//! Sync-Constraints: <lockstep constraints, e.g., mxplan == ntype == ntype2>
```

Retain `Replaces`, `Contract-Spec`, `Original-Author(s)`, and `Contributors`.

## Worked example

```rust
//! SPDX-License-Identifier: CC0-1.0
//! Origin-Class: WEPP-Core-Public-Domain
//! Migration-Method: direct-port-fixed-to-rust
//! Replaces: src/watbal.for
//! Contract-Spec: docs/specs/source/watbal_spec.md
//! Original-Author(s): M. Reza Savabi
//! Contributors: Charles R. Meyer; Dennis C. Flanagan; Shuhui Dun; Erin Brooks; Jim Frankenberger; Codex; Claude Opus 4.7; Roger Lew
```

## Change control

Any changes to these fields or extraction rules must be coordinated with
`wepp-palimpsest` attribution policy to avoid governance drift.

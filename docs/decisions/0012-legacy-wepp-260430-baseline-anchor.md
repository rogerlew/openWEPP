# ADR-0012: Legacy provenance/comparator baseline is pinned to wepp_260430 hotfix snapshot

**Status:** Accepted  
**Date:** 2026-05-21  
**Deciders:** Roger Lew, Codex

## Context

openWEPP uses legacy WEPP sources for static provenance mapping and legacy
binary outputs for comparator investigation. The active `wepp-forest` HEAD
continues to move during defect remediation and work-package churn, which makes
"legacy baseline" references ambiguous unless the source and binary snapshot are
explicitly anchored.

For architecture-first contract and parser work, openWEPP needs one stable,
reproducible legacy baseline for:

1. static source inspection,
2. provenance citations in specs/contracts,
3. comparator baseline binaries and hash verification.

## Decision

1. The canonical legacy baseline for openWEPP provenance/comparator work is the
   dedicated worktree:
   - `/workdir/wepp-forest_260430_baseline`
   - alias path: `/home/workdir/wepp-forest_260430_baseline`
2. The canonical baseline source commit is:
   - `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`
   - commit message: `Fix wshpas leap-day truncation and re-release wepp_260430`
3. Canonical comparator baseline binaries and hashes are:
   - `release/wepp_260430`:
     `b11c94d8ba19deb941e4221977c4ec3437f41f612a2b816beacaca1b15563292`
   - `release/wepp_260430_hill`:
     `3b2fdd2b7a9e264b84f1e7b161dfb0730d49d3cb652218139efeb3ba17d7a160`
4. `wepp-forest` active HEAD (`/workdir/wepp-forest`) remains available for
   exploratory analysis, but normative contract/spec provenance and baseline
   comparator references must default to the pinned baseline worktree unless a
   deviation is explicitly justified with commit SHA evidence.
5. Changing the canonical baseline requires a new ADR (or explicit superseding
   ADR) with updated commit/hash evidence.
6. Binary pass serialization (HBP shard family) remains implemented based on
   `/workdir/wepp-forest` contract/implementation authority, especially:
   - `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md`
   - `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md`
   This ADR's pinned static-code baseline does not supersede HBP contract
   authority; HBP work packages must record the exact `/workdir/wepp-forest`
   commit SHA used for serialization/parser provenance.

## Consequences

- Legacy provenance references become reproducible and reviewable across work
  packages.
- Comparator investigations can distinguish baseline deltas from moving-source
  drift.
- Existing historical docs that reference `/workdir/wepp-forest` are not
  invalidated; they are treated as historical citations unless re-ratified
  against the pinned baseline.
- HBP serialization/parser evolution continues to track `/workdir/wepp-forest`
  contract surfaces, with explicit commit-SHA provenance in implementation
  packages.
- Teams must keep the baseline worktree available in development environments
  where provenance/comparator workflows run.

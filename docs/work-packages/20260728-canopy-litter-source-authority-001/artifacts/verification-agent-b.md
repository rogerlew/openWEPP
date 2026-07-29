# Terminal Verification B

Status: `FAIL`

Disposition: `HOLD`

Evidence class: `Static + Ran`

Reviewed snapshot: exact current implementation after explicit exhaustive
`measured_daily` record enforcement.

## Verified strengths

- The canonical contracts retain predictive evergreen-needle and fine-woody
  deposition as
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`.
- No stock-times-turnover surrogate, branch-mortality-to-deposition shortcut,
  pooled-carbon conversion, or external-source canopy debit was found.
- The external boundary rows in the ADR-0042 readiness matrix are correctly
  separate and use
  `IMPLEMENTED / NOT_APPLICABLE / NOT_APPLICABLE`.
- Path parsing verifies SHA-256 before parsing the strict forcing CSV.
- The real native run publishes separate leaf, needle, fine-woody, status,
  mode, and surface/interrill/rill state operands. Its fixture reconstructs
  the source sum and the three parallel no-action recurrences.

Ran:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: `6 passed, 0 failed`.

```text
cargo nextest run -p openwepp-runner \
  native_forest_yaml_executes_through_the_direct_production_consumer
```

Result: `1 passed, 220 skipped`.

```text
cargo nextest run -p openwepp-runner \
  canopy_phenology_02_real_consumers_share_the_typed_native_state
```

Result: `1 passed, 220 skipped`.

## Findings

### VB-01 — Authority and material bindings are not fully authenticated

Severity: `BLOCKING`

`ForestVegetationAuthority.checksum` is only required to be nonempty. It has
no typed digest algorithm, source object, claim anchor, or byte verification,
even though `not_applicable` depends on this classification. Derived forcing
similarly accepts a nonempty free-form `transformation_authority` and
`Vec<String>` inputs without typed identities/digests or admission control.
That is insufficient to prove separately admitted interval-to-daily
authority.

The material guard also checks only that a payload class occurs somewhere in
the top-level class list. In a mixed classification, a complete needle
payload can therefore select a broadleaf class, and a complete fine-woody
payload can select `non_woody`. This does not enforce the reviewed
tissue-to-functional-class compatibility.

Required closure: authenticate or admission-bind classification and
transformation authority; type every derivation input identity/digest; require
needle payloads to use needleleaf classes and fine-woody payloads to use woody
classes; add direct rejection tests.

### VB-02 — Contradictory provenance and dry-mass metadata can pass

Severity: `BLOCKING`

The parser validates original and executable support independently but never
relates them. An identity payload can claim original support different from
its executable support despite using the same source/executable object. The
access/version date is only nonempty rather than validated as `YYYY-MM-DD`.
An `oven_dry` payload also needs no drying duration or constant-mass endpoint.

Required closure: require identity support equality and exact-daily identity
semantics; constrain any derived support to its admitted transformation;
validate typed access/version dates; require a complete drying endpoint; add
contradiction tests.

### VB-03 — Missing tissue is published as numeric zero

Severity: `BLOCKING`

`direct_production_tissue_litter` maps `not_represented` to `0.0`, and the
research trace requires the tissue operand and combined total to be finite
numbers. This conflicts with the canonical rule that `not_represented` is
disclosed incompleteness, never numeric zero. A status string distinguishes
the state, but the numeric total can still be interpreted as source-complete.

Required closure: keep any internal neutral arithmetic separate from
publication; publish a nullable/typed unknown operand or an explicit
incomplete-ledger/total status, and prove an incomplete source cannot be
labeled complete.

### VB-04 — Contract-derived vectors do not cover the frozen rejection set

Severity: `BLOCKING`

The six-test suite proves contract anchors, one prescribed identity case,
wrong digest, interval-as-`measured_daily`, missing measured dates, and CRLF
rejection. It does not directly prove exhaustive measured-daily acceptance
and observed zero, status/payload contradictions, authenticated
`not_applicable`, tissue/material/site/OFE mismatches, malformed mass/date/
support/path cases, identity/derivation contradictions, duplicate or omitted
parallel projection, external canopy debit rejection, or source-completeness
publication.

Required closure: add direct positive/negative vectors for every applicable
contract family, or cite exact existing test evidence for a reviewed
not-applicable disposition.

### VB-05 — Cover/erosion consumer closure stops at adjacent state

Severity: `BLOCKING`

The real fixture proves the source reaches post-decay interrill and rill
masses. Static inspection shows cover is derived from those masses and erosion
executes, but the trace/test does not publish and independently reconstruct
the interrill/rill cover operands or assert that the real erosion consumer
receives those exact values. The surface-to-depth and frost checks are
credible; the required cover/erosion operand proof remains incomplete.

Required closure: capture/publish the two residue-cover operands, reconstruct
them independently from the authenticated fixture, and assert the real
erosion call consumes them.

### VB-06 — Terminal package and gate evidence is stale

Severity: `BLOCKING FOR CLOSURE`

- `gate-results.md` and `contract-test-implementation-evidence.md` report
  `5/5`, while the exact suite now has six tests.
- `line-count-governance.md` reports `forest_litter.rs` at 797 lines and the
  contract test at 184; exact counts are 853 and 196.
- The recorded warnings-denied Clippy/full-profile run predates the latest
  schema/test edit and is not exact-head evidence for this snapshot.
- Markdown validation, dual terminal disposition, exact-diff reconciliation,
  and final closure remain pending.
- `final-disposition.md` still says the package stopped before contract and
  production edits and that the interface is not implemented.
- `verification-agent-a.md` remains `NOT RUN`.
- Root `Cargo.toml` and `Cargo.lock` changed for this increment but are named
  only in the owned-file manifest, not the declared package write set.

Required closure: correct implementation findings first, reconcile the
declared write set and exact diff, refresh line counts and all package
artifacts, rerun applicable terminal-diff gates, complete both independent
verifications and Markdown validation, and replace the stale
authority-synthesis disposition with a truthful implementation disposition.

## Verification disposition

The predictive-science stop-loss is correct and must remain. The authenticated
external-boundary implementation is not terminally closed on this snapshot.
Any one of `VB-01` through `VB-06` is sufficient to retain package `HOLD`.

## Final Terminal Re-verification — 2026-07-28

Status: `PASS`

Disposition: `PROCEED TO FINAL PACKAGE DISPOSITION`

Evidence class: `Static + Ran + terminal receipt`

This section preserves the immutable initial `FAIL / HOLD` above and verifies
the corrected exact tree independently.

### Correction verification

1. `VB-01 — CLOSED.` Vegetation classification now names an immutable
   relative source object, date, claim anchor, digest algorithm, and SHA-256.
   The parser verifies the exact classification CSV bytes and requires the
   inline class sequence to match. Needle payloads accept only needleleaf
   classes; fine-woody payloads reject `non_woody`. Derived and interval
   execution is rejected outright in this identity-only increment, so no
   free-form transformation metadata can authorize temporal disaggregation.

2. `VB-02 — CLOSED.` Identity forcing requires exact equality between
   original and executable support, exact-daily resolution, identical
   source/executable path and digest, and a parsed authority date. Every dry
   mass record now requires either positive drying duration or a nonempty
   constant-mass criterion.

3. `VB-03 — CLOSED.` Noncomplete tissue publishes a JSON `null` operand.
   `source_completeness` is independently derived and validated from tissue
   statuses: either applicable `not_represented` tissue makes it
   `incomplete`; complete or authority-backed inapplicable tissue does not
   masquerade as missing numeric zero. The real unrepresented-source run
   asserts both null operands and the incomplete aggregate status.

4. `VB-04 — CLOSED.` The contract-derived suite now contains 16 tests. It
   covers both admitted daily modes and explicit measured zero, classification
   and forcing digest authentication, interval/derivation rejection,
   exhaustive support, canonical bytes, duplicate dates, negative mass,
   tissue/class compatibility, support and drying contradictions, site/OFE
   mismatch, path escape, and status/payload contradiction. The real-run test
   adds outside-support, null-publication, exact source-sum, and consumer
   vectors; the source guard proves the one-handoff/no-readdition topology.

5. `VB-05 — CLOSED.` The authenticated real fixture independently
   reconstructs all three post-decay pools, weighted ground mass,
   interrill/rill/composite cover, and residue depth. The active erosion
   implementation records the exact `DirectWave1DailyState` canopy,
   interrill, and rill cover values after the real erosion call; the trace
   proves those consumer values equal the independently reconstructed residue
   values. Active frost records the exact thermal residue depth and canopy
   height supplied to its real compute path, and the test proves nonzero
   leaf-off reaches that consumer.

6. `VB-06 — CLOSED.` The declared write set now includes root `Cargo.toml`
   and `Cargo.lock`. Artifacts report the current 16-test suite, corrected
   implementation, accepted terminal findings, and terminal receipts. Exact
   line counts checked in this verification are 948 for `forest_litter.rs`,
   397 for the contract test, 2,984 for the extracted input parser, 2,890 for
   runner `03_tests.rs`, 2,844 for
   `00_builders_and_authority.rs`, 110 for
   `tests03/canopy_litter_boundary_helpers.rs`, and 164 for
   `00e_native_canopy_trace.rs`. No touched nonexempt Rust file is at or above
   3,000 lines.

### Independent focused execution

Ran on the corrected tree:

```text
cargo nextest run --test canopy_litter_external_boundary_contract
```

Result: `16 passed, 0 skipped`.

```text
cargo nextest run -p openwepp-runner \
  native_forest_yaml_executes_through_the_direct_production_consumer
```

Result: `1 passed, 220 skipped`.

```text
cargo nextest run -p openwepp-runner \
  canopy_phenology_02_real_consumers_share_the_typed_native_state
```

Result: `1 passed, 220 skipped`.

```text
cargo nextest run -p openwepp-hillslope-orchestrator \
  r7b_constructor_type_size_layout_is_bounded
```

Result: `1 passed, 410 skipped`.

Terminal receipts additionally record:

- `cargo clippy --workspace --all-targets -- -D warnings` — `PASS`;
- exact-head full correctness — `2,117 passed`, `29` profile-declared skips,
  `757.402 seconds`.

### Final verification disposition

`PASS`. No blocker from `VB-01` through `VB-06` remains. The authenticated
identity-only prescribed/exhaustive-daily boundary, null incompleteness
publication, parallel residue closure, and real cover/depth/erosion/frost
consumer claims are supported by the corrected exact tree.

This pass does not lift or narrow the predictive-science stop-loss.
Evergreen-needle and fine-woody predictive deposition remain:

```text
AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED
```

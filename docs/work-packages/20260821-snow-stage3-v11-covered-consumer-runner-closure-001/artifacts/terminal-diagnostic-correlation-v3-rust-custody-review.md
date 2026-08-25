# Terminal diagnostic correlation V3 Rust/custody review

Evidence class: `Static`

Reviewed frozen inputs:

- authority SHA-256
  `5f9c0d66c13e3b11b921822c114877d1a61f233ae9e20c1164c21c9440c544e4`;
- adapter-manifest SHA-256
  `5209dd6b80c54563755d92ced9ce367df4708053ba9e4a28c4b23882064075db`.

Both hashes matched the requested files. I did not read or communicate with
the other V3 reviewer. This review changed no source, authority, manifest,
package or candidate-manifest file.

## Static custody and API assessment

The high-level private architecture remains feasible. The provider closure in
`snow_stage3_v11_terminal_execution.rs` owns each complete
`CoveredCarrierPhaseResultV1`; `evaluation.rs` owns coupling convergence and
selection; and `terminal_event.rs` owns adaptive trial position, pair error,
retry and floor admission. A private sealed mode, a zero-sized `NoEvidence`
monomorphization, fixed-size keys, and a crate-private `cfg(test)` capture path
can preserve the existing public and `pub(crate)` wrapper signatures. No
external integration test, feature, callback, global recorder or unwind
boundary is required.

V3 also correctly separates append-time provider evidence from coupling
iteration, coupling selection, and terminal-solver hydrology joining. Its live
role discriminants match `CoveredTerminalTrialRoleV1`, including `Retry`,
`BracketLower`, `BracketUpper`, and `Root`. The floor branch in
`terminal_event.rs` is genuinely pre-provider, so a separate admission record
can prove unchanged provider-call cardinality.

Those architectural improvements do not make the frozen wire implementable.
The companion manifest is not a closed adapter schema for the live Rust types,
and one required record violates V3's own ownership split.

## Findings

### Critical

1. **The adapter manifest does not specify the actual live receipt fields and
   therefore defers schema decisions to implementation.** A7 says that
   `TerminalSnowSoilTrialReceiptV1` contains a schema version, beginning snow
   and soil digests, one snow temperature, one soil-interface temperature,
   conductivity, path length, `q_ss`, a top-boundary-credit digest, and a
   production receipt digest. The live type contains none of the schema,
   beginning-state digest, conductivity, path-length, `q_ss`, or
   top-boundary-credit fields. It instead contains `ofe_id`,
   `canonical_source_sha256`, separate beginning and ending snow and soil
   temperatures, and `ending_soil_candidate_sha256`. A7 also describes
   `TerminalSnowSoilHeatReceiptV1` as containing residual and input/output
   owner digests, while the live type contains `ofe_id`, beginning snow,
   ending dormant snow, ending soil, and limiting-boundary receipt digests and
   no residual field. These are not equivalent-name differences. An
   implementation would have to invent derivation, omission, or source-capture
   rules forbidden by the manifest's completeness rule.

2. **A8 is likewise not a closed encoding of the live soil types.** The live
   `SoilThermalTopBoundaryCreditV1` includes `lane_id`, string-backed `OfeId`,
   `SoilLayerId`, `ResourceOwnerId`, configuration and state SHA-256 values,
   signed `i64` support endpoints, two distinct energy values, and a snow--soil
   receipt SHA-256. A8 instead specifies generic OFE/layer/support/heat and
   beginning/ending soil digests, without field order, string encoding, signed
   integer encoding, or the actual distinct operands. The common encoding
   defines only unsigned integers and incorrectly cannot encode the live
   signed support fields. `SoilThermalSnapshot` is also delegated to
   "production declaration order" rather than enumerated. This fails the
   requested exact widths, byte order, and no-deferred-field-list boundary.

3. **A9 contradicts the V3 record ownership boundary.** The authority says the
   provider-owned `TerminalCarrierPhaseRecordV3` includes A9 hydrology replay
   while excluding any hydrology-complete ending joint, and says that joint is
   created only later by the terminal solver's existing
   `join_hydrology_ending`. A9 mandates "the exact hydrology-complete ending
   joint as A2." A provider-time carrier record cannot possess that later
   terminal-solver-owned value. Including it recreates TDCV2-NUM-006;
   excluding it violates the frozen manifest.

### Major

1. **Several other adapters use open-ended "declaration-order" placeholders
   instead of a frozen field schema.** A4 does not enumerate the live
   `Stage3PrecipitationPhaseParcelSetV1` fields (including `u16`
   `schema_version`, `ofe_id`, ground-basis flag, destination records and the
   tagged enthalpy-provider variants). A5 does not enumerate
   `Stage3SnowCoveredLowerBoundary`, `CoveredCarrierInitialGuessV1`,
   `CoveredLseIterationState`, `CoveredCarrierComponentState`, or the variants
   and nested candidates in `UncommittedCoveredV8OwnerEnvelope`. The manifest
   provides no encoding for `u16`, string-backed IDs, several nested enums, or
   private owner-envelope variants. "Declaration-order thermodynamic/source
   fields" is an implementation choice, not a canonical schema.

2. **The named six-file implementation boundary is insufficient for the
   manifest's demanded adapters.** The forwarding chain can carry fixed-size
   keys, but the required payloads are owned across
   `snow_stage3_v11_precipitation.rs`, `v11_covered/receipt_sets.rs`,
   `land_surface_energy_shadow/covered_v8_owner.rs`,
   `v11_covered/physical_outcome_ledger.rs`,
   `v9_real_consumer_shadow.rs`, and external LSE types. Several relevant
   fields and enums are module-private and have no complete replay accessor.
   Implementing the frozen manifest from only the listed forwarding files is
   impossible without either unauthorized owner-module/API edits or
   approximate reconstruction. A later exact-file intent cannot cure a defect
   in what the reviewed authority currently claims is a complete forwarding
   chain and closed manifest.

3. **The canonical wire is incomplete beyond A1--A10.** The authority names
   keys, iteration records, selection records, selected-trial records, pair
   decisions, admissions, and the rejected-prefix record, but neither frozen
   file gives their exact framed field tags, option/sequence nesting, key
   encodings, selection-proof encoding, typed-error discriminant encoding, or
   final physical-result byte encoding. The manifest's common rules do not
   resolve those choices. Consequently post-return resolution cannot produce
   one uniquely specified byte stream or prove "exact physical error bytes"
   without implementation-time schema invention.

### Minor

1. **The claim that every captured physical float is finite is broader than
   the existing Rust admission proof.** Capture includes diagnostics and
   nested owner/receipt fields from multiple subsystems, while the manifest
   does not cite a validator establishing finiteness for every listed value.
   The wire should define rejection of nonfinite diagnostic evidence after
   physical return or specify raw-bit support for it; it must not assume a
   cross-type invariant not established by the forwarding path.

## Noninterference and compilation-boundary disposition

The selected crate-private `cfg(test)` boundary is sound in principle:
library unit tests see crate-private items, whereas a separate integration
crate does not inherit the library's `cfg(test)`. A sealed private generic
core can keep mode parameters out of existing externally reachable signatures,
and `NoEvidence` can avoid arena allocation and runtime mode selection. Exact
machine-code identity is not implied by generic refactoring, so the eventual
claim should remain behavioral/API/call-order noninterference unless codegen
identity is separately measured.

The post-return failure boundary is also feasible only after capture-time work
is reduced to infallible movement/cloning and append operations. Canonical
serialization, arena resolution, digest validation, assertions, and artifact
I/O can occur after retaining the physical result and beginning witness. The
frozen manifest gaps currently prevent defining what must be retained and how
it is serialized; they are authority defects, not implementation details.

## Recommendation

**HOLD**

The architecture remains feasible, but the exact frozen V3 authority and
adapter manifest do not authorize an implementable, uniquely encoded custody
path. The A7/A8 live-type mismatches, A9 ownership contradiction, incomplete
nested schemas, insufficient exact-file boundary, and missing non-carrier
record encodings must be corrected in a reviewed successor before any source
edit or implementation intent. This HOLD authorizes no correlation seam,
receipt capture, matrix, temporal operator, Batch V2, event, receiver, restart,
runner, Child 3 or cutover work.

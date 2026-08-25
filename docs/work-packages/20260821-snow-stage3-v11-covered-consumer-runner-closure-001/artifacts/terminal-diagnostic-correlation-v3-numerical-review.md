# Terminal diagnostic correlation V3 numerical review

Evidence class: `Static`

Reviewed SHA-256:

- authority: `5f9c0d66c13e3b11b921822c114877d1a61f233ae9e20c1164c21c9440c544e4`;
- adapter manifest: `5209dd6b80c54563755d92ced9ce367df4708053ba9e4a28c4b23882064075db`.

Both hashes matched. I made no source, authority, manifest, package or candidate
manifest edit and did not read or communicate with the other V3 reviewer.

Recommendation: **HOLD**.

## Static reachability assessment

The V3 ownership decomposition is structurally aligned with the live call
graph. The provider can append an immutable carrier-phase record; the coupling
loop in `evaluation.rs` can append iteration records and emit selection only
after convergence; `terminal_event.rs` can bind the selected trial after
`join_hydrology_ending`, assign pair position independently of live provider
role, and emit a later pre-provider floor decision. Fixed-size keys can carry
that identity through the existing forwarding chain without making evidence a
physical operand.

The role tags and adaptive mappings now match the live enum and loop:
`Full/Retry` occupy COARSE, `Half1` occupies FINE_1, and `Half2` occupies
FINE_2. The prior pair remains `REJECT_RETRY`; the following
`BelowCarrierDomain` branch occurs before a provider call. The error formulas,
canonical component order, binary64 left fold, and first-bitwise-equal winner
also freeze the V2 numerical ambiguities. The winner is correctly diagnostic
only.

## Findings

1. **Critical — A9 reintroduces the exact ownership conflation that
   TDCV2-NUM-006 is meant to remove.** The authority says
   `TerminalCarrierPhaseRecordV3` contains A9 while excluding any
   hydrology-complete ending joint. A9, however, normatively says that after
   the child/parent WB14 replay there follows "the exact hydrology-complete
   ending joint as A2." In the live chain that joint is available only after
   `terminal_event.rs` applies `terminal_transition` and invokes
   `join_hydrology_ending`; it is not provider-owned at carrier-record append
   time. Implementing A9 literally would require delayed mutation or would put
   terminal-solver-owned evidence back into the immutable provider record.
   Omitting that A2 would violate the frozen manifest. This is a direct
   contradiction between the two reviewed files, not an implementation
   detail.

2. **Critical — A1--A10 are not a complete implementation-freezing schema.**
   Several adapters name broad production values followed by "declaration
   order" or semantic summaries instead of listing their actual fields,
   widths, variant payloads and nested ordering. A5 is the clearest example:
   it does not enumerate `UncommittedCoveredV8OwnerEnvelope` or its nested
   owner variants, and it describes `CoveredLseIterationState` using residual,
   active-set, convergence and digest fields that the live struct does not
   have. Its actual fields include variable-length component temperatures and
   `CoveredCarrierComponentState` values, whose string and option encodings are
   not completely specified by A5. A8 likewise delegates a
   `SoilThermalSnapshot` to production declaration order without enumerating
   its variant/collection payload. A schema whose bytes depend on inspecting
   declarations during implementation has deferred the very field-list
   judgment V3 claims to freeze.

3. **Critical — A7 does not encode the live receipt it names.** The frozen
   A7 field list for `TerminalSnowSoilTrialReceiptV1` includes a schema
   version, beginning state digests, path length, conductivity, `q_ss`, and a
   top-boundary-credit digest. The live receipt instead contains support,
   lane, `OfeId`, canonical-source digest, four beginning/ending snow/soil
   temperatures, snow and soil heat, ending-soil-candidate digest, and receipt
   digest. Conversely, A7 omits the live `OfeId`, canonical source, ending
   temperatures and exact candidate field. The subsequent
   `TerminalSnowSoilHeatReceiptV1` summary also does not enumerate its live
   limiting-boundary and owner digest fields. There is therefore no canonical
   byte representation for the actual typed receipts without inventing an
   adapter or manufacturing unavailable fields.

4. **Major — A4 does not match the live precipitation parcel-set shape.** The
   live `Stage3PrecipitationPhaseParcelSetV1` has `schema_version:u16`, support,
   lane, `OfeId`, an OFE-ground-basis boolean, beginning-snow and topology
   digests, destinations, parcels and receipt digest. A4 instead describes a
   lane-count wrapper and source/interception/snow-ground/liquid-ground totals,
   does not freeze the live destination and parcel variant fields, and globally
   states that every adapter begins with `schema_version=u32(1)`. It is unclear
   whether the live `u16` schema is preserved as payload or replaced. This
   prevents exact replay and cardinality validation of prescribed amounts.

5. **Major — the selected-trial hydrology join is reachable, but its exact
   custody bytes remain internally inconsistent.** V3 correctly places the
   hydrology-complete ending A2 in `SelectedTerminalTrialRecordV3`. Because A9
   also requires it inside each provider-owned carrier record, the validator
   cannot enforce a single ownership boundary or prove that discarded
   coupling iterations were not retroactively joined to downstream hydrology.
   The separate immutable selection record closes substitution only if the
   carrier adapter itself remains upstream-only.

## Cardinality and numerical disposition

The V3 authority text, considered apart from its manifest contradiction,
closes pair cardinality: exactly one COARSE/FINE_1/FINE_2 triple owns each
evaluated decision, selected iterations are bound by explicit keys, and the
terminal floor admission contains no fabricated trial and proves unchanged
provider-call count. Its binary64 error evidence is sufficiently precise to
reconstruct the live five-component maximum, including ties and signed zero.

Those improvements do not overcome the closed-wire failures above. A
successor must remove the hydrology-complete joint from provider-owned A9 and
replace every declaration-order or semantic placeholder with exact adapters
that match the current Rust declarations, including all nested variants and
collections. Until those exact frozen schemas are reviewed, implementation
would necessarily invent evidence fields or encoding choices. No source
implementation or receipt capture is authorized.

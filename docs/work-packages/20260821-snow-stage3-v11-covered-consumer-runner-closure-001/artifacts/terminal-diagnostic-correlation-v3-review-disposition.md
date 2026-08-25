# Terminal diagnostic correlation V3 review disposition

Disposition: **HOLD / NO IMPLEMENTATION INTENT / NO SOURCE AUTHORITY**

Reviewed frozen inputs:

- V3 authority SHA-256:
  `5f9c0d66c13e3b11b921822c114877d1a61f233ae9e20c1164c21c9440c544e4`;
- V3 adapter manifest SHA-256:
  `5209dd6b80c54563755d92ced9ce367df4708053ba9e4a28c4b23882064075db`;
- preserved V2 authority SHA-256:
  `f4a7ff15127fdfd5068f16126f440a57a25026b44a5c610f175dfab30417cc5c`.

Both independent reviewers verified both V3 hashes, made no source or frozen
candidate edits, did not communicate with one another, and returned `HOLD`.
The two-GO gate failed. No exact-file implementation intent, correlation seam,
receipt capture or estimator matrix is authorized.

## Accepted progress

Both reviews agree that the high-level private architecture is feasible. V3
materially closes the V2 numerical-control findings by separating provider,
coupling iteration, coupling selection and terminal-solver ownership; keeping
arena entries immutable; separating pair position from exact live provider
role; separating `REJECT_RETRY` from the later pre-provider
`BELOW_CARRIER_DOMAIN`; and freezing delta direction, component order,
binary64 maximum fold and first-bitwise-equal diagnostic winner. The private
sealed mode, crate-unit-test boundary and post-return failure boundary remain
viable in principle.

## Accepted blocking findings

1. A9 requires a hydrology-complete ending joint while the provider-owned
   carrier record that embeds A9 explicitly excludes that later terminal-
   solver-owned value. This recreates `TDCV2-NUM-006` and must be removed in a
   successor.
2. A7 does not match the live `TerminalSnowSoilTrialReceiptV1` or
   `TerminalSnowSoilHeatReceiptV1` fields. It invents unavailable conductivity,
   path, `q_ss`, residual and state fields while omitting live OFE, source,
   temperature, owner and limiting-boundary fields.
3. A8 does not match `SoilThermalTopBoundaryCreditV1`, omits its actual IDs,
   signed `i64` support, configuration/state identities and two energy values,
   and does not enumerate `SoilThermalSnapshot`.
4. A4/A5 and other adapters use `declaration order` placeholders, omit nested
   variant and collection schemas, and name fields absent from live structs.
   The precipitation payload also fails to bind the live `u16` schema and
   exact destination/parcel/enthalpy-provider variants.
5. The six forwarding files cannot alone access every private owner payload
   demanded by the manifest. A successor must distinguish fixed-size
   forwarding from exact owner-module capture/access and authorize the exact
   complete file boundary before implementation review.
6. Keys, iteration/selection/trial/pair/admission/prefix records and typed
   physical errors lack complete exact framed field tags and nested encodings.
   The adapter manifest therefore does not yet define one unique byte stream.
7. The manifest assumes universal finiteness without establishing it and has
   no signed-integer/nonfinite diagnostic encoding policy.

These are authority defects. They cannot be repaired through implementation
judgment or a later implementation-intent diff. A reviewed successor must be
generated from the actual Rust declarations and enumerate every field, enum
variant, option, string-backed ID, signed/unsigned width, sequence/map order
and nested receipt payload. Provider-owned A9 must stop before hydrology join,
which remains exclusively in `SelectedTerminalTrialRecordV3`.

## Retained hold

The existing result remains exactly
`Stage3(TerminalNumerics(BelowCarrierDomain))`. Production Rust remains
unchanged; `43cc9bbea2fbf5fe6ab6596cee4162de75cef999` remains the last qualified
physical implementation. SnowEnergy v21/LSE v11/SnowFreeze v139/CoupledTime
v6 remain corrected but unverified. Diagnostic capture, final v21 review,
temporal operator, Batch V2, event, receiver, restart, runner, Child 3 and
cutover remain prohibited.

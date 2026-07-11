# Contract and provenance

Status: complete
Evidence mode: Static

Amended contract `0.1.5` and spec `0.1.2`: raw record-4 cardinality closes
before topology normalization; raw fields remain immutable/observable;
compatibility derives `nchnum_norm` only afterward and exposes the first
normalized-count IDs; structural mismatch is exact non-collapsible `CHN-E-002`.
The contract adds `INV-CHN-013` and normative A-H obligations.

Pinned authority: `wshinp.for:467-475` reads the raw implied-DO list at line
470 before clamping `nchnum` at lines 473-475; `chnrt.for:773-774` loops over post-clamp `nchnum`, thereby
consuming the first normalized-count entries. Anchors use baseline commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Final hashes: contract
`da94093ff009be0e8ee618783c799d1ed70c0377b398e546fecd7e5c6c605be6`;
spec `1d21069f186876faf19dfc4b2f300fdd17bad825cc54381d8c0c609b60c37ae2`.

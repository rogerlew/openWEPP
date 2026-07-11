# Finding Disposition

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-CONTRACT-MISMATCH`.

Accepted finding `CHANINP-RAW-NCHNUM-CARDINALITY`: contract source model and
field mapping distinguish raw `nchnum_input` from `nchnum_norm`, while
`normalize_nchnum` returns the normalized value for both before record-cardinality
closure. The compatibility fixture supplies raw `99` with two IDs and the test
expects exposed input `2`, blessing the conflation. Pinned legacy reads raw
cardinality before clamping.

Accepted: formatter-only CRAP is excluded under ADR-0021. Follow-up: a
contract-first defect package must ratify raw-vs-normalized retention/cardinality,
add explicit A-H obligations, correct/tests as authorized, and close module
coverage before CQR reruns. No findings remain undispositioned.

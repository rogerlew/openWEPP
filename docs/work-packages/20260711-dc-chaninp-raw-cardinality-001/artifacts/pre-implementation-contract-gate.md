# Pre-implementation contract gate

Status: PASS
Evidence mode: Static and Ran

Both independent reviewers passed the amended authority/test gate after all
wording findings were fixed. The gate confirms: canonical `INV-CHN-013`;
direct legacy read/clamp evidence separated from inferred fail-closed policy;
non-collapsible behavior scoped only to conditional record-4 closure for
non-negative raw counts; ordinary line1..3 compatibility defaulting retained;
negative-count policy explicit; tail-unknown W005 bound; and exact strict/
compat red tests plus consumer proof correctly formed.

Ran: parser 19 pass / 3 intended red; consumer 18 pass / 1 intended red.
Production parser and network-frame sources had empty diffs. Authorized plan:
raw parse and conditional closure, raw retention, count normalization,
`take(nchnum_norm)` projection, topology warning over all raw IDs, and pass
through only the ratified record-4 `CHN-E-002` class.

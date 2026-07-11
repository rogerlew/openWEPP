# Worker handoff

Status: complete
Evidence mode: Static and Ran

No corrective handoff remains for `CHANINP-RAW-NCHNUM-CARDINALITY`. The parser
now closes nonnegative raw record-4 cardinality before normalization and the
network frame consumes the normalized count. `ichnum_norm` remains a
parser-level projection; any future downstream ID-list consumption requires
its own real-consumer package and evidence.

After terminal commit, the serialized queue advances to FQ-03
`CHN-E006-EXTRA-RATING-ROW`.

# CHAN.INP parser fixtures

These small text fixtures exercise `SC-INFILE-CHANINP-001` strict and
compatibility parser branches. They are hand-authored from the canonical
four-record grammar and pinned legacy read order in
`/workdir/wepp-forest_260430_baseline/src/wshinp.for` at commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; they are not acquired external
data or generated run outputs.

`compat_nchnum_clamped.chaninp` is deliberately malformed: raw `nchnum=99`
has only two IDs and must fail `CHN-E-002` before normalization.
`compat_nchnum_raw_closed.chaninp` is the paired valid boundary: it supplies 99
IDs, preserves the raw count/list, and then normalizes topology consumption to
the first two IDs for `nchan=2`.

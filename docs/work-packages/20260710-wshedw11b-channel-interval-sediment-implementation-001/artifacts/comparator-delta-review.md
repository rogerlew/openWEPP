# Comparator Delta Review

Status: `EXECUTED-PASS-WITH-AUTHORIZED-DELTAS`

Evidence mode: `Static + Ran` against pinned baseline commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

## High-confidence direct lineage

- `wshchr.for`: KW/MC segmented recurrence, `mofapp=1` lateral averaging,
  branch-specific `qref`, dynamic-MC refresh, `qmaxi/qlavg` wave-end gate, and
  outlet-only epsilon normalization.
- `chnrt.for`: `qe`, effective-length partition, `qu`, and per-length lateral
  hydraulics.
- `dcap.for`: `timpot` re-incision and capped erosion-to-geometry transition.
- `case12.for`, `case34.for`, `detach.for`, `enddet.for`: branch topology and
  solved bracket/span consumption.
- `convrt.for`/`prtcmp.for`: channel-indexed bed-composition fractions.

Exact source hashes and the pinned commit are in
`logs/legacy-source-provenance.log`.

## Intentional contract-authorized deltas

- The active interval lane uses `t_exp = t_norm = dtchr`; it does not reuse the
  legacy event `tb = 2*rundur` scalar clock.
- Exact overlap projection of HBP `V_h/S_h`, the day-level class blend, and
  per-interval sediment sequencing have no like-for-like legacy runtime
  comparator. They are governed by `SC-ROUTE-001` v53.
- Equal sediment totals in the release spike/spread fixture publish equal
  240 kg terminal mass by exact class continuity, while water peaks remain
  timing-sensitive. Equality is not treated as evidence of scalar fallback;
  interval tests separately prove different sediment egress timing.

Disposition: no unexplained high-confidence delta remains. Comparator agreement
is an investigation flag, not the implementation target.

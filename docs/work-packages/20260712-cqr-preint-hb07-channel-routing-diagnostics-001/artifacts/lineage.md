# WS11 Muskingum-Cunge Lineage

Pinned `wshchr.for`/`MVPMC3` authority supplies geometry, dynamic reference
flow, celerity, `K/X`, coefficients, lateral term and recurrence. The reduced
runtime forms `qref` from current/prior `qin` and prior `q1`, solves Manning
depth, computes shape-specific `ckref`, then evaluates `tk`, `dencx`, `cx`,
`c0..c4` and `q1` in the original order.

The mechanical extraction moved only the `ckref` match into
`ws11_dynamic_muskingum_celerity`. Tests reconstruct fresh/carried operands
bit-for-bit. The DC removes the unauthorized clamp and uses the existing exact
`cx` domain error. WS10 publication reaches the W11C runner consumer, whose
seven scenarios consume routed water, storage, peak and sediment outputs.


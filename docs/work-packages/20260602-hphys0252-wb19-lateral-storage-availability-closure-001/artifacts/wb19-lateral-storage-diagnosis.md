# WB19 Lateral Storage Diagnosis

Status: complete

Evidence mode: static

Static:

- Baseline hourly lateral capacity loop computes:
  `drfc(i) = fc(i) + (1-coca(i))*dg(i)`, then
  `fzdrfc = max(drfc(i)-frzw(i),0)`.
- Baseline capacity-active layer condition is `st(i) >= fzdrfc` plus
  bottom-contiguous `meblfc`.
- Baseline `tdvv` is assembled as `Σ(st(i)-fzdrfc)` over capacity-active
  layers.
- Baseline hourly conductivity loop remains `st(i) >= drfc(i)` and
  `fffx = (st(i)-drfc(i))/(ul(i)-drfc(i))`.
- Baseline top-down lateral withdrawal floors layers at `fzdrfc`.

Diagnosis:

- Pre-HPHYS0252 Rust used raw `drfc` for capacity, available pool, and
  top-down withdrawal. That under-represented lateral capacity when
  `frzw(i) > 0` and over-constrained the withdrawal floor relative to the
  pinned hourly baseline.
- The production correction ports the baseline split without changing
  conductivity or adding publication compensation.

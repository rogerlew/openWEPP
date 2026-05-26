# EROD19 Route Ingress and Publication Matrix

Status: complete
Evidence mode: static
Date: 2026-05-26

| surface | source | EROD19 behavior |
|---|---|---|
| `nslpts`, `xu/xl`, `ainf/binf/cinf`, `ainftc/binftc/cinftc`, `qostar`, `xdetst`, `lddend` | runtime ingress (EROD18 seam + MOFE03 seeding) | validated as typed route inputs |
| `theta`, `phi`, `taucn` | EROD13 updates when present | used directly for baseline-derived branch math |
| `theta`, `erod14_beta` | fallback when EROD13 updates absent | derived fallback to keep MOFE03 runner continuity without silent defaults |
| `mshear`, `xc1`, `xc2` | EROD19 kernel migration | published from `xcrit`-equivalent classification path |
| `du`, `dl` | EROD19 kernel migration | published from upper-boundary branch (`abs(qostar) < .0011`) |
| `xdbeg`, `xdend` | EROD19 kernel migration | published from deposition/detachment branch with `depc/depend`-style solve |
| `ndep`, `ldlast`, `lddend` | EROD19 kernel migration | published branch-followup state for post-detachment deposition tracking |

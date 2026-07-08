# Legacy Cropland Source Audit

Status: queued placeholder.

Execution must classify each candidate source for projected Lane D coefficients.

| Candidate | Potential target | Classification | Evidence |
|---|---|---|---|
| random roughness / `rrough` / `rrc` | `D_r_m`, `lambda`, form/wave roughness | pending | baseline source-line audit required |
| rill/interrill friction terms | `k_o` or diagnostic comparator | pending | baseline source-line audit required |
| cover/residue/rock/basal cover | `lambda` / roughness element coverage | pending | baseline source-line audit required |
| live plant `LAI` / `canhgt` | dynamic vegetation operands already authorized | context | map to current rev-21/rev-36 authority |
| crop/management class | class default or reject | pending | contract decision required |

Rejected aliases must be named explicitly so a later implementation cannot
quietly map unrelated erosion or publication fields into Lane D coefficients.

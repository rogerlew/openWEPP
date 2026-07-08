# Conservation Reconstruction

Status: `passed`

Evidence mode: `Ran:` reconstruction plus `Static:` lineage review.

W7R reconstructs sediment-sensitive public operands from generated HBP and
public routed-output artifacts:

- HBP latest event `tdet/tdep` matches `totalwatsed3.tdet/tdep`.
- HBP hourly sediment mass closes to `tdet - tdep`.
- `totalwatsed3.sed_del` matches `ebe_pw0.sediment_yield`.
- `sed_del` is not `tdet`, `tdep`, or `tdet - tdep`.

Detailed reconstruction is in `sediment-reconstruction.md`.

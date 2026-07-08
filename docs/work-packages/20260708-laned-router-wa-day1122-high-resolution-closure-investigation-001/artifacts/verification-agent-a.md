# Verification Agent A

Status: PASS
Evidence mode: Static/Ran.

Verifier: Beauvoir

## Scope

Independently parsed package-local `mesh-ladder-summary.json`, run logs,
completed-rung traces, and parquet outputs as needed. No files edited.

## Results

PASS. No numeric discrepancies found.

Verified:
- Release provenance matches: Git HEAD
  `03429ce41d439ff3ab2425bf93a8e00d5c39fa42`, binary SHA256
  `6dcd6275d5d8891a23258fb84a5d143c57b1b0f251f709c8f27711ebc2551308`,
  size `9903568`.
- WA rung status/timing matches raw logs and the summary table:
  `baseline_fixed10`, `dx20`, `dx10`, `dx5` PASS; `dx2p5`, `dx1p25` FAIL.
- `dx2p5` and `dx1p25` day-1122 residual operands, residual recomputation,
  relative error, and litre conversions match raw `time.log` failure lines.
- Completed-rung day-1122 books match recomputation from trace rows.
- Day-1418 lane-5 attribution matches for dominant clamp/outlet and amplified
  `dx10/dx5` storage.
- Hydrology-source deltas are zero across completed rungs. An independent full
  `H1.wat.parquet` comparison found max delta `0.0` for `P`, `RM`, `Q`,
  `QOFE`, `UpStrmQ`, `latqcc`, `Area`, and `SoilWaterTotal`.
- Cell counts for `108.34 m` OFEs match: fixed/dx20 `10`, `dx10` `11`,
  `dx5` `22`, `dx2p5` `44`, `dx1p25` `87`.

Clarifying note:
- Top `mesh_end_storage_m3` for `baseline_fixed10`/`dx20` is day 784 lane 5,
  not day 1418 lane 5. The package JSON records this correctly; the package
  claim is about dominant clamp/outlet magnitude and amplified `dx10/dx5`
  storage on day 1418.

## Verdict

PASS for the package-local numeric evidence and active-router clamp-numerics
hold interpretation.

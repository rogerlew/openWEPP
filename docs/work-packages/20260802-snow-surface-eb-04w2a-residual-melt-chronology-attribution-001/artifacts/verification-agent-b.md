# Verification Agent B

Status: `PASS FOR EXECUTED / HOLD / PARTIAL ADMISSION`.

Evidence mode: **Ran + Static**.

## Terminal Verdict

W2A is internally consistent and may close only in its declared `HOLD`
disposition. The direct-production operands are admissible as descriptive
evidence for the exact retained calibration cells. The snowbench chronology,
seasonal SWE, and albedo contrast remain withdrawn. No promotion, fit,
transferability, independent-validation, or production-physics claim passes.

## Independent Checks

- Recomputed the package tool SHA-256
  (`fa5399db...0ad2`) and freeze SHA-256 (`ff9f4595...ffd73`) and matched both
  to the freeze, receipt, and all eight cell-provenance records.
- Verified exactly eight unique four-lane-by-two-model receipts, return code
  zero for each, prospective starts after the corrected freeze, and every
  recorded fixture and output file against its SHA-256.
- Recomputed the raw JSON and CSV hashes
  (`96e0b40a...800365c`, `f82c72f6...a71b9`) and matched them to the
  adjudicated publication view. The authoritative JSON marks the harness
  `WITHDRAWN` and publishes null chronology, seasonal-trajectory, and albedo-
  response values; the authoritative CSV likewise leaves those fields empty.
- Independently reconstructed direct cold-content hits as Mica `8/23`, Niwot
  `16/40`, Paradise `0/19`, and Snowbird `12/22`; corrected early-gap late-
  input hits as Niwot `0/27` and Snowbird `5/16`; and the empirical
  `|b + c| > |a + d|` result at Mica, Paradise, and Snowbird but not Niwot.
- Confirmed maximum retained direct closure of `2.221e-15 m` and Stage-3
  energy closure of `6.094e-08 J m^-2`. These operands remain separate from
  the failed snowbench ledger.
- Confirmed the static activation mechanism at the frozen source revision:
  the public entry predicate does not inspect typed hourly snowfall, while the
  inactive path records no accumulation. The package correctly bounds this
  diagnosis to snowbench typed forcing because direct-production phase/input
  authority on the four decisive dates is unresolved.
- Ran the package self-check, package and roadmap/catalog Markdown lint,
  four-figure XML parse, same-stem sidecar inventory, and `git diff --check`;
  all passed. Rasterized and visually inspected all four figures. Withdrawal
  banners and gray invalid series are conspicuous, the raw-flag panel is
  masked, and the sidecars state populations, units, aggregation, and limits.
- Rechecked terminal scope: only the new package tree and the three authorized
  roadmap/catalog files differ. The transient package-local Python bytecode
  found during verification was removed; no package-local `__pycache__`
  remains.

## Hold And Successor Assessment

The `0.0377-0.0708 m` SWE non-closure changes subsequent snow state, so common-
mode loss cannot rescue the albedo comparison. Correcting the public
activation path requires Rust, contract, test, and real-consumer work that W2A
prospectively protected. EB-04W2B is therefore a warranted, bounded hold-lift
package: reconcile phase/input authority, conserve positive typed snowfall,
add a mixed-event regression and fail-closed daily closure, prove both real
consumers, and only then rerun the unchanged contrast.

Recommendation: `PASS` the evidence lifecycle and retain package status
`HOLD`; advance EB-04W2B next.

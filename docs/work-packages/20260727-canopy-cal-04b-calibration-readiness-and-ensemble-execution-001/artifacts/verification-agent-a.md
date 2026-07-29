# Terminal Verification A

Status: `PASS AFTER CORRECTION`

Evidence class: `Ran + Static`

The verifier independently checked the exact diff, package-local Rust/Python
corrections, published hashes, external evidence roots, freeze chronology,
retention identity, Git LFS object, and full-workspace evidence.

One medium finding was accepted and closed. The frozen terminal validator's
four-local-ULP-width comparison can admit more than four representable steps at
a binade boundary. The freeze-bound source was preserved byte-for-byte:
`validate.py` SHA-256
`bfe5cc855f00d3c9b8e948429eb39415e43557dedf0db9fc22b3d7b81432c0c1`
matches `freeze-custody-controls.csv`. A separate post-freeze exact-rank audit
and cross-binade regressions prove all 1,598 finite objectives have step
histogram `{0: 986, 1: 576, 2: 35, 3: 1}`.

Final verification confirms:

- terminal validation passes for 9,261 calibration candidates, 37 accepted
  members, 37 holdout rows, and 177 freeze members;
- rejected pre-open freezes have no passing barrier, token, or holdout output;
- the accepted digest has two distinct receipts before the durable token;
- retention reconstructs raw SHA-256 `30d24d96...714620` and records the exact
  absolute command;
- package gates, 38 Python tests, 34 Rust tests, and the 2,101-test full profile
  pass; and
- no production, contract, protected fixture, or unrelated user-owned file is
  in the package diff.

No correctness or custody blocker remains. Harvard was not rerun.

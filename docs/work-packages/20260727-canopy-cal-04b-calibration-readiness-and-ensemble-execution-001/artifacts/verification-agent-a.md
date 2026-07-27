# Terminal Verification A

Status: `PASS`

Evidence class: `Ran`

Reconstructed checks:

- exactly four observed receipts exist; commands 1–3 are `PASS/0` and
  native-proof is `FAIL/1`;
- receipt/log/manifest hashes and all five incident-004 published hashes match;
- native-default has 16,437 rows and 131,496 bit-exact compared values;
  interior has 11,185 rows before the day-11,186 typed guard;
- no later population, synthetic, reconstruction, readiness, freeze, holdout,
  token, or digest artifact exists;
- all 25 CAL-04B CSV files parse with consistent widths;
- terminal-HOLD scaffold and executor validators, 15 Python tests, Markdown
  lint for 39 CAL-04B + 25 successor + 2 catalog files, and diff hygiene pass;
- the active CAL-04B kickoff is absent and its archived SHA-256 is
  `102f5a6dc7dc3bf2ed0c71a563fdc0914a2b7d23eebc0a3f8a9a90429faff9c4`.

`TV-A-001` is closed by the exact current Markdown evidence. Harvard is sealed
and the successor scaffold is complete. No blocker remains.

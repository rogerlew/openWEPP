# Coverage after

Status: PASS
Evidence mode: Ran

Terminal focused measurement after decomposition:

| Metric | Covered / total | Percent | Required | Result |
| --- | ---: | ---: | ---: | --- |
| Lines | 1,020 / 1,048 | 97.328% | >=90% | PASS |
| Regions | 1,597 / 1,717 | 93.011% | >=90% | PASS |
| Functions | 67 / 73 | 91.781% | reviewed named floor | PASS |

All logical functions except `for_batch` meet the 75% function-region floor.
`for_batch` is a reviewed, closed-list infrastructure exclusion at 66.667%:
its normal multi-batch path, callback-success path, callback-error propagation,
file-open mapping, malformed-Parquet reader construction, and row-offset logic
are exercised. The remaining dependency-origin reader-build/page-read errors
cannot be selected independently through the public writer without producing
implementation-specific corrupt Parquet. Its low CRAP of 8.815 and complete
typed mapping make exclusion safer than introducing a test-only seam into the
production reader.

Raw evidence:

- `lcov-after.info`: SHA-256
  `7d2ce90592050ac2ee8edddf8f1129202767126c6a436d9817d8555ae4c0a569`.
- `coverage-after.json`: SHA-256
  `f2fc7e7434e43dc8545daebe0cb45120138a77980d2a6ca367d9181c1c693be1`.

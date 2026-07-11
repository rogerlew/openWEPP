# Final Disposition

Status: `EXECUTED-HOLD-CQR-NIGHTLY-LOCAL-CONTRACT-MISMATCH`.

The behavior-preserving attempt passed focused tests, coverage, CRAP, clippy,
and semantic review, but cannot satisfy canonical `G-CHN-013`: extra
rating-curve records relative to `icntrl` must emit `CHN-E-006`, whereas current
scaffold behavior emits `CHN-E-002`. Correcting the typed error is outside this
CQR package. Target and test edits are rolled back exactly to scaffold
`a7d07708`. This local hold does not invalidate the batch baseline.

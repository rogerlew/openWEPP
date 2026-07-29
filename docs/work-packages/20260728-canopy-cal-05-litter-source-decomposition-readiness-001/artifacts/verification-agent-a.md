# Terminal Verification A — Operator-Adjudicated Exact Tree

Evidence class: `Ran: exact-tree gates and independent claim-boundary checks;
Static: operator authority, retrospective provenance, Rust/Python, lifecycle,
write-set, contract, and LFS review`

Verifier disposition: `PASS`

Package disposition:
`COMPLETE / READINESS PASS / SOURCE AUTHORITY SUCCESSOR REQUIRED`

## Findings

No blocking correctness finding remains.

### VA-001 — `CORRECTED`

The executor uses one 7,300-day `DirectRunIdentity` per member, seeds the actual
`0..7299` frame index, retains `frame_day_index`, and validates it against
ordered year/day chronology. The regenerated primary and ridge traces contain
116,800 and 36,500 rows respectively, with exact carried surface/interrill/rill
state and day-280 source pulses.

### VA-002 — `LIFTED BY EXPLICIT OPERATOR GOVERNANCE`

Incident 002 remains explicitly labeled retrospective. The operator's
2026-07-28 statement, “I authorize the retrospective analysis,” is retained in
`operator-governance-adjudication.md` and `execution-incident-002.md`. The
authorization is narrow and sufficient to resolve the package's procedural
stop-loss:

- the analysis uses only the already frozen, exhaustively executed grid and
  analytic ridge;
- no axis, observation, objective, tolerance, result, stopping rule, or
  parameter selection changed;
- the method is not relabeled prospective; and
- no fitted or preferred source/rate value is authorized.

Under that adjudication, the named direct-runtime surface source/rate operator
is truthfully
`IMPLEMENTED / CALIBRATION_READY_DATA_LIMITED / PARTIALLY_IDENTIFIABLE`.
Independent recomputation still exactly reproduces all eight sensitivities,
covariance `0.16261898422070004 kg m^-2 yr^-2`, and correlation
`0.99937720235305838`.

The readiness result does not propagate beyond that operator:

- native leaf transfer remains
  `IMPLEMENTED / NOT_CALIBRATION_READY / NOT_ASSESSED`;
- recurring needle and fine-woody sources remain
  `AUTHORITY_MISSING / NOT_CALIBRATION_READY / NOT_ASSESSED`;
- source composition remains authority-missing and nonidentifiable; and
- empirical decomposition fitting remains
  `NOT_CALIBRATION_READY / NOT_ASSESSED / AUTHORITY_BLOCKED`.

Harvard carbon remains separate from modeled dry mass, organic-horizon stock
remains separate from the modeled surface pool, no unavailable source is
zero, and no decay value is fitted.

### VA-003 — `CORRECTED`

The normal terminal validator passes. Optimized Python exits 1 with an explicit
`RuntimeError` before making an evidence claim, so assertion removal cannot
produce a false pass.

### VA-004 — `CORRECTED`

The summarizer reads the frozen ridge execution and terminal-target designs.
The validator independently reconstructs all eight sensitivities, covariance,
correlation, and units from design-bound retained traces. No duplicated ridge
constant remains authoritative.

## Verified evidence

- Primary reconstruction passes 16/16 and ridge reconstruction passes 5/5;
  maximum retained difference is `8.881784197001252e-16`.
- `S020-K050` is the sole recovered truth. All five terminal-ridge members
  close within `1.1102230246251565e-15 kg m^-2`.
- All 16 boundary/failure cases match the expected state or typed variant and
  field.
- All 28 Harvard rows preserve unique admitted keys, periods, counts,
  `stock_use_not=1`, and exact source values.
- The three predecessor authority hashes remain exact:
  `68b3635c...fb9`, `e21a917c...90b3`, and `83e749a3...d986`.
- Git LFS attributes and object identities pass. Producer is
  `e0778f36e0286a30fe17523cd19e0f204928c8c17211409de17b7af4cb970e63`
  (61,711,995 bytes); ridge is
  `4ee2c2e0c757a7aa61c37ff3c9627f74a98f78e489d615e05a4acbe720421d1d`
  (19,750,700 bytes).
- The package has exactly 51 retained non-build files, including the operator
  adjudication. CAL-05-attributable writes remain within the declared
  package/docs set; no production crate, canonical contract, fixture,
  canonical test, management, default, or scientific parameter changed.

## Direct gates

- Focused runner nextest
  `136f03f0-4dc1-4f96-ae98-5774c628ed2e`: 1 passed, 220 skipped.
- Package Rust test harness: pass, zero tests.
- Package Rust format and warnings-denied Clippy: pass.
- Repository-configured dependency policy: advisories, bans, licenses, and
  sources all `ok`, with configured unmatched warnings.
- Normal terminal validator: pass; optimized validator: expected fail-closed
  exit 1.
- CAL-05 Markdown: 25 files, zero errors or warnings. Authority-package,
  roadmap, and catalog Markdown also pass.
- `git lfs fsck --objects` and `git diff --check`: pass.

## Residual risk and missing tests

- The package-local Rust harness has zero unit tests. Retained execution and
  independent reconstruction are sufficient for this frozen package, but
  reusable tooling should add regression tests for multi-day identity, pulse
  chronology, carried ground pools, and typed failures.
- `analyze.py` reports one mixed-dimension reconstruction maximum under a
  mass-unit label. Current values reconstruct, but reusable validation should
  separate dimensional labels and tolerances.
- Recurring needle and fine-woody source authority remains absent. The bounded
  contract-first `CANOPY-LITTER-SOURCE-AUTHORITY-01` successor is still
  required before Orders 6-8.

## Verdict

`PASS`. Explicit operator governance lifts VA-002 while preserving its
retrospective label and all source/empirical-calibration limits. VA-001,
VA-003, and VA-004 remain corrected. The exact tree supports
`COMPLETE / READINESS PASS / SOURCE AUTHORITY SUCCESSOR REQUIRED`.

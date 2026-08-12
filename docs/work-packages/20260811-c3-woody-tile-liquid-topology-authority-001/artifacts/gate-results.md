# Gate Results

Status: `PASS / focused, heavy, and dual terminal verification complete`

Evidence mode: `Ran`

## 2026-08-12 Focused Iteration

1. `cargo check -p openwepp-vegetation`: PASS after moving the E04 authority
   guard before all E04 evaluation.
2. `cargo fmt --all -- --check`: initially reported two formatting diffs in the
   new authority test; `cargo fmt --all` applied the mechanical correction.
3. `cargo nextest run --test vegetation_boundary_authority_contract --profile quick`:
   FAIL `11/14` on stale V1 lifecycle assertions, then FAIL `13/14` on a
   line-wrap-sensitive new assertion; failures are preserved here.
4. Same authority command after reconciliation: PASS `14/14`.
5. `.venv/bin/python .../reference_calculator.py`: PASS with
   `"all_pass": true`; committed output diff is empty.
6. Initial science-admission run: FAIL because the draft/in-review
   `SC-LANDSURFACEENERGY-001` had been amended. The amendment was narrowed back
   to the approved vegetation contract; LSE and WATBAL were restored byte-for-byte
   to HEAD because their existing ownership invariants are sufficient.

Heavy workspace gates remain `NOT RUN` pending two focused science reviews.

## 2026-08-12 Focused Authority Gate

- science admission: PASS,
  `A0_ADMITTED contracts=44 science_surfaces=0`, authority receipt
  `13e890ed51b3eff382ab9b3bdffc5509a6af240d320237a7257491b7e2bc9b07`;
- authority anti-evasion: PASS;
- AUTH11: PASS `3/3`;
- vegetation authority suite: PASS `14/14`;
- `SC-VEGETATION-001` unit compliance: PASS;
- package Markdown: PASS `27 files`, zero findings;
- science-contract Markdown: PASS `62 files`, zero findings;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS.

## 2026-08-12 Accepted-Finding Remediation

- both historical HOLD reviews were preserved unchanged; all ten findings were
  accepted and mapped in `review-finding-disposition.md`;
- independent oracle regeneration: PASS, byte-identical committed fixture,
  all 31 executable checks true;
- canonical V2 JSON sort check: PASS, sorted serialization SHA-256 equals file
  SHA-256 `e62d448b045db1577fe9367b5b531fcd7b1cfc9b544800c11c4ed305d14da10a`;
- shared transaction contract SHA-256:
  `bbe498113e3130825b03e0e0a0a6134fa708c37326a3663f994dc44e3422f725`;
- vegetation authority suite after independent fixture reconstruction was
  strengthened: PASS `14/14`;
- focused authority-test Clippy: PASS with `-D warnings`;
- authority anti-evasion: PASS;
- AUTH11: PASS `3/3`;
- both `SC-VEGETATION-001` and `SC-VEGETATIONTRANSACTION-001` unit-compliance
  lints: PASS;
- science admission: expected FAIL while the corrected contracts remain
  `in_review/draft`; this is the required fix for `A-HIGH-005`, not a released
  authority failure. Exact output named `SC-VEGETATION-001` as non-admitted;
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

Heavy workspace gates remain `NOT RUN` until both independent rereviews pass.

## 2026-08-12 Repeat-Review Correction

- first repeat topology review: HOLD on executable oracle coverage, potential
  unit/state serialization conflict, and focused Clippy;
- first repeat resource review: HOLD on the overlapping executable oracle
  coverage finding;
- focused Clippy after the 31-name test strengthening: FAIL
  `clippy::too_many_lines`; preserved as invalidated evidence;
- oracle was rebuilt with explicit case operands/results and regenerates
  byte-identically; fixture SHA-256 is
  `e487413142c463a81a4e29d4887cdf4fa339eadeaeeda0a4cf92ffbf2ceb76a7`;
- V2 section/digest reconciliation after the `mm H2O` and recursive lexical
  serialization correction: both canonical copies SHA-256
  `b2b01f965f83a52f4c800c489079c88d97179ed6a8191734b541115308b97a5c`;
- vegetation authority suite: PASS `14/14`;
- focused authority-test Clippy after helper decomposition: PASS with
  `-D warnings`;
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

Heavy workspace gates remain `NOT RUN` pending the second repeat of both
independent science reviews.

## 2026-08-12 Substitute-Physics Removal

- second resource/transaction rereview: PASS with no unresolved material
  finding;
- second topology rereview: HOLD on uncited substitute cap-to-vapor and
  simplified wet/FvCB oracle responses;
- deleted the uncited response and narrowed controlled vapor operands to
  topology causality only; the fixture now names
  `STAGE_B_E11_E15_EXACT_ORACLE` as the required complete coupled acceptance
  gate;
- wet-energy and PAR locality now call the existing digest-bound V1 independent
  oracle's wet-surface energy and full FvCB/co-limitation implementations;
- regenerated fixture SHA-256:
  `c02e5e2a2287d84cfc584a6e3ec9c499cf7168160bc71f2577323f19dcb50bf1`;
- vegetation authority suite: PASS `14/14`;
- focused authority-test Clippy: PASS with `-D warnings`;
- oracle regeneration diff, `cargo fmt --all -- --check`, and
  `git diff --check`: PASS.

Heavy workspace gates remain `NOT RUN` pending the third topology rereview.

## 2026-08-12 Promotion Gate

- final topology/energy science rereview: PASS / GO, no unresolved material
  finding;
- final resource/transaction science rereview: PASS, no unresolved material
  finding;
- promoted `SC-VEGETATION-001@6` and
  `SC-VEGETATIONTRANSACTION-001@1` to `approved/active`;
- final transaction-contract SHA-256:
  `c94d3c5745fd801b092f992b46fb6f5d4684b70acf24f198c4d4d6fdc42785c8`;
- final promoted V2 definition SHA-256, identical in both copies:
  `38e1bb90abd3ff82879f7d9c80b0377bb510a3b97fdd2b6f07c12b7c42b80dc3`;
- science admission: PASS,
  `A0_ADMITTED contracts=45 science_surfaces=0`, authority receipt
  `464b2675f17f75a6a9e92c6de0a70dae76ef03ca092c23f29d2ad965d62be628`;
- vegetation authority suite: PASS `14/14`;
- authority anti-evasion: PASS;
- AUTH11: PASS `3/3`;
- both affected contract unit checks: PASS;
- `cargo fmt --all -- --check` and `git diff --check`: PASS.

Heavy workspace gates delegated to the required comparator runner.

## 2026-08-12 Heavy-Gate Closure

- workspace Clippy: PASS after test-only lint remediation;
- full workspace nextest: PASS `2422/2422`, 33 skipped by the canonical full
  profile, elapsed `3270.563 s`;
- doc tests: PASS;
- `cargo deny check`: PASS;
- formatting and diff hygiene: PASS.

Historical attempts remain in `comparator-gate-results.md`: two runs were
externally interrupted, one failed because root-backed `/tmp` exhausted space,
and one detached launcher failed before command execution. The terminal run used
absolute external scratch on `/home` and completed without interruption.

## 2026-08-12 Terminal Verification

- verifier A: PASS with no unresolved material finding;
- verifier B first pass: HOLD on stale lifecycle summaries and absent exact
  terminal-diff reconciliation only; no science, oracle, digest, or gate defect;
- both accidentally retained Cargo target trees were moved intact outside the
  repository while concise failure logs were preserved;
- lifecycle summaries and `terminal-diff-reconciliation.md` were corrected for
  verifier B's exact-byte rereview.
- verifier B corrected-byte rereview: PASS with no unresolved material finding;
- both terminal verifiers pass; the kickoff prompt was archived byte-for-byte.

## 2026-08-12 Post-Archive Hygiene

- vegetation authority suite: PASS `14/14`;
- science admission: PASS `contracts=45`, receipt
  `464b2675f17f75a6a9e92c6de0a70dae76ef03ca092c23f29d2ad965d62be628`;
- authority anti-evasion: PASS;
- package and catalog Markdown: PASS, zero findings;
- `git diff --check`: PASS.

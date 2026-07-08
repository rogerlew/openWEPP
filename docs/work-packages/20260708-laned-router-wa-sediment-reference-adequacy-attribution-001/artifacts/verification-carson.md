# Verification - Carson

Evidence mode: Static + Ran.

Verifier: `019f4228-7167-7f53-9b48-baffa860911d`

## Verdict

Numeric/provenance result: PASS.

Initial package governance result: HOLD because `gate-results.md`,
`review-*.md`, and `verification-*.md` were missing at review time.

Final disposition: governance artifacts added; local gates rerun in
`gate-results.md`.

## Independent Checks Reported

The verifier ran/read-only checks:

- Independent parquet/JSONL hash plus annual/daily recomputation.
- `PYTHONPYCACHEPREFIX=/tmp/openwepp-wa-verify-pycache .venv/bin/python -m py_compile .../analyze_wa_sediment_reference.py`
- `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001 --format json`
- `markdown-doc validate ...`
- `git diff --check`
- `sha256sum target/release/openwepp-cli-hill`
- `stat target/release/openwepp-cli-hill`

## Verified Values

- Release binary SHA-256:
  `8876fa04ca520126b958d83a7c5777da6f793e51fba4c346432f065b31647aaa`
- Release binary size: `9944528` bytes.
- Candidate year-4 `tdep`: `0.6107069659777166 kg`.
- Reference year-4 `tdep`: `0.5974836468326581 kg`.
- Absolute delta: `0.013223319145058476 kg`.
- Relative delta: `0.022131683796129127`.
- One-third adequacy threshold: `0.006666666666666667`.
- Year 4 has exactly one nonzero daily `tdep` delta, sim day `1126`,
  julian `30`.
- Day-1126 `runvol`, `sbrunv`, and `peakro` deltas are all zero.
- Day-1126 trace:
  - source delta `0`,
  - aggregate outlet delta `0.01827025610623423 m3`,
  - end-storage delta `0.0033593971208620843 m3`,
  - tail fold `0`,
  - clamps `0`,
  - max lane shape L1 `0.0007414490157977821`,
  - terminal outlet delta `-0.003359397088388505 m3`,
  - terminal shape L1 `0.0006352335679617539`.

## Disposition

The numeric attribution and provenance are accepted. The missing governance
artifacts were added after verification and are recorded in `gate-results.md`.

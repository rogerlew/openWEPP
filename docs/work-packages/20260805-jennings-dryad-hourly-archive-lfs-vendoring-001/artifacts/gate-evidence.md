# Gate Evidence

Ran: 2026-08-05.

- `bash tools/release/check_authority_suite_antievasion.sh`: pass.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract`:
  pass, 3 tests.
- `sha256sum -c SHA256SUMS`: pass, 10 files.
- Exact LFS attribute, staged-pointer, local-object, size, checksum, line-count,
  and header checks: pass.
- `git diff --check`: pass before independent review.

Documentation gates and final diff reconciliation are recorded after review.

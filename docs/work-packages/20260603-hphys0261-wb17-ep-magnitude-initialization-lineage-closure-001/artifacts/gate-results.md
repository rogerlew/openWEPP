# Gate Results

Status: completed

Evidence mode: ran

## Gates

Ran: `cargo fmt --check`

Result: pass.

Ran: `cargo clippy --workspace --all-targets -- -D warnings`

Result: pass after formatting test literals with separators.

Ran: `cargo test --workspace`

Result: pass.

Ran: `cargo deny check`

Result: pass with pre-existing duplicate/unmatched-license warnings; final
summary reported `advisories ok, bans ok, licenses ok, sources ok`.

Ran: `bash tools/release/check_authority_suite_antievasion.sh`

Result: pass.

Ran: `cargo test --test auth11_required_suite_obligation_guards_contract`

Result: pass: `2 passed; 0 failed`.

Ran: `wctl doc-lint --path docs/work-packages/20260603-hphys0261-wb17-ep-magnitude-initialization-lineage-closure-001`

Result: command completed but reported `0 files validated, 0 errors,
0 warnings`; package paths appear outside the configured recursive lint
selection for this wrapper.

Ran: `wctl doc-lint --path docs/work-packages/README.md`

Result: pass: `1 files validated, 0 errors, 0 warnings`.

Ran: `git diff --check`

Result: pass.

Ran: `/workdir/wepppy/.venv/bin/python -m py_compile docs/work-packages/20260603-hphys0261-wb17-ep-magnitude-initialization-lineage-closure-001/artifacts/hphys0261_diagnostics.py`

Result: pass. Generated `__pycache__` files were removed before commit.

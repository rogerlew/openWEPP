# Gate Results

Evidence mode: Ran.

## Required Gates

| Gate | Result | Evidence |
|---|---|---|
| Analyzer compile | PASS | `PYTHONPYCACHEPREFIX=/tmp/openwepp-wa-sediment-pycache .venv/bin/python -m py_compile docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/analyze_wa_sediment_reference.py` exited `0`. |
| Analyzer replay | PASS | `PYTHONPYCACHEPREFIX=/tmp/openwepp-wa-sediment-pycache .venv/bin/python docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001/artifacts/analyze_wa_sediment_reference.py` regenerated `artifacts/wa-sediment-attribution.json` and `.md`; exited `0`. |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001 --format json` scanned `18` files, `0` errors, `0` warnings. |
| Markdown/doc validate | PASS | `markdown-doc validate --path docs/ROADMAP.md --path docs/work-packages/README.md --path docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001 --format json` scanned `18` files, `0` errors. |
| `git diff --check` | PASS | Exited `0`, no whitespace findings. |
| Package pycache exclusion | PASS | `find docs/work-packages/20260708-laned-router-wa-sediment-reference-adequacy-attribution-001 -name __pycache__ -o -name '*.pyc'` returned no files. |

## Conditional Gates

| Gate | Result | Rationale |
|---|---|---|
| Contract/profile/BEI checks | NOT RUN | No `SC-*` contract, profile, or BEI file changed. |
| Focused Lane D / `ofe_routing` tests | NOT RUN | No Rust code changed; this package replays existing run outputs. |
| `cargo fmt --check` | NOT RUN | No Rust code changed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | NOT RUN | No Rust code changed. |
| `cargo nextest run --workspace --profile full` | NOT RUN | No Rust code changed. |
| `cargo deny check` | NOT RUN | No dependency or Rust code changed. |

## Independent Verification

`artifacts/verification-carson.md` records independent recomputation of hashes,
annual `tdep:4`, day-1126 attribution, active trace values, and release-binary
provenance. Numeric/provenance result: PASS.

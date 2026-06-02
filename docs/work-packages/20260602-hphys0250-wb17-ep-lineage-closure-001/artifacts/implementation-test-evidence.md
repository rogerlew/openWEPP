# Implementation and Test Evidence

Status: complete

Evidence mode: static + ran

Static:

- Runner preserves PL schedule activation through scheduler execution instead
  of stripping the activation sentinel before dispatch.
- Runtime projection allows established perennial `jdplt=0` slots and seeds
  initial live canopy into PL state from initial `cancov` using baseline
  `init1.for` / `initgr.for` lineage.
- WB11 growth/decomposition transition phases publish computed state/seed
  surfaces instead of NOPing after scheduler dispatch.
- WB13 final `Ep` publication prefers flux-surface `Ep` from post-WB19
  `PlantRootUptake`.
- WB15 runoff reconciliation canonicalizes only within-tolerance negative
  `I`/liquid roundoff before writeback.

Ran:

- Targeted HPHYS0250 and projection tests passed; see
  `contract-test-implementation-evidence.md`.
- `cargo test -p openwepp-runner --lib -- --nocapture` passed `45/45`.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed with existing duplicate/license allowance
  warnings only.
- `bash tools/release/check_authority_suite_antievasion.sh` passed.
- `cargo test --test auth11_required_suite_obligation_guards_contract`
  passed `2/2`.
- `git diff --check` passed.

Full-suite evidence:

- Full 39 runtime root: `/tmp/hphys0250_20260602T175731Z`.
- Runtime status: `39/39` completed.
- Semantic reports: `39/39` completed.
- Semantic pass: `0/39`.

Follow-up evidence (2026-06-02):

Static:

- Added WEPPpy `Agriculture/corn-no till.man` growth-coefficient regression coverage via the existing canonical fixture lineage, preserving `beinp`, `gddmax`, `xmxlai`, and `rdmax` from parser output through PL runtime slot symbols and primary aliases.
- Corrected the management fixture provenance README from `/home/workdir/wepppy/...` to `/workdir/wepppy/...`, matching the local checkout.
- Added root `.venv` discovery instructions for agents in `AGENTS.md`; `.venv` remains ignored and untracked.

Ran:

- `.venv/bin/python -m ensurepip --upgrade` bootstrapped pip in the repo-local `.venv`.
- `.venv/bin/python -m pip install pandas` installed `pandas==3.0.3` with `numpy==2.4.6`.
- `cargo fmt --check` passed.
- `cargo test -p openwepp-hillslope-orchestrator management_runtime_projection_preserves_wepppy_corn_no_till_growth_coefficients -- --nocapture` passed `1/1`.
- `git diff --check` passed.

# HPHYS0203 Gate Results

Status: completed  
Evidence mode: Ran

## Workspace validation gates
All required gates executed from `/home/workdir/openWEPP`:

1. `cargo fmt --check` -> pass.
2. `cargo clippy --workspace --all-targets -- -D warnings` -> pass.
3. `cargo test --workspace` -> pass.
4. `cargo deny check` -> pass (duplicate crate and
   `license-not-encountered` warnings only; exit code `0`).

## Diagnostic parity context (MEASURE-HP203-004)
Diagnostic rerun evidence was summarized from:
`/tmp/hphys0207_20260530T042607Z/parity/`.
HPHYS0203 introduced contract/test robustness coverage only (no production
kernel math changes), so comparator evidence is recorded as diagnostic context.

1. Hillslope batch status:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_batch_status.tsv`
   -> `39/39` rows with `rc=0`.
2. Semantic comparator status:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic_status.tsv`
   -> `39/39` rows with `rc=0`.
3. Semantic summary exists:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json`.
4. Targeted diagnostic fail-hillslope counts (from semantic reports):
   - `Dp`: `39/39`
   - `latqcc`: `39/39`
   - `Total-Soil`: `39/39`
   - `SoilWaterTotal`: `39/39`
   - `ProfileDepth`: `0/39`
   - `ProfilePorosityCap`: `0/39`
   - `ProfileFCStore`: `27/39`
   - `ProfileWPStore`: `1/39`

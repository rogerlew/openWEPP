# tests/fixtures/AGENTS.md
> Fixture installation playbook for openWEPP tests.

## Authorship
**This document and all AGENTS.md documents are maintained by GitHub Copilot /
Codex, which retain full authorship rights for all AGENTS.md content revisions.**
Revisions must preserve applicable user direction, package scope, review
expectations, and higher-precedence governance.

## Scope
- This directory stores durable inputs and observed-data fixtures used by
  integration tests, harnesses, authority suites, and comparator workflows.
- Keep fixtures deterministic, source-provenanced, and small enough for routine
  checkout unless the fixture family explicitly needs source-native external data.
- Do not install generated run outputs unless a test explicitly validates output
  parsing or protected-output behavior.

## Fixture Classes
- `infile/` - parser-level examples for individual WEPP input formats.
- `cli01/` - compact CLI run-directory fixtures.
- `watershed/` - multi-hillslope watershed model fixtures.
- `snowfreeze_observed/`, `snotel_observed/`, `cancov_forest/` - paired
  wepp.cloud model inputs plus observed validation data.
- `forest_lateral_flow_authority/` and similar future directories - source-native
  external-authority candidate datasets; not automatically active verdict suites.
- `constitutive/` - hand-authored or derived minimal state fixtures for contract
  and constitutive-law tests.

## General Rules
1. Read root `AGENTS.md`, `tests/AGENTS.md`, and this file before adding or
   changing fixtures.
2. Prefer source-native files plus a small normalized layer over undocumented
   hand edits.
3. Every fixture family needs a `README.md` or per-site `manifest.md` that states
   provenance, scope, units, known modifications, and intended tests.
4. Preserve source filenames when that helps auditability; use stable
   lower-snake-case directory names for local fixture directories.
5. Keep model inputs separate from observed data. Put observations under an
   `observations/` subtree unless the whole fixture family is source-native data.
6. Record checksums for acquired data (`SHA256SUMS`, `input-manifest.sha256`, or
   a family-specific equivalent) and verify them after installation.
7. Do not commit local `.venv`, caches, run products, temporary downloads, or
   credentials.

## wepp.cloud Hillslope Model Fixtures
- Record the source wepp.cloud run path or run slug, selected TopazID, resolved
  `wepp_id`/`pN`, modeled centroid/elevation when relevant, climate period, cover,
  soil, and any observation binding.
- For source-native wepp.cloud substrates, install the complete legacy
  single-hillslope set: `pN.run`, `pN.man`, `pN.slp`, `pN.sol`, `pN.cli`, plus
  hillslope sidecars such as `snow.txt`, `pmetpara.txt`, and `gwcoeff.txt`.
- Copy sidecar files from the same source wepp run directory as the selected
  `pN.*` files. Do not reconstruct them from defaults unless the fixture README
  documents the derivation. At minimum inspect and copy `snow.txt`,
  `pmetpara.txt`, `gwcoeff.txt`, and `frost.txt` when present.
- Do not imply that a line-oriented wepp.cloud `pN.run` is directly runnable by
  `openwepp-cli-hill`. CLI-runnable fixtures need a schema-versioned TOML
  runfile (`schema = "openwepp-hillslope-runfile-v1"`) that binds the source
  inputs and outputs.
- Current hillslope CLI recipe:

```sh
openwepp-cli-hill \
  --run-dir <fixture_dir> \
  --run-file <toml-runfile>.run \
  --output-dir <output_dir>
```

- Exclude watershed-scoped files (`chan.inp`, `chntyp.txt`, `tc.txt`,
  structure/impoundment files) from single-hillslope fixtures unless the test
  actually needs them.
- Treat `wepp_ui.txt` as an optional hillslope legacy feature-flag sentinel, not
  a categorical watershed file. Include it only when the test needs legacy
  `wepp_ui` behavior.
- If an as-built input is changed after extraction, document the exact line-level
  change and why. Examples include deliberate `ksflag` activation for frost
  observations. Do not leave silent model edits.
- Include a one-line run recipe in the family README or site manifest.

## wepp.cloud Watershed Model Fixtures
- Preserve the source substrate shape expected by watershed parsers/runners.
  Include every hillslope `pN.*` file needed by the selected watershed plus
  shared sidecars and watershed files such as channels, topology/translator
  inputs, `tc.txt`, `wepp_ui.txt`, impoundments, and structures when applicable.
- Copy shared sidecars from the source watershed `wepp/runs/` root along with the
  `pN.*` and `pw0.*` files. Common examples are `snow.txt`, `pmetpara.txt`,
  `gwcoeff.txt`, `wepp_ui.txt`, `tc.txt`, `tcr.txt`, `chan.inp`, `chntyp.txt`,
  `lcwb.txt`, and `phosphorus.txt`; install whichever are present and relevant
  to the runner path, and document any deliberate omissions.
- Record the source wepp.cloud run, model extent, number of hillslopes/channels,
  TopazID-to-`pN` mapping source, and any files deliberately omitted.
- Add an input manifest with SHA-256 hashes for all installed source inputs.
- A committed wepp.cloud watershed directory is a source/parser fixture unless it
  also includes the current openWEPP watershed TOML runfile
  (`schema = "openwepp-watershed-runfile-v1"`) and the pass/HBP bindings required
  by `openwepp-cli-watershed`.
- Current watershed CLI recipe:

```sh
openwepp-cli-watershed \
  --run-dir <fixture_dir> \
  --run-file <toml-runfile>.run \
  --output-dir <output_dir>
```

- Do not trim watershed fixtures by guessing unused files. Remove only files that
  are proven out of scope for the runner path under test and document that choice.

## Observed Dataset Fixtures
- Observations are external authority, not legacy parity targets. State the
  measurement method, units, cadence, period, coordinates/elevation when known,
  license/citation, and applicability limits.
- Keep raw/source-native data or provider metadata in `observations/provenance/`
  or a source-native fixture family. Put normalized tables in
  `observations/sites/`, `observations/profiles/`, or a similarly explicit
  subtree.
- Normalized CSVs must have stable columns, explicit units in the README or
  manifest, and deterministic missing-data handling. Do not silently canonicalize
  questionable measurements into model-friendly values.
- If an observed dataset is only a candidate authority, label it that way. Do not
  write defect verdict language until a science contract, rubric, or work package
  defines the acceptance envelope.
- For external-authority suite posture, cohort fixtures, or required-case binding
  edits, run:

```sh
bash tools/release/check_authority_suite_antievasion.sh
cargo nextest run --test auth11_required_suite_obligation_guards_contract
```

## Large And Binary Files
- Use Git LFS for source-native archives and binary data likely to grow history:
  `.zip`, `.pdf`, `.xlsx`, `.xls`, `.XLS`, rasters, NetCDF, parquet, and similar
  when the fixture family needs those files committed.
- LFS patterns are intentionally scoped. Add or update `.gitattributes` for the
  fixture family being installed; do not assume a global extension rule exists.
- Add or update `.gitattributes` before `git add`, then confirm with
  `git check-attr filter -- <path>`, `git lfs status`, and
  `git show :path/to/file` for representative staged files.
- Small text-like fixtures may remain normal Git objects when `git check-attr`
  is intentionally unspecified. Document that choice if the extension usually
  implies binary or large data.
- Prefer installing only the subset required by the fixture purpose. Full archives
  are acceptable when provider checksums, original layout, or reproducibility
  require them.

## Validation Checklist
- Check file inventory: `find tests/fixtures/<family> -type f | sort`.
- Verify checksums: `sha256sum -c <manifest>`.
- Confirm no HTML error pages were saved as data: use `file`, row counts, archive
  tests, or workbook readability checks appropriate to the format.
- For wepp.cloud model fixtures, run the intended CLI/harness path when practical.
- For observed-data fixtures, run the relevant harness classification or parser
  test when one exists.
- Record skipped validation plainly in the family README, package artifact, or
  final handoff.

## Common Pitfalls
- Mixing watershed files into hillslope fixtures without a consumer.
- Treating a source observation as a defect verdict before authority/rubric
  acceptance exists.
- Omitting the wepp.cloud run slug or TopazID-to-`pN` mapping.
- Committing transformed data without the raw source or enough provenance to
  reproduce the transform.
- Forgetting LFS before staging large binary sources.

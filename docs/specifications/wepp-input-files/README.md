# WEPP Input File Specifications

Canonical location for openWEPP-owned WEPP input-file specifications used by parser and science-contract work.

## Ownership (Locked)

openWEPP owns these specifications.

- `docs/specifications/wepp-input-files/specs/` is the canonical source of truth.
- Specs may be revised directly in this repository as architecture and contracts mature.
- For producer behavior, the `wepppy` stack is the canonical implementation reference and must be examined/cited when validating or amending input-file contracts.
- When a contract and `wepppy` producer behavior diverge, resolve the mismatch explicitly by correcting `wepppy`, this repo contract text, or both, with provenance recorded.

## Canonical Specs

- `specs/cligenparms.md`
- `specs/climate-file.spec.md`
- `specs/landuse-migration-cli.spec.md`
- `specs/management-yaml.spec.md`
- `specs/plant-file.spec.md`
- `specs/soil-file.spec.md`

Current gaps to author/import next:
- watershed input specs (for example `.str`, `.chn`, `.imp`)
- sidecar specs (for example irrigation sidecars, `pmetpara.txt`,
  `snow.txt`, `frost.txt`, `gwcoeff.txt`, `phosphorus.txt`, `wepp_ui.txt`,
  `tc.txt`, `tcr.txt`, `lcwb.txt`, `chan.inp`, plus explicit disposition of
  debug/instrumentation sidecars)

Explicit out-of-scope parser carry-forward:
- wepp-forest-revegetation sidecars (`firedate.txt`, `cancov.txt`,
  `simfire.txt`)

## Input Surface Registry (Normative)

- `input-surface-registry.md` is the canonical registry of parser-governed
  input surfaces and dispositions (`active`/`planned`/`deferred`/`unsupported`).

## Parser Contract Requirements (Normative)

- `parser-contract-requirements.md` defines required `SC-INFILE-*` content,
  including typed data-model requirements and parse-to-simulation propagation
  mapping obligations.

## Specification Authoring Procedure (Normative)

- `../wepp-input-specification-authoring-procedure.md` defines the required
  workflow for authoring and promoting a comprehensive WEPP input
  specification corpus across hillslope, watershed, and sidecar surfaces.

## Parser Authoring Procedure (Normative)

- `../wepp-input-file-parser-contract-authoring-procedure.md` defines the
  required authoring/review/disposition/verification workflow for
  `SC-INFILE-*` contracts.

## Initial Bootstrap Provenance

Initial content was bootstrapped from:
- `/workdir/wepppy/wepppy/weppcloud/routes/usersum/input-file-specifications/`
- `wepppy` commit `0c9eea73b4bb76af1e757d6b71816daf6ca7e607` (2026-05-19)

This provenance is historical context only and does not restrict openWEPP edits.

## Update Policy

- openWEPP edits are first-class, but producer-contract edits must be cross-checked against the canonical `wepppy` producer stack.
- If importing upstream changes later, treat them as proposed diffs and review before merge; if correctness gaps are found, patch upstream producer code and/or openWEPP contract text explicitly.
- Keep variable naming continuity with legacy WEPP/wepp-forest symbols when practical, with explicit alias mapping where openWEPP boundary names differ.

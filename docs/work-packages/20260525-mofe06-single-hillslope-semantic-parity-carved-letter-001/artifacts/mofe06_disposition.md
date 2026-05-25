# MOFE06 Disposition

- status: HOLD
- date: 2026-05-25

Static:
- Package scope is evidence execution only; no production code changes.

Ran:
- Selection and audit commands completed; candidate generation commands failed
  with typed parser errors as recorded in `mofe06-semantic-parity-execution-report.md`.

Disposition rationale:
- The package objective was executed through candidate selection and MOFE
  closure audit on carved-letter `H324`.
- openWEPP candidate generation is blocked by current carved-letter MOFE input
  compatibility gaps (`p324.run` legacy format; slope and soil parse failures
  under current parser expectations).
- Without a candidate WAT surface, semantic parity comparator execution cannot
  complete.

Blocking evidence:
- `CLIHILL-E-010` invalid TOML for legacy `p324.run`.
- `CLIHILL-E-010` slope parse failure (`expected integer, got '0.0000'`).
- `CLIHILL-E-010` soil parse failure (`SOL-E-006` OFE header arity mismatch).

Required follow-on:
- Parser/input compatibility closure for carved-letter MOFE slope/soil encodings
  (or an authoritative preprocessor contract) before re-running this parity lane.

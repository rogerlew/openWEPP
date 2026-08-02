# Security Impact

Status: `PASS / NO PRODUCTION IMPACT`

Evidence mode: **Static + Ran**.

- The executable path is a package-local analysis tool operating on copied
  fixtures beneath `target/`; source fixtures and production runtime inputs are
  unchanged.
- `OPENWEPP_*` state was sanitized for every run, preventing ambient selector
  or output configuration from changing the experiment.
- The release binary, source HEAD, tool, predecessor evidence, freeze, and each
  cell provenance record are hash-bound.
- Malformed climate rows fail closed. No fallback or default silently replaces
  missing required data.
- The exact diff contains no credentials, tokens, network access, production
  Rust, public schema, science contract, source observation, or fixture edits.

The package introduces no production attack surface or deployment behavior.

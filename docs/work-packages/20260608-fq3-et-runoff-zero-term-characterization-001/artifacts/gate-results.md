# Gate Results

Evidence mode: Ran

## Characterization Gates

1. 42-prefix comparator cohort executed from authoritative run root: PASS
2. Legacy WAT interchange non-empty and schema-valid: PASS
3. Group x term classification ledger produced: PASS
4. Defect-shaped handoff produced with authority envelopes: PASS
5. Truthfulness labels applied in artifacts: PASS

## Tooling Gates

- `wctl doc-lint --path /workdir/openWEPP/docs/work-packages/20260608-fq3-et-runoff-zero-term-characterization-001`: BLOCKED
	- Observed failure: panic in `ignore` crate (`path is expected to be under the root`) when invoked from `/workdir/wepppy` against cross-repo path.
	- This tooling issue is external to package content; characterization gates remain satisfied.

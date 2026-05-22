# ARCH19 Kickoff Agent Prompt

You are executing `20260522-arch19-run-and-parquet-boundary-contracts-001`.

Objectives:
1. Author canonical top-level `.run` boundary contract.
2. Author canonical parquet boundary contract with schema governance rules.
3. Inventory `/workdir/wepppyo3` parquet writer/schema surfaces and map them
   into openWEPP-owned authority statements.
4. Produce cross-file closure map linking `.run`/parquet boundaries to
   parser/runtime surfaces and follow-on package ownership.
5. Produce dual review/disposition/verification artifacts.

Constraints:
- openWEPP owns contract authority; external references inform but do not
  override openWEPP contracts.
- Preserve WEPP variable naming continuity and explicit alias mapping policy.
- Do not silently defer unresolved boundary ownership ambiguities.
- Correctness over completion: unresolved top-level boundary ambiguities remain
  `HOLD`.
- Use truthfulness posture (`Static:` vs `Ran:`) in artifacts.

Required outputs:
- `artifacts/run-boundary-contract-authority.md`
- `artifacts/parquet-boundary-contract-authority.md`
- `artifacts/wepppyo3-parquet-schema-reference-inventory.md`
- `artifacts/run-parquet-cross-file-closure-map.md`
- `artifacts/arch19-follow-on-acceptance-criteria.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/arch19_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- Docs-only scope: static validation and artifact completeness checks.
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

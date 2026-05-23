# PL09 Kickoff Agent Prompt

You are executing
`20260522-pl09-openwepp-totality-and-pl08-hold-lift-discovery-001`.

Domain context (explicit):
- This is a greenfield scientific hydrology simulation engine (`openWEPP`).
- This package is discovery/governance for PL hold-lift readiness, not new
  kernel/process implementation.
- Legacy comparator/provenance authority defaults to
  `/workdir/wepp-forest_260430_baseline` per ADR-0012.

Objectives:
1. Inventory the totality of implemented openWEPP surfaces relevant to PL
   parity and PL08 hold status.
2. Perform representation/discovery decomposition against
   `/workdir/wepp-forest_260430_baseline` for PL growth/decomposition/residue
   semantics.
3. Synthesize PL08 hold evidence into explicit blockers vs investigatory gaps
   using confidence-tier policy.
4. Publish dependency-ordered work-package queue required to lift PL08 hold.

Mandatory scope boundaries:
- Include: parser seams, runtime projection seams, typed kernel-facing PL
  state surfaces, comparator-readiness surfaces, and prior PL package evidence
  relevant to PL08 hold.
- Exclude: implementing new process physics or treating comparator replay as a
  substitute for representation discovery/decomposition.
- Exclude: rewriting non-PL package dispositions except where they directly
  constrain PL08 hold-lift decisions.

Constraints:
- Preserve architecture-first and science-contract authority posture.
- Do not invent physics/semantics; source every claim from baseline code,
  openWEPP code, contracts, or recorded evidence.
- Use pinned baseline authority by default.
- Use evidence-mode labeling (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion; unresolved high-severity hold blockers remain
  `HOLD`.
- Apply confidence-tier policy explicitly:
  Tier-A unresolved parity blockers keep PL08 in `HOLD`;
  Tier-B/Tier-C deltas are investigation signals unless severity escalates.
- Preserve canonical symbol continuity from legacy WEPP/OpenWEPP contracts;
  when boundary names differ, include explicit alias mapping.

Required outputs:
- `artifacts/openwepp-totality-implementation-inventory.md`
- `artifacts/wepp-forest-pl-representation-decomposition-map.md`
- `artifacts/openwepp-vs-baseline-pl-parity-gap-register.md`
- `artifacts/pl08-hold-evidence-synthesis.md`
- `artifacts/pl08-hold-lift-decision-record.md`
- `artifacts/pl08-hold-lift-work-package-queue.md`
- `artifacts/worker-handoff.md`
- `artifacts/owned-file-manifest.md`
- `artifacts/gate-results.md`
- `artifacts/pl09_disposition.md`
- `artifacts/review_agent_a.md`
- `artifacts/review_agent_b.md`
- `artifacts/verification_agent_a.md`
- `artifacts/verification_agent_b.md`

Required gates:
- Docs-only execution: artifact completeness and consistency checks.
- Docs checks must confirm artifact-to-artifact consistency (inventory ->
  parity gaps -> hold evidence -> decision record -> queue -> disposition).
- If code is changed, run:
  1. `cargo fmt --check`
  2. `cargo clippy --workspace --all-targets -- -D warnings`
  3. `cargo test --workspace`
  4. `cargo deny check`

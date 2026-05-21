# INFILE17 Kickoff Agent Prompt

You are executing 20260520-infile17-author-sc-infile-tcr-001.

Objectives:
1. Author/update `docs/specifications/wepp-input-files/specs/tcr.spec.md` with openWEPP-owned parser specification.
2. Author/update `SC-INFILE-TCR-001` under canonical contract location.
3. Ensure full parser-contract section compliance per requirements.
4. Produce independent dual-agent review artifacts and dispositions.
5. Produce dual-agent verification artifacts after fix pass.
6. Update registry status for `infile-channel-tcr` consistently with disposition.

Constraints:
- Evidence mode: `Static` unless execution is explicitly run.
- Use `[DIRECT]` and `[INFERENCE]` evidence tags per claim.
- Use legacy WEPP/wepp-forest variable symbols as canonical names; add explicit
  alias mapping for openWEPP boundary names.
- Every invariant/parser rule must map to an explicit enforcement guard path.
- Correctness over completion: unresolved critical correctness gaps must remain
  `HOLD`.

Required outputs:
- `docs/specifications/wepp-input-files/specs/tcr.spec.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md`
- `artifacts/parser-contracts/SC-INFILE-TCR-001/contract_ref.md`
- `artifacts/parser-contracts/SC-INFILE-TCR-001/review_agent_a.md`
- `artifacts/parser-contracts/SC-INFILE-TCR-001/review_agent_b.md`
- `artifacts/parser-contracts/SC-INFILE-TCR-001/disposition.md`
- `artifacts/parser-contracts/SC-INFILE-TCR-001/verification_agent_a.md`
- `artifacts/parser-contracts/SC-INFILE-TCR-001/verification_agent_b.md`

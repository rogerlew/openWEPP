# INSPEC07 Kickoff Agent Prompt

You are executing 20260520-inspec07-author-wepp-input-spec-pmetpara-001.

Objectives:
1. Author `docs/specifications/wepp-input-files/specs/pmetpara.spec.md` as canonical openWEPP WEPP input specification.
2. Satisfy the required section set from
   `docs/specifications/wepp-input-specification-authoring-procedure.md`.
3. Anchor claims to usersum2024 and legacy/modern provenance sources with
   explicit evidence tags.
4. Produce independent dual-agent review and disposition artifacts.
5. Produce dual-agent verification artifacts after fix pass.
6. Declare handoff mapping to parser contract `SC-INFILE-PMETPARA-001`.

Constraints:
- Evidence mode: `Static` unless execution is explicitly run.
- Use `[DIRECT]` and `[INFERENCE]` evidence tags per claim.
- Use legacy WEPP/wepp-forest symbols as canonical field names and provide
  explicit alias mapping when boundary names differ.
- Correctness over completion: unresolved critical gaps remain `HOLD`.

Required outputs:
- `docs/specifications/wepp-input-files/specs/pmetpara.spec.md`
- `artifacts/input-specs/SPEC-INFILE-PMETPARA-001/spec_ref.md`
- `artifacts/input-specs/SPEC-INFILE-PMETPARA-001/review_agent_a.md`
- `artifacts/input-specs/SPEC-INFILE-PMETPARA-001/review_agent_b.md`
- `artifacts/input-specs/SPEC-INFILE-PMETPARA-001/disposition.md`
- `artifacts/input-specs/SPEC-INFILE-PMETPARA-001/verification_agent_a.md`
- `artifacts/input-specs/SPEC-INFILE-PMETPARA-001/verification_agent_b.md`

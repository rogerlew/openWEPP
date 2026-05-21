# Review Agent B — SPEC-INFILE-WATERSHED-CHANNEL-CHN-001

Evidence: Static

## Findings (severity-ranked)

### B1 — High
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:147`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:201`
- Issue: `ipeak > 2` conditional dependency on `chan.inp` is documented, but typed error/default behavior is not normatively defined for strict mode vs compatibility fallback.
- Why it matters: This is a control-flow-affecting branch; missing normative behavior prevents deterministic parser-contract guard mapping and may cause inconsistent simulation assembly outcomes.
- Proposed disposition: `amend` (add explicit strict-mode error and compatibility-mode fallback contract for missing/invalid `chan.inp`).

### B2 — Medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:211`
- Issue: "Minimal valid" example uses `ipeak=4`, which triggers `chan.inp` coupling, but the example is presented without required sidecar context.
- Why it matters: Example suite should provide self-contained validity patterns; this can mislead fixture authoring and parser conformance tests.
- Proposed disposition: `amend` (either provide companion `chan.inp` snippet/context or change minimal valid `.chn` example to a mode that does not require sidecar coupling).

### B3 — Low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/watershed-channel-file.spec.md:10`
- Issue: `last_updated_utc` is date-only (`2026-05-20`) instead of UTC timestamp format used elsewhere.
- Why it matters: Metadata consistency supports automated linting and traceability in review/disposition tooling.
- Proposed disposition: `amend` (normalize to full UTC timestamp, e.g., `YYYY-MM-DDTHH:MM:SSZ`).

## Final Recommendation
`HOLD`

Rationale: High-severity sidecar-branch behavior remains underspecified and must be resolved before promotion.

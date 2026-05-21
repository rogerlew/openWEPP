# Review Agent B — SPEC-INFILE-SLOPE-SLP-001

Evidence: Static

## Findings (severity-ranked)

### B1 — Medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:177`
- Issue: Invalid example "mixed `xinput` mode" is not actually a decisive mixed-mode case.
- Why it matters: The example uses `0.0` and `100.0`; because `0.0` is valid in both normalized and absolute modes, this case can be interpreted as purely absolute-distance mode, which weakens the normative guard definition.
- Proposed disposition: `amend` (replace with an unambiguous mixed-mode violation, e.g., include both a normalized mid-point and an absolute terminal point inconsistent with the declared mode).

### B2 — Medium
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:43`, `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:130`
- Issue: Legacy no-version compatibility branch is defined in the datver matrix, but typed error/default behavior does not explicitly define strict-mode rejection when compatibility mode is disabled.
- Why it matters: Procedure requires explicit defaulting/missing behavior and guardable outcomes; without an explicit strict-mode failure path, parser-contract guard mapping is underspecified.
- Proposed disposition: `amend` (add explicit error expectation for no-datver input when compatibility mode is off).

### B3 — Low
- File/line: `/home/workdir/openWEPP/docs/specifications/wepp-input-files/specs/slope-file.spec.md:214`
- Issue: `SLOPE-GAP-004` is listed inside HOLD register but marked as non-blocking.
- Why it matters: This can create ambiguity in promotion gates and disposition tracking.
- Proposed disposition: `amend` (mark as provenance note outside HOLD blockers, or clearly classify as non-blocking in register schema).

## Final Recommendation
`HOLD`

Rationale: Required sections are present, but unresolved HOLD gaps remain and amendments above are needed to tighten correctness and guard-path clarity.

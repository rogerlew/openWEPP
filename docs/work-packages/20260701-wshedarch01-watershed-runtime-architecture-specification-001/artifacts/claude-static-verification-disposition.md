# Claude Static Verification Disposition

Status: `UPDATED`

Evidence class: `Static`.

Scope: disposition of Claude's static verification review of
`docs/architecture/watershed-runtime-architecture-specification.md` Draft,
Revision 2.

## Review Verdict

Claude verified that the specification is accurate and grounded against the
recorded WSHEDPERF01 artifacts, current watershed CLI/orchestrator source,
configuration, package dispositions, and architecture references. The review did
not rerun cargo, tests, or benchmarks.

## Confirmed Evidence

- Referenced authority documents exist and the architecture README indexes the
  watershed runtime spec.
- WSHEDPERF01 timing and profile numbers in the spec match recorded artifacts.
- The current-runtime description matches `openwepp-cli-watershed` and
  `WatershedWritebackSurface` source structure.
- The silent zero synthesis concern is real in the current CLI when
  latest-event payloads are absent.
- The WSHEDPERF01 shell-loop/shared-output characterization is supported by the
  baseline command log.
- The closure gates match root governance, and `--jobs` is genuinely new.
- Dual-review findings were dispositioned in Revision 2.

## Findings Disposition

| Finding | Disposition | Evidence |
| --- | --- | --- |
| WSHEDPERF01 accepted baseline used `--legacy-sidecar-discovery`, while future canonical benchmark mode should remove discovery. The spec needed a sidecar-discovery axis so discovery-on/off timings are not compared as the same scope. | `accepted-fixed` | Revision 4 adds a sidecar/input-discovery measurement axis, labels WSHEDPERF01 as `legacy-sidecar-discovery-on`, requires performance reports to record discovery mode, and adds W3 acceptance language preventing discovery-on/off conflation. |
| Follow-on ADR and ROADMAP entries were missing even though the spec requires ADR ratification before becoming binding. | `accepted-fixed-for-roadmap`; `ADR-open-by-design` | Revision 4 adds a watershed runtime performance queue to `docs/ROADMAP.md` and updates ratification requirements. The ADR itself remains excluded scope for WSHEDARCH01 and is queued as WSHED-ADR. |
| Section 3.6 used logical output name `chan.out.parquet`; the runfile field is `chan_out`. | `accepted-fixed` | Revision 4 annotates `chan.out.parquet` with the `chan_out` output/runfile field. |
| Whether a pass with no latest-event payload can be valid `NoEvent` rather than a hard error is unresolved. | `accepted-follow-up` | Revision 4 makes the question explicit in Open Questions and updates W2 acceptance so latest-event payload handling is resolved contract-first before implementation. |

## Result

Claude's verification is incorporated. The spec remains draft authority pending
ADR ratification and implementation evidence.

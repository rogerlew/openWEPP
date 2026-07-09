# Review

Status: `EXECUTED-COMPLETE`
Evidence: `Static + Ran`

## Review 1: Noether Lineage Review

Static: Noether performed read-only lineage review of the pinned legacy
`chan.inp` absent/open-error branch. Findings:

- Accepted: pinned legacy directly supports `cbase=0`, `ichout=0`,
  `nchnum=0`, no selected channel output IDs, and no channel output.
- Accepted: legacy `dtchr` is ambiguous on open/line-1 failure because the
  shared normalization block can execute without a freshly read value. WSHED-W10
  therefore ratifies the deterministic openWEPP compatibility value
  (`dtchr_input_s=60`, `dtchr_norm_s=60`, `ntchr=1440`) in
  `SC-INFILE-CHANINP-001` and `SC-SYSTEM-001`.
- Accepted: current code before WSHED-W10 had three mismatches: parser default
  `ichout=1`, frame rejection of defaulted parser outcomes, and CLI hidden
  `None` fallback with `dtchr=3600`/`ntchr=24`.

Disposition: all accepted and corrected.

## Review 2: Kuhn Rust Review

Static + Ran: Kuhn performed a read-only post-implementation Rust review and ran
`git diff --check`.

| Finding | Severity | Disposition |
| --- | --- | --- |
| `NotApplicable` could become a hidden fallback for `ipeak > 2` if a mismatched `ChaninpFile` reached `WatershedNetworkFrame`. | Medium | Accepted. Added an `ipeak` consistency guard in `build_routing_globals`, retained `ChaninpNotRuntimeReady` for this hard-fail path, and added `wshedw10_not_applicable_chaninp_cannot_mask_required_channel_sidecar`. |
| `ChaninpNotRuntimeReady` was stale/unreachable after accepting defaulted compatibility states. | Low | Accepted. Repurposed the error for parser/channel context mismatch and illegal not-applicable handoff, with both `chaninp_ipeak` and `channel_ipeak` in the error. |
| CLI warning provenance dropped `ChaninpWarning.line`. | Low | Accepted. Added `format_chaninp_warning` so line-bearing parser warnings surface as `chan.inp CODE line N message`; missing-file warnings remain line-free. |
| Configured missing `inputs.chaninp` behavior is not directly covered by a new W10 test. | Residual risk | Documented. Source still fails configured unreadable sidecar at `CLIWAT-E-029`; WSHED-W10's changed path is unconfigured/missing legacy behavior. |
| Package truthfulness artifacts still said `EXECUTING`. | Artifact defect | Accepted. Updated package and artifacts to `EXECUTED-COMPLETE` after closure gates. |

## Local Review

Static: local review checked that:

- `DefaultedCompat` and `OpenErrorCollapsedCompat` require explicit parser
  options before frame construction can proceed.
- `NotApplicable` falls through only when parser and channel `ipeak` agree and
  the channel branch is not `ipeak > 2`.
- The CLI still rejects explicitly configured unreadable `inputs.chaninp` before
  runtime (`CLIWAT-E-029`), preserving operator-error protection.
- No hidden `dtchr=3600` fallback string remains in production code.

Disposition: no unresolved findings.

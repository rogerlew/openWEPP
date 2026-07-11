# Verification Agent A

Status: `PASS — EXECUTED-HOLD-SANITY-FAIL LEGITIMATE`

Evidence mode: `Static + Ran`

Static: re-read Review Agent A, both review findings and their disposition,
the corrected test harness, package acceptance rules, HBP/`chan.inp` fixture
construction, sanity and release evidence, final gate evidence, finding
mechanisms, line-count/security posture, and queued WSHED-W11D successor.

Ran: lightweight verification only. `git diff --check` passed; `wc -l` reports
1,309 lines for the changed Rust test; final release-log recount confirms 35
result rows, 33 finding rows, and 12 timestep-comparison rows. I did not rerun
the matrix or workspace cargo gates; this verification inspected their retained
final-tree logs and run IDs.

## Review A finding closure

| Finding | Verification | Evidence |
|---|---|---|
| A-001 | PASS, fixed | The generated sidecar now writes topology element IDs `3 4`; the harness requires `ParsedBranch`, zero warnings, normalized 3,600/600-second grids, `nchnum=2`, and `[3, 4]`. `test-design.md` matches the code. Corrected debug run `f695f3db-0627-4c28-8d97-8e5c5d023158` and exact-release run `29024159-9f78-4506-9918-09c7f007af0d` reran all 35 cases. |
| A-002 | PASS, fixed | Both written HBP files are reparsed with expected hillslope IDs and their serialized hourly arrays are summed as external-input authority. Test, package, catalog, and evidence now call the water identity a serialized-input routed ledger/algebraic diagnostic and explicitly reject it as independent anti-tautological conservation proof. Negative storage is not accepted through the exact ledger identity. |
| A-003 | PASS, fixed | `release-binary-provenance.md` records the exact release build command, absolute child binary, SHA-256, size/mtime, exact environment-plus-nextest command, run ID, and log. Gate evidence records focused/release/protected tests, format, focused/workspace clippy, erosion, full workspace, deny, Markdown lint, and diff hygiene. |
| A-004 | PASS, fixed | Final changed test is 1,309 lines, below the 2,000-line WARN and 3,000-line blocker thresholds; no production Rust file changed. |

Historical review artifacts correctly retain the originally observed source
locations and counts; `review-disposition.md` and current implementation/evidence
carry the accepted corrections.

## Result and fixture verification

- Branch coverage is complete: CREAMS (`ipeak=2`) has five cases; KW, static
  MC, and variable MC (`ipeak=3,4,5`) each have five cases at both 3,600 and
  600 seconds, totaling 35 real-CLI executions and 12 timestep comparisons.
- The sidecar is canonical and warning-free on wave branches. H1/H2 payload
  state is reparsed from disk; wave cases retain paired hourly EVENT authority,
  while CREAMS uses the intended EVENT/NOEVENT forms.
- Exact zero behavior, finite outputs, shape sensitivity, later-pulse storage
  distinction, serialized-input algebraic ledger, and 240 kg wave sediment
  closure pass. These passing diagnostics do not override failed physical
  sign/publication requirements.
- Test-only binary selection remains bounded to the integration harness and
  child execution uses explicit `Command` arguments. Security impact remains
  `NONE`.

## SANITY-FAIL reconfirmation

`SANITY-FAIL` remains directly supported on the corrected final tree:

- `W11C-F001`: KW and static MC still publish material first-day negative
  routed storage. Corrected release minima remain `-65.192021 m3` and
  `-210.400475 m3`, exactly associated with terminal output beyond the sole
  7,200 m3 external input. The package no longer misapplies a dedicated
  storage tolerance; it states the zero-initial/source/loss magnitude basis
  and leaves lower-bound authority/correction to W11D.
- `W11C-F002`: passive static/variable MC peak ratios still reach
  `1.549880`, and variable-MC early-spike peak still changes
  `1.185839 -> 3.071519 m3/s` across the two grids. This is correctly retained
  as a High defect-shaped investigation finding, not adjudicated by an invented
  universal threshold.
- `W11C-F003`: event-scalar CREAMS still publishes 14,400 m3 from 7,200 m3
  external input, first-channel identity, and nonterminal/rate-like sediment.
  The harness no longer asserts the known-bad first-channel ID as required
  behavior, so a future production correction will not be defect-locked.
- `W11C-F004`: the old three-record `nchnum=0` sidecar's 60-second compatibility
  alias remains proven; the corrected W11C `nchnum=2` sidecar now separately
  proves real 3,600/600-second execution without claiming the parser defect is
  fixed.

The physical-sanity gate is therefore truthfully `FAIL` even though the
characterization test executable and every regression/quality gate pass.

## Gate and disposition legitimacy

Retained final-tree evidence records:

- focused corrected matrix: PASS, 35 cases;
- protected test file: PASS, 3/3;
- exact release matrix: PASS, 35 results and retained findings;
- workspace clippy: PASS;
- erosion profile: PASS, 313/313;
- full profile: PASS, 1,678/1,678;
- cargo deny, formatting, Markdown lint, and diff hygiene: PASS;
- physical sanity acceptance: FAIL.

The required physical gate is current-scope and is not deferred or relabeled.
Because W11C explicitly excludes production correction, complete disposition
would be false; `EXECUTED-HOLD-SANITY-FAIL` is the legitimate terminal state
for this characterization package.

WSHED-W11D is queued as a contract-first DC-ExecPlan and owns all four defect
families under a concrete correction authority envelope: KW/MC recurrence and
storage, terminal event-scalar publication, and canonical `nchnum=0` parsing.
It prohibits clamps, peak clipping, mass masking, silent defaulting, and
surrogate physics, and requires corrected release/closure/review gates.

No blocking Verification A finding remains. The package's currently
`EXECUTING` disposition/catalog placeholders and dual-verification gate may be
transitioned to `EXECUTED-HOLD-SANITY-FAIL` only after Verification B also
lands; that ordering is correct and is not evidence deferral.

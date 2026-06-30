# Disposition

Evidence class: Ran + Static

Result: `EXECUTED-HOLD-STAGE0-PREMISE-CORRECTED`.

Stage 0 executed against the current direct production path and produced a clear
correction:

- H2637 direct production RSS is approximately `1.16 GiB`.
- Removing optional WAT/PASS/plot outputs from the H2637 runfile did not
  materially change RSS.
- A tiny two-day fixture used only `19584 KiB`.
- Static code review shows unconditional whole-run publication retention,
  cloning, and projection-row materialization.

The setup-time symbol-map carrier remains an architecture violation and a
necessary deletion target. It is not, on the evidence gathered here, the first
RSS lever. The package therefore stops before Stage 1 because the package itself
required RSS to drop materially per stage, and typed setup deletion alone does
not have evidence that it can meet that gate.

## Named Blocker

`BLOCKED-BY-RETAINED-DIRECT-PUBLICATION-RSS`.

The current direct path retains all direct publication rows and builds output
projection vectors even for minimized-output runs. That retained-publication
state must be streamed, dropped, or summarized before the typed setup rewrite can
be expected to satisfy the RSS gate.

## Follow-On First Action

Scaffold a `DIRECT PUBLICATION STREAMING / RETAINED LEDGER TRIM` package with
these first implementation targets:

1. Remove the `DirectPublicationExecution` clone in direct output assembly.
2. Conditionalize WAT/PASS projection-row construction on requested outputs.
3. Stream direct publication rows into HBP/loss/manifest/WAT/PASS consumers, or
   retain only compact summaries needed by those consumers.
4. Re-run the same H2637 full/minimized/small-fixture RSS profile and byte
   identity gates.

After that package materially reduces RSS, resume typed setup and symbol-map
carrier deletion with a realistic RSS baseline and unchanged identity gates.

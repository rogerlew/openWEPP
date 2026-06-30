# Disposition

Result:
`EXECUTED-CHECKPOINT-GATE1-PASSED-PHASE3-PENDING`.

This package expanded the typed day-zero seed-computation slice. The production
surface seeder now calls typed projection cores for:

- WB18/WB19 lane substep controls and multi-OFE hourly-carry activation;
- rainfall/hyetograph normalization;
- WB11 initial layer water stores and storage totals;
- fine-frost frozen-depth refresh;
- residue interception and `Ws` defaults;
- WB12 reconciliation seed defaults;
- ET-demand seed, including Priestley-Taylor and EVAPPM/PMET branches;
- `efflen` and default `m`;
- WB16 `ealpha` compatibility-default decision.

Focused shadow tests prove those typed projection values match the existing
day-zero surface outputs. The broader WB11 publication test slice and focused
clippy passed.

Autonomous completion resume built the typed carrier and cut production
consumers over to it. Production direct execution and snowbench diagnostic
replay now construct seed authority from typed parsed inputs, sidecars, and
day-one climate. The lane constructor, `DirectProductionDayInputBuilder`,
coupling metadata, Wave-2 flag, and winter hourly geometry all consume the
typed carrier.

Gate 1 passed with real endpoint evidence:

- H2637 HBP/loss/PASS/WAT/plot are byte-identical against clean `5b139058`.
- cli01 HBP/loss/WAT/plot are byte-identical.
- The focused multi-OFE/Wave-2 fixture passes.
- H2637 current run remains direct with `compatibility_edge_invocations=0`.
- H2637 RSS improved from `113268 KiB` to `91692 KiB`.
- Full workspace gates passed, including nextest full, clippy, deny,
  anti-evasion, required-suite obligation guard, Markdown lint/validate, and
  whitespace diff check.

Next action: Phase 3 deletion of the now-orphaned symbol-map seed authority and
production compatibility runtime machinery, keeping only the explicit
deprecated `--compatibility-runtime` seam.

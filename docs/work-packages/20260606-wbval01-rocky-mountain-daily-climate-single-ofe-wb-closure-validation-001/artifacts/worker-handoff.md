# Worker Handoff

Status: executed-hold

Evidence mode: Static

Static:

WBVAL01 is executed-hold.

What was learned:

- `/wc1/runs/in/indispensable-presenter/wepp/runs/` contains `22` single-OFE
  hillslopes (`p1..p22`) plus `pw0` as a `9`-OFE observed-only surface.
- Current `openwepp-cli-hill` does not parse legacy WEPP text `.run` files;
  generated TOML wrappers are required for this lane.
- The run's empty `wepp_ui.txt` sentinel selects the hourly lane while the
  `.cli` files remain CLIGEN daily, non-breakpoint forcing.
- `12/22` single-OFE hillslopes emit WAT ledgers and all `12` are
  `conservation-break` for full years `2..6`.
- `10/22` fail closed before WAT publication:
  - `CLIM-RUNTIME-E-017`: `p2`, `p4`, `p6`, `p9`, `p14`, `p17`
  - `HKERNEL-WB11-PERC-E-003`: `p7`, `p11`, `p18`, `p20`
- Year `1` needs an initial storage surface before full-calendar conservation
  classification can be made.

Next mechanism rung:

1. **Frost** remains the next mechanism rung.
2. Start with emitted conservation-break targets in priority order:
   `p19`, `p22`, `p16`, `p5`, `p21`, `p3`, `p8`, `p15`, `p10`, `p12`, `p1`,
   `p13`.
3. Preserve the snow/`RM` comparator route suspension behind
   `docs/backlog/20260605-snow-code-deferred-science-review.md`.

Required unblocker before full-population closure:

- Address the `CLIM-RUNTIME-E-017` and `HKERNEL-WB11-PERC-E-003` fail-closed
  blockers with contract-first evidence. Do not clamp, normalize, or
  canonicalize-and-proceed through either domain violation.
- Add or expose a pre-day-1 initial storage surface if year-1 full-calendar
  closure remains a package requirement.

Primary artifacts:

- `run-manifest.md`
- `single-ofe-closure-ledger.md`
- `rung2-frost-target-handoff.md`
- `gate-results.md`
- `disposition.md`
- `review-disposition.md`

# HOLD Legitimacy Audit

Status: **EXECUTED-HOLD-ACTIVE-OWNER-TIMING-BUDGET**.

Evidence mode: Ran + Static.

## Closed Blocker

### HOLD-1 from D15 rerun: `NegativeOutletBin`

Ran:

- Baseline H2637 ignored shadow test failed with `NegativeOutletBin`.
- After correction, the same test passed: `1` passed / `1` skipped,
  `484.578 s`.
- Release shadow endpoint timing now completes: `91.59 s` user / `1:31.67`
  wall.

Static correction: the shadow runtime intended a `+6 h` drain tail after the
last active source hour but capped the route window at one day. For day-88
hour-24 source, the cap removed the tail and left the terminal negative bin
unabsorbed. The correction keeps the 24-hour source window and allows the
zero-source drain tail to run.

## Remaining Hold Conditions

### HOLD-A: Active production owner path is absent

Static:

- `05_runner_execution_and_outputs.rs` creates only the optional
  `OPENWEPP_LANED_SHADOW` collector.
- `runoff.rs` still calls `apply_dc01_runon_supply_admission()` in production.
- `seam.rs` has activation helpers, but production runtime does not invoke the
  active closure hard-fail.
- Production erosion inputs still select `Dc01SourceShape` with
  `routed_hydrograph_runoff_fraction: None`.
- The direct executor runs each lane/day through all production spans before
  post-row dynamic transfer publication. A real active owner needs routed water
  available before downstream runon admission and before erosion consumes the
  hourly shape.

Why this is legitimate: implementing active ownership as a post-hoc shadow
collector or compatibility wrapper would be producer-only evidence and would
not move the real downstream consumers. The required fix is a production phase
integration change: compute/source the routed water at the correct point in
the day/lane execution order, disable DC01 for active routed lanes, feed
closure and erosion consumers, and preserve default/off identity.

First actionable follow-on: open an active-owner implementation package that
adds an explicit opt-in selector and restructures the direct production
day/lane execution so routed-water ownership occurs before downstream runon
admission and erosion shape consumption.

### HOLD-B: H2637 opt-in timing exceeds the D14 budget

Ran:

- Current default/off: `2.49 s` user / `0:02.51` wall.
- Current shadow-on: `91.59 s` user / `1:31.67` wall.
- D14 optimized shadow budget: about `29.9 s` wall/user.
- Current slot profile records `16,936,089` solver steps versus D14's
  `10,334,879` witness.

Why this is legitimate: D15 requires an endpoint timing refresh before
activation. The terminal fix makes the endpoint available, but the refreshed
cost is materially above the recorded D14 budget and has not been operator-
adjudicated as acceptable for opt-in production. Proceeding to an activation
claim would hide a current activation precondition behind a successful
correctness fix.

First actionable follow-on: open the next D15 package to resolve the active
production ownership blocker and optimize/adjudicate the `~91.6 s` H2637
opt-in timing before activation.

## Considered In-Envelope Route

The direct route would be to implement the active owner in this package after
the terminal-bin fix. That route was rejected because it would require a
production execution-order change and timing optimization/adjudication beyond
the safe closure envelope. No partial activation flip was made.

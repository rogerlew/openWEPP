# Execution And State Map

Status: `executed`

Evidence mode: `Static + Ran`

## Daily Order

1. The runner reads start-of-day snow state and constructs the complete daily
   snow partition at
   `00c_day_input_builder_impl.rs:158-218`.
2. Frost consumes the prior/start-of-day snow geometry at `:219-231`.
3. Active snow advances sequentially for 24 hours at
   `infiltration_reconciliation.rs:883-925`. A new pack started in an hour does
   not melt until a later hour; a negative daily mean suppresses melt for the
   whole day at `:1313-1344`.
4. Each warm-branch hour mixes new snow, evaluates signed CoE melt, applies only
   positive melt to depth, fills/releases capacity, applies sublimation, and
   carries depth, CoE density, retained liquid, and albedo state.
5. Daily finalization sums signed raw diagnostics, routes positive hourly state
   loss, bounds it by available SWE, reconstructs SWE, adds released rain, and
   rescales the positive hourly shape to the authoritative daily routed amount
   at `infiltration_reconciliation.rs:1885-1935,1964-2037` and
   `runoff_reconciliation.rs:2294-2323`.
6. The orchestrator resolves physical density after the CoE boundary and then
   invokes Stage 3 on the post-CoE/post-density layers at
   `runoff_reconciliation.rs:287-332,536-798`.
7. Stage 3 changes its diagnostic layer temperature, cold content, retained
   liquid, refreeze, and meltwater temperature. It leaves CoE SWE and routed
   liquid authoritative, except that an enabled Stage-3 sublimation selector
   may remove mass at `runoff_reconciliation.rs:333-430`.
8. The runner sends residual rain through interception, adds routed melt once
   to the hyetograph, and supplies the same daily liquid to the runtime at
   `00c_day_input_builder_impl.rs:309-364`.
9. Direct runtime consumes liquid before R4G projects the already-computed snow
   carry state (`direct_runtime/03_executor.rs:1333-1353`). Storage carries SWE,
   depth, density, retained liquid, and layers at
   `direct_runtime/storage.rs:525-613`; publication reads runtime SWE/depth at
   `direct_runtime/01_publication.rs:493-522`.

## Authoritative State And Diagnostic Aliases

| Surface | Role | Mutation/carry rule |
|---|---|---|
| CoE depth/density | Authoritative hourly melt boundary | Separate carry for nonlegacy physical density; positive melt changes geometry, negative melt remains diagnostic. |
| Runtime SWE | Authoritative daily snow mass/publication | Reconstructed from prior SWE plus primitive additions and removals; not independently debited downstream. |
| Physical density/layers | Authoritative depth/density and Stage-3 geometry | Updated after CoE mass; does not feed the next nonlegacy CoE boundary. |
| Retained CoE liquid | Authoritative in-pack liquid store for capacity routing | Bounded by current 1% pore-volume capacity and released on excess/contraction/exhaustion. |
| `raw_melt` | Diagnostic alias | Signed sum of hourly empirical CoE terms; not an additive mass sink. |
| `routed_melt` | Downstream liquid alias | Exactly pack loss plus released rain; added once after interception. |
| Stage-3 retained/refrozen liquid | Diagnostic thermal/liquid state | Does not reduce actual routed melt or restore SWE under INV-080/081. |

## Reachable Mixed-Sign Branch

The code routes `sum(max(hourly melt,0))` while retaining negative terms only in
the raw diagnostic (`runoff_reconciliation.rs:2294-2323`). Accepted v3 traces
contain 6,716 primary-window days with both positive and negative hourly terms,
including 1,510 at Snowbird. They contain 1,031 mixed-sign days with positive
routed liquid and positive Stage-3 refreeze, including 298 at Snowbird.
Snowbird loses `1.019 m` over 1,150 negative-daily-net days.
This contradicts the factual reachability premise in
`SC-SNOWFREEZE-001` INV-015 and activates its explicit re-adjudication clause.
It does not prove that simply netting negative melt into SWE would be physical.

## Publication And Closure Result

Independent all-row reconstruction found maximum absolute residuals of
`1.56e-17 m` for `routed - pack_loss - rain_released` and `1.00e-12 m`
for daily storage. Omitted or duplicated routed melt, released rain, or runtime
SWE in the trace is therefore excluded as the accumulation-deficit source.
WAT values were checksum-bound but not parsed; publication evidence is limited
to static real-consumer lineage and does not claim dynamic WAT value closure.

## Missing Produced Surface

`DirectSnowStage3Diagnostics` contains `incoming_liquid_m`,
`routed_liquid_m`, `retained_liquid_m`, and `liquid_closure_residual_m`, but the
real JSONL consumer omits them. Layer differences are not a valid substitute
because density projection can split, merge, trim, or create layer geometry.
This current-scope evidence gap prevents independent Stage-3 liquid closure.

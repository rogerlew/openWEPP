# Result-Blind Protocol Amendment

Status: `v1 rejected before result execution / v2 frozen`

Evidence mode: `Static`

No new CLI replay or result-bearing analysis had started when four independent
static audits challenged the scaffold freeze. The original
`analysis-freeze.json` remains byte-identical to scaffold commit `6ab0946b` as
custody evidence, but it is not the execution authority.

The audits found three closure-blocking defects:

1. peak, event, class, aggregation, eligibility, and zero-denominator operators
   were incomplete;
2. the downstream-dominance verdict was causally inverted and ignored the
   accepted Stage-3 snow-neutral direct-path result; and
3. upstream gross-positive/loss tracking is adjacent-ledger localization, not
   independent proof that generation is excessive.

They also established that `stage3_cold_content_before_j_m2` is measured after
the CoE debit and density projection, that producer retained amount is not the
full day-over-day layer-store delta, and that window-wide signed cancellation
is temporally nonlocal.

`analysis-freeze-v2.json` retains the cohort, selectors, dates, numerical
thresholds, observation role, and claim limits. It completes all missing
operators, adds the daily-local signed diagnostic recommended by the
result-blind audit, makes the original seasonal signed quantity secondary,
labels cold-content and downstream-throughput measures correctly, and freezes
an explicit causal truth table. Results will be written only under the new
`target/snow_prepeak_mass_transition_physics_adjudication_v2/` namespace.

All v2 numerical screens retain the scaffold's `ASSUMED_FOR_EXECUTION` status.
Completing their operators and causal truth table did not promote them to
physical, calibration, validation, or acceptance thresholds.

This is prospective protocol repair, not result-driven threshold movement.

The first v2 tool invocation stopped in preflight before fixture copying or CLI
execution because the scaffold hashes used a filename-plus-digest
serialization, while the accepted predecessor and package tool use the
canonical path/digest/size manifest serialization. The empty output directory
was removed, the four v2 identities were corrected to the already-recorded
predecessor receipt values, and no result operator or numerical threshold
changed.

After the four runs completed, the first compatibility report was invalidated:
the package parser compared the whole nested hourly object and the schema label,
so it counted expected additive v4 hourly fields and `v3 -> v4` as changes. WAT
and HBP were already bit-identical. The parser was corrected to project both
top-level and hourly v4 additions before comparing every old operand. The exact
run outputs are identity-checked and reanalyzed in place; no model output,
scientific operator, threshold, or other result changed. Review then found two
additional evidence-only lineage defects: storage proof had consumed reported
daily accumulation instead of independently summed hourly snowfall, and the
top-level routed anti-alias had been compared to the exact incoming handoff
instead of Stage-3 routed liquid. Both were corrected, regression-tested, and
reanalyzed without changing model output or a result operator.

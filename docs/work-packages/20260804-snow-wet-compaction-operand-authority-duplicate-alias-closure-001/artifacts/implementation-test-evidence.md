# Implementation And Test Evidence

Status: complete / terminal validation pass

Evidence mode: Static + Ran

Production finalization now computes one private
`SnowCouplingOutcome::wet_compaction_liquid_input_m` from the positive parts of
the exact hourly applied CoE melt plus retained and released snow-contact rain.
The value is checked with the typed
`snow.wet_compaction_liquid_input_m` finite/nonnegative guard. Inactive snow
coupling initializes the private value to zero.

The density boundary now consumes only that private field. The former separate
`routed_melt_m` argument was removed, preventing the caller from reconstructing
the duplicate. The bulk and multilayer density algorithms, constants, cap,
and formulas were not changed.

The internal CoE snowbench boundary now publishes gross positive generated
melt. The offline density replay requires that column plus retained and
released rain, uses their exact sum, and retains state loss/routed melt as
diagnostic context only. Historical boundary files without the exact lineage
fail closed. The public v1 report summaries and the public replay CSV retain
their prior shapes; no runtime or public trace schema changed.

The materiality tool now validates its own evidence before publication. It
hard-fails operand/predecessor reconstruction above `1e-12 m`, upstream mass,
Stage-3 closure, or layer closure above `1e-9 m`, density-process closure above
`1e-9 kg m^-3`, and a zero changed-driver or changed-density cohort. Stage-3
disposition remains separately observational because density-mediated routing,
storage, and refreeze may legitimately change while incoming mass and closure
remain invariant.

Ran from `/home/workdir/openWEPP`:

```text
cargo fmt --all -- --check
.venv/bin/python docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/materialize_snowbird_development_cli.py --check
cargo nextest run --test snow_wet_compaction_operand_authority --no-fail-fast
cargo nextest run -p openwepp-runner -E 'test(boundary_requires_finite_nonnegative_authoritative_source_columns) | test(replay_uses_generated_melt_and_contact_rain_once)'
cargo nextest run -p openwepp-hillslope-orchestrator -E 'test(helper_sums_positive_melt_and_contact_rain_and_fails_closed)'
git diff --check
```

The exact-source focused suite passed `8/8` under Nextest run ID
`a895fdfc-1dce-4b4d-9d5f-79c9e0f5225a`; the offline behavioral tests passed
`2/2` under `3c69762e-7ff9-4df0-b247-a3b7b642ab95`; and the typed production
guard test passed under `9a4e08ab-13b2-4eff-8120-8608cfb718a6`. The
review-amendment regression first failed as intended under
`aaa24321-f2c8-438f-a47b-74f3c9cc7bb0`, then passed under
`3f8084ae-2d21-422a-843a-30d925a1515c`. Canonical four-site,
density/layer materiality, and full-workspace results are recorded in
`gate-results.md`. The independent terminal runner completed the quick retry
(`2181/2181`), frost (`358/358`), and Critical full workspace (`2270/2270`)
without failure.

# Contract-Test Implementation Evidence

Status: `PASS / exact amendment re-review PASS/PASS`.

Evidence mode: `Ran`.

Ran on the amended working tree:

- `cargo nextest run --test snow_stage3_turbulent_operator_reconciliation_contract --test snow_stage3_shadow_observability_contract` — PASS (`12/12`).

At exact clean `27e310a27d313235066a41acec8fb7d1d3442e10`, every integration
target containing the current `contract_version: 129` binding passed
(`164/164` across `38` binaries in `285.947 s`). The canonical Binding
Exposure validator also passed all `11` rows.

At final exact clean reviewed commit
`49e358c689163b1a701a2d504e5396fb67545733`, the focused contract selection
passed `16/16`, the Binding Exposure validator passed all `11` rows, and every
v129-bound integration target passed `164/164` across `38` binaries in
`282.882 s`.

The new contract test uses section-scoped assertions for the canonical
algorithm, alias/unit exception, tolerance rows, guard map, boundary
disposition, and Binding Exposure. It pins Q-positive projection/evolution
predicates, exact support and endpoint equations, turbulent termination and
post-melt N/A semantics, the `1,440` bound, albedo lineage, predecessor bridge,
exact projection identity, exclusive fallback/class precedence, and
no-authoritative-mutation posture.

After implementation re-review, the focused contract target passed `6/6` with
the exact `longwave_model_id` and `sublimation_model_id` selector-lineage field
bindings. The canonical Binding Exposure validator again passed all `11` rows.

The focused contract target also binds receipt/path site custody and the
absence of a fabricated row-level `site_id`; the contract/observability
selection passes `12/12` after this correction.

After the v1 pre-result execution rejection, the same focused selection passed
`12/12` with section-scoped bindings for inactive-day schema v6, all-24-hour
`operator_not_selected` status, zero sentinel identities, and exact disabled
inactive schema-v4 preservation. Binding Exposure remained `11/11`.

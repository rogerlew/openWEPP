# Obligation-to-Test Map

| Obligation | Direct evidence |
| --- | --- |
| stage-receipt field/error order | `stage_receipt_reconstruction_preserves_field_order_and_collections` |
| LIGHT, FINAL_LIGHT, unknown, missing audit | `public_stage_selection_preserves_light_final_and_rejection_shapes` |
| READY audit, LIGHT prefix, HEAVY suffix, resume | `ready_audited_heavy_preserves_import_and_final_receipt_bindings` |
| source mutation and checkout precedence | `source_mutation_and_checkout_precedence_are_preserved` |
| environment and missing inventory | `environment_and_missing_observed_inventory_are_fail_closed` |
| JUnit exact inventory | `junit_contract_dispatch_preserves_exact_inventory` |
| artifact reset and source paths | `real_artifact_reset_and_source_selection_cover_every_contract` |
| real/synthetic bytes and envelopes | `artifact_publication_selects_real_and_synthetic_sources` |
| per-output source mapping | `real_output_source_selection_covers_every_contract` |
| canonical directory rejection | `canonical_directory_accepts_only_directories` |

Static: this glue module carries no `SC-*` process-physics obligation. Its
applicable obligations are trust, ordering, schema, artifact, and fail-closed
execution behavior, all mapped above.

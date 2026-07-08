# Gate Results

Status: `EXECUTED-HOLD-DX5-PROMOTION-UNRATIFIED`
Evidence mode: Ran.

## Package Gates

| Gate | Status | Evidence |
|------|--------|----------|
| `git diff --check` | PASS | Final run after artifact closure; no output. |
| Markdown/doc lint for touched docs | PASS | Final scoped `markdown-doc lint` over package, `SC-OFEROUTE-001`, registry, work-package README, and roadmap: 0 errors, 0 warnings. |
| Exact runner-binary provenance | PASS | `artifacts/mesh-ladder-summary.md`; release binary `target/release/openwepp-cli-hill`, SHA256 `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`, git HEAD `abc69bdda5458dd5389902e61a7626213aaf54cb`. |
| Full selected-cohort active plain target-`dx` mesh ladder | PASS | 24/24 rungs completed; see `artifacts/mesh-ladder-summary.md` and `.json`. |
| H2637 synthetic stress ladder reported separately | PASS | H2637 all six rungs completed and is recorded as synthetic stress only. |
| Fine-reference adequacy against `dx1p25` | FAIL | `mn_corn_h4` `dx2p5` vs `dx1p25` shape max L1 `0.02018051100943346 > 0.0166667`; hold condition. |
| Candidate-vs-reference and baseline-vs-reference tables | PASS | `artifacts/mesh-ladder-summary.md`; candidate table is explicitly provisional because fine-reference adequacy is not fully closed. |
| Active-mode closure and rev-40/rev-41 clamp proof | PASS | Max cascade/seam/identity residual rel `5.102757679200487e-13` / `1.0677536270982221e-13` / `4.790129104777042e-13`; max supply reconstruction rel `8.378371644554163e-16`; max total clamp `3.5677495352737114e-12 m3`; max total clamp/source ratio `4.122656124588304e-18`. |
| Fixed `dt` caps across rungs | PASS | Diagnostic selector leaves `LANED_ACTIVE_SAMPLE_DT_S = 900` and `LANED_ACTIVE_MAX_DT_S = 300` unchanged; no `dt` selector or tuning used. |
| Case-4 focused machinery tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --lib case4 stage_flux_limiter_prevents_positive_clamp_injection final_tvd_scaling_preserves_positivity_and_total day_closure_enforces_cascade_and_identity_tolerances` -> 11 passed, 2 slow. |
| Focused Lane D / mesh-selector tests | PASS | `cargo nextest run -p openwepp-runner --lib mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx trace_selector_requires_explicit_one` -> 2 passed. |
| Contract/profile/BEI checks | PASS | `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` exited 0 with known `PASS-DEFERRED` consolidation note; `bash tools/release/check_sc_unit_compliance.sh --path ...` passed; `bash tools/release/check_unit_registry.sh` passed 21/21. |
| Protected default/off byte identity if production/default surfaces change | NOT RUN | No production/default mesh-policy change. |
| DC01/no-double-feed proof if production active routing changes | NOT RUN | No production active routing ownership change. |
| Routed-hydrograph-to-erosion consumer proof if production active routing changes | NOT RUN | No production routed-hydrograph consumer change; trace evidence is diagnostic. |
| `cargo fmt --check` | PASS | Ran after scaffold; no output. No Rust code changed in this package. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Ran after scaffold and docs/contract edits; no warnings. |
| `cargo nextest run --workspace --profile full` | PASS | 1420 tests run: 1420 passed, 3 skipped. |
| `cargo deny check` | PASS | advisories/bans/licenses/sources ok. |
| Source-level anti-evasion guards | NOT RUN | No required-case binding, cohort fixture, or external-authority suite posture changed. |
| `.rs` line-count governance | PASS | `artifacts/line-count-governance.md`; advisory scan only because no Rust code changed. |

## Outcome

The package closes held. The rev-41 solver lifts the WA fine-reference closure
failure, but the package cannot promote target-`dx` because the strict
fine-reference adequacy gate remains failed for `mn_corn_h4`, and the only
provisionally clean candidate table (`dx5`) has large unratified runtime cost.


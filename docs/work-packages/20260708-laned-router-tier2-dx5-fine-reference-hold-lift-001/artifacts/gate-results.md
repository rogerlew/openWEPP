# Gate Results

Status: `PASS-HOLD-PACKAGE`
Evidence mode: Ran.

## Execution Gates

| Gate | Result | Evidence |
|---|---|---|
| Exact release-runner provenance | PASS | `target/release/openwepp-cli-hill` SHA256 `8427529a166a880699fd06a6a39ed6f6bb23ca039a62dc670cd784ebce11e6f6`; build command `cargo build --release -p openwepp-runner --bins`; execution HEAD `25a9f52d2b6dba7d18188d2e0d0523c4f0d7f6a1`. |
| `mn_corn_h4` `dx0p625` opt-in active run | PASS | Package-local ladder ran seven rungs, including `dx0p625`, with `OPENWEPP_LANED_ACTIVE=1` and trace enabled; all rungs passed active closure. |
| Strict one-third adequacy gate | FAIL | Expected hold evidence: `dx1p25` vs `dx0p625` shape max L1 `0.02094494047849004 > 0.0166667`; see `fine-reference-adequacy.md`. |
| Shape-surface attribution precheck | PASS | No uniform-shape, degenerate-shape, positive-shape, tail-fold, or end-storage cliff; max row recorded at `sim_day_index=792`, `lane_index=1`. |
| Fixed `dt` caps across rungs | PASS | No `dt` selector exists; runs use retained `LANED_ACTIVE_SAMPLE_DT_S = 900` and `LANED_ACTIVE_MAX_DT_S = 300`. |

## Local Gates

| Gate | Result | Evidence |
|---|---|---|
| `git diff --check` | PASS | Ran, no output. |
| Markdown/doc lint | PASS | `markdown-doc lint --path docs/work-packages/20260708-laned-router-tier2-dx5-fine-reference-hold-lift-001 --path docs/work-packages/README.md --path docs/ROADMAP.md` -> `20 files validated, 0 errors, 0 warnings`. |
| `cargo fmt --check` | PASS | Ran, no output. |
| Focused runner mesh-selector tests | PASS | `cargo nextest run -p openwepp-runner --lib mesh_policy_parser_defaults_parses_and_rejects_invalid_target_dx trace_selector_requires_explicit_one` -> `2 passed`. |
| Focused orchestrator active mesh/closure tests | PASS | `cargo nextest run -p openwepp-hillslope-orchestrator --lib mesh_policy_resolves_fixed_target_floor_and_cap day_closure_enforces_cascade_and_identity_tolerances stage_flux_limiter_prevents_positive_clamp_injection final_tvd_scaling_preserves_positivity_and_total` -> `4 passed`. |

## Not Applicable

No `SC-*` contract, Rust production code, required-case binding, cohort fixture,
or external-authority suite posture changed. Therefore these gates were not
required for this hold package:

- contract/profile/BEI checks;
- protected default/off byte identity;
- DC01/no-double-feed proof;
- routed-hydrograph-to-erosion consumer proof beyond retained active trace
  evidence;
- `cargo clippy --workspace --all-targets -- -D warnings`;
- `cargo nextest run --workspace --profile full`;
- `cargo deny check`;
- source-level anti-evasion guards.

If a production mesh-policy flip is attempted in a successor package, those
gates become required again.

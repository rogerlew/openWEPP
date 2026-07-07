# Implementation

Status: EXECUTED. Evidence mode: Static + Ran.

## Summary

ADR-0037 was implemented as terminal removal from main:

- Archived final working hybrid state on branch
  `abandoned/hybrid-implicit-stepping` at
  `b1d5fd4410b700012d857ef4056000163e6aa6a0`.
- Deleted `SC-OFEROUTE-002.md` from main and marked its registry row
  `withdrawn`, pointing at the archive branch.
- Removed the live hybrid rows from `SC-OFEROUTE-001` and appended rev 37;
  historical revs 28-35 remain as provenance.
- Deleted `ofe_routing/implicit_recession.rs`.
- Removed hybrid composition, cooldown, deficit-carry composition helpers,
  hybrid D-val harnesses, direct-runtime selector/counters, runner manifest
  selector/counters, and implicit profile counters.
- Retained the active plain owner path, default/off behavior, shadow path,
  explicit route profile counters, and plain Case-4 oracle surface.

## Env-Var Posture

`OPENWEPP_LANED_ACTIVE_IMPLICIT` is no longer a selector. Any presence of the
variable now fails closed at runner startup with a typed
`HillslopeCliError::RuntimeSurfaceFailure` naming
`OPENWEPP_LANED_ACTIVE_IMPLICIT` and ADR-0037. It is checked before run-dir
setup, so a stale operator environment cannot silently select or ignore the
abandoned path.

Focused evidence:

- Ran: `cargo nextest run --test laned_shadow_h2637 abandoned_implicit_selector_env_fails_closed_at_startup --profile quick`
- Result: `1` test passed.

## Plain-Path Identity

The acceptance identity gate passed for all four selected-cohort members.
Pre-strip and post-strip release binaries differed by SHA256, but the
protected active-plain HBP and pass-parquet outputs were byte-identical:

- `h2637`: HBP
  `efd8c4255fbe976ecafb2bc89defb7bebd4e2054c9e65c89cd5353c4c31c3790`;
  pass parquet
  `21c54bf2b045c3fb2f79f39ca174e36a4d188b39f7064f2a75f1170be6bb1656`.
- `mn_corn_h4`: HBP
  `2f200c2ee0ad4f1b581d6d95aafe7bc2ff2ba5368afa96846263ea86b5243e18`;
  pass parquet
  `a364287f6fe348f609d25f341823781fdb6885607644eb531050ba1abbf5084f`.
- `n_idaho_forest_h1`: HBP
  `5ccf8c4edb1bacb862b92161171b35fb0790df263424a47647ca3df47e52a394`;
  pass parquet
  `be510725f5bd7bc92c2cb86742d352c7931e02831ef85853093d83e4e2726c77`.
- `wa_cascades_forest_h1`: HBP
  `3640fdf3b3c1d3bf61189a9430fe268143ce9db0e1996cb89e614cfd4d5c4f23`;
  pass parquet
  `bb3b2e03f3fbd5834eb65a06c59476aba8a383bb021e9f669be7825a342f9e63`.

Evidence:

- `artifacts/plain-identity-baseline.md`
- `artifacts/plain-identity-after.md`
- `artifacts/plain-identity-baseline.json`
- `artifacts/plain-identity-after.json`

## Line-Count Governance

Ran: `find crates/openwepp-hillslope-orchestrator/src crates/openwepp-runner/src tests/integration -name '*.rs' -print0 | xargs -0 wc -l | sort -nr | sed -n '1,40p'`

Result: no 3000+ non-exempt file. Existing 2000+ WARN files remain; this
package is a removal package and reduced the touched route code. The touched
runner builder file remains a pre-existing WARN at 2743 lines and should be
split by a future direct-publication builder decomposition package, not by
this terminal strip.

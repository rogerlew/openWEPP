# CLIM01 Disposition

Status: `HOLD`
Evidence mode: `Ran + Static`

Static:
- Legacy climate behavior reconstruction completed from baseline source.
- openWEPP detailed spec, consumer map, parser/architecture map, and coverage matrix completed.

Ran:
- Docs-only completeness and consistency gates passed (`artifacts/gate-results.md`).

## Disposition Summary

1. CLIM01 objectives 1-5 are complete at documentation/specification level.
2. Scope boundaries were enforced: continuous-daily and breakpoint covered; single-storm explicitly excluded.
3. Evidence traceability is complete and anchored to `/workdir/wepp-forest_260430_baseline`.
4. Promotion remains blocked by unresolved implementation-governance items:
- `CLIM-ARCH-GAP-001`: no climate parser-to-runtime adapter seam in orchestrator crates yet.
- `CLIM-ARCH-GAP-004`: no climate-specific parser-to-kernel seam integration tests yet.
5. Ratified CLIM01 decisions:
- `DECISION-CLIM01-001`: breakpoint cardinality target should match legacy capacity (`1500`), with parser/runtime alignment to follow.
- `DECISION-CLIM01-002`: do not carry forward the disabled dewpoint-based winter partition branch; keep active temperature-threshold path unless superseding authority is introduced.
- `DECISION-CLIM01-003`: support explicit `datver=0.0` override (`iclig=0`) and `datver>=4.0` (`iclig=1`), and hard-guard pre-4 nonzero inputs (`iclig=2`); this is not a `0.8` factor rule.
- `DECISION-CLIM01-004`: treat legacy zero-drain non-increasing-time handling as a bug and enforce strict breakpoint `dtime>0` guards (duplicate/decreasing times hard-fail).

## Final Verdict

`HOLD` (docs/spec package complete; implementation-governance closure unresolved).

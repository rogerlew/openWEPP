# Disposition

Status: complete.

Closure: `COMPLETE-10-3-2-CANOPY-STRATUM-BINDING-DISPOSITIONED`.

The package mapped Harvard and Marcell observed canopy strata to the current
modeled surfaces and closed with a binding constraint.

Current Harvard and Marcell fixtures are single static mixed-forest hillslopes
with runtime `cancov = 0.55`. They do not bind to the advertised observed
strata: Harvard `hemlock` / `hardwood` / `open`, or Marcell
`conifer` / `deciduous` / `open`.

The fixtures remain useful as mixed-hillslope diagnostics and planning anchors,
but they cannot carry canopy-stratum verdicts until paired variants or an
explicit aggregate observation binding exists.

No fixture inputs, production Rust code, science contracts, output schemas,
defaults, selectors, coefficients, radiation, albedo, density, melt, or frost
behavior changed.

## Revision 2026-06-26 — paired per-stratum hillslopes built

The closure condition above ("generate paired model variants for the observed
strata") is now **met for Marcell and partial for Harvard.** Six additional
within-watershed hillslopes were added to `tests/fixtures/cancov_forest/`, giving
model counterparts per stratum:

| Site | Observed stratum | New model binding | Status |
|---|---|---|---|
| Marcell | conifer | `marcell_conifer_mn` (topaz 52→p8, `forest`) | bound |
| Marcell | deciduous | `marcell_deciduous_mn` (topaz 73→p15) | bound |
| Marcell | open | `marcell_open_mn` (topaz 42→p6, `short grass`) | bound |
| Harvard | hardwood | `harvard_deciduous_ma` (topaz 41→p6) | bound |
| Harvard | open | `harvard_open_ma` (topaz 31→p3, `short grass`) | bound |
| Harvard | **hemlock** | — (no pure conifer hillslope in Harvard delineation) | **unbound** |

- **Marcell is now fully stratified** — its delineation contains `forest`/conifer,
  `deciduous`, `mixed`, and `short grass`/open hillslopes, so all three observed
  strata (conifer/deciduous/open) have model counterparts. The spatial binding
  constraint is resolved for Marcell.
- **Harvard is partial** — hardwood and open now bind, but the Harvard NLCD
  delineation produced no pure conifer/evergreen hillslope, so the **hemlock**
  stratum has no pure model counterpart (`harvard_mixed_ma` is the closest proxy).
  A clean hemlock pairing needs either a re-delineation that isolates an evergreen
  hillslope or an explicit hemlock↔mixed proxy rule.

**Revision 2026-06-26 — follow-on prerequisites resolved**

The two non-spatial preconditions named above are now resolved outside the
original package:

- **Per-day canopy:** SNOWDENSITY-10.3.1a routes direct-production daily
  `cancov` into snowbench and CoE replay through `canopy_series.csv`.
- **Observation ingest:** Harvard HF237 and Marcell RDS-2021-0016 normalized
  tables are installed under `tests/fixtures/cancov_forest/observations/`.

Current 10.3.3 readiness: Marcell conifer/deciduous/open and Harvard
open/hardwood are model-bound with observations installed. Harvard hemlock is
observation-installed but still unbound to a pure model hillslope; it must be
excluded, reported unbound, or explicitly proxy-scoped before verdict use.

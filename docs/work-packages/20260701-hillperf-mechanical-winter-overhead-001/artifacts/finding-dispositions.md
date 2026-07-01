# Finding Dispositions

Per-finding outcomes for the package's owned findings (F2, F3, F5, F6, F7,
F8). Evidence class marked per item.

## F7 — landed (commit `06f5dc0a`)

`diagnostic_count_to_f64` now a direct cast; bit-equivalence pinned by test.
Identity gate CLEAN.

## F2 — landed (this package)

Deferred-construction (`_with`) guard variants added
(`require_state_range_with`, `require_dynamic_state_range_with`,
`require_direct_typed_snow_value_with`); `require_state_range` (enum form)
defers internally, fixing every enum call site with no call-site change. All
eager guard-feeding symbol constructions across `frost.rs`, `frost_entry.rs`,
`runoff_reconciliation.rs`, `infiltration_reconciliation.rs` converted
(7 tmpadj/hour + 2 frost-layer sites; 8-symbol layer-state cluster;
6 + 9 inline-static dynamic-range sites; 20 + 32 + 5 snow-value funnel
sites). Dead eager variants deleted. Error values byte-identical (same
constructors, same strings, built in the failure branch). Static: verified
the seven tmpadj symbols and the layer-state cluster symbols have no
success-path consumer. Ran: identity gate CLEAN (51.57 s indicative — the
headline movement of the package so far, ~70 → ~52 s).

## F3 — naive form NON-VIABLE (verified); narrowed form deferred to re-profile

**Ran/Static verification (the package's like-for-like pre-step):** the two
`DirectWinterHourlyContext` argument sets are **not** field-identical —
the snow call (`00a_snow_frost_authority_impl.rs:359-369`) zeroes the four
frost-state fields and passes `self.snow_phase_model`, while the frost call
(`:170-180`) passes live frost state and hardcodes
`SnowPhasePartitionModel::LegacyRst`. Sharing the built forcing array would
change the phase partition each consumer sees — an output-changing edit, out
of scope for this package. Narrowed variant (factor the forcing builder into
a shared phase-independent core + per-model phase partition) is possible but
only worth doing if the exit re-profile still shows material weight in
`build_simimpl28_hourly_winter_forcing_typed`; deferred to that decision
point.

## F5 — in progress

Verified (Static): `fit_legacy_tmpcft_curve` inputs are exactly the twelve
monthly max/min normals, static per lane for the run — the fit hoists to the
frost typed authority (once per lane) and rides `DirectFrostThermalInputs`
(3 `Copy` f64 fields) into both solves. Only 3 construction sites ripple.
`DirectFrostThermalInputs::zero()` composes with the fit's zero-input early
return ({0,0,0}) so the zero case is value-identical.

## F6 — partially landed (this package), remainder assessed

Landed: erosion enabled-check hoisted above the inputs clone
(`erosion.rs`); ET trace event (6 Vec clones) and percolation trace event
(2 Vec clones) construction moved behind their `OnceLock` config checks.
Remainder assessed (Static): the span-report shadow-projection clones can
only be saved by dropping the projection fields from the span reports
(production discards them; only tests read them) — a report-shape refactor.
The `format!("{ratio_trunc:.0}").parse::<usize>()` round-trip in
`frost_fine_layer_count_for_layer` is an F7-sibling — candidate for the same
treatment with care for parse-failure domain equivalence. **Both deferred to
the exit re-profile decision point** (same rule as F3-narrowed): with the
endpoint already at ~52 s indicative after F2, each remaining item is
individually ≤1–2 s and only worth its churn if the re-profile still shows
the corresponding bins as material. This keeps the package converging on its
gate rather than grinding the tail.

## F8 — SKIPPED (manifest-entangled, low value)

**Static:** the manifest's `direct_runtime_counters` are a DIRECT_AUDIT
snapshot including `phase_view_constructions`; removing the executor's
counter-only phase-view loop would change a manifest value, violating the
package's manifest-allowlist gate for an estimated ≤0.5 s. Skipped unless
the exit re-profile shows the audit atomics as a material bin.

## Workspace-suite failure investigation (open)

The F2 pipeline's `cargo nextest` failed fast on one test outside the
orchestrator lib suite (145/145 pass there). `--no-fail-fast` run in
progress to name it; disposition (caused-by-this-package vs pre-existing on
the branch base) will be recorded here with evidence before the F2 commit.

### Suite-failure disposition (resolved): worktree environment, not this package

Ran: all eight failures were instant (≤0.2 s) Python-environment errors —
`required repo-local Python interpreter missing: .venv/bin/python` for seven
(the fresh worktree lacks the untracked `.venv`), and
`ModuleNotFoundError: pandas` inside the hphys0298 harness's transitive
import for the eighth (present in the main checkout's `.venv` but absent
from `tools/owcmp/requirements.lock.txt` — an untracked test dependency
worth a lock-file follow-up, out of this package's write set). After
`uv venv` + `uv pip sync tools/owcmp/requirements.lock.txt` + `pandas`, the
whole 40-test group passes. No physics/assertion failure occurred at any
point; no production code was changed in response.

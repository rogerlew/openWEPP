# Worker Handoff

Status: complete.

Closure: `COMPLETE-10-3-2-CANOPY-STRATUM-BINDING-DISPOSITIONED`.

Carry-forward constraints:

- Paired model variants now exist for Marcell conifer/deciduous/open and Harvard
  hardwood/open.
- Harvard HF237 and Marcell RDS-2021-0016 stratified observation tables are
  installed under `tests/fixtures/cancov_forest/observations/`.
- SNOWDENSITY-10.3.1a routes per-day direct-production `cancov` into snowbench
  and CoE replay.
- 10.3.3 gradient melt adjudication may use Marcell conifer/deciduous/open and
  Harvard hardwood/open as stratum-bound cells.
- Harvard hemlock remains observation-installed but unbound to a pure model
  hillslope; exclude it, report it as unbound, or explicitly proxy-scope it
  before verdict use.

No production code, contract, output schema, selector, constant, or production
behavior changed by the observation install.

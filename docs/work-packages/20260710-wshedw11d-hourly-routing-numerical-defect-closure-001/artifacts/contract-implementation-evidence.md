# Contract Implementation Evidence

Status: `PASS`

Evidence mode: `Static`

## Authority disposition

Canonical contracts were amended before contract tests or production code:

- `SC-ROUTE-001` revisions 55-56 add `INV-ROUTE-021` for the complete pinned
  `it=1..ntchr` recurrence, branch-specific `sinit`/`sfnl`/`chvol` hydraulic
  closure, and no-peak storage retention; `INV-ROUTE-022` binds typed
  Muskingum-Cunge numerical-admissibility rejection.
- `SC-SYSTEM-001` revisions 89-90 add `INV-SYSTEM-036`: event extensive
  outputs reduce terminal channel-oriented outlets through channel and
  impoundment dependencies, with impoundments declared sediment-authority
  boundaries rather than silently carrying unsupported ancestry.
- `SC-INFILE-CHANINP-001` revisions 0.1.3-0.1.4 make record 4 conditional on
  `nchnum > 0`, distinguish the two canonical applicability forms, and refresh
  lifecycle metadata; canonical `nchnum=0` closes after three records.

The amendments preserve the package's protected boundaries: no empirical
damping, coefficient repair, peak/volume clip, negative-storage clamp,
surrogate recurrence, or compatibility-default masking is authorized.

## Defect-to-mechanism and guard map

| Defect | Localized mechanism | Canonical disposition | Production enforcement |
|---|---|---|---|
| `W11D-ROUTE-STORAGE-001` | The route aliases the time-zero state to interval terminal 1, executes `ntchr-1` updates, reconstructs every branch from boundary mean area, and aliases the unrestricted flux residual to storage/volume. | Pinned `wshchr.for` executes `it=1..ntchr`, uses all `nseg+1` terminal spatial Manning areas for KW and the inlet/outlet mean for MC, carries storage, retains it on a zero-peak day, and closes `chvol = volint + sinit - sfnl`. | Separate time-zero state; exactly `ntchr` updates; KW spatial-area and MC boundary-area storage; zero-peak retention; finite/nonnegative available-volume guard under `WKERNEL-WS10-CHANNEL-E-003`. |
| `W11D-MC-PEAK-001` | W11C grids produce a materially negative `c3`, so the recurrence is non-monotone and amplifies passive peaks. | `INV-ROUTE-022`: coefficient sum/nonnegativity and source-aware maximum principle; reject the configured grid typed. | Static and dynamic coefficient validation and per-segment passive-bound guard, all before publication. |
| `W11D-EVENT-PUBLICATION-001` | `build_publication_frame` selects every dispatched channel on the non-interval lane and publishes `qsed_kg_s` as `kg`; downstream water duration can re-scale upstream rates; a channel/impoundment/channel chain can count both channels. | `INV-SYSTEM-036`: select terminal channel-oriented outlets through intervening impoundments; integrate each direct rate over its channel-ancestry sediment duration; do not imply sediment carry through an impoundment. | Terminal selector follows consumed impoundments, duration reconstruction uses channel-dependency hourly `S_h` ancestry or routed event duration, and the impoundment anti-alias test excludes pre-impoundment yield. |
| `W11D-CHANINP-ZERO-001` | `parse_required_branch` requires exactly four nonempty records before it parses `nchnum`. | Pinned `wshinp.for` implied-DO consumes no `ichnum` record when `nchnum=0`. | Parse three fixed records first, derive required record count from `nchnum`, and retain requested timestep without fallback. |

## Operand lineage

| Published/guarded operand | Units and basis | Direct source / authority | Authority class | Transformation / normalization | Anti-alias comparator |
|---|---|---|---|---|---|
| initial storage `sinit` | `m^3`, one reach at day start | prior same-channel `final_storage_m3`; fresh time-zero boundary `qin(0)/q1(0)`; pinned `wshchr.for` | authoritative | fresh inlet/outlet Manning-area mean times length; otherwise exact prior `sfnl` carry | not interval terminal 1 or `Σ(qin+qlat-q1)dt` |
| final storage `sfnl` | `m^3`, one reach at true day end | terminal spatial state, geometry, roughness, slope, length; pinned `wshchr.for:450-469,574-615` | authoritative | KW: mean Manning area over all `nseg+1` nodes; MC: terminal inlet/outlet area mean; zero peak: all available volume retained | not boundary mean on multi-segment KW, last interval start, or unrestricted flux residual |
| interval `storage_change_m3[]` | `m^3` per interval | `qin+qlat-q1`, interval grid | diagnostic only | rate residual times `dtchr` | may not replace `sinit/sfnl` |
| physical daily `volint` | `m^3`, local plus upstream daily extensive volumes | local `qlat` interval sum plus dependency `channel_outflow_m3`; pinned `volint/tmpvol` lineage | authoritative | local rates times `dtchr`; dependency volumes are already extensive | not dependency `Σq1 dt` |
| daily outlet `chvol` | `m^3`, one reach/day | `volint+sinit-sfnl`; pinned `wshchr.for` | authoritative | storage-closed available volume; only declared roundoff may become exact zero | not `Σq1 dt` when the grid retains water |
| MC `c1..c3` and output | dimensionless coefficients; `m^3 s^-1` output | pinned `K`, `X`, `c1..c4` recurrence plus HEC-HMS/NEH stability authority | authoritative when admissible | finite sum/monotonicity; source-aware maximum includes `qlat (m^2/s) * dx (m)` | no coefficient clamp, damping, fallback, or peak clip |
| public event runoff | `m^3`, independent terminal outlets | routed daily state for terminal channel-oriented IDs; `INV-SYSTEM-036` | authoritative | sum terminal extensive volumes after channel/impoundment dependency traversal | not serial internal-throughflow or pre-/post-impoundment sum |
| public event sediment | `kg`, independent terminal outlets | terminal `qsed (kg/s)` plus channel-dependency ancestry `S_h` timing or direct-event duration; `INV-SYSTEM-036` | authoritative within implemented channel sediment domain | rate times seconds, then terminal extensive sum; impoundments break sediment ancestry | not raw `qsed`, downstream water-duration scaling, or pre-impoundment sediment |
| zero-count `dtchr` | `s`, normalized daily routing grid | parsed record 1; pinned `wshinp.for` and `SC-INFILE-CHANINP-001` | authoritative | standard timestep normalization; `ntchr*dtchr=86400 s` | not 60-second compatibility default |

## Independent bounds

- Water: `external inflow + sinit = terminal volume + sfnl` within
  `TOL-ROUTE-009`; every extensive term is finite and nonnegative. A separate
  rectangular-Manning inversion reconstructs multi-segment KW `sfnl` and
  numerically rejects boundary-mean and flux-residual aliases.
- MC: an admissible passive convex recurrence cannot exceed the maximum of its
  three prior/current discharge sources plus the explicit lateral-source
  contribution; the coefficients sum to one within `TOL-ROUTE-010`.
- Serial publication: removing an internal channel from the terminal set must
  leave terminal yield unchanged while diagnostics retain both channel rows.
- Parser: a three-record `nchnum=0` payload and a four-record positive-count
  payload are distinguishable; real CLI output on zero-count `dtchr=600`
  matches the positive-count 600-second control and differs from the
  60-second default candidate; an extra record remains a strict closure error.

Static authority gate: `PASS`. Contract-derived tests are the next permitted
write phase; production code remains unchanged at this point.

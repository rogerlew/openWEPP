# Source Map

Status: `QUEUED`

Record the implementation source map before production edits.

Required rows:

- `gwcoeff.txt` parser output -> runtime groundwater authority state.
- Direct deep-percolation output -> groundwater recharge `D_i`.
- Groundwater state carry -> `S_i`, prior `Qb`, prior `Qs`.
- Generated `gwbfv`/`gwdsv` -> HBP/pass or watershed handoff.
- Lane D active ledger -> generated baseflow/deep seepage export totals.
- Active surface-router source series -> negative proof that `gwbfv`/`gwdsv`
  are absent.
- Publication/output metadata -> generated zero vs disabled/missing/legacy
  distinction.

# Disposition

Status: `COMPLETE`
Evidence mode: Static + Ran.

## Decision

`EXECUTED-COMPLETE-CONDITIONAL-DEFAULT-ACTIVATION`.

The package closes the operator-directed all/none/mixed selector policy:

- all scheduled lanes coefficient-complete -> active Lane D runs by default;
- no scheduled lanes coefficient-complete -> legacy/off path remains active;
- mixed coefficient authority -> fail closed;
- explicit active remains fail-closed on missing coefficients;
- explicit active-disable forces legacy/off and conflicts with explicit active.

## Review Finding Disposition

`review-codex.md` initially blocked closure. Disposition:

- High, lossy default eligibility: ACCEPTED/FIXED. The upstream schedule-slot
  authority builder now returns `None` only for truly all-absent slot
  authority. Incomplete slot authority and inconsistent per-slot authority fail
  closed during typed seed construction. The public default resolver then only
  sees complete, truly absent, or lane-mixed states.
- High, missing coefficient-complete default-active evidence: FIXED. Final
  ignored H2637 acceptance vector passed in `563.620s`; default-active and
  explicit-active HBP/parquet hashes match.
- Medium, missing byte-identity fallback evidence: FIXED/PARTLY STATIC.
  No-coefficient fallback and explicit-disable runs emit no `laned_active`
  block; hashes are recorded in `default-activation-evidence.md/json`. The
  protected-output identity statement is relative to the retained legacy/off
  path for each fixture because the package did not change legacy/off
  serialization.
- Medium, unit/contract-derived resolver tests: DISPOSITIONED. Direct private
  builder unit construction was not added. The package accepts public
  integration-level contract vectors instead because they exercise parser,
  typed projection, default resolver, active config attachment, and manifest
  publication together.

`verification-codex.md` initially blocked closure on missing gate artifacts and
un-dispositioned review findings. `gate-results.md` and this disposition close
those gaps.

No open findings remain at package disposition.

## Residual Risk

- Full watershed-facing HBP outlet re-pointing remains a named follow-on under
  SC-RUNOFFPART/SC-ROUTE authority.
- Active-mode sediment water-magnitude/process-physics adjudication remains
  out of scope.
- Runtime cost is inherited from rev 45 dx5 active mesh policy and is not
  changed here.

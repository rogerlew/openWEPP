# Test Plan

Status: `QUEUED`

Required contract-derived tests:

- `TV-GWBASEFLOW-001`: disabled branch with no coefficient defaults.
- `TV-GWBASEFLOW-002`: one-lane Lane D recurrence over at least two days.
- `TV-GWBASEFLOW-003`: domain failures and outflow-over-storage guard.
- `TV-GWBASEFLOW-004`: generated `gwbfv`/`gwdsv` read by the real consumer when
  export closure is claimed.
- `TV-GWBASEFLOW-005`: `bftharea` threshold branch or explicit hold if
  watershed topology area is outside envelope.
- `TV-GWBASEFLOW-006`: namespace separation among `gwbfv`, `gwdsv`, `latqcc`,
  and `cbase`.
- `TV-GWBASEFLOW-007`: Lane D active MOFE ledger excludes generated baseflow
  from the surface source series.
- `TV-GWBASEFLOW-008`: publication/output anti-alias distinguishes generated
  zero, disabled, missing, and legacy-carried states.

Focused runtime tests:

- `gwcoeff` parser handoff into runtime request.
- Lane D single-OFE active recurrence vector.
- Lane D MOFE aggregate recharge/storage/baseflow/deep-seepage vector.
- Protected legacy/off/default identity or unchanged fallback proof.
- Native malformed/mixed groundwater authority fail-closed proof.

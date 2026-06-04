# Review Agent A

Status: completed
Evidence mode: static

Static: Independent Rust code review by subagent Hume. Ran: no commands by
reviewer.

## Findings and Dispositions

- Finding A1, High: `wind` was incorrectly typed as `m s^-1` even though input
  and contract authority define it as direction. Disposition: accepted. Fix:
  production leaves `wind` scalar/follow-up, registry splits `wind_direction`
  from `vwind`, and tests assert `wind` remains scalar while `vwind` is typed.
- Finding A2, High: registry `TypedRequired` overclaimed watershed-prefixed
  aliases still published as scalar. Disposition: accepted. Fix: registry rows
  split `hs{ofe}_stmdur`, `hs{ofe}_stmstr`, `hs{ofe}_timem_{idx4}`, and
  `hs{ofe}_intsty_{idx4}` into follow-up rows.
- Finding A3, Medium: `winter.hourly.rad_mj_m2_{idx4}` was migrated in code
  but left `FollowUpRequired` in the registry. Disposition: accepted. Fix:
  promoted row to `TypedRequired` and added registry typed-posture assertions.
- Residual risk: dynamic series error labels are family-level (`timem_*`,
  `intsty_*`). Disposition: follow-up. Rationale: fail-closed typed errors are
  preserved; exact dynamic label ergonomics can be improved separately.

Ran: not-run by reviewer.

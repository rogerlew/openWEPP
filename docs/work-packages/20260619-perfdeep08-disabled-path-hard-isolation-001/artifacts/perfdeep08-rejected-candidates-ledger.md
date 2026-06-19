# PERFDEEP08 Rejected Candidates Ledger

Status: complete.
Evidence mode: Static/Ran.

## PERFDEEP08 Candidates

| Candidate | Result | Disposition |
|---|---:|---|
| Disabled diagnostic hook cache: cache PERFDEEP02 roundtrip env lookup and skip inactive indexed-shadow thread-local hooks. | `691.93 s`, RSS `229444 KB` | Rejected/reverted; slower than PERFDEEP07 `685.85 s` and above P0 `<= 676.67 s`. |
| Scheduler PERFDEEP flag-hoist micro-change. | Not timed | Reverted before timing; touched `scheduler.rs` (`3179` lines) and would require line-count closure before retention. |

## PERFDEEP07 Rejected Candidates Carried Forward

Do not repeat without new evidence:

- disabling the production indexed scheduler path entirely:
  `753.38 s`, `755.48 s`;
- rebuilding indexed writeback authority after every day:
  `1035.90 s`;
- propagating indexed surfaces through execution reports:
  `1054.71 s`;
- direct logical-map bypass whenever hot tables are absent:
  `688.54 s`.

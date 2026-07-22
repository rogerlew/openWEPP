# Coverage After

Ran: authoritative reviewed-isolation changed-head measurement passed at clean
exact `9c0db17d83247e138ccce08943ac9bfc83915021`. Production is `resume.rs` lines
1--899, before `#[cfg(test)]` at line 900.

- Tests: 127 discovered, 125 passed, zero failed, two intentional isolated-child
  ignores.
- Lines: 667/722, 92.3823%.
- Regions: 1,008/1,179, 85.4962%.
- Function floor: 29/29 at or above 75%; minimum 79.5918% in
  `collect_regular_files`.
- Both same-process checkpoint characterization tests passed in the
  instrumented traversal.
- Evidence: `/tmp/cqr-resume-isolation-Rm2zRX`.

| Artifact | SHA-256 |
| --- | --- |
| LCOV | `ebfcffaf585499af0a86a4b6cc2495bc662e8c7e968016c23c307c4767da06f3` |
| CRAP JSON | `9f571177251738e68496de858b5f4ab8f180903efc1fc7457c73430a1334eb82` |
| LLVM JSON | `80d90f68241c4c204d3ce90f0464fdf941eb2a1cfbed406a23769c6f48b8e27e` |
| run log | `e9f681f7a6f98baf93d326011d70386ef6473c020748ca2a90245604b70ddaf8` |
| function TSV | `c54cdf9b4729a3b294af093019a1ad1a569d197d1665f147ae272b6a268a7ad6` |

Ran: harness wall was 585.28 seconds and total traversal wall was 608.66
seconds. The 628 MB disposable target was removed after compact evidence export.

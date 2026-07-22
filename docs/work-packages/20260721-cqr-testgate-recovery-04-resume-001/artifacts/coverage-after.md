# Coverage After

Ran: authoritative changed-head measurement passed at clean exact
`47eb418d4700a009b01c7345962b36960329ab1a`. Production is `resume.rs` lines
1--899, before `#[cfg(test)]` at line 900.

- Tests: 127 discovered, 125 passed, zero failed, two intentional isolated-child
  ignores.
- Lines: 667/722, 92.3823%.
- Regions: 1,008/1,179, 85.4962%.
- Function floor: 29/29 at or above 75%; minimum 79.5918% in
  `collect_regular_files`.
- Evidence: `/tmp/cqr-resume-retry-I0dN4H`.

| Artifact | SHA-256 |
| --- | --- |
| LCOV | `aeb736f9d7081f69ff4066c686c8cda1b327ff5a3878dee019206694367e1a35` |
| CRAP JSON | `ee5faa926ed93fbf323dd0724d27516b581402546e81bb5378824bbf55db9f10` |
| LLVM JSON | `d2054ac76ec5a7923732762151f3c06f91159acad415cc1f5886775c477ceab1` |
| run log | `a3cae2ac5245cf28fde457690116e79b845fb175321dcae9b3a30275a0eda79d` |
| function TSV | `c54cdf9b4729a3b294af093019a1ad1a569d197d1665f147ae272b6a268a7ad6` |

Ran: harness wall was 496.31 seconds and total traversal wall was 519.85
seconds. The 628 MB disposable target was removed after compact evidence export.

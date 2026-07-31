# Gate Results

Status: `complete / pass`

Evidence mode: `Ran`

| Requirement | Result |
| --- | --- |
| Python compile | PASS |
| CQR self-test | PASS, 118.88 seconds |
| CQR handoff binary | PASS 4/4, 117.456 seconds |
| Split assurance blocker cases | PASS 14/14, 557.596 seconds |
| Complete assurance publication binary | PASS 37/37, 1811.491 seconds |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| Assurance `validate --all` | PASS; 3 DRAFT, 0 public |
| Human-review render | Reused current EB-03A evidence: 92/92 files; EB-03B changes no report input |
| Affected Markdown lint | PASS, 24 files over four invocations |
| `git diff --check` | PASS |
| Quick profile | PASS 2109/2109, 36 skipped, 55 slow, 2180.377 seconds |
| Frost profile | PASS 324/324, 1875 skipped, 1 slow, 544.130 seconds |
| Critical full profile | PASS 2158/2158, 29 skipped, 37 slow, 2452.436 seconds |

## Heavy Profile Identity

- Base HEAD: `53ff5854f2b870c742dc74998e3393e8512dbc59`.
- Terminal executable-tree digest over `Cargo.toml`, `Cargo.lock`,
  `.config/nextest.toml`, and all Rust/Python files below `crates/`, `tests/`,
  and `tools/`: `19b3776f00262a977f6df3387890cd9ee4b931cfc331d045ad129b751e4b51ec`.
- EB-03B executable diff digest:
  `f0a2a000478bd9b65c5f54a8f2a7c0c3d5c08080041489fffd4994acbcf4af5f`.
- Stable two-file patch ID:
  `641a861edf0e13e904c80adb0e58b606258d0b9a`.
- CQR file SHA-256:
  `1145d108fc6d3aa16b0ebcf0e52378db071ecbe6b7d98656efdb81b225a069c8`.
- Assurance publication test SHA-256:
  `a02cac56be1e66457e5c81c0f701b5a12d1b498f405af8d903077e8acd021fe6`.
- Quick run `ad35d09e-b7cd-4698-823c-27d7ee375230`;
  `/tmp/openwepp-eb03b-heavy/quick.log`.
- Frost run `f8669779-a747-447f-abb3-8364d9ab3e12`;
  `/tmp/openwepp-eb03b-heavy-resume/frost.log`.
- Full run `bd84eb5d-358d-45ac-961f-ee248e02a55e`;
  `/tmp/openwepp-eb03b-heavy-resume/full.log`.

The first runner's receipt wrapper recorded exit 1 after quick despite
Nextest's complete all-pass summary and no failed rows. Independent inspection
identified process-substitution/`PIPESTATUS` capture and summary-pattern bugs;
quick is adjudicated PASS from its authoritative Nextest header and summary.
Frost and full used corrected immediate status capture and returned exit 0.
Quick was not rerun.

Static terminal verification confirmed the current quick/frost/full inventories
remain exactly 2109/324/2158 and no tracked executable source is newer than the
accepted full log. The digest binds the unchanged executable tree exercised by
the terminal runs; later edits are disposition Markdown only.

No required gate is deferred, waived, filtered, or satisfied by a timeout
increase.

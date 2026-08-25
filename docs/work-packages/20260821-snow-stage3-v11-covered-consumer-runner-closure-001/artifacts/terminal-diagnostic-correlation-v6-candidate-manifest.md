# Terminal diagnostic correlation V6 candidate manifest

Status: `FROZEN FOR DUAL REVIEW / NO SOURCE AUTHORITY`

Base HEAD and origin/main: `6836e4cae6bab3a70767d64ab3e6a96e990745fe`.
Last qualified physical implementation: `43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

| Candidate member | SHA-256 |
|---|---|
| `terminal-diagnostic-correlation-authority-v6.md` | `6a555bda01d2a74b7be7908ffea9ab08b600885bc72194a55a5cba5e3e5dc275` |
| `terminal-diagnostic-correlation-v6-schema.json` | `83833348b37226baddc09728f2cb8dfaf0bb085ea393f681ae7b96ecf2f048de` |
| `terminal-v6-calculated-resolution-report.md` | `2977a1fee574e76bf2c61c3a4b4968217e67d458b378844cc0d8f3854438d888` |
| `terminal-v6-carrier-projection.md` | `3c1ba73d972df9d6dbfa79f26d24e2138f6eb23e57c40db9a520661d52d17452` |
| `terminal-v6-compiler-bindings.md` | `319dd2bffa2a58ebd8da6669a0d4059b019ac21709f5e6d925a501c40661a97b` |
| `terminal-v6-compiler-index-provenance.md` | `e967692463d722ea057a1b01383ae9803dda0c866dec8bcd0b194c27bcf736e5` |
| `terminal-v6-dto-graph.md` | `90b962caac46476bdddc05912254ff74931200cd392642c4942e36fcf9be4fe0` |
| `terminal-v6-evidence-sufficiency-matrix.md` | `2c5884735bf49c235e47822399deca60ba2cccfc34d1b9fb4cb576f33ea2c9c0` |
| `terminal-v6-owner-access-plan.md` | `7e527ca082d47af466d07c44acff348930d69c688d0e6e9bda6af8ddb3d48eff` |
| `terminal_v6_compiler_index_tool/Cargo.toml` | `6b82786c7de076341cc28285a46d8b0ae45f882de6e699a49ec3dafa49bf7d8b` |
| `terminal_v6_compiler_index_tool/Cargo.lock` | `aca6204187ca9788ef0c9e0d30894a005e52993bbb8569b9a8d9cc66805e5e72` |
| `terminal_v6_compiler_index_tool/guard.sh` | `ed7c08a8595437632faca74d2ce9e67f6e40677d0039d7f7f004fa09ca2e3a2d` |
| `terminal_v6_compiler_index_tool/src/main.rs` | `1d1446955bdfb49e0dc1a7ab88ae2077424393611514fad08339f97ea086b165` |

Ran before freeze:

- private rustdoc compiler-index regeneration: PASS;
- DTO closure/non-null nested schemas: 20/20 PASS;
- compiler source/output selector bindings: 60 PASS;
- carrier projection coverage: 13/13 PASS;
- exact stack snapshot source list: 23/23 plus three caller-local locations;
- unresolved/ambiguous/stale/private-access negative fixtures: 4/4 PASS;
- formatting and staged diff hygiene: PASS;
- V20/V21 historical guards: 5/5 PASS, nextest
  `d530b879-7004-475e-ae24-1223a20cc762`;
- byte-comparison V6 regeneration guard: PASS.

Any edit to a listed member invalidates both reviews. Reviewers must verify all
13 hashes and remain independent/read-only.

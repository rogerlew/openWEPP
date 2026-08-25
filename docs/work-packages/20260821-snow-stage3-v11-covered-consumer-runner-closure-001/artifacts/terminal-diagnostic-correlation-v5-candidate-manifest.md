# Terminal diagnostic correlation V5 candidate manifest

Status: `FROZEN FOR DUAL REVIEW / NO SOURCE AUTHORITY`

Base HEAD and origin/main: `fb676c63a55c6733caa9e653eaef2416d6c07109`.
Last qualified physical implementation: `43cc9bbea2fbf5fe6ab6596cee4162de75cef999`.

| Candidate member | SHA-256 |
|---|---|
| `terminal-diagnostic-correlation-authority-v5.md` | `155173f3e24fd4e028bdb02ea22145053fbc2079595961dd6fef12ad3fbc6855` |
| `terminal-diagnostic-correlation-v5-schema.json` | `8b9c98a42a3175c4c42ceb0e208c44f10457ba537d4c93f3279551eface829cb` |
| `terminal-v5-resolved-type-graph.md` | `5eb707197433e0b074f0cdd4b0ea1bff4480c193846a0ef734375e69477b5764` |
| `terminal-v5-source-projection-matrix.md` | `49de213cd06c5621796cd0413b95b1e258cb515c2b505da28f35c08b3c84ba9a` |
| `terminal-v5-generated-canonical-wire.md` | `868734d875ff458fbea198fd0b46a9430620d09d3320ef82788d8d79bcee9b44` |
| `terminal-v5-native-wire-verification.md` | `dfc1cc8f46dd733fc7a3cf22037f40738548e841f66b375e51a320894c8dae01` |
| `terminal-v5-owner-access-plan.md` | `12623ef1b00a7a75c2e13625daf8fb414099271b934816f436075e3d8ff9d704` |
| `terminal-v5-unresolved-stale-node-report.md` | `f69bef0a6dae1ff4f892062ea5f8e83e76c616849eeb04148aa683dd68984b97` |
| `terminal_v5_schema_tool/Cargo.toml` | `d884a993f3ec9476f31018e6bb5df08506ad877db507888b0f221cc83e2a916b` |
| `terminal_v5_schema_tool/Cargo.lock` | `69fbb11692c30b6d5fa6946925cc7977be9abed9aac3fe2fb0a6ee25c20fdeef` |
| `terminal_v5_schema_tool/guard.sh` | `acc47cd4a227fb79dc0e5b56b22ca930a5f99ff6a036e6188323c6bc0316836b` |
| `terminal_v5_schema_tool/src/main.rs` | `b001543aa0a4ee248f1532ee90003a4edab08c42f169a0ebe2d0938702e89a6d` |

Ran before freeze:

- `cargo fmt` for the private tool: PASS.
- `git diff --check`: PASS.
- V20/V21 historical structural guards: 5/5 PASS, nextest run
  `f12e6f92-cca2-4469-bdf5-7a92a1d2d245`.
- V5 generation and byte-comparison regeneration guard: PASS; nodes 10/10,
  carrier fields 13/13, native wires 0, unresolved/stale nodes 0.

Any edit to a listed member invalidates both reviews. Reviews are read-only and
must verify every listed hash before evaluating the candidate.

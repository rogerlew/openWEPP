# High-B Gate Results

Evidence class: **Ran**

- Source SHA: `a7737e3d4e3b27c11e60a545a4b65741860f5da5`.
- Date: `2026-07-12`.
- All commands ran from `/home/workdir/openWEPP` and exited `0`.

| Gate / exact command | Result | Elapsed | Max RSS |
| --- | --- | ---: | ---: |
| `cargo nextest run --workspace --profile quick` | PASS `1,812/1,812`; 28 skipped; 3 slow | `2:30.67` | 208,772 KB |
| `cargo nextest run --workspace --profile full` | PASS `1,889/1,889`; 3 skipped; 4 slow | `9:47.50` | 209,440 KB |
| `cargo fmt --check` | PASS | `0:02.45` | 69,492 KB |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | `0:02.15` | 155,584 KB |
| `cargo deny check` | PASS: advisories, bans, licenses and sources | `0:00.88` | 79,316 KB |

## Archived Provenance

| Artifact | SHA-256 |
| --- | --- |
| `quick.log` | `b6c29e75564f6f306e1572580f5b97fe2b2b4b0019feadce23bd7e0ea63a8dfa` |
| `quick.time` | `e39ff0422cf6f24432b11596d4b570106c6050b37216fbb9a64be508c260e242` |
| `full.log` | `84e6a3ef111a4b6244e08059e6475dfc9acc91a142661ac4b3b0f1ca199c51bb` |
| `full.time` | `eb0f8767cf98be69de4dd4d55aef324069c317903e839162e48e3a0d0a8bff88` |
| `fmt.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `fmt.time` | `b435d3611700d16893679d599ebf5afd2962f0b074c1a895332229d56d8ec0d5` |
| `clippy.log` | `cfd509d6a41ad8ceedc4516edd37d0b8a67953d235c00746f4cf9e8037e076b3` |
| `clippy.time` | `9891948be88301989f055742b58843918e5029dd89daa65c92907024e36f6376` |
| `deny.log` | `f1a0fca39d4280363937aabd77783990ea6480bd9ca257816de3b68fc8efa845` |
| `deny.time` | `291919d4a2d1de7a935e7e2393adad5aaaf61559ef16cdee1c86a25795ba7821` |

Logs and GNU-time records are archived under `hb/final/gates/`. This remains a
tranche-final gate record; terminal transition is not marked here.

# Baseline and recovery manifest

Status: `PHASE 0 COMPLETE — RECOVERY INVENTORY PRESERVED`

Evidence mode: `Static + Ran`

The package was started at the requested checkpoint without resetting the
checkout.

| Identity | Value |
|---|---|
| HEAD / branch | `0d56001edbbe55b119a5248249fcedefe2a3bbb0` / `main` |
| predecessor checkpoint | same commit; predecessor catalog HEAD `192b6e3561c9aeb3967230eb3b5eb5a889e05af682d94bb6b69d0fe508f8e115`; roadmap HEAD `b576c89130080482343e01e663bd4e101a0545709134869708f6b20dcacfe751` |
| toolchain | `rustc 1.95.0 (59807616e 2026-04-14)`, LLVM 21.1.8, Cargo 1.95.0 |
| lockfile | `Cargo.lock` SHA-256 `faad8355b0df159281c5855b0443a38f716b7d2a3fd4529cd2d8a14c8e33ac1c` |
| host | Linux `ow-dev-01`, kernel `7.0.0-29-generic`, `x86_64`, 16 logical CPUs; CPU model `13th Gen Intel Core i7-13620H`; observed scaling 40%, max 4.9 GHz |
| affinity | shell allowed `0-15`; timed fallback pins CPU 0 with `taskset -c 0` |
| build/runtime | `RUST_MIN_STACK=67108864`, `CARGO_PROFILE_RELEASE_LTO=false`, `nix develop`, release profile, one test thread; no `LD_PRELOAD`, allocator override, or relevant `OPENWEPP_*` treatment variable |
| scratch/cache | `/tmp/openwepp-roger-openWEPP-295c6e060aa9`, `/workdir/.cache/openwepp`; package outputs under this package's `artifacts/raw/` |

At intake the worktree had only the package-owned documentation edits plus the
pre-existing out-of-scope `tmp/` evidence:

```
M  docs/ROADMAP.md
M  docs/work-packages/README.md
?? docs/work-packages/20260904-stage3-authentic-coverage-memory-attribution-001/
?? tmp/
```

No tracked Rust path was dirty. Pre-edit snapshots for the two tracked docs are
the exact HEAD blobs: catalog SHA-256
`192b6e3561c9aeb3967230eb3b5eb5a889e05af682d94bb6b69d0fe508f8e115`, roadmap
SHA-256 `b576c89130080482343e01e663bd4e101a0545709134869708f6b20dcacfe751`.
Every new package path was `ABSENT_BEFORE` (no overwrite). No Rust path was
authorized or touched, so no source pre-edit snapshot is applicable.

The retained executable source identity is the predecessor's post-revert Rust
manifest `2813f6e8faabb9408bac5e59b9271626ff5bcdc7fe49ab6dda810d3a1c3eee0d`;
the cached release test binary used by the historical candidate is separately
identified as `f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`
and is not treated as retained source. Current fallback output records its
actual emitted binary identity rather than conflating these hashes.

The cached historical v31 test binary `f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`
was directly executed for both its focused component-replay parity test and
the authentic release profile; raw transcripts and exit statuses are preserved
under `artifacts/raw/`.

Recoverability search covered current tracked history, all reachable refs,
`git fsck --unreachable` objects, `/workdir/openWEPP-feasibility-bench`,
`/workdir/wepp-forest_260430_baseline`, predecessor patch/review artifacts, the
named raw logs, and the session-local recovery bundle `/tmp/v31_recover/` plus
`/tmp/v31.diff`, `/tmp/v31_owned_patches.json`, and
`/tmp/runner_v31_patch_calls.txt`. The bundle contains seven full deleted
revision-31 source files (solver evaluation/solve/tests/transaction/bridge,
runner qualification, and runner Cargo metadata), and the patch/session files
contain the added diagnostic modules. Their byte hashes are:

```
7428c99c0bcb3dfd7378a7e0b9e03a3967d01dd89786d65bee8b861a53fcbbf5  solver_covered_evaluation.rs
e943c257681e76cc4ed69616d64ce5a7dbd7a36ef5eeaf9926bee3c55917b629  solver_covered_solve.rs
d278873783ff3ed370673a984387519f1bbe3ee987b9d94469c07d6ef04f164b  solver_tests.rs
aa677f8470b25e240026c72925ac438692b658f0923604d50ec5c71cb743137e  transaction.rs
e6a360e5bdd9a9e471ec2e24309c2cec1fd7dcb23b6a1970abbe49b29c82f796  transaction_v3_bridge.rs
0876774cdefdfe6090dd35e0c8547aa5e9b2aa945a41f150df98af03270bb6ae  runner-Cargo.toml
52d3a3142e05d10bcb2fade640f19cd4b3afcf1622f28fa400879fd383a2a186  runner qualification
```

These are recoverable candidate bytes for diagnostic reconstruction, not proof
that the original process retained them. The bundle does not include every
later-added standalone test fragment as a full file; those are present as
session patch payloads and require ordered replay to reconstruct. Revision-61
implementation patches are likewise recoverable from the ordered primary
session payloads at
`/home/roger/.codex/sessions/2026/09/01/rollout-2026-09-01T13-05-03-01a05e93-405f-7912-9703-35f217b13b0a.jsonl`
(39 feed-forward patch calls scanned; 33 implementation calls in the relevant
window, 94,433 input bytes). The package preserved a 58-entry ordered
inventory, including 38 source patch attempts, and replayed those source
patches in an isolated exact-HEAD worktree: 22 applied and 16 failed. The
partial tree failed scoped `cargo check` with missing request fields/API
symbols; see `artifacts/raw/v61_patch_replay_status.log` and
`artifacts/raw/v61_isolated_cargo_check.log` for immutable transcripts. Its
archived three-run output and source manifest remain the historical measurement
record; this package did not reconstruct or execute that candidate. No
candidate was copied into the retained checkout or executed in this phase; any
future reconstruction must use a separate source/build tree and be labeled
nonhistorical.

Historical raw-log SHA-256 values are preserved separately:

| Raw log | SHA-256 |
|---|---|
| revision-61 release three-run | `7bb6f118dfcad3f0a46aa6efd6af356463316725b8784c13cd94ae2bb1a8cf0d` |
| component baseline three-run | `52292cb7c6ddcb7cae087cbd5ba8dbe3e8bb5265abc6d42446ece95f916ffe04` |
| component candidate three-run | `00ba46b4dfafd63958a187015bb1e065c5f5d928db80f8ed5b0f97c2a1e28e4e` |
| carrier static attribution | `933db97e7dc928431c2dfd8065ef929a4c4a3578905b6c0d29254e4bdcb14d61` |

## Recovery boundary

The exact historical v31 executable is available and was rerun directly. Its
ordinary release output stops at the aggregate assertion, but the bounded
debugger capture at the audit-return boundary decoded all 2,000 authentic
sweeps before aggregation and identified the boundary-aware one-sided-stencil
shortfall; the lifecycle capture separately distinguishes the authentic audit
from the forced-complete oracle. No source or binary was mutated. The remaining
source-recovery boundary is revision 61: its historical source-manifest digest
is a one-way identity without per-file bytes or Git-index custody, and the
recovered session payloads contain multiple correction cuts and a reversion
without an authenticated marker for the measured candidate cut. Automatic
exact-HEAD, a28, full ordered-patch, and pre-cleanup cut replays all fail before
an equivalent build. The reverse-cleanup hypothesis was also tested from a
clean exact-HEAD clone: twelve successful cleanup inversions and one
formatting-context failure left a malformed tree with ten Cargo errors because
the cleanup payloads omit placement context and some candidate-side request
edits. A manually reconciled detached tree compiles and completes the real
consumer, but its hash-identified diagnostic bytes cannot be promoted to the
opaque historical source cut; the historical candidate binary is also absent.
This is the package's precise evidence boundary: further revision-61 memory
attribution requires a newly supplied exact candidate source/binary cut or an
owner-authorized matched diagnostic build.

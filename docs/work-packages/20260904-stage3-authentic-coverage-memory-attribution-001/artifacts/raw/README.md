# Raw evidence directory

Raw command transcripts, profiler/mapping captures, schemas, executable
analysis scripts, hashes, and clearly labelled nonproduction candidate patches
belong here. Preserve raw bytes; do not normalize whitespace or rewrite a
failed run into a passing record.

The authorized `comparator_suite_runner` spawn was attempted first and failed
before execution with the fixed-role service error: `You've hit your usage
limit for GPT-5.3-Codex-Spark` (reported 2026-09-04). The package therefore
authorizes the parent to run the same bounded commands as a recorded fallback;
no comparator output is represented as comparator-generated evidence.

The first frozen-loop attempt was interrupted during observed B/block 1 after
the observer contract review identified timeout, environment, manifest-format,
PID-custody, and atomic-snapshot defects. Its partial outputs were preserved
under `aborted_frozen_attempt_20260904/` before the corrected loop was started;
they are not part of the frozen result set.

The first corrected R0 loop then failed closed at block 5/B with one
`field_gap`; its complete partial set is preserved under
`aborted_frozen_attempt_20260904_block5_field_gap/` with a SHA-256 manifest.
The R0 completed 12-block set (valid measurements but no persisted readiness
field) is preserved under
`frozen_control_attempt_without_persisted_readiness_20260904/`. Protocol R1
then persisted executable readiness and completed a fresh 12-block admitted
set. The admitted hashes are:

- `current_memory_results.jsonl`: `08aa5e5c1282a22e0eceffb40c6fa24ad880bed8b8dcc52d9aef3ab9720b08d0`
- `current_memory_samples.csv`: `0a7aa5cf742333de2b48c03b7ad5c8183b5e02621142ae3dd7dbe64cd3f7be18`
- `current_memory_manifest.jsonl`: `7ef7ba6c68330485bcd5b946ec4242f886285dfe4594a611d8145b97d766b320`
- `run_memory_observer.sh`: `70134949b52ca1a0024b6a53d2db1a8a0510e52723d6718fbe690677d1a2a4ec`

The isolated revision-31 recovery attempt was not promoted: the recovered
tracked six-file diff plus later recovered files cannot compile against the
supporting a28c55 tree. The exact compiler transcript is retained as
`v31_isolated_reconstruction_check.log` (SHA-256
`1ad6069cf355f524ddd2312bb62870ddccb0f7bf935c4102dd5d3831cedc7fa2`).
The ordered v31 session-patch replay is separately preserved as
`v31_patch_replay_status.log` (SHA-256
`090d46bf57159c6425d9b6afbe0fbb2ed171bedf550fd25a7c34d6dc86524f7d`): 153
patch payloads applied and 99 failed in the isolated reconstruction, with no
equivalent candidate build produced.

The v31 recovery inputs are now package-custodied as
`v31_ordered_patch_inventory.json` (SHA-256
`06febbf56f9dfed874283c73dd48c5048a8c9173070f0e9fffde0d801530337c`),
`v31_candidate_diff.patch` (SHA-256
`4159be9caa027395d02ce349b9733fb5abc64a5227628507818ded23ce91dcbb`), and
`runner_v31_patch_calls.txt` (SHA-256
`9127f039223856501378ce2c5914b1541a68e2286f1ea0d1737ba87184d69df6`).
The seven full recovered v31 files are package-custodied under
`raw/v31_recover/`; their hashes match the recovery manifest and are retained
as nonproduction candidate bytes only.

The historical revision-61 session payload inventory is preserved as
`v61_ordered_patch_inventory.json` (SHA-256
`2fa6aaca6c4eb4860deabe8a2da21f87822a6bf5fd1782482cd0e6b0772cfe8c`). An
isolated exact-HEAD replay applied 22 of 38 source patches and failed 16;
`v61_patch_replay_status.log` records the ordered outcomes (SHA-256
`d965ca6fa82557d4e5a9e17ffa6ba36585310c6071474b39f090032de39da47c`). The
partial tree then failed the scoped orchestrator/runner `cargo check` with
missing request fields/API symbols; transcript
`v61_isolated_cargo_check.log` is retained (SHA-256
`7fc04ecc596a2d4c4a3120bc808ff123efd95b7c0657fa597c3133ab68aba21e`). No
reconstructed tree was executed as a candidate or copied into the retained
checkout.

The cached historical revision-31 test binary
`/workdir/.cache/openwepp/targets/openWEPP-295c6e060aa9/release/deps/openwepp_runner-ce7ba1c0f7527921`
was verified at SHA-256
`f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`. Its
focused real-runner component-replay/forced-complete parity test passed (1/1,
exit 0; `historical_v31_binary_component_replay_run.log`, SHA-256
`c1991da957217c16597b3d5a34ea29b3bd080cfae0ad360f23eabd9269f3da44`). The
same binary's authentic one-OFE release profile reproduced the historical
`component_replay_audit.rs:131` assertion panic (exit 101; no JSON sweep row;
`historical_v31_binary_release_profile_run.log`, SHA-256
`cb7c9b2814479fac776a2132a51fa3836577ed19e95cde460c2be9dc78dba332`).

The bounded debugger capture then stopped at the exact binary's audit-return
boundary before aggregation. `historical_v31_gdb_take_return.gdb` (SHA-256
`b0cf2e8b157536d894e475930428ca31dbc6ca80fc5c16e544c5806900235eb8`) and its
raw transcript (SHA-256
`1aa29a99c6dc993906fea9fcc2883ef939ba99f05a4fcd67d94297e346425877`) scan all
2,000 authentic N=2/S=6 sweeps. They reconcile to 200 rows at
`48/14/10/24` (iteration 0/1 for maps 0..99) and 1,800 rows at
`54/14/16/24` (600 iteration-0/1 rows for the other 300 maps plus 1,200
iteration-2/3/4 rows across all maps), with zero rows at the required
`58/14/16/28`; the compact decoded record is
`historical_v31_gdb_take_return_summary.txt` (SHA-256
`73c0d88bfa476592ed53826a6a133f5961eb28508eecb7144edb5557a95cf366`). A
lifecycle-count transcript
(`historical_v31_gdb_lifecycle_counts.log`, SHA-256
`07bf092e930de5c083b11ceb4178e3e480505a81b24a4f43e4b963db84710c76`) confirms
that the authentic audit is take 1 and the forced-complete oracle is a distinct
take 2. No assertion bypass or binary mutation was used.

The independent offline paired reconstruction is persisted as
`memory_reconstruction_summary.json` (SHA-256
`7aa1e6b5a1d6d4bffe41e55e72c57ce242070782d3c942de403c59a05d632f46`) so the
reported medians, pair signs, counts, and closure values are machine-readable.

The retained production-source replay-symbol search and canonical solver entry
are preserved in `retained_replay_static_search.log` (SHA-256
`15131b10fc1def656d95fec50c6d88db4ad644e994c38989d940d9520aab6e1a`). The
search returned no retained
component-replay symbol before the canonical `solve_covered_column` entry.

To avoid mixing the candidate with its later cleanup, a cut-aware replay of
session patch indexes 18–40 (before cleanup patch 41) was also attempted in a
second detached worktree. Fifteen of 23 payloads applied and eight failed;
the scoped compile failed with seven missing symbols/fields. The ordered
transcript is `v61_candidate_cut_replay_status.log` (SHA-256
`35ede123381836d3cf75cedad1ea683d2ccf1d5c4e7e2ecee7d27cc75d10ab40`) and the
compiler transcript is `v61_candidate_cut_cargo_check.log` (SHA-256
`5198b7c6b598b71a581c0d31ed34bb269b8411e49b9cbebabad1021439978917`).

The failed v61 contexts were then reconciled manually in the detached cut
only. `v61_manual_reconciliation_feasibility.log` (SHA-256
`b1d62f645de1e7268263a0d316b283c74b29d94fa3733f13d295b941b6b1ea55`) records the resulting `cargo check --tests`
pass. The documented changed-Rust manifest recipe found 13 paths and produced
`19a07121e6ecdfd8e1b0bd559404bcc69554943ef5c1d68b5d53d91534623421`; the ordered
13-row stream is preserved in `v61_manual_candidate_changed_rust_hashes.txt`
(SHA-256 `229b17c644b1e54dbbb9e836752f5f499bb7b6477171d126339dbf879a392348`); an earlier
undocumented canonical computation recorded `2fc5e8cafdde5520dfaca4c074a8d1d0bc3f60548c44b9abb6817bb981e6eb7f`.
Neither is comparable to the historical `650f6713…` digest without the
historical Git-index/per-file manifest and candidate binary, so the repaired
tree cannot be promoted to a matched candidate or used for causal memory
attribution. The one-day real-consumer probe nevertheless completed with exit
0 and exact `200` feed-forward calls, `48/56/20/32/4` counters, and closure;
its extracted `rss_kib=58,440` and `run_wall_us=4,028,455` are recorded in
`v61_manual_reconciled_release_probe.log` (SHA-256
`dbced82c9d15ad63faa652e2bdf609b90a9a7d241b02fea845e5b634fb9955c1`). These
values are diagnostic-only because the source/index custody is not historical.

Verifier B also tested the remaining inverse-cleanup hypothesis from a clean
detached `0d56001ed` clone. The original session shows 13 cleanup calls
succeeded and three were rejected; reversing only the successful calls in
strict descending order yielded 12 applications and one formatting-context
failure, then `cargo check -p openwepp-runner --tests` failed with 10 concrete
source errors. The cleanup hunks omit both unambiguous placement context and
some candidate-side request edits, so they are not a complete reversible
exact-source snapshot. The command/outcome record is
`v61_reverse_cleanup_reconstruction_verification_b.log` (SHA-256
`f79dcd05571582bec7695020d6f845de6b687c257bfb58b3dd20437a13311a3c`).

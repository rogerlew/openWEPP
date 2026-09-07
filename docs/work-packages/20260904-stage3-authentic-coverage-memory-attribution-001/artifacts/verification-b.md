# Independent terminal verification B

Status: **PASS — APPROVE TERMINAL HOLD / NO SUCCESSOR**

Evidence mode: `Static + Ran`

Verification cut: SHA-256
`7fe4c52a8f73d8264e0494be449d846f6a5502915025582446b586593c375efa`
over the lexically ordered `sha256sum` stream for the 171 package files other
than `review-a.md`, `review-b.md`, `verification-a.md`, and
`verification-b.md`, using repository-root-relative paths. Independent reviews
A and B both return PASS on this exact evidence cut; their current artifact
hashes are respectively
`83ae6156437f4b6162597110b57528093c9bd72c78048a2cbfd3e1dc29e6ad91`
and `186b91087fb49286b9b92303691e35d65ba0269eee62ef215379e96de86eab0e`.

## Findings

No blocking or non-blocking evidence finding remains on this cut.

## Verdict

**PASS — the terminal evidence HOLD is legitimate.** The exact historical
revision-31 runtime discriminator is directly observed, and every credible
bounded route retained by the package for authenticating the historical
revision-61 treatment has been exercised. The result is **HOLD**, not GO: the
failed revision-31 release assertion remains failed, neither rejected
mechanism is restored or qualified, no causal revision-61 memory attribution
is claimed, and no successor is authorized.

This is not an implementation-impossibility claim. A coherent nonhistorical
revision-61 diagnostic candidate was built and run. The terminal boundary is
historical identity: the retained evidence cannot authenticate that manually
reconciled tree as the source and executable used for the old measurements.

## Revision-31 limiting predicate

The exact historical executable hashes to
`f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`.
Its focused real-runner replay/parity test passed, while its authentic one-OFE
release test exited 101 at `component_replay_audit.rs:131` before JSON. The
debugger capture stops at the audit-return boundary before aggregation and
does not mutate the binary or bypass the assertion.

Independent parsing reconstructed all 2,000 contiguous N=2/S=6 rows:

| Iteration | `48/14/10/24` | `54/14/16/24` |
|---:|---:|---:|
| 0 | 100 | 300 |
| 1 | 100 | 300 |
| 2 | 0 | 400 |
| 3 | 0 | 400 |
| 4 | 0 | 400 |
| **Total** | **200** | **1,800** |

Every row has `collecting=1`, `completed=1`, and `failed=0`, and every row
satisfies `logical = identity + component + complete`. There is no logical-58,
complete-28, or exact `58/14/16/28` row. After the true occupancy and soil
conjuncts, `logical_probe_count == 58` is therefore the first false release
predicate.

The decoded representative stencil vectors and recovered enum/source mapping
give the source-real cause. The low-count vector has four beta-upper and six
sun/shade/wet-temperature-lower inward one-sided coordinates, hence
`14 + 10 + 24 = 48`. The high-count vector retains the four beta-upper
one-sided coordinates, hence `14 + 16 + 24 = 54`. These boundary-aware
one-sided stencils omit one side of the signed probe at closed-domain values.
The separate lifecycle capture records authentic begin/take 1, forced-oracle
begin/take 2, then aggregation, excluding collector loss, incomplete sweeps,
and oracle/audit conflation. The coverage matrix correctly distinguishes
`target_completion=false` from `all_captured_sweeps_completed=true`.

## Revision-61 historical identity and recovery

The historical record retains aggregate source digest
`650f67132aec95818228c1f3ed85db7310a7e66671dcb5f94cb18948ae257d41`
and binary digest
`e6b57efad9121915e95d45fa85323dfd29e9b5b8c8201f6356a2c6a10ea2df65`,
but not the aggregate's ordered path/hash stream, the corresponding source and
Git-index bytes, or the executable bytes. The historical binary was absent
from the 804 cached `openwepp_runner-*` files inspected by this verifier.

The corrected manual diagnostic custody contains 13 changed/untracked Rust
rows. Its file artifact hashes to
`229b17c644b1e54dbbb9e836752f5f499bb7b6477171d126339dbf879a392348`,
and its embedded ordered-stream digest is
`19a07121e6ecdfd8e1b0bd559404bcc69554943ef5c1d68b5d53d91534623421`.
The detached tree passes scoped `cargo check --tests`; diagnostic binary
`d71fcef567e02ad1a06d5268bfee20dd84c4e751785f5ec4d4798dbe463ce3bb`
passes the real one-OFE consumer with 200 calls, protected
`48/56/20/32/4` counters, and exact closure.

That successful run proves feasibility, not historical equivalence. The
`19a07121...` diagnostic digest and historical `650f6713...` digest cannot be
compared as source-identity values because the historical digest's input
domain and dirty/index state are missing. Their textual inequality is not used
as an equivalence discriminator. The earlier `2fc5e8ca...` value is correctly
retained only as an undocumented-domain computation. Manual placement choices
also cannot authenticate missing historical bytes or reproduce the missing
historical build/allocation identity.

The package has now exhausted the retained recovery routes:

- broad ordered replay applied 22 source patches and rejected 16, then failed
  scoped Cargo checking with six errors;
- the pre-cleanup candidate cut applied 15 of 23 payloads and rejected eight,
  then failed with seven errors;
- manual reconciliation produced the runnable but explicitly nonhistorical
  diagnostic tree;
- reverse cleanup from clean detached exact HEAD attempted the 13 originally
  successful cleanup payloads in order 56 down through 41. Twelve inverse
  applications succeeded, index 52 failed on formatting context, and the
  resulting 11-path tree (digest
  `475580610f492e4f92781cacae3660a9d9dad08dc9cd7c3ccc98307812516c42`)
  failed scoped Cargo checking with ten recorded errors.

The reverse-cleanup transcript hashes to
`f79dcd05571582bec7695020d6f845de6b687c257bfb58b3dd20437a13311a3c`.
It demonstrates that the cleanup deltas omit unambiguous placement context and
candidate-side request edits; they are not a complete reversible source
snapshot. An opaque one-way aggregate plus incomplete/ambiguous deltas cannot
recover the missing historical preimage. A future matched experiment on a
newly frozen diagnostic candidate would answer a different, nonhistorical
mechanism question and is not an unexhausted route to historical attribution.

## Memory and consumer semantics

The source/consumer map correctly follows the real runner through the
terminal/provider, strict V8 projection, native V3 adoption, canonical covered
solver, and committed publication. The retained-package evidence classifies
component replay as not applicable because that retained implementation had no
replay consumer hook or published replay counter; unrelated carrier-
finalization timing is not used as replay evidence.

Historical `rss_kib` is the first numeric `VmRSS` value read from
`/proc/self/status` during JSON construction before fixture cleanup. It is a
late point sample, not peak RSS, heap bytes, allocator return, post-cleanup
residency, or HWM. The admitted same-binary R1 observer control contains two
warmups and 12 balanced A/B blocks. All 24 admitted arms exit zero with exact
science, closure, and `48/4/44/20/32/4/56` counters.

Independent arithmetic reproduces endpoint medians A/B of 70,850/70,556 KiB,
exactly 4/12 positive B-A endpoint deltas, and paired endpoint median
`-298 KiB`. All 12 wall-time deltas are positive, with minimum `+48,930 us`
and paired median `+577,643.5 us`. The reported `75,664 KiB` statistic is the
median of each B arm's maximum successfully sampled kernel `VmHWM`; it is only
a lower bound on any later ultimate process-lifetime HWM. R1 therefore proves
observer perturbation, not a revision-61 treatment effect. The three old
baseline/candidate endpoint values are unpaired, overlapping, and lack
HWM/smaps/heap/cleanup custody, so causal memory attribution remains
unsupported.

## Current evidence identities

| Evidence | SHA-256 |
|---|---|
| `historical_v31_gdb_take_return.gdb` | `b0cf2e8b157536d894e475930428ca31dbc6ca80fc5c16e544c5806900235eb8` |
| `historical_v31_gdb_take_return.log` | `1aa29a99c6dc993906fea9fcc2883ef939ba99f05a4fcd67d94297e346425877` |
| `historical_v31_gdb_take_return_summary.txt` | `73c0d88bfa476592ed53826a6a133f5961eb28508eecb7144edb5557a95cf366` |
| `historical_v31_gdb_lifecycle_counts.log` | `07bf092e930de5c083b11ceb4178e3e480505a81b24a4f43e4b963db84710c76` |
| `v61_manual_candidate_changed_rust_hashes.txt` | `229b17c644b1e54dbbb9e836752f5f499bb7b6477171d126339dbf879a392348` |
| `v61_manual_reconciliation_feasibility.log` | `b1d62f645de1e7268263a0d316b283c74b29d94fa3733f13d295b941b6b1ea55` |
| `v61_manual_reconciled_release_probe.log` | `dbced82c9d15ad63faa652e2bdf609b90a9a7d241b02fea845e5b634fb9955c1` |
| `v61_reverse_cleanup_reconstruction_verification_b.log` | `f79dcd05571582bec7695020d6f845de6b687c257bfb58b3dd20437a13311a3c` |
| `current_memory_results.jsonl` | `08aa5e5c1282a22e0eceffb40c6fa24ad880bed8b8dcc52d9aef3ab9720b08d0` |
| `current_memory_samples.csv` | `0a7aa5cf742333de2b48c03b7ad5c8183b5e02621142ae3dd7dbe64cd3f7be18` |
| `current_memory_manifest.jsonl` | `7ef7ba6c68330485bcd5b946ec4242f886285dfe4594a611d8145b97d766b320` |
| `memory_reconstruction_summary.json` | `7aa1e6b5a1d6d4bffe41e55e72c57ce242070782d3c942de403c59a05d632f46` |

## Verification-B checks

- **Ran:** recomputed the 171-file evidence digest and every exact evidence
  hash above; confirmed both final reviews pin the same current digest and PASS.
- **Ran:** parsed all 2,000 GDB rows and independently reproduced the count-by-
  iteration table, target absence, lifecycle flags, and bucket identities.
- **Ran:** parsed 26 memory result rows and 1,611 samples; independently
  reproduced pair signs/medians, sampled-HWM wording, protected counters, and
  science/closure invariance.
- **Ran:** parsed all package JSON/JSONL, ran `bash -n` on the observer, and
  checked this artifact for whitespace errors.
- **Static:** checked the source/consumer map, retained-replay classification,
  exact release failure posture, historical-equivalence rationale, recovery
  transcripts, finding disposition, and final disposition. No failed/non-run
  gate is relabeled as passing. No retained Rust, Cargo, authority, tolerance,
  or acceptance-rule file was edited by this verifier. No heavy release was
  rerun.

Final recommendation: **PASS / APPROVE TERMINAL HOLD / NO GO / NO SUCCESSOR**.

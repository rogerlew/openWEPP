# Independent terminal verification A

Status: **PASS — APPROVE TERMINAL HOLD**

Evidence mode: `Static + Ran`

Verification cut: SHA-256
`7fe4c52a8f73d8264e0494be449d846f6a5502915025582446b586593c375efa`
over the lexically ordered `sha256sum` stream for the 171 package files other
than `review-a.md`, `review-b.md`, `verification-a.md`, and
`verification-b.md`. Both final reviews independently pin this same cut and
return PASS. Verifier A changed no retained Rust or artifact other than this
file and did not run a Cargo or heavy release workload for this final check.

## Findings

No blocking or nonblocking verification findings.

## Revision-31 runtime discriminator

**Verified closed.** The exact historical executable independently hashes to
`f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`.
The ordinary focused real-runner diagnostic passed 1/1, while the authentic
release profile exited 101 at the unchanged `58/14/16/28` assertion. The
debugger capture observes the audit Vec at the first take return, before
aggregation, without source/binary mutation or assertion bypass.

Independent parsing of the raw take-return transcript reconstructed all 2,000
contiguous N=2/S=6 rows, 400 maps, and five sweeps per map. Every row has
`collecting=1`, `completed=1`, `failed=0`, and satisfies
`logical = identity + component + complete`. The exact cross-tab is:

| Iteration | `48/14/10/24` | `54/14/16/24` |
|---:|---:|---:|
| 0 | 100 | 300 |
| 1 | 100 | 300 |
| 2 | 0 | 400 |
| 3 | 0 | 400 |
| 4 | 0 | 400 |
| **Total** | **200** | **1,800** |

No row has logical 58, complete 28, or tuple `58/14/16/28`. The corrected
matrix separately records `target_completion=false` and
`all_captured_sweeps_completed=true`, avoiding the earlier completion
ambiguity.

The representative 29-coordinate stencil vectors reconcile with recovered
source: the 48-count vector has ten inward one-sided coordinates (four beta
upper-bound and six sun/shade/wet-temperature lower-bound coordinates), while
the 54-count vector retains only the four beta one-sided coordinates. One
signed probe is omitted per one-sided coordinate, giving `14+10+24=48` and
`14+16+24=54`. With N=2 and S=6 already true, logical count 58 is the first
false assertion conjunct. The separate lifecycle log records authentic
begin/take 1, forced-complete-oracle begin/take 2, then aggregation; incomplete
sweeps, collector loss, and oracle conflation are excluded.

Exact v31 capture identities:

- GDB script: `b0cf2e8b157536d894e475930428ca31dbc6ca80fc5c16e544c5806900235eb8`
- take-return log: `1aa29a99c6dc993906fea9fcc2883ef939ba99f05a4fcd67d94297e346425877`
- corrected summary: `73c0d88bfa476592ed53826a6a133f5961eb28508eecb7144edb5557a95cf366`
- lifecycle log: `07bf092e930de5c083b11ceb4178e3e480505a81b24a4f43e4b963db84710c76`

The raw-memory decode uses offsets derived from package-custodied recovered
declarations because the binary lacks usable type debug information. Vec
lengths, ordinal/cardinality structure, flag bytes, count identities, and both
stencil vectors cross-check that layout. This is a bounded diagnostic
limitation, not a blocker to the observed first-false-predicate result.

## Revision-61 evidence boundary

**Verified as a legitimate terminal historical-evidence HOLD.** The corrected
pre-cleanup replay covers indexes 18–40 and reconciles to 15 PASS / 8 FAIL;
its partial Cargo check exits 101 with seven errors. Manual reconciliation then
produced a coherent detached candidate: the scoped check passed and the real
one-OFE consumer exited zero with 200 feed-forward calls, exact
`48/56/20/32/4` protected counters, and exact
source/outlet/storage/clamp closure. Thus implementation effort and ordinary
patch mechanics are not presented as the terminal boundary.

The package now custodies the exact 13-row changed/untracked-Rust hash stream:

- row-stream artifact SHA-256:
  `229b17c644b1e54dbbb9e836752f5f499bb7b6477171d126339dbf879a392348`;
- embedded ordered-stream digest:
  `19a07121e6ecdfd8e1b0bd559404bcc69554943ef5c1d68b5d53d91534623421`;
- reconciliation record:
  `b1d62f645de1e7268263a0d316b283c74b29d94fa3733f13d295b941b6b1ea55`;
- release-probe record:
  `dbced82c9d15ad63faa652e2bdf609b90a9a7d241b02fea845e5b634fb9955c1`;
- diagnostic binary:
  `d71fcef567e02ad1a06d5268bfee20dd84c4e751785f5ec4d4798dbe463ce3bb`.

The earlier `2fc5e8ca…e6eb7f` value is correctly labeled an undocumented
computation and is not used as an equivalence test. The historical record
preserves aggregate source digest `650f6713…57d41` and binary digest
`e6b57efa…ea2df65`, but not the historical dirty path/hash stream, Git-index
or per-file source bytes, or executable bytes. The diagnostic and historical
aggregates therefore have unprovably different domains and cannot authenticate
an equivalent historical treatment.

The remaining reverse-cleanup hypothesis was also executed. Its custodied log
hashes to
`f79dcd05571582bec7695020d6f845de6b687c257bfb58b3dd20437a13311a3c`.
Reversing the 13 originally successful cleanup calls applies 12 inversions and
fails one formatting-context match; the resulting 11-path tree has digest
`475580610f492e4f92781cacae3660a9d9dad08dc9cd7c3ccc98307812516c42`
and fails the scoped check with ten errors. The cleanup deltas omit placement
context and candidate-side request edits, so they are not a reversible exact
source snapshot. Manual choices can create a new diagnostic candidate, but
cannot recover the missing historical bytes or allocation lifetime. A matched
historical v61 causal memory treatment cannot be produced from retained
evidence; a future nonhistorical study would be a new, separately authorized
experiment.

## Memory, science, and retained-tree checks

Independent reconstruction of the admitted R1 files confirms 26 JSONL rows
(two warmups plus 12 balanced A/B blocks), 1,611 joined CSV samples, all exits
zero, all observed arms ready and complete, zero field gaps, and exact private
clean + private dirty identities. All admitted arms retain exact closure and
`48/4/44/20/32/4/56` counters.

Endpoint A/B medians are `70,850/70,556 KiB`; exactly 4/12 paired B−A deltas
are positive and the paired median is `-298 KiB`. All 12 paired wall-time
deltas are positive, with minimum `+48,930 us` and median `+577,643.5 us`.
The median maximum successfully sampled kernel VmHWM is `75,664 KiB`, correctly
treated as a lower bound on any later lifetime peak. Historical `rss_kib`
remains a late `/proc/self/status` VmRSS point sample before cleanup—not peak,
heap, allocator-return, or HWM evidence. R1 measures observer perturbation,
not a v61 treatment.

Exact R1 identities are results
`08aa5e5c1282a22e0eceffb40c6fa24ad880bed8b8dcc52d9aef3ab9720b08d0`,
samples `0a7aa5cf742333de2b48c03b7ad5c8183b5e02621142ae3dd7dbe64cd3f7be18`,
manifest `7ef7ba6c68330485bcd5b946ec4242f886285dfe4594a611d8145b97d766b320`,
and reconstruction summary
`7aa1e6b5a1d6d4bffe41e55e72c57ce242070782d3c942de403c59a05d632f46`.

**Ran:** raw hash verification, v31 row/cross-tab/lifecycle reconstruction,
R1 JSONL/CSV joins and arithmetic, JSON parsing, observer-script `bash -n`,
package-scoped `git diff --check`, and retained-tree Rust/Cargo status search.
All bounded checks passed. HEAD remains
`0d56001edbbe55b119a5248249fcedefe2a3bbb0`; no retained production Rust,
Cargo, authority, tolerance, or acceptance-rule edit is present. Review A and
review B hash to `83ae6156…6ad91` and `186b9108…ab0e` respectively and both
approve this exact evidence cut.

## Verdict

**PASS — APPROVE TERMINAL HOLD / NO GO / NO SUCCESSOR.** The v31 runtime
discriminator is directly closed. Revision-61 causal memory attribution is
unavailable at a precise historical-custody boundary after the safe retained
recovery alternatives were exercised. Neither rejected mechanism is restored
or qualified, and the package truthfully authorizes no successor.

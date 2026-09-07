# Independent terminal review B

Status: **PASS — APPROVE TERMINAL HOLD**

Evidence mode: `Static + Ran`

Review cut: SHA-256
`7fe4c52a8f73d8264e0494be449d846f6a5502915025582446b586593c375efa`
over the lexically ordered `sha256sum` stream for the 171 package files other
than `review-a.md`, `review-b.md`, `verification-a.md`, and
`verification-b.md`. Excluding the assurance outputs keeps this reviewed
evidence cut stable while those roles publish their fresh verdicts.

## Verdict

**PASS — the package's terminal HOLD is legitimate.** This verdict was
refreshed against the exact current package bytes after the final memory-
attribution and raw-custody corrections. The exact historical
revision-31 runtime discriminator is now directly observed and reconciled. The
revision-61 investigation has progressed beyond automatic patch failure,
manual-reconciliation feasibility, and reverse-cleanup recovery. It proves
that a coherent diagnostic candidate can compile and run, but also proves that
the retained materials cannot authenticate that candidate as the historical
measured source/binary. No historical v61 causal memory attribution follows.

The supported disposition is therefore **HOLD / insufficient historical
evidence / no successor authorization**, not GO and not a claim that the
mechanism itself cannot be implemented. No blocking Review-B finding remains
on this cut. Review A and both terminal verifications have also been refreshed,
return PASS, and approve the terminal HOLD; the package's required independent
assurance set is therefore complete.

## Finding disposition

1. **Prior v31 runtime-discriminator blocker — CLOSED.**

   The exact cached historical binary hashes to
   `f9386eec584664f9639da281c15796730240239cd43ad2f158f4fa6d27fbeeaf`.
   The debugger stops at
   `take_covered_component_dependency_replay_audit` return before aggregation,
   without changing the binary or bypassing the assertion. Independent parsing
   of `historical_v31_gdb_take_return.log` found all 2,000 authentic N=2/S=6
   sweep rows, all completed and not failed, with no target row.

2. **Prior v31 frequency/summary and completion ambiguity — CLOSED.**

   The corrected summary and coverage matrix distinguish sweep completion from
   target completion. Exactly 200 low-count rows are iteration 0/1 for maps
   0..99. The 1,800 high-count rows comprise 600 iteration-0/1 rows for the
   remaining 300 maps plus all 1,200 iteration-2/3/4 rows. The matrix records
   `target_completion=false` and `all_captured_sweeps_completed=true`.

3. **Prior v61 mechanical-recovery blocker — CLOSED AS A PRECISE EVIDENCE
   BOUNDARY.**

   Manual reconciliation produced a compiling and runnable diagnostic
   candidate, so generic patch difficulty is no longer offered as the HOLD
   reason. Its 13-row changed-Rust stream is now package-custodied, exactly
   reproducible, and explicitly nonhistorical. Reverse-cleanup reconstruction
   independently demonstrates why the retained cleanup payloads are not a
   complete authenticated predecessor snapshot.

4. **Prior v61 hash/equivalence wording blocker — CLOSED.**

   The package no longer treats `2fc5e8ca… != 650f6713…` as an equivalence
   test. It records `2fc5e8ca…` as an undocumented earlier computation,
   identifies the reproducible diagnostic stream as `19a07121…23421`, and
   explains that neither diagnostic aggregate is comparable to the historical
   aggregate without the historical dirty path/hash stream, Git-index/source
   bytes, and binary.

## Direct revision-31 reconstruction

`historical_v31_gdb_take_return.log` independently hashes to
`1aa29a99c6dc993906fea9fcc2883ef939ba99f05a4fcd67d94297e346425877`;
its corrected compact summary hashes to
`73c0d88bfa476592ed53826a6a133f5961eb28508eecb7144edb5557a95cf366`.
The independent row reconstruction is:

| Iteration | `48/14/10/24` | `54/14/16/24` |
|---:|---:|---:|
| 0 | 100 | 300 |
| 1 | 100 | 300 |
| 2 | 0 | 400 |
| 3 | 0 | 400 |
| 4 | 0 | 400 |
| **Total** | **200** | **1,800** |

Every row satisfies `logical = identity + component + complete`,
`collecting=1`, `completed=1`, and `failed=0`. No row has logical 58, complete
28, or tuple `58/14/16/28`.

The recovered enum maps stencil bytes `0/1/2/3` to
`Centered/InwardLower/InwardUpper/RejectedBeforeProbe`. The directly dumped
first vector has four beta-upper and six sun/shade/wet-lower one-sided
coordinates; dry-stem remains centered because 273.15 K is interior to its
200–350 K domain. It reconciles as `14 + 10 + 24 = 48`. The directly dumped
iteration-2 vector retains only the four beta-upper coordinates and reconciles
as `14 + 16 + 24 = 54`. Ground plus six soil coordinates supply the 14
identity-anchor probes; the beta probes are in the complete bucket. Thus the
first false conjunct after the true N=2/S=6 conditions is logical count 58,
with the one-sided stencil mechanism explaining the missing signed probes.

The two vectors are representative direct captures; the global distribution
comes from every decoded row. The binary lacks usable type debug information,
so the debugger uses offsets from package-custodied recovered declarations.
Vec lengths, ordinals, topology, flags, count partition identities, and both
29-byte vectors cross-check that bounded decoding. This limitation is stated
and does not overturn the diagnostic result.

The lifecycle log hashes to
`07bf092e930de5c083b11ceb4178e3e480505a81b24a4f43e4b963db84710c76`.
It records authentic begin/take 1, a distinct forced-complete-oracle begin/take
2, and aggregation only after both. Collector loss, incomplete sweeps, and
oracle/audit conflation are excluded.

## Revision-61 evidence boundary

The following corrected custody independently checks:

| Evidence | SHA-256 / result |
|---|---|
| Manual candidate 13-row hash stream | `229b17c644b1e54dbbb9e836752f5f499bb7b6477171d126339dbf879a392348`; embedded stream digest `19a07121e6ecdfd8e1b0bd559404bcc69554943ef5c1d68b5d53d91534623421` |
| Manual reconciliation feasibility | `b1d62f645de1e7268263a0d316b283c74b29d94fa3733f13d295b941b6b1ea55`; `cargo check --tests` passes |
| Manual real-consumer probe | `dbced82c9d15ad63faa652e2bdf609b90a9a7d241b02fea845e5b634fb9955c1`; exit 0, 200 calls, exact `48/56/20/32/4` counters and closure |
| Diagnostic binary | `d71fcef567e02ad1a06d5268bfee20dd84c4e751785f5ec4d4798dbe463ce3bb` |
| Reverse-cleanup verification | `f79dcd05571582bec7695020d6f845de6b687c257bfb58b3dd20437a13311a3c`; 12/13 successful cleanup inversions apply, but the reconstructed tree has ten compile errors and missing candidate-side edits/context |

The reverse-cleanup result is decisive about the retained route: cleanup
payloads preserve useful fragments but omit field placement context and some
candidate-side edits. Manual placement can create *a* working candidate, as
the successful diagnostic run shows, but those choices are not authenticated
historical bytes. The historical record preserves only aggregate source digest
`650f67132aec95818228c1f3ed85db7310a7e66671dcb5f94cb18948ae257d41`
and binary digest `e6b57efa…ea2df65`; the corresponding dirty path/hash stream,
Git-index/source bytes, and executable are absent.

This is a precise impossibility from the retained evidence: exact historical
equivalence and a matched historical v61 treatment cannot be established from
an opaque one-way aggregate plus incomplete/ambiguous deltas. It is not an
implementation-impossibility claim. A future nonhistorical mechanism study
could freeze the diagnostic candidate and run matched A/B controls, but it
would not retroactively identify the measured historical v61 source cut.

## Other reviewed surfaces

| Surface | Result |
|---|---|
| Authentic topology and seed bounds | **PASS** — N=2/S=6; beta=1.0 upper; sun/shade/wet=273.15 K lower; dry-stem interior. |
| Retained replay classification | **PASS** — replay implementation/counter absent; carrier-finalization telemetry is unrelated. |
| Historical `rss_kib` semantics | **PASS** — `/proc/self/status` `VmRSS` point sample before cleanup, not peak/HWM/heap/return evidence. |
| R1 observer control | **PASS** — balanced, fail-closed same-binary A/B; exact outputs/counters; observer perturbation only. |
| Memory arithmetic | **PASS** — 4/12 positive endpoint deltas, all 12 positive wall deltas, paired wall median `+577,643.5 us`. |
| Historical v61 causality | **HOLD / insufficient evidence** — no matched authenticated treatment exists and no causal claim is made. |
| Successor authorization | **PASS / none** — neither rejected mechanism is restored or qualified. |

## Reviewer-run evidence

- Ran an independent parser over all 2,000 GDB sweep rows and reconstructed the
  full count/iteration table, flags, target absence, and bucket arithmetic.
- Read the recovered stencil enum, trial-domain predicate, identity-anchor
  mapping, and probe-bucket increment sites; reconciled both dumped vectors.
- Recomputed the 13-row diagnostic source stream directly from the detached
  tree: byte-for-byte equal to the custodied rows and aggregate
  `19a07121…23421`.
- Recomputed the exact GDB, lifecycle, v61 feasibility, release-probe,
  source-stream, and reverse-cleanup hashes quoted above.
- Inspected the reverse-cleanup attempt's call selection, reverse order,
  context failure, ten compiler errors, and missing request-field edits.
- Ran scoped `git diff --check`: PASS. No retained Rust, authority, tolerance,
  acceptance rule, or production path was edited by this reviewer.

Final recommendation: **APPROVE TERMINAL HOLD / NO SUCCESSOR**. The remaining
v61 limitation is a precise historical-custody boundary after the available
recovery alternatives were exercised, not a generic patch failure.

# Verification agent A

Status: **PASS after reverification**
Evidence mode: Static and Ran as labeled

## Terminal verification — 2026-07-11

### Review-finding closure

| Finding | Verification | Result |
| --- | --- | --- |
| `A-FINAL-001` / `B-FINAL-002` A-H binding | Static: the map names tests for every applicable A-H family; the focused test now covers `NaN`, positive infinity, and negative infinity for both `dtchr` and `cbase`. Ran: parser suite passed 36/36. | PASS |
| `A-FINAL-002` consumer-claim narrowing | Static: package and implementation evidence now limit the real consumer claim to network-frame consumption of `nchnum_norm`, but canonical contract/spec wording remains contradictory; see `VA-FINAL-001`. | HOLD |
| `A-FINAL-003` cover-first/final provenance | Static + Ran: contemporaneous before JSON/LCOV/CRAP hashes match. The before source map identifies the monolithic `parse_required_branch` at line 395 with CC 41/CRAP 45.052, while terminal evidence identifies the decomposed function at line 586 with CC 15/CRAP 16.352. The before record also reports 35 passing correction tests and science-tier 93.123% line/96.970% region coverage. Because the defect-specific tests could not pass on scaffold behavior and the raw source maps distinguish the pre-/post-decomposition structures, this is sufficient direct sequencing evidence. Reconstructing an uncaptured source hash now would add inferred, not contemporaneous, evidence and is not required. Terminal source/test and raw output identities, sizes, timings, and commands are exact. | PASS |
| `A-FINAL-004` / `B-FINAL-003` terminal gates | Ran evidence logs match their recorded hashes: fmt, workspace clippy, 1,747/1,747 full nextest, deny, Markdown lint, and diff check passed. Final disposition/handoff appropriately remain pending dual verification. | PASS pending normal terminal bookkeeping |
| `B-FINAL-001` diagnostic priority | Static: `parse_conditional_ichnum` compares raw token count before ID parsing. The named strict/compat regression binds wrong arity plus invalid token to exact `CHN-E-002`. Ran: focused suite passed. | PASS |
| `B-FINAL-004` pinned citation | Static: artifact and canonical text now cite `wshinp.for:467-475`, explicitly locating read line 470 and clamps 473-475. | PASS |
| `B-FINAL-005` line counts | Static: all touched Rust files are listed; 1,018, 931, and 1,186 lines are below warning level. | PASS |

### Remaining findings

1. **High — VA-FINAL-001: canonical consumer claims are internally
   inconsistent.** `SC-INFILE-CHANINP-001` Section 2.2 and its additional
   obligations correctly say that `ichnum_norm` remains a parser projection
   and that no downstream ID-list consumer is claimed. However, the propagation
   map still names “runtime channel-ID output matching” as the downstream
   consumer for `ichnum_norm`, and the boundary map still calls the combined
   normalized count/list surface a “runtime-facing normalized export.” The
   input spec likewise calls `ichnum_norm` the “normalized consumer list.”
   Production search confirms only the parser reads/writes `ichnum_norm`; the
   network frame consumes only `nchnum_norm`. Reconcile these rows to the proved
   parser-projection boundary before closure. This is the accepted
   `A-FINAL-002` claim correction, not new downstream implementation scope.

2. **Medium — VA-FINAL-002: recorded canonical-document hashes are stale.**
   `contract-and-provenance.md` labels its hashes final, but current SHA-256 is
   `55d0e9985f2d5610d9ce9ca38e32eecbd8a19db9d4741320289ee1e265a76783`
   for the contract and
   `23299cba5478c3ba8d29b78161c8efa5d5c4aa38d18e51026e7ae272f5fcd9d4`
   for the spec, not the values recorded there. Refresh them after
   `VA-FINAL-001` is fixed. Also update the stale 35-test statement in
   `numeric-behavior-equivalence.md` to terminal 36/36 before disposition.

### Independent Ran evidence

- `cargo nextest run --test infile_chaninp_parser_contract`: PASS 36/36, run
  `b4d149ea-5576-4d83-ba9f-04c8aa4d7535`.
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract`: PASS
  19/19, run `f4315ba3-7168-4c6f-b4b4-e86d20d9ede2`.
- `markdown-doc lint --path
  docs/work-packages/20260711-dc-chaninp-raw-cardinality-001`: PASS, 29 files,
  zero errors/warnings.
- `git diff --check`: PASS.
- Static hashes: source
  `f7857a4cbd5a0bdb5f7ade1bf4e2d8871811988791f79dcb77fe5af33b59646d`;
  parser test
  `bb7475b308acd1364e7c8037fc4495a321ec1de8d46abb78a0fdf4c62f620c9e`.
- Raw terminal metrics/hashes independently match: 687/741 lines (92.713%),
  738/763 regions (96.723%), named-function floor 80%, and maximum
  deduplicated CRAP 16.352.

## Verification disposition

**HOLD.** Implementation behavior, A-H closure, cover-first sequencing, final
metrics, and heavy gates pass. Correct the remaining canonical claim
contradiction and stale evidence identities, then rerun scoped documentation
lint/diff check and reverify those two findings. No production or test change is
required by this HOLD.

## REVERIFICATION — 2026-07-11

Status: **PASS**

Evidence mode: Static and Ran as labeled.

### HOLD-finding closure

1. **`VA-FINAL-001` — PASS.** Static: canonical wording is now consistent.
   Contract Section 2.2 says the network frame consumes `nchnum_norm` and
   `ichnum_norm` remains a parser projection. The propagation map gives
   `ichnum_norm` phase `init` and states that no downstream consumer is proved.
   The boundary map separates runtime-facing normalized scalar/count fields
   from `normalized_ids`, which is explicitly parser-level. The additional
   obligation disclaims a downstream ID-list consumer. The input specification
   now uses “normalized parser projection,” identifies only the pinned legacy
   prefix-selection rule, and says no openWEPP downstream ID-list consumer is
   proved. Package and implementation evidence make the same narrowed claim.

2. **`VA-FINAL-002` — PASS.** Ran: current canonical hashes match the recorded
   final values: contract
   `da94093ff009be0e8ee618783c799d1ed70c0377b398e546fecd7e5c6c605be6`
   and spec
   `1d21069f186876faf19dfc4b2f300fdd17bad825cc54381d8c0c609b60c37ae2`.
   Static: numeric/behavior evidence now reports terminal 36 parser and 19
   consumer tests. The exact A-H map remains complete, with all six non-finite
   field/class vectors bound and G explicitly reviewed N/A.

### Reconstruction and terminal identity

Static + Ran: the isolated review-corrected monolith reconstruction is complete
and does not replace or mutate terminal source. The retained snapshot hash is
`63f700ff562fd9fb351ee9ce6cc95faf89db055d7981fb4561a5149b3f7f2dbd`
(28,745 bytes, 965 lines). Its copied focused test matches terminal hash
`bb7475b308acd1364e7c8037fc4495a321ec1de8d46abb78a0fdf4c62f620c9e`.
Independent rerun in `/tmp/openwepp-fq02-reconstruct` passed 36/36, nextest run
`c90f6a68-2b06-4d76-8a91-83458fa38719`.

The retained reconstruction evidence matches its manifest:

- LCOV: `f5d5b88ab52abd65125d7f2592a2d774520b23410d3c06688dab1fb90ca8c7db`,
  208,169 bytes.
- JSON: `b3dc21a7e33f5facaa7ada02f64ee0bfb4a379cc6cb3e2d06d064fd73980503d`,
  1,067,845 bytes.
- CRAP: `13cbb36851014914b942fe705caf16ba03b9ec35bfd05c54e9f70713e07ede46`,
  2,850,604 bytes.
- Timing files record zero exits at 0.38 seconds clean, 37.76 seconds LCOV,
  1.04 seconds JSON, and 1.16 seconds CRAP.
- Target reconstruction coverage is 652/706 lines (92.351%) and 708/733
  regions (96.589%); 32/34 functions executed. Monolithic
  `parse_required_branch` is CC 42, 83.957% covered, and CRAP 49.283. Thus the
  final review-corrected monolith independently clears the science-tier safety
  net and still requires decomposition.

Ran after the isolated check: workspace terminal source remains
`f7857a4cbd5a0bdb5f7ade1bf4e2d8871811988791f79dcb77fe5af33b59646d`
and terminal focused test remains
`bb7475b308acd1364e7c8037fc4495a321ec1de8d46abb78a0fdf4c62f620c9e`.
The previously verified terminal metrics remain 92.713% lines, 96.723%
regions, 80% named-function floor, and maximum deduplicated CRAP 16.352.

### Reverification checks

- Ran: isolated reconstruction focused test passed 36/36.
- Ran: package Markdown lint passed 29 files with zero errors/warnings.
- Ran: `git diff --check` passed.
- Static: no A/B or Verification A finding remains undispositioned or deferred
  in current package scope.

### Reverification disposition

**PASS.** Both Verification A HOLD findings are closed. Canonical claims match
the proved parser/count-consumer boundary, identities are exact, A-H is fully
bound, reconstruction proves the review-corrected monolith safety net, terminal
workspace identity is unchanged, and no current-scope blocker remains.

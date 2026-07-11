# Review agent B

Status: pre-implementation PASS; final review pending
Evidence mode: Static and Ran

Initial `GO-WITH-AMENDMENTS` findings: scope pass-through to conditional
record-4 `CHN-E-002`; assert exact payload in both modes; retain ordinary W003
default regression; ratify/test discarded-tail W005; clarify production plan.
All were accepted/fixed. Reviewer confirmed raw parse/closure then normalize/
take semantics, unchanged normalized network-frame consumer, and returned
`PREIMPLEMENTATION PASS` with no blocker.

## FINAL REVIEW — governance and contract

Verdict: **HOLD**.

Evidence mode: Static and Ran.

Ran:

- `cargo nextest run --test infile_chaninp_parser_contract` passed `35/35`
  (run ID `30e78769-debe-4371-971d-2d9e4e90d834`).
- `cargo nextest run --test wshedw5_typed_watershed_runtime_contract` passed
  `19/19` (run ID `01f200ea-88c1-4739-84e5-31e924b78e9a`).
- Pinned baseline HEAD is
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`. Static inspection confirms
  `wshinp.for` reads the raw implied-DO ID list before clamping `nchnum`, and
  `chnrt.for` consumes the post-clamp prefix.
- Recorded coverage/CRAP hashes match the six raw package files. Final target
  JSON reports `685/733` lines (`93.452%`) and `734/756` regions (`97.090%`);
  the deduplicated target CRAP maximum is `16.352`.

Static findings, ordered by severity:

1. **High — conditional record-4 diagnostic priority is not closed.**
   `parse_conditional_ichnum` parses every ID token before comparing the token
   count with non-negative raw `nchnum_input`. A wrong-arity row containing an
   invalid token therefore emits `CHN-E-001`; compatibility mode can collapse
   it to `CHN-W-003` default output. `INV-CHN-013`, `G-CHN-003`, and the
   Section 13 E/H obligations require every missing/wrong-arity conditional
   record-4 case to return exact non-collapsible `CHN-E-002` in both modes.
   Count tokens before parsing them and add strict/compat mixed-invalid-token +
   wrong-arity regressions that assert the complete `CHN-E-002` payload and no
   typed/defaulted output.
2. **High — A-H obligation closure is not evidenced.**
   `obligation-to-test-map.md` remains `active`, labels A/B/C/E/H `bound/red`,
   and leaves D/F partial; it names test categories rather than binding each
   obligation to test function names. In addition, the F obligation requires
   `NaN` and both infinities for every real token, while the current suite
   covers only `dtchr={NaN,-inf}` and `cbase={+inf}`. Complete the matrix for
   both `dtchr` and `cbase`, update the map to exact test-function bindings,
   and leave no partial/red row.
3. **High — required closure evidence is still `NOT RUN`.**
   `gate-results.md`, both verification artifacts, final disposition, and
   worker handoff remain queued/not-run. The package cannot claim terminal
   completion until focused fixes are remeasured, the delegated current-source
   formatting/clippy/full-nextest/deny loop passes, both verifiers pass, and
   every review finding is dispositioned and verified. This is current package
   scope and cannot be deferred.
4. **Medium — provenance line citations overstate their exact anchors.**
   `contract-and-provenance.md` says `wshinp.for:473-514` reads the raw
   implied-DO list, but the pinned file performs that read at line 470 and the
   clamp at lines 473-475. The spec repeats `473-514` for the read sequence.
   Use an exact range including lines 467-475 (or separately cite read and
   clamp lines) so Static evidence matches the source.
5. **Medium — line-count governance omits a touched Rust test.**
   `line-count-governance.md` records `chaninp.rs` and the parser test but not
   the changed consumer test
   `tests/integration/wshedw5_typed_watershed_runtime_contract.rs` (currently
   1,186 lines). Record and disposition every touched `.rs` file.

The core correction authority and claim limits otherwise pass Static review:
the contract distinguishes direct pinned read/clamp ordering from the
openWEPP exact-arity inference; source fields are retained before normalized
projection; `network_frame.rs` consumes `nchnum_norm`; the consumer test claims
only normalized-count projection and raw parser observability; no routing
physics or broader activation claim is made. The findings above are all
in-envelope and must be fixed rather than deferred or converted to a HOLD
boundary.

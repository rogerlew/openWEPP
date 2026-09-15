Static/Ran evidence — independent QA review of the corrected offline selector, decoder, focused-control receipt, sole selective-pass receipt, and bounded selected-output custody. I did not read or traverse the raw corpus, run Rust/build/runtime/probe commands, or rerun the selector.

## Findings

None.

The wrapper traversal is confined to direct elements of exactly one top-level `physical` array. Static inspection and the retained 10/10 focused controls cover root fields before/after `physical`, nested decoys, missing/duplicate/wrong-type wrapper forms, trailing/malformed input, exact integers/decimals, target-kind refusal, selected-member duplication, strict byte rejection, and the real `select -> decode_selected -> typed_bytes` path.

The sole authorized retry is source-bound: `select_pair.py` SHA-256 `c8804dfb0b59f7077e9933aee67facfe1701df293a3b4037627f7b8d24f08d84` completed exit 0 in 244.678 s under the declared 1 GiB address-space cap and 900 s timeout. Its COMPLETE receipt reconciles the immutable source hash `709dc499…`, 8,236,702,644 bytes, 123,093 physical rows, one physical array, EOF, and stable before/path/fd identity. It selects only ordinals 123091 and 123092 with their expected kinds; the 297,873-byte event stream is below the 2 MiB cap and independently hashes to `2525f332e1a3550ec436b3e3d4c7b03984a5abd94320aee46b1fee5565409a9e`.

The decoder invocation is separately retained with argv, working directory, timestamps, 1 GiB cap, decoder SHA-256 `85246cdf0f431aed59b03f2d3f0a03ffd1ac8d9c26f520fa039816d60723f551`, pre-decode selected-event hash, and exit 0. Its COMPLETE receipt retains all six required raw JSON payloads and their hashes. Independent file hashes match that receipt. Re-encoded decoded rows are explicitly labelled as non-lexical representations; raw typed payload bytes remain separate.

## Non-blocking debt/follow-ups

- This review accepts only offline extraction and bounded payload custody. It does not establish native-context, reader, witness, conservation, restart, or production acceptance.
- Preserve the failed prior selector/pass beside this successful retry, as already required by the authorization.

## Same-reviewer custody follow-up

Ran: an independent, bounded reconstruction of all 12,675 JSON event lines from the 297,873-byte selected stream. The audit did not import `select_pair.py` or `decode_pair.py`, did not access the raw corpus, rejected duplicate/misplaced map keys and mismatched container endings, retained exact integer/decimal types and member order, and required exactly the two selected ordinals.

The reconstructed records have 14 members at ordinal 123091 and 10 at ordinal 123092. Each is strictly equal, including type and object-member order, to its exported `.row.json`. For every required typed member (six total), the audit required exactly `{"Ok": [...]}`, rejected non-integer or out-of-range elements, rebuilt the byte array, and compared it byte-for-byte with the corresponding retained `.raw.json`; all six also match their recorded length and SHA-256. This closes the selected-member-preservation verification gap.

`operand-joins.json` is consistent with those retained raw-payload hashes and labels its equality/prefix results as bounded report evidence. It does not enlarge this QA acceptance beyond custody.

## QA pass

**PASS for the assigned wrapper traversal and selected-payload custody scope.** No maintainability, control-quality, or evidence-custody blocker remains in the reviewed cut.

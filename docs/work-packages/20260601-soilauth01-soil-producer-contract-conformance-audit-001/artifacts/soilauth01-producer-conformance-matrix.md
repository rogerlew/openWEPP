# SOILAUTH01 Producer Conformance Matrix

Status: complete  
Evidence mode: Static + Ran

## Scope
Datver-complete (`7778/9002/9003/9005`) `.sol` producer-contract conformance
audit across:
1. openWEPP producer contract (`soil-file.spec.md`),
2. openWEPP parser/runtime contract and parser implementation
   (`SC-INFILE-SOIL-001`, `soil.rs`),
3. canonical producer behavior (`wepppy`).

## Evidence
Static:
- `docs/specifications/wepp-input-files/specs/soil-file.spec.md:67-79,142-150`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md:45-46,58-63,206-215`
- `crates/openwepp-input-contract/src/parsers/soil.rs:366-403,479-550,610-629,632-647,793-944,1183-1216`
- `/workdir/wepppy/wepppy/wepp/soils/utils/wepp_soil_util.py:219-237,558-612`
- `/workdir/wepppy/wepppy/wepp/soils/utils/utils.py:100-107`

Ran:
- `cargo test --test infile_soil_parser_contract` (14 passed).

## Datver Family Conformance

| Surface | Contract / parser strict target | Parser compatibility behavior | Canonical `wepppy` producer behavior | Verdict |
| --- | --- | --- | --- | --- |
| `7778` OFE header arity (`slid texid nsl salb sat ki kr shcrit avke`) | 9 tokens with explicit `avke` | Quoted header may be 8 tokens; `avke := 0.0` normalization | Emits quoted header with 8 tokens (no `avke`) | mismatch `SA01-M002` |
| `9002/9003/9005` policy row placement | Strict sequence is header row then policy row | Compatibility may parse policy-first | Emits policy row first, then quoted OFE header | mismatch `SA01-M001` |
| `9002` policy row fields | `ksatadj luse stext ksatfac ksatrec` | Same arity with quote-tokenized `luse/stext` | Emits matching 5-field row | conforming |
| `9003` policy row fields | `ksatadj luse burn_code stext lkeff` | Same arity with quote-tokenized `luse/stext` | Emits matching 5-field row | conforming |
| `9005` policy row fields | `ksatadj luse burn_code stext texid_enum uksat lkeff` | Same arity with quote-tokenized `luse/stext` | Emits matching 7-field row | conforming |
| `7778` layer row arity | 11 fields | 11 fields | Emits 11 fields | conforming |
| `9002/9003/9005` layer row arity | 18 fields (11 base + 7 Rosetta append) | 18 fields | Emits 11 base + 7 Rosetta append | conforming |
| Restrictive layer placement/cardinality | Profile-level restrictive row expected by strict grammar | Compatibility accepts per-OFE restrictive rows when identical | Emits one restrictive row per OFE block | mismatch `SA01-M003` |
| Quoted text token style | Compatibility tokenizer handles single-quoted whitespace tokens | Single-quote tokenization only | Producer emits single quotes normally, but switches to double quotes when value contains apostrophe | mismatch `SA01-M004` |

## Mismatch Ledger (Priority + Ownership)

### `SA01-M001` (P0) - `9002/9003/9005` policy-first ordering
- Observed: canonical producer writes policy row before OFE header.
- Strict contract/parser target: header first, policy second.
- Current behavior: compatibility mode accepts policy-first; strict rejects.
- Owner: producer + contract/parser reconciliation.
- SOILAUTH02 closure: decide and codify canonical write-order authority, then
  align producer emission and contract/parser tests.

### `SA01-M002` (P0) - missing explicit `avke` in `7778/9002/9003/9005`
- Observed: canonical producer omits trailing `avke`; parser compatibility
  backfills `0.0` for quoted 8-token header.
- Producer contract target: emit `avke` explicitly.
- Owner: producer primary, contract/parser validation secondary.
- SOILAUTH02 closure: make canonical producer emit explicit `avke` and
  maintain compatibility tests for legacy inbound files.

### `SA01-M003` (P1) - restrictive-row authority drift
- Observed: canonical producer serializes restrictive row per OFE; parser strict
  expects footer-level row, compatibility normalizes identical per-OFE rows.
- Contract text currently scopes per-OFE compatibility language to legacy `7778`
  while parser implementation applies compatibility path to all datvers that
  require restrictive rows.
- Owner: contract + parser + producer policy reconciliation.
- SOILAUTH02 closure: explicitly ratify authoritative placement/cardinality and
  enforce with fixtures/tests.

### `SA01-M004` (P1) - quote-style compatibility gap
- Observed: canonical producer emits double-quoted tokens when values contain a
  single quote; parser compatibility tokenization is single-quote only.
- Owner: parser compatibility + producer text-token policy.
- SOILAUTH02 closure: either (a) extend parser tokenization to support
  canonical double-quote fallback, or (b) harden producer emission policy to
  disallow unsupported token shapes with typed failure.

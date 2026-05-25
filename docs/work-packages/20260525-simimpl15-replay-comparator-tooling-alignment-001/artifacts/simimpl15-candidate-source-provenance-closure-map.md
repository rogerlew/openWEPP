# simimpl15-candidate-source-provenance-closure-map

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- Closure target: `SIMIMPL13-TOOL-004` candidate-source provenance ambiguity.
- Policy implemented:
- `.dat` accepts `native-runtime-dat` or `conversion-derived-dat`.
- `.parquet` accepts `native-runtime-parquet`.
- Invalid source/format combinations hard-fail.
- Conversion-derived dat strict evidence is explicitly non-promotable for final Tier-A closeout.

## Ran
- Contract test `pl14s_contract_conformance_classifies_candidate_source_provenance` passed.
- Contract test `pl14r_contract_conformance_holds_when_strict_source_is_non_promotable` passed.
- Targeted SIMIMPL15 integration set passed.

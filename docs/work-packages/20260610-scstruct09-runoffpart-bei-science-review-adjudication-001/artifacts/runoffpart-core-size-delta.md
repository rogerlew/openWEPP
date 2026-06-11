# SCSTRUCT09 RUNOFFPART Core Size Delta

Evidence: Static
Date: 2026-06-11

## Measurement

Baseline is the SCSTRUCT08 post-triage working tree immediately before SCSTRUCT09
row adjudication. SCSTRUCT09 performed no narrative relocation.

| Measurement | Before SCSTRUCT09 | After SCSTRUCT09 | Delta |
|---|---:|---:|---:|
| Bytes (`wc -c`) | 99,626 | 100,180 | +554 |
| Whitespace tokens (`wc -w`) | 11,736 | 11,767 | +31 |
| Narrative relocated | 0 rows | 0 rows | 0 |

## Interpretation

SCSTRUCT09 increased the core slightly because the BEI row notes now carry exact
binding-ID mappings. This is intentional: all rows were live map-in-core
authority, so token reduction would have required moving active guard/vector
content out of the binding core, which the package forbids.

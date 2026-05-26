# EROD20 Symbolization Matrix

Status: complete
Evidence mode: static
Date: 2026-05-26

| prior literal | new constant | runtime usage |
|---|---|---|
| `1..=4` route case bounds | `EROD14_CASE_MIN`, `EROD14_CASE_MAX` | EROD14 case-domain guards |
| `5 + class_count*6` | `EROD14_BASE_UPDATE_FIELD_COUNT`, `EROD14_CLASS_UPDATE_FIELD_COUNT` | EROD14 update payload capacity sizing |
| `1.0e-8` | `EROD14_ATTENUATION_FLOOR` | EROD14 attenuation truncation guard |
| `+0.005` | `EROD14_ENRICHMENT_RATIO_OFFSET` | EROD14 enrichment ratio output |
| `1e-7`, `0.0001`, `0.01`, `0.001`, `10`, `1000`, `0.2` | `EROD19_*` constant family | EROD19 helper/solver thresholds, sentinel, iteration, fallback scaling |

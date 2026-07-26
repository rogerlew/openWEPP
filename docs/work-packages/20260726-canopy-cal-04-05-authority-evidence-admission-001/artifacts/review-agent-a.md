# Independent Review A

Evidence class: `Ran source/tool checks plus static authority review`

Status: `HOLD / FINDINGS RETURNED`

The reviewer independently inspected original objects and rebuilt the
extractor byte-identically. Checksums, site/study partition independence,
interval uncertainty, the prior-free design, and conservative CAL-05 treatment
passed.

Findings:

- `HIGH`: CAL-04 lacked exact native model-to-observation operators and a
  frozen aggregation rule. Several initially retained Harvard endpoints had no
  direct native equivalent.
- `HIGH`: HF003 EML says no 1992 fall campaign, while sparse raw values had
  entered the initial extraction.
- `MEDIUM`: distinguish a user-delivered operator request from a merely
  prepared package artifact.

Terminal verdict was `HOLD`. CAL-05 independently requires `HOLD` even after
CAL-04 corrections.

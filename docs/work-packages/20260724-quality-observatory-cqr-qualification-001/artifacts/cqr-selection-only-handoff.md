# CQR Selection-Only Handoff

Evidence class: Ran.

The authoritative forest1 CQR consumer classified quality evidence
`1969457920ea9560534dca066d19650b670d5da5d04886b09b9bd70693db185e`
as `CURRENT`. Intake used the complete candidate set with no selection limit
and recorded `collection_launched=false`.

The report contains two raw and two adjudicated CRAP rows, zero actionable
rows, and therefore an empty module ranking and selection. Selection review
remains required by contract; no module implementation or coverage
recollection was authorized.

A disposable exact publication copy was changed by one trailing byte in
`adjudicated-crap-report.json`. CQR rejected it as `INVALID` because the
complete control receipt no longer bound the supplied publication. The
tampered intake also records `collection_launched=false`. The disposable
fixture and its temporary checkout were removed after inspection.

Retained receipts:

- `cqr-quality-evidence-intake-current.json`
- `cqr-quality-evidence-intake-tampered.json`

The workstation-only exploratory intake was `STALE` because its toolchain
identity differs from forest1. It is not the authoritative consumer result and
did not launch collection.

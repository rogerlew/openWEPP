# Worker Handoff

Status: `complete`

## Next Authorized Priority

The roadmap's next program item is `ASSURE-02`, a documentation-only package
for the manuscript-first scientific-assurance architecture. It must preserve
the closure contract in `docs/ROADMAP.md`: real nonpublic manuscript before
schema, no v2 production/generated-content edits, evidence-led bounded non-snow
pilot selection, dual independent review, and user or named scientific-steward
acceptance before later assurance work.

Do not use the v1 SNOTEL candidate as openWEPP's snow/frost assessment and do
not begin WEPPcloud vendoring. Snow/frost remains the later flagship synthesis;
vendoring is the mandatory `ASSURE-08` pre-beta gate.

## Other Prospective Work

- `CANOPY-PHENOLOGY` remains promoted but queued. It becomes active only when
  an operator changes the current priority and authorizes its next contract-
  first increment.
- `CQR-NIGHTLY` remains recurring operator-triggered maintenance and does not
  automatically block `ASSURE-02`.
- `SC-SED-001#GAP-SED-008` remains an unpromoted, consumer-pulled per-class-
  hourly interchange concept. W11 channel-hourly work and `GAP-SED-009`/WB16
  are not open roadmap items.

## Maintenance Contract

When a roadmap item closes, remove it in the closing package and record the
outcome in `docs/work-packages/README.md`. Do not add completion narratives back
to the roadmap. Promote a backlog concept only with a state, owning queue, and
advancement trigger.

Local tooling note: the installed `wctl` wrapper currently fails before command
dispatch because its Python environment lacks `typer`. This package used the
installed `markdown-doc` binary directly and recorded the substitution; the
wrapper issue did not block validation.

No review finding is deferred or handed off.

# openWEPP End-User Documentation

This directory holds end-user-facing documentation, organized following the wepppy `usersum` convention so it can be vendored into wepppy's in-app documentation engine.

See wepppy's usersum specification at `wepppy/wepppy/weppcloud/routes/usersum/specification.md` for the vendor workflow and document-manifest conventions.

Authoring conventions for documents in this directory are normative in the
openWEPP repository at `docs/standards/usersum-authoring-style-guide.md`
(document shapes, audience/register, style rules, and the pre-landing
checklist). It is contributor documentation, deliberately not hyperlinked or
vendored here.

## Audiences

- Modelers running openWEPP via wepppy
- Modelers running openWEPP directly via the CLI binaries
- Scientific reviewers evaluating model outputs

## Status
Pre-alpha. Content will be authored as engine capability lands.

## Model science

- [Hillslope hydrology and sediment physics](hillslope-hydrology-and-sediment-physics.md) — how openWEPP represents overland flow routing, erosion, and baseflow, and how that relates to legacy WEPP.
- [Snow and frost in openWEPP](snow-frost-modeling-and-validation.md) — winter
  physics and scientific rationale, linked to its evaluation method and
  assurance dossier.

## Scientific assurance

- [Scientific assurance catalog](assurance/README.md) — what was evaluated,
  what the evidence currently supports, where it applies, what remains
  unknown, and a worksheet for application decisions.

## openWEPP CLI reference

- [Documentation agent](documentation-agent.md) — discover and route end-user CLI
  documentation updates.
- [CLI run index](cli-run-index.md) — quick map of all openWEPP CLI entrypoints and run patterns.
- [openwepp-cli-hill](openwepp-cli-hill.md) — run a single hillslope from run files.
- [openwepp-cli-watershed](openwepp-cli-watershed.md) — run a full watershed from run files.
- [openwepp-cli-totalwatsed3](openwepp-cli-totalwatsed3.md) — aggregate hillslope outputs into a total-watershed parquet.
- [open_wepp_runner](open_wepp_runner.md) — orchestrate CLI runs and release helper workflows.
- [openwepp-snowbench](openwepp-snowbench.md) — run snow benchmark/profiling commands.

## Standard handoff

- Use this standardized handoff target when routing CLI documentation work:
  - `handoff: usersum/documentation-agent`
  - `target_path: usersum/documentation-agent.md`

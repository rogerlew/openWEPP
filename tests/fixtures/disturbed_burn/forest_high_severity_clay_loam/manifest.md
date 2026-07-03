# Forest high-severity burn — clay loam (peakflow-artifact anchor)

Fixture `forest_high_severity_clay_loam` — the artifact-reproduction anchor for
the physically impossible legacy burned-forest peakflow that the
[disturbed-forest fidelity campaign](../../../../docs/planning/disturbed-forest-fidelity-strategy.md)
(WS-2 ksatadj re-port, WS-3 magnitude adjudication) must resolve.

| Field | Value |
|---|---|
| Source | `wepppy/tests/disturbed/disturbed_matrix0/runs/p4.*` (disturbed test matrix) |
| Cell | texture **clay loam** × veg **forest** × severity **high** → `wepp_id p4` (`H4`) |
| Management | `UnDisturbed/High_Severity_Fire.man` (100-yr expansion) |
| Soil | `Forest clay loam.sol` → 9002 disturbed (`forest high sev fire`, clay loam) |
| Slope | 201.68 m single OFE, ~43% avg grade (canonical steep forest hillslope) |
| Climate | McKenzie Bridge RS, OR — 100 yr, ~1194 mm/yr (`p4.cli`, CLIGEN) |

## Why this fixture exists — the artifact

Running this cell in legacy WEPP produces peak-discharge EVENT records up to
**380,150 m³/s** (mean per-event peak **3,128 m³/s** over 480 events;
`H4.pass.dat` field 13 `peakro`) against an **unburned forest baseline of
~0.008 m³/s**. A peak of 380,150 m³/s from a single ~201 m hillslope is
physically impossible — larger than the Amazon's discharge. This is the legacy
peak-runoff model **blowing up under burn hydrophobicity**, and it is the
`~1446.7 m³/s` forest-high cross-texture mean recorded in
`wepppy/tests/disturbed/analysis_results.md`.

**The directional law is real** (burn ↑ peak); **the magnitude is not** and is
NOT an openWEPP acceptance target (ADR-0011 "test a law, not a number").

## The three drivers, all in `p4.sol`

This single burned soil exercises all three campaign concerns at once (from its
9002 `Replacements` block):

- `ksatadj -> 1` — the forest saturated-conductivity model fires. **It is
  unimplemented in openWEPP's `direct_runtime` lane** (deleted 2026-06-30 with
  the symbol-map lane; `SC-SUBHYD-001` `INV-SUBHYD-032`). WS-2 re-ports it.
- `ksflag -> 0` — frost disabled. This is the **legacy lever** the campaign
  removes: openWEPP keeps **frost on (`ksflag=1`)** and augments `ksatadj` to be
  sensible on its own.
- `keffflag -> 1`, `lkeff -> 0.1` — the 9002 hydrophobicity burn-conductivity
  floor that drives the runoff/peak spike.

## As-built, deliberately (NOT the cancov `ksflag` flip)

`p4.sol` is preserved **exactly as built** with `ksflag = 0`. Unlike the
`cancov_forest` fixtures (which flip `# ksflag -> 0` to `1` so frost stays
active), this fixture keeps `ksflag = 0` **on purpose** — it is the
artifact-*reproducing* input, and flipping it would lose the anchor. The
openWEPP decouple target (`ksflag = 1`, frost on, `ksatadj` re-ported) is a
**runtime posture** the WS-2/WS-3 work applies, not an edit to this fixture.

## Intended tests

- **WS-2 (ksatadj re-port):** with `ksatadj=1` implemented under
  `INV-SUBHYD-032` (source-intent `avsat/(avpor·avcpm)`), frost on, openWEPP
  produces a **physically plausible** peak — the magnitude-sanity guard the
  1446.7/380,150 legacy value fails.
- **WS-3 (magnitude adjudication):** locate where the legacy peak calc diverges
  to river-scale under hydrophobicity and establish openWEPP's sensible envelope;
  confirm the burn-ordering *direction* is preserved.

## Contents & provenance

`p4.{run,man,slp,sol,cli}`; checksums in `SHA256SUMS`. No `snow.txt` /
`pmetpara.txt` / `gwcoeff.txt` sidecars — the disturbed test matrix did not
generate them; supply openWEPP defaults or document derivation if a run needs
them. The line-oriented `p4.run` is wepp.cloud provenance and is **not** directly
runnable by `openwepp-cli-hill`; a schema-versioned TOML runfile
(`schema = "openwepp-hillslope-runfile-v1"`) binding these inputs is required to
execute the cell (see `tests/fixtures/AGENTS.md`).

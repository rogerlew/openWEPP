# Channel Routing Options Input File Specification (`chan.inp`)

## 1. Header metadata
- `spec_id`: `SPEC-INFILE-CHANINP-001`
- `surface_id`: `infile-channel-contrast`
- `title`: `WEPP Channel Routing Options Sidecar (chan.inp)`
- `status`: `draft-HOLD`
- `owner`: `openWEPP`
- `spec_version`: `0.1.0`
- `last_updated_utc`: `2026-05-21`
- `evidence_mode`: `Static`

## Evidence anchors
- [DIRECT] `usersum2024` defines `chan.inp` purpose and four-line format.
  Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9546-9558`.
- [DIRECT] Legacy `wepp-forest` reads `chan.inp` only when `ipeak > 2`, applies clamps/defaults, and opens `chan.out`/`chanwb.out` conditionally.
  Evidence: `/workdir/wepp-forest/src/wshinp.for:469-514`.
- [DIRECT] Legacy symbols, meanings, and units for channel-routing control variables (`ichout`, `nchnum`, `dtchr`, `cbase`, `ichnum`) are declared in `cchrt.inc`.
  Evidence: `/workdir/wepp-forest/src/cchrt.inc:7-40`.
- [DIRECT] Legacy timestep limits come from `pmxchr.inc` (`dtlowl=60`, `dtupl1=3600`, `dtupl2=1800`, `mxtchr=1440`).
  Evidence: `/workdir/wepp-forest/src/pmxchr.inc:6-16`.
- [DIRECT] `wepppy` writes `chan.inp` as four lines and constrains UI overrides (`ichout_override` in `{1,3}`, `dtchr_override >= 60`).
  Evidence: `/workdir/wepppy/wepppy/nodb/core/wepp.py:2514-2588`, `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:115-132`.
- [DIRECT] `wepppyo3` repo scope is WEPP output/interchange and helper modules; no dedicated `chan.inp` parser contract is documented in module inventory.
  Evidence: `/workdir/wepppyo3/README.md:70-73`, `/workdir/wepppyo3/README.md:128-146`.

## 2. Surface scope and applicability
- File surface: `chan.inp` (`infile-channel-contrast`).
- Domain: watershed/channel-routing sidecar for updated routing methods.
- Applicability gate: consumed only in routing modes where `ipeak > 2` in the watershed channel file path.

[DIRECT] `usersum2024` states `chan.inp` provides additional options for updated watershed routing methods and defines line-by-line format.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9546-9558`.

[DIRECT] Legacy code branches on `if(ipeak > 2)` before attempting to read `chan.inp`.
Evidence: `/workdir/wepp-forest/src/wshinp.for:469-487`.

[INFERENCE] openWEPP should treat `chan.inp` as conditionally applicable by routing mode rather than universally required for watershed runs.

## 3. Version/datver applicability matrix

| Case | `chan.inp` internal version line | Legacy behavior | OpenWEPP draft stance |
| --- | --- | --- | --- |
| A | none (no explicit version token) | [DIRECT] parsed as raw 4-line payload when `ipeak > 2` | [INFERENCE] canonical grammar is unversioned; version compatibility is inherited from coupled watershed channel/run configuration |
| B | coupled to `.chn` routing-mode selection | [DIRECT] `ipeak` value governs read-path activation | [INFERENCE] treat `.chn` + `chan.inp` as a coupled surface contract |

[DIRECT] `usersum2024` `chan.inp` format does not include a version line.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9548-9555`.

[DIRECT] `ipeak` semantics originate in watershed channel file definitions and route method selection.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7245-7254`, `/workdir/wepp-forest/src/wshinp.for:469-514`.

## 4. Record grammar and line-by-line format definition

### 4.1 Canonical grammar

```text
Line 1: ichout dtchr
Line 2: cbase
Line 3: nchnum
Line 4: ichnum_1 ichnum_2 ... ichnum_nchnum
```

[DIRECT] `usersum2024` provides this 4-line shape and example values.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9548-9561`.

### 4.2 Legacy parser sequence and normalization

[DIRECT] Legacy read sequence:
1. `read(24,*) ichout, dtchr`
2. `read(24,*) cbase`
3. `read(24,*) nchnum`
4. `read(24,*) (ichnum(ichan), ichan=1,nchnum)`
5. clamp/normalize output controls and routing timestep.
Evidence: `/workdir/wepp-forest/src/wshinp.for:473-514`.

[DIRECT] Legacy timestep normalization:
- lower bound: `dtchr >= dtlowl (60 s)`
- upper bound: `dtchr <= dtupl1 (3600 s)` for continuous (`imodel==1`)
- upper bound: `dtchr <= dtupl2 (1800 s)` for event
- derive integer daily step count: `ntchr = 86400/dtchr + 0.99`, capped at `mxtchr`, then recompute `dtchr = 86400/ntchr`.
Evidence: `/workdir/wepp-forest/src/wshinp.for:488-496`, `/workdir/wepp-forest/src/pmxchr.inc:6-16`.

## 5. Field dictionary table with canonical symbols and alias mapping

| Canonical symbol | Line | Units | Type | Cardinality | Requiredness | Meaning | openWEPP boundary alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ichout` | 1a | unitless enum | int | 1/file | conditional (required when `ipeak>2`) | channel-flow output selector (`0` none, `1` peak, `2` daily avg, `3` timestep) | `channel_output_mode` |
| `dtchr` | 1b | s | real/int | 1/file | conditional (required when `ipeak>2`) | routing/output timestep | `channel_routing_timestep_s` |
| `cbase` | 2 | m^3/s/m^2 | real | 1/file | conditional (required when `ipeak>2`) | unit-area baseflow coefficient | `unit_area_baseflow_coefficient` |
| `nchnum` | 3 | count | int | 1/file | conditional (required when `ipeak>2`) | number of channels selected for routing output | `channel_output_count` |
| `ichnum(i)` | 4 | element ID | int | `nchnum`/file | conditional (required when `nchnum>0`) | selected channel identifiers from watershed structure element IDs | `channel_output_element_ids` |

[DIRECT] Variable definitions and enumerations are described in usersum and legacy include comments.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9548-9555`, `/workdir/wepp-forest/src/cchrt.inc:21-40`.

### 5.1 Enum dictionary
- `ichout=0`: no output.
- `ichout=1`: peak flow time and rate.
- `ichout=2`: daily average flowrate.
- `ichout=3`: timestep flowrate.

[DIRECT] Values and descriptions are listed in usersum and mirrored in `cchrt.inc`.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9549-9552`, `/workdir/wepp-forest/src/cchrt.inc:24-29`.

### 5.2 Alias mapping policy
[INFERENCE] Canonical WEPP names (`ichout`, `dtchr`, `cbase`, `nchnum`, `ichnum`) remain authoritative in this specification and parser-contract traces. openWEPP internal naming may differ but must preserve one-to-one alias mapping to these symbols for provenance continuity.

## 6. Conditional branches and optional sections
1. Branch A (activation): `chan.inp` read-path executes only when `ipeak > 2`.
2. Branch B (I/O errors): legacy catches open/read errors and defaults output controls.
3. Branch C (output files): `chan.out` and `chanwb.out` are opened only when `ichout > 0` and `nchnum > 0`.
4. Branch D (`nchnum` clamp): values below 0 are clamped to 0; above `nchan` are clamped to `nchan`.

[DIRECT] Branch behavior in legacy path.
Evidence: `/workdir/wepp-forest/src/wshinp.for:469-514`.

## 7. Cross-file consistency constraints and coupling dependencies
1. `ipeak` coupling: `chan.inp` applicability depends on watershed channel-file routing-method selection (`.chn` line 3).
2. ID-space coupling: each `ichnum(i)` must reference channel element IDs from watershed structure topology (`.str` element identifiers).
3. Count coupling: `nchnum` must be less than or equal to legacy `nchan` derived from structure/channel consistency checks.
4. Runtime coupling: channel-routing output selection intersects with routing loops that compare current element ID against `ichnum` list.
5. Workflow coupling: `wepppy` authoring path maps `chn_topaz_ids_of_interest` to WEPP element IDs, then emits `chan.inp` four-line payload.

[DIRECT] Coupling evidence:
- `ipeak` gate + `nchnum` clamp: `/workdir/wepp-forest/src/wshinp.for:469-514`
- runtime matching against `ichnum`: `/workdir/wepp-forest/src/chnrt.for:773-774`
- `wepppy` generation path: `/workdir/wepppy/wepppy/nodb/core/wepp.py:2568-2588`

[INFERENCE] openWEPP parser/data-model contracts should validate channel-ID namespace against loaded watershed topology, not just parse token counts.

## 8. Defaulting and missing-file behavior, including typed error expectations

### 8.1 Legacy-derived defaulting behavior
- If file open/read fails in `ipeak > 2` mode, legacy drops to `ichout=0` and/or `nchnum=0` via labeled error branches.
- `dtchr` is then bounded/normalized before daily routing-step setup.
- Legacy sets `cbase=0.` before attempting to read line 2.

[DIRECT] Evidence: `/workdir/wepp-forest/src/wshinp.for:470-496`.

### 8.2 OpenWEPP typed-error expectations (draft)
- `MissingFile { surface_id, path }` when `ipeak > 2` and strict-mode requires explicit sidecar presence.
- `UnexpectedEof { line_no }` when required lines are truncated.
- `ParseNumber { line_no, token }` for numeric conversion failure.
- `ChannelOutputCountInvalid { nchnum, nchan }` when channel-selection cardinality violates topology bounds.
- `ChannelOutputIdUnknown { ichnum, valid_ids }` when selected IDs are not present in watershed structure elements.
- `RoutingTimestepOutOfRange { dtchr, lower_s, upper_s, mode }` when strict-mode rejects out-of-range values instead of clamping.

[INFERENCE] Typed errors above are openWEPP boundary names; normative semantics come from usersum grammar plus legacy branches.

## 9. Example snippets (minimal valid, representative, invalid)

### 9.1 Minimal valid (single selected channel)
```text
1 600
0
1
4
```

[DIRECT] Aligns with usersum grammar and example structure.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9556-9561`.

### 9.2 Representative valid (timestep output, two channels)
```text
3 300
0
2
4 5
```

[INFERENCE] Representative routing-output selection for two structure element IDs.

### 9.3 Invalid example A (missing line 4 while `nchnum > 0`)
```text
3 600
0
2
```
Expected: `UnexpectedEof` / `ChannelOutputCountInvalid`.

### 9.4 Invalid example B (negative timestep)
```text
1 -10
0
1
4
```
Legacy path clamps to lower bound; strict openWEPP mode should raise `RoutingTimestepOutOfRange`.

## 10. Gap/conflict register with explicit `HOLD` conditions

| Gap ID | Statement | Evidence | Status |
| --- | --- | --- | --- |
| G1 | `usersum2024` line-2 guidance says `cbase` typically `1e-6` or smaller, but static search in legacy source did not find downstream computational use of `cbase` outside parse/common-block storage. | [DIRECT] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9553`; `/workdir/wepp-forest/src/cchrt.inc:39`; `/workdir/wepp-forest/src/wshinp.for:470-474`; `rg -n "\bcbase\b" /workdir/wepp-forest/src` | `HOLD` |
| G2 | Legacy error labels can bypass explicit `dtchr` assignment before clamp path when line-1 parse fails, creating ambiguous initialization semantics. | [DIRECT] `/workdir/wepp-forest/src/wshinp.for:473-496`; no `dtchr` initialization found via `rg -n "\bdtchr\s*=\s*" /workdir/wepp-forest/src` beyond `wshinp.for`. | `HOLD` |
| G3 | `wepppy` UI/parser constrains `ichout_override` to `{1,3}`, while usersum/legacy enumerate `0..3` and include mode `2`. | [DIRECT] `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:122-132`; `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9549-9552`; `/workdir/wepp-forest/src/cchrt.inc:24-29` | `HOLD` |
| G4 | `wepppyo3` currently provides WEPP output/interchange surfaces but no dedicated `chan.inp` input parser contract in documented module surface. | [DIRECT] `/workdir/wepppyo3/README.md:70-73`; `/workdir/wepppyo3/README.md:128-146` | `HOLD` |

## 11. Parser-contract handoff map (`SC-INFILE-CHANINP-001`)

| Contract area | Source requirement | Parser-contract expectation |
| --- | --- | --- |
| Applicability gate | Section 2/6 | Parse/apply only when watershed routing mode requires it (`ipeak > 2` policy explicitly codified). |
| Grammar | Section 4 | Enforce 4-line grammar with cardinality binding between `nchnum` and line-4 list length. |
| Symbol continuity + aliases | Section 5 | Preserve legacy symbol names as canonical; maintain explicit alias map for openWEPP data model. |
| Cross-file coupling | Section 7 | Validate `ichnum` against watershed structure element IDs and channel topology closure. |
| Defaults vs strict errors | Section 8 | Support explicit strict-mode typed errors; legacy-compat mode may emulate clamping/default branches with trace events. |
| Conflict/gap carry-forward | Section 10 | Keep contract `HOLD` until `cbase` semantics, `dtchr` error-path initialization policy, and `ichout` domain policy are dispositioned. |

- `parser_contract_id`: `SC-INFILE-CHANINP-001`
- `handoff_status`: `ready-for-contract-authoring (with HOLD gaps)`

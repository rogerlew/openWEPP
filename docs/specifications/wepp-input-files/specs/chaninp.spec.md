# Channel Routing Options Input File Specification (`chan.inp`)

## 1. Header metadata
- `spec_id`: `SPEC-INFILE-CHANINP-001`
- `surface_id`: `infile-channel-contrast`
- `title`: `WEPP Channel Routing Options Sidecar (chan.inp)`
- `status`: `in_review`
- `owner`: `openWEPP`
- `spec_version`: `0.1.2`
- `last_updated_utc`: `2026-07-11T00:00:00Z`
- `evidence_mode`: `Static`

## Evidence anchors
- [DIRECT] `usersum2024` defines `chan.inp` purpose and four-line format.
  Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9546-9558`.
- [DIRECT] Pinned legacy `wepp-forest` reads `chan.inp` only when
  `ipeak > 2`, applies clamps/defaults, and opens `chan.out`/`chanwb.out`
  conditionally.
  Evidence: `/workdir/wepp-forest_260430_baseline/src/wshinp.for:469-514`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- [DIRECT] Legacy symbols, meanings, and units for channel-routing control
  variables (`ichout`, `nchnum`, `dtchr`, `cbase`, `ichnum`) are declared in
  `cchrt.inc`.
  Evidence: `/workdir/wepp-forest_260430_baseline/src/cchrt.inc:7-40`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- [DIRECT] Legacy timestep limits come from `pmxchr.inc` (`dtlowl=60`,
  `dtupl1=3600`, `dtupl2=1800`, `mxtchr=1440`).
  Evidence: `/workdir/wepp-forest_260430_baseline/src/pmxchr.inc:6-16`
  at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
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
Evidence: `/workdir/wepp-forest_260430_baseline/src/wshinp.for:469-487`.

[INFERENCE] openWEPP should treat `chan.inp` as conditionally applicable by routing mode rather than universally required for watershed runs.

## 3. Version/datver applicability matrix

| Case | `datver` / activation state | Legacy behavior | OpenWEPP draft stance |
| --- | --- | --- | --- |
| A | no `datver` line (canonical format) and `ipeak <= 2` | [DIRECT] read-path not entered. | [INFERENCE] `SurfaceNotApplicable(surface_id=infile-channel-contrast, reason=ipeak_le_2)`; no parser error. |
| B | no `datver` line and `ipeak > 2`, full 4-line payload | [DIRECT] raw 4-line parse, then clamp/normalize controls. | [INFERENCE] canonical parse path with mode-dependent strict vs compat handling for normalization. |
| C | no `datver` line and `ipeak > 2`, file missing | [DIRECT] open failure branch defaults output controls. | [INFERENCE] strict: `MissingRequiredSurfaceError(surface_id=infile-channel-contrast, when=ipeak_gt_2)`; compat: WSHED-W10 default branch with `ichout=0`, `dtchr=60`, `ntchr=1440`, `cbase=0`, `nchnum=0`, empty `ichnum`, and no channel output. |
| D | no `datver` line and `ipeak > 2`, open fails (non-ENOENT I/O) | [DIRECT] open/read error labels collapse to default branches. | [INFERENCE] strict: `InputOpenError(surface_id=infile-channel-contrast, cause=...)`; compat: legacy-default branch plus `CompatibilityWarning(open_error_collapsed_with_default=true)`. |
| E | no `datver` line and `ipeak > 2`, truncated/malformed payload | [DIRECT] read error labels route to defaults; no typed distinction. | [INFERENCE] strict: `UnexpectedEof` / `ParseNumber`; compat: legacy-default branch plus parse-warning event. |
| F | extraneous version/header text present | [DIRECT] no dedicated version record is parsed; values are consumed positionally. | [INFERENCE] strict: reject non-numeric first required token with `ParseNumber`; compat: if numeric-leading values still satisfy positional parse, trailing text policy applies per Section 8. |

[DIRECT] `usersum2024` `chan.inp` format does not include a version line.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9548-9555`.

[DIRECT] `ipeak` semantics originate in watershed channel file definitions and route method selection.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:7245-7254`, `/workdir/wepp-forest_260430_baseline/src/wshinp.for:469-514`.

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
Evidence: `/workdir/wepp-forest_260430_baseline/src/wshinp.for:467-475`.

`[DIRECT]` The implied-DO in step 4 uses the raw line-3 count and precedes the
legacy clamp. `[INFERENCE]` Therefore source-record
cardinality closes before step 5: record 4 must contain exactly raw `nchnum`
IDs in strict and compatibility modes. Compatibility then retains raw
`nchnum_input`/`ichnum_input`, clamps `nchnum_norm` to `[0,nchan]`, and exposes
the first `nchnum_norm` raw IDs as `ichnum_norm`, matching the pinned legacy
prefix-selection rule. No openWEPP downstream ID-list consumer is currently
proved. The exact-arity/fail-closed policy is an
openWEPP inference: a conditional raw record-4 mismatch is `CHN-E-002`, not a
default. Negative counts remain the existing domain/normalization branch.

[DIRECT] Legacy timestep normalization:
- lower bound: `dtchr >= dtlowl (60 s)`
- upper bound: `dtchr <= dtupl1 (3600 s)` for continuous (`imodel==1`)
- upper bound: `dtchr <= dtupl2 (1800 s)` for event
- derive integer daily step count: `ntchr = 86400/dtchr + 0.99`, capped at `mxtchr`, then recompute `dtchr = 86400/ntchr`.
Evidence: `/workdir/wepp-forest_260430_baseline/src/wshinp.for:488-496`, `/workdir/wepp-forest_260430_baseline/src/pmxchr.inc:6-16`.

## 5. Field dictionary table with canonical symbols and alias mapping

| Canonical symbol | Line | Units | Type | Cardinality | Requiredness | Meaning | openWEPP boundary alias |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ichout` | 1a | unitless enum | int | 1/file | conditional (required when `ipeak>2`) | channel-flow output selector (`0` none, `1` peak, `2` daily avg, `3` timestep) | `channel_output_mode` |
| `dtchr` | 1b | s | real/int | 1/file | conditional (required when `ipeak>2`) | routing/output timestep | `channel_routing_timestep_s` |
| `cbase` | 2 | m^3/s/m^2 | real | 1/file | conditional (required when `ipeak>2`) | unit-area baseflow coefficient | `unit_area_baseflow_coefficient` |
| `nchnum` | 3 | count | int | 1/file | conditional (required when `ipeak>2`) | raw number of IDs read from record 4; retained before topology normalization | `nchnum_input`; normalized alias `channel_output_count` |
| `ichnum(i)` | 4 | element ID | int | raw `nchnum`/file | conditional (required when raw `nchnum>0`) | raw selected IDs; normalized parser projection is the first `nchnum_norm` entries | `ichnum_input`; normalized alias `channel_output_element_ids` |

[DIRECT] Variable definitions and enumerations are described in usersum and legacy include comments.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9548-9555`, `/workdir/wepp-forest_260430_baseline/src/cchrt.inc:21-40`.

### 5.1 Enum dictionary
- `ichout=0`: no output.
- `ichout=1`: peak flow time and rate.
- `ichout=2`: daily average flowrate.
- `ichout=3`: timestep flowrate.

[DIRECT] Values and descriptions are listed in usersum and mirrored in `cchrt.inc`.
Evidence: `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9549-9552`, `/workdir/wepp-forest_260430_baseline/src/cchrt.inc:24-29`.

### 5.2 Alias mapping policy
[INFERENCE] Canonical WEPP names (`ichout`, `dtchr`, `cbase`, `nchnum`, `ichnum`) remain authoritative in this specification and parser-contract traces. openWEPP internal naming may differ but must preserve one-to-one alias mapping to these symbols for provenance continuity.

## 6. Conditional branches and optional sections
1. Branch A (activation): `chan.inp` read-path executes only when `ipeak > 2`.
2. Branch B (I/O errors): legacy catches open/read errors and defaults output controls.
3. Branch C (output files): `chan.out` and `chanwb.out` are opened only when `ichout > 0` and `nchnum > 0`.
4. Branch D (`nchnum` clamp): values below 0 are clamped to 0; above `nchan` are clamped to `nchan`.

[DIRECT] Branch behavior in legacy path.
Evidence: `/workdir/wepp-forest_260430_baseline/src/wshinp.for:469-514`.

## 7. Cross-file consistency constraints and coupling dependencies
1. `ipeak` coupling: `chan.inp` applicability depends on watershed channel-file routing-method selection (`.chn` line 3).
2. ID-space coupling: each `ichnum(i)` must reference channel element IDs from watershed structure topology (`.str` element identifiers).
3. Count coupling: `nchnum` must be less than or equal to legacy `nchan` derived from structure/channel consistency checks.
4. Runtime coupling: channel-routing output selection intersects with routing loops that compare current element ID against `ichnum` list.
5. Workflow coupling: `wepppy` authoring path maps `chn_topaz_ids_of_interest` to WEPP element IDs, then emits `chan.inp` four-line payload.

[DIRECT] Coupling evidence:
- `ipeak` gate + `nchnum` clamp: `/workdir/wepp-forest_260430_baseline/src/wshinp.for:469-514`
- runtime matching against `ichnum`: `/workdir/wepp-forest_260430_baseline/src/chnrt.for:773-774`
- `wepppy` generation path: `/workdir/wepppy/wepppy/nodb/core/wepp.py:2568-2588`

[INFERENCE] openWEPP parser/data-model contracts should validate channel-ID namespace against loaded watershed topology, not just parse token counts.

## 8. Defaulting and missing-file behavior, including typed error expectations

### 8.1 Legacy-derived defaulting behavior
- If file open/read fails in `ipeak > 2` mode, legacy drops to `ichout=0` and/or `nchnum=0` via labeled error branches.
- `dtchr` is then bounded/normalized before daily routing-step setup.
- Legacy sets `cbase=0.` before attempting to read line 2.

[DIRECT] Evidence: `/workdir/wepp-forest_260430_baseline/src/wshinp.for:470-496`.

### 8.1a WSHED-W10 compatibility default branch

WSHED-W10 ratifies the deterministic openWEPP compatibility state used when
`chan.inp` is absent, unreadable, or collapsed after an ordinary token,
line1..3 structural, or non-cardinality parse failure in
compatibility mode. The legacy open/read-error labels directly support
`ichout=0`, `nchnum=0`, an empty selected-channel list, no channel output, and
`cbase=0`. The legacy branch can reach timestep normalization without a freshly
read `dtchr`; openWEPP resolves that ambiguous initialization point by fixing the
compatibility default to the lower-bound/mxtchr closure already represented by
the parser.

| Field | WSHED-W10 compat default |
| --- | --- |
| `parse_outcome` | `DefaultedCompat` or `OpenErrorCollapsedCompat` |
| `ichout` | `0` |
| `dtchr_input_s` | `60` |
| `dtchr_norm_s` | `60` |
| `ntchr` | `1440` |
| `cbase_m3_s_m2` | `0.0` |
| `nchnum_input` / `nchnum_norm` | `0` / `0` |
| `ichnum_input` / `ichnum_norm` | empty / empty |
| `chan_output_enabled` | `false` |

This default must be emitted as an explicit typed parser outcome with a
compatibility warning. Watershed runtime may consume that typed state directly;
it must not synthesize a separate hidden set of routing globals from an absent
optional `chan.inp` object.

`INV-CHN-013` raw conditional record-4 cardinality is explicitly excluded from
this default branch and returns `CHN-E-002` in both modes.

### 8.2 OpenWEPP strict-vs-compat typed expectations (draft)

| Condition | strict mode | compat mode |
| --- | --- | --- |
| `ipeak <= 2` (surface not applicable) | `SurfaceNotApplicable(surface_id=infile-channel-contrast, reason=ipeak_le_2)` trace event only. | same as strict. |
| `ipeak > 2` and `chan.inp` missing | `MissingRequiredSurfaceError(surface_id=infile-channel-contrast)` | WSHED-W10 branch with `CompatibilityDefaultApplied(ichout=0, dtchr=60, ntchr=1440, nchnum=0, cbase=0)`. |
| `ipeak > 2` and open fails (non-ENOENT I/O) | `InputOpenError(surface_id=infile-channel-contrast, cause=...)` | legacy-default branch plus `CompatibilityWarning(open_error_collapsed_with_default=true)`. |
| Required line1..3 truncated, or ordinary non-cardinality parse failure | `UnexpectedEof(surface_id=infile-channel-contrast, line_no=...)` | legacy-default branch plus `CompatibilityWarning(truncated_payload_defaulted=true)`. |
| Record 4 token count differs from raw `nchnum_input` | `CHN-E-002` | same exact `CHN-E-002`; structural cardinality is not defaultable |
| Numeric parse failure on required tokens | `ParseNumber(surface_id=infile-channel-contrast, line_no=..., token=...)` | legacy-default branch plus `CompatibilityWarning(parse_failure_defaulted=true)`. |
| `ichout` outside canonical domain (`0..3`) | `FieldRangeError(field=ichout, expected="0..3")` | clamp to legacy default branch (`ichout=0`) with trace event. |
| `nchnum` outside topology bounds (`0..nchan`) | `ChannelOutputCountInvalid(nchnum=..., nchan=...)` | clamp to `[0, nchan]` with `CompatibilityWarning(nchnum_clamped=true)`. |
| `dtchr` outside allowed range or requiring `ntchr` cap recompute | `RoutingTimestepOutOfRange(dtchr=..., lower_s=60, upper_s=...)` | legacy normalization path (`dtchr` bounded, `ntchr` capped, recomputed `dtchr`) with trace event. |
| Unknown `ichnum(i)` not in watershed structure ID set | `ChannelOutputIdUnknown(ichnum=..., valid_ids=...)` | `CompatibilityWarning(unknown_channel_id_retained=true)` unless downstream contract mandates strict topology closure. |

[INFERENCE] Typed outcomes above are openWEPP boundary names; legacy-derived compat branches preserve default/clamp semantics while surfacing explicit diagnostics.

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

## 10. Gap/conflict register

| Gap ID | Provenance tags | Statement | Evidence | Status |
| --- | --- | --- | --- | --- |
| `CHANINP-GAP-001` | `usersum2024`, `legacy-code` | `usersum2024` line-2 guidance says `cbase` typically `1e-6` or smaller, but static search in legacy source did not find downstream computational use of `cbase` outside parse/common-block storage. | [DIRECT] `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9553`; `/workdir/wepp-forest_260430_baseline/src/cchrt.inc:39`; `/workdir/wepp-forest_260430_baseline/src/wshinp.for:470-474`; `rg -n "\bcbase\b" /workdir/wepp-forest_260430_baseline/src` | `RATIFIED-W4DR-006` |
| `CHANINP-GAP-002` | `legacy-code` | Legacy error labels can bypass explicit `dtchr` assignment before clamp path when line-1 parse fails, creating ambiguous initialization semantics. | [DIRECT] `/workdir/wepp-forest_260430_baseline/src/wshinp.for:473-496`; no `dtchr` initialization found via `rg -n "\bdtchr\s*=\s*" /workdir/wepp-forest_260430_baseline/src` beyond `wshinp.for`. | `RATIFIED-WSHED-W10` |
| `CHANINP-GAP-003` | `wepppy`, `usersum2024`, `legacy-code` | `wepppy` UI/parser constrains `ichout_override` to `{1,3}`, while usersum/legacy enumerate `0..3` and include mode `2`. | [DIRECT] `/workdir/wepppy/wepppy/nodb/core/wepp_input_parser.py:122-132`; `/home/workdir/openWEPP/references/copyrighted/source_pdfs/WEPP_usersum2024.txt:9549-9552`; `/workdir/wepp-forest_260430_baseline/src/cchrt.inc:24-29` | `RATIFIED-W4DR-004` |
| `CHANINP-GAP-004` | `wepppyo3` | `wepppyo3` currently provides WEPP output/interchange surfaces but no dedicated `chan.inp` input parser contract in documented module surface. | [DIRECT] `/workdir/wepppyo3/README.md:70-73`; `/workdir/wepppyo3/README.md:128-146` | `RATIFIED-W4DR-003` |

## 11. Parser-contract handoff map (`SC-INFILE-CHANINP-001`)

| Contract area | Source requirement | Parser-contract expectation |
| --- | --- | --- |
| Applicability gate | Section 2/6 | Parse/apply only when watershed routing mode requires it (`ipeak > 2` policy explicitly codified). |
| Grammar | Section 4 | Enforce 4-line grammar with cardinality binding between `nchnum` and line-4 list length. |
| Symbol continuity + aliases | Section 5 | Preserve legacy symbol names as canonical; maintain explicit alias map for openWEPP data model. |
| Cross-file coupling | Section 7 | Validate `ichnum` against watershed structure element IDs and channel topology closure. |
| Defaults vs strict errors | Section 8 | Support explicit strict-mode typed errors; legacy-compat mode may emulate clamping/default branches with trace events. |
| Conflict/gap carry-forward | Section 10 | Parser contract owns ratified default/compat posture; remaining implementation must preserve explicit strict-vs-compat outcomes. |

- `parser_contract_id`: `SC-INFILE-CHANINP-001`
- `canonical_contract_path`: `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `handoff_status`: `ratified-for-wshed-w10-implementation`

## 12. Revision history

| Date | Version | Change |
| --- | --- | --- |
| 2026-07-11 | 0.1.2 | Ratified raw record-4 cardinality before compatibility normalization, raw-field retention, first-`nchnum_norm` normalized IDs, and non-collapsible `CHN-E-002`. |
| 2026-07-09 | 0.1.1 | WSHED-W10 ratified typed compatibility defaults for absent/unreadable/malformed `chan.inp` and pinned legacy evidence paths to `/workdir/wepp-forest_260430_baseline`. |

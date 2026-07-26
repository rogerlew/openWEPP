# CAL-02 Admission

Verdict: `READY_BOUNDED`

CANOPY-CAL-02 may execute a bounded, deterministic site-specific analytical
reconstruction using the exact report-linked WEPPcloud source fixtures under
`tests/fixtures/canopy_phenology/elliot_reproduction/source/weppcloud/`.
Hubbard Brook is bound to `unassailable-sensuousness` `p1`/`H1`; Santee is
bound to `clean-burning-griddle` `p2`/`H2`. Both run files encode 100 years,
and both exact climate headers identify CLIGEN `5.32300` with seed `12345`.

The admitted arms are:

- Hubbard source forcing with its exact `p1.man` baseline;
- Hubbard source forcing with Bill's delivered hardwood management
  (`dropfc=0.95`);
- Hubbard source forcing with a branch derived from that delivered file by
  changing only `dropfc` to `0.92`; and
- Santee source forcing with its exact `p2.man` baseline; and
- Santee source forcing with Bill's delivered mixed-forest management.

Only management replacement, the single `dropfc` branch edit, and run-file
relative-path rewrites required by an isolated CAL-02 fixture are authorized.
CAL-02 must verify both site `SHA256SUMS` manifests and record every derived
identity before running. No calibration or other operand edit is admitted.

The exact report-era executable is accessible on BLARHG at
`C:\WEPP\wepp\wepp_2012.exe`, SHA-256
`6104a3440624ad54aa6c3660794280adfd600d4a11b98559c6205a73cd47fc3f`.
This does not make the Windows rerun exact: Bill's manually transcribed
2006.5-format soils/slopes, Windows run files, constant-cover files, runtime
libraries, and Windows text outputs remain absent.

The reconstruction must:

- execute both Hubbard `dropfc=0.92` and `dropfc=0.95`;
- classify Yang `7.6 Mg/ha` as standing foliage biomass, not annual leaf fall;
- keep foliage/needle and woody litter distinct wherever native diagnostics
  permit;
- keep forest-floor stock separate from total fuel;
- keep hillslope surface runoff and sediment separate from watershed discharge
  and channel sediment;
- exclude the two AI-attributed field values; and
- make no parameter-selection or production-default change.

The retained selected WEPPcloud outputs and return-period JSON may be checked
byte-for-byte and analytically. Bill's Windows chart/table values may expose
direction and discrepancy, but are not exact equivalence targets until his
converted Windows project and output bundle is recovered.

Neither source-native management is constant cover. Bill's Windows
constant-cover comparison remains `BLOCKED_SOURCE_BUNDLE` unless a superseding
record admits a real hash-bound comparator.

The machine-readable authority is `cal02-admission.json`.

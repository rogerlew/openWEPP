# Kernel-Profile Compliance Checklist

Status: completed-with-hold

Evidence mode: static+ran

Static: Reviewed HPHYS0263 package artifacts, canonical contract edits, and
production diffs against the kernel-process contract profile.

Ran: Validation gates are recorded in `gate-results.md`.

## Checklist

- [x] Contract-first sequence recorded in `package.md`,
  `contract-implementation-evidence.md`, and
  `pre-implementation-contract-gate.md`.
- [x] Canonical `SC-*` authority amended before production code edits in
  `SC-EVAP-001` and `SC-WATBAL-001`.
- [x] Contract-derived test added and red pre-implementation failure recorded.
- [x] Baseline provenance cites pinned
  `/workdir/wepp-forest_260430_baseline/src/evappm.for:181-388` and
  `/workdir/wepp-forest_260430_baseline/src/sunmap.for:181-234`.
- [x] Production PMET demand equations preserve legacy symbol continuity for
  `ed`, `ra`, `rso`, `rbo`, `rn`, `fwv`, `dlt`, `pb`, `gma`, `etorc`, `rhd`,
  `kcbadj`, `kcbcon`, `etke`, `etkr`, `etks`, `TEW`, `REW`, `TAW`, and `RAW`.
- [x] No heuristic/proxy PMET demand substitute added; the previous
  Priestley-Taylor fallback is no longer used when `iflget != 1`.
- [x] Required runtime inputs fail explicitly through typed WB11 seed errors
  instead of silent defaults.
- [x] Runtime projection changes are explicit for `pmetpara.txt`
  discoverability, `canhgt`, `deglat`, and `elevm`.
- [x] Truthfulness labels are present in evidence artifacts.
- [x] Full H1..H39 hillslope diagnostics are recorded.
- [x] Disposition remains `HOLD` for full `evappm.for` routine closure because
  pinned `evappm.for:391-454` post-ET soil-evaporation redistribution is not
  migrated in this package.

## Profile Notes

- The migrated subset is sufficient for WB11 PMET demand seeding and H1/H7/H39
  branch classification.
- Full routine closure is intentionally not claimed because the remaining
  `evappm.for` storage redistribution mutates soil water after `Es`.
- Independent dual sub-agent review/verification is not claimed for this run;
  local review and local verification are recorded because sub-agent dispatch
  was not explicitly requested in the user instruction for HPHYS0263.

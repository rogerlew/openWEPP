# Worker Handoff

Status: complete

Evidence mode: ran

Static:

- HPHYS0307 executed the HPHYS0306 continuation and remains `HOLD`.
- Production physics edits remain unauthorized.

Ran:

- Added `SC-WATBAL-001#INV-WATBAL-080`.
- Added and ran `hphys0307_melt_call_branch_activation_contract`.
- Classified all nine H1/H7/H39 rows:
  - seven `baseline-extra-melt-call-hold`;
  - one `openwepp-extra-melt-call-hold`;
  - one `same-hour-multi-source-hold`.
- No row authorizes downstream compensation.

## Required Continuation

The next work package should diagnose the source-line cause of the branch-extra
keys before any numeric melt-term correction. Scope:

- Extract exact fixed-baseline state values and predicate outcomes for the
  HPHYS0307 baseline-extra/openWEPP-extra keys from `winter.for`/`snowd.for`.
- Compare those outcomes against openWEPP state ordering around
  `compute_simimpl29_melt_hour` and `melt_branch_active` publication.
- Port/correct openWEPP branch activation only if source-line provenance proves
  an openWEPP defect.
- Keep H39 first-2013 same-hour `cmelt`/`snodpt` as separate source-ordering
  scope unless branch-predicate diagnosis directly covers it.
- Keep WB13/WB17/WB18/WB19/WB12 compensation prohibited.

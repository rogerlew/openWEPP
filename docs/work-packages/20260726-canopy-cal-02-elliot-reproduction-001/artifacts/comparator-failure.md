# Comparator Failure Evidence

Evidence class: `Ran`

The required read-only `comparator_suite_runner` invoked the exact harness:

```text
.venv/bin/python tools/canopy_phenology/elliot_reproduction.py run \
  --results-root /tmp/openwepp-cal02-20260726-203727-11375 \
  --remote-root C:/Users/roger/AppData/Local/Temp/openwepp-cal02-20260726-203727-11375 \
  --host BLARHG
```

WEPP accepted the derived diagnostic run control, exact constant management,
slope, and climate filenames, then failed while reading the exact source soil:

```text
forrtl: severe (64): input conversion error, unit 11, file
C:\Users\roger\AppData\Local\Temp\openwepp-cal02-20260726-203727-11375\
hubbard_constant\run\p1.sol
```

All five arms exited `64` on their exact source soil. The Hubbard soil SHA-256 was
`01cf3b649f59bcd5e1c2b43e51fd56b026664e96a6c3e215140878e2452ec31c`.
The Santee soil SHA-256 was
`c0ae98f8924b6786e8c079bf91fec0cad96ed212e174f531eb60399ff1b10951`.
The file identifies source representation `9002`. The retained CAL-01
missing-source record names Bill's 2006.5-format Windows soil as absent.

The executable SHA-256 matched
`6104a3440624ad54aa6c3660794280adfd600d4a11b98559c6205a73cd47fc3f`
both before and after execution. The partial output files contain headers only
and are rejected as scientific results. No arm produced an accepted row.

The temporary roots retain full manifests and logs. `failure-receipt.json`
commits their per-arm manifest/log/command hashes and exact command without
committing generated run debris.

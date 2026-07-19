# Global CRAP Closure

Ran: exact forest1 candidate on 2026-07-19 PDT.

The fresh global acquisition ran directly inside the reviewed isolated runner
on commit `7ccc61d5e405529789417f87130978f63679ded5` with planning base
`86bce645ae53d5ef9b984666fdb20206f9a62e7e`:

```text
bash tools/release/run_adjudicated_crap_gate.sh \
  --base-ref 86bce645ae53d5ef9b984666fdb20206f9a62e7e \
  --head-ref 7ccc61d5e405529789417f87130978f63679ded5 \
  --output-dir global-crap
```

- Exit: 0; elapsed 1,909 seconds.
- Instrumented full-profile Nextest: 2,165/2,165 passed, 5 skipped, 10 slow.
- Production entries assessed: 10,714.
- Raw / adjudicated / actionable rows: 2 / 2 / 0.
- Touched production files: 11; touched actionable rows: 0.
- Closure eligible: true; debt status: `PASS`.
- Source remained clean; the before, report, and final source manifests contain
  249 sources at SHA-256
  `9368ec0df2841cef560e3d5f8fba8466de78caae51e7d69285bd4beee1485f10`.
- CRAP JSON SHA-256:
  `69d32261a6406353a2e992100bb79d83f1904427208fb85043f1f582d75b070e`.
- LCOV SHA-256:
  `848c51d97b6232b1d38f3824388c6863fcfd0467c23ed79b30fa30dbc77a4a6f`.
- Adjudicated report JSON / Markdown SHA-256:
  `d4ac6256465c1ea6e491020519dc499fa7052364a3b94acfbf39f92a10cc1698` /
  `39837de8f8092bf1b78042fdf60efb04767a565949e5d29af270bc140d93e74c`.
- Run-status SHA-256:
  `6d4dbd275fce1169c02c1fd1fefb082f443c254818f9870eef8db4f35dfa6d3c`.

The two raw rows are the existing valid adjudications `CQR-LOW-L08` and
`CQR-LOW-L11`; invalid/stale adjudications and actionable rows are empty.

The first wrapper invocation exited before clone or test execution because its
shell transport had no script argument and used a non-existent candidate SHA.
It produced no coverage or CRAP evidence and did not consume the one heavy-run
budget. The corrected invocation verified the exact pushed SHA before starting.

After receipt extraction, the source clone, Cargo cache, target, evidence,
temporary files, home, and diagnostics were purged from all six writable
runner surfaces. Only the GitHub listener remained, and the provider returned
online and idle.

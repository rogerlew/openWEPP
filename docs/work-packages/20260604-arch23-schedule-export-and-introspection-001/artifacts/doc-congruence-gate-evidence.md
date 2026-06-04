# Doc Congruence Gate Evidence

Status: complete
Evidence mode: Ran

## Gate

- `bash tools/release/check_hillslope_schedule_export.sh`

## Passing Evidence

Ran: `bash tools/release/check_hillslope_schedule_export.sh` returned exit 0 and printed:

```text
hillslope schedule export artifacts are congruent
```

## Intentional Drift Check

Ran: temporarily appended a blank line to `docs/architecture/generated/hillslope-phase-schedule.json`, ran the gate, captured failure, and restored the artifact by trap before rerunning the passing gate.

Observed failure:

```text
status=1
--- docs/architecture/generated/hillslope-phase-schedule.json
+++ /tmp/tmp.55lJqSl3ZC/hillslope-phase-schedule.json
@@ -142,4 +142,3 @@
     "closure_diagnostics"
   ]
 }
-
hillslope schedule export drift detected for hillslope-phase-schedule.json
Regenerate with:
  cargo run --manifest-path crates/openwepp-hillslope-orchestrator/Cargo.toml --bin openwepp_hillslope_schedule_export -- generate --output-dir docs/architecture/generated
```

Ran: after restoration, `bash tools/release/check_hillslope_schedule_export.sh` again returned exit 0.

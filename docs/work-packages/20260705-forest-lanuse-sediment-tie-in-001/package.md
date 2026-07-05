# FOREST-LANUSE SEDIMENT TIE-IN

Status: `IN-EXECUTION` (Claude-executed, operator path direction
2026-07-05: "stay on the hillslope sediment in forest lanuse
Papanicolaou path"). Branch: `forest-lanuse-sediment-tie-in`.

## What this is

With WS1's native forest lanuse merged, the erosion seed's hardcoded
`is_cropland: false` (the recorded 1b-C adjudication item) becomes
resolvable from the PARSED lanuse. The nuance the 1b-C note recorded:
legacy runs every production landuse as `lanuse = 1` cropland (the
masquerade), so legacy p61/p102 exercised the CROPLAND interrill
delivery branch (`param.for:412-450`: `rif = −23·rrc + 1.14`, per-class
`drinti`, `intdr < 1`) — while our hardcoded non-cropland branch uses
`intdr = 1`. Neither branch is wrong per se; they belong to different
lanuse declarations.

## Design

1. **`is_cropland` flows from the parsed lanuse, schedule-scoped** (the
   same shape as the tillage detector): Cropland yearlies → `true`
   (legacy-faithful `drinti` branch on masquerade managements), Forest
   yearlies → `false` (`intdr = 1`, the source-true `lanuse ≠ 1`
   branch). Mixed schedules fail closed (one lanuse per lane — the WS1
   reconciliation already polices this).
2. **Identity-doc adjudication recorded, not decided:** whether
   roughness-driven interrill delivery is universal physics (making the
   legacy non-cropland `intdr = 1` a symptom-partition) is flagged as a
   science item in SC-SED-001 — the port stays source-true per branch.
3. **Evidence re-run:** the masquerade instruments (p61, p102, WS3)
   now exercise the `drinti` branch — re-evidence against legacy
   (expected: p61 delivery moves DOWN from 3.97 as `intdr < 1`
   suppresses interrill supply; the water-cut band 0.6–1× is the
   judgment frame). The erod16 instrument (crafted non-cropland) is
   unchanged.
4. **Native-forest sediment proof:** extend the WS1 native-forest CLI
   fixture with sediment assertions — the first "hillslope sediment on
   forest lanuse" end-to-end evidence (non-cropland branch, Wave-1
   enabled by construction).

## Acceptance

- p61/p102 within the water-cut judgment band post-change; erod16
  byte-stable; the native-forest fixture publishes sediment.
- SC-SED-001 amendment (lanuse-sourced `is_cropland` + the flagged
  universality question).
- Gates per the local-ci standard; full at branch head; Codex review.

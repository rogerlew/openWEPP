# PERFDEEP07 Direct-Frame Implementation Plan

Status: blocked by HOLD.
Evidence mode: Static.

Direct-frame implementation planning did not start because the ordered
default-disabled P0 timing gate failed.

## Required Contents

Populate after the disabled-path P0 gate passes.

Required sections:

- migrated phase span and stop boundary;
- `HillslopeDayContext`, forcing, frame, and view API changes;
- initial seed boundary and shadow oracle;
- fields and arrays included from PERFDEEP06 inventory;
- publication projection/shadow edge;
- opt-in switch and fail-closed behavior;
- no-hot-loop-map absence checklist for the migrated success path.

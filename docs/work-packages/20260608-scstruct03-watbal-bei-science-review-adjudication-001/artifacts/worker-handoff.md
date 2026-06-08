# SCSTRUCT03 Worker Handoff

Evidence mode: Static
Status: executed-HOLD handoff

## Current state

SCSTRUCT03 completed batches 1-6 and stopped with 33 deferred BEI rows. The remaining work is not mechanical relocation; it is contract-authoring/promotion or exact mapping proof for live obligations.

## Next command to inspect current gate

`python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`

Expected current result: `PASS-DEFERRED` with deferred rows remaining and exit code `1` under `--strict`.

## First actionable work

Use `artifacts/followon-queue.md`. For each owner/gate, either:

1. promote precise `INV-WATBAL-*` / `OBL-WATBAL-*` binding rows through full review/verification, or
2. prove a complete existing mapping and update the BEI, or
3. keep a narrower HOLD with a cited authority gap.

Do not relocate remaining core narrative until one of those outcomes is recorded.

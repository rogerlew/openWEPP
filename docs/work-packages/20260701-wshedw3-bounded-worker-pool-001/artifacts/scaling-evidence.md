# Scaling Evidence

Status: `QUEUED`

W3 execution must record canonical scaling evidence here.

Required canonical surface:

- Fixture: committed `tests/fixtures/watershed/carnivorous-adobo/`.
- Mode: `strict-committed-fixture`, sidecar/input-discovery mode explicitly
  labeled.
- Job counts: `1`, `2`, `4`, `8`, `16`, and `32` where hardware permits.
- Repeats: at least three clean repeats per accepted job count.
- Metrics: wall, user, system, max RSS, CPU inventory, per-job duration
  distribution, route-stage duration, total end-to-end duration, and output
  identity evidence against `--jobs 1`.

Contextual surfaces:

- Arboreal-dendrite, `/wc1`, scratch, or legacy comparisons may be recorded
  only as contextual engineering-budget evidence.
- Discovery-on and discovery-off timings must not be compared as the same
  benchmark surface.

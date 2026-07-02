# Output Identity Evidence

Status: `blocked`

Evidence mode: `Ran:` one public watershed probe; full W7 identity blocked.

Full W7 `--jobs 1` vs `--jobs N` identity was not run because no accepted
sediment-active fixture exists.

Supporting public-path probe:

- Fixture: `tests/fixtures/watershed/carnivorous-adobo/`
- Command: `target/release/openwepp-cli-watershed --jobs 8`
- Output: `/tmp/wshedw7_probe_carn/out`
- Result: command completed and wrote required watershed outputs after the
  relative-path supervisor fix.
- Sediment result: rejected for W7 acceptance because detachment, deposition,
  and sediment delivery remained zero.

Identity must be rerun by the hold-lift package after a committed fixture
produces nonzero sediment.

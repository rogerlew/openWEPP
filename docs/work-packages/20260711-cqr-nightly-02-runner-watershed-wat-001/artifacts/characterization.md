# Characterization

Ran: ten target-local tests pass. Six new tests cover every selected public/file
path plus guard families; four retained tests cover row decoding, optional-null
semantics, invalid area, and outlet lateral aggregation.

The successful same-day multi-OFE publication fixture separates Area (`100`,
`300` m2), runoff Q (`5`, `7` mm), QOFE (`6`, `8` mm), and optional baseflow
(`0`) so rejected aliases cannot satisfy the expected weighted runoff depth
(`6.5` mm), QOFE (`7.5` mm), or runoff volume (`2.6` m3). Volume is independently
reconstructed as `6.5 * 400 / 1000`, not read from producer intermediate state.

```text
cargo nextest run -p openwepp-runner --lib watershed_wat::tests
```

Result: `10` passed, `94` filtered/skipped.

Fixture directories use process-local atomic identifiers and RAII cleanup; no
wall clock, random input, or panic-leaking manual cleanup remains.

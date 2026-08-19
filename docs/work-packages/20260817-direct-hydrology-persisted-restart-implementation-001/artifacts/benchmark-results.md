# Benchmark Results

Ran release build on 2026-08-19 in the repository Nix shell, Linux, Rust
1.95.0, on commit lineage `32b54d447`, selector
`restart-v1-benchmark`, 20 samples after one construction warm-up. Fixture is
the real interval-24, two-destination, 48-interval repository fixture.

| Operation | Median | Maximum |
|---|---:|---:|
| interval-24 serialization | 175 us | 233 us |
| admission | 6,220 us | 9,239 us |
| isolated host restore | 3,507 us | 5,498 us |
| remaining 24 intervals | 100,769 us | 127,787 us |
| finish | 46 us | 127 us |
| exact abort bytes | 29 us | 53 us |

Checkpoint size is 264,895 bytes. These are observational default-off budgets;
no wire compression or authority change was needed. Peak allocation was not
instrumented and is recorded as a nonblocking measurement limitation.


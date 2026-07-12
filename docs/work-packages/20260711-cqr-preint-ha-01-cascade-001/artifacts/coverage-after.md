# Coverage After

Evidence class: **Ran**

Delegated iteration full-library metrics:

- lines: 406/428 (`94.860%`);
- regions: 531/562 (`94.484%`);
- functions: 21/21 (`100%`);
- `interpolate_unit_discharge`: 44/44 regions (`100%`);
- lowest non-excluded production closure: 11/12 (`91.667%`).

Both formats passed 341/341 tests. LCOV SHA-256
`ce267937a009d70a0c2b4f4b0553da3f60feab085bc29ef1c3e7bbf3260ae6ca`;
JSON SHA-256
`3262e619778f492019a26bfe9e34c1158837e43b60327fc6901b8708feca0287`.
Artifacts remain under `/tmp/openwepp-cqr-preint-ha01-after*`.

The authoritative final-source workspace JSON reports production-only
`cascade.rs` at 207/211 lines (`98.104%`) and 283/288 regions (`98.264%`). All
11 logical production functions/closures are covered; the lowest floor is
11/12 (`91.667%`), `sample_upstream_point` is 9/9, and
`interpolate_unit_discharge` is 44/44. Whole-file supplemental coverage is
422/442 lines and 554/581 regions. JSON SHA-256 is
`eaabd76ed3adcf48dc7d73040ecc8feba88f004a7629b7225ea1286ee723f599`.

Target science/floor coverage passes. The first final LCOV capture is not the
accepted workspace comparison because selector-suite interference lowered one
non-target runner function's coverage. Same-source retry 1 returned to the exact
start failure set and is the accepted LCOV/CRAP comparison; no non-target row
changed.

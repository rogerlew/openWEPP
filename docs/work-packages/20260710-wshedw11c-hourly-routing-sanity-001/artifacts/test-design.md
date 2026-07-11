# Test Design

Status: `EXECUTED-CURRENT`

Evidence mode: `Static`

The binding scenario matrix, operands, reconstruction, and classification rules
are declared in `package.md`.

Implementation uses the existing real-CLI HBP builder and Parquet reader in
`mt3_hbp_hourly_consumer_contract.rs`. Wave branches use paired EVENT payloads
for both hillslope contributors. The CREAMS zero control uses canonical
NOEVENT payloads, and its nonzero cases use an upstream EVENT plus downstream
NOEVENT because a zero-valued EVENT is not the scalar lane's no-runoff form.

The `chan.inp` payload is four canonical records:

```text
3 <dtchr>
0.0
2
3 4
```

`nchnum=2` is intentional. The parser drops empty lines, so the old W11B
`nchnum=0` three-line fixture collapsed to compatibility defaults and did not
exercise its written `dtchr=600`. W11C parses its generated sidecar before each
CLI call and asserts `ParsedBranch`, zero warnings, `dtchr_norm_s`, `ntchr`,
`nchnum_norm`, and topology channel IDs `[3, 4]`.

The test is a characterization harness: executable/finite/zero/closure
properties remain assertions. Physical findings such as material negative
storage and passive-route peak amplification are emitted as structured
`W11C_FINDING` records so the full matrix completes and can be reviewed.

# R6F Current Failure Reproduction

Status: pending execution.

R6F starts by reproducing the inherited R6E cutover failure exactly. The
failure is an iteration starting point, not a valid final disposition.

## Expected Inherited Marker

`HOLD-R6E-HBP-DIRECT-PROCESS-PARITY-MISMATCH`

Expected stderr includes:

```text
HBP byte identity failed: direct=1654 bytes compatibility=1654 bytes
```

## Reproduction Command

| Field | Value |
|---|---|
| Date/time | Pending |
| Commit | Pending |
| Command | Pending |
| Fixture/run | Pending |
| Environment variables | Pending |
| Exit code | Pending |

## Observed Result

| Evidence | Observed value |
|---|---|
| Hold marker | Pending |
| Direct bytes | Pending |
| Compatibility bytes | Pending |
| Public HBP written | Pending |
| Public WAT written | Pending |
| Public PASS written | Pending |
| Public loss written | Pending |
| Public manifest written | Pending |
| Direct frame counter | Pending |
| Direct executor counter | Pending |
| Publication capture counter | Pending |
| Direct compute counter | Pending |
| State mutation counter | Pending |
| Downstream operand counter | Pending |
| Shadow projection counter | Pending |
| Skeleton-run counter | Pending |
| Compatibility-edge counter | Pending |

## Candidate Artifacts

| Artifact | Path | Notes |
|---|---|---|
| Direct HBP candidate | Pending |  |
| Compatibility HBP candidate | Pending |  |
| Direct WAT candidate | Pending |  |
| Compatibility WAT candidate | Pending |  |
| Direct PASS candidate | Pending |  |
| Compatibility PASS candidate | Pending |  |
| Direct loss candidate | Pending |  |
| Compatibility loss candidate | Pending |  |
| Direct manifest candidate | Pending |  |
| Compatibility manifest candidate | Pending |  |

## Immediate Next Step

After reproduction, update `r6f-blocker-ledger.md` and
`r6f-hbp-byte-diff.md`. Do not stop at this artifact.

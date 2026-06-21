# R6F Output Family Burndown

Status: scaffolded.

Do not move an output family to closed until the family reads typed direct
projection only and the required parity/reconstruction evidence exists.

| Family | Required gate | Current state | Evidence | Next action |
|---|---|---|---|---|
| HBP | Byte identity plus field/operand lineage. | Open | `r6f-hbp-byte-diff.md` pending. | Reduce inherited byte mismatch. |
| WAT | Arrow row, schema, value, and metadata parity. | Pending HBP. | Pending. | Start after HBP identity. |
| PASS | Arrow row, schema, value, and metadata parity with PASS fixture coverage. | Pending HBP. | Pending. | Ensure fixture emits PASS Parquet. |
| Loss | JSON identity. | Pending HBP. | Pending. | Compare direct and compatibility candidates. |
| Manifest | Direct provenance/checksum parity. | Pending HBP. | Pending. | Cut over manifest writer to direct projection. |
| Public writes | Direct cutover writes all required output files. | Pending all parity. | Pending. | Enable only after gates pass. |

## Iteration Notes

| Date | Family | Change | Result |
|---|---|---|---|
|  |  |  |  |

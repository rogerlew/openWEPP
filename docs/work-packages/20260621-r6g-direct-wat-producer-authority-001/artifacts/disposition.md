# R6G Disposition

Status: executed-held.

Final verdict:

`HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT`

R6G executed the in-envelope direct WAT producer work needed to reduce the R6F
producer-authority gap for the inherited current fixture: WAT id/year/profile
fields no longer reduce, first-day ET/storage matches, and direct storage
projection includes residual liquid water. The current-fixture HBP identity gate
remains green, and the first WAT row now matches compatibility exactly without
using compatibility WB13 rows or post-scheduler runtime surfaces as direct
authority.

R6G cannot truthfully close as `COMPLETE-R6G-DIRECT-WAT-PRODUCER-AUTHORITY`
because day-2 WAT parity still fails on exactly `Es`, `Total-Soil`, and
`SoilWaterTotal`. The remaining mechanism is outside the static precomputed day
input model: PMET component inputs for a day must be constructed after the
previous direct day commits its carried layer state. Filling those values from
compatibility WB13 rows or runtime surfaces would violate the R6 cutover
authority model, so the package holds instead of aliasing.

Review disposition records accepted follow-ups for canonical multi-OFE WAT id
authority, lane-dimensional direct day inputs, and full allowlisted
no-compatibility symbol lineage. Those follow-ups are why this package is
`executed-held`, not complete.

Required next action:

Close `HOLD-R6G-WAT-PMET-DAY-STATE-CARRY-BUILDER-ABSENT` by replacing the
precomputed multi-day PMET direct publication input vector with an interleaved
direct day-input builder that constructs day `n` PMET ET operands from
direct-carried layer state after day `n-1` direct commit, then reruns the R6
publication cutover gates.

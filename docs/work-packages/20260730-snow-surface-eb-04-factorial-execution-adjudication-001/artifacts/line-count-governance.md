# Line-Count Governance

Status: `complete`

Evidence mode: `Static`

| File | Base lines | Terminal lines | Disposition |
| --- | ---: | ---: | --- |
| direct-runner day/trace builder | 2,297 | 2,326 | WARN |
| package-local factorial tool | n/a | 1,751 | Evidence tooling; visible |

The touched Rust file was already above the 2,000-line warning threshold and
remains below 3,000. It is an established split-module surface; EB-04 adds one
contiguous diagnostic serialization block and no new control-flow family.
Further decomposition inside a result-bearing science package would mix a
mechanical refactor with the evidence increment. Follow-on split intent:
runner-maintenance work should extract the R7H snow-trace serializer and
thermal field formatter into a dedicated direct-publication submodule while
preserving exact JSON bytes and focused trace tests. No 3,000-line hard
threshold is crossed.

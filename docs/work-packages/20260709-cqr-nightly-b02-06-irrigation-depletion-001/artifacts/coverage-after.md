# Coverage After

Ran: delegated valid workspace-instrumented measurement against source SHA-256
`3e37ccb9bf45fa7dc2169597a20fd705b5c2faac442b131588c9d94bdaca8399`.
The root-only command was recorded but has empty dependency LCOV; the accepted
workspace-instrumented command ran the same 21-test contract suite and exited
0.

Production coverage is `1023/1108` direct DA lines (92.329%) and `1128/1231`
unique source-region union (91.633%). Four functions have CRAP line coverage
below 75%, but their source-range region coverage is 75% or above; no function
is below the ADR-0021 region floor.

LCOV SHA-256: `dd2330850017791327e783a63f5eef575ecad4dd5fbba60abfd952a8b389f990`.
LLVM JSON SHA-256:
`f6709dc5e476e0671c140ec264fb613d6d2baf2dadc3a4ad1715b998ab2dc4c0`.

# Changed-Head Qualification Intent

Active scaffold: `622c4e8084102c884a74556306e481a746f2f4b1`.

Dispatch exactly one forest1 TESTGATE run for the pushed schema-corrected
hardening head using this package. Recovery roots `30026171869-1` and
`30031338388-1` must remain invalidated; the incompatible receipt rejection
must validate in the aggregate receipt, and the current admitted attempt must
proceed without importing either root. Run `30034378700` proved the corrected
schema path but exhausted the former 40 GiB `/t` bound during full-workspace
execution. The changed-head qualification is the single infrastructure retry
against the corrected 56 GiB forest1 tmpfs envelope.

Run `30037453241` proved `/t` remained healthy but exposed the independent
2 GiB `/tmp` cap and non-executable test-temp surface. The terminal
qualification uses the corrected bounded 24 GiB executable `/tmp` mount.

Run `30040042088` then passed 2,304 of 2,305 full-workspace tests; its sole
failure was the stale runner-contract assertion for the former capacity
values. The terminal changed-head qualification includes the corrected exact
mount and memory assertions.

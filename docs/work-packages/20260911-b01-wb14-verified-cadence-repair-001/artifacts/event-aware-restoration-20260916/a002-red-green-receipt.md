# A-002 bounded restorer red/green receipt

Ran under `timeout` (900 seconds), `prlimit` address space 16 GiB,
output-file limit 1 GiB, disabled core dumps, and one test thread. The only
runtime input was the immutable member export
`/workdir/openwepp-experiments/b01-wb14-cadence/corrected-recorder-export-20260916-1`.

The red selector was
`v9_real_consumer_shadow::accepted_publication_chronology_tests::archived_restorer_uses_replayed_terminal_event_tail_and_rejects_wire_poisons`.
Its frozen binary was
`/tmp/openwepp-b01-wb14-cadence-targets/event-aware-restoration-20260916/a002-red-direct-restorer.frozen`
(SHA-256 `e1c078af8b32d93b94d2de702db94dec3ce579036a7fe569e5c2d2c24b98d399`).
It exited 101 after 10.32 seconds with the exact targeted refusal
`Identity("archived publication day chronology")` at the direct restorer call.

After removing only the contradictory comparison of the last positive support
ending owner to the replayed wire ending owner, the same selector passed 1/1.
Its frozen binary was
`/tmp/openwepp-b01-wb14-cadence-targets/event-aware-restoration-20260916/a002-green-restorer.frozen`
(SHA-256 `aa33c7a35c2afbc1f5d992b9e0536ba9796acf55ad5fea0aaf17e67f29455ced`).
It completed in 39.15 seconds, maximum RSS 927,408 KiB. The direct test uses
the canonical archive wire with terminal events and retains typed-wire omitted,
duplicate, and wrong-final-owner poisons. The adjacent chronology control
covers zero, one, and multiple endpoint handoffs plus ordinal/tick/parent/order
poisons.

Raw stdout, stderr, and `/usr/bin/time -v` output are retained as
`a002-red-direct.*` and `a002-green.*`. The prior `a002-red.*` fixture setup
failed before reaching the restorer and remains a non-acceptance setup failure.

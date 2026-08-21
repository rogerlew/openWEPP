Status: complete
Evidence mode: Ran

Ran: The ordinary attachment tests cover default-off absence,
restart-before-event, ordinary scheduler terminal consumption, restart after
the event, replay no-op/exactly-once surface state, late publication failure
rollback, and immutable batch delivery.

Ran: The integration handoff test covers shared-carrier reconstruction,
support/tick selection, atomic failure, complete-owner validation, restart
round-trip, and two sequential terminal commits.

Ran: `nix develop --command cargo nextest run --test
snow_stage3_shared_carrier_terminal_handoff_implementation --test
snow_terminal_enthalpy_event_numerics_contract --test
auth11_required_suite_obligation_guards_contract` — 12 passed.

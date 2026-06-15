# CQR10 Kernel Profile Compliance

Status: complete.

Static: CQR10 is kernel-affecting because fixed-date irrigation parser output
is projected into kernel-facing hillslope runtime symbols.

Static: kernel-profile protected boundaries checked:

- no science contract, threshold, unit, symbol, alias, parser, or public API
  authority was changed;
- all fixed-date `irrigation.fixeddate.*` symbol names and scalar meanings were
  preserved;
- all `HillslopeRuntimeInputError` fail-closed variants, field strings, and
  allowed strings in the scoped projection were preserved;
- fixed-date event order, per-event schedule projection, and active-record
  advancement order were preserved;
- furrow `total_duration` and `total_volume` expression order was preserved:
  depletion tail add, active duration add, then `supply_rate * active_duration`
  accumulation.

Ran: focused tests exercised sprinkler projection, furrow projection, header
guards, record-count guard, sprinkler-rate guard, and furrow-window guard.

Ran: integration workspace gate included
`irrig10_fixeddate_contract_vector_couples_irrigation_depth_into_runoff_and_storage`
and parser/runtime seam tests; `cargo test --workspace` exited `0`.

Conclusion: no kernel-profile violation found. No independent conservation
operand reconstruction was required because this package changed only private
runtime projection decomposition, not a conservation publication formula.

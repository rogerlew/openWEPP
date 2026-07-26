# Real Production Consumer Lineage

Evidence class: `Static plus focused run`

Producer:
`ForestCanopyState::advance` returns `ForestCanopyDailyResult` in
`openwepp-plant-phenology`.

In-memory handoff:
`DirectProductionDayInputBuilder::native_forest_growth_state_for_build` retains
the complete daily result and projects its canopy into the typed growth state,
surface-litter input, snow canopy argument, interception inputs, ET inputs,
frost residue context, and erosion inputs.

Runner handoff:
`DirectProductionDayInputBuilder::build` supplies
`DirectPublicationDayInput` to
`run_publication_stream_with_interleaved_day_inputs_and_day_frames`.

Real downstream consumption:
the production executor computes the day frame, including growth,
interception, ET, snow/frost, decomposition, runoff, and erosion. Only in the
post-execution consumer callback does
`maybe_write_canopy_research_trace` combine the retained producer record with
the actual `DirectDayFrame` consumer operands/results.

Negative compatibility proof:
the selected path is `DirectExecutorMode::ProductionDirect`; no deleted
skeleton/shadow publication mode or legacy plant-output parser supplies a
trace value. The existing focused native-forest test executes this exact
production path and independently compares native canopy values with the real
growth, snow, WB15, ET, decomposition, frost, and erosion consumers.

The optional trace is write-only diagnostics. No trace value is read by the
runner, orchestrator, kernel, or output publisher.

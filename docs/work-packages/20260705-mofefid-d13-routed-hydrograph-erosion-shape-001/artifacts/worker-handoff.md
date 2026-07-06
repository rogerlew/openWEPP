# Worker Handoff

Status: **COMPLETE**.

## D13 Result

D13 closes the active-candidate routed-hydrograph erosion hourly-shape
consumer path. When the explicit D13 selector is set to `RoutedHydrograph`,
the Wave-1 erosion substrate consumes the routed hydrograph shape rather than
the DC01 source-shape weights, and malformed/missing candidate surfaces fail
closed.

Default/off construction remains on `Dc01SourceShape`; D13 does not activate
production Lane D routing.

## Follow-On Ownership

- D10 remains responsible for the Case-4 shock-numerics/source-authority hold.
- D14 owns Lane D runtime profiling and behavior-preserving optimization.
- D15 owns the opt-in production activation selector, including proof that the
  production active path supplies `routed_hydrograph_runoff_fraction` to the
  D13 erosion consumer and that protected default/off outputs remain byte-flat.
- D16 owns any default-promotion policy after opt-in activation evidence.

## Explicit Non-Claims

- No DC01 disable or routing-owns-water flip was performed.
- No D10 numerical-method correction was implemented.
- No D11 friction-source/default-promotion change was made.
- No D12 melt-limb hourly-source rule was changed.
- No D14 profiling/optimization was added.
- No HBP/pass schema change was made.
- No surrogate, provisional, proxy, or heuristic process physics was added.

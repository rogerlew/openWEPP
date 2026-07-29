# Execution Control Contract

Evidence class: `Static: frozen before result-bearing execution`

The exact design, units, objectives, tolerance, enumeration, failure handling,
and stopping rule are specified in `package.md`. The producer may emit only
declared daily inputs and computed runtime states. The independent reconstructor
must not read producer-computed states when deriving its expected values.

All five `DirectResiduePartitionInputs` values are frozen at zero and labeled
`ASSUMED_FOR_EXECUTION`: standing mass, flat offset, buried mass, cover
fraction, and interrill weight.

For one-based day `t`, `I_t` is the synthetic surface-litter pulse on day 280
and zero otherwise:

```text
tave_t = midpoint(tmax_t, tmin_t)
t1_t = (tave_t + 6.1)^2
temperature_factor_t = t1_t * (2 * 1528.81 - t1_t) / 1528.81^2
standing_water_factor_t = 1 when precipitation_t >= 0.004 m
flat_water_factor_t = water_stress_fraction_t
environment_index_t = min(temperature_factor_t, flat_water_factor_t)
decay_factor_t = exp(-environment_index_t * k)
M_t = (M_(t-1) + I_t) * decay_factor_t
```

`M_0=0.20 kg m^-2` precedes year 1 day 1. Emitted `M_t` becomes
the next seed across days and years. Source enters before same-day decay. No
action occurs. Year-20 day-365 post-decay state is the terminal sample. The
reconstructor derives all factors from retained inputs and reports the first
divergent year, day, and operand.

Every candidate is retained. Non-finite or negative inputs are expected typed
failures, never imputed. Zero rate is an allowed direct-kernel degenerate case,
not a native projection claim. Results
cannot widen the axes, change the truth vector, introduce weights, or select a
preferred empirical source or decay value.

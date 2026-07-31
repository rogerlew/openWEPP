# Thin-Pack Authority Reconciliation

Evidence: `Static`

Libsnobal `_calc_layers.c` sets `layer_count=0` when total snow mass is less
than or equal to `tstep_info[SMALL_TSTEP].threshold`; the default in `snobal.h`
is exactly `1 kg m^-2`. `_below_thold.c` and `_divide_tstep.c` establish that
this is the terminal resolved-layer boundary after the `60/10/1 kg m^-2`
cadence hierarchy.

The source has a second, distinct branch. When total mass is resolved but the
lower layer has mass strictly below `1 kg m^-2`, `_calc_layers.c` collapses to
one layer and continues. Lower-layer equality remains a two-layer solve. The
openWEPP translation therefore preserves both the source ordering and the
different `<=`/`<` comparison sides: total mass branches before partition;
sub-resolution lower mass collapses only the thermal partition.

Libsnobal `_adj_layers.c` then converts residual snow mass to water and clears
cold content. OpenWEPP cannot import that phase disposition because the
campaign contract assigns snow existence, melt, and liquid routing to CoE.
The admissible translation is therefore narrower:

- retain CoE snow mass and the persistent layer state;
- preserve existing mass-proportional cold content;
- treat total mass at/below the threshold as unresolved by Stage 3;
- apply no further Stage 3 exchange until total mass again exceeds the exact
  resolved-pack boundary;
- collapse a strictly sub-threshold lower thermal volume into a conservative
  whole-pack solve without deleting mass or cold content; and
- retain typed guards for invalid states above the boundary.

This is a fixed model-domain rule, not a fitted limiter. It avoids the rejected
alternatives of clamping temperature, inventing a positive vapor-pressure
epsilon, deleting cold content, or converting the remnant to meltwater.

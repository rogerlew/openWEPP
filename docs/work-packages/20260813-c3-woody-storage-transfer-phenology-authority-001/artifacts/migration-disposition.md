# Migration Disposition

Static: V6-to-V7 is an identity transition because the serialized state shape
is unchanged. Seasonal-deciduous migration preserves every non-identity byte
and recomputes only model/configuration/state identities. It never executes a
phenology event.

Evergreen migration additionally validates exact `f_cur=1` and zero-class
storage/transfer C/N across all six tissues. Violations yield an exhaustive
unresolved-field report; no value is moved, cleared, averaged, or synthesized.


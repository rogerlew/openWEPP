# Selected Storage-Transfer Rule

Static: At one accepted seasonal-deciduous `Dormant -> Onset` upward crossing,
for each of six tissues and independently for carbon and nitrogen:

```text
Prep = 0.5 * beginning_storage
prepared_storage = beginning_storage - Prep
prepared_transfer = beginning_transfer + Prep
```

The operation consumes immutable beginning storage, adds to existing transfer,
and changes neither display nor total vegetation C/N. It emits no atmospheric,
mineral-N, litter, CWD, dry-material, or respiration flux.


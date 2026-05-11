# Science Contracts

This directory hosts openWEPP's working copies and pointers to the authoritative science contracts maintained in wepp-palimpsest.

## Authoritative source

`wepp-palimpsest/docs/science-contracts/` is the upstream registry. Each kernel in openWEPP corresponds to one or more `SC-DOMAIN-NNN.md` contracts there.

Stable references use:

```
SC-<DOMAIN>-<NNN>#INV-<DOMAIN>-<NNN>
```

## openWEPP usage

- Kernel-port work packages cite the upstream contract by ID.
- A Rust kernel is ported only after its upstream contract has reached `active` maturity (per [../decisions/0002-clean-room-model.md](../decisions/0002-clean-room-model.md)).
- openWEPP does not create new science contracts. If a port surfaces an under-specified state surface, the gap is documented and routed back to wepp-palimpsest for contract authoring.

## Tolerance bounds

Semantic-parity tolerance bounds for the openWEPP oracle harness are part of the contract. Per [../decisions/0003-parity-semantic-not-bit.md](../decisions/0003-parity-semantic-not-bit.md), contracts without explicit tolerance bounds block port acceptance.

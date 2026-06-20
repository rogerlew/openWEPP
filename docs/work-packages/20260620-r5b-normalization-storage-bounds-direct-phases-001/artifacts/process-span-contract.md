# Process Span Contract

Static:

R5B adds two direct day-level phase spans before the existing R4 hydrology
direct spans:

1. `Normalization`: normalize direct forcing/transfer/storage context into
   downstream operands and shadow projection.
2. `StorageBounds`: validate normalized direct storage/domain constraints and
   expose downstream bounded-storage operands.

The existing R3A input-accounting span remains available for focused tests but
is no longer the executor lifecycle's normalization implementation.

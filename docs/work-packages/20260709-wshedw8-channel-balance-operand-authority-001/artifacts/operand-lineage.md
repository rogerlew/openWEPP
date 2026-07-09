# Operand Lineage

Status: `EXECUTED-COMPLETE`
Evidence: `Static + ran`

| Public field | Units | Typed source | Authority | Notes |
|---|---:|---|---|---|
| `Inflow (m^3)` | m^3 | `RoutedChannelState.channel_inflow_m3` -> `WatershedPublicationFrame.channel_inflow_m3` | `SC-SYSTEM-001` WSHED-W8 amendment | Direct lane source is WS11 `runvol_case`; must not alias watershed runoff/outflow. |
| `Outflow (m^3)` | m^3 | `RoutedChannelState.channel_outflow_m3` -> `WatershedPublicationFrame.channel_outflow_m3` | `SC-SYSTEM-001` WSHED-W8 amendment | Must not alias inflow/runoff. |
| `Storage (m^3)` | m^3 | `RoutedChannelState.channel_storage_m3` -> `WatershedPublicationFrame.channel_storage_m3` | `SC-SYSTEM-001` WSHED-W8 amendment | Current direct lane carries explicit zero until storage physics owns a nonzero routed term. |
| `Baseflow (m^3)` | m^3 | `RoutedChannelState.channel_baseflow_m3` -> `WatershedPublicationFrame.channel_baseflow_m3` | `SC-GWBASEFLOW-001` + `SC-SYSTEM-001` WSHED-W8 amendment | Existing M-T2 field; included in channel-balance publication surface. |
| `Loss (m^3)` | m^3 | `RoutedChannelState.channel_loss_m3` -> `WatershedPublicationFrame.channel_loss_m3` | `SC-SYSTEM-001` WSHED-W8 amendment + `SC-ROUTE-001` transmission-loss follow-on authority | Current direct lane carries explicit zero until `tl`/transmission-loss physics owns a nonzero routed term. |
| `Balance (m^3)` | m^3 | writer reconstruction from public operands | `SC-SYSTEM-001` WSHED-W8 amendment | `Inflow - Outflow - Loss - Storage`; null if any required operand is null. |

## Non-Alias Evidence

- `typed_publication_writer_reads_publication_frame_directly` writes
  `Inflow = 31`, `Outflow = 20`, `Storage = 2`, `Loss = 3`, and proves
  `Balance = 6`; this fails if balance omits storage or if outflow aliases
  inflow/runoff.
- `typed_publication_projects_non_aliased_channel_balance_operands` executes the
  real typed dispatch, selects a WS11 wave branch with distinct inflow/outflow,
  and proves publication fields equal routed state while public
  `runoff_volume_m3` remains routed outflow.
- `typed_publication_writer_keeps_unavailable_operands_null` proves default
  typed publication does not synthesize unavailable channel-balance operands.

# Scheduler And Cadence Decision

Status: `PARTIAL / V2 internal column order implemented; full cadence pending`

One caller-supplied finite positive `dt_s` binds one immutable forcing/state snapshot. Order is identity/schema/topology validation; persistent T10 and GSI edge state; top-down radiation; interception carry; coupled gas/energy/hydraulic Stage A; water authorization and cap-active Stage C re-solve; turnover/retranslocation; potential then final N demand; N authorization/final use; allocation and receiver proposals; independent water/energy/C/N/DM reconstruction; owner validation; one atomic commit. Rate-to-amount conversion occurs only at ledger boundaries using that `dt_s`.

Increment 2A implements only the column portion of that frozen order: validate
exact V2 configuration/state and tile forcing; sort tiles canonically and each
column top-to-bottom; solve one occupancy; route its accepted throughfall plus
both drainage terms before solving the descendant; bypass stemflow to same-tile
ground; close occupancy, column, then weighted stand. Potential and final kinds
share this routing engine, but their real coupled solvers are not yet wired.

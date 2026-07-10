# Hold Legitimacy Audit

Status: `EXECUTED-HOLD-MISSING-CHANNEL-HOURLY-SEDIMENT-SEQUENCING-AUTHORITY`

Evidence mode: `Static` plus targeted `Ran` source searches.

## Exact Declared Boundary

The package explicitly permits hold when a required channel branch needs
missing baseline/contract physics. Pinned baseline `chnrt` routes sediment once
per event after water routing and has no per-interval class state. Current
`SC-ROUTE-001#INV-ROUTE-005(e)` preserves a single-rate scope limit until
channels carry hourly surfaces. ADR-0036 requires paired timing consumption but
does not define interval WS18-WS26 mutation/carry semantics.

## In-Package Route Considered

The audit considered:

- projecting `V_h/S_h` conservatively to the channel grid;
- using ADR-0036's uniform event-fraction reconstruction for local class mass;
- porting baseline `wshchr` for water branches 3-5;
- invoking the existing channel sediment solver once per hour or `dtchr`
  interval and publishing each result downstream.

The final step is not authority-backed. It leaves undefined whether channel
width/profile/bed state resets or carries, how detachment/deposition interacts
with water-routing storage, and which discharge/interval basis controls the
quasi-steady solve. Choosing any answer here would create surrogate physics.

## Why W11 Cannot Close

Water-only implementation cannot satisfy W11's current sediment consumer and
conservation gates. A scalar or uniformly redistributed channel output would
violate the real-consumer rule. The correct current behavior is the existing
typed M-T3 failure when hourly hillslopes meet dependency nodes.

## First Action to Lift

Close `WSHED-W11-HOLD-001`: execute WSHED-W11A to acquire and ratify canonical
per-interval channel sediment sequencing/state authority, including the bed/
profile carry rule, supported temporal quantum, class continuity, and guard/
tolerance/test obligations. Then resume W11 from its contract-first phase.

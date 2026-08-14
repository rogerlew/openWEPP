# Production Byte Invariance

Status: `focused structural and canonical-projection PASS`

Evidence class: `Static design`

`DirectRunFrame` currently supplies complete structural `Clone + PartialEq`
but no canonical serialization. Child 2 will not mislabel debug text or a
partial layer map as complete production bytes.

Closure requires both:

- immutable-borrow plus whole-structure equality for the actual production
  frame; and
- deterministic canonical serialization of the bounded real-hydrology
  arbitration projection, with sensitivity to every projected field.

Runner-level production output bytes remain a later Child-4 real-consumer
gate. This bounded package does not claim a production run occurred.

Focused evidence compares the immutable original `DirectRunFrame` by complete
structural equality and compares canonical beginning hydrology snapshot bytes
across adapter, arbitration and candidate. The projection covers run/day/OFE
identity, transaction, owner, interval, lane topology/area, aggregate water and
transfer state, ordered layer IDs, and all twelve production layer fields. It
does not claim to serialize unrelated production publication, growth, or
winter-column fields; those remain protected by whole-frame structural
equality.

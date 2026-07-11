# Security impact

Status: pass
Evidence mode: Static

No authentication, authorization, unsafe code, external command, network,
deserialization trust boundary, or secret handling changed. Fail-closed raw
cardinality validation reduces malformed-input ambiguity. Resource bounds are
unchanged because the parser already materialized the source ID record and
normalization remains capped by the canonical channel maximum.

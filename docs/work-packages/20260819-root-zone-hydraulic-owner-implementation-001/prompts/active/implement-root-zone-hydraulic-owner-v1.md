# Implement Root-Zone Hydraulic Owner V1

Do not start until `20260819-root-zone-hydraulic-owner-authority-001` releases
`OPENWEPP_ROOT_ZONE_HYDRAULIC_OWNER_V1`. Preserve V10 equations, restart wire,
selectors, defaults and production outputs. Implement a private receipt from
current staged owners every interval; caller templates may validate but never
drive physics. The authority blocker was released at local commit `de2b078fa`;
keep this prompt active through implementation, review, terminal verification,
and Child-4 closure.

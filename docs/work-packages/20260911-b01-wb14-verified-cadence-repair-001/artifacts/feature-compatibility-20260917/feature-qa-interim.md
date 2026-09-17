Static interim QA, /root/feature_qa; no terminal approval.

Current candidate differs from frozen baseline in10 Rust files only, chiefly cfg(test) serde derives and three tests gated by persisted-restart-v1 plus one dual-feature importer. C1 stub absent and C2 DirectGrowthActiveContext derive remains feature-only with snake_case intact. Newly gated tests call restart-only construction/admission; dual-feature importer calls both feature-specific pathways. Need final source and fresh checks before acceptance.

QA1 / C2 cluster: an intermediate DirectGrowthAction test Serialize derive lacked matching test-enabled serde(rename_all="snake_case"). Every added test serialization condition must mirror wire-affecting serde attributes. The implementer reported reverting that incomplete cascade; terminal verification was not completed.

No terminal inventory, behavioral execution, lint attribution or acceptance review is claimed. Interrupted at hard budget stop before resumed work; no source or scripts authored by this reviewer.

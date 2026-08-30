#[test]
fn retired_snow_stage3_shadow_is_not_compiled_or_exported_in_production() {
    let runtime = include_str!("../direct_runtime.rs");
    assert!(
        runtime.contains("#[cfg(test)]\nmod snow_stage3_shadow;"),
        "the retired pre-V11 attachment module must remain unit-test-only"
    );
    assert!(
        runtime.contains("#[cfg(test)]\npub use snow_stage3_shadow::"),
        "retired attachment DTOs must not remain in the production API"
    );

    let core_frames = include_str!("00_core_frames.rs");
    let retired_field = core_frames
        .split("pub snow_stage3_shadow:")
        .next()
        .expect("retired test-only attachment field must remain visible to historical tests");
    assert!(
        retired_field.ends_with("#[cfg(test)]\n    "),
        "the retired attachment state must not be present in production DirectRunFrame"
    );
}

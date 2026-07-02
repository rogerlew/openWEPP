use std::collections::BTreeSet;
use std::error::Error as _;
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_input_contract::parsers::chaninp::{
    ChaninpParseOptions, ChaninpParseOutcome, parse_chaninp_from_path,
};
use openwepp_input_contract::parsers::slope::{SlopeParserOptions, parse_slope_file};
use openwepp_input_contract::parsers::watershed_channel::{
    WatershedChannelParseMode, WatershedChannelParseOptions, parse_watershed_channel_from_path,
};
use openwepp_input_contract::parsers::watershed_impoundment::{
    ParseMode as WatershedImpoundmentParseMode, WatershedImpoundmentParseOptions,
    parse_watershed_impoundment_from_path,
};
use openwepp_input_contract::parsers::watershed_structure::{
    DatverSource, ParseMode, WatershedStructureFile, WatershedStructureParseError,
    WatershedStructureParseOptions, WatershedStructureWarningCode,
    parse_watershed_structure_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/infile/watershed_structure")
        .join(name)
}

fn watershed_fixture_root(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/watershed")
        .join(name)
}

fn carnivorous_adobo_fixture_root() -> PathBuf {
    watershed_fixture_root("carnivorous-adobo")
}

fn onshore_xenophobia_fixture_root() -> PathBuf {
    watershed_fixture_root("onshore-xenophobia")
}

fn assert_repo_local_watershed_fixture_path(path: &Path, fixture_name: &str) {
    let path_text = path.to_string_lossy();
    assert!(
        path_text.contains(&format!("tests/fixtures/watershed/{fixture_name}")),
        "fixture path must be the committed openWEPP fixture path, got {path_text}"
    );
    assert!(
        !path_text.contains("/wc1/"),
        "persistent gate path must not read /wc1 directly: {path_text}"
    );
    assert!(
        !path_text.contains("wepppy"),
        "persistent gate path must not read wepppy directly: {path_text}"
    );
}

fn assert_repo_local_fixture_path(path: &Path) {
    assert_repo_local_watershed_fixture_path(path, "carnivorous-adobo");
}

fn assert_watershed_structure_error_display(error: &WatershedStructureParseError, expected: &str) {
    assert_eq!(error.to_string(), expected);
}

fn assert_carnivorous_adobo_inventory() -> PathBuf {
    let fixture_root = carnivorous_adobo_fixture_root();
    assert_repo_local_fixture_path(&fixture_root);

    let runs = fixture_root.join("runs");
    assert!(runs.is_dir(), "fixture runs directory must be committed");
    let manifest = fixture_root.join("input-manifest.sha256");
    assert!(
        manifest.is_file(),
        "fixture checksum manifest must be committed"
    );

    let manifest_text =
        fs::read_to_string(&manifest).expect("fixture checksum manifest should be readable");
    assert_eq!(
        manifest_text.lines().count(),
        208,
        "manifest should cover the committed input/runfile inventory"
    );
    assert!(!manifest_text.contains("/wc1/"));
    assert!(!manifest_text.contains("wepppy"));

    let committed_files = fs::read_dir(&runs)
        .expect("fixture runs directory should be readable")
        .count();
    assert_eq!(committed_files, 208);

    for hillslope_id in 1..=32 {
        for extension in ["run", "man", "slp", "cli", "sol", "source.run"] {
            let path = runs.join(format!("p{hillslope_id}.{extension}"));
            assert!(
                path.is_file(),
                "missing committed hillslope input {}",
                path.display()
            );
            assert_repo_local_fixture_path(&path);
        }
    }

    for required in [
        "case.run",
        "pw0.run",
        "pw0.str",
        "pw0.chn",
        "pw0.imp",
        "pw0.slp",
        "pw0.cli",
        "pw0.sol",
        "pw0.man",
        "chan.inp",
        "chntyp.txt",
        "gwcoeff.txt",
        "pmetpara.txt",
        "snow.txt",
        "tc.txt",
        "wepp_ui.txt",
    ] {
        let path = runs.join(required);
        assert!(
            path.is_file(),
            "missing committed watershed input {}",
            path.display()
        );
        assert_repo_local_fixture_path(&path);
    }

    runs
}

fn assert_fixture_contents_do_not_embed_operator_paths(runs: &Path) {
    for entry in fs::read_dir(runs).expect("fixture runs directory should be readable") {
        let path = entry.expect("fixture entry should be readable").path();
        assert_ne!(
            path.extension().and_then(|value| value.to_str()),
            Some("err")
        );
        assert_ne!(
            path.extension().and_then(|value| value.to_str()),
            Some("tif")
        );

        let content =
            fs::read_to_string(&path).expect("committed text fixture input should be readable");
        assert!(
            !content.contains("/wc1/"),
            "committed fixture input must not embed /wc1 dependency: {}",
            path.display()
        );
        assert!(
            !content.contains("wepppy"),
            "committed fixture input must not embed wepppy dependency: {}",
            path.display()
        );
    }
}

fn parse_carnivorous_adobo_structure(runs: &Path) -> (WatershedStructureFile, BTreeSet<i32>) {
    let structure_path = runs.join("pw0.str");
    let structure_line_count = fs::read_to_string(&structure_path)
        .expect("watershed structure should be readable")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    let mut structure_options =
        WatershedStructureParseOptions::compatibility(32, structure_line_count - 1);
    structure_options.expected_channel_count = Some(15);
    structure_options.expected_impoundment_count = Some(0);
    let structure = parse_watershed_structure_from_path(&structure_path, structure_options)
        .expect("committed carnivorous-adobo structure should parse");

    assert_eq!(structure.datver_source, DatverSource::ExplicitHeader);
    assert_eq!(structure.nhill, 32);
    assert_eq!(structure.rows.len(), 15);
    assert_eq!(structure.summary.channel_count, 15);
    assert_eq!(structure.summary.impoundment_count, 0);
    assert_eq!(structure.summary.max_hillslope_ref, 32);
    assert!(structure.warnings.is_empty());

    let channel_ids: BTreeSet<i32> = structure
        .rows
        .iter()
        .filter(|row| row.element_type_code == 2)
        .map(|row| row.element_id)
        .collect();
    assert_eq!(channel_ids, (33..=47).collect::<BTreeSet<_>>());

    (structure, channel_ids)
}

fn assert_carnivorous_adobo_channel_inputs(
    runs: &Path,
    structure: &WatershedStructureFile,
    channel_ids: &BTreeSet<i32>,
) {
    let channel = parse_watershed_channel_from_path(
        runs.join("pw0.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            expected_channel_count: Some(structure.summary.channel_count),
            chan_inp_present: true,
            tcr_overlay_present: false,
            slplst_override: None,
        },
    )
    .expect("committed carnivorous-adobo watershed channel should parse");
    assert_eq!(channel.nchan, 15);
    assert_eq!(channel.ipeak, 4);
    assert!(channel.sidecar_required);

    let chaninp = parse_chaninp_from_path(
        runs.join("chan.inp"),
        ChaninpParseOptions::strict(channel.ipeak, channel.nchan),
        channel_ids,
    )
    .expect("committed carnivorous-adobo chan.inp should parse");
    assert_eq!(chaninp.parse_outcome, ChaninpParseOutcome::ParsedBranch);
    assert!(chaninp.chaninp_required);
    let chaninp_options = chaninp
        .options
        .expect("parsed chan.inp branch should export options");
    assert_eq!(chaninp_options.nchnum_norm, 15);
    assert_eq!(chaninp_options.ichnum_norm.len(), 15);
}

fn assert_carnivorous_adobo_impoundment_and_slope(runs: &Path, structure: &WatershedStructureFile) {
    let mut impoundment_options = WatershedImpoundmentParseOptions {
        mode: WatershedImpoundmentParseMode::Compatibility,
        ..WatershedImpoundmentParseOptions::default()
    };
    impoundment_options.expected_structural_count = Some(structure.summary.impoundment_count);
    let impoundment =
        parse_watershed_impoundment_from_path(runs.join("pw0.imp"), impoundment_options)
            .expect("committed carnivorous-adobo impoundment file should parse");
    assert_eq!(impoundment.declared_count, 0);
    assert_eq!(impoundment.parsed_count, 0);

    let slope = parse_slope_file(&runs.join("pw0.slp"), SlopeParserOptions::compatibility())
        .expect("committed carnivorous-adobo watershed slope should parse");
    assert_eq!(slope.ofe_count, 15);
}

fn assert_onshore_xenophobia_inventory() -> PathBuf {
    let fixture_root = onshore_xenophobia_fixture_root();
    assert_repo_local_watershed_fixture_path(&fixture_root, "onshore-xenophobia");

    let runs = fixture_root.join("runs");
    assert!(runs.is_dir(), "fixture runs directory must be committed");
    let manifest = fixture_root.join("input-manifest.sha256");
    assert!(
        manifest.is_file(),
        "fixture checksum manifest must be committed"
    );

    let manifest_text =
        fs::read_to_string(&manifest).expect("fixture checksum manifest should be readable");
    assert_eq!(
        manifest_text.lines().count(),
        7_847,
        "manifest should cover the full committed onshore inventory"
    );
    assert!(!manifest_text.contains("/wc1/"));
    assert!(!manifest_text.contains("wepppy"));

    for hillslope_id in 1..=1_305 {
        for extension in ["run", "man", "slp", "cli", "sol", "source.run"] {
            let path = runs.join(format!("p{hillslope_id}.{extension}"));
            let metadata = fs::symlink_metadata(&path)
                .unwrap_or_else(|error| panic!("missing {}: {error}", path.display()));
            if extension == "cli" {
                assert!(
                    metadata.file_type().is_symlink(),
                    "climate input should preserve source hard-link sharing as symlink: {}",
                    path.display()
                );
            } else {
                assert!(
                    metadata.is_file(),
                    "missing committed hillslope input {}",
                    path.display()
                );
            }
            assert_repo_local_watershed_fixture_path(&path, "onshore-xenophobia");
        }
    }

    let shared_climate = runs.join("shared/onshore-xenophobia.cli");
    assert!(
        shared_climate.is_file(),
        "shared canonical climate file must be committed"
    );
    assert_eq!(
        fs::metadata(&shared_climate)
            .expect("shared climate metadata should be readable")
            .len(),
        2_594_125
    );

    let pw0_cli = runs.join("pw0.cli");
    let pw0_cli_metadata =
        fs::symlink_metadata(&pw0_cli).expect("pw0.cli metadata should be readable");
    assert!(pw0_cli_metadata.file_type().is_symlink());

    for required in [
        "case.run",
        "pw0.run",
        "pw0.str",
        "pw0.chn",
        "pw0.imp",
        "pw0.slp",
        "pw0.cli",
        "pw0.sol",
        "pw0.man",
        "chan.inp",
        "chntyp.txt",
        "gwcoeff.txt",
        "pmetpara.txt",
        "snow.txt",
        "tc.txt",
        "wepp_ui.txt",
    ] {
        let path = runs.join(required);
        assert!(path.exists(), "missing watershed input {}", path.display());
        assert_repo_local_watershed_fixture_path(&path, "onshore-xenophobia");
    }

    runs
}

fn assert_onshore_contents_do_not_embed_operator_paths(runs: &Path) {
    for entry in walk_fixture_inputs(runs) {
        let content =
            fs::read_to_string(&entry).expect("committed text fixture input should be readable");
        assert!(
            !content.contains("/wc1/"),
            "committed fixture input must not embed /wc1 dependency: {}",
            entry.display()
        );
        assert!(
            !content.contains("wepppy"),
            "committed fixture input must not embed wepppy dependency: {}",
            entry.display()
        );
    }
}

fn walk_fixture_inputs(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut inputs = Vec::new();
    while let Some(path) = pending.pop() {
        for entry in fs::read_dir(&path).expect("fixture directory should be readable") {
            let path = entry.expect("fixture entry should be readable").path();
            let metadata =
                fs::symlink_metadata(&path).expect("fixture metadata should be readable");
            if metadata.is_dir() {
                pending.push(path);
            } else if metadata.is_file() {
                assert_ne!(
                    path.extension().and_then(|value| value.to_str()),
                    Some("err")
                );
                assert_ne!(
                    path.extension().and_then(|value| value.to_str()),
                    Some("tif")
                );
                inputs.push(path);
            }
        }
    }
    inputs
}

fn parse_onshore_xenophobia_structure(runs: &Path) -> (WatershedStructureFile, BTreeSet<i32>) {
    let structure_path = runs.join("pw0.str");
    let structure_line_count = fs::read_to_string(&structure_path)
        .expect("watershed structure should be readable")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    let mut structure_options =
        WatershedStructureParseOptions::compatibility(1_305, structure_line_count - 1);
    structure_options.expected_channel_count = Some(544);
    structure_options.expected_impoundment_count = Some(0);
    let structure = parse_watershed_structure_from_path(&structure_path, structure_options)
        .expect("committed onshore-xenophobia structure should parse");

    assert_eq!(structure.datver_source, DatverSource::ExplicitHeader);
    assert_eq!(structure.nhill, 1_305);
    assert_eq!(structure.rows.len(), 544);
    assert_eq!(structure.summary.channel_count, 544);
    assert_eq!(structure.summary.impoundment_count, 0);
    assert_eq!(structure.summary.max_hillslope_ref, 1_305);
    assert!(structure.warnings.is_empty());

    let channel_ids: BTreeSet<i32> = structure
        .rows
        .iter()
        .filter(|row| row.element_type_code == 2)
        .map(|row| row.element_id)
        .collect();
    assert_eq!(channel_ids.len(), 544);
    assert_eq!(channel_ids.first().copied(), Some(1_306));
    assert_eq!(channel_ids.last().copied(), Some(1_849));

    (structure, channel_ids)
}

fn assert_onshore_channel_inputs(
    runs: &Path,
    structure: &WatershedStructureFile,
    channel_ids: &BTreeSet<i32>,
) {
    let channel = parse_watershed_channel_from_path(
        runs.join("pw0.chn"),
        WatershedChannelParseOptions {
            mode: WatershedChannelParseMode::Compatibility,
            expected_channel_count: Some(structure.summary.channel_count),
            chan_inp_present: true,
            tcr_overlay_present: false,
            slplst_override: None,
        },
    )
    .expect("committed onshore-xenophobia watershed channel should parse");
    assert_eq!(channel.nchan, 544);
    assert_eq!(channel.ipeak, 4);
    assert!(channel.sidecar_required);

    let chaninp = parse_chaninp_from_path(
        runs.join("chan.inp"),
        ChaninpParseOptions::compatibility(channel.ipeak, channel.nchan),
        channel_ids,
    )
    .expect("committed onshore-xenophobia chan.inp should parse");
    assert_eq!(chaninp.parse_outcome, ChaninpParseOutcome::ParsedBranch);
    assert!(chaninp.chaninp_required);
    let chaninp_options = chaninp
        .options
        .expect("parsed chan.inp branch should export options");
    assert_eq!(chaninp_options.nchnum_norm, 1);
    assert_eq!(chaninp_options.ichnum_norm, [1_849]);
}

fn assert_onshore_impoundment_and_slope(runs: &Path, structure: &WatershedStructureFile) {
    let mut impoundment_options = WatershedImpoundmentParseOptions {
        mode: WatershedImpoundmentParseMode::Compatibility,
        ..WatershedImpoundmentParseOptions::default()
    };
    impoundment_options.expected_structural_count = Some(structure.summary.impoundment_count);
    let impoundment =
        parse_watershed_impoundment_from_path(runs.join("pw0.imp"), impoundment_options)
            .expect("committed onshore-xenophobia impoundment file should parse");
    assert_eq!(impoundment.declared_count, 0);
    assert_eq!(impoundment.parsed_count, 0);

    let slope = parse_slope_file(&runs.join("pw0.slp"), SlopeParserOptions::compatibility())
        .expect("committed onshore-xenophobia watershed slope should parse");
    assert_eq!(slope.ofe_count, 544);
}

#[test]
fn strict_mode_parses_valid_watershed_structure() {
    let mut options = WatershedStructureParseOptions::strict(2, 2);
    options.expected_channel_count = Some(1);
    options.expected_impoundment_count = Some(1);

    let parsed =
        parse_watershed_structure_from_path(fixture_path("strict_valid_two_rows.str"), options)
            .expect("strict mode valid .str file should parse");

    assert_eq!(parsed.datver_source, DatverSource::ExplicitHeader);
    assert!((parsed.datver - 94.301).abs() < 1e-12);
    assert_eq!(parsed.rows.len(), 2);
    assert_eq!(parsed.summary.channel_count, 1);
    assert_eq!(parsed.summary.impoundment_count, 1);
    assert_eq!(parsed.summary.max_hillslope_ref, 2);
    assert_eq!(parsed.rows[0].element_id, 3);
    assert_eq!(parsed.rows[1].element_id, 4);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn carnivorous_adobo_committed_fixture_is_repo_local_32_hillslope_gate() {
    let runs = assert_carnivorous_adobo_inventory();
    assert_fixture_contents_do_not_embed_operator_paths(&runs);
    let (structure, channel_ids) = parse_carnivorous_adobo_structure(&runs);
    assert_carnivorous_adobo_channel_inputs(&runs, &structure, &channel_ids);
    assert_carnivorous_adobo_impoundment_and_slope(&runs, &structure);
}

#[test]
fn onshore_xenophobia_committed_fixture_is_full_1305_hillslope_gate() {
    let runs = assert_onshore_xenophobia_inventory();
    assert_onshore_contents_do_not_embed_operator_paths(&runs);
    let (structure, channel_ids) = parse_onshore_xenophobia_structure(&runs);
    assert_onshore_channel_inputs(&runs, &structure, &channel_ids);
    assert_onshore_impoundment_and_slope(&runs, &structure);
}

#[test]
fn compatibility_mode_accepts_legacy_no_datver_and_warns() {
    let mut options = WatershedStructureParseOptions::compatibility(2, 2);
    options.expected_channel_count = Some(1);
    options.expected_impoundment_count = Some(1);

    let parsed =
        parse_watershed_structure_from_path(fixture_path("compat_no_datver_valid.str"), options)
            .expect("compat mode should accept no-datver legacy path");

    assert_eq!(parsed.datver_source, DatverSource::LegacyCompatNoDatver);
    assert_eq!(parsed.warnings.len(), 1);
    assert_eq!(
        parsed.warnings[0].code,
        WatershedStructureWarningCode::StrW001
    );
}

#[test]
fn watershed_structure_parse_error_display_strings_are_stable_for_input_shape() {
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::InputOpenError {
            path: PathBuf::from("missing.str"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "boom"),
        },
        "STR-E-000 failed to open watershed structure file 'missing.str': boom",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::TokenParseError {
            line: 8,
            field: "datver",
            token: "bad".to_string(),
        },
        "STR-E-001 line 8: failed to parse field 'datver' from token 'bad'",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::RecordArityError {
            line: 9,
            expected: 13,
            found: 12,
        },
        "STR-E-002 line 9: expected 13 token(s), found 12",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::LegacyNoDatverDisallowed {
            line: 1,
            token: "3".to_string(),
        },
        "STR-E-003 line 1: strict mode requires explicit datver, got leading token '3'",
    );
}

#[test]
fn watershed_structure_parse_error_display_strings_are_stable_for_domains() {
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::UnsupportedDatver {
            line: 1,
            datver: 90.0,
            min_supported: 94.301,
        },
        "STR-E-003 line 1: unsupported datver 90; minimum supported 94.301",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::ElementTypeDomainError { line: 2, value: 4 },
        "STR-E-004 line 2: invalid element type code 4; expected 2 or 3",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::DisconnectedElementError {
            line: 3,
            record_index: 2,
        },
        "STR-E-005 line 3: structure row 2 has no non-zero contributors",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::ContributorDomainError {
            line: 4,
            field: "nhleft",
            value: -1,
            expected: "0 or 1..nhill",
        },
        concat!(
            "STR-E-006 line 4: contributor field 'nhleft' has invalid value -1; ",
            "expected 0 or 1..nhill"
        ),
    );
}

#[test]
fn watershed_structure_parse_error_display_strings_are_stable_for_counts() {
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::ChannelCountMismatch {
            expected: 2,
            observed: 1,
        },
        "STR-E-007 channel count mismatch: expected 2, observed 1",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::ImpoundmentCountMismatch {
            expected: 2,
            observed: 1,
        },
        "STR-E-008 impoundment count mismatch: expected 2, observed 1",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::HillslopeCoverageMismatch {
            expected_nhill: 3,
            observed_nhmax: 2,
        },
        "STR-E-009 hillslope coverage mismatch: expected nhill 3, observed nhmax 2",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::NhillContextError { nhill: 0 },
        "STR-E-009 invalid nhill context 0; expected > 0",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::RecordCountMismatch {
            expected: 3,
            observed: 2,
        },
        "STR-E-011 structure row-count mismatch: expected 3, observed 2",
    );
    assert_watershed_structure_error_display(
        &WatershedStructureParseError::InvariantViolation {
            context: "expected_rows topology context is required for row closure",
        },
        concat!(
            "STR-E-010 invariant violation: ",
            "expected_rows topology context is required for row closure"
        ),
    );
}

#[test]
fn watershed_structure_parse_error_source_is_only_input_open_source() {
    let io_error = WatershedStructureParseError::InputOpenError {
        path: PathBuf::from("missing.str"),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "boom"),
    };
    assert!(io_error.source().is_some());

    let non_io_error = WatershedStructureParseError::NhillContextError { nhill: 0 };
    assert!(non_io_error.source().is_none());
}

#[test]
fn strict_mode_rejects_no_datver_path() {
    let err = parse_watershed_structure_from_path(
        fixture_path("compat_no_datver_valid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("strict mode must reject no-datver path");

    assert!(matches!(
        err,
        WatershedStructureParseError::LegacyNoDatverDisallowed { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-003");
}

#[test]
fn strict_mode_rejects_unsupported_datver() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_unsupported_datver_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("unsupported datver must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::UnsupportedDatver { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-003");
}

#[test]
fn strict_mode_rejects_invalid_row_arity() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_arity_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("row arity mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::RecordArityError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-002");
}

#[test]
fn strict_mode_rejects_invalid_element_type_domain() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_element_type_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("invalid element type must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ElementTypeDomainError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-004");
}

#[test]
fn strict_mode_rejects_disconnected_structure_row() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_disconnected_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("disconnected row must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::DisconnectedElementError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-005");
}

#[test]
fn strict_mode_rejects_invalid_hillslope_domain() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_hillslope_domain_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("invalid hillslope id domain must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ContributorDomainError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-006");
}

#[test]
fn strict_mode_rejects_invalid_upstream_reference_domain() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_upstream_reference_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("invalid channel/impoundment contributor ref must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ContributorDomainError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-006");
}

#[test]
fn strict_mode_rejects_row_count_mismatch() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions::strict(2, 3),
    )
    .expect_err("row-count mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::RecordCountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-011");
}

#[test]
fn strict_mode_rejects_channel_count_mismatch() {
    let mut options = WatershedStructureParseOptions::strict(2, 2);
    options.expected_channel_count = Some(2);

    let err =
        parse_watershed_structure_from_path(fixture_path("strict_valid_two_rows.str"), options)
            .expect_err("channel-count mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ChannelCountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-007");
}

#[test]
fn strict_mode_rejects_impoundment_count_mismatch() {
    let mut options = WatershedStructureParseOptions::strict(2, 2);
    options.expected_impoundment_count = Some(2);

    let err =
        parse_watershed_structure_from_path(fixture_path("strict_valid_two_rows.str"), options)
            .expect_err("impoundment-count mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ImpoundmentCountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-008");
}

#[test]
fn strict_mode_rejects_hillslope_coverage_mismatch() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions::strict(3, 2),
    )
    .expect_err("hillslope coverage mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::HillslopeCoverageMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-009");
}

#[test]
fn strict_mode_rejects_missing_file_with_typed_open_error() {
    let err = parse_watershed_structure_from_path(
        fixture_path("does_not_exist.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("missing file must produce typed open error");

    assert!(matches!(
        err,
        WatershedStructureParseError::InputOpenError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-000");
}

#[test]
fn strict_mode_rejects_invalid_nhill_context() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions::strict(0, 2),
    )
    .expect_err("nhill context must be positive");

    assert!(matches!(
        err,
        WatershedStructureParseError::NhillContextError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-009");
}

#[test]
fn strict_mode_requires_expected_rows_context() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions {
            mode: ParseMode::Strict,
            nhill: 2,
            expected_rows: None,
            expected_channel_count: None,
            expected_impoundment_count: None,
        },
    )
    .expect_err("expected_rows context is required");

    assert!(matches!(
        err,
        WatershedStructureParseError::InvariantViolation { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-010");
}

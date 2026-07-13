#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::too_many_lines
)]

pub mod cli;
mod convert;
pub mod disturbed;

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use openwepp_input_contract::parsers::management::{
    ManagementParseOutput, ParseMode, PlantScenarioData, YearlyScenarioData,
    parse_management_from_path,
};
use openwepp_management_schema as management_yaml;
use serde::{Deserialize, Serialize};

pub const MIGRATOR_NAME: &str = concat!("openwepp-landuse-migrate ", env!("CARGO_PKG_VERSION"));
pub const TARGET_FORMAT: &str = management_yaml::MANAGEMENT_YAML_FORMAT;

pub trait LanduseMigrator {
    fn source_versions(&self) -> &[&'static str];
    fn target_version(&self) -> MigrationTarget;
    fn required_args(
        &self,
        source: &SourceManagement,
    ) -> Result<MigrationArgSpec, LanduseMigrationError>;
    fn validate(
        &self,
        source: &SourceManagement,
        args: &MigrationAuthority,
    ) -> Result<ValidationReport, LanduseMigrationError>;
    fn migrate(
        &self,
        source: SourceManagement,
        request: &MigrationRequest,
    ) -> Result<MigrationOutputYaml, LanduseMigrationError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MigrationTarget {
    OwLanuse1,
    Latest,
}

impl MigrationTarget {
    #[must_use]
    pub const fn resolved_datver(self) -> &'static str {
        match self {
            Self::OwLanuse1 | Self::Latest => management_yaml::OW_LANUSE_1_DATVER,
        }
    }

    #[must_use]
    pub const fn as_cli_value(self) -> &'static str {
        match self {
            Self::OwLanuse1 => "ow-lanuse-1",
            Self::Latest => "latest",
        }
    }
}

impl fmt::Display for MigrationTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_cli_value())
    }
}

impl FromStr for MigrationTarget {
    type Err = LanduseMigrationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "ow-lanuse-1" => Ok(Self::OwLanuse1),
            "latest" => Ok(Self::Latest),
            other => Err(LanduseMigrationError::UnsupportedTarget {
                target: other.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportFormat {
    #[default]
    Text,
    Json,
    Toml,
}

impl FromStr for ReportFormat {
    type Err = LanduseMigrationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "toml" => Ok(Self::Toml),
            other => Err(LanduseMigrationError::InvalidCommand {
                detail: format!("unsupported format {other}; expected text, json, or toml"),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SourceManagement {
    Flat {
        path: PathBuf,
        parsed: ManagementParseOutput,
    },
    Yaml {
        path: PathBuf,
        document: management_yaml::ManagementYamlDocument,
    },
}

impl SourceManagement {
    pub fn parse_path(path: impl AsRef<Path>) -> Result<Self, LanduseMigrationError> {
        let path_ref = path.as_ref();
        if management_yaml::consumer_accepts_management_yaml_extension(path_ref) {
            let document = management_yaml::parse_management_yaml_from_path(path_ref)
                .map_err(LanduseMigrationError::from)?;
            return Ok(Self::Yaml {
                path: path_ref.to_path_buf(),
                document,
            });
        }

        let parsed = parse_management_from_path(path_ref, ParseMode::Strict).map_err(|source| {
            LanduseMigrationError::InputParse {
                path: path_ref.to_path_buf(),
                detail: source.to_string(),
            }
        })?;
        Ok(Self::Flat {
            path: path_ref.to_path_buf(),
            parsed,
        })
    }

    #[must_use]
    pub fn source_format(&self) -> &'static str {
        match self {
            Self::Flat { .. } => "flat-management",
            Self::Yaml { .. } => management_yaml::MANAGEMENT_YAML_FORMAT,
        }
    }

    #[must_use]
    pub fn source_datver(&self) -> &str {
        match self {
            Self::Flat { parsed, .. } => &parsed.datver,
            Self::Yaml { document, .. } => &document.datver,
        }
    }

    #[must_use]
    pub fn path(&self) -> &Path {
        match self {
            Self::Flat { path, .. } | Self::Yaml { path, .. } => path,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationAuthority {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disturbed_class: Option<String>,
    #[serde(default)]
    pub disturbed_class_map: ClassMap,
}

impl MigrationAuthority {
    pub fn merge_from_args_file(
        &mut self,
        args_file: MigrationArgsFile,
    ) -> Result<(), LanduseMigrationError> {
        merge_optional_disturbed_class(
            &mut self.disturbed_class,
            args_file.disturbed_class,
            "disturbed_class",
        )?;
        self.disturbed_class_map
            .merge_checked(args_file.disturbed_class_map)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassMap {
    #[serde(default)]
    pub plant_scenario_name: BTreeMap<String, DisturbedClassAssignment>,
    #[serde(default)]
    pub plant_index: BTreeMap<usize, DisturbedClassAssignment>,
    #[serde(default)]
    pub schedule_slot: BTreeMap<String, DisturbedClassAssignment>,
    #[serde(default)]
    pub ofe_index: BTreeMap<usize, DisturbedClassAssignment>,
}

impl ClassMap {
    fn is_empty(&self) -> bool {
        self.plant_scenario_name.is_empty()
            && self.plant_index.is_empty()
            && self.schedule_slot.is_empty()
            && self.ofe_index.is_empty()
    }

    pub fn merge_checked(&mut self, other: Self) -> Result<(), LanduseMigrationError> {
        for (key, value) in other.plant_scenario_name {
            merge_assignment(
                &mut self.plant_scenario_name,
                key,
                value,
                "plant_scenario_name",
            )?;
        }
        for (key, value) in other.plant_index {
            merge_assignment(&mut self.plant_index, key, value, "plant_index")?;
        }
        for (key, value) in other.schedule_slot {
            merge_assignment(&mut self.schedule_slot, key, value, "schedule_slot")?;
        }
        for (key, value) in other.ofe_index {
            merge_assignment(&mut self.ofe_index, key, value, "ofe_index")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisturbedClassAssignment {
    pub disturbed_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationArgsFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<MigrationTarget>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disturbed_class: Option<String>,
    #[serde(default)]
    pub disturbed_class_map: ClassMap,
}

#[derive(Debug, Clone)]
pub struct MigrationRequest {
    pub input: PathBuf,
    pub target: MigrationTarget,
    pub output: Option<PathBuf>,
    pub authority: MigrationAuthority,
    pub dry_run: bool,
    pub report: Option<PathBuf>,
    pub report_format: ReportFormat,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrationArgSpec {
    pub source_format: String,
    pub source_datver: String,
    pub target_format: String,
    pub target_schema_version: u16,
    pub target_datver: String,
    pub detected_landuses: Vec<DetectedLanduse>,
    pub scheduled_crop_slots: Vec<ScheduledCropSite>,
    pub required_arguments: Vec<String>,
    pub accepted_class_map_key_types: Vec<String>,
    pub blocking_unsupported_source_landuses: Vec<String>,
    pub global_disturbed_class_admissible: bool,
}

impl MigrationArgSpec {
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        text.push_str("source_format: ");
        text.push_str(&self.source_format);
        text.push('\n');
        text.push_str("source_datver: ");
        text.push_str(&self.source_datver);
        text.push('\n');
        text.push_str("target_format: ");
        text.push_str(&self.target_format);
        text.push('\n');
        text.push_str("target_datver: ");
        text.push_str(&self.target_datver);
        text.push('\n');
        text.push_str("required_arguments:\n");
        for argument in &self.required_arguments {
            text.push_str("- ");
            text.push_str(argument);
            text.push('\n');
        }
        text.push_str("accepted_class_map_key_types:\n");
        for key_type in &self.accepted_class_map_key_types {
            text.push_str("- ");
            text.push_str(key_type);
            text.push('\n');
        }
        text
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DetectedLanduse {
    pub section: String,
    pub index: usize,
    pub name: String,
    pub landuse: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ScheduledCropSite {
    pub rotation_index: usize,
    pub year_in_rotation: usize,
    pub ofe_index: usize,
    pub crop_slot: usize,
    pub yearly_ref: usize,
    pub plant_index: usize,
    pub plant_scenario_name: String,
}

impl ScheduledCropSite {
    #[must_use]
    pub fn schedule_slot_key(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.rotation_index, self.year_in_rotation, self.ofe_index, self.crop_slot
        )
    }

    #[must_use]
    pub fn label(&self) -> String {
        format!(
            "schedule_slot={} plant_index={} plant_scenario_name={}",
            self.schedule_slot_key(),
            self.plant_index,
            self.plant_scenario_name
        )
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedCoefficientSite {
    pub plant_index: usize,
    pub plant_scenario_name: String,
    pub disturbed_class: String,
    pub k_o: f64,
    pub form_c_d: f64,
    pub d_r_m: f64,
    pub lambda: f64,
    pub vegetation_c_d: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ValidationReport {
    pub valid: bool,
    pub source_format: String,
    pub source_datver: String,
    pub target_format: String,
    pub target_datver: String,
    pub message: String,
    pub resolved_coefficients: Vec<ResolvedCoefficientSite>,
}

impl ValidationReport {
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        text.push_str(if self.valid {
            "valid: true\n"
        } else {
            "valid: false\n"
        });
        text.push_str("source_format: ");
        text.push_str(&self.source_format);
        text.push('\n');
        text.push_str("source_datver: ");
        text.push_str(&self.source_datver);
        text.push('\n');
        text.push_str("target_datver: ");
        text.push_str(&self.target_datver);
        text.push('\n');
        text.push_str("message: ");
        text.push_str(&self.message);
        text.push('\n');
        text
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrationReport {
    pub source_path: String,
    pub source_format: String,
    pub source_datver: String,
    pub target_format: String,
    pub target_schema_version: u16,
    pub target_datver: String,
    pub output_path: Option<String>,
    pub dry_run: bool,
    pub disturbed_table_id: String,
    pub disturbed_table_version: String,
    pub disturbed_table_checksum: String,
    pub disturbed_table_source_authority: String,
    pub resolved_coefficients: Vec<ResolvedCoefficientSite>,
    pub migration_steps: Vec<String>,
}

impl MigrationReport {
    #[must_use]
    pub fn to_text(&self) -> String {
        let mut text = String::new();
        text.push_str("source_path: ");
        text.push_str(&self.source_path);
        text.push('\n');
        text.push_str("source_format: ");
        text.push_str(&self.source_format);
        text.push('\n');
        text.push_str("source_datver: ");
        text.push_str(&self.source_datver);
        text.push('\n');
        text.push_str("target_datver: ");
        text.push_str(&self.target_datver);
        text.push('\n');
        text.push_str("output_path: ");
        text.push_str(self.output_path.as_deref().unwrap_or("<dry-run>"));
        text.push('\n');
        text.push_str("migration_steps:\n");
        for step in &self.migration_steps {
            text.push_str("- ");
            text.push_str(step);
            text.push('\n');
        }
        text
    }
}

#[derive(Debug, Clone)]
pub struct MigrationOutputYaml {
    pub output_path: Option<PathBuf>,
    pub yaml: String,
    pub document: management_yaml::ManagementYamlDocument,
    pub report: MigrationReport,
}

#[derive(Debug)]
pub enum LanduseMigrationError {
    InputParse {
        path: PathBuf,
        detail: String,
    },
    ManagementYaml {
        detail: String,
    },
    UnsupportedTarget {
        target: String,
    },
    UnsupportedSourceDatver {
        datver: String,
    },
    UnsupportedSourceLanduse {
        detail: String,
    },
    MissingMigrationAuthority {
        site: String,
    },
    UnknownDisturbedClass {
        disturbed_class: String,
    },
    InvalidRouteCoefficientRow {
        disturbed_class: String,
        detail: String,
    },
    ClassMapConflict {
        site: String,
        classes: Vec<String>,
    },
    PartialClassMap {
        missing_sites: Vec<String>,
    },
    NativeMissingRoutingCoefficients {
        plant_index: usize,
        plant_name: String,
    },
    InvalidStructuredFile {
        path: PathBuf,
        detail: String,
    },
    InvalidOutputExtension {
        path: PathBuf,
    },
    OutputExists {
        path: PathBuf,
    },
    Io {
        action: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    InvalidCommand {
        detail: String,
    },
}

impl fmt::Display for LanduseMigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputParse { path, detail } => {
                write!(
                    f,
                    "LANDUSE-MIGRATE-E-001: unable to parse {}: {detail}",
                    path.display()
                )
            }
            Self::ManagementYaml { detail } => {
                write!(
                    f,
                    "LANDUSE-MIGRATE-E-002: invalid management YAML: {detail}"
                )
            }
            Self::UnsupportedTarget { target } => write!(
                f,
                "LANDUSE-MIGRATE-E-003: unsupported target {target}; supported targets are ow-lanuse-1 and latest"
            ),
            Self::UnsupportedSourceDatver { datver } => write!(
                f,
                "LANDUSE-MIGRATE-E-004: unsupported source datver {datver} for this migration"
            ),
            Self::UnsupportedSourceLanduse { detail } => {
                write!(
                    f,
                    "LANDUSE-MIGRATE-E-005: unsupported source landuse: {detail}"
                )
            }
            Self::MissingMigrationAuthority { site } => write!(
                f,
                "LANDUSE-MIGRATE-E-006: missing disturbed-class authority for {site}; run --args-for-migration-to ow-lanuse-1"
            ),
            Self::UnknownDisturbedClass { disturbed_class } => write!(
                f,
                "LANDUSE-MIGRATE-E-007: unknown disturbed class {disturbed_class:?}"
            ),
            Self::InvalidRouteCoefficientRow {
                disturbed_class,
                detail,
            } => write!(
                f,
                "LANDUSE-MIGRATE-E-008: invalid route coefficient row for {disturbed_class:?}: {detail}"
            ),
            Self::ClassMapConflict { site, classes } => write!(
                f,
                "LANDUSE-MIGRATE-E-009: conflicting disturbed classes for {site}: {}",
                classes.join(", ")
            ),
            Self::PartialClassMap { missing_sites } => write!(
                f,
                "LANDUSE-MIGRATE-E-010: partial class map; missing disturbed classes for {}",
                missing_sites.join("; ")
            ),
            Self::NativeMissingRoutingCoefficients {
                plant_index,
                plant_name,
            } => write!(
                f,
                "LANDUSE-MIGRATE-E-011: native source plant {plant_index} ({plant_name}) is missing routing_coefficients"
            ),
            Self::InvalidStructuredFile { path, detail } => write!(
                f,
                "LANDUSE-MIGRATE-E-012: invalid structured file {}: {detail}",
                path.display()
            ),
            Self::InvalidOutputExtension { path } => write!(
                f,
                "LANDUSE-MIGRATE-E-013: producer output path must end in lowercase .yaml: {}",
                path.display()
            ),
            Self::OutputExists { path } => write!(
                f,
                "LANDUSE-MIGRATE-E-014: output path already exists and --force is not supported: {}",
                path.display()
            ),
            Self::Io {
                action,
                path,
                source,
            } => write!(
                f,
                "LANDUSE-MIGRATE-E-015: failed to {action} {}: {source}",
                path.display()
            ),
            Self::InvalidCommand { detail } => {
                write!(f, "LANDUSE-MIGRATE-E-016: {detail}")
            }
        }
    }
}

impl std::error::Error for LanduseMigrationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<management_yaml::ManagementYamlError> for LanduseMigrationError {
    fn from(value: management_yaml::ManagementYamlError) -> Self {
        Self::ManagementYaml {
            detail: value.to_string(),
        }
    }
}

pub struct OpenWeppLanduseMigrator {
    target: MigrationTarget,
}

impl OpenWeppLanduseMigrator {
    #[must_use]
    pub const fn new(target: MigrationTarget) -> Self {
        Self { target }
    }
}

impl LanduseMigrator for OpenWeppLanduseMigrator {
    fn source_versions(&self) -> &[&'static str] {
        &["95.7", "98.4", "2016.3", "2017.1", "ow-lanuse-1"]
    }

    fn target_version(&self) -> MigrationTarget {
        self.target
    }

    fn required_args(
        &self,
        source: &SourceManagement,
    ) -> Result<MigrationArgSpec, LanduseMigrationError> {
        required_args_for_source(source, self.target)
    }

    fn validate(
        &self,
        source: &SourceManagement,
        args: &MigrationAuthority,
    ) -> Result<ValidationReport, LanduseMigrationError> {
        validate_source_for_target(source, self.target, args)
    }

    fn migrate(
        &self,
        source: SourceManagement,
        request: &MigrationRequest,
    ) -> Result<MigrationOutputYaml, LanduseMigrationError> {
        migrate_source(&source, request)
    }
}

pub fn required_args_for_path(
    path: impl AsRef<Path>,
    target: MigrationTarget,
) -> Result<MigrationArgSpec, LanduseMigrationError> {
    let source = SourceManagement::parse_path(path)?;
    required_args_for_source(&source, target)
}

pub fn validate_path(
    request: &MigrationRequest,
) -> Result<ValidationReport, LanduseMigrationError> {
    let source = SourceManagement::parse_path(&request.input)?;
    validate_source_for_target(&source, request.target, &request.authority)
}

pub fn migrate_path(
    request: &MigrationRequest,
) -> Result<MigrationOutputYaml, LanduseMigrationError> {
    let source = SourceManagement::parse_path(&request.input)?;
    migrate_source(&source, request)
}

pub fn load_args_file(path: impl AsRef<Path>) -> Result<MigrationArgsFile, LanduseMigrationError> {
    read_structured_file(path)
}

pub fn load_class_map(path: impl AsRef<Path>) -> Result<ClassMap, LanduseMigrationError> {
    read_structured_file(path)
}

pub fn authority_from_files(
    mut authority: MigrationAuthority,
    args_file: Option<&Path>,
    class_map_file: Option<&Path>,
) -> Result<MigrationAuthority, LanduseMigrationError> {
    if let Some(path) = args_file {
        authority.merge_from_args_file(load_args_file(path)?)?;
    }
    if let Some(path) = class_map_file {
        authority
            .disturbed_class_map
            .merge_checked(load_class_map(path)?)?;
    }
    Ok(authority)
}

pub fn format_arg_spec(
    spec: &MigrationArgSpec,
    format: ReportFormat,
) -> Result<String, LanduseMigrationError> {
    format_serializable(spec, format, || spec.to_text())
}

pub fn format_validation_report(
    report: &ValidationReport,
    format: ReportFormat,
) -> Result<String, LanduseMigrationError> {
    format_serializable(report, format, || report.to_text())
}

pub fn format_migration_report(
    report: &MigrationReport,
    format: ReportFormat,
) -> Result<String, LanduseMigrationError> {
    format_serializable(report, format, || report.to_text())
}

fn required_args_for_source(
    source: &SourceManagement,
    target: MigrationTarget,
) -> Result<MigrationArgSpec, LanduseMigrationError> {
    let detected_landuses = detected_landuses(source);
    let scheduled_crop_slots = match source {
        SourceManagement::Flat { parsed, .. } => scheduled_crop_sites(parsed)?,
        SourceManagement::Yaml { .. } => Vec::new(),
    };
    let mut required_arguments = Vec::new();
    let mut accepted_key_types = Vec::new();
    let mut blocking = Vec::new();
    let mut global_admissible = false;

    match source {
        SourceManagement::Flat { parsed, .. } if is_legacy_datver(&parsed.datver) => {
            required_arguments.push("--to ow-lanuse-1".to_string());
            required_arguments
                .push("--disturbed-class <class> or --disturbed-class-map <path>".to_string());
            accepted_key_types.extend(
                [
                    "plant_scenario_name",
                    "plant_index",
                    "schedule_slot",
                    "ofe_index",
                ]
                .into_iter()
                .map(str::to_string),
            );
            global_admissible = !scheduled_crop_slots.is_empty();
            blocking.extend(legacy_blocking_landuses(parsed));
        }
        SourceManagement::Flat { parsed, .. }
            if parsed.datver == management_yaml::OW_LANUSE_1_DATVER =>
        {
            for (index, plant) in parsed.registries.plants.iter().enumerate() {
                match &plant.data {
                    PlantScenarioData::Cropland(data)
                        if plant.meta.landuse
                            == openwepp_input_contract::parsers::management::NATIVE_CROPLAND_LANUSE_SENTINEL =>
                    {
                        if data.routing.is_none() {
                            blocking.push(format!(
                                "plant_index {} ({}) missing routing_coefficients",
                                index + 1,
                                plant.meta.name
                            ));
                        }
                    }
                    PlantScenarioData::Forest(data) => {
                        if data.routing.is_none() {
                            blocking.push(format!(
                                "plant_index {} ({}) missing routing_coefficients",
                                index + 1,
                                plant.meta.name
                            ));
                        }
                    }
                    PlantScenarioData::Cropland(_) => blocking.push(format!(
                        "plant_index {} ({}) is legacy compatibility cropland under ow-lanuse-1",
                        index + 1,
                        plant.meta.name
                    )),
                }
            }
        }
        SourceManagement::Yaml { .. } => {}
        SourceManagement::Flat { parsed, .. } => {
            blocking.push(format!("unsupported datver {}", parsed.datver));
        }
    }

    Ok(MigrationArgSpec {
        source_format: source.source_format().to_string(),
        source_datver: source.source_datver().to_string(),
        target_format: TARGET_FORMAT.to_string(),
        target_schema_version: management_yaml::MANAGEMENT_YAML_SCHEMA_VERSION,
        target_datver: target.resolved_datver().to_string(),
        detected_landuses,
        scheduled_crop_slots,
        required_arguments,
        accepted_class_map_key_types: accepted_key_types,
        blocking_unsupported_source_landuses: blocking,
        global_disturbed_class_admissible: global_admissible,
    })
}

fn validate_source_for_target(
    source: &SourceManagement,
    target: MigrationTarget,
    args: &MigrationAuthority,
) -> Result<ValidationReport, LanduseMigrationError> {
    let (document, resolved, steps) = convert::source_to_yaml_document(source, target, args)?;
    management_yaml::validate_management_yaml_document(&document)?;
    Ok(ValidationReport {
        valid: true,
        source_format: source.source_format().to_string(),
        source_datver: source.source_datver().to_string(),
        target_format: TARGET_FORMAT.to_string(),
        target_datver: target.resolved_datver().to_string(),
        message: format!("validation succeeded; {} migration step(s)", steps.len()),
        resolved_coefficients: resolved,
    })
}

fn migrate_source(
    source: &SourceManagement,
    request: &MigrationRequest,
) -> Result<MigrationOutputYaml, LanduseMigrationError> {
    let (document, resolved, steps) =
        convert::source_to_yaml_document(source, request.target, &request.authority)?;
    let output_path = request
        .output
        .clone()
        .unwrap_or_else(|| management_yaml::default_migrated_management_yaml_path(source.path()));
    validate_producer_output_path(&output_path)?;
    if output_path.exists() && !request.dry_run {
        return Err(LanduseMigrationError::OutputExists { path: output_path });
    }

    let yaml = management_yaml::to_management_yaml_string(&document)?;
    let actual_output_path = if request.dry_run {
        None
    } else {
        fs::write(&output_path, &yaml).map_err(|source| LanduseMigrationError::Io {
            action: "write",
            path: output_path.clone(),
            source,
        })?;
        Some(output_path)
    };

    let report = MigrationReport {
        source_path: source.path().display().to_string(),
        source_format: source.source_format().to_string(),
        source_datver: source.source_datver().to_string(),
        target_format: TARGET_FORMAT.to_string(),
        target_schema_version: management_yaml::MANAGEMENT_YAML_SCHEMA_VERSION,
        target_datver: request.target.resolved_datver().to_string(),
        output_path: actual_output_path
            .as_ref()
            .map(|path| path.display().to_string()),
        dry_run: request.dry_run,
        disturbed_table_id: disturbed::DISTURBED_ROUTE_TABLE_ID.to_string(),
        disturbed_table_version: disturbed::DISTURBED_ROUTE_TABLE_VERSION.to_string(),
        disturbed_table_checksum: disturbed::disturbed_route_table_checksum(),
        disturbed_table_source_authority: disturbed::DISTURBED_ROUTE_TABLE_SOURCE_AUTHORITY
            .to_string(),
        resolved_coefficients: resolved,
        migration_steps: steps,
    };

    if let Some(report_path) = &request.report {
        let report_text = format_migration_report(&report, request.report_format)?;
        fs::write(report_path, report_text).map_err(|source| LanduseMigrationError::Io {
            action: "write",
            path: report_path.clone(),
            source,
        })?;
    }

    Ok(MigrationOutputYaml {
        output_path: actual_output_path,
        yaml,
        document,
        report,
    })
}

fn validate_producer_output_path(path: &Path) -> Result<(), LanduseMigrationError> {
    if management_yaml::producer_accepts_management_yaml_extension(path) {
        Ok(())
    } else {
        Err(LanduseMigrationError::InvalidOutputExtension {
            path: path.to_path_buf(),
        })
    }
}

fn scheduled_crop_sites(
    parsed: &ManagementParseOutput,
) -> Result<Vec<ScheduledCropSite>, LanduseMigrationError> {
    let mut sites = Vec::new();
    for slot in &parsed.schedule.slots {
        for (crop_position, yearly_ref) in slot.yearly_refs.iter().copied().enumerate() {
            let yearly = parsed
                .registries
                .yearlies
                .get(yearly_ref.saturating_sub(1))
                .ok_or_else(|| LanduseMigrationError::UnsupportedSourceLanduse {
                    detail: format!("dangling yearly_ref {yearly_ref}"),
                })?;
            let YearlyScenarioData::Cropland(crop) = &yearly.data else {
                continue;
            };
            let plant = parsed
                .registries
                .plants
                .get(crop.itype.saturating_sub(1))
                .ok_or_else(|| LanduseMigrationError::UnsupportedSourceLanduse {
                    detail: format!("dangling plant reference {}", crop.itype),
                })?;
            sites.push(ScheduledCropSite {
                rotation_index: slot.rotation_index + 1,
                year_in_rotation: slot.year_in_rotation + 1,
                ofe_index: slot.ofe_index + 1,
                crop_slot: crop_position + 1,
                yearly_ref,
                plant_index: crop.itype,
                plant_scenario_name: plant.meta.name.clone(),
            });
        }
    }
    Ok(sites)
}

fn resolve_legacy_classes(
    parsed: &ManagementParseOutput,
    authority: &MigrationAuthority,
) -> Result<BTreeMap<usize, &'static str>, LanduseMigrationError> {
    let sites = scheduled_crop_sites(parsed)?;
    if sites.is_empty() {
        return Err(LanduseMigrationError::UnsupportedSourceLanduse {
            detail: "legacy migration found no scheduled cropland sites".to_string(),
        });
    }
    if authority.disturbed_class.is_none() && authority.disturbed_class_map.is_empty() {
        return Err(LanduseMigrationError::MissingMigrationAuthority {
            site: "legacy cropland migration".to_string(),
        });
    }

    let mut by_plant: BTreeMap<usize, BTreeSet<&'static str>> = BTreeMap::new();
    let mut missing = Vec::new();
    for site in &sites {
        let classes = candidate_classes_for_site(site, authority)?;
        if classes.is_empty() {
            missing.push(site.label());
            continue;
        }
        if classes.len() > 1 {
            return Err(LanduseMigrationError::ClassMapConflict {
                site: site.label(),
                classes: classes.into_iter().map(str::to_string).collect(),
            });
        }
        by_plant
            .entry(site.plant_index)
            .or_default()
            .extend(classes);
    }

    for (index, plant) in parsed.registries.plants.iter().enumerate() {
        let plant_index = index + 1;
        if by_plant.contains_key(&plant_index) {
            continue;
        }
        let classes = candidate_classes_for_plant(plant_index, &plant.meta.name, authority)?;
        if classes.is_empty() {
            missing.push(format!(
                "plant_index={plant_index} plant_scenario_name={}",
                plant.meta.name
            ));
            continue;
        }
        by_plant.entry(plant_index).or_default().extend(classes);
    }

    if !missing.is_empty() {
        return Err(LanduseMigrationError::PartialClassMap {
            missing_sites: missing,
        });
    }

    let mut resolved = BTreeMap::new();
    for (plant_index, classes) in by_plant {
        if classes.len() > 1 {
            return Err(LanduseMigrationError::ClassMapConflict {
                site: format!("plant_index={plant_index}"),
                classes: classes.into_iter().map(str::to_string).collect(),
            });
        }
        let Some(class) = classes.into_iter().next() else {
            return Err(LanduseMigrationError::MissingMigrationAuthority {
                site: format!("plant_index={plant_index}"),
            });
        };
        resolved.insert(plant_index, class);
    }
    Ok(resolved)
}

fn candidate_classes_for_site(
    site: &ScheduledCropSite,
    authority: &MigrationAuthority,
) -> Result<BTreeSet<&'static str>, LanduseMigrationError> {
    let mut classes =
        candidate_classes_for_plant(site.plant_index, &site.plant_scenario_name, authority)?;

    if let Some(assignment) = authority
        .disturbed_class_map
        .schedule_slot
        .get(&site.schedule_slot_key())
        .or_else(|| {
            authority
                .disturbed_class_map
                .schedule_slot
                .get(&site.schedule_slot_key().replace(':', "/"))
        })
    {
        classes.insert(disturbed::normalize_disturbed_class(
            &assignment.disturbed_class,
        )?);
    }
    if let Some(assignment) = authority.disturbed_class_map.ofe_index.get(&site.ofe_index) {
        classes.insert(disturbed::normalize_disturbed_class(
            &assignment.disturbed_class,
        )?);
    }
    Ok(classes)
}

fn candidate_classes_for_plant(
    plant_index: usize,
    plant_name: &str,
    authority: &MigrationAuthority,
) -> Result<BTreeSet<&'static str>, LanduseMigrationError> {
    let mut classes = BTreeSet::new();
    if let Some(global) = &authority.disturbed_class {
        classes.insert(disturbed::normalize_disturbed_class(global)?);
    }
    if let Some(assignment) = authority
        .disturbed_class_map
        .plant_scenario_name
        .get(plant_name)
    {
        classes.insert(disturbed::normalize_disturbed_class(
            &assignment.disturbed_class,
        )?);
    }
    if let Some(assignment) = authority.disturbed_class_map.plant_index.get(&plant_index) {
        classes.insert(disturbed::normalize_disturbed_class(
            &assignment.disturbed_class,
        )?);
    }
    Ok(classes)
}

fn merge_optional_disturbed_class(
    existing: &mut Option<String>,
    incoming: Option<String>,
    site: &str,
) -> Result<(), LanduseMigrationError> {
    let Some(incoming) = incoming else {
        return Ok(());
    };
    if let Some(existing_value) = existing {
        let existing_class = disturbed::normalize_disturbed_class(existing_value)?;
        let incoming_class = disturbed::normalize_disturbed_class(&incoming)?;
        if existing_class != incoming_class {
            return Err(LanduseMigrationError::ClassMapConflict {
                site: site.to_string(),
                classes: vec![existing_class.to_string(), incoming_class.to_string()],
            });
        }
        return Ok(());
    }
    *existing = Some(incoming);
    Ok(())
}

fn merge_assignment<K>(
    target: &mut BTreeMap<K, DisturbedClassAssignment>,
    key: K,
    value: DisturbedClassAssignment,
    key_type: &str,
) -> Result<(), LanduseMigrationError>
where
    K: Clone + Ord + fmt::Display,
{
    if let Some(existing) = target.get(&key) {
        let existing_class = disturbed::normalize_disturbed_class(&existing.disturbed_class)?;
        let incoming_class = disturbed::normalize_disturbed_class(&value.disturbed_class)?;
        if existing_class != incoming_class {
            return Err(LanduseMigrationError::ClassMapConflict {
                site: format!("{key_type}={key}"),
                classes: vec![existing_class.to_string(), incoming_class.to_string()],
            });
        }
        return Ok(());
    }
    target.insert(key, value);
    Ok(())
}

fn is_legacy_datver(datver: &str) -> bool {
    matches!(datver, "95.7" | "98.4" | "2016.3" | "2017.1")
}

fn legacy_blocking_landuses(parsed: &ManagementParseOutput) -> Vec<String> {
    parsed
        .registries
        .plants
        .iter()
        .enumerate()
        .filter_map(|(index, plant)| {
            if plant.meta.landuse == 1 {
                None
            } else {
                Some(format!(
                    "plant_index {} ({}) landuse {}",
                    index + 1,
                    plant.meta.name,
                    plant.meta.landuse
                ))
            }
        })
        .collect()
}

fn detected_landuses(source: &SourceManagement) -> Vec<DetectedLanduse> {
    match source {
        SourceManagement::Flat { parsed, .. } => parsed
            .registries
            .plants
            .iter()
            .enumerate()
            .map(|(index, plant)| DetectedLanduse {
                section: "plants".to_string(),
                index: index + 1,
                name: plant.meta.name.clone(),
                landuse: plant.meta.landuse,
            })
            .collect(),
        SourceManagement::Yaml { document, .. } => document
            .plants
            .iter()
            .enumerate()
            .map(|(index, plant)| DetectedLanduse {
                section: "plants".to_string(),
                index: index + 1,
                name: plant.name().to_string(),
                landuse: match plant {
                    management_yaml::PlantScenario::NativeCropland { .. } => 4,
                    management_yaml::PlantScenario::NativeForest { .. } => 3,
                },
            })
            .collect(),
    }
}

fn read_structured_file<T>(path: impl AsRef<Path>) -> Result<T, LanduseMigrationError>
where
    T: for<'de> Deserialize<'de>,
{
    let path_ref = path.as_ref();
    let input = fs::read_to_string(path_ref).map_err(|source| LanduseMigrationError::Io {
        action: "read",
        path: path_ref.to_path_buf(),
        source,
    })?;
    match path_ref
        .extension()
        .and_then(|extension| extension.to_str())
    {
        Some("toml") => {
            toml::from_str(&input).map_err(|error| LanduseMigrationError::InvalidStructuredFile {
                path: path_ref.to_path_buf(),
                detail: error.to_string(),
            })
        }
        Some("json") => serde_json::from_str(&input).map_err(|error| {
            LanduseMigrationError::InvalidStructuredFile {
                path: path_ref.to_path_buf(),
                detail: error.to_string(),
            }
        }),
        Some("yaml" | "yml" | "YAML" | "YML") => serde_yaml::from_str(&input).map_err(|error| {
            LanduseMigrationError::InvalidStructuredFile {
                path: path_ref.to_path_buf(),
                detail: error.to_string(),
            }
        }),
        _ => Err(LanduseMigrationError::InvalidStructuredFile {
            path: path_ref.to_path_buf(),
            detail: "expected .toml, .json, .yaml, or .yml".to_string(),
        }),
    }
}

fn format_serializable<T, F>(
    value: &T,
    format: ReportFormat,
    text: F,
) -> Result<String, LanduseMigrationError>
where
    T: Serialize,
    F: FnOnce() -> String,
{
    match format {
        ReportFormat::Text => Ok(text()),
        ReportFormat::Json => serde_json::to_string_pretty(value)
            .map(|mut value| {
                value.push('\n');
                value
            })
            .map_err(|error| LanduseMigrationError::InvalidCommand {
                detail: format!("failed to render json: {error}"),
            }),
        ReportFormat::Toml => {
            toml::to_string_pretty(value).map_err(|error| LanduseMigrationError::InvalidCommand {
                detail: format!("failed to render toml: {error}"),
            })
        }
    }
}

pub(crate) fn legacy_class_map(
    parsed: &ManagementParseOutput,
    authority: &MigrationAuthority,
) -> Result<BTreeMap<usize, &'static str>, LanduseMigrationError> {
    resolve_legacy_classes(parsed, authority)
}

pub(crate) fn is_legacy_source_datver(datver: &str) -> bool {
    is_legacy_datver(datver)
}

#[cfg(test)]
mod m11_tests {
    use std::error::Error as _;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time must be after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "openwepp-landuse-migrate-m11-{label}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("M-11 temp directory should be created");
        path
    }

    fn assignment(disturbed_class: &str) -> DisturbedClassAssignment {
        DisturbedClassAssignment {
            disturbed_class: disturbed_class.to_string(),
        }
    }

    #[test]
    fn m11_all_error_displays_sources_and_yaml_conversion_are_stable() {
        let displays = vec![
            (
                LanduseMigrationError::InputParse {
                    path: PathBuf::from("input.man"),
                    detail: "bad token".to_string(),
                },
                "LANDUSE-MIGRATE-E-001: unable to parse input.man: bad token",
                false,
            ),
            (
                LanduseMigrationError::ManagementYaml {
                    detail: "bad yaml".to_string(),
                },
                "LANDUSE-MIGRATE-E-002: invalid management YAML: bad yaml",
                false,
            ),
            (
                LanduseMigrationError::UnsupportedTarget {
                    target: "future".to_string(),
                },
                "LANDUSE-MIGRATE-E-003: unsupported target future; supported targets are ow-lanuse-1 and latest",
                false,
            ),
            (
                LanduseMigrationError::UnsupportedSourceDatver {
                    datver: "old".to_string(),
                },
                "LANDUSE-MIGRATE-E-004: unsupported source datver old for this migration",
                false,
            ),
            (
                LanduseMigrationError::UnsupportedSourceLanduse {
                    detail: "range".to_string(),
                },
                "LANDUSE-MIGRATE-E-005: unsupported source landuse: range",
                false,
            ),
            (
                LanduseMigrationError::MissingMigrationAuthority {
                    site: "plant_index=1".to_string(),
                },
                "LANDUSE-MIGRATE-E-006: missing disturbed-class authority for plant_index=1; run --args-for-migration-to ow-lanuse-1",
                false,
            ),
            (
                LanduseMigrationError::UnknownDisturbedClass {
                    disturbed_class: "unknown".to_string(),
                },
                "LANDUSE-MIGRATE-E-007: unknown disturbed class \"unknown\"",
                false,
            ),
            (
                LanduseMigrationError::InvalidRouteCoefficientRow {
                    disturbed_class: "bare".to_string(),
                    detail: "negative k".to_string(),
                },
                "LANDUSE-MIGRATE-E-008: invalid route coefficient row for \"bare\": negative k",
                false,
            ),
            (
                LanduseMigrationError::ClassMapConflict {
                    site: "plant_index=1".to_string(),
                    classes: vec!["bare".to_string(), "forest".to_string()],
                },
                "LANDUSE-MIGRATE-E-009: conflicting disturbed classes for plant_index=1: bare, forest",
                false,
            ),
            (
                LanduseMigrationError::PartialClassMap {
                    missing_sites: vec!["plant=1".to_string(), "plant=2".to_string()],
                },
                "LANDUSE-MIGRATE-E-010: partial class map; missing disturbed classes for plant=1; plant=2",
                false,
            ),
            (
                LanduseMigrationError::NativeMissingRoutingCoefficients {
                    plant_index: 2,
                    plant_name: "Pine".to_string(),
                },
                "LANDUSE-MIGRATE-E-011: native source plant 2 (Pine) is missing routing_coefficients",
                false,
            ),
            (
                LanduseMigrationError::InvalidStructuredFile {
                    path: PathBuf::from("args.json"),
                    detail: "bad json".to_string(),
                },
                "LANDUSE-MIGRATE-E-012: invalid structured file args.json: bad json",
                false,
            ),
            (
                LanduseMigrationError::InvalidOutputExtension {
                    path: PathBuf::from("output.yml"),
                },
                "LANDUSE-MIGRATE-E-013: producer output path must end in lowercase .yaml: output.yml",
                false,
            ),
            (
                LanduseMigrationError::OutputExists {
                    path: PathBuf::from("output.yaml"),
                },
                "LANDUSE-MIGRATE-E-014: output path already exists and --force is not supported: output.yaml",
                false,
            ),
            (
                LanduseMigrationError::Io {
                    action: "read",
                    path: PathBuf::from("args.json"),
                    source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
                },
                "LANDUSE-MIGRATE-E-015: failed to read args.json: denied",
                true,
            ),
            (
                LanduseMigrationError::InvalidCommand {
                    detail: "bad flag".to_string(),
                },
                "LANDUSE-MIGRATE-E-016: bad flag",
                false,
            ),
        ];

        assert_eq!(displays.len(), 16);
        for (error, expected, has_source) in displays {
            assert_eq!(error.to_string(), expected);
            assert_eq!(error.source().is_some(), has_source, "{expected}");
        }

        let converted =
            LanduseMigrationError::from(management_yaml::ManagementYamlError::MissingField {
                path: "plants[0].routing_coefficients".to_string(),
            });
        assert_eq!(
            converted.to_string(),
            "LANDUSE-MIGRATE-E-002: invalid management YAML: MAN-YAML-E-008: missing plants[0].routing_coefficients",
        );
        assert!(converted.source().is_none());
    }

    #[test]
    fn m11_authority_from_files_merges_in_declared_order() {
        let dir = temp_dir("authority-success");
        let args_path = dir.join("args.json");
        fs::write(
            &args_path,
            r#"{"target":"latest","disturbed_class":"agriculture crops","disturbed_class_map":{"plant_index":{"1":{"disturbed_class":"bare"}}}}"#,
        )
        .expect("args file should write");
        let map_path = dir.join("map.json");
        fs::write(
            &map_path,
            r#"{"plant_index":{"1":{"disturbed_class":"bare"}},"ofe_index":{"2":{"disturbed_class":"forest"}}}"#,
        )
        .expect("class map should write");

        let initial = MigrationAuthority {
            disturbed_class: Some("agriculture crops".to_string()),
            disturbed_class_map: ClassMap::default(),
        };
        assert_eq!(
            authority_from_files(initial.clone(), None, None).unwrap(),
            initial
        );

        let merged = authority_from_files(initial, Some(&args_path), Some(&map_path))
            .expect("agreeing authority files should merge");
        assert_eq!(merged.disturbed_class.as_deref(), Some("agriculture crops"));
        assert_eq!(
            merged.disturbed_class_map.plant_index.get(&1),
            Some(&assignment("bare")),
        );
        assert_eq!(
            merged.disturbed_class_map.ofe_index.get(&2),
            Some(&assignment("forest")),
        );
    }

    #[test]
    fn m11_authority_from_files_conflicts_preserve_precedence() {
        let dir = temp_dir("authority-conflict");
        let args_path = dir.join("args.json");
        fs::write(
            &args_path,
            r#"{"disturbed_class":"agriculture crops","disturbed_class_map":{"plant_index":{"1":{"disturbed_class":"bare"}}}}"#,
        )
        .expect("args file should write");
        let map_path = dir.join("map.json");
        fs::write(
            &map_path,
            r#"{"plant_index":{"1":{"disturbed_class":"forest"}}}"#,
        )
        .expect("class map should write");

        let global_conflict = authority_from_files(
            MigrationAuthority {
                disturbed_class: Some("bare".to_string()),
                disturbed_class_map: ClassMap::default(),
            },
            Some(&args_path),
            Some(&map_path),
        )
        .unwrap_err();
        assert!(matches!(
            global_conflict,
            LanduseMigrationError::ClassMapConflict { ref site, ref classes }
                if site == "disturbed_class"
                    && classes == &["bare".to_string(), "agriculture crops".to_string()]
        ));

        let file_conflict = authority_from_files(
            MigrationAuthority::default(),
            Some(&args_path),
            Some(&map_path),
        )
        .unwrap_err();
        assert!(matches!(
            file_conflict,
            LanduseMigrationError::ClassMapConflict { ref site, ref classes }
                if site == "plant_index=1"
                    && classes == &["bare".to_string(), "forest".to_string()]
        ));
    }

    #[test]
    fn m11_authority_from_files_reports_the_first_malformed_stage() {
        let dir = temp_dir("authority-malformed");
        let missing_args = dir.join("missing-args.json");
        let malformed_args = dir.join("malformed-args.json");
        fs::write(&malformed_args, "{").expect("malformed args should write");
        let valid_args = dir.join("valid-args.json");
        fs::write(&valid_args, r#"{"disturbed_class":"bare"}"#).expect("valid args should write");
        let malformed_map = dir.join("malformed-map.json");
        fs::write(&malformed_map, "{").expect("malformed map should write");

        let missing = authority_from_files(
            MigrationAuthority::default(),
            Some(&missing_args),
            Some(&malformed_map),
        )
        .unwrap_err();
        assert!(matches!(
            missing,
            LanduseMigrationError::Io { ref path, .. } if path == &missing_args
        ));

        let malformed_first = authority_from_files(
            MigrationAuthority::default(),
            Some(&malformed_args),
            Some(&malformed_map),
        )
        .unwrap_err();
        assert!(matches!(
            malformed_first,
            LanduseMigrationError::InvalidStructuredFile { ref path, .. }
                if path == &malformed_args
        ));

        let malformed_second = authority_from_files(
            MigrationAuthority::default(),
            Some(&valid_args),
            Some(&malformed_map),
        )
        .unwrap_err();
        assert!(matches!(
            malformed_second,
            LanduseMigrationError::InvalidStructuredFile { ref path, .. }
                if path == &malformed_map
        ));
    }
}

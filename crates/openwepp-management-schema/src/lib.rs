#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const MANAGEMENT_YAML_FORMAT: &str = "openwepp-management-yaml";
pub const MANAGEMENT_YAML_SCHEMA_VERSION: u16 = 1;
pub const OW_LANUSE_1_DATVER: &str = "ow-lanuse-1";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagementYamlDocument {
    pub format: String,
    pub schema_version: u16,
    pub datver: String,
    pub topology: Topology,
    pub metadata: ManagementMetadata,
    #[serde(default)]
    pub plants: Vec<PlantScenario>,
    #[serde(default)]
    pub operations: Vec<OperationScenario>,
    #[serde(default)]
    pub initial_conditions: Vec<InitialConditionScenario>,
    #[serde(default)]
    pub surface_effects: Vec<SurfaceEffectScenario>,
    #[serde(default)]
    pub contours: Vec<ContourScenario>,
    #[serde(default)]
    pub drains: Vec<DrainScenario>,
    #[serde(default)]
    pub yearly_scenarios: Vec<YearlyScenario>,
    pub schedule: ManagementSchedule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Topology {
    pub nofes: usize,
    pub total_years: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagementMetadata {
    pub name: String,
    pub description: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ManagementProvenance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagementProvenance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_datver: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migrator: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "landuse", rename_all = "snake_case", deny_unknown_fields)]
pub enum PlantScenario {
    NativeCropland {
        name: String,
        description: Vec<String>,
        crunit: String,
        canopy_line: [f64; 10],
        growth_line: [f64; 8],
        mfocod: usize,
        residue_line: [f64; 10],
        terminal_line: [f64; 3],
        #[serde(default, skip_serializing_if = "Option::is_none")]
        rcc: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        routing_coefficients: Option<RouteCoefficients>,
    },
    NativeForest {
        name: String,
        description: Vec<String>,
        forest_class: String,
        growth: PlantForestGrowth,
        phenology: PlantForestPhenology,
        cf: f64,
        diam: f64,
        decomposition: PlantForestDecomposition,
        community: PlantForestCommunity,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        routing_coefficients: Option<RouteCoefficients>,
    },
}

impl PlantScenario {
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::NativeCropland { name, .. } | Self::NativeForest { name, .. } => name,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteCoefficients {
    pub k_o: f64,
    pub form_c_d: f64,
    pub d_r_m: f64,
    pub lambda: f64,
    pub vegetation_c_d: f64,
    pub authority: RouteCoefficientAuthority,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteCoefficientAuthority {
    pub source: String,
    pub version: String,
    pub checksum: String,
    pub disturbed_class: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_authority: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlantForestGrowth {
    pub bb: f64,
    pub bbb: f64,
    pub beinp: f64,
    pub btemp: f64,
    pub otemp: f64,
    pub gddmax: f64,
    pub dlai: f64,
    pub dropfc: f64,
    pub decfct: f64,
    pub spriod: f64,
    pub extnct: f64,
    pub flivmx: f64,
    pub hmax: f64,
    pub hi: f64,
    pub pltol: f64,
    pub xmxlai: f64,
    pub rsr: f64,
    pub rtmmax: f64,
    pub rdmax: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlantForestPhenologyModel {
    GeneralizedGsiV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlantForestPhenology {
    pub model: PlantForestPhenologyModel,
    pub summer_foliar_biomass_kg_m2: f64,
    pub evergreen_fraction: f64,
    pub structural_canopy_cover_fraction: f64,
    pub structural_biomass_kg_m2: f64,
    pub minimum_temperature_inactive_c: f64,
    pub minimum_temperature_unconstrained_c: f64,
    pub vapor_pressure_deficit_unconstrained_pa: f64,
    pub vapor_pressure_deficit_inactive_pa: f64,
    pub photoperiod_inactive_hours: f64,
    pub photoperiod_unconstrained_hours: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlantForestDecomposition {
    pub oratea: f64,
    pub orater: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlantForestCommunity {
    pub tempmn: f64,
    pub gtemp: f64,
    pub plive: f64,
    pub wood: f64,
    pub grass: PlantForestStratum,
    pub shrub: PlantForestStratum,
    pub tree: PlantForestStratum,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlantForestStratum {
    pub coeff: f64,
    pub diam: f64,
    pub hgt: f64,
    pub pop: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "landuse", rename_all = "snake_case", deny_unknown_fields)]
pub enum OperationScenario {
    NativeCropland {
        name: String,
        description: Vec<String>,
        mfo1: f64,
        mfo2: f64,
        numof: usize,
        pcode: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cltpos: Option<usize>,
        effect_line: Vec<f64>,
        #[serde(default)]
        extension_lines: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "landuse", rename_all = "snake_case", deny_unknown_fields)]
pub enum InitialConditionScenario {
    NativeCropland {
        name: String,
        description: Vec<String>,
        base_line: [f64; 6],
        iresd: usize,
        imngmt: usize,
        residue_line: [f64; 5],
        rtyp: usize,
        thaw_line: [f64; 5],
        terminal_line: [f64; 2],
        #[serde(default, skip_serializing_if = "Option::is_none")]
        understory_line: Option<[f64; 2]>,
    },
    NativeForest {
        name: String,
        description: Vec<String>,
        cancov: f64,
        inrcov: f64,
        rilcov: f64,
        rrinit: f64,
        iresd: usize,
        imngmt: usize,
        sumrtm: f64,
        sumsrm: f64,
        tillay1: f64,
        tillay2: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        understory_line: Option<[f64; 2]>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "landuse", rename_all = "snake_case", deny_unknown_fields)]
pub enum SurfaceEffectScenario {
    NativeCropland {
        name: String,
        description: Vec<String>,
        ntill: usize,
        operations: Vec<SurfaceOperation>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SurfaceOperation {
    pub mdate: usize,
    pub op_ref: usize,
    pub tildep: f64,
    pub typtil: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContourScenario {
    pub name: String,
    pub description: Vec<String>,
    pub cntslp: f64,
    pub rdghgt: f64,
    pub rowlen: f64,
    pub rowspc: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contours_perm: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrainScenario {
    pub name: String,
    pub description: Vec<String>,
    pub ddrain: f64,
    pub drainc: f64,
    pub drdiam: f64,
    pub sdrain: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "landuse", rename_all = "snake_case", deny_unknown_fields)]
pub enum YearlyScenario {
    NativeCropland {
        name: String,
        description: Vec<String>,
        itype: usize,
        tilseq: usize,
        conset: usize,
        drset: usize,
        imngmt: usize,
        branch: YearlyCroplandBranch,
    },
    NativeForest {
        name: String,
        description: Vec<String>,
        itype: usize,
        #[serde(default)]
        jdharv: usize,
        #[serde(default)]
        jdplt: usize,
        #[serde(default)]
        jdstop: usize,
        #[serde(default)]
        rw: f64,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum YearlyCroplandBranch {
    AnnualOrFallow {
        jdharv: usize,
        jdplt: usize,
        rw: f64,
        resmgt: usize,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        extension: Option<YearlyAnnualExtension>,
    },
    Perennial {
        jdharv: usize,
        jdplt: usize,
        jdstop: usize,
        rw: f64,
        mgtopt: usize,
        #[serde(default)]
        cut_days: Vec<usize>,
        #[serde(default)]
        grazing_cycles: Vec<YearlyPerennialGrazingCycle>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum YearlyAnnualExtension {
    Herbicide {
        jdherb: usize,
    },
    Burn {
        jdburn: usize,
        fbmag: f64,
        fbrnog: f64,
    },
    Silage {
        jdslge: usize,
    },
    Cut {
        jdcut: usize,
        frcut: f64,
    },
    Remove {
        jdmove: usize,
        frmove: f64,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct YearlyPerennialGrazingCycle {
    pub animal: f64,
    pub area: f64,
    pub bodywt: f64,
    pub digest: f64,
    pub gday: usize,
    pub gend: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagementSchedule {
    pub ofe_initial_refs: Vec<usize>,
    pub rotation_repeats: usize,
    pub rotation_years: usize,
    pub slots: Vec<ManagementScheduleSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagementScheduleSlot {
    pub rotation_index: usize,
    pub year_in_rotation: usize,
    pub ofe_index: usize,
    pub yearly_refs: Vec<usize>,
}

#[derive(Debug)]
pub enum ManagementYamlError {
    InputOpen {
        path: PathBuf,
        source: io::Error,
    },
    UnsupportedExtension {
        path: PathBuf,
    },
    YamlParse {
        detail: String,
    },
    InvalidIdentity {
        field: &'static str,
        expected: &'static str,
        observed: String,
    },
    UnsupportedSchemaVersion {
        observed: u16,
    },
    UnsupportedDatver {
        observed: String,
    },
    InvalidField {
        path: String,
        detail: String,
    },
    MissingField {
        path: String,
    },
}

impl fmt::Display for ManagementYamlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpen { path, source } => {
                write!(
                    f,
                    "MAN-YAML-E-001: unable to read '{}': {source}",
                    path.display()
                )
            }
            Self::UnsupportedExtension { path } => write!(
                f,
                "MAN-YAML-E-002: unsupported management YAML extension for '{}'",
                path.display()
            ),
            Self::YamlParse { detail } => write!(f, "MAN-YAML-E-003: invalid YAML: {detail}"),
            Self::InvalidIdentity {
                field,
                expected,
                observed,
            } => write!(
                f,
                "MAN-YAML-E-004: invalid {field}: expected {expected}, observed {observed}"
            ),
            Self::UnsupportedSchemaVersion { observed } => {
                write!(f, "MAN-YAML-E-005: unsupported schema_version {observed}")
            }
            Self::UnsupportedDatver { observed } => {
                write!(f, "MAN-YAML-E-006: unsupported datver {observed}")
            }
            Self::InvalidField { path, detail } => {
                write!(f, "MAN-YAML-E-007: invalid {path}: {detail}")
            }
            Self::MissingField { path } => write!(f, "MAN-YAML-E-008: missing {path}"),
        }
    }
}

impl std::error::Error for ManagementYamlError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpen { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_management_yaml_from_path(
    path: impl AsRef<Path>,
) -> Result<ManagementYamlDocument, ManagementYamlError> {
    let path_ref = path.as_ref();
    if !consumer_accepts_management_yaml_extension(path_ref) {
        return Err(ManagementYamlError::UnsupportedExtension {
            path: path_ref.to_path_buf(),
        });
    }
    let input = fs::read_to_string(path_ref).map_err(|source| ManagementYamlError::InputOpen {
        path: path_ref.to_path_buf(),
        source,
    })?;
    parse_management_yaml_from_str(&input)
}

pub fn parse_management_yaml_from_str(
    input: &str,
) -> Result<ManagementYamlDocument, ManagementYamlError> {
    let document = serde_yaml::from_str::<ManagementYamlDocument>(input).map_err(|error| {
        ManagementYamlError::YamlParse {
            detail: error.to_string(),
        }
    })?;
    validate_management_yaml_document(&document)?;
    Ok(document)
}

pub fn to_management_yaml_string(
    document: &ManagementYamlDocument,
) -> Result<String, ManagementYamlError> {
    validate_management_yaml_document(document)?;
    serde_yaml::to_string(document).map_err(|error| ManagementYamlError::YamlParse {
        detail: error.to_string(),
    })
}

pub fn validate_management_yaml_document(
    document: &ManagementYamlDocument,
) -> Result<(), ManagementYamlError> {
    if document.format != MANAGEMENT_YAML_FORMAT {
        return Err(ManagementYamlError::InvalidIdentity {
            field: "format",
            expected: MANAGEMENT_YAML_FORMAT,
            observed: document.format.clone(),
        });
    }
    if document.schema_version != MANAGEMENT_YAML_SCHEMA_VERSION {
        return Err(ManagementYamlError::UnsupportedSchemaVersion {
            observed: document.schema_version,
        });
    }
    if document.datver != OW_LANUSE_1_DATVER {
        return Err(ManagementYamlError::UnsupportedDatver {
            observed: document.datver.clone(),
        });
    }
    validate_positive("topology.nofes", document.topology.nofes)?;
    validate_positive("topology.total_years", document.topology.total_years)?;
    validate_description("metadata.description", &document.metadata.description)?;
    validate_positive_count("plants", document.plants.len())?;
    validate_positive_count("initial_conditions", document.initial_conditions.len())?;
    validate_positive_count("yearly_scenarios", document.yearly_scenarios.len())?;

    for (index, plant) in document.plants.iter().enumerate() {
        validate_plant(index, plant)?;
    }
    for (index, operation) in document.operations.iter().enumerate() {
        validate_operation(index, operation)?;
    }
    for (index, initial) in document.initial_conditions.iter().enumerate() {
        validate_initial(index, initial, document.plants.len())?;
    }
    for (index, surface) in document.surface_effects.iter().enumerate() {
        validate_surface(index, surface, document.operations.len())?;
    }
    for (index, contour) in document.contours.iter().enumerate() {
        validate_description(
            &format!("contours[{index}].description"),
            &contour.description,
        )?;
    }
    for (index, drain) in document.drains.iter().enumerate() {
        validate_description(&format!("drains[{index}].description"), &drain.description)?;
    }
    for (index, yearly) in document.yearly_scenarios.iter().enumerate() {
        validate_yearly(index, yearly, document)?;
    }
    validate_schedule(document)
}

#[must_use]
pub fn consumer_accepts_management_yaml_extension(path: impl AsRef<Path>) -> bool {
    terminal_extension_is_one_of(path.as_ref(), &["yaml", "YAML", "yml", "YML"])
}

#[must_use]
pub fn producer_accepts_management_yaml_extension(path: impl AsRef<Path>) -> bool {
    terminal_extension_is_one_of(path.as_ref(), &["yaml"])
}

#[must_use]
pub fn default_migrated_management_yaml_path(input: impl AsRef<Path>) -> PathBuf {
    let input_ref = input.as_ref();
    let mut output = input_ref.as_os_str().to_os_string();
    output.push(".yaml");
    PathBuf::from(output)
}

fn terminal_extension_is_one_of(path: &Path, allowed: &[&str]) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| allowed.contains(&extension))
}

fn validate_plant(index: usize, plant: &PlantScenario) -> Result<(), ManagementYamlError> {
    match plant {
        PlantScenario::NativeCropland {
            description,
            routing_coefficients,
            ..
        } => {
            validate_description(&format!("plants[{index}].description"), description)?;
            validate_route_coefficients(
                routing_coefficients.as_ref(),
                &format!("plants[{index}].routing_coefficients"),
            )
        }
        PlantScenario::NativeForest {
            description,
            phenology,
            routing_coefficients,
            ..
        } => {
            validate_description(&format!("plants[{index}].description"), description)?;
            validate_forest_phenology(index, phenology)?;
            validate_route_coefficients(
                routing_coefficients.as_ref(),
                &format!("plants[{index}].routing_coefficients"),
            )
        }
    }
}

fn validate_forest_phenology(
    index: usize,
    phenology: &PlantForestPhenology,
) -> Result<(), ManagementYamlError> {
    let path = format!("plants[{index}].phenology");
    validate_positive_finite(
        &format!("{path}.summer_foliar_biomass_kg_m2"),
        phenology.summer_foliar_biomass_kg_m2,
    )?;
    validate_unit_interval(
        &format!("{path}.evergreen_fraction"),
        phenology.evergreen_fraction,
    )?;
    validate_non_negative_finite(
        &format!("{path}.structural_canopy_cover_fraction"),
        phenology.structural_canopy_cover_fraction,
    )?;
    if phenology.structural_canopy_cover_fraction > 0.999 {
        return Err(ManagementYamlError::InvalidField {
            path: format!("{path}.structural_canopy_cover_fraction"),
            detail: "must be at most 0.999".to_string(),
        });
    }
    validate_non_negative_finite(
        &format!("{path}.structural_biomass_kg_m2"),
        phenology.structural_biomass_kg_m2,
    )?;
    validate_ordered_finite_pair(
        &path,
        "minimum_temperature_inactive_c",
        phenology.minimum_temperature_inactive_c,
        "minimum_temperature_unconstrained_c",
        phenology.minimum_temperature_unconstrained_c,
    )?;
    validate_ordered_finite_pair(
        &path,
        "vapor_pressure_deficit_unconstrained_pa",
        phenology.vapor_pressure_deficit_unconstrained_pa,
        "vapor_pressure_deficit_inactive_pa",
        phenology.vapor_pressure_deficit_inactive_pa,
    )?;
    validate_ordered_finite_pair(
        &path,
        "photoperiod_inactive_hours",
        phenology.photoperiod_inactive_hours,
        "photoperiod_unconstrained_hours",
        phenology.photoperiod_unconstrained_hours,
    )?;
    if phenology.vapor_pressure_deficit_unconstrained_pa < 0.0
        || phenology.photoperiod_inactive_hours < 0.0
        || phenology.photoperiod_unconstrained_hours > 24.0
    {
        return Err(ManagementYamlError::InvalidField {
            path,
            detail: "GSI thresholds are outside their physical domains".to_string(),
        });
    }
    Ok(())
}

fn validate_ordered_finite_pair(
    path: &str,
    lower_name: &str,
    lower: f64,
    upper_name: &str,
    upper: f64,
) -> Result<(), ManagementYamlError> {
    validate_finite_value(&format!("{path}.{lower_name}"), lower)?;
    validate_finite_value(&format!("{path}.{upper_name}"), upper)?;
    if lower >= upper {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: format!("{lower_name} must be less than {upper_name}"),
        });
    }
    Ok(())
}

fn validate_finite_value(path: &str, value: f64) -> Result<(), ManagementYamlError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must be finite".to_string(),
        })
    }
}

fn validate_route_coefficients(
    routing: Option<&RouteCoefficients>,
    path: &str,
) -> Result<(), ManagementYamlError> {
    let routing = routing.ok_or_else(|| ManagementYamlError::MissingField {
        path: path.to_string(),
    })?;
    validate_positive_finite(&format!("{path}.k_o"), routing.k_o)?;
    for (field, value) in [
        ("form_c_d", routing.form_c_d),
        ("d_r_m", routing.d_r_m),
        ("vegetation_c_d", routing.vegetation_c_d),
    ] {
        validate_non_negative_finite(&format!("{path}.{field}"), value)?;
    }
    validate_unit_interval(&format!("{path}.lambda"), routing.lambda)?;
    validate_non_empty(
        &format!("{path}.authority.source"),
        &routing.authority.source,
    )?;
    validate_non_empty(
        &format!("{path}.authority.version"),
        &routing.authority.version,
    )?;
    validate_non_empty(
        &format!("{path}.authority.checksum"),
        &routing.authority.checksum,
    )?;
    validate_non_empty(
        &format!("{path}.authority.disturbed_class"),
        &routing.authority.disturbed_class,
    )?;
    if let Some(source_authority) = &routing.authority.source_authority {
        validate_non_empty(
            &format!("{path}.authority.source_authority"),
            source_authority,
        )?;
    }
    Ok(())
}

fn validate_operation(
    index: usize,
    operation: &OperationScenario,
) -> Result<(), ManagementYamlError> {
    let OperationScenario::NativeCropland {
        description,
        effect_line,
        ..
    } = operation;
    validate_description(&format!("operations[{index}].description"), description)?;
    if effect_line.is_empty() {
        return Err(ManagementYamlError::InvalidField {
            path: format!("operations[{index}].effect_line"),
            detail: "must contain at least one value".to_string(),
        });
    }
    Ok(())
}

fn validate_initial(
    index: usize,
    initial: &InitialConditionScenario,
    plant_count: usize,
) -> Result<(), ManagementYamlError> {
    match initial {
        InitialConditionScenario::NativeCropland {
            description,
            iresd,
            imngmt,
            rtyp,
            ..
        } => {
            validate_description(
                &format!("initial_conditions[{index}].description"),
                description,
            )?;
            validate_reference(
                &format!("initial_conditions[{index}].iresd"),
                *iresd,
                plant_count,
            )?;
            validate_positive(&format!("initial_conditions[{index}].imngmt"), *imngmt)?;
            validate_positive(&format!("initial_conditions[{index}].rtyp"), *rtyp)
        }
        InitialConditionScenario::NativeForest {
            description,
            iresd,
            imngmt,
            ..
        } => {
            validate_description(
                &format!("initial_conditions[{index}].description"),
                description,
            )?;
            validate_reference(
                &format!("initial_conditions[{index}].iresd"),
                *iresd,
                plant_count,
            )?;
            validate_positive(&format!("initial_conditions[{index}].imngmt"), *imngmt)
        }
    }
}

fn validate_surface(
    index: usize,
    surface: &SurfaceEffectScenario,
    operation_count: usize,
) -> Result<(), ManagementYamlError> {
    let SurfaceEffectScenario::NativeCropland {
        description,
        ntill,
        operations,
        ..
    } = surface;
    validate_description(
        &format!("surface_effects[{index}].description"),
        description,
    )?;
    if *ntill != operations.len() {
        return Err(ManagementYamlError::InvalidField {
            path: format!("surface_effects[{index}].ntill"),
            detail: format!("must match operations length {}", operations.len()),
        });
    }
    for (operation_index, operation) in operations.iter().enumerate() {
        validate_reference(
            &format!("surface_effects[{index}].operations[{operation_index}].op_ref"),
            operation.op_ref,
            operation_count,
        )?;
    }
    Ok(())
}

fn validate_yearly(
    index: usize,
    yearly: &YearlyScenario,
    document: &ManagementYamlDocument,
) -> Result<(), ManagementYamlError> {
    match yearly {
        YearlyScenario::NativeCropland {
            description,
            itype,
            tilseq,
            conset,
            drset,
            ..
        } => {
            validate_description(
                &format!("yearly_scenarios[{index}].description"),
                description,
            )?;
            validate_reference(
                &format!("yearly_scenarios[{index}].itype"),
                *itype,
                document.plants.len(),
            )?;
            validate_zero_or_reference(
                &format!("yearly_scenarios[{index}].tilseq"),
                *tilseq,
                document.surface_effects.len(),
            )?;
            validate_zero_or_reference(
                &format!("yearly_scenarios[{index}].conset"),
                *conset,
                document.contours.len(),
            )?;
            validate_zero_or_reference(
                &format!("yearly_scenarios[{index}].drset"),
                *drset,
                document.drains.len(),
            )
        }
        YearlyScenario::NativeForest {
            description, itype, ..
        } => {
            validate_description(
                &format!("yearly_scenarios[{index}].description"),
                description,
            )?;
            validate_reference(
                &format!("yearly_scenarios[{index}].itype"),
                *itype,
                document.plants.len(),
            )
        }
    }
}

fn validate_schedule(document: &ManagementYamlDocument) -> Result<(), ManagementYamlError> {
    let schedule = &document.schedule;
    if schedule.ofe_initial_refs.len() != document.topology.nofes {
        return Err(ManagementYamlError::InvalidField {
            path: "schedule.ofe_initial_refs".to_string(),
            detail: format!("must contain {} entries", document.topology.nofes),
        });
    }
    for (index, initial_ref) in schedule.ofe_initial_refs.iter().enumerate() {
        validate_reference(
            &format!("schedule.ofe_initial_refs[{index}]"),
            *initial_ref,
            document.initial_conditions.len(),
        )?;
    }
    validate_positive("schedule.rotation_repeats", schedule.rotation_repeats)?;
    validate_positive("schedule.rotation_years", schedule.rotation_years)?;
    let derived_total_years = schedule.rotation_repeats * schedule.rotation_years;
    if derived_total_years != document.topology.total_years {
        return Err(ManagementYamlError::InvalidField {
            path: "topology.total_years".to_string(),
            detail: format!("must equal schedule duration {derived_total_years}"),
        });
    }
    let expected_slots =
        schedule.rotation_repeats * schedule.rotation_years * document.topology.nofes;
    if schedule.slots.len() != expected_slots {
        return Err(ManagementYamlError::InvalidField {
            path: "schedule.slots".to_string(),
            detail: format!("must contain {expected_slots} slots"),
        });
    }
    let mut observed_slots = HashSet::with_capacity(expected_slots);
    for (index, slot) in schedule.slots.iter().enumerate() {
        validate_reference(
            &format!("schedule.slots[{index}].rotation_index"),
            slot.rotation_index,
            schedule.rotation_repeats,
        )?;
        validate_reference(
            &format!("schedule.slots[{index}].year_in_rotation"),
            slot.year_in_rotation,
            schedule.rotation_years,
        )?;
        validate_reference(
            &format!("schedule.slots[{index}].ofe_index"),
            slot.ofe_index,
            document.topology.nofes,
        )?;
        if !observed_slots.insert((slot.rotation_index, slot.year_in_rotation, slot.ofe_index)) {
            return Err(ManagementYamlError::InvalidField {
                path: format!("schedule.slots[{index}]"),
                detail: format!(
                    "duplicate rotation/year/OFE coverage ({}, {}, {})",
                    slot.rotation_index, slot.year_in_rotation, slot.ofe_index
                ),
            });
        }
        validate_positive_count(
            &format!("schedule.slots[{index}].yearly_refs"),
            slot.yearly_refs.len(),
        )?;
        for (yearly_index, yearly_ref) in slot.yearly_refs.iter().enumerate() {
            validate_reference(
                &format!("schedule.slots[{index}].yearly_refs[{yearly_index}]"),
                *yearly_ref,
                document.yearly_scenarios.len(),
            )?;
        }
    }
    for rotation_index in 1..=schedule.rotation_repeats {
        for year_in_rotation in 1..=schedule.rotation_years {
            for ofe_index in 1..=document.topology.nofes {
                if !observed_slots.contains(&(rotation_index, year_in_rotation, ofe_index)) {
                    return Err(ManagementYamlError::InvalidField {
                        path: "schedule.slots".to_string(),
                        detail: format!(
                            "missing rotation/year/OFE coverage ({rotation_index}, {year_in_rotation}, {ofe_index})"
                        ),
                    });
                }
            }
        }
    }
    Ok(())
}

fn validate_description(path: &str, description: &[String]) -> Result<(), ManagementYamlError> {
    if description.len() != 3 {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must contain exactly three lines".to_string(),
        });
    }
    Ok(())
}

fn validate_non_empty(path: &str, value: &str) -> Result<(), ManagementYamlError> {
    if value.trim().is_empty() {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must be non-empty".to_string(),
        });
    }
    Ok(())
}

fn validate_positive(path: &str, value: usize) -> Result<(), ManagementYamlError> {
    if value == 0 {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must be positive".to_string(),
        });
    }
    Ok(())
}

fn validate_positive_count(path: &str, value: usize) -> Result<(), ManagementYamlError> {
    if value == 0 {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must contain at least one entry".to_string(),
        });
    }
    Ok(())
}

fn validate_reference(
    path: &str,
    value: usize,
    max_allowed: usize,
) -> Result<(), ManagementYamlError> {
    if value == 0 || value > max_allowed {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: format!("must be in 1..={max_allowed}"),
        });
    }
    Ok(())
}

fn validate_zero_or_reference(
    path: &str,
    value: usize,
    max_allowed: usize,
) -> Result<(), ManagementYamlError> {
    if value == 0 {
        return Ok(());
    }
    validate_reference(path, value, max_allowed)
}

fn validate_non_negative_finite(path: &str, value: f64) -> Result<(), ManagementYamlError> {
    if !value.is_finite() {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must be finite".to_string(),
        });
    }
    if value < 0.0 {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must be non-negative".to_string(),
        });
    }
    Ok(())
}

fn validate_positive_finite(path: &str, value: f64) -> Result<(), ManagementYamlError> {
    validate_non_negative_finite(path, value)?;
    if value <= 0.0 {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must be positive".to_string(),
        });
    }
    Ok(())
}

fn validate_unit_interval(path: &str, value: f64) -> Result<(), ManagementYamlError> {
    validate_non_negative_finite(path, value)?;
    if value > 1.0 {
        return Err(ManagementYamlError::InvalidField {
            path: path.to_string(),
            detail: "must be in 0..=1".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    fn valid_document() -> ManagementYamlDocument {
        parse_management_yaml_from_str(VALID_ROUTING_YAML).expect("fixture must remain valid")
    }

    fn assert_invalid_field(
        result: Result<(), ManagementYamlError>,
        expected_path: &str,
        expected_detail: &str,
    ) {
        assert!(matches!(
            result,
            Err(ManagementYamlError::InvalidField { path, detail })
                if path == expected_path && detail == expected_detail
        ));
    }

    #[test]
    fn management_yaml_error_display_and_sources_are_exact() {
        let errors = [
            ManagementYamlError::InputOpen {
                path: PathBuf::from("management.yaml"),
                source: io::Error::new(io::ErrorKind::PermissionDenied, "denied"),
            },
            ManagementYamlError::UnsupportedExtension {
                path: PathBuf::from("management.man"),
            },
            ManagementYamlError::YamlParse {
                detail: "bad yaml".to_string(),
            },
            ManagementYamlError::InvalidIdentity {
                field: "format",
                expected: MANAGEMENT_YAML_FORMAT,
                observed: "other".to_string(),
            },
            ManagementYamlError::UnsupportedSchemaVersion { observed: 2 },
            ManagementYamlError::UnsupportedDatver {
                observed: "legacy".to_string(),
            },
            ManagementYamlError::InvalidField {
                path: "topology.nofes".to_string(),
                detail: "must be positive".to_string(),
            },
            ManagementYamlError::MissingField {
                path: "plants[0].routing_coefficients".to_string(),
            },
        ];
        let expected = [
            "MAN-YAML-E-001: unable to read 'management.yaml': denied",
            "MAN-YAML-E-002: unsupported management YAML extension for 'management.man'",
            "MAN-YAML-E-003: invalid YAML: bad yaml",
            "MAN-YAML-E-004: invalid format: expected openwepp-management-yaml, observed other",
            "MAN-YAML-E-005: unsupported schema_version 2",
            "MAN-YAML-E-006: unsupported datver legacy",
            "MAN-YAML-E-007: invalid topology.nofes: must be positive",
            "MAN-YAML-E-008: missing plants[0].routing_coefficients",
        ];
        for (error, expected) in errors.iter().zip(expected) {
            assert_eq!(error.to_string(), expected);
        }
        assert!(errors[0].source().is_some());
        for error in &errors[1..] {
            assert!(error.source().is_none());
        }
    }

    #[test]
    fn document_identity_and_required_field_priority_is_exact() {
        let mut document = valid_document();
        document.format = "other".to_string();
        document.schema_version = 2;
        document.datver = "legacy".to_string();
        document.topology.nofes = 0;
        assert!(matches!(
            validate_management_yaml_document(&document),
            Err(ManagementYamlError::InvalidIdentity {
                field: "format",
                ..
            })
        ));

        document.format = MANAGEMENT_YAML_FORMAT.to_string();
        assert!(matches!(
            validate_management_yaml_document(&document),
            Err(ManagementYamlError::UnsupportedSchemaVersion { observed: 2 })
        ));
        document.schema_version = MANAGEMENT_YAML_SCHEMA_VERSION;
        assert!(matches!(
            validate_management_yaml_document(&document),
            Err(ManagementYamlError::UnsupportedDatver { ref observed }) if observed == "legacy"
        ));
        document.datver = OW_LANUSE_1_DATVER.to_string();
        assert_invalid_field(
            validate_management_yaml_document(&document),
            "topology.nofes",
            "must be positive",
        );

        document.topology.nofes = 1;
        document.topology.total_years = 0;
        assert_invalid_field(
            validate_management_yaml_document(&document),
            "topology.total_years",
            "must be positive",
        );
        document.topology.total_years = 1;
        document.metadata.description.pop();
        assert_invalid_field(
            validate_management_yaml_document(&document),
            "metadata.description",
            "must contain exactly three lines",
        );
    }

    #[test]
    fn schedule_validation_branch_order_and_payloads_are_exact() {
        let base = valid_document();

        let mut row = base.clone();
        row.schedule.ofe_initial_refs.clear();
        assert_invalid_field(
            validate_schedule(&row),
            "schedule.ofe_initial_refs",
            "must contain 1 entries",
        );

        let mut row = base.clone();
        row.schedule.ofe_initial_refs[0] = 0;
        assert_invalid_field(
            validate_schedule(&row),
            "schedule.ofe_initial_refs[0]",
            "must be in 1..=1",
        );

        let mut row = base.clone();
        row.schedule.rotation_repeats = 0;
        assert_invalid_field(
            validate_schedule(&row),
            "schedule.rotation_repeats",
            "must be positive",
        );

        let mut row = base.clone();
        row.schedule.rotation_years = 0;
        assert_invalid_field(
            validate_schedule(&row),
            "schedule.rotation_years",
            "must be positive",
        );

        let mut row = base.clone();
        row.topology.total_years = 2;
        assert_invalid_field(
            validate_schedule(&row),
            "topology.total_years",
            "must equal schedule duration 1",
        );

        let mut row = base.clone();
        row.schedule.slots.clear();
        assert_invalid_field(
            validate_schedule(&row),
            "schedule.slots",
            "must contain 1 slots",
        );

        for (mutate, path) in [
            (
                (|row: &mut ManagementYamlDocument| {
                    row.schedule.slots[0].rotation_index = 0;
                }) as fn(&mut ManagementYamlDocument),
                "schedule.slots[0].rotation_index",
            ),
            (
                |row: &mut ManagementYamlDocument| {
                    row.schedule.slots[0].year_in_rotation = 0;
                },
                "schedule.slots[0].year_in_rotation",
            ),
            (
                |row: &mut ManagementYamlDocument| row.schedule.slots[0].ofe_index = 0,
                "schedule.slots[0].ofe_index",
            ),
        ] {
            let mut row = base.clone();
            mutate(&mut row);
            assert_invalid_field(validate_schedule(&row), path, "must be in 1..=1");
        }

        let mut row = base.clone();
        row.schedule.slots[0].yearly_refs.clear();
        assert_invalid_field(
            validate_schedule(&row),
            "schedule.slots[0].yearly_refs",
            "must contain at least one entry",
        );

        let mut row = base.clone();
        row.schedule.slots[0].yearly_refs[0] = 0;
        assert_invalid_field(
            validate_schedule(&row),
            "schedule.slots[0].yearly_refs[0]",
            "must be in 1..=1",
        );
    }

    #[test]
    fn public_path_parse_and_serialization_error_surfaces_are_bound() {
        assert!(matches!(
            parse_management_yaml_from_path("management.man"),
            Err(ManagementYamlError::UnsupportedExtension { .. })
        ));
        let missing = std::env::temp_dir().join(format!(
            "openwepp-management-schema-missing-{}.yaml",
            std::process::id()
        ));
        assert!(matches!(
            parse_management_yaml_from_path(missing),
            Err(ManagementYamlError::InputOpen { .. })
        ));
        assert!(matches!(
            parse_management_yaml_from_str("not: [valid"),
            Err(ManagementYamlError::YamlParse { .. })
        ));
        let serialized = to_management_yaml_string(&valid_document()).unwrap();
        assert!(serialized.contains("format: openwepp-management-yaml"));
    }

    #[test]
    fn cropland_validator_branches_and_priorities_are_bound() {
        let description = vec!["d1".to_string(), "d2".to_string(), "d3".to_string()];
        let operation = OperationScenario::NativeCropland {
            name: "operation".to_string(),
            description: description.clone(),
            mfo1: 0.0,
            mfo2: 0.0,
            numof: 1,
            pcode: 1,
            cltpos: None,
            effect_line: vec![1.0],
            extension_lines: Vec::new(),
        };
        assert!(validate_operation(0, &operation).is_ok());
        let mut empty_operation = operation.clone();
        let OperationScenario::NativeCropland { effect_line, .. } = &mut empty_operation;
        effect_line.clear();
        assert_invalid_field(
            validate_operation(0, &empty_operation),
            "operations[0].effect_line",
            "must contain at least one value",
        );

        let initial = InitialConditionScenario::NativeCropland {
            name: "initial".to_string(),
            description: description.clone(),
            base_line: [0.0; 6],
            iresd: 1,
            imngmt: 1,
            residue_line: [0.0; 5],
            rtyp: 1,
            thaw_line: [0.0; 5],
            terminal_line: [0.0; 2],
            understory_line: None,
        };
        assert!(validate_initial(0, &initial, 1).is_ok());

        let surface = SurfaceEffectScenario::NativeCropland {
            name: "surface".to_string(),
            description: description.clone(),
            ntill: 1,
            operations: vec![SurfaceOperation {
                mdate: 1,
                op_ref: 1,
                tildep: 0.0,
                typtil: 1,
            }],
        };
        assert!(validate_surface(0, &surface, 1).is_ok());
        let mut bad_surface = surface.clone();
        let SurfaceEffectScenario::NativeCropland { ntill, .. } = &mut bad_surface;
        *ntill = 0;
        assert_invalid_field(
            validate_surface(0, &bad_surface, 1),
            "surface_effects[0].ntill",
            "must match operations length 1",
        );

        let mut document = valid_document();
        document.operations.push(operation);
        document.surface_effects.push(surface);
        document.contours.push(ContourScenario {
            name: "contour".to_string(),
            description: description.clone(),
            cntslp: 0.0,
            rdghgt: 0.0,
            rowlen: 0.0,
            rowspc: 0.0,
            contours_perm: None,
        });
        document.drains.push(DrainScenario {
            name: "drain".to_string(),
            description: description.clone(),
            ddrain: 0.0,
            drainc: 0.0,
            drdiam: 0.0,
            sdrain: 0.0,
        });
        let yearly = YearlyScenario::NativeCropland {
            name: "yearly".to_string(),
            description,
            itype: 1,
            tilseq: 1,
            conset: 1,
            drset: 1,
            imngmt: 1,
            branch: YearlyCroplandBranch::AnnualOrFallow {
                jdharv: 1,
                jdplt: 1,
                rw: 1.0,
                resmgt: 1,
                extension: None,
            },
        };
        assert!(validate_yearly(0, &yearly, &document).is_ok());
    }

    #[test]
    fn extension_policy_accepts_consumer_aliases_but_producer_only_yaml() {
        assert!(consumer_accepts_management_yaml_extension("field.man.yaml"));
        assert!(consumer_accepts_management_yaml_extension("field.YAML"));
        assert!(consumer_accepts_management_yaml_extension("field.yml"));
        assert!(consumer_accepts_management_yaml_extension("field.YML"));
        assert!(!consumer_accepts_management_yaml_extension("field.man"));

        assert!(producer_accepts_management_yaml_extension("field.man.yaml"));
        assert!(!producer_accepts_management_yaml_extension("field.YAML"));
        assert!(!producer_accepts_management_yaml_extension("field.yml"));
    }

    #[test]
    fn default_migration_path_appends_yaml_to_full_source_name() {
        assert_eq!(
            default_migrated_management_yaml_path("hillslope.man"),
            PathBuf::from("hillslope.man.yaml")
        );
    }

    #[test]
    fn parser_accepts_uppercase_yml_consumer_extension() {
        let path = std::env::temp_dir().join(format!(
            "openwepp-management-schema-{}-fixture.YML",
            std::process::id()
        ));
        std::fs::write(&path, VALID_ROUTING_YAML).expect("fixture should be writable");
        let parsed =
            parse_management_yaml_from_path(&path).expect("uppercase .YML should dispatch");
        std::fs::remove_file(&path).ok();

        assert_eq!(parsed.format, MANAGEMENT_YAML_FORMAT);
        assert_eq!(parsed.datver, OW_LANUSE_1_DATVER);
    }

    #[test]
    fn validation_rejects_missing_route_coefficients() {
        let err = parse_management_yaml_from_str(MISSING_ROUTING_YAML)
            .expect_err("missing routing coefficients must fail closed");
        assert!(err.to_string().contains("plants[0].routing_coefficients"));
    }

    #[test]
    fn validation_rejects_invalid_route_coefficient_domains() {
        let zero_ko = VALID_ROUTING_YAML.replace("k_o: 500.0", "k_o: 0.0");
        let zero_err =
            parse_management_yaml_from_str(&zero_ko).expect_err("zero k_o must fail closed");
        assert!(zero_err.to_string().contains("routing_coefficients.k_o"));

        let high_lambda = VALID_ROUTING_YAML.replace("lambda: 0.2", "lambda: 1.2");
        let lambda_err = parse_management_yaml_from_str(&high_lambda)
            .expect_err("lambda above one must fail closed");
        assert!(
            lambda_err
                .to_string()
                .contains("routing_coefficients.lambda")
        );
    }

    #[test]
    fn validation_rejects_duplicate_schedule_slot_coverage() {
        let duplicate = VALID_ROUTING_YAML
            .replace("nofes: 1", "nofes: 2")
            .replace("ofe_initial_refs:\n    - 1", "ofe_initial_refs:\n    - 1\n    - 1")
            .replace(
                "  slots:\n    - rotation_index: 1\n      year_in_rotation: 1\n      ofe_index: 1\n      yearly_refs:\n        - 1",
                "  slots:\n    - rotation_index: 1\n      year_in_rotation: 1\n      ofe_index: 1\n      yearly_refs:\n        - 1\n    - rotation_index: 1\n      year_in_rotation: 1\n      ofe_index: 1\n      yearly_refs:\n        - 1",
            );
        let err = parse_management_yaml_from_str(&duplicate)
            .expect_err("duplicate OFE schedule coverage must fail closed");
        assert!(err.to_string().contains("duplicate rotation/year/OFE"));
    }

    const VALID_ROUTING_YAML: &str = r"
format: openwepp-management-yaml
schema_version: 1
datver: ow-lanuse-1
topology:
  nofes: 1
  total_years: 1
metadata:
  name: Forest_Management
  description: [d1, d2, d3]
plants:
  - landuse: native_forest
    name: Forest_High_Severity_Fire
    description: [d1, d2, d3]
    forest_class: forest_high_sev_fire
    growth:
      bb: 14.0
      bbb: 3.0
      beinp: 0.0
      btemp: 2.0
      otemp: 20.0
      gddmax: 0.1
      dlai: 0.5
      dropfc: 1.0
      decfct: 1.0
      spriod: 90.0
      extnct: 0.45
      flivmx: 17.0
      hmax: 0.2
      hi: 0.42
      pltol: 0.0
      xmxlai: 2.0
      rsr: 0.33
      rtmmax: 0.2
      rdmax: 0.3
    phenology:
      model: generalized_gsi_v1
      summer_foliar_biomass_kg_m2: 0.2
      evergreen_fraction: 0.2
      structural_canopy_cover_fraction: 0.2
      structural_biomass_kg_m2: 0.1
      minimum_temperature_inactive_c: -2.0
      minimum_temperature_unconstrained_c: 5.0
      vapor_pressure_deficit_unconstrained_pa: 900.0
      vapor_pressure_deficit_inactive_pa: 4100.0
      photoperiod_inactive_hours: 10.0
      photoperiod_unconstrained_hours: 11.0
    cf: 5.0
    diam: 0.005
    decomposition:
      oratea: 0.0
      orater: 0.0
    community:
      tempmn: -5.0
      gtemp: 5.0
      plive: 0.2
      wood: 0.1
      grass: { coeff: 0.0, diam: 0.0, hgt: 0.0, pop: 0.0 }
      shrub: { coeff: 0.0, diam: 0.0, hgt: 0.0, pop: 0.0 }
      tree: { coeff: 0.02, diam: 2.0, hgt: 8.0, pop: 500.0 }
    routing_coefficients:
      k_o: 500.0
      form_c_d: 1.25
      d_r_m: 0.06
      lambda: 0.2
      vegetation_c_d: 0.7
      authority:
        source: disturbed-route-coefficients
        version: 2026-07-08
        checksum: fixture
        disturbed_class: high_severity_fire
initial_conditions:
  - landuse: native_forest
    name: Forest_Initial
    description: [d1, d2, d3]
    cancov: 0.4
    inrcov: 0.3
    rilcov: 0.3
    rrinit: 0.06
    iresd: 1
    imngmt: 2
    sumrtm: 0.1
    sumsrm: 0.2
    tillay1: 0.1
    tillay2: 0.2
yearly_scenarios:
  - landuse: native_forest
    name: Forest_Yearly
    description: [d1, d2, d3]
    itype: 1
schedule:
  ofe_initial_refs:
    - 1
  rotation_repeats: 1
  rotation_years: 1
  slots:
    - rotation_index: 1
      year_in_rotation: 1
      ofe_index: 1
      yearly_refs:
        - 1
";

    const MISSING_ROUTING_YAML: &str = r"
format: openwepp-management-yaml
schema_version: 1
datver: ow-lanuse-1
topology:
  nofes: 1
  total_years: 1
metadata:
  name: Forest_Management
  description: [d1, d2, d3]
plants:
  - landuse: native_forest
    name: Forest_High_Severity_Fire
    description: [d1, d2, d3]
    forest_class: forest_high_sev_fire
    growth:
      bb: 14.0
      bbb: 3.0
      beinp: 0.0
      btemp: 2.0
      otemp: 20.0
      gddmax: 0.1
      dlai: 0.5
      dropfc: 1.0
      decfct: 1.0
      spriod: 90.0
      extnct: 0.45
      flivmx: 17.0
      hmax: 0.2
      hi: 0.42
      pltol: 0.0
      xmxlai: 2.0
      rsr: 0.33
      rtmmax: 0.2
      rdmax: 0.3
    phenology:
      model: generalized_gsi_v1
      summer_foliar_biomass_kg_m2: 0.2
      evergreen_fraction: 0.2
      structural_canopy_cover_fraction: 0.2
      structural_biomass_kg_m2: 0.1
      minimum_temperature_inactive_c: -2.0
      minimum_temperature_unconstrained_c: 5.0
      vapor_pressure_deficit_unconstrained_pa: 900.0
      vapor_pressure_deficit_inactive_pa: 4100.0
      photoperiod_inactive_hours: 10.0
      photoperiod_unconstrained_hours: 11.0
    cf: 5.0
    diam: 0.005
    decomposition:
      oratea: 0.0
      orater: 0.0
    community:
      tempmn: -5.0
      gtemp: 5.0
      plive: 0.2
      wood: 0.1
      grass: { coeff: 0.0, diam: 0.0, hgt: 0.0, pop: 0.0 }
      shrub: { coeff: 0.0, diam: 0.0, hgt: 0.0, pop: 0.0 }
      tree: { coeff: 0.02, diam: 2.0, hgt: 8.0, pop: 500.0 }
initial_conditions:
  - landuse: native_forest
    name: Forest_Initial
    description: [d1, d2, d3]
    cancov: 0.4
    inrcov: 0.3
    rilcov: 0.3
    rrinit: 0.06
    iresd: 1
    imngmt: 2
    sumrtm: 0.1
    sumsrm: 0.2
    tillay1: 0.1
    tillay2: 0.2
yearly_scenarios:
  - landuse: native_forest
    name: Forest_Yearly
    description: [d1, d2, d3]
    itype: 1
schedule:
  ofe_initial_refs: [1]
  rotation_repeats: 1
  rotation_years: 1
  slots:
    - rotation_index: 1
      year_in_rotation: 1
      ofe_index: 1
      yearly_refs: [1]
";
}

#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use openwepp_management_schema as management_yaml;

const ALLOWED_DATVERS: &[&str] = &["95.7", "98.4", "2016.3", "2017.1", OW_LANUSE_1_DATVER];

/// openWEPP-native management datver that unlocks first-class `lanuse` modes
/// (currently the forest branch plus native-cropland authoring) under ADR-0034
/// / the management-`lanuse` authority contract (`LANUSE-AUTH-1..6`).
/// Deliberately not a legacy WEPP datver token, so no legacy `.man` collides
/// with the native carve.
pub const OW_LANUSE_1_DATVER: &str = "ow-lanuse-1";

/// Native forest `lanuse` sentinel (legacy WEPP forest code). Under the
/// `ow-lanuse-1` datver this selects the first-class `PlantScenarioData::Forest`
/// / `InitialScenarioData::Forest` / `YearlyScenarioData::Forest` carve; under
/// every legacy datver it stays rejected (`LANUSE-AUTH-4` quarantine).
const FOREST_LANUSE_SENTINEL: usize = 3;

/// Native cropland `lanuse` sentinel. This keeps first-class openWEPP cropland
/// inputs distinct from legacy WEPP cropland compatibility records
/// (`landuse=1`) while reusing the cropland section grammar.
pub const NATIVE_CROPLAND_LANUSE_SENTINEL: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseMode {
    Strict,
    Compatibility,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementSectionCounts {
    pub ncrop: usize,
    pub nop: usize,
    pub nini: usize,
    pub nseq: usize,
    pub ncnt: usize,
    pub ndrain: usize,
    pub nscen: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioMeta {
    pub name: String,
    pub description: [String; 3],
    pub landuse: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlantScenario {
    pub meta: ScenarioMeta,
    pub data: PlantScenarioData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlantScenarioData {
    Cropland(PlantCroplandData),
    /// openWEPP-native forest `lanuse` mode (`ow-lanuse-1` datver, forest
    /// sentinel). First-class typed operands per `LANUSE-AUTH-1/-2`, not a
    /// cropland masquerade.
    Forest(PlantForestData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlantCroplandData {
    pub crunit: String,
    pub canopy_line: [f64; 10],
    pub growth_line: [f64; 8],
    pub mfocod: usize,
    pub residue_line: [f64; 10],
    pub terminal_line: [f64; 3],
    pub rcc: Option<f64>,
    pub routing: Option<RoutingCoefficientExtension>,
}

/// Optional native-landuse Lane D routing coefficient extension. Presence is
/// enforced by the Lane D builder, not by legacy parse compatibility.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoutingCoefficientExtension {
    /// Laminar skin friction coefficient `k_o`.
    pub skin_friction_coefficient_ko: f64,
    /// Form-resistance drag coefficient `C_d`.
    pub form_drag_coefficient: f64,
    /// Roughness-element tip height `D_r` (m).
    pub roughness_element_height_m: f64,
    /// Roughness concentration `lambda`.
    pub roughness_concentration: f64,
    /// Vegetation drag coefficient `C_d`.
    pub vegetation_drag_coefficient: f64,
}

/// First-class forest plant-growth operand block for the openWEPP-native forest
/// `lanuse` mode. Tier A (`growth`/`decomposition`) carries the shared
/// growth-kernel symbols; Tier B (`community`) mirrors the abandoned legacy
/// rangeland grammar as a **structural reference only** (ADR-0017) and is parsed
/// and stored now but consumed by the WS-4 plant-community growth model, not this
/// increment.
///
/// Authority: the operands are explicit and typed (`LANUSE-AUTH-2`
/// fail-closed); the `(texture × class)` land-soil lookup remains the single
/// source of truth for the parameters it owns (`LANUSE-AUTH-6`), reconciled
/// against `forest_class` by the orchestrator reconciliation manifest.
#[derive(Debug, Clone, PartialEq)]
pub struct PlantForestData {
    /// Disturbed/forest class key (e.g. `young_forest`, `high_severity_fire`)
    /// joining the `.man` scenario to its authoritative lookup row, the
    /// `openwepp-disturbed.json` binding, and the `.sol` `DisturbedPolicy`.
    pub forest_class: String,
    pub growth: PlantForestGrowth,
    /// Flat residue-cover equation coefficient (m^2/kg) — litter cover→mass
    /// inversion for the initial residue-depth seed (cropland `cf` analogue).
    pub cf: f64,
    /// Mean stem/branch diameter at maturity (m) — residue-depth seed operand.
    pub diam: f64,
    pub decomposition: PlantForestDecomposition,
    pub community: PlantForestCommunity,
    pub routing: Option<RoutingCoefficientExtension>,
}

/// Tier-A shared growth-kernel operands (the 19 symbols the daily growth kernel
/// consumes). Explicit and required; missing/untyped values fail closed
/// (`LANUSE-AUTH-2`).
#[derive(Debug, Clone, Copy, PartialEq)]
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

/// Tier-A decomposition-surface operands.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlantForestDecomposition {
    pub oratea: f64,
    pub orater: f64,
}

/// Tier-B plant-community operands. Field structure mirrors the legacy
/// rangeland (`iplant=2`) grammar as a structural reference only; values and the
/// consuming plant-community canopy/decline model are re-derived under the
/// growth–canopy contract at WS-4. Stored now, no current kernel reads them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlantForestCommunity {
    pub tempmn: f64,
    pub gtemp: f64,
    pub plive: f64,
    pub wood: f64,
    pub grass: PlantForestStratum,
    pub shrub: PlantForestStratum,
    pub tree: PlantForestStratum,
}

/// Tier-B structural stratum (grass / shrub / tree) — projected-area
/// coefficient, mean canopy diameter, mean height, belt-transect population.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlantForestStratum {
    pub coeff: f64,
    pub diam: f64,
    pub hgt: f64,
    pub pop: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationScenario {
    pub meta: ScenarioMeta,
    pub data: OperationScenarioData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationScenarioData {
    Cropland(OperationCroplandData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationCroplandData {
    pub mfo1: f64,
    pub mfo2: f64,
    pub numof: usize,
    pub pcode: usize,
    pub cltpos: Option<usize>,
    pub effect_line: Vec<f64>,
    pub extension_lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitialScenario {
    pub meta: ScenarioMeta,
    pub data: InitialScenarioData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InitialScenarioData {
    Cropland(InitialCroplandData),
    /// openWEPP-native forest initial-condition block (`ow-lanuse-1`).
    Forest(InitialForestData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitialCroplandData {
    pub base_line: [f64; 6],
    pub iresd: usize,
    pub imngmt: usize,
    pub residue_line: [f64; 5],
    pub rtyp: usize,
    pub thaw_line: [f64; 5],
    pub terminal_line: [f64; 2],
    pub understory_line: Option<[f64; 2]>,
}

/// First-class forest initial-condition block. Cover/roughness
/// (`cancov/inrcov/rilcov/rrinit`) are promoted to first-class forest fields
/// (`LANUSE-AUTH-6` single-source-of-truth) rather than read through the cropland
/// `IniLoopCropland` template. Ridge geometry (`rspace`/`rtyp`) is deliberately
/// **not** carried — forest has no ridge, and `LANUSE-AUTH-3` forbids inferring
/// routing roughness from ridge fields.
#[derive(Debug, Clone, PartialEq)]
pub struct InitialForestData {
    /// Initial canopy cover fraction (0-1).
    pub cancov: f64,
    /// Interrill ground cover fraction (0-1).
    pub inrcov: f64,
    /// Rill ground cover fraction (0-1).
    pub rilcov: f64,
    /// Initial random roughness (m).
    pub rrinit: f64,
    /// Residue plant-scenario reference (1-based `ncrop` index) supplying the
    /// litter operands for the residue-depth seed.
    pub iresd: usize,
    /// Cropping-system flag (perennial forest uses `imngmt = 2`).
    pub imngmt: usize,
    /// Initial total root mass (kg/m^2).
    pub sumrtm: f64,
    /// Initial standing residue mass (kg/m^2).
    pub sumsrm: f64,
    /// Thermal-conductivity layer-1 depth (m).
    pub tillay1: f64,
    /// Thermal-conductivity layer-2 depth (m).
    pub tillay2: f64,
    /// Optional understory interrill/rill cover fractions (`usinrcol`,
    /// `usrilcol`).
    pub understory_line: Option<[f64; 2]>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceScenario {
    pub meta: ScenarioMeta,
    pub ntill: usize,
    pub operations: Vec<SurfaceOperation>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SurfaceOperation {
    pub mdate: usize,
    pub op_ref: usize,
    pub tildep: f64,
    pub typtil: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContourScenario {
    pub meta: ScenarioMeta,
    pub cntslp: f64,
    pub rdghgt: f64,
    pub rowlen: f64,
    pub rowspc: f64,
    pub contours_perm: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DrainScenario {
    pub meta: ScenarioMeta,
    pub ddrain: f64,
    pub drainc: f64,
    pub drdiam: f64,
    pub sdrain: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct YearlyScenario {
    pub meta: ScenarioMeta,
    pub data: YearlyScenarioData,
}

#[derive(Debug, Clone, PartialEq)]
pub enum YearlyScenarioData {
    Cropland(YearlyCroplandData),
    /// openWEPP-native forest yearly schedule (`ow-lanuse-1`): an established
    /// perennial-vegetation slot with no tillage/cut/graze management.
    Forest(YearlyForestData),
}

/// First-class forest yearly-schedule block. Forest is an established perennial
/// stand: no surface-effect (`tilseq`), contour (`conset`), or drain (`drset`)
/// scenario, and no annual cut/graze management. The schedule days default to
/// the established-perennial sentinel (`0`) unless a stand-replacing event is
/// modelled.
#[derive(Debug, Clone, PartialEq)]
pub struct YearlyForestData {
    /// Forest plant-scenario reference (1-based `ncrop` index).
    pub itype: usize,
    /// Harvest / stand-removal day-of-year (`0` = established, no harvest).
    pub jdharv: usize,
    /// Planting / establishment day-of-year (`0` = established).
    pub jdplt: usize,
    /// Growth-stop day-of-year (`0` = no explicit stop).
    pub jdstop: usize,
    /// Row width (m) — `0` for a natural forest stand.
    pub rw: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct YearlyCroplandData {
    pub itype: usize,
    pub tilseq: usize,
    pub conset: usize,
    pub drset: usize,
    pub imngmt: usize,
    pub branch: YearlyCroplandBranch,
}

#[derive(Debug, Clone, PartialEq)]
pub enum YearlyCroplandBranch {
    AnnualOrFallow(YearlyAnnualFallowData),
    Perennial(YearlyPerennialData),
}

#[derive(Debug, Clone, PartialEq)]
pub struct YearlyAnnualFallowData {
    pub jdharv: usize,
    pub jdplt: usize,
    pub rw: f64,
    pub resmgt: usize,
    pub extension: Option<YearlyAnnualExtension>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct YearlyPerennialData {
    pub jdharv: usize,
    pub jdplt: usize,
    pub jdstop: usize,
    pub rw: f64,
    pub mgtopt: usize,
    pub cut_days: Vec<usize>,
    pub grazing_cycles: Vec<YearlyPerennialGrazingCycle>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct YearlyPerennialGrazingCycle {
    pub animal: f64,
    pub area: f64,
    pub bodywt: f64,
    pub digest: f64,
    pub gday: usize,
    pub gend: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementSectionMeta {
    pub name: String,
    pub description: [String; 3],
    pub nofes: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ManagementScenarioRegistries {
    pub plants: Vec<PlantScenario>,
    pub operations: Vec<OperationScenario>,
    pub initials: Vec<InitialScenario>,
    pub surfaces: Vec<SurfaceScenario>,
    pub contours: Vec<ContourScenario>,
    pub drains: Vec<DrainScenario>,
    pub yearlies: Vec<YearlyScenario>,
    pub management_meta: ManagementSectionMeta,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementScheduleSlot {
    pub rotation_index: usize,
    pub year_in_rotation: usize,
    pub ofe_index: usize,
    pub crop_slots: usize,
    pub yearly_refs: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagementSchedule {
    pub ofe_initial_refs: Vec<usize>,
    pub rotation_repeats: usize,
    pub rotation_years: usize,
    pub slots: Vec<ManagementScheduleSlot>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ManagementParseOutput {
    pub datver: String,
    pub topology_count: usize,
    pub declared_total_years: usize,
    pub section_counts: ManagementSectionCounts,
    pub registries: ManagementScenarioRegistries,
    pub schedule: ManagementSchedule,
}

#[derive(Debug)]
pub enum ManagementParseError {
    InputOpenError {
        path: PathBuf,
        source: io::Error,
    },
    MissingRecord {
        field: &'static str,
    },
    TokenParseError {
        field: &'static str,
        value: String,
    },
    RecordArityError {
        field: &'static str,
        observed: usize,
        expected: &'static str,
    },
    UnsupportedDatver {
        datver: String,
    },
    InvalidCount {
        field: &'static str,
        value: i64,
    },
    InvalidOptionDomain {
        field: &'static str,
        value: i64,
        allowed: &'static str,
    },
    UnsupportedLanduse {
        section: &'static str,
        landuse: usize,
    },
    /// A forest (`ow-lanuse-1`) scenario appeared in a section that carries no
    /// forest payload (operation / surface / contour / drain). Forest defines no
    /// tillage, contour, or drain physics, so this fails closed rather than
    /// inventing an empty scenario (`LANUSE-AUTH-2`).
    ForestSectionNotApplicable {
        section: &'static str,
    },
    DanglingScenarioReference {
        field: &'static str,
        value: usize,
        max_allowed: usize,
    },
    TotalYearMismatch {
        declared_total_years: usize,
        derived_total_years: usize,
    },
    DateDomainError {
        field: &'static str,
        value: i64,
        allowed: &'static str,
    },
    TrailingInput {
        first_unconsumed_line: usize,
    },
    YamlInputError {
        detail: String,
    },
}

impl ManagementParseError {
    #[must_use]
    pub fn contract_error_id(&self) -> &'static str {
        match self {
            Self::TokenParseError { .. } => "MAN-E-001",
            Self::InputOpenError { .. }
            | Self::MissingRecord { .. }
            | Self::RecordArityError { .. } => "MAN-E-002",
            Self::UnsupportedDatver { .. } => "MAN-E-003",
            Self::InvalidOptionDomain { .. }
            | Self::UnsupportedLanduse { .. }
            | Self::ForestSectionNotApplicable { .. } => "MAN-E-004",
            Self::InvalidCount { .. } => "MAN-E-005",
            Self::TrailingInput { .. } => "MAN-E-006",
            Self::TotalYearMismatch { .. } => "MAN-E-008",
            Self::DanglingScenarioReference { .. } => "MAN-E-009",
            Self::DateDomainError { .. } => "MAN-E-010",
            Self::YamlInputError { .. } => "MAN-YAML-E-000",
        }
    }
}

impl fmt::Display for ManagementParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputOpenError { path, source } => write!(
                f,
                "{}: could not open {} ({source})",
                self.contract_error_id(),
                path.display()
            ),
            Self::MissingRecord { field } => write!(
                f,
                "{}: missing required record for {field}",
                self.contract_error_id()
            ),
            Self::TokenParseError { field, value } => write!(
                f,
                "{}: invalid token for {field}: {value}",
                self.contract_error_id()
            ),
            Self::RecordArityError {
                field,
                observed,
                expected,
            } => write!(
                f,
                "{}: invalid arity for {field}: observed {observed}, expected {expected}",
                self.contract_error_id()
            ),
            Self::UnsupportedDatver { datver } => write!(
                f,
                "{}: unsupported datver {datver}",
                self.contract_error_id()
            ),
            Self::InvalidCount { field, value } => write!(
                f,
                "{}: invalid count for {field}: {value}",
                self.contract_error_id()
            ),
            Self::InvalidOptionDomain {
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: invalid option for {field}: {value} (allowed: {allowed})",
                self.contract_error_id()
            ),
            Self::UnsupportedLanduse { section, landuse } => write!(
                f,
                "{}: unsupported landuse {landuse} in {section}; rangeland simulation is not supported in openWEPP",
                self.contract_error_id()
            ),
            Self::ForestSectionNotApplicable { section } => write!(
                f,
                "{}: forest lanuse is not applicable in the {section} section; forest defines no tillage/contour/drain scenario",
                self.contract_error_id()
            ),
            Self::DanglingScenarioReference {
                field,
                value,
                max_allowed,
            } => write!(
                f,
                "{}: {field} reference {value} exceeds max allowed {max_allowed}",
                self.contract_error_id()
            ),
            Self::TotalYearMismatch {
                declared_total_years,
                derived_total_years,
            } => write!(
                f,
                "{}: declared total years ({declared_total_years}) do not match derived schedule years ({derived_total_years})",
                self.contract_error_id()
            ),
            Self::DateDomainError {
                field,
                value,
                allowed,
            } => write!(
                f,
                "{}: invalid julian/day value for {field}: {value} (allowed: {allowed})",
                self.contract_error_id()
            ),
            Self::TrailingInput {
                first_unconsumed_line,
            } => write!(
                f,
                "{}: trailing input starts at normalized line {}",
                self.contract_error_id(),
                first_unconsumed_line
            ),
            Self::YamlInputError { detail } => {
                write!(f, "{}: {detail}", self.contract_error_id())
            }
        }
    }
}

impl std::error::Error for ManagementParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InputOpenError { source, .. } => Some(source),
            _ => None,
        }
    }
}

pub fn parse_management_from_path(
    path: impl AsRef<Path>,
    mode: ParseMode,
) -> Result<ManagementParseOutput, ManagementParseError> {
    let path_ref = path.as_ref();
    let content =
        fs::read_to_string(path_ref).map_err(|source| ManagementParseError::InputOpenError {
            path: path_ref.to_path_buf(),
            source,
        })?;
    parse_management_from_str(&content, mode)
}

pub fn parse_management_document_from_path(
    path: impl AsRef<Path>,
    mode: ParseMode,
) -> Result<ManagementParseOutput, ManagementParseError> {
    let path_ref = path.as_ref();
    if management_yaml::consumer_accepts_management_yaml_extension(path_ref) {
        let document = management_yaml::parse_management_yaml_from_path(path_ref)
            .map_err(map_management_yaml_error)?;
        return management_yaml_document_to_parse_output(document);
    }
    parse_management_from_path(path_ref, mode)
}

fn map_management_yaml_error(error: management_yaml::ManagementYamlError) -> ManagementParseError {
    match error {
        management_yaml::ManagementYamlError::InputOpen { path, source } => {
            ManagementParseError::InputOpenError { path, source }
        }
        other => ManagementParseError::YamlInputError {
            detail: other.to_string(),
        },
    }
}

fn management_yaml_document_to_parse_output(
    document: management_yaml::ManagementYamlDocument,
) -> Result<ManagementParseOutput, ManagementParseError> {
    management_yaml::validate_management_yaml_document(&document)
        .map_err(map_management_yaml_error)?;

    let plants = document
        .plants
        .into_iter()
        .map(yaml_plant_to_management)
        .collect::<Result<Vec<_>, _>>()?;
    let operations = document
        .operations
        .into_iter()
        .map(yaml_operation_to_management)
        .collect::<Result<Vec<_>, _>>()?;
    let initials = document
        .initial_conditions
        .into_iter()
        .map(yaml_initial_to_management)
        .collect::<Result<Vec<_>, _>>()?;
    let surfaces = document
        .surface_effects
        .into_iter()
        .map(yaml_surface_to_management)
        .collect::<Result<Vec<_>, _>>()?;
    let contours = document
        .contours
        .into_iter()
        .map(yaml_contour_to_management)
        .collect::<Result<Vec<_>, _>>()?;
    let drains = document
        .drains
        .into_iter()
        .map(yaml_drain_to_management)
        .collect::<Result<Vec<_>, _>>()?;
    let yearlies = document
        .yearly_scenarios
        .into_iter()
        .map(yaml_yearly_to_management)
        .collect::<Result<Vec<_>, _>>()?;

    let section_counts = ManagementSectionCounts {
        ncrop: plants.len(),
        nop: operations.len(),
        nini: initials.len(),
        nseq: surfaces.len(),
        ncnt: contours.len(),
        ndrain: drains.len(),
        nscen: yearlies.len(),
    };
    let schedule = ManagementSchedule {
        ofe_initial_refs: document.schedule.ofe_initial_refs,
        rotation_repeats: document.schedule.rotation_repeats,
        rotation_years: document.schedule.rotation_years,
        slots: document
            .schedule
            .slots
            .into_iter()
            .map(yaml_schedule_slot_to_management)
            .collect::<Result<Vec<_>, _>>()?,
    };
    Ok(ManagementParseOutput {
        datver: document.datver,
        topology_count: document.topology.nofes,
        declared_total_years: document.topology.total_years,
        section_counts,
        registries: ManagementScenarioRegistries {
            plants,
            operations,
            initials,
            surfaces,
            contours,
            drains,
            yearlies,
            management_meta: ManagementSectionMeta {
                name: document.metadata.name,
                description: vec_description_to_array(
                    "metadata.description",
                    document.metadata.description,
                )?,
                nofes: document.topology.nofes,
            },
        },
        schedule,
    })
}

fn yaml_plant_to_management(
    plant: management_yaml::PlantScenario,
) -> Result<PlantScenario, ManagementParseError> {
    match plant {
        management_yaml::PlantScenario::NativeCropland {
            name,
            description,
            crunit,
            canopy_line,
            growth_line,
            mfocod,
            residue_line,
            terminal_line,
            rcc,
            routing_coefficients,
        } => Ok(PlantScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                NATIVE_CROPLAND_LANUSE_SENTINEL,
                "plants[].description",
            )?,
            data: PlantScenarioData::Cropland(PlantCroplandData {
                crunit,
                canopy_line,
                growth_line,
                mfocod,
                residue_line,
                terminal_line,
                rcc,
                routing: routing_coefficients
                    .as_ref()
                    .map(yaml_routing_to_management),
            }),
        }),
        management_yaml::PlantScenario::NativeForest {
            name,
            description,
            forest_class,
            growth,
            cf,
            diam,
            decomposition,
            community,
            routing_coefficients,
        } => Ok(PlantScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                FOREST_LANUSE_SENTINEL,
                "plants[].description",
            )?,
            data: PlantScenarioData::Forest(PlantForestData {
                forest_class,
                growth: PlantForestGrowth {
                    bb: growth.bb,
                    bbb: growth.bbb,
                    beinp: growth.beinp,
                    btemp: growth.btemp,
                    otemp: growth.otemp,
                    gddmax: growth.gddmax,
                    dlai: growth.dlai,
                    dropfc: growth.dropfc,
                    decfct: growth.decfct,
                    spriod: growth.spriod,
                    extnct: growth.extnct,
                    flivmx: growth.flivmx,
                    hmax: growth.hmax,
                    hi: growth.hi,
                    pltol: growth.pltol,
                    xmxlai: growth.xmxlai,
                    rsr: growth.rsr,
                    rtmmax: growth.rtmmax,
                    rdmax: growth.rdmax,
                },
                cf,
                diam,
                decomposition: PlantForestDecomposition {
                    oratea: decomposition.oratea,
                    orater: decomposition.orater,
                },
                community: PlantForestCommunity {
                    tempmn: community.tempmn,
                    gtemp: community.gtemp,
                    plive: community.plive,
                    wood: community.wood,
                    grass: yaml_stratum_to_management(community.grass),
                    shrub: yaml_stratum_to_management(community.shrub),
                    tree: yaml_stratum_to_management(community.tree),
                },
                routing: routing_coefficients
                    .as_ref()
                    .map(yaml_routing_to_management),
            }),
        }),
    }
}

fn yaml_operation_to_management(
    operation: management_yaml::OperationScenario,
) -> Result<OperationScenario, ManagementParseError> {
    match operation {
        management_yaml::OperationScenario::NativeCropland {
            name,
            description,
            mfo1,
            mfo2,
            numof,
            pcode,
            cltpos,
            effect_line,
            extension_lines,
        } => Ok(OperationScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                NATIVE_CROPLAND_LANUSE_SENTINEL,
                "operations[].description",
            )?,
            data: OperationScenarioData::Cropland(OperationCroplandData {
                mfo1,
                mfo2,
                numof,
                pcode,
                cltpos,
                effect_line,
                extension_lines,
            }),
        }),
    }
}

fn yaml_initial_to_management(
    initial: management_yaml::InitialConditionScenario,
) -> Result<InitialScenario, ManagementParseError> {
    match initial {
        management_yaml::InitialConditionScenario::NativeCropland {
            name,
            description,
            base_line,
            iresd,
            imngmt,
            residue_line,
            rtyp,
            thaw_line,
            terminal_line,
            understory_line,
        } => Ok(InitialScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                NATIVE_CROPLAND_LANUSE_SENTINEL,
                "initial_conditions[].description",
            )?,
            data: InitialScenarioData::Cropland(InitialCroplandData {
                base_line,
                iresd,
                imngmt,
                residue_line,
                rtyp,
                thaw_line,
                terminal_line,
                understory_line,
            }),
        }),
        management_yaml::InitialConditionScenario::NativeForest {
            name,
            description,
            cancov,
            inrcov,
            rilcov,
            rrinit,
            iresd,
            imngmt,
            sumrtm,
            sumsrm,
            tillay1,
            tillay2,
            understory_line,
        } => Ok(InitialScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                FOREST_LANUSE_SENTINEL,
                "initial_conditions[].description",
            )?,
            data: InitialScenarioData::Forest(InitialForestData {
                cancov,
                inrcov,
                rilcov,
                rrinit,
                iresd,
                imngmt,
                sumrtm,
                sumsrm,
                tillay1,
                tillay2,
                understory_line,
            }),
        }),
    }
}

fn yaml_surface_to_management(
    surface: management_yaml::SurfaceEffectScenario,
) -> Result<SurfaceScenario, ManagementParseError> {
    match surface {
        management_yaml::SurfaceEffectScenario::NativeCropland {
            name,
            description,
            ntill,
            operations,
        } => Ok(SurfaceScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                NATIVE_CROPLAND_LANUSE_SENTINEL,
                "surface_effects[].description",
            )?,
            ntill,
            operations: operations
                .into_iter()
                .map(|operation| SurfaceOperation {
                    mdate: operation.mdate,
                    op_ref: operation.op_ref,
                    tildep: operation.tildep,
                    typtil: operation.typtil,
                })
                .collect(),
        }),
    }
}

fn yaml_contour_to_management(
    contour: management_yaml::ContourScenario,
) -> Result<ContourScenario, ManagementParseError> {
    Ok(ContourScenario {
        meta: yaml_scenario_meta(
            contour.name,
            contour.description,
            NATIVE_CROPLAND_LANUSE_SENTINEL,
            "contours[].description",
        )?,
        cntslp: contour.cntslp,
        rdghgt: contour.rdghgt,
        rowlen: contour.rowlen,
        rowspc: contour.rowspc,
        contours_perm: contour.contours_perm,
    })
}

fn yaml_drain_to_management(
    drain: management_yaml::DrainScenario,
) -> Result<DrainScenario, ManagementParseError> {
    Ok(DrainScenario {
        meta: yaml_scenario_meta(
            drain.name,
            drain.description,
            NATIVE_CROPLAND_LANUSE_SENTINEL,
            "drains[].description",
        )?,
        ddrain: drain.ddrain,
        drainc: drain.drainc,
        drdiam: drain.drdiam,
        sdrain: drain.sdrain,
    })
}

fn yaml_yearly_to_management(
    yearly: management_yaml::YearlyScenario,
) -> Result<YearlyScenario, ManagementParseError> {
    match yearly {
        management_yaml::YearlyScenario::NativeCropland {
            name,
            description,
            itype,
            tilseq,
            conset,
            drset,
            imngmt,
            branch,
        } => Ok(YearlyScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                NATIVE_CROPLAND_LANUSE_SENTINEL,
                "yearly_scenarios[].description",
            )?,
            data: YearlyScenarioData::Cropland(YearlyCroplandData {
                itype,
                tilseq,
                conset,
                drset,
                imngmt,
                branch: yaml_yearly_branch_to_management(branch),
            }),
        }),
        management_yaml::YearlyScenario::NativeForest {
            name,
            description,
            itype,
            jdharv,
            jdplt,
            jdstop,
            rw,
        } => Ok(YearlyScenario {
            meta: yaml_scenario_meta(
                name,
                description,
                FOREST_LANUSE_SENTINEL,
                "yearly_scenarios[].description",
            )?,
            data: YearlyScenarioData::Forest(YearlyForestData {
                itype,
                jdharv,
                jdplt,
                jdstop,
                rw,
            }),
        }),
    }
}

fn yaml_yearly_branch_to_management(
    branch: management_yaml::YearlyCroplandBranch,
) -> YearlyCroplandBranch {
    match branch {
        management_yaml::YearlyCroplandBranch::AnnualOrFallow {
            jdharv,
            jdplt,
            rw,
            resmgt,
            extension,
        } => YearlyCroplandBranch::AnnualOrFallow(YearlyAnnualFallowData {
            jdharv,
            jdplt,
            rw,
            resmgt,
            extension: extension.as_ref().map(yaml_yearly_extension_to_management),
        }),
        management_yaml::YearlyCroplandBranch::Perennial {
            jdharv,
            jdplt,
            jdstop,
            rw,
            mgtopt,
            cut_days,
            grazing_cycles,
        } => YearlyCroplandBranch::Perennial(YearlyPerennialData {
            jdharv,
            jdplt,
            jdstop,
            rw,
            mgtopt,
            cut_days,
            grazing_cycles: grazing_cycles
                .into_iter()
                .map(|cycle| YearlyPerennialGrazingCycle {
                    animal: cycle.animal,
                    area: cycle.area,
                    bodywt: cycle.bodywt,
                    digest: cycle.digest,
                    gday: cycle.gday,
                    gend: cycle.gend,
                })
                .collect(),
        }),
    }
}

fn yaml_yearly_extension_to_management(
    extension: &management_yaml::YearlyAnnualExtension,
) -> YearlyAnnualExtension {
    match extension {
        management_yaml::YearlyAnnualExtension::Herbicide { jdherb } => {
            YearlyAnnualExtension::Herbicide { jdherb: *jdherb }
        }
        management_yaml::YearlyAnnualExtension::Burn {
            jdburn,
            fbmag,
            fbrnog,
        } => YearlyAnnualExtension::Burn {
            jdburn: *jdburn,
            fbmag: *fbmag,
            fbrnog: *fbrnog,
        },
        management_yaml::YearlyAnnualExtension::Silage { jdslge } => {
            YearlyAnnualExtension::Silage { jdslge: *jdslge }
        }
        management_yaml::YearlyAnnualExtension::Cut { jdcut, frcut } => {
            YearlyAnnualExtension::Cut {
                jdcut: *jdcut,
                frcut: *frcut,
            }
        }
        management_yaml::YearlyAnnualExtension::Remove { jdmove, frmove } => {
            YearlyAnnualExtension::Remove {
                jdmove: *jdmove,
                frmove: *frmove,
            }
        }
    }
}

fn yaml_schedule_slot_to_management(
    slot: management_yaml::ManagementScheduleSlot,
) -> Result<ManagementScheduleSlot, ManagementParseError> {
    Ok(ManagementScheduleSlot {
        rotation_index: yaml_one_based_to_zero_based(
            "schedule.slots[].rotation_index",
            slot.rotation_index,
        )?,
        year_in_rotation: yaml_one_based_to_zero_based(
            "schedule.slots[].year_in_rotation",
            slot.year_in_rotation,
        )?,
        ofe_index: yaml_one_based_to_zero_based("schedule.slots[].ofe_index", slot.ofe_index)?,
        crop_slots: slot.yearly_refs.len(),
        yearly_refs: slot.yearly_refs,
    })
}

fn yaml_routing_to_management(
    routing: &management_yaml::RouteCoefficients,
) -> RoutingCoefficientExtension {
    RoutingCoefficientExtension {
        skin_friction_coefficient_ko: routing.k_o,
        form_drag_coefficient: routing.form_c_d,
        roughness_element_height_m: routing.d_r_m,
        roughness_concentration: routing.lambda,
        vegetation_drag_coefficient: routing.vegetation_c_d,
    }
}

const fn yaml_stratum_to_management(
    stratum: management_yaml::PlantForestStratum,
) -> PlantForestStratum {
    PlantForestStratum {
        coeff: stratum.coeff,
        diam: stratum.diam,
        hgt: stratum.hgt,
        pop: stratum.pop,
    }
}

fn yaml_scenario_meta(
    name: String,
    description: Vec<String>,
    landuse: usize,
    description_field: &'static str,
) -> Result<ScenarioMeta, ManagementParseError> {
    Ok(ScenarioMeta {
        name,
        description: vec_description_to_array(description_field, description)?,
        landuse,
    })
}

fn vec_description_to_array(
    field: &'static str,
    description: Vec<String>,
) -> Result<[String; 3], ManagementParseError> {
    let observed = description.len();
    description
        .try_into()
        .map_err(|_| ManagementParseError::RecordArityError {
            field,
            observed,
            expected: "3 description lines",
        })
}

fn yaml_one_based_to_zero_based(
    field: &'static str,
    value: usize,
) -> Result<usize, ManagementParseError> {
    value
        .checked_sub(1)
        .ok_or(ManagementParseError::InvalidCount { field, value: 0 })
}

#[derive(Debug)]
struct ParsedManagementHeader {
    datver: String,
    topology_count: usize,
    declared_total_years: usize,
    datver_family: DatverFamily,
}

#[derive(Debug)]
struct ParsedManagementSections {
    counts: ManagementSectionCounts,
    plants: Vec<PlantScenario>,
    operations: Vec<OperationScenario>,
    initials: Vec<InitialScenario>,
    surfaces: Vec<SurfaceScenario>,
    contours: Vec<ContourScenario>,
    drains: Vec<DrainScenario>,
    yearlies: Vec<YearlyScenario>,
}

#[derive(Debug)]
struct ParsedManagementMeta {
    name: String,
    description: [String; 3],
}

#[derive(Debug)]
struct ParsedManagementSchedule {
    nofes: usize,
    schedule: ManagementSchedule,
}

pub fn parse_management_from_str(
    input: &str,
    mode: ParseMode,
) -> Result<ManagementParseOutput, ManagementParseError> {
    let lines = normalize_lines(input);
    let mut cursor = Cursor::new(lines.as_slice(), mode);

    let header = parse_management_header(&mut cursor)?;
    let sections = parse_management_sections(&mut cursor, header.datver_family)?;
    let meta = parse_management_meta(&mut cursor)?;
    let parsed_schedule = parse_management_schedule(
        &mut cursor,
        header.topology_count,
        header.declared_total_years,
        &sections.counts,
    )?;

    validate_cross_section_references(
        &sections.counts,
        &sections.plants,
        &sections.operations,
        &sections.initials,
        &sections.surfaces,
        &sections.yearlies,
        mode,
    )?;

    if let Some(first_unconsumed_line) = cursor.first_unconsumed_line_number() {
        return Err(ManagementParseError::TrailingInput {
            first_unconsumed_line,
        });
    }

    Ok(ManagementParseOutput {
        datver: header.datver,
        topology_count: header.topology_count,
        declared_total_years: header.declared_total_years,
        section_counts: sections.counts,
        registries: ManagementScenarioRegistries {
            plants: sections.plants,
            operations: sections.operations,
            initials: sections.initials,
            surfaces: sections.surfaces,
            contours: sections.contours,
            drains: sections.drains,
            yearlies: sections.yearlies,
            management_meta: ManagementSectionMeta {
                name: meta.name,
                description: meta.description,
                nofes: parsed_schedule.nofes,
            },
        },
        schedule: parsed_schedule.schedule,
    })
}

fn parse_management_header(
    cursor: &mut Cursor<'_>,
) -> Result<ParsedManagementHeader, ManagementParseError> {
    let datver_raw = cursor.next_required("datver")?;
    let datver = cursor.parse_token("datver", datver_raw)?;
    if !ALLOWED_DATVERS.contains(&datver.as_str()) {
        return Err(ManagementParseError::UnsupportedDatver { datver });
    }

    let topology_count = parse_positive_required(cursor, "nofe_or_nchan")?;
    let declared_total_years = parse_positive_required(cursor, "total_years")?;

    Ok(ParsedManagementHeader {
        datver_family: datver_family(&datver),
        datver,
        topology_count,
        declared_total_years,
    })
}

fn parse_management_sections(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<ParsedManagementSections, ManagementParseError> {
    let ncrop = cursor.parse_non_negative_required("ncrop")?;
    let plants = parse_plant_section(cursor, ncrop, datver_family)?;

    let nop = cursor.parse_non_negative_required("nop")?;
    let operations = parse_operation_section(cursor, nop, datver_family)?;

    let nini = cursor.parse_non_negative_required("nini")?;
    let initials = parse_initial_section(cursor, nini, datver_family)?;

    let nseq = cursor.parse_non_negative_required("nseq")?;
    let surfaces = parse_surface_section(cursor, nseq, datver_family)?;

    let ncnt = cursor.parse_non_negative_required("ncnt")?;
    let contours = parse_contour_section(cursor, ncnt, datver_family)?;

    let ndrain = cursor.parse_non_negative_required("ndrain")?;
    let drains = parse_drain_section(cursor, ndrain, datver_family)?;

    let nscen = cursor.parse_non_negative_required("nscen")?;
    let yearlies = parse_yearly_section(cursor, nscen, datver_family)?;

    let section_counts = ManagementSectionCounts {
        ncrop,
        nop,
        nini,
        nseq,
        ncnt,
        ndrain,
        nscen,
    };

    Ok(ParsedManagementSections {
        counts: section_counts,
        plants,
        operations,
        initials,
        surfaces,
        contours,
        drains,
        yearlies,
    })
}

fn parse_management_meta(
    cursor: &mut Cursor<'_>,
) -> Result<ParsedManagementMeta, ManagementParseError> {
    Ok(ParsedManagementMeta {
        name: cursor.next_required("man_name")?.to_string(),
        description: [
            cursor.next_required("man_desc_1")?.to_string(),
            cursor.next_required("man_desc_2")?.to_string(),
            cursor.next_required("man_desc_3")?.to_string(),
        ],
    })
}

fn parse_management_schedule(
    cursor: &mut Cursor<'_>,
    topology_count: usize,
    declared_total_years: usize,
    section_counts: &ManagementSectionCounts,
) -> Result<ParsedManagementSchedule, ManagementParseError> {
    let nofes = cursor.parse_non_negative_required("nofes")?;
    if nofes != topology_count {
        return Err(ManagementParseError::DanglingScenarioReference {
            field: "nofes",
            value: nofes,
            max_allowed: topology_count,
        });
    }

    let mut ofe_initial_refs = Vec::with_capacity(nofes);
    for _ in 0..nofes {
        let ofe_ref = cursor.parse_non_negative_required("ofeindx")?;
        validate_reference("ofeindx", ofe_ref, section_counts.nini)?;
        ofe_initial_refs.push(ofe_ref);
    }

    let rotation_repeats = parse_positive_required(cursor, "nrots")?;
    let rotation_years = parse_positive_required(cursor, "nyears")?;

    let derived_total_years = rotation_repeats.saturating_mul(rotation_years);
    if declared_total_years != derived_total_years {
        return Err(ManagementParseError::TotalYearMismatch {
            declared_total_years,
            derived_total_years,
        });
    }

    let mut slots = Vec::new();
    for rotation_index in 0..rotation_repeats {
        for year_in_rotation in 0..rotation_years {
            for ofe_index in 0..nofes {
                let crop_slots = cursor.parse_non_negative_required("nycrop")?;
                let mut yearly_refs = Vec::with_capacity(crop_slots);
                for _ in 0..crop_slots {
                    let manindx = cursor.parse_non_negative_required("manindx")?;
                    validate_reference("manindx", manindx, section_counts.nscen)?;
                    yearly_refs.push(manindx);
                }

                slots.push(ManagementScheduleSlot {
                    rotation_index,
                    year_in_rotation,
                    ofe_index,
                    crop_slots,
                    yearly_refs,
                });
            }
        }
    }

    Ok(ParsedManagementSchedule {
        nofes,
        schedule: ManagementSchedule {
            ofe_initial_refs,
            rotation_repeats,
            rotation_years,
            slots,
        },
    })
}

fn parse_positive_required(
    cursor: &mut Cursor<'_>,
    field: &'static str,
) -> Result<usize, ManagementParseError> {
    let value = cursor.parse_non_negative_required(field)?;
    if value == 0 {
        return Err(ManagementParseError::InvalidCount { field, value: 0 });
    }
    Ok(value)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DatverFamily {
    V95_7,
    V98_4,
    V2016Plus,
    /// openWEPP-native `ow-lanuse-1` family: first-class `lanuse` modes
    /// unlocked; legacy option extensions parse as the `2016.3+` family.
    OwLanuse1,
}

impl DatverFamily {
    /// The `ow-lanuse-1` native family unlocks the first-class forest carve.
    const fn forest_mode_enabled(self) -> bool {
        matches!(self, Self::OwLanuse1)
    }

    /// The `ow-lanuse-1` native family also allows an explicit native cropland
    /// sentinel, distinct from legacy compatibility cropland (`landuse=1`).
    const fn native_cropland_mode_enabled(self) -> bool {
        matches!(self, Self::OwLanuse1)
    }

    /// Legacy option domains (operation pcode, resmgt, mgtopt) for the native
    /// family follow the `2016.3+` datver superset.
    const fn legacy_option_family(self) -> Self {
        match self {
            Self::OwLanuse1 => Self::V2016Plus,
            other => other,
        }
    }
}

fn cropland_landuse_allowed(datver_family: DatverFamily, landuse: usize) -> bool {
    landuse == 1
        || (datver_family.native_cropland_mode_enabled()
            && landuse == NATIVE_CROPLAND_LANUSE_SENTINEL)
}

fn cropland_allowed_label(datver_family: DatverFamily) -> &'static str {
    if datver_family.native_cropland_mode_enabled() {
        "1 or 4 (native cropland)"
    } else {
        "1"
    }
}

fn datver_family(datver: &str) -> DatverFamily {
    match datver {
        "95.7" => DatverFamily::V95_7,
        "98.4" => DatverFamily::V98_4,
        OW_LANUSE_1_DATVER => DatverFamily::OwLanuse1,
        _ => DatverFamily::V2016Plus,
    }
}

/// Forest section policy for operation / surface / contour / drain: forest
/// defines no tillage, contour, or drain scenario, so a forest sentinel in
/// these sections fails closed (`LANUSE-AUTH-2`) rather than inventing an empty
/// scenario. Under legacy datvers the forest sentinel never reaches this guard
/// (it is rejected earlier by the per-section `landuse != 1` gate).
fn forest_section_not_applicable_guard(
    datver_family: DatverFamily,
    landuse: usize,
    section: &'static str,
) -> Result<(), ManagementParseError> {
    if datver_family.forest_mode_enabled() && landuse == FOREST_LANUSE_SENTINEL {
        return Err(ManagementParseError::ForestSectionNotApplicable { section });
    }
    Ok(())
}

fn parse_plant_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<PlantScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "plant")?;
        if datver_family.forest_mode_enabled() && meta.landuse == FOREST_LANUSE_SENTINEL {
            let data = parse_plant_forest(cursor, datver_family)?;
            scenarios.push(PlantScenario {
                meta,
                data: PlantScenarioData::Forest(data),
            });
            continue;
        }
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "plant",
                landuse: meta.landuse,
            });
        }
        if !cropland_landuse_allowed(datver_family, meta.landuse) {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "iplant",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: cropland_allowed_label(datver_family),
            });
        }

        let crunit = cursor.next_required("plant.crunit")?.to_string();
        let canopy_line = parse_f64_array::<10>(cursor, "plant.canopy_line")?;
        let growth_line = parse_f64_array::<8>(cursor, "plant.growth_line")?;
        let mfocod = cursor.parse_non_negative_required("plant.mfocod")?;
        if !(1..=2).contains(&mfocod) {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "mfocod",
                value: i64::try_from(mfocod).unwrap_or(i64::MAX),
                allowed: "1..2",
            });
        }
        let residue_line = parse_f64_array::<10>(cursor, "plant.residue_line")?;
        let terminal_tokens = cursor.parse_tokens_required("plant.terminal_line")?;
        if terminal_tokens.len() != 3 && terminal_tokens.len() != 4 {
            return Err(ManagementParseError::RecordArityError {
                field: "plant.terminal_line",
                observed: terminal_tokens.len(),
                expected: "3 or 4",
            });
        }
        let mut terminal_values = [0.0_f64; 3];
        for (idx, slot) in terminal_values.iter_mut().enumerate() {
            *slot = parse_f64_token("plant.terminal_line", &terminal_tokens[idx])?;
        }
        let rcc = if terminal_tokens.len() == 4 {
            Some(parse_f64_token("plant.rcc", &terminal_tokens[3])?)
        } else {
            None
        };

        let routing = parse_optional_routing_coefficients(
            cursor,
            datver_family.native_cropland_mode_enabled()
                && meta.landuse == NATIVE_CROPLAND_LANUSE_SENTINEL,
        )?;

        scenarios.push(PlantScenario {
            meta,
            data: PlantScenarioData::Cropland(PlantCroplandData {
                crunit,
                canopy_line,
                growth_line,
                mfocod,
                residue_line,
                terminal_line: terminal_values,
                rcc,
                routing,
            }),
        });
    }
    Ok(scenarios)
}

/// Parse the openWEPP-native forest plant block (`ow-lanuse-1`, forest
/// sentinel). Fixed-arity numeric lines make every Tier-A operand explicit and
/// required; a missing or non-numeric value fails closed at parse
/// (`LANUSE-AUTH-2`). Domain validation (fraction/positive bounds) is enforced
/// downstream in the runtime projection, mirroring the cropland parser.
fn parse_plant_forest(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<PlantForestData, ManagementParseError> {
    let class_raw = cursor.next_required("plant.forest.class")?;
    let forest_class = cursor.parse_token("plant.forest.class", class_raw)?;

    // Tier-A growth operands (openWEPP forest authority; explicit, fail-closed).
    let growth_a = parse_f64_array::<5>(cursor, "plant.forest.growth_bb_bbb_beinp_btemp_extnct")?;
    let growth_b = parse_f64_array::<5>(cursor, "plant.forest.growth_flivmx_hmax_hi_gddmax_dlai")?;
    let growth_c =
        parse_f64_array::<5>(cursor, "plant.forest.growth_otemp_pltol_spriod_rsr_rtmmax")?;
    // Tier-A lookup-owned operands (authoritative `(texture × class)` table;
    // reconciled by the orchestrator against `forest_class`, `LANUSE-AUTH-6`).
    let lookup = parse_f64_array::<4>(cursor, "plant.forest.lookup_xmxlai_rdmax_decfct_dropfc")?;
    // Residue-depth seed operands.
    let residue = parse_f64_array::<2>(cursor, "plant.forest.residue_cf_diam")?;
    // Tier-A decomposition operands.
    let decomposition = parse_f64_array::<2>(cursor, "plant.forest.decomposition_oratea_orater")?;
    // Tier-B plant-community operands (WS-4; rangeland-shaped structural reference).
    let community = parse_f64_array::<4>(cursor, "plant.forest.community_tempmn_gtemp_plive_wood")?;
    let grass = parse_f64_array::<4>(cursor, "plant.forest.grass_coeff_diam_hgt_pop")?;
    let shrub = parse_f64_array::<4>(cursor, "plant.forest.shrub_coeff_diam_hgt_pop")?;
    let tree = parse_f64_array::<4>(cursor, "plant.forest.tree_coeff_diam_hgt_pop")?;
    let routing = parse_optional_routing_coefficients(cursor, datver_family.forest_mode_enabled())?;

    Ok(PlantForestData {
        forest_class,
        growth: PlantForestGrowth {
            bb: growth_a[0],
            bbb: growth_a[1],
            beinp: growth_a[2],
            btemp: growth_a[3],
            extnct: growth_a[4],
            flivmx: growth_b[0],
            hmax: growth_b[1],
            hi: growth_b[2],
            gddmax: growth_b[3],
            dlai: growth_b[4],
            otemp: growth_c[0],
            pltol: growth_c[1],
            spriod: growth_c[2],
            rsr: growth_c[3],
            rtmmax: growth_c[4],
            xmxlai: lookup[0],
            rdmax: lookup[1],
            decfct: lookup[2],
            dropfc: lookup[3],
        },
        cf: residue[0],
        diam: residue[1],
        decomposition: PlantForestDecomposition {
            oratea: decomposition[0],
            orater: decomposition[1],
        },
        community: PlantForestCommunity {
            tempmn: community[0],
            gtemp: community[1],
            plive: community[2],
            wood: community[3],
            grass: PlantForestStratum {
                coeff: grass[0],
                diam: grass[1],
                hgt: grass[2],
                pop: grass[3],
            },
            shrub: PlantForestStratum {
                coeff: shrub[0],
                diam: shrub[1],
                hgt: shrub[2],
                pop: shrub[3],
            },
            tree: PlantForestStratum {
                coeff: tree[0],
                diam: tree[1],
                hgt: tree[2],
                pop: tree[3],
            },
        },
        routing,
    })
}

fn parse_operation_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<OperationScenario>, ManagementParseError> {
    let pcode_family = datver_family.legacy_option_family();
    (0..count)
        .map(|_| parse_operation_scenario(cursor, datver_family, pcode_family))
        .collect()
}

fn parse_operation_scenario(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
    pcode_family: DatverFamily,
) -> Result<OperationScenario, ManagementParseError> {
    let meta = parse_scenario_meta(cursor, "operation")?;
    validate_non_forest_cropland_landuse(datver_family, meta.landuse, "operation", "iop")?;

    let (mfo1, mfo2, numof) = parse_operation_mfo_line(cursor)?;
    let (pcode, cltpos) = parse_operation_code(cursor, pcode_family)?;
    let effect_line = parse_operation_effect_line(cursor)?;
    let extension_lines = parse_operation_extension_lines(cursor, pcode)?;

    Ok(OperationScenario {
        meta,
        data: OperationScenarioData::Cropland(OperationCroplandData {
            mfo1,
            mfo2,
            numof,
            pcode,
            cltpos,
            effect_line,
            extension_lines,
        }),
    })
}

fn validate_non_forest_cropland_landuse(
    datver_family: DatverFamily,
    landuse: usize,
    section: &'static str,
    option_field: &'static str,
) -> Result<(), ManagementParseError> {
    forest_section_not_applicable_guard(datver_family, landuse, section)?;
    if landuse == 2 {
        return Err(ManagementParseError::UnsupportedLanduse { section, landuse });
    }
    if !cropland_landuse_allowed(datver_family, landuse) {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: option_field,
            value: i64::try_from(landuse).unwrap_or(i64::MAX),
            allowed: cropland_allowed_label(datver_family),
        });
    }
    Ok(())
}

fn parse_operation_mfo_line(
    cursor: &mut Cursor<'_>,
) -> Result<(f64, f64, usize), ManagementParseError> {
    let tokens = cursor.parse_tokens_required("op.mfo_line")?;
    if tokens.len() != 3 {
        return Err(ManagementParseError::RecordArityError {
            field: "op.mfo_line",
            observed: tokens.len(),
            expected: "3",
        });
    }
    Ok((
        parse_f64_token("op.mfo1", &tokens[0])?,
        parse_f64_token("op.mfo2", &tokens[1])?,
        parse_usize_token("op.numof", &tokens[2])?,
    ))
}

fn parse_operation_code(
    cursor: &mut Cursor<'_>,
    pcode_family: DatverFamily,
) -> Result<(usize, Option<usize>), ManagementParseError> {
    let tokens = cursor.parse_tokens_required("op.pcode")?;
    if tokens.is_empty() || tokens.len() > 2 {
        return Err(ManagementParseError::RecordArityError {
            field: "op.pcode",
            observed: tokens.len(),
            expected: "1 or 2",
        });
    }

    let pcode = parse_usize_token("op.pcode", &tokens[0])?;
    validate_operation_pcode(pcode_family, pcode)?;
    Ok((pcode, parse_operation_cltpos(pcode, &tokens)?))
}

fn validate_operation_pcode(
    pcode_family: DatverFamily,
    pcode: usize,
) -> Result<(), ManagementParseError> {
    let valid_pcodes = match pcode_family {
        DatverFamily::V95_7 => &[1, 2, 3, 4][..],
        DatverFamily::V98_4 => &[1, 2, 3, 4, 10, 11, 12, 13][..],
        DatverFamily::V2016Plus | DatverFamily::OwLanuse1 => {
            &[1, 2, 3, 4, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19][..]
        }
    };
    if !valid_pcodes.contains(&pcode) {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "pcode",
            value: i64::try_from(pcode).unwrap_or(i64::MAX),
            allowed: "datver-specific operation-code allowlist",
        });
    }
    Ok(())
}

fn parse_operation_cltpos(
    pcode: usize,
    tokens: &[String],
) -> Result<Option<usize>, ManagementParseError> {
    if pcode != 3 {
        if tokens.len() != 1 {
            return Err(ManagementParseError::RecordArityError {
                field: "op.pcode",
                observed: tokens.len(),
                expected: "1",
            });
        }
        return Ok(None);
    }

    if tokens.len() != 2 {
        return Err(ManagementParseError::RecordArityError {
            field: "op.cltpos",
            observed: tokens.len(),
            expected: "2",
        });
    }
    let cltpos = parse_usize_token("op.cltpos", &tokens[1])?;
    if cltpos != 1 && cltpos != 2 {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "cltpos",
            value: i64::try_from(cltpos).unwrap_or(i64::MAX),
            allowed: "1 or 2",
        });
    }
    Ok(Some(cltpos))
}

fn parse_operation_effect_line(cursor: &mut Cursor<'_>) -> Result<Vec<f64>, ManagementParseError> {
    let tokens = cursor.parse_tokens_required("op.effect_line")?;
    if tokens.len() != 7 && tokens.len() != 9 {
        return Err(ManagementParseError::RecordArityError {
            field: "op.effect_line",
            observed: tokens.len(),
            expected: "7 or 9",
        });
    }
    tokens
        .iter()
        .map(|token| parse_f64_token("op.effect_line", token))
        .collect()
}

fn parse_operation_extension_lines(
    cursor: &mut Cursor<'_>,
    pcode: usize,
) -> Result<Vec<String>, ManagementParseError> {
    let mut lines = Vec::new();
    if !operation_reads_extension_line(pcode) {
        return Ok(lines);
    }

    let first = cursor.next_required("op.ext_line_1")?.to_string();
    let needs_second =
        operation_may_read_second_extension_line(pcode) && first.split_whitespace().count() < 2;
    lines.push(first);
    if needs_second {
        lines.push(cursor.next_required("op.ext_line_2")?.to_string());
    }
    Ok(lines)
}

const fn operation_reads_extension_line(pcode: usize) -> bool {
    matches!(pcode, 10 | 11 | 12 | 13 | 14 | 15 | 18 | 19)
}

const fn operation_may_read_second_extension_line(pcode: usize) -> bool {
    matches!(pcode, 10 | 12 | 15 | 18 | 19)
}

fn parse_initial_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<InitialScenario>, ManagementParseError> {
    (0..count)
        .map(|_| parse_initial_scenario(cursor, datver_family))
        .collect()
}

fn parse_initial_scenario(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<InitialScenario, ManagementParseError> {
    let meta = parse_scenario_meta(cursor, "initial")?;
    if datver_family.forest_mode_enabled() && meta.landuse == FOREST_LANUSE_SENTINEL {
        return Ok(InitialScenario {
            meta,
            data: InitialScenarioData::Forest(parse_initial_forest(cursor)?),
        });
    }

    validate_non_forest_cropland_landuse(datver_family, meta.landuse, "initial", "lanuse")?;
    Ok(InitialScenario {
        meta,
        data: InitialScenarioData::Cropland(parse_initial_cropland(cursor)?),
    })
}

fn parse_initial_cropland(
    cursor: &mut Cursor<'_>,
) -> Result<InitialCroplandData, ManagementParseError> {
    let base_line = parse_f64_array::<6>(cursor, "ini.base_line")?;
    let iresd = cursor.parse_non_negative_required("iresd")?;
    let imngmt = cursor.parse_non_negative_required("imngmt")?;
    if !(1..=3).contains(&imngmt) {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "imngmt",
            value: i64::try_from(imngmt).unwrap_or(i64::MAX),
            allowed: "1..3",
        });
    }
    let residue_line = parse_f64_array::<5>(cursor, "ini.residue_line")?;
    let rtyp = cursor.parse_non_negative_required("rtyp")?;
    if !(1..=2).contains(&rtyp) {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "rtyp",
            value: i64::try_from(rtyp).unwrap_or(i64::MAX),
            allowed: "1..2",
        });
    }
    let thaw_line = parse_f64_array::<5>(cursor, "ini.thaw_line")?;
    let (terminal_line, understory_line) = parse_initial_terminal_line(cursor)?;

    Ok(InitialCroplandData {
        base_line,
        iresd,
        imngmt,
        residue_line,
        rtyp,
        thaw_line,
        terminal_line,
        understory_line,
    })
}

fn parse_initial_terminal_line(
    cursor: &mut Cursor<'_>,
) -> Result<([f64; 2], Option<[f64; 2]>), ManagementParseError> {
    let tokens = cursor.parse_tokens_required("ini.terminal_line")?;
    if tokens.len() != 2 && tokens.len() != 4 {
        return Err(ManagementParseError::RecordArityError {
            field: "ini.terminal_line",
            observed: tokens.len(),
            expected: "2 or 4",
        });
    }
    let terminal_line = [
        parse_f64_token("ini.sumrtm", &tokens[0])?,
        parse_f64_token("ini.sumsrm", &tokens[1])?,
    ];
    let understory_line = if tokens.len() == 4 {
        Some([
            parse_f64_token("ini.usinrcol", &tokens[2])?,
            parse_f64_token("ini.usrilcol", &tokens[3])?,
        ])
    } else {
        None
    };
    Ok((terminal_line, understory_line))
}

/// Parse the openWEPP-native forest initial-condition block (`ow-lanuse-1`).
/// Cover/roughness are first-class (`LANUSE-AUTH-6`); ridge geometry
/// (`rspace`/`rtyp`) is intentionally absent (`LANUSE-AUTH-3`). Forest is an
/// established perennial stand, so `imngmt` must be `2`.
fn parse_initial_forest(
    cursor: &mut Cursor<'_>,
) -> Result<InitialForestData, ManagementParseError> {
    let cover = parse_f64_array::<4>(cursor, "ini.forest.cover_cancov_inrcov_rilcov_rrinit")?;
    let iresd = cursor.parse_non_negative_required("ini.forest.iresd")?;
    let imngmt = cursor.parse_non_negative_required("ini.forest.imngmt")?;
    if imngmt != 2 {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "imngmt",
            value: i64::try_from(imngmt).unwrap_or(i64::MAX),
            allowed: "2 (forest established perennial)",
        });
    }
    let thermal = parse_f64_array::<2>(cursor, "ini.forest.thermal_tillay1_tillay2")?;

    let terminal_tokens = cursor.parse_tokens_required("ini.forest.terminal_line")?;
    if terminal_tokens.len() != 2 && terminal_tokens.len() != 4 {
        return Err(ManagementParseError::RecordArityError {
            field: "ini.forest.terminal_line",
            observed: terminal_tokens.len(),
            expected: "2 or 4",
        });
    }
    let sumrtm = parse_f64_token("ini.forest.sumrtm", &terminal_tokens[0])?;
    let sumsrm = parse_f64_token("ini.forest.sumsrm", &terminal_tokens[1])?;
    let understory_line = if terminal_tokens.len() == 4 {
        Some([
            parse_f64_token("ini.forest.usinrcol", &terminal_tokens[2])?,
            parse_f64_token("ini.forest.usrilcol", &terminal_tokens[3])?,
        ])
    } else {
        None
    };

    Ok(InitialForestData {
        cancov: cover[0],
        inrcov: cover[1],
        rilcov: cover[2],
        rrinit: cover[3],
        iresd,
        imngmt,
        sumrtm,
        sumsrm,
        tillay1: thermal[0],
        tillay2: thermal[1],
        understory_line,
    })
}

fn parse_surface_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<SurfaceScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "surface")?;
        validate_non_forest_cropland_landuse(datver_family, meta.landuse, "surface", "iseq")?;

        let ntill = cursor.parse_non_negative_required("ntill")?;
        let mut operations = Vec::with_capacity(ntill);
        for _ in 0..ntill {
            let mdate = parse_julian_day(cursor, "mdate", false)?;
            let op_ref = cursor.parse_non_negative_required("op")?;
            let tildep = cursor.parse_f64_required("tildep")?;
            let typtil = cursor.parse_non_negative_required("typtil")?;
            if typtil != 1 && typtil != 2 {
                return Err(ManagementParseError::InvalidOptionDomain {
                    field: "typtil",
                    value: i64::try_from(typtil).unwrap_or(i64::MAX),
                    allowed: "1 or 2",
                });
            }

            operations.push(SurfaceOperation {
                mdate,
                op_ref,
                tildep,
                typtil,
            });
        }

        scenarios.push(SurfaceScenario {
            meta,
            ntill,
            operations,
        });
    }
    Ok(scenarios)
}

fn parse_contour_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<ContourScenario>, ManagementParseError> {
    (0..count)
        .map(|_| parse_contour_scenario(cursor, datver_family))
        .collect()
}

fn parse_contour_scenario(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<ContourScenario, ManagementParseError> {
    let meta = parse_scenario_meta(cursor, "contour")?;
    validate_non_forest_cropland_landuse(datver_family, meta.landuse, "contour", "icont")?;
    let (cntslp, rdghgt, rowlen, rowspc, contours_perm) =
        parse_contour_values(cursor, datver_family)?;
    Ok(ContourScenario {
        meta,
        cntslp,
        rdghgt,
        rowlen,
        rowspc,
        contours_perm,
    })
}

fn parse_contour_values(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<(f64, f64, f64, f64, Option<usize>), ManagementParseError> {
    let tokens = cursor.parse_tokens_required("contour.values")?;
    if tokens.len() != 4 && tokens.len() != 5 {
        return Err(ManagementParseError::RecordArityError {
            field: "contour.values",
            observed: tokens.len(),
            expected: "4 or 5",
        });
    }
    if tokens.len() == 5 && datver_family.legacy_option_family() != DatverFamily::V2016Plus {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "contours_perm",
            value: 1,
            allowed: "2016.3+ datver only",
        });
    }

    Ok((
        parse_f64_token("cntslp", &tokens[0])?,
        parse_f64_token("rdghgt", &tokens[1])?,
        parse_f64_token("rowlen", &tokens[2])?,
        parse_f64_token("rowspc", &tokens[3])?,
        parse_optional_contours_perm(&tokens)?,
    ))
}

fn parse_optional_contours_perm(tokens: &[String]) -> Result<Option<usize>, ManagementParseError> {
    if tokens.len() == 5 {
        return Ok(Some(parse_usize_token("contours_perm", &tokens[4])?));
    }
    Ok(None)
}

fn parse_drain_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<DrainScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "drain")?;
        validate_non_forest_cropland_landuse(datver_family, meta.landuse, "drain", "dcont")?;

        let values = parse_f64_array::<4>(cursor, "drain.values")?;
        scenarios.push(DrainScenario {
            meta,
            ddrain: values[0],
            drainc: values[1],
            drdiam: values[2],
            sdrain: values[3],
        });
    }
    Ok(scenarios)
}

fn parse_yearly_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<YearlyScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "yearly")?;
        if datver_family.forest_mode_enabled() && meta.landuse == FOREST_LANUSE_SENTINEL {
            let data = parse_yearly_forest(cursor)?;
            scenarios.push(YearlyScenario {
                meta,
                data: YearlyScenarioData::Forest(data),
            });
            continue;
        }
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "yearly",
                landuse: meta.landuse,
            });
        }
        if !cropland_landuse_allowed(datver_family, meta.landuse) {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "iscen",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: cropland_allowed_label(datver_family),
            });
        }

        let itype = cursor.parse_non_negative_required("itype")?;
        let tilseq = cursor.parse_non_negative_required("tilseq")?;
        let conset = cursor.parse_non_negative_required("conset")?;
        let drset = cursor.parse_non_negative_required("drset")?;
        let imngmt = cursor.parse_non_negative_required("imngmt")?;
        if !(1..=3).contains(&imngmt) {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "imngmt",
                value: i64::try_from(imngmt).unwrap_or(i64::MAX),
                allowed: "1..3",
            });
        }

        let branch = if imngmt == 1 || imngmt == 3 {
            YearlyCroplandBranch::AnnualOrFallow(parse_yearly_annual_fallow(cursor, datver_family)?)
        } else {
            YearlyCroplandBranch::Perennial(parse_yearly_perennial(cursor, datver_family)?)
        };

        scenarios.push(YearlyScenario {
            meta,
            data: YearlyScenarioData::Cropland(YearlyCroplandData {
                itype,
                tilseq,
                conset,
                drset,
                imngmt,
                branch,
            }),
        });
    }
    Ok(scenarios)
}

/// Parse the openWEPP-native forest yearly schedule (`ow-lanuse-1`): an
/// established perennial stand. Forest carries no surface-effect / contour /
/// drain reference (`tilseq/conset/drset` must be `0`), is perennial
/// (`imngmt = 2`), and has no annual cut/graze management (`mgtopt = 3` idle;
/// stand management is deferred to WS-4). Fails closed on any other combination.
fn parse_yearly_forest(cursor: &mut Cursor<'_>) -> Result<YearlyForestData, ManagementParseError> {
    let itype = cursor.parse_non_negative_required("yearly.forest.itype")?;
    forest_yearly_zero_ref_guard(cursor, "tilseq")?;
    forest_yearly_zero_ref_guard(cursor, "conset")?;
    forest_yearly_zero_ref_guard(cursor, "drset")?;

    let imngmt = cursor.parse_non_negative_required("yearly.forest.imngmt")?;
    if imngmt != 2 {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "imngmt",
            value: i64::try_from(imngmt).unwrap_or(i64::MAX),
            allowed: "2 (forest established perennial)",
        });
    }

    let jdharv = parse_julian_day(cursor, "yearly.forest.jdharv", true)?;
    let jdplt = parse_julian_day(cursor, "yearly.forest.jdplt", true)?;
    let jdstop = parse_julian_day(cursor, "yearly.forest.jdstop", true)?;
    let rw = cursor.parse_f64_required("yearly.forest.rw")?;

    let mgtopt = cursor.parse_non_negative_required("yearly.forest.mgtopt")?;
    if mgtopt != 3 {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "mgtopt",
            value: i64::try_from(mgtopt).unwrap_or(i64::MAX),
            allowed: "3 (forest idle perennial; cut/graze management is WS-4)",
        });
    }

    Ok(YearlyForestData {
        itype,
        jdharv,
        jdplt,
        jdstop,
        rw,
    })
}

/// Forest yearly schedule references no surface-effect / contour / drain
/// scenario, so the corresponding index must be the `0` sentinel.
fn forest_yearly_zero_ref_guard(
    cursor: &mut Cursor<'_>,
    field: &'static str,
) -> Result<(), ManagementParseError> {
    let value = cursor.parse_non_negative_required(field)?;
    if value != 0 {
        return Err(ManagementParseError::InvalidOptionDomain {
            field,
            value: i64::try_from(value).unwrap_or(i64::MAX),
            allowed: "0 (forest has no surface-effect/contour/drain scenario)",
        });
    }
    Ok(())
}

fn parse_yearly_annual_fallow(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<YearlyAnnualFallowData, ManagementParseError> {
    let header = parse_yearly_annual_fallow_header(cursor)?;
    validate_yearly_annual_resmgt(datver_family, header.resmgt)?;
    let extension = parse_yearly_annual_extension(cursor, header.resmgt)?;

    Ok(YearlyAnnualFallowData {
        jdharv: header.jdharv,
        jdplt: header.jdplt,
        rw: header.rw,
        resmgt: header.resmgt,
        extension,
    })
}

#[derive(Debug, Clone, Copy)]
struct YearlyAnnualFallowHeader {
    jdharv: usize,
    jdplt: usize,
    rw: f64,
    resmgt: usize,
}

fn parse_yearly_annual_fallow_header(
    cursor: &mut Cursor<'_>,
) -> Result<YearlyAnnualFallowHeader, ManagementParseError> {
    let jdharv = parse_julian_day(cursor, "jdharv", false)?;
    let jdplt = parse_julian_day(cursor, "jdplt", false)?;
    let rw = cursor.parse_f64_required("rw")?;
    let resmgt = cursor.parse_non_negative_required("resmgt")?;

    Ok(YearlyAnnualFallowHeader {
        jdharv,
        jdplt,
        rw,
        resmgt,
    })
}

fn validate_yearly_annual_resmgt(
    datver_family: DatverFamily,
    resmgt: usize,
) -> Result<(), ManagementParseError> {
    // `ow-lanuse-1` follows the 2016.3+ legacy option domain (§1.4).
    let valid_resmgt = match datver_family.legacy_option_family() {
        DatverFamily::V2016Plus => (1, 7),
        _ => (1, 6),
    };
    if resmgt < valid_resmgt.0 || resmgt > valid_resmgt.1 {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "resmgt",
            value: i64::try_from(resmgt).unwrap_or(i64::MAX),
            allowed: "1..6 (95.7/98.4) or 1..7 (2016.3+)",
        });
    }
    Ok(())
}

fn parse_yearly_annual_extension(
    cursor: &mut Cursor<'_>,
    resmgt: usize,
) -> Result<Option<YearlyAnnualExtension>, ManagementParseError> {
    let extension = match resmgt {
        1 => Some(YearlyAnnualExtension::Herbicide {
            jdherb: parse_julian_day(cursor, "jdherb", false)?,
        }),
        2 => Some(YearlyAnnualExtension::Burn {
            jdburn: parse_julian_day(cursor, "jdburn", false)?,
            fbmag: cursor.parse_f64_required("fbmag")?,
            fbrnog: cursor.parse_f64_required("fbrnog")?,
        }),
        3 => Some(YearlyAnnualExtension::Silage {
            jdslge: parse_julian_day(cursor, "jdslge", false)?,
        }),
        4 => Some(YearlyAnnualExtension::Cut {
            jdcut: parse_julian_day(cursor, "jdcut", false)?,
            frcut: cursor.parse_f64_required("frcut")?,
        }),
        5 => Some(YearlyAnnualExtension::Remove {
            jdmove: parse_julian_day(cursor, "jdmove", false)?,
            frmove: cursor.parse_f64_required("frmove")?,
        }),
        6 => None,
        7 => {
            parse_yearly_annual_cut_records(cursor)?;
            None
        }
        _ => unreachable!(),
    };
    Ok(extension)
}

fn parse_yearly_annual_cut_records(cursor: &mut Cursor<'_>) -> Result<(), ManagementParseError> {
    let _cut_flag = cursor.parse_non_negative_required("annual_cut.flag")?;
    let ncut = cursor.parse_non_negative_required("annual_cut.ncut")?;
    if ncut == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "annual_cut.ncut",
            value: 0,
        });
    }
    for _ in 0..ncut {
        parse_yearly_annual_cut_entry(cursor)?;
    }
    Ok(())
}

fn parse_yearly_annual_cut_entry(cursor: &mut Cursor<'_>) -> Result<(), ManagementParseError> {
    let tokens = cursor.parse_tokens_required("annual_cut.entry")?;
    if tokens.len() < 2 {
        return Err(ManagementParseError::RecordArityError {
            field: "annual_cut.entry",
            observed: tokens.len(),
            expected: "2+",
        });
    }
    let day = parse_i64_token("annual_cut.day", &tokens[0])?;
    validate_julian_day("annual_cut.day", day, false)?;
    let _fraction = parse_f64_token("annual_cut.fraction", &tokens[1])?;
    Ok(())
}

fn parse_yearly_perennial(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<YearlyPerennialData, ManagementParseError> {
    let header = parse_yearly_perennial_header(cursor)?;
    validate_yearly_perennial_mgtopt(datver_family, header.mgtopt)?;
    let payload = parse_yearly_perennial_payload(cursor, header.mgtopt)?;

    Ok(YearlyPerennialData {
        jdharv: header.jdharv,
        jdplt: header.jdplt,
        jdstop: header.jdstop,
        rw: header.rw,
        mgtopt: header.mgtopt,
        cut_days: payload.cut_days,
        grazing_cycles: payload.grazing_cycles,
    })
}

#[derive(Debug, Clone, Copy)]
struct YearlyPerennialHeader {
    jdharv: usize,
    jdplt: usize,
    jdstop: usize,
    rw: f64,
    mgtopt: usize,
}

#[derive(Debug, Clone)]
struct YearlyPerennialPayload {
    cut_days: Vec<usize>,
    grazing_cycles: Vec<YearlyPerennialGrazingCycle>,
}

fn parse_yearly_perennial_header(
    cursor: &mut Cursor<'_>,
) -> Result<YearlyPerennialHeader, ManagementParseError> {
    let jdharv = parse_julian_day(cursor, "jdharv", true)?;
    let jdplt = parse_julian_day(cursor, "jdplt", true)?;
    let jdstop = parse_julian_day(cursor, "jdstop", true)?;
    let rw = cursor.parse_f64_required("rw")?;
    let mgtopt = cursor.parse_non_negative_required("mgtopt")?;

    Ok(YearlyPerennialHeader {
        jdharv,
        jdplt,
        jdstop,
        rw,
        mgtopt,
    })
}

fn validate_yearly_perennial_mgtopt(
    datver_family: DatverFamily,
    mgtopt: usize,
) -> Result<(), ManagementParseError> {
    // `ow-lanuse-1` follows the 2016.3+ legacy option domain (§1.4).
    let datver_family = datver_family.legacy_option_family();
    let allowed = match datver_family {
        DatverFamily::V2016Plus => "1..7",
        _ => "1..3",
    };
    let is_in_domain = match datver_family {
        DatverFamily::V2016Plus => (1..=7).contains(&mgtopt),
        _ => (1..=3).contains(&mgtopt),
    };
    if !is_in_domain {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "mgtopt",
            value: i64::try_from(mgtopt).unwrap_or(i64::MAX),
            allowed,
        });
    }
    Ok(())
}

fn parse_yearly_perennial_payload(
    cursor: &mut Cursor<'_>,
    mgtopt: usize,
) -> Result<YearlyPerennialPayload, ManagementParseError> {
    match mgtopt {
        1 => Ok(YearlyPerennialPayload {
            cut_days: parse_yearly_perennial_cut_days(cursor)?,
            grazing_cycles: Vec::new(),
        }),
        2 => Ok(YearlyPerennialPayload {
            cut_days: Vec::new(),
            grazing_cycles: parse_yearly_perennial_grazing_cycles(cursor)?,
        }),
        3 => Ok(YearlyPerennialPayload {
            cut_days: Vec::new(),
            grazing_cycles: Vec::new(),
        }),
        4..=7 => Err(ManagementParseError::InvalidOptionDomain {
            field: "mgtopt",
            value: i64::try_from(mgtopt).unwrap_or(i64::MAX),
            allowed: "openWEPP parser currently supports perennial mgtopt 1..3",
        }),
        _ => unreachable!(),
    }
}

fn parse_yearly_perennial_cut_days(
    cursor: &mut Cursor<'_>,
) -> Result<Vec<usize>, ManagementParseError> {
    let ncut = cursor.parse_non_negative_required("ncut")?;
    if ncut == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "ncut",
            value: 0,
        });
    }
    let mut cut_days = Vec::with_capacity(ncut);
    for _ in 0..ncut {
        cut_days.push(parse_yearly_perennial_cut_day(cursor)?);
    }
    Ok(cut_days)
}

fn parse_yearly_perennial_cut_day(cursor: &mut Cursor<'_>) -> Result<usize, ManagementParseError> {
    let tokens = cursor.parse_tokens_required("cutday")?;
    if tokens.is_empty() || tokens.len() > 2 {
        return Err(ManagementParseError::RecordArityError {
            field: "cutday",
            observed: tokens.len(),
            expected: "1 or 2",
        });
    }
    let cutday = parse_i64_token("cutday", &tokens[0])?;
    validate_julian_day("cutday", cutday, false)?;
    usize::try_from(cutday).map_err(|_| ManagementParseError::TokenParseError {
        field: "cutday",
        value: tokens[0].clone(),
    })
}

fn parse_yearly_perennial_grazing_cycles(
    cursor: &mut Cursor<'_>,
) -> Result<Vec<YearlyPerennialGrazingCycle>, ManagementParseError> {
    let ncycle = cursor.parse_non_negative_required("ncycle")?;
    if ncycle == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "ncycle",
            value: 0,
        });
    }
    let mut grazing_cycles = Vec::with_capacity(ncycle);
    for _ in 0..ncycle {
        grazing_cycles.push(parse_yearly_perennial_grazing_cycle(cursor)?);
    }
    Ok(grazing_cycles)
}

fn parse_yearly_perennial_grazing_cycle(
    cursor: &mut Cursor<'_>,
) -> Result<YearlyPerennialGrazingCycle, ManagementParseError> {
    let cycle_tokens = cursor.parse_tokens_required("graze_cycle")?;
    if cycle_tokens.len() != 4 {
        return Err(ManagementParseError::RecordArityError {
            field: "graze_cycle",
            observed: cycle_tokens.len(),
            expected: "4",
        });
    }
    let animal = parse_f64_token("animal", &cycle_tokens[0])?;
    let area = parse_f64_token("area", &cycle_tokens[1])?;
    let bodywt = parse_f64_token("bodywt", &cycle_tokens[2])?;
    let digest = parse_f64_token("digest", &cycle_tokens[3])?;
    let gday = parse_julian_day(cursor, "gday", false)?;
    let gend = parse_julian_day(cursor, "gend", false)?;
    Ok(YearlyPerennialGrazingCycle {
        animal,
        area,
        bodywt,
        digest,
        gday,
        gend,
    })
}

fn validate_cross_section_references(
    counts: &ManagementSectionCounts,
    plants: &[PlantScenario],
    operations: &[OperationScenario],
    initials: &[InitialScenario],
    surfaces: &[SurfaceScenario],
    yearlies: &[YearlyScenario],
    mode: ParseMode,
) -> Result<(), ManagementParseError> {
    if plants.len() != counts.ncrop
        || operations.len() != counts.nop
        || initials.len() != counts.nini
        || surfaces.len() != counts.nseq
        || yearlies.len() != counts.nscen
    {
        return Err(ManagementParseError::MissingRecord {
            field: "section_count_closure",
        });
    }

    for initial in initials {
        let iresd = match &initial.data {
            InitialScenarioData::Cropland(data) => data.iresd,
            InitialScenarioData::Forest(data) => data.iresd,
        };
        validate_reference("iresd", iresd, counts.ncrop)?;
    }

    for surface in surfaces {
        for op in &surface.operations {
            validate_reference("op", op.op_ref, counts.nop)?;
        }
    }

    for yearly in yearlies {
        match &yearly.data {
            YearlyScenarioData::Cropland(data) => {
                validate_reference("itype", data.itype, counts.ncrop)?;
                // Legacy 98.x payloads may use `tilseq=0` to indicate no surface-effect scenario.
                let compat_tilseq_zero_sentinel =
                    mode == ParseMode::Compatibility && counts.nseq > 0 && data.tilseq == 0;
                if !compat_tilseq_zero_sentinel {
                    validate_reference("tilseq", data.tilseq, counts.nseq)?;
                }
                validate_reference("conset", data.conset, counts.ncnt)?;
                validate_reference("drset", data.drset, counts.ndrain)?;
            }
            YearlyScenarioData::Forest(data) => {
                // Forest itype references a forest plant scenario; the
                // surface/contour/drain references are the `0` sentinel already
                // enforced at parse (`parse_yearly_forest`).
                validate_reference("itype", data.itype, counts.ncrop)?;
            }
        }
    }

    Ok(())
}

fn validate_reference(
    field: &'static str,
    value: usize,
    declared_count: usize,
) -> Result<(), ManagementParseError> {
    if declared_count == 0 {
        if value != 0 {
            return Err(ManagementParseError::DanglingScenarioReference {
                field,
                value,
                max_allowed: 0,
            });
        }
        return Ok(());
    }

    if !(1..=declared_count).contains(&value) {
        return Err(ManagementParseError::DanglingScenarioReference {
            field,
            value,
            max_allowed: declared_count,
        });
    }
    Ok(())
}

fn parse_scenario_meta(
    cursor: &mut Cursor<'_>,
    section: &'static str,
) -> Result<ScenarioMeta, ManagementParseError> {
    let name = cursor.next_required("scenario.name")?.to_string();
    let description = [
        cursor.next_required("scenario.desc_1")?.to_string(),
        cursor.next_required("scenario.desc_2")?.to_string(),
        cursor.next_required("scenario.desc_3")?.to_string(),
    ];
    let landuse = cursor.parse_non_negative_required("landuse")?;
    if landuse == 2 {
        return Err(ManagementParseError::UnsupportedLanduse { section, landuse });
    }
    Ok(ScenarioMeta {
        name,
        description,
        landuse,
    })
}

fn parse_f64_array<const N: usize>(
    cursor: &mut Cursor<'_>,
    field: &'static str,
) -> Result<[f64; N], ManagementParseError> {
    let tokens = cursor.parse_tokens_required(field)?;
    if tokens.len() != N {
        return Err(ManagementParseError::RecordArityError {
            field,
            observed: tokens.len(),
            expected: "fixed record arity",
        });
    }
    let mut values = [0.0_f64; N];
    for (idx, token) in tokens.iter().enumerate() {
        values[idx] = parse_f64_token(field, token)?;
    }
    Ok(values)
}

fn parse_optional_routing_coefficients(
    cursor: &mut Cursor<'_>,
    extension_allowed: bool,
) -> Result<Option<RoutingCoefficientExtension>, ManagementParseError> {
    let Some(marker) = cursor.peek_value() else {
        return Ok(None);
    };
    if marker != "routing_coefficients" && marker != "routing_coefficients_v1" {
        return Ok(None);
    }
    if !extension_allowed {
        return Err(ManagementParseError::InvalidOptionDomain {
            field: "plant.routing_coefficients",
            value: 1,
            allowed: "ow-lanuse-1 native forest (landuse=3) or native cropland (landuse=4)",
        });
    }

    let _ = cursor.next_required("plant.routing_coefficients.marker")?;
    let values = parse_f64_array::<5>(cursor, "plant.routing_coefficients")?;
    Ok(Some(RoutingCoefficientExtension {
        skin_friction_coefficient_ko: values[0],
        form_drag_coefficient: values[1],
        roughness_element_height_m: values[2],
        roughness_concentration: values[3],
        vegetation_drag_coefficient: values[4],
    }))
}

fn parse_julian_day(
    cursor: &mut Cursor<'_>,
    field: &'static str,
    allow_zero: bool,
) -> Result<usize, ManagementParseError> {
    let value = cursor.parse_i64_required(field)?;
    validate_julian_day(field, value, allow_zero)?;
    usize::try_from(value).map_err(|_| ManagementParseError::TokenParseError {
        field,
        value: value.to_string(),
    })
}

fn validate_julian_day(
    field: &'static str,
    value: i64,
    allow_zero: bool,
) -> Result<(), ManagementParseError> {
    if allow_zero && value == 0 {
        return Ok(());
    }
    if (1..=366).contains(&value) {
        return Ok(());
    }

    Err(ManagementParseError::DateDomainError {
        field,
        value,
        allowed: if allow_zero { "0 or 1..366" } else { "1..366" },
    })
}

fn parse_f64_token(field: &'static str, token: &str) -> Result<f64, ManagementParseError> {
    token
        .parse::<f64>()
        .map_err(|_| ManagementParseError::TokenParseError {
            field,
            value: token.to_string(),
        })
}

fn parse_i64_token(field: &'static str, token: &str) -> Result<i64, ManagementParseError> {
    token
        .parse::<i64>()
        .map_err(|_| ManagementParseError::TokenParseError {
            field,
            value: token.to_string(),
        })
}

fn parse_usize_token(field: &'static str, token: &str) -> Result<usize, ManagementParseError> {
    let value = parse_i64_token(field, token)?;
    if value < 0 {
        return Err(ManagementParseError::InvalidCount { field, value });
    }
    usize::try_from(value).map_err(|_| ManagementParseError::InvalidCount { field, value })
}

#[derive(Debug, Clone)]
struct NormalizedLine {
    line_no: usize,
    value: String,
}

fn normalize_lines(input: &str) -> Vec<NormalizedLine> {
    let raw_lines: Vec<&str> = input.lines().collect();
    let mut keep_blank_indices: HashSet<usize> = HashSet::new();

    for (idx, raw_line) in raw_lines.iter().enumerate() {
        if raw_line.contains("#landuse") || raw_line.contains(" # landuse") {
            for offset in 1..=3 {
                if idx >= offset {
                    keep_blank_indices.insert(idx - offset);
                }
            }
        }
    }

    let mut output = Vec::new();
    for (line_idx, raw_line) in raw_lines.iter().enumerate() {
        let without_comment = match raw_line.split_once('#') {
            Some((prefix, _)) => prefix,
            None => raw_line,
        };
        let trimmed = without_comment.trim();
        if !trimmed.is_empty() || keep_blank_indices.contains(&line_idx) {
            output.push(NormalizedLine {
                line_no: line_idx + 1,
                value: trimmed.to_string(),
            });
        }
    }
    output
}

struct Cursor<'a> {
    lines: &'a [NormalizedLine],
    mode: ParseMode,
    index: usize,
}

impl<'a> Cursor<'a> {
    fn new(lines: &'a [NormalizedLine], mode: ParseMode) -> Self {
        Self {
            lines,
            mode,
            index: 0,
        }
    }

    fn next_required(&mut self, field: &'static str) -> Result<&'a str, ManagementParseError> {
        let value = self
            .lines
            .get(self.index)
            .map(|line| line.value.as_str())
            .ok_or(ManagementParseError::MissingRecord { field })?;
        self.index += 1;
        Ok(value)
    }

    fn peek_value(&self) -> Option<&'a str> {
        self.lines.get(self.index).map(|line| line.value.as_str())
    }

    fn parse_token(&self, field: &'static str, raw: &str) -> Result<String, ManagementParseError> {
        let mut parts = raw.split_whitespace();
        let first = parts
            .next()
            .ok_or_else(|| ManagementParseError::TokenParseError {
                field,
                value: raw.to_string(),
            })?;
        if matches!(self.mode, ParseMode::Strict) && parts.next().is_some() {
            return Err(ManagementParseError::TokenParseError {
                field,
                value: raw.to_string(),
            });
        }
        Ok(first.to_string())
    }

    fn parse_tokens_required(
        &mut self,
        field: &'static str,
    ) -> Result<Vec<String>, ManagementParseError> {
        let raw = self.next_required(field)?;
        let tokens: Vec<String> = raw.split_whitespace().map(ToString::to_string).collect();
        if tokens.is_empty() {
            return Err(ManagementParseError::TokenParseError {
                field,
                value: raw.to_string(),
            });
        }
        Ok(tokens)
    }

    fn parse_i64_required(&mut self, field: &'static str) -> Result<i64, ManagementParseError> {
        let raw = self.next_required(field)?;
        let token = self.parse_token(field, raw)?;
        parse_i64_token(field, token.as_str())
    }

    fn parse_f64_required(&mut self, field: &'static str) -> Result<f64, ManagementParseError> {
        let raw = self.next_required(field)?;
        let token = self.parse_token(field, raw)?;
        parse_f64_token(field, token.as_str())
    }

    fn parse_non_negative_required(
        &mut self,
        field: &'static str,
    ) -> Result<usize, ManagementParseError> {
        let value = self.parse_i64_required(field)?;
        if value < 0 {
            return Err(ManagementParseError::InvalidCount { field, value });
        }
        usize::try_from(value).map_err(|_| ManagementParseError::InvalidCount { field, value })
    }

    fn first_unconsumed_line_number(&self) -> Option<usize> {
        self.lines.get(self.index).map(|line| line.line_no)
    }
}

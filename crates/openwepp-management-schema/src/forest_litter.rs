use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{ManagementYamlDocument, ManagementYamlError, PlantScenario};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForestLitterFunctionalClass {
    NeedleleafEvergreen,
    NeedleleafDeciduous,
    BroadleafEvergreen,
    BroadleafDeciduous,
    NonWoody,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestVegetationAuthority {
    pub source_identity: String,
    pub source_uri_or_path: String,
    pub access_or_version_date: String,
    pub claim_anchor: String,
    pub digest_algorithm: String,
    pub source_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestVegetationClassification {
    pub functional_classes: Vec<ForestLitterFunctionalClass>,
    pub authority: ForestVegetationAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForestLitterTissueStatus {
    Complete,
    NotRepresented,
    NotApplicable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForestLitterBoundaryMode {
    PrescribedScenario,
    MeasuredDaily,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForestLitterCalendar {
    ProlepticGregorian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForestLitterMassState {
    OvenDry,
    DryToConstantMass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForestLitterBarkTreatment {
    Included,
    Excluded,
    Separated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForestLitterResolution {
    ExactDaily,
    Interval,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterDryMassBasis {
    pub state: ForestLitterMassState,
    pub drying_temperature_c: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drying_duration_hours: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constant_mass_criterion: Option<String>,
    pub horizontal_area_basis: bool,
    pub units: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterSpatialSupport {
    pub site_or_plot: String,
    pub ofe_binding: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterAuthority {
    pub source_identity: String,
    pub source_uri_or_path: String,
    pub access_or_version_date: String,
    pub claim_anchor: String,
    pub digest_algorithm: String,
    pub source_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterOriginalObservation {
    pub support_start: String,
    pub support_end: String,
    pub resolution: ForestLitterResolution,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval_definition: Option<String>,
    pub units: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterExecutableForcing {
    pub path: String,
    pub digest_algorithm: String,
    pub executable_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterDerivation {
    pub identity: String,
    pub version: String,
    pub inputs: Vec<String>,
    pub transformation_authority: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ForestLitterDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ForestLitterDailyEntry {
    pub date: ForestLitterDate,
    pub deposited_kg_m2: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterTissuePayload {
    pub mode: ForestLitterBoundaryMode,
    pub support_start: String,
    pub support_end: String,
    pub calendar: ForestLitterCalendar,
    pub species_or_functional_type: ForestLitterFunctionalClass,
    pub included_material: String,
    pub excluded_material: String,
    pub mass_basis: ForestLitterDryMassBasis,
    pub spatial_support: ForestLitterSpatialSupport,
    pub authority: ForestLitterAuthority,
    pub original_observation: ForestLitterOriginalObservation,
    pub executable_forcing: ForestLitterExecutableForcing,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation: Option<ForestLitterDerivation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_diameter_mm: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bark_treatment: Option<ForestLitterBarkTreatment>,
    #[serde(skip)]
    pub entries: Vec<ForestLitterDailyEntry>,
}

impl ForestLitterTissuePayload {
    #[must_use]
    pub fn daily_mass_kg_m2(&self, date: ForestLitterDate) -> Option<f64> {
        self.entries
            .binary_search_by_key(&date, |entry| entry.date)
            .ok()
            .map(|index| self.entries[index].deposited_kg_m2)
    }

    pub fn support(&self) -> Result<(ForestLitterDate, ForestLitterDate), ManagementYamlError> {
        Ok((
            parse_date("support_start", &self.support_start)?,
            parse_date("support_end", &self.support_end)?,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestLitterTissue {
    pub status: ForestLitterTissueStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<ForestLitterTissuePayload>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ForestSurfaceLitterForcing {
    pub vegetation: ForestVegetationClassification,
    pub needle: ForestLitterTissue,
    pub fine_woody: ForestLitterTissue,
}

pub(super) fn validate_forcing_structure(
    forcing: &ForestSurfaceLitterForcing,
    plant_index: usize,
) -> Result<(), ManagementYamlError> {
    let root = format!("plants[{plant_index}].surface_litter_forcing");
    nonempty(
        &format!("{root}.vegetation.authority.source_identity"),
        &forcing.vegetation.authority.source_identity,
    )?;
    nonempty(
        &format!("{root}.vegetation.authority.source_uri_or_path"),
        &forcing.vegetation.authority.source_uri_or_path,
    )?;
    nonempty(
        &format!("{root}.vegetation.authority.access_or_version_date"),
        &forcing.vegetation.authority.access_or_version_date,
    )?;
    nonempty(
        &format!("{root}.vegetation.authority.claim_anchor"),
        &forcing.vegetation.authority.claim_anchor,
    )?;
    parse_date(
        &format!("{root}.vegetation.authority.access_or_version_date"),
        &forcing.vegetation.authority.access_or_version_date,
    )?;
    validate_sha256(
        &format!("{root}.vegetation.authority"),
        &forcing.vegetation.authority.digest_algorithm,
        &forcing.vegetation.authority.source_digest,
    )?;
    if forcing.vegetation.functional_classes.is_empty() {
        return invalid(
            &format!("{root}.vegetation.functional_classes"),
            "must not be empty",
        );
    }
    let unique = forcing
        .vegetation
        .functional_classes
        .iter()
        .copied()
        .collect::<HashSet<_>>();
    if unique.len() != forcing.vegetation.functional_classes.len() {
        return invalid(
            &format!("{root}.vegetation.functional_classes"),
            "must be unique",
        );
    }
    validate_tissue(
        &forcing.needle,
        &forcing.vegetation.functional_classes,
        false,
        &format!("{root}.needle"),
    )?;
    validate_tissue(
        &forcing.fine_woody,
        &forcing.vegetation.functional_classes,
        true,
        &format!("{root}.fine_woody"),
    )
}

pub(super) fn hydrate_document_forcings(
    document: &mut ManagementYamlDocument,
    management_path: &Path,
) -> Result<(), ManagementYamlError> {
    let base = management_path.parent().unwrap_or_else(|| Path::new("."));
    let site = document.metadata.name.clone();
    let nofes = document.topology.nofes;
    for (index, plant) in document.plants.iter_mut().enumerate() {
        let PlantScenario::NativeForest {
            surface_litter_forcing: Some(forcing),
            ..
        } = plant
        else {
            continue;
        };
        validate_forcing_structure(forcing, index)?;
        hydrate_vegetation_classification(
            &forcing.vegetation,
            base,
            &format!("plants[{index}].surface_litter_forcing.vegetation"),
        )?;
        hydrate_tissue(
            &mut forcing.needle,
            base,
            &site,
            nofes,
            &format!("plants[{index}].surface_litter_forcing.needle"),
        )?;
        hydrate_tissue(
            &mut forcing.fine_woody,
            base,
            &site,
            nofes,
            &format!("plants[{index}].surface_litter_forcing.fine_woody"),
        )?;
    }
    Ok(())
}

fn validate_tissue(
    tissue: &ForestLitterTissue,
    classes: &[ForestLitterFunctionalClass],
    fine_woody: bool,
    path: &str,
) -> Result<(), ManagementYamlError> {
    match tissue.status {
        ForestLitterTissueStatus::Complete => {
            let payload =
                tissue
                    .payload
                    .as_ref()
                    .ok_or_else(|| ManagementYamlError::MissingField {
                        path: format!("{path}.payload"),
                    })?;
            validate_payload(payload, classes, fine_woody, path)
        }
        ForestLitterTissueStatus::NotRepresented => {
            if tissue.payload.is_some() {
                invalid(path, "not_represented must not carry a numeric payload")
            } else {
                Ok(())
            }
        }
        ForestLitterTissueStatus::NotApplicable => {
            if tissue.payload.is_some() {
                return invalid(path, "not_applicable must not carry a numeric payload");
            }
            let applicable = if fine_woody {
                classes
                    .iter()
                    .any(|class| *class != ForestLitterFunctionalClass::NonWoody)
            } else {
                classes.iter().any(|class| {
                    matches!(
                        class,
                        ForestLitterFunctionalClass::NeedleleafEvergreen
                            | ForestLitterFunctionalClass::NeedleleafDeciduous
                    )
                })
            };
            if applicable {
                invalid(
                    path,
                    "not_applicable contradicts the authority-backed vegetation classes",
                )
            } else {
                Ok(())
            }
        }
    }
}

fn validate_payload(
    payload: &ForestLitterTissuePayload,
    classes: &[ForestLitterFunctionalClass],
    fine_woody: bool,
    path: &str,
) -> Result<(), ManagementYamlError> {
    if !classes.contains(&payload.species_or_functional_type) {
        return invalid(
            &format!("{path}.payload.species_or_functional_type"),
            "must match the authority-backed vegetation classification",
        );
    }
    let material_compatible = if fine_woody {
        payload.species_or_functional_type != ForestLitterFunctionalClass::NonWoody
    } else {
        matches!(
            payload.species_or_functional_type,
            ForestLitterFunctionalClass::NeedleleafEvergreen
                | ForestLitterFunctionalClass::NeedleleafDeciduous
        )
    };
    if !material_compatible {
        return invalid(
            &format!("{path}.payload.species_or_functional_type"),
            "is incompatible with the declared tissue material",
        );
    }
    for (field, value) in [
        ("included_material", payload.included_material.as_str()),
        ("excluded_material", payload.excluded_material.as_str()),
        ("mass_basis.units", payload.mass_basis.units.as_str()),
        (
            "spatial_support.site_or_plot",
            payload.spatial_support.site_or_plot.as_str(),
        ),
        (
            "authority.source_identity",
            payload.authority.source_identity.as_str(),
        ),
        (
            "authority.source_uri_or_path",
            payload.authority.source_uri_or_path.as_str(),
        ),
        (
            "authority.access_or_version_date",
            payload.authority.access_or_version_date.as_str(),
        ),
        (
            "authority.claim_anchor",
            payload.authority.claim_anchor.as_str(),
        ),
        (
            "original_observation.units",
            payload.original_observation.units.as_str(),
        ),
        (
            "executable_forcing.path",
            payload.executable_forcing.path.as_str(),
        ),
    ] {
        nonempty(&format!("{path}.payload.{field}"), value)?;
    }
    validate_mass_basis(payload, path)?;
    validate_support_and_provenance(payload, path)?;
    validate_material_definition(payload, fine_woody, path)
}

fn validate_mass_basis(
    payload: &ForestLitterTissuePayload,
    path: &str,
) -> Result<(), ManagementYamlError> {
    if payload.mass_basis.units != "kg_dry_mass_m2_day"
        || payload.original_observation.units != "kg_dry_mass_m2_day"
    {
        return invalid(
            path,
            "runtime and exact-daily original units must be kg_dry_mass_m2_day",
        );
    }
    if !payload.mass_basis.horizontal_area_basis {
        return invalid(
            &format!("{path}.payload.mass_basis.horizontal_area_basis"),
            "must be true",
        );
    }
    if !payload.mass_basis.drying_temperature_c.is_finite() {
        return invalid(
            &format!("{path}.payload.mass_basis.drying_temperature_c"),
            "must be finite",
        );
    }
    if let Some(hours) = payload.mass_basis.drying_duration_hours
        && (!hours.is_finite() || hours <= 0.0)
    {
        return invalid(
            &format!("{path}.payload.mass_basis.drying_duration_hours"),
            "must be positive and finite",
        );
    }
    if payload.mass_basis.state == ForestLitterMassState::DryToConstantMass
        && payload
            .mass_basis
            .constant_mass_criterion
            .as_deref()
            .is_none_or(str::is_empty)
    {
        return invalid(
            &format!("{path}.payload.mass_basis.constant_mass_criterion"),
            "is required for dry_to_constant_mass",
        );
    }
    if payload.mass_basis.drying_duration_hours.is_none()
        && payload
            .mass_basis
            .constant_mass_criterion
            .as_deref()
            .is_none_or(str::is_empty)
    {
        return invalid(
            &format!("{path}.payload.mass_basis"),
            "requires drying_duration_hours or constant_mass_criterion",
        );
    }
    Ok(())
}

fn validate_support_and_provenance(
    payload: &ForestLitterTissuePayload,
    path: &str,
) -> Result<(), ManagementYamlError> {
    let (start, end) = payload.support()?;
    if start > end {
        return invalid(path, "support_start must not follow support_end");
    }
    let original_start = parse_date(
        &format!("{path}.payload.original_observation.support_start"),
        &payload.original_observation.support_start,
    )?;
    let original_end = parse_date(
        &format!("{path}.payload.original_observation.support_end"),
        &payload.original_observation.support_end,
    )?;
    if original_start > original_end {
        return invalid(path, "original observation support is reversed");
    }
    parse_date(
        &format!("{path}.payload.authority.access_or_version_date"),
        &payload.authority.access_or_version_date,
    )?;
    if payload.original_observation.resolution != ForestLitterResolution::ExactDaily {
        return invalid(
            path,
            "this boundary implementation accepts exact_daily original objects only",
        );
    }
    if payload.derivation.is_some() {
        return invalid(
            path,
            "derived forcing is not admitted by this identity-only implementation",
        );
    }
    validate_sha256(
        &format!("{path}.payload.authority"),
        &payload.authority.digest_algorithm,
        &payload.authority.source_digest,
    )?;
    validate_sha256(
        &format!("{path}.payload.executable_forcing"),
        &payload.executable_forcing.digest_algorithm,
        &payload.executable_forcing.executable_digest,
    )?;
    if payload.authority.source_uri_or_path != payload.executable_forcing.path
        || payload.authority.source_digest != payload.executable_forcing.executable_digest
    {
        return invalid(
            path,
            "identity payload requires identical source/executable path and digest",
        );
    }
    if original_start != start || original_end != end {
        return invalid(
            path,
            "identity payload requires original and executable support to match exactly",
        );
    }
    Ok(())
}

fn hydrate_vegetation_classification(
    classification: &ForestVegetationClassification,
    base: &Path,
    path: &str,
) -> Result<(), ManagementYamlError> {
    let source_path = resolve_relative(
        base,
        &classification.authority.source_uri_or_path,
        &format!("{path}.authority.source_uri_or_path"),
    )?;
    let bytes = read_bytes(&source_path)?;
    verify_digest(
        &format!("{path}.authority.source_digest"),
        &bytes,
        &classification.authority.source_digest,
    )?;
    let observed = parse_functional_class_csv(path, &bytes)?;
    if observed == classification.functional_classes {
        Ok(())
    } else {
        invalid(
            &format!("{path}.functional_classes"),
            "must exactly match the authenticated classification object",
        )
    }
}

fn parse_functional_class_csv(
    path: &str,
    bytes: &[u8],
) -> Result<Vec<ForestLitterFunctionalClass>, ManagementYamlError> {
    let text = canonical_utf8_lines(path, bytes)?;
    let mut lines = text.split_terminator('\n');
    if lines.next() != Some("functional_class") {
        return invalid(path, "classification header must be functional_class");
    }
    let classes = lines
        .map(|value| match value {
            "needleleaf_evergreen" => Ok(ForestLitterFunctionalClass::NeedleleafEvergreen),
            "needleleaf_deciduous" => Ok(ForestLitterFunctionalClass::NeedleleafDeciduous),
            "broadleaf_evergreen" => Ok(ForestLitterFunctionalClass::BroadleafEvergreen),
            "broadleaf_deciduous" => Ok(ForestLitterFunctionalClass::BroadleafDeciduous),
            "non_woody" => Ok(ForestLitterFunctionalClass::NonWoody),
            _ => Err(invalid_error(
                path,
                "classification contains an unknown functional class",
            )),
        })
        .collect::<Result<Vec<_>, _>>()?;
    if classes.is_empty() {
        return invalid(path, "classification must contain at least one class");
    }
    Ok(classes)
}

fn validate_material_definition(
    payload: &ForestLitterTissuePayload,
    fine_woody: bool,
    path: &str,
) -> Result<(), ManagementYamlError> {
    if fine_woody {
        let diameter =
            payload
                .maximum_diameter_mm
                .ok_or_else(|| ManagementYamlError::MissingField {
                    path: format!("{path}.payload.maximum_diameter_mm"),
                })?;
        if !diameter.is_finite() || diameter <= 0.0 {
            return invalid(
                &format!("{path}.payload.maximum_diameter_mm"),
                "must be positive and finite",
            );
        }
        if payload.bark_treatment.is_none() {
            return Err(ManagementYamlError::MissingField {
                path: format!("{path}.payload.bark_treatment"),
            });
        }
    } else if payload.maximum_diameter_mm.is_some() || payload.bark_treatment.is_some() {
        return invalid(
            path,
            "needle payload must not carry fine-wood diameter/bark fields",
        );
    }
    Ok(())
}

fn hydrate_tissue(
    tissue: &mut ForestLitterTissue,
    base: &Path,
    site: &str,
    nofes: usize,
    path: &str,
) -> Result<(), ManagementYamlError> {
    let Some(payload) = tissue.payload.as_mut() else {
        return Ok(());
    };
    if payload.spatial_support.site_or_plot != site {
        return invalid(
            &format!("{path}.payload.spatial_support.site_or_plot"),
            "must equal management metadata.name",
        );
    }
    if payload.spatial_support.ofe_binding == 0 || payload.spatial_support.ofe_binding > nofes {
        return invalid(
            &format!("{path}.payload.spatial_support.ofe_binding"),
            "must identify a declared OFE",
        );
    }
    let source_path = resolve_relative(
        base,
        &payload.authority.source_uri_or_path,
        &format!("{path}.payload.authority.source_uri_or_path"),
    )?;
    let executable_path = resolve_relative(
        base,
        &payload.executable_forcing.path,
        &format!("{path}.payload.executable_forcing.path"),
    )?;
    let source_bytes = read_bytes(&source_path)?;
    verify_digest(
        &format!("{path}.payload.authority.source_digest"),
        &source_bytes,
        &payload.authority.source_digest,
    )?;
    let executable_bytes = if executable_path == source_path {
        source_bytes
    } else {
        read_bytes(&executable_path)?
    };
    verify_digest(
        &format!("{path}.payload.executable_forcing.executable_digest"),
        &executable_bytes,
        &payload.executable_forcing.executable_digest,
    )?;
    let (start, end) = payload.support()?;
    payload.entries = parse_csv(path, &executable_bytes, start, end, payload.mode)?;
    Ok(())
}

fn parse_csv(
    path: &str,
    bytes: &[u8],
    support_start: ForestLitterDate,
    support_end: ForestLitterDate,
    mode: ForestLitterBoundaryMode,
) -> Result<Vec<ForestLitterDailyEntry>, ManagementYamlError> {
    let text = canonical_utf8_lines(path, bytes)?;
    let mut lines = text.split_terminator('\n');
    if lines.next() != Some("date,deposited_kg_m2") {
        return invalid(path, "forcing CSV header must be date,deposited_kg_m2");
    }
    let mut entries = Vec::new();
    let mut previous = None;
    for (offset, line) in lines.enumerate() {
        if line.is_empty() {
            return invalid(path, "forcing CSV must not contain blank lines");
        }
        let (date_text, mass_text) = line.split_once(',').ok_or_else(|| {
            invalid_error(
                path,
                &format!(
                    "forcing CSV row {} must contain exactly two fields",
                    offset + 2
                ),
            )
        })?;
        if mass_text.contains(',') || !valid_decimal(mass_text) {
            return invalid(path, "deposited mass must be a nonnegative base-10 decimal");
        }
        let date = parse_date(path, date_text)?;
        if date < support_start || date > support_end {
            return invalid(path, "forcing CSV date lies outside declared support");
        }
        if previous.is_some_and(|value| date <= value) {
            return invalid(
                path,
                "forcing CSV dates must be unique and strictly increasing",
            );
        }
        let mass = mass_text
            .parse::<f64>()
            .map_err(|_| invalid_error(path, "deposited mass is not numeric"))?;
        if !mass.is_finite() || mass < 0.0 {
            return invalid(path, "deposited mass must be finite and nonnegative");
        }
        entries.push(ForestLitterDailyEntry {
            date,
            deposited_kg_m2: mass,
        });
        previous = Some(date);
    }
    if entries.is_empty() {
        return invalid(
            path,
            "complete forcing must contain at least one dated entry",
        );
    }
    if mode == ForestLitterBoundaryMode::MeasuredDaily {
        validate_exhaustive_daily(path, &entries, support_start, support_end)?;
    }
    Ok(entries)
}

fn canonical_utf8_lines<'a>(path: &str, bytes: &'a [u8]) -> Result<&'a str, ManagementYamlError> {
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return invalid(path, "authenticated CSV must be UTF-8 without BOM");
    }
    if bytes.contains(&b'\r') {
        return invalid(path, "authenticated CSV must use LF, not CRLF");
    }
    if !bytes.ends_with(b"\n") {
        return invalid(path, "authenticated CSV must end with one final LF");
    }
    std::str::from_utf8(bytes).map_err(|_| invalid_error(path, "authenticated CSV must be UTF-8"))
}

fn validate_exhaustive_daily(
    path: &str,
    entries: &[ForestLitterDailyEntry],
    support_start: ForestLitterDate,
    support_end: ForestLitterDate,
) -> Result<(), ManagementYamlError> {
    let mut expected = support_start;
    for entry in entries {
        if entry.date != expected {
            return invalid(
                path,
                "measured_daily forcing must explicitly record every supported date",
            );
        }
        if expected == support_end {
            return Ok(());
        }
        expected = next_date(expected).ok_or_else(|| {
            invalid_error(
                path,
                "measured_daily support cannot advance past the calendar",
            )
        })?;
    }
    invalid(
        path,
        "measured_daily forcing must explicitly record every supported date",
    )
}

fn next_date(date: ForestLitterDate) -> Option<ForestLitterDate> {
    let max_day = days_in_month(date.year, date.month)?;
    if date.day < max_day {
        return Some(ForestLitterDate {
            day: date.day + 1,
            ..date
        });
    }
    if date.month < 12 {
        return Some(ForestLitterDate {
            year: date.year,
            month: date.month + 1,
            day: 1,
        });
    }
    date.year.checked_add(1).map(|year| ForestLitterDate {
        year,
        month: 1,
        day: 1,
    })
}

fn parse_date(field: &str, value: &str) -> Result<ForestLitterDate, ManagementYamlError> {
    let bytes = value.as_bytes();
    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return invalid(field, "date must use YYYY-MM-DD");
    }
    let year = decimal_i32(&value[0..4]).ok_or_else(|| invalid_error(field, "invalid year"))?;
    let month = decimal_u8(&value[5..7]).ok_or_else(|| invalid_error(field, "invalid month"))?;
    let day = decimal_u8(&value[8..10]).ok_or_else(|| invalid_error(field, "invalid day"))?;
    let max_day =
        days_in_month(year, month).ok_or_else(|| invalid_error(field, "invalid month"))?;
    if day == 0 || day > max_day {
        return invalid(field, "invalid Gregorian day");
    }
    Ok(ForestLitterDate { year, month, day })
}

fn days_in_month(year: i32, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => Some(29),
        2 => Some(28),
        _ => None,
    }
}

fn decimal_i32(value: &str) -> Option<i32> {
    value
        .bytes()
        .all(|byte| byte.is_ascii_digit())
        .then(|| value.parse::<i32>().ok())
        .flatten()
}

fn decimal_u8(value: &str) -> Option<u8> {
    value
        .bytes()
        .all(|byte| byte.is_ascii_digit())
        .then(|| value.parse::<u8>().ok())
        .flatten()
}

fn valid_decimal(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    let mut dot = false;
    let mut digit = false;
    for byte in value.bytes() {
        if byte.is_ascii_digit() {
            digit = true;
        } else if byte == b'.' && !dot {
            dot = true;
        } else {
            return false;
        }
    }
    digit
}

fn validate_sha256(path: &str, algorithm: &str, digest: &str) -> Result<(), ManagementYamlError> {
    if algorithm != "sha256" {
        return invalid(path, "digest_algorithm must be sha256");
    }
    if digest.len() != 64
        || !digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return invalid(path, "digest must be 64 lowercase hexadecimal characters");
    }
    Ok(())
}

fn resolve_relative(
    base: &Path,
    value: &str,
    field: &str,
) -> Result<std::path::PathBuf, ManagementYamlError> {
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return invalid(field, "must be a confined relative file path");
    }
    Ok(base.join(path))
}

fn read_bytes(path: &Path) -> Result<Vec<u8>, ManagementYamlError> {
    fs::read(path).map_err(|source| ManagementYamlError::InputOpen {
        path: path.to_path_buf(),
        source,
    })
}

fn verify_digest(path: &str, bytes: &[u8], expected: &str) -> Result<(), ManagementYamlError> {
    let observed = format!("{:x}", Sha256::digest(bytes));
    if observed == expected {
        Ok(())
    } else {
        invalid(
            path,
            &format!("SHA-256 digest mismatch: expected {expected}, observed {observed}"),
        )
    }
}

fn nonempty(path: &str, value: &str) -> Result<(), ManagementYamlError> {
    if value.trim().is_empty() {
        invalid(path, "must not be empty")
    } else {
        Ok(())
    }
}

fn invalid<T>(path: &str, detail: &str) -> Result<T, ManagementYamlError> {
    Err(invalid_error(path, detail))
}

fn invalid_error(path: &str, detail: &str) -> ManagementYamlError {
    ManagementYamlError::InvalidField {
        path: path.to_string(),
        detail: detail.to_string(),
    }
}

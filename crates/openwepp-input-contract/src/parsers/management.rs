#![allow(clippy::missing_errors_doc, clippy::too_many_lines)]

use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ALLOWED_DATVERS: &[&str] = &["95.7", "98.4", "2016.3", "2017.1"];

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
            Self::InvalidOptionDomain { .. } | Self::UnsupportedLanduse { .. } => "MAN-E-004",
            Self::InvalidCount { .. } => "MAN-E-005",
            Self::TrailingInput { .. } => "MAN-E-006",
            Self::TotalYearMismatch { .. } => "MAN-E-008",
            Self::DanglingScenarioReference { .. } => "MAN-E-009",
            Self::DateDomainError { .. } => "MAN-E-010",
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

pub fn parse_management_from_str(
    input: &str,
    mode: ParseMode,
) -> Result<ManagementParseOutput, ManagementParseError> {
    let lines = normalize_lines(input);
    let mut cursor = Cursor::new(lines.as_slice(), mode);

    let datver_raw = cursor.next_required("datver")?;
    let datver = cursor.parse_token("datver", datver_raw)?;
    if !ALLOWED_DATVERS.contains(&datver.as_str()) {
        return Err(ManagementParseError::UnsupportedDatver { datver });
    }

    let topology_count = cursor.parse_non_negative_required("nofe_or_nchan")?;
    if topology_count == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "nofe_or_nchan",
            value: 0,
        });
    }

    let declared_total_years = cursor.parse_non_negative_required("total_years")?;
    if declared_total_years == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "total_years",
            value: 0,
        });
    }

    let datver_family = datver_family(&datver);

    let ncrop = cursor.parse_non_negative_required("ncrop")?;
    let plants = parse_plant_section(&mut cursor, ncrop)?;

    let nop = cursor.parse_non_negative_required("nop")?;
    let operations = parse_operation_section(&mut cursor, nop, datver_family)?;

    let nini = cursor.parse_non_negative_required("nini")?;
    let initials = parse_initial_section(&mut cursor, nini)?;

    let nseq = cursor.parse_non_negative_required("nseq")?;
    let surfaces = parse_surface_section(&mut cursor, nseq)?;

    let ncnt = cursor.parse_non_negative_required("ncnt")?;
    let contours = parse_contour_section(&mut cursor, ncnt, datver_family)?;

    let ndrain = cursor.parse_non_negative_required("ndrain")?;
    let drains = parse_drain_section(&mut cursor, ndrain)?;

    let nscen = cursor.parse_non_negative_required("nscen")?;
    let yearlies = parse_yearly_section(&mut cursor, nscen, datver_family)?;

    let section_counts = ManagementSectionCounts {
        ncrop,
        nop,
        nini,
        nseq,
        ncnt,
        ndrain,
        nscen,
    };

    let management_name = cursor.next_required("man_name")?.to_string();
    let management_description = [
        cursor.next_required("man_desc_1")?.to_string(),
        cursor.next_required("man_desc_2")?.to_string(),
        cursor.next_required("man_desc_3")?.to_string(),
    ];

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

    let rotation_repeats = cursor.parse_non_negative_required("nrots")?;
    if rotation_repeats == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "nrots",
            value: 0,
        });
    }

    let rotation_years = cursor.parse_non_negative_required("nyears")?;
    if rotation_years == 0 {
        return Err(ManagementParseError::InvalidCount {
            field: "nyears",
            value: 0,
        });
    }

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

    validate_cross_section_references(
        &section_counts,
        &plants,
        &operations,
        &initials,
        &surfaces,
        &yearlies,
        mode,
    )?;

    if let Some(first_unconsumed_line) = cursor.first_unconsumed_line_number() {
        return Err(ManagementParseError::TrailingInput {
            first_unconsumed_line,
        });
    }

    Ok(ManagementParseOutput {
        datver,
        topology_count,
        declared_total_years,
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
                name: management_name,
                description: management_description,
                nofes,
            },
        },
        schedule: ManagementSchedule {
            ofe_initial_refs,
            rotation_repeats,
            rotation_years,
            slots,
        },
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DatverFamily {
    V95_7,
    V98_4,
    V2016Plus,
}

fn datver_family(datver: &str) -> DatverFamily {
    match datver {
        "95.7" => DatverFamily::V95_7,
        "98.4" => DatverFamily::V98_4,
        _ => DatverFamily::V2016Plus,
    }
}

fn parse_plant_section(
    cursor: &mut Cursor<'_>,
    count: usize,
) -> Result<Vec<PlantScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "plant")?;
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "plant",
                landuse: meta.landuse,
            });
        }
        if meta.landuse != 1 {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "iplant",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: "1",
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
            }),
        });
    }
    Ok(scenarios)
}

fn parse_operation_section(
    cursor: &mut Cursor<'_>,
    count: usize,
    datver_family: DatverFamily,
) -> Result<Vec<OperationScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "operation")?;
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "operation",
                landuse: meta.landuse,
            });
        }
        if meta.landuse != 1 {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "iop",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: "1",
            });
        }

        let mfo_tokens = cursor.parse_tokens_required("op.mfo_line")?;
        if mfo_tokens.len() != 3 {
            return Err(ManagementParseError::RecordArityError {
                field: "op.mfo_line",
                observed: mfo_tokens.len(),
                expected: "3",
            });
        }
        let mfo1 = parse_f64_token("op.mfo1", &mfo_tokens[0])?;
        let mfo2 = parse_f64_token("op.mfo2", &mfo_tokens[1])?;
        let numof = parse_usize_token("op.numof", &mfo_tokens[2])?;

        let pcode_tokens = cursor.parse_tokens_required("op.pcode")?;
        if pcode_tokens.is_empty() || pcode_tokens.len() > 2 {
            return Err(ManagementParseError::RecordArityError {
                field: "op.pcode",
                observed: pcode_tokens.len(),
                expected: "1 or 2",
            });
        }
        let pcode = parse_usize_token("op.pcode", &pcode_tokens[0])?;
        let valid_pcodes = match datver_family {
            DatverFamily::V95_7 => &[1, 2, 3, 4][..],
            DatverFamily::V98_4 => &[1, 2, 3, 4, 10, 11, 12, 13][..],
            DatverFamily::V2016Plus => &[1, 2, 3, 4, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19][..],
        };
        if !valid_pcodes.contains(&pcode) {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "pcode",
                value: i64::try_from(pcode).unwrap_or(i64::MAX),
                allowed: "datver-specific operation-code allowlist",
            });
        }
        let cltpos = if pcode == 3 {
            if pcode_tokens.len() != 2 {
                return Err(ManagementParseError::RecordArityError {
                    field: "op.cltpos",
                    observed: pcode_tokens.len(),
                    expected: "2",
                });
            }
            let cltpos = parse_usize_token("op.cltpos", &pcode_tokens[1])?;
            if cltpos != 1 && cltpos != 2 {
                return Err(ManagementParseError::InvalidOptionDomain {
                    field: "cltpos",
                    value: i64::try_from(cltpos).unwrap_or(i64::MAX),
                    allowed: "1 or 2",
                });
            }
            Some(cltpos)
        } else {
            if pcode_tokens.len() != 1 {
                return Err(ManagementParseError::RecordArityError {
                    field: "op.pcode",
                    observed: pcode_tokens.len(),
                    expected: "1",
                });
            }
            None
        };

        let effect_tokens = cursor.parse_tokens_required("op.effect_line")?;
        if effect_tokens.len() != 7 && effect_tokens.len() != 9 {
            return Err(ManagementParseError::RecordArityError {
                field: "op.effect_line",
                observed: effect_tokens.len(),
                expected: "7 or 9",
            });
        }
        let mut effect_line = Vec::with_capacity(effect_tokens.len());
        for token in &effect_tokens {
            effect_line.push(parse_f64_token("op.effect_line", token)?);
        }

        let mut extension_lines = Vec::new();
        match pcode {
            10 | 12 | 15 | 18 | 19 => {
                extension_lines.push(cursor.next_required("op.ext_line_1")?.to_string());
                let token_count = extension_lines[0].split_whitespace().count();
                if token_count < 2 {
                    extension_lines.push(cursor.next_required("op.ext_line_2")?.to_string());
                }
            }
            11 | 13 | 14 => {
                extension_lines.push(cursor.next_required("op.ext_line_1")?.to_string());
            }
            _ => {}
        }

        scenarios.push(OperationScenario {
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
        });
    }
    Ok(scenarios)
}

fn parse_initial_section(
    cursor: &mut Cursor<'_>,
    count: usize,
) -> Result<Vec<InitialScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "initial")?;
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "initial",
                landuse: meta.landuse,
            });
        }
        if meta.landuse != 1 {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "lanuse",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: "1",
            });
        }

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

        let terminal_tokens = cursor.parse_tokens_required("ini.terminal_line")?;
        if terminal_tokens.len() != 2 && terminal_tokens.len() != 4 {
            return Err(ManagementParseError::RecordArityError {
                field: "ini.terminal_line",
                observed: terminal_tokens.len(),
                expected: "2 or 4",
            });
        }
        let terminal_line = [
            parse_f64_token("ini.sumrtm", &terminal_tokens[0])?,
            parse_f64_token("ini.sumsrm", &terminal_tokens[1])?,
        ];
        let understory_line = if terminal_tokens.len() == 4 {
            Some([
                parse_f64_token("ini.usinrcol", &terminal_tokens[2])?,
                parse_f64_token("ini.usrilcol", &terminal_tokens[3])?,
            ])
        } else {
            None
        };

        scenarios.push(InitialScenario {
            meta,
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
        });
    }
    Ok(scenarios)
}

fn parse_surface_section(
    cursor: &mut Cursor<'_>,
    count: usize,
) -> Result<Vec<SurfaceScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "surface")?;
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "surface",
                landuse: meta.landuse,
            });
        }
        if meta.landuse != 1 {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "iseq",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: "1",
            });
        }

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
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "contour")?;
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "contour",
                landuse: meta.landuse,
            });
        }
        if meta.landuse != 1 {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "icont",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: "1",
            });
        }

        let tokens = cursor.parse_tokens_required("contour.values")?;
        if tokens.len() != 4 && tokens.len() != 5 {
            return Err(ManagementParseError::RecordArityError {
                field: "contour.values",
                observed: tokens.len(),
                expected: "4 or 5",
            });
        }
        if tokens.len() == 5 && datver_family != DatverFamily::V2016Plus {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "contours_perm",
                value: 1,
                allowed: "2016.3+ datver only",
            });
        }

        let cntslp = parse_f64_token("cntslp", &tokens[0])?;
        let rdghgt = parse_f64_token("rdghgt", &tokens[1])?;
        let rowlen = parse_f64_token("rowlen", &tokens[2])?;
        let rowspc = parse_f64_token("rowspc", &tokens[3])?;
        let contours_perm = if tokens.len() == 5 {
            Some(parse_usize_token("contours_perm", &tokens[4])?)
        } else {
            None
        };

        scenarios.push(ContourScenario {
            meta,
            cntslp,
            rdghgt,
            rowlen,
            rowspc,
            contours_perm,
        });
    }
    Ok(scenarios)
}

fn parse_drain_section(
    cursor: &mut Cursor<'_>,
    count: usize,
) -> Result<Vec<DrainScenario>, ManagementParseError> {
    let mut scenarios = Vec::with_capacity(count);
    for _ in 0..count {
        let meta = parse_scenario_meta(cursor, "drain")?;
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "drain",
                landuse: meta.landuse,
            });
        }
        if meta.landuse != 1 {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "dcont",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: "1",
            });
        }

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
        if meta.landuse == 2 {
            return Err(ManagementParseError::UnsupportedLanduse {
                section: "yearly",
                landuse: meta.landuse,
            });
        }
        if meta.landuse != 1 {
            return Err(ManagementParseError::InvalidOptionDomain {
                field: "iscen",
                value: i64::try_from(meta.landuse).unwrap_or(i64::MAX),
                allowed: "1",
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

fn parse_yearly_annual_fallow(
    cursor: &mut Cursor<'_>,
    datver_family: DatverFamily,
) -> Result<YearlyAnnualFallowData, ManagementParseError> {
    let jdharv = parse_julian_day(cursor, "jdharv", false)?;
    let jdplt = parse_julian_day(cursor, "jdplt", false)?;
    let rw = cursor.parse_f64_required("rw")?;
    let resmgt = cursor.parse_non_negative_required("resmgt")?;

    let valid_resmgt = match datver_family {
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
            let _cut_flag = cursor.parse_non_negative_required("annual_cut.flag")?;
            let ncut = cursor.parse_non_negative_required("annual_cut.ncut")?;
            if ncut == 0 {
                return Err(ManagementParseError::InvalidCount {
                    field: "annual_cut.ncut",
                    value: 0,
                });
            }
            for _ in 0..ncut {
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
            }
            None
        }
        _ => unreachable!(),
    };

    Ok(YearlyAnnualFallowData {
        jdharv,
        jdplt,
        rw,
        resmgt,
        extension,
    })
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
        let InitialScenarioData::Cropland(data) = &initial.data;
        validate_reference("iresd", data.iresd, counts.ncrop)?;
    }

    for surface in surfaces {
        for op in &surface.operations {
            validate_reference("op", op.op_ref, counts.nop)?;
        }
    }

    for yearly in yearlies {
        let YearlyScenarioData::Cropland(data) = &yearly.data;
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

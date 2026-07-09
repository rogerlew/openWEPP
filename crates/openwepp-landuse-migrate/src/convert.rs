use openwepp_input_contract::parsers::management as flat;
use openwepp_management_schema as yaml;
use sha2::{Digest, Sha256};

use crate::disturbed::{
    DISTURBED_ROUTE_TABLE_ID, DISTURBED_ROUTE_TABLE_SOURCE_AUTHORITY,
    DISTURBED_ROUTE_TABLE_VERSION, disturbed_route_table_checksum, row_for_disturbed_class,
};
use crate::{
    LanduseMigrationError, MigrationAuthority, MigrationTarget, ResolvedCoefficientSite,
    SourceManagement, legacy_class_map,
};

pub(crate) fn source_to_yaml_document(
    source: &SourceManagement,
    target: MigrationTarget,
    authority: &MigrationAuthority,
) -> Result<
    (
        yaml::ManagementYamlDocument,
        Vec<ResolvedCoefficientSite>,
        Vec<String>,
    ),
    LanduseMigrationError,
> {
    match source {
        SourceManagement::Yaml { document, .. } => {
            yaml::validate_management_yaml_document(document)?;
            Ok((
                document.clone(),
                Vec::new(),
                vec![format!(
                    "validated native YAML target {}",
                    target.resolved_datver()
                )],
            ))
        }
        SourceManagement::Flat { path, parsed } => {
            if crate::is_legacy_source_datver(&parsed.datver) {
                legacy_flat_to_yaml(parsed, &path.display().to_string(), target, authority)
            } else if parsed.datver == yaml::OW_LANUSE_1_DATVER {
                native_flat_to_yaml(parsed, &path.display().to_string(), target)
            } else {
                Err(LanduseMigrationError::UnsupportedSourceDatver {
                    datver: parsed.datver.clone(),
                })
            }
        }
    }
}

fn legacy_flat_to_yaml(
    parsed: &flat::ManagementParseOutput,
    source_path: &str,
    target: MigrationTarget,
    authority: &MigrationAuthority,
) -> Result<
    (
        yaml::ManagementYamlDocument,
        Vec<ResolvedCoefficientSite>,
        Vec<String>,
    ),
    LanduseMigrationError,
> {
    let class_by_plant = legacy_class_map(parsed, authority)?;
    validate_legacy_source_landuses(parsed)?;
    let mut resolved = Vec::new();
    let document = flat_to_yaml_document(
        parsed,
        source_path,
        target,
        |plant_index, plant_name, routing| {
            if routing.is_some() {
                return Err(LanduseMigrationError::UnsupportedSourceLanduse {
                    detail: "legacy source unexpectedly carried native routing coefficients"
                        .to_string(),
                });
            }
            let Some(disturbed_class) = class_by_plant.get(&plant_index).copied() else {
                return Err(LanduseMigrationError::MissingMigrationAuthority {
                    site: format!("plant_index={plant_index} plant_scenario_name={plant_name}"),
                });
            };
            let row = row_for_disturbed_class(disturbed_class)?;
            resolved.push(ResolvedCoefficientSite {
                plant_index,
                plant_scenario_name: plant_name.to_string(),
                disturbed_class: row.disturbed_class.to_string(),
                k_o: row.k_o,
                form_c_d: row.form_c_d,
                d_r_m: row.d_r_m,
                lambda: row.lambda,
                vegetation_c_d: row.vegetation_c_d,
            });
            Ok(yaml::RouteCoefficients {
                k_o: row.k_o,
                form_c_d: row.form_c_d,
                d_r_m: row.d_r_m,
                lambda: row.lambda,
                vegetation_c_d: row.vegetation_c_d,
                authority: yaml::RouteCoefficientAuthority {
                    source: DISTURBED_ROUTE_TABLE_ID.to_string(),
                    version: DISTURBED_ROUTE_TABLE_VERSION.to_string(),
                    checksum: disturbed_route_table_checksum(),
                    disturbed_class: row.disturbed_class.to_string(),
                    source_authority: Some(DISTURBED_ROUTE_TABLE_SOURCE_AUTHORITY.to_string()),
                },
            })
        },
    )?;
    yaml::validate_management_yaml_document(&document)?;
    Ok((
        document,
        resolved,
        vec![
            "parsed frozen legacy flat management source".to_string(),
            "resolved required disturbed-class authority".to_string(),
            "emitted coefficient-complete ow-lanuse-1 YAML".to_string(),
        ],
    ))
}

fn native_flat_to_yaml(
    parsed: &flat::ManagementParseOutput,
    source_path: &str,
    target: MigrationTarget,
) -> Result<
    (
        yaml::ManagementYamlDocument,
        Vec<ResolvedCoefficientSite>,
        Vec<String>,
    ),
    LanduseMigrationError,
> {
    let mut resolved = Vec::new();
    let document = flat_to_yaml_document(
        parsed,
        source_path,
        target,
        |plant_index, plant_name, routing| {
            let Some(routing) = routing else {
                return Err(LanduseMigrationError::NativeMissingRoutingCoefficients {
                    plant_index,
                    plant_name: plant_name.to_string(),
                });
            };
            let authority_class = "native-flat-explicit-routing";
            resolved.push(ResolvedCoefficientSite {
                plant_index,
                plant_scenario_name: plant_name.to_string(),
                disturbed_class: authority_class.to_string(),
                k_o: routing.skin_friction_coefficient_ko,
                form_c_d: routing.form_drag_coefficient,
                d_r_m: routing.roughness_element_height_m,
                lambda: routing.roughness_concentration,
                vegetation_c_d: routing.vegetation_drag_coefficient,
            });
            Ok(yaml::RouteCoefficients {
                k_o: routing.skin_friction_coefficient_ko,
                form_c_d: routing.form_drag_coefficient,
                d_r_m: routing.roughness_element_height_m,
                lambda: routing.roughness_concentration,
                vegetation_c_d: routing.vegetation_drag_coefficient,
                authority: yaml::RouteCoefficientAuthority {
                    source: "flat-ow-lanuse-1-routing_coefficients".to_string(),
                    version: yaml::OW_LANUSE_1_DATVER.to_string(),
                    checksum: checksum_for_native_flat_routing(routing),
                    disturbed_class: authority_class.to_string(),
                    source_authority: Some(
                        "explicit routing_coefficients block embedded in flat ow-lanuse-1 source"
                            .to_string(),
                    ),
                },
            })
        },
    )?;
    yaml::validate_management_yaml_document(&document)?;
    Ok((
        document,
        resolved,
        vec![
            "parsed flat ow-lanuse-1 source bridge".to_string(),
            "preserved explicit native routing coefficients".to_string(),
            format!("resolved target alias to {}", target.resolved_datver()),
        ],
    ))
}

fn flat_to_yaml_document<F>(
    parsed: &flat::ManagementParseOutput,
    source_path: &str,
    target: MigrationTarget,
    mut route_for_plant: F,
) -> Result<yaml::ManagementYamlDocument, LanduseMigrationError>
where
    F: FnMut(
        usize,
        &str,
        Option<flat::RoutingCoefficientExtension>,
    ) -> Result<yaml::RouteCoefficients, LanduseMigrationError>,
{
    Ok(yaml::ManagementYamlDocument {
        format: yaml::MANAGEMENT_YAML_FORMAT.to_string(),
        schema_version: yaml::MANAGEMENT_YAML_SCHEMA_VERSION,
        datver: target.resolved_datver().to_string(),
        topology: yaml::Topology {
            nofes: parsed.topology_count,
            total_years: parsed.declared_total_years,
        },
        metadata: yaml::ManagementMetadata {
            name: parsed.registries.management_meta.name.clone(),
            description: description_vec(&parsed.registries.management_meta.description),
            provenance: Some(yaml::ManagementProvenance {
                source_path: Some(source_path.to_string()),
                source_datver: Some(parsed.datver.clone()),
                migrator: Some(crate::MIGRATOR_NAME.to_string()),
            }),
        },
        plants: parsed
            .registries
            .plants
            .iter()
            .enumerate()
            .map(|(index, plant)| plant_to_yaml(index + 1, plant, &mut route_for_plant))
            .collect::<Result<Vec<_>, _>>()?,
        operations: parsed
            .registries
            .operations
            .iter()
            .map(operation_to_yaml)
            .collect(),
        initial_conditions: parsed
            .registries
            .initials
            .iter()
            .map(initial_to_yaml)
            .collect(),
        surface_effects: parsed
            .registries
            .surfaces
            .iter()
            .map(surface_to_yaml)
            .collect(),
        contours: parsed
            .registries
            .contours
            .iter()
            .map(contour_to_yaml)
            .collect(),
        drains: parsed.registries.drains.iter().map(drain_to_yaml).collect(),
        yearly_scenarios: parsed
            .registries
            .yearlies
            .iter()
            .map(yearly_to_yaml)
            .collect(),
        schedule: schedule_to_yaml(&parsed.schedule),
    })
}

fn plant_to_yaml<F>(
    plant_index: usize,
    plant: &flat::PlantScenario,
    route_for_plant: &mut F,
) -> Result<yaml::PlantScenario, LanduseMigrationError>
where
    F: FnMut(
        usize,
        &str,
        Option<flat::RoutingCoefficientExtension>,
    ) -> Result<yaml::RouteCoefficients, LanduseMigrationError>,
{
    match &plant.data {
        flat::PlantScenarioData::Cropland(data) => {
            if plant.meta.landuse != 1
                && plant.meta.landuse != flat::NATIVE_CROPLAND_LANUSE_SENTINEL
            {
                return Err(LanduseMigrationError::UnsupportedSourceLanduse {
                    detail: format!(
                        "plant_index {plant_index} ({}) has unsupported cropland landuse {}",
                        plant.meta.name, plant.meta.landuse
                    ),
                });
            }
            Ok(yaml::PlantScenario::NativeCropland {
                name: plant.meta.name.clone(),
                description: description_vec(&plant.meta.description),
                crunit: data.crunit.clone(),
                canopy_line: data.canopy_line,
                growth_line: data.growth_line,
                mfocod: data.mfocod,
                residue_line: data.residue_line,
                terminal_line: data.terminal_line,
                rcc: data.rcc,
                routing_coefficients: Some(route_for_plant(
                    plant_index,
                    &plant.meta.name,
                    data.routing,
                )?),
            })
        }
        flat::PlantScenarioData::Forest(data) => Ok(yaml::PlantScenario::NativeForest {
            name: plant.meta.name.clone(),
            description: description_vec(&plant.meta.description),
            forest_class: data.forest_class.clone(),
            growth: yaml::PlantForestGrowth {
                bb: data.growth.bb,
                bbb: data.growth.bbb,
                beinp: data.growth.beinp,
                btemp: data.growth.btemp,
                otemp: data.growth.otemp,
                gddmax: data.growth.gddmax,
                dlai: data.growth.dlai,
                dropfc: data.growth.dropfc,
                decfct: data.growth.decfct,
                spriod: data.growth.spriod,
                extnct: data.growth.extnct,
                flivmx: data.growth.flivmx,
                hmax: data.growth.hmax,
                hi: data.growth.hi,
                pltol: data.growth.pltol,
                xmxlai: data.growth.xmxlai,
                rsr: data.growth.rsr,
                rtmmax: data.growth.rtmmax,
                rdmax: data.growth.rdmax,
            },
            cf: data.cf,
            diam: data.diam,
            decomposition: yaml::PlantForestDecomposition {
                oratea: data.decomposition.oratea,
                orater: data.decomposition.orater,
            },
            community: yaml::PlantForestCommunity {
                tempmn: data.community.tempmn,
                gtemp: data.community.gtemp,
                plive: data.community.plive,
                wood: data.community.wood,
                grass: stratum_to_yaml(data.community.grass),
                shrub: stratum_to_yaml(data.community.shrub),
                tree: stratum_to_yaml(data.community.tree),
            },
            routing_coefficients: Some(route_for_plant(
                plant_index,
                &plant.meta.name,
                data.routing,
            )?),
        }),
    }
}

fn operation_to_yaml(operation: &flat::OperationScenario) -> yaml::OperationScenario {
    match &operation.data {
        flat::OperationScenarioData::Cropland(data) => yaml::OperationScenario::NativeCropland {
            name: operation.meta.name.clone(),
            description: description_vec(&operation.meta.description),
            mfo1: data.mfo1,
            mfo2: data.mfo2,
            numof: data.numof,
            pcode: data.pcode,
            cltpos: data.cltpos,
            effect_line: data.effect_line.clone(),
            extension_lines: data.extension_lines.clone(),
        },
    }
}

fn initial_to_yaml(initial: &flat::InitialScenario) -> yaml::InitialConditionScenario {
    match &initial.data {
        flat::InitialScenarioData::Cropland(data) => {
            yaml::InitialConditionScenario::NativeCropland {
                name: initial.meta.name.clone(),
                description: description_vec(&initial.meta.description),
                base_line: data.base_line,
                iresd: data.iresd,
                imngmt: data.imngmt,
                residue_line: data.residue_line,
                rtyp: data.rtyp,
                thaw_line: data.thaw_line,
                terminal_line: data.terminal_line,
                understory_line: data.understory_line,
            }
        }
        flat::InitialScenarioData::Forest(data) => yaml::InitialConditionScenario::NativeForest {
            name: initial.meta.name.clone(),
            description: description_vec(&initial.meta.description),
            cancov: data.cancov,
            inrcov: data.inrcov,
            rilcov: data.rilcov,
            rrinit: data.rrinit,
            iresd: data.iresd,
            imngmt: data.imngmt,
            sumrtm: data.sumrtm,
            sumsrm: data.sumsrm,
            tillay1: data.tillay1,
            tillay2: data.tillay2,
            understory_line: data.understory_line,
        },
    }
}

fn surface_to_yaml(surface: &flat::SurfaceScenario) -> yaml::SurfaceEffectScenario {
    yaml::SurfaceEffectScenario::NativeCropland {
        name: surface.meta.name.clone(),
        description: description_vec(&surface.meta.description),
        ntill: surface.ntill,
        operations: surface
            .operations
            .iter()
            .map(|operation| yaml::SurfaceOperation {
                mdate: operation.mdate,
                op_ref: operation.op_ref,
                tildep: operation.tildep,
                typtil: operation.typtil,
            })
            .collect(),
    }
}

fn contour_to_yaml(contour: &flat::ContourScenario) -> yaml::ContourScenario {
    yaml::ContourScenario {
        name: contour.meta.name.clone(),
        description: description_vec(&contour.meta.description),
        cntslp: contour.cntslp,
        rdghgt: contour.rdghgt,
        rowlen: contour.rowlen,
        rowspc: contour.rowspc,
        contours_perm: contour.contours_perm,
    }
}

fn drain_to_yaml(drain: &flat::DrainScenario) -> yaml::DrainScenario {
    yaml::DrainScenario {
        name: drain.meta.name.clone(),
        description: description_vec(&drain.meta.description),
        ddrain: drain.ddrain,
        drainc: drain.drainc,
        drdiam: drain.drdiam,
        sdrain: drain.sdrain,
    }
}

fn yearly_to_yaml(yearly: &flat::YearlyScenario) -> yaml::YearlyScenario {
    match &yearly.data {
        flat::YearlyScenarioData::Cropland(data) => yaml::YearlyScenario::NativeCropland {
            name: yearly.meta.name.clone(),
            description: description_vec(&yearly.meta.description),
            itype: data.itype,
            tilseq: data.tilseq,
            conset: data.conset,
            drset: data.drset,
            imngmt: data.imngmt,
            branch: yearly_branch_to_yaml(&data.branch),
        },
        flat::YearlyScenarioData::Forest(data) => yaml::YearlyScenario::NativeForest {
            name: yearly.meta.name.clone(),
            description: description_vec(&yearly.meta.description),
            itype: data.itype,
            jdharv: data.jdharv,
            jdplt: data.jdplt,
            jdstop: data.jdstop,
            rw: data.rw,
        },
    }
}

fn yearly_branch_to_yaml(branch: &flat::YearlyCroplandBranch) -> yaml::YearlyCroplandBranch {
    match branch {
        flat::YearlyCroplandBranch::AnnualOrFallow(data) => {
            yaml::YearlyCroplandBranch::AnnualOrFallow {
                jdharv: data.jdharv,
                jdplt: data.jdplt,
                rw: data.rw,
                resmgt: data.resmgt,
                extension: data.extension.as_ref().map(yearly_extension_to_yaml),
            }
        }
        flat::YearlyCroplandBranch::Perennial(data) => yaml::YearlyCroplandBranch::Perennial {
            jdharv: data.jdharv,
            jdplt: data.jdplt,
            jdstop: data.jdstop,
            rw: data.rw,
            mgtopt: data.mgtopt,
            cut_days: data.cut_days.clone(),
            grazing_cycles: data
                .grazing_cycles
                .iter()
                .map(|cycle| yaml::YearlyPerennialGrazingCycle {
                    animal: cycle.animal,
                    area: cycle.area,
                    bodywt: cycle.bodywt,
                    digest: cycle.digest,
                    gday: cycle.gday,
                    gend: cycle.gend,
                })
                .collect(),
        },
    }
}

fn yearly_extension_to_yaml(
    extension: &flat::YearlyAnnualExtension,
) -> yaml::YearlyAnnualExtension {
    match extension {
        flat::YearlyAnnualExtension::Herbicide { jdherb } => {
            yaml::YearlyAnnualExtension::Herbicide { jdherb: *jdherb }
        }
        flat::YearlyAnnualExtension::Burn {
            jdburn,
            fbmag,
            fbrnog,
        } => yaml::YearlyAnnualExtension::Burn {
            jdburn: *jdburn,
            fbmag: *fbmag,
            fbrnog: *fbrnog,
        },
        flat::YearlyAnnualExtension::Silage { jdslge } => {
            yaml::YearlyAnnualExtension::Silage { jdslge: *jdslge }
        }
        flat::YearlyAnnualExtension::Cut { jdcut, frcut } => yaml::YearlyAnnualExtension::Cut {
            jdcut: *jdcut,
            frcut: *frcut,
        },
        flat::YearlyAnnualExtension::Remove { jdmove, frmove } => {
            yaml::YearlyAnnualExtension::Remove {
                jdmove: *jdmove,
                frmove: *frmove,
            }
        }
    }
}

fn schedule_to_yaml(schedule: &flat::ManagementSchedule) -> yaml::ManagementSchedule {
    yaml::ManagementSchedule {
        ofe_initial_refs: schedule.ofe_initial_refs.clone(),
        rotation_repeats: schedule.rotation_repeats,
        rotation_years: schedule.rotation_years,
        slots: schedule
            .slots
            .iter()
            .map(|slot| yaml::ManagementScheduleSlot {
                rotation_index: slot.rotation_index + 1,
                year_in_rotation: slot.year_in_rotation + 1,
                ofe_index: slot.ofe_index + 1,
                yearly_refs: slot.yearly_refs.clone(),
            })
            .collect(),
    }
}

fn stratum_to_yaml(stratum: flat::PlantForestStratum) -> yaml::PlantForestStratum {
    yaml::PlantForestStratum {
        coeff: stratum.coeff,
        diam: stratum.diam,
        hgt: stratum.hgt,
        pop: stratum.pop,
    }
}

fn description_vec(description: &[String; 3]) -> Vec<String> {
    description.to_vec()
}

fn validate_legacy_source_landuses(
    parsed: &flat::ManagementParseOutput,
) -> Result<(), LanduseMigrationError> {
    for (index, plant) in parsed.registries.plants.iter().enumerate() {
        if plant.meta.landuse != 1 || !matches!(plant.data, flat::PlantScenarioData::Cropland(_)) {
            return Err(LanduseMigrationError::UnsupportedSourceLanduse {
                detail: format!(
                    "legacy plant_index {} ({}) has unsupported landuse {}",
                    index + 1,
                    plant.meta.name,
                    plant.meta.landuse
                ),
            });
        }
    }
    Ok(())
}

fn checksum_for_native_flat_routing(routing: flat::RoutingCoefficientExtension) -> String {
    let canonical = format!(
        "{}|{}|{}|{}|{}",
        routing.skin_friction_coefficient_ko,
        routing.form_drag_coefficient,
        routing.roughness_element_height_m,
        routing.roughness_concentration,
        routing.vegetation_drag_coefficient
    );
    let digest = Sha256::digest(canonical.as_bytes());
    let mut hex = String::with_capacity(71);
    hex.push_str("sha256:");
    for byte in digest {
        push_hex_byte(&mut hex, byte);
    }
    hex
}

fn push_hex_byte(output: &mut String, byte: u8) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    output.push(HEX[usize::from(byte >> 4)] as char);
    output.push(HEX[usize::from(byte & 0x0f)] as char);
}

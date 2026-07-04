//! EROD16 Wave-1 continuity fixture-forcing conservation gate
//! (SC-SED-001, erosion port Increment-1b-A integration).
//!
//! Drives the Wave-1 single-OFE sediment-continuity solver with **real
//! McKenzie Bridge OR forcing** through the **production operand
//! producers** (`openwepp_hillslope_orchestrator::erosion_*`): the
//! `forest_high_severity_clay_loam` fixture is run end-to-end through
//! `openwepp-cli-hill`, its storm-day runoff forcing (`runvol`,
//! `peakro`) is read back from the published pass parquet, and each storm
//! day above the legacy event-size gate is solved with operands built by
//! the production producers from the fixture's own soil erodibility
//! (`ki`/`kr`/`shcrit`), texture-derived particle classes, and slope
//! profile.
//!
//! Evidence class: **Ran** for the fixture execution, the production
//! operand producers, and the solver. This test replaced the earlier
//! test-harness operand port (Increment-1) with the 1b-A production
//! producers; the two operands still built here as documented test inputs
//! are the ones whose production producers are NOT pure functions and are
//! deferred within the WP:
//!   - `effint`/`effdrr` (rainfall-excess mean intensity / duration) —
//!     a runtime export from the WB14/WB16 excess machinery, not a pure
//!     producer; approximated here by `effint = runoff/effdrn`,
//!     `effdrr = effdrn` (conservative). Its runtime export is 1b-A's
//!     runtime-integration item.
//!   - `kiadjf`/`kradjf`/`tcadjf` (daily erodibility adjustments) — the
//!     `soil.for` daily chain is Increment-1b-B; held at the day-zero
//!     initialization value 1.0 here (legacy `inidat.for:424`).
//!
//! The fixture is a forest high-severity burn (physically non-cropland
//! even though it masquerades as `lanuse = 1`), so the non-cropland
//! interrill delivery branch (`intdr = 1`) is the correct one.
//!
//! Gate (hard): every storm day solves fail-closed-clean; the
//! telescoping conservation identity `exported - inflow =
//! detachment - deposition` holds on every active day; the fixture
//! generates nonzero detachment and exports sediment at the toe (the
//! McKenzie-class activation proof). Magnitudes are not asserted
//! (ADR-0017).

// Legacy naming continuity (`effdrr`/`effdrn`) and prose fixture-name
// tokens mirror the `.for` sources and the WP artifacts; the single
// end-to-end fixture driver is one long function by design.
#![allow(clippy::similar_names, clippy::doc_markdown, clippy::too_many_lines)]

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::{
    DirectWave1ContinuityInputs, ErosionRillCoverInputs, ErosionShearSlopes, ErosionTextureInputs,
    compute_direct_wave1_continuity, derive_wave1_slope_segments, erosion_detinr,
    erosion_effective_particle, erosion_falvel, erosion_interrill_delivery_ratio,
    erosion_particle_composition, erosion_rill_hydraulics, erosion_transport_coefficients,
};
use openwepp_input_contract::parsers::slope::{SlopeParserOptions, parse_slope_file};
use openwepp_input_contract::parsers::soil::{ParserMode, SoilParserOptions, parse_soil};
use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

// `contin.for:977` event-size bypass bounds.
const PASSBY_RUNOFF_M: f64 = 0.010;
const PASSBY_PEAKRO_M_S: f64 = 2.78e-6;
// Increment-1b-B pending: daily erodibility adjustments held at the
// `inidat.for:424` day-zero initialization value.
const ADJUSTMENT_FACTOR_INIT: f64 = 1.0;
// Cropland rill spacing default (`xinflo.for:132` context; 1.0 m).
const RSPACE_M: f64 = 1.0;

#[derive(Debug, Clone, Copy)]
struct StormDay {
    sim_day_index: usize,
    runoff_depth_m: f64,
    peakro_m_s: f64,
}

#[test]
fn erod16_wave1_continuity_conserves_on_mckenzie_clay_loam_storm_forcing() {
    let fixture = fixture_path();
    let run_dir = copy_fixture_to_temp(&fixture, "erod16_wave1_p4");
    normalize_legacy_nan_dewpoint_tokens(&run_dir);

    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p4.run.toml"),
            output_dir: run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("clay-loam McKenzie fixture should run end-to-end");
    let pass_parquet = report
        .optional_outputs
        .iter()
        .find(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .is_some_and(|name| name.ends_with(".pass.parquet"))
        })
        .expect("fixture run should publish the pass parquet");

    // Fixture-derived static operands.
    let soil = parse_soil(
        &fs::read_to_string(run_dir.join("p4.sol")).expect("fixture soil should read"),
        SoilParserOptions {
            mode: ParserMode::Compatibility,
            allow_legacy_aliases: true,
            expected_topology_count: None,
            topology_scope: None,
        },
    )
    .expect("fixture soil should parse");
    let soil_ofe = soil.ofes.first().expect("fixture soil has one OFE");
    let layer = soil_ofe.layers.first().expect("fixture soil has layers");
    let sand = layer.sand_pct / 100.0;
    let clay = layer.clay_pct / 100.0;
    let silt = 1.0 - sand - clay;
    let orgmat = layer.orgmat_pct / 100.0;

    let slope = parse_slope_file(&run_dir.join("p4.slp"), SlopeParserOptions::strict())
        .expect("fixture slope should parse");
    let slope_ofe = slope.ofes.first().expect("fixture slope has one OFE");
    let slplen_m = slope_ofe.slplen;
    let fwidth_m = slope_ofe.fwidth;
    let points_m: Vec<(f64, f64)> = slope_ofe
        .points
        .iter()
        .map(|point| (point.xinput * slplen_m, point.slpinp))
        .collect();
    let avgslp = average_slope(
        &slope_ofe
            .points
            .iter()
            .map(|p| (p.xinput, p.slpinp))
            .collect::<Vec<_>>(),
    );
    assert!(avgslp > 0.3, "canonical steep forest hillslope expected");
    let segments = derive_wave1_slope_segments(&points_m, slplen_m, avgslp)
        .expect("fixture slope profile must fit");
    let last = segments.last().expect("segments exist");
    let slpend = (last.a + last.b) * avgslp;

    // Production particle classes and effective particle (1b-A producers).
    let texture = ErosionTextureInputs {
        sand,
        clay,
        silt,
        orgmat,
    };
    let classes = erosion_particle_composition(&texture).expect("production particle composition");
    let (diaeff, spgeff) =
        erosion_effective_particle(&classes).expect("production effective particle");
    let veleff = erosion_falvel(spgeff, diaeff);
    assert!(veleff > 0.0);
    // Forest burn: non-cropland interrill delivery (`intdr = 1`).
    let intdr = erosion_interrill_delivery_ratio(false, 0.0, &classes)
        .expect("production interrill delivery");

    // Bare-burn rill cover: no residue/live cover, so frccov = frlive = 0
    // and frctrl = frcsol = 1.11 (`frcfac.for:222`).
    let cover = ErosionRillCoverInputs {
        rilcov: 0.0,
        canhgt_m: 0.0,
        hmax_m: 0.0,
        flivmx: 0.0,
    };
    let slopes = ErosionShearSlopes {
        cnslp: avgslp,
        slpend,
    };

    // Storm-day forcing from the fixture's own published pass parquet.
    let storm_days = read_storm_days(pass_parquet, fwidth_m, slplen_m);
    assert!(
        !storm_days.is_empty(),
        "McKenzie clay-loam fixture must produce storm days above the \
         legacy event-size gate (erosion-inert fixtures are excluded by \
         the handoff)"
    );

    let mut total_detachment_kg = 0.0;
    let mut total_deposition_kg = 0.0;
    let mut days_with_export = 0_usize;
    for storm in &storm_days {
        // Production rill hydraulics + transport coefficients (1b-A).
        let qout = storm.peakro_m_s * slplen_m;
        let qshear = qout * RSPACE_M;
        let hydraulics = erosion_rill_hydraulics(qshear, &slopes, &cover, 0.0, RSPACE_M)
            .unwrap_or_else(|error| {
                panic!(
                    "production rill hydraulics must resolve for storm day {}: {error:?}",
                    storm.sim_day_index
                )
            });
        let transport = erosion_transport_coefficients(
            hydraulics.shrsol_pa,
            hydraulics.shrend_pa,
            &classes,
            sand,
        )
        .unwrap_or_else(|error| {
            panic!(
                "production transport coefficients must resolve for storm day {}: {error:?}",
                storm.sim_day_index
            )
        });

        // effint/effdrr: WB14/WB16 export deferred (documented above);
        // approximated by the runoff-duration surface.
        let effdrn_s = (storm.runoff_depth_m / storm.peakro_m_s).min(86_400.0);
        let effdrr_s = effdrn_s;
        let effint = storm.runoff_depth_m / effdrr_s;

        // Production detinr (1b-A); kiadjf at the 1b-B-pending init value.
        let detinr = erosion_detinr(
            soil_ofe.ki,
            ADJUSTMENT_FACTOR_INIT,
            effint,
            storm.runoff_depth_m,
            effdrr_s,
            intdr,
            RSPACE_M,
            hydraulics.width_m,
        )
        .expect("production detinr must resolve");

        let inputs = DirectWave1ContinuityInputs {
            enabled: true,
            segments: segments.clone(),
            peakro_m_s: storm.peakro_m_s,
            runoff_depth_m: storm.runoff_depth_m,
            qin_m2_s: 0.0,
            efflen_m: slplen_m,
            slplen_m,
            cntlen_m: slplen_m,
            rspace_m: RSPACE_M,
            width_m: hydraulics.width_m,
            field_width_m: fwidth_m,
            effdrn_s,
            effdrr_s,
            kr_s_m: soil_ofe.kr,
            kradjf: ADJUSTMENT_FACTOR_INIT,
            shcrit_pa: soil_ofe.shcrit,
            tcadjf: ADJUSTMENT_FACTOR_INIT,
            detinr_kg_s_m2: detinr,
            shrsol_pa: hydraulics.shrsol_pa,
            tcend_kg_s_m: transport.tcend_kg_s_m,
            ktrato: transport.ktrato,
            veleff_m_s: veleff,
            beta: 0.5,
            strldn: 0.0,
            surface_frozen: false,
            theta_suppressed: false,
        };

        let state = compute_direct_wave1_continuity(&inputs).unwrap_or_else(|error| {
            panic!(
                "storm day {} (runoff {:.4} m, peakro {:.3e} m/s) must solve \
                 fail-closed-clean: {error:?}",
                storm.sim_day_index, storm.runoff_depth_m, storm.peakro_m_s
            )
        });
        assert!(
            state.active,
            "storm day {} above the event gate must activate the solve",
            storm.sim_day_index
        );

        // Hard conservation identity on every active day (denormalized).
        let detach_kg_m = state.total_detachment_kg / fwidth_m;
        let depos_kg_m = state.total_deposition_kg / fwidth_m;
        let residual = (state.exported_sediment_kg_m - state.inflow_sediment_kg_m)
            - (detach_kg_m - depos_kg_m);
        let scale = state
            .exported_sediment_kg_m
            .abs()
            .max(detach_kg_m.abs())
            .max(1.0e-9);
        assert!(
            residual.abs() <= 1.0e-9 * scale,
            "storm day {} conservation residual {residual} exceeds gate at scale {scale}",
            storm.sim_day_index
        );

        // Pointwise invariants on the committed grid (INV-SED-001/003/006).
        for i in 0..state.load.len() {
            assert!(state.load[i] >= 0.0 && state.load[i].is_finite());
            assert!(
                state.tcap[i] >= 0.0,
                "transport capacity clamp (INV-SED-006)"
            );
        }

        total_detachment_kg += state.total_detachment_kg;
        total_deposition_kg += state.total_deposition_kg;
        if state.exported_sediment_kg_m > 0.0 {
            days_with_export += 1;
        }
    }

    // The McKenzie-class activation proof: the fixture generates real
    // detachment on storm days (magnitude is NOT asserted — ADR-0017).
    assert!(
        total_detachment_kg > 0.0,
        "clay-loam McKenzie storm population must generate detachment"
    );
    assert!(
        days_with_export > 0,
        "storm days must export sediment at the OFE toe"
    );
    assert!(
        total_deposition_kg >= 0.0,
        "deposition totals must be nonnegative"
    );

    // ------------------------------------------------------------------
    // E.1 (Increment 1c-fidelity) depositing-limb coverage: the same real
    // storm forcing and production operand chain, on a crafted CONCAVE
    // validation profile (steep upper reach, near-flat toe). Transport
    // capacity collapses on the toe, so the load built upslope must
    // deposit — proving the deposition surfaces (`tdep`) conserve under
    // producer-built operands, not only under crafted unit payloads.
    //
    // The profile is a validation INSTRUMENT, deliberately near the
    // 101-point grid's resolution envelope: toe flatness is calibrated to
    // this fixture's weak rill erodibility (`kr = 6e-5`, the load is
    // mostly interrill supply), and a flat toe also makes the
    // detachment relaxation stiff (`eata ∝ shrsol/tcend` grows as the
    // end shear collapses). On the slowest-peak storms that stiffness
    // exceeds what the fixed grid can resolve, and the solver REFUSES
    // via the named `flux_closure` discretization gate rather than
    // mis-integrating — the fail-closed design working as intended
    // (legacy's identical 100-point grid has no such gate and integrates
    // those days silently). The loop below therefore classifies
    // outcomes: clean solves must conserve and (in aggregate) deposit;
    // `flux_closure` refusals are permitted only as a bounded tail; any
    // OTHER error class fails the test.
    // ------------------------------------------------------------------
    let concave_points_norm: [(f64, f64); 6] = [
        (0.0, 0.85),
        (0.30, 0.70),
        (0.55, 0.18),
        (0.75, 0.03),
        (0.88, 0.008),
        (1.0, 0.003),
    ];
    let concave_points_m: Vec<(f64, f64)> = concave_points_norm
        .iter()
        .map(|(x, s)| (x * slplen_m, *s))
        .collect();
    let concave_avgslp = average_slope(concave_points_norm.as_ref());
    let concave_segments = derive_wave1_slope_segments(&concave_points_m, slplen_m, concave_avgslp)
        .expect("concave depositing profile must fit");
    let concave_last = concave_segments.last().expect("segments exist");
    let concave_slopes = ErosionShearSlopes {
        cnslp: concave_avgslp,
        slpend: (concave_last.a + concave_last.b) * concave_avgslp,
    };

    let mut depositing_days = 0_usize;
    let mut concave_clean_days = 0_usize;
    let mut concave_flux_refusals = 0_usize;
    let mut concave_detachment_kg = 0.0;
    let mut concave_deposition_kg = 0.0;
    for storm in &storm_days {
        let qout = storm.peakro_m_s * slplen_m;
        let qshear = qout * RSPACE_M;
        let hydraulics = erosion_rill_hydraulics(qshear, &concave_slopes, &cover, 0.0, RSPACE_M)
            .unwrap_or_else(|error| {
                panic!(
                    "concave rill hydraulics must resolve for storm day {}: {error:?}",
                    storm.sim_day_index
                )
            });
        let transport = erosion_transport_coefficients(
            hydraulics.shrsol_pa,
            hydraulics.shrend_pa,
            &classes,
            sand,
        )
        .unwrap_or_else(|error| {
            panic!(
                "concave transport coefficients must resolve for storm day {}: {error:?}",
                storm.sim_day_index
            )
        });
        let effdrn_s = (storm.runoff_depth_m / storm.peakro_m_s).min(86_400.0);
        let effdrr_s = effdrn_s;
        let effint = storm.runoff_depth_m / effdrr_s;
        let detinr = erosion_detinr(
            soil_ofe.ki,
            ADJUSTMENT_FACTOR_INIT,
            effint,
            storm.runoff_depth_m,
            effdrr_s,
            intdr,
            RSPACE_M,
            hydraulics.width_m,
        )
        .expect("concave detinr must resolve");

        let inputs = DirectWave1ContinuityInputs {
            enabled: true,
            segments: concave_segments.clone(),
            peakro_m_s: storm.peakro_m_s,
            runoff_depth_m: storm.runoff_depth_m,
            qin_m2_s: 0.0,
            efflen_m: slplen_m,
            slplen_m,
            cntlen_m: slplen_m,
            rspace_m: RSPACE_M,
            width_m: hydraulics.width_m,
            field_width_m: fwidth_m,
            effdrn_s,
            effdrr_s,
            kr_s_m: soil_ofe.kr,
            kradjf: ADJUSTMENT_FACTOR_INIT,
            shcrit_pa: soil_ofe.shcrit,
            tcadjf: ADJUSTMENT_FACTOR_INIT,
            detinr_kg_s_m2: detinr,
            shrsol_pa: hydraulics.shrsol_pa,
            tcend_kg_s_m: transport.tcend_kg_s_m,
            ktrato: transport.ktrato,
            veleff_m_s: veleff,
            beta: 0.5,
            strldn: 0.0,
            surface_frozen: false,
            theta_suppressed: false,
        };

        let state = match compute_direct_wave1_continuity(&inputs) {
            Ok(state) => state,
            Err(error) => {
                let detail = format!("{error:?}");
                assert!(
                    detail.contains("flux_closure"),
                    "concave storm day {}: only the named flux-closure \
                     discretization refusal is permitted on the crafted-stiff \
                     instrument profile, got {detail}",
                    storm.sim_day_index
                );
                concave_flux_refusals += 1;
                continue;
            }
        };
        assert!(state.active);
        concave_clean_days += 1;

        // The same hard conservation identity on the depositing profile.
        let detach_kg_m = state.total_detachment_kg / fwidth_m;
        let depos_kg_m = state.total_deposition_kg / fwidth_m;
        let residual = (state.exported_sediment_kg_m - state.inflow_sediment_kg_m)
            - (detach_kg_m - depos_kg_m);
        let scale = state
            .exported_sediment_kg_m
            .abs()
            .max(detach_kg_m.abs())
            .max(1.0e-9);
        assert!(
            residual.abs() <= 1.0e-9 * scale,
            "concave storm day {} conservation residual {residual} exceeds gate",
            storm.sim_day_index
        );

        if state.total_deposition_kg > 0.0 {
            depositing_days += 1;
            // Net accounting on a depositing day: the toe export cannot
            // exceed what detachment supplied minus what deposited (the
            // telescoping identity, restated as an export bound).
            assert!(
                state.exported_sediment_kg_m <= detach_kg_m - depos_kg_m + 1.0e-9 * scale,
                "concave storm day {} export must respect the deposition debit",
                storm.sim_day_index
            );
        }

        concave_detachment_kg += state.total_detachment_kg;
        concave_deposition_kg += state.total_deposition_kg;
    }

    // The depositing-limb proof: the concave toe must actually deposit on
    // real storm forcing (nonzero `tdep` is a produced value, not a
    // structural zero), while the profile still detaches upslope. The
    // discretization refusals must stay a bounded tail of the storm
    // population (the instrument must not degrade into refusing its way
    // past the coverage it exists to provide).
    println!(
        "concave depositing instrument: storms={} clean={concave_clean_days} \
         flux_refusals={concave_flux_refusals} depositing={depositing_days} \
         tdet={concave_detachment_kg:.1} kg tdep={concave_deposition_kg:.1} kg",
        storm_days.len()
    );
    assert!(
        concave_detachment_kg > 0.0,
        "concave profile must still detach on its steep upper reach"
    );
    assert!(
        depositing_days > 0 && concave_deposition_kg > 0.0,
        "concave near-flat toe must produce nonzero conserving deposition \
         on real storm forcing (days={depositing_days}, tdep={concave_deposition_kg} kg)"
    );
    assert!(
        concave_clean_days > 0 && depositing_days * 4 >= concave_clean_days,
        "the depositing limb must engage on a substantial share of clean \
         solves (depositing={depositing_days}, clean={concave_clean_days})"
    );
    assert!(
        concave_flux_refusals * 5 <= storm_days.len(),
        "flux-closure refusals must stay a bounded tail (<= 20%) of the \
         storm population (refusals={concave_flux_refusals}, \
         storms={})",
        storm_days.len()
    );
}

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("disturbed_burn")
        .join("forest_high_severity_clay_loam")
}

fn copy_fixture_to_temp(fixture: &Path, prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
    fs::create_dir_all(destination.join("output")).expect("output dir should be creatable");
    for name in ["p4.run.toml", "p4.sol", "p4.man", "p4.slp", "p4.cli"] {
        fs::copy(fixture.join(name), destination.join(name))
            .unwrap_or_else(|error| panic!("fixture file {name} should copy: {error}"));
    }
    destination
}

fn normalize_legacy_nan_dewpoint_tokens(run_dir: &Path) {
    let path = run_dir.join("p4.cli");
    let contents = fs::read_to_string(&path).expect("climate file should read");
    let updated = contents.replace(" nan", " 0.0");
    if updated != contents {
        fs::write(&path, updated).expect("climate file should be writable");
    }
}

/// `profil.for:37-45`: average slope = total drop / normalized length via
/// trapezoid integration of the slope profile.
fn average_slope(points: &[(f64, f64)]) -> f64 {
    let mut drop = 0.0;
    for window in points.windows(2) {
        let (x0, s0) = window[0];
        let (x1, s1) = window[1];
        drop += (x1 - x0) * (s0 + s1) / 2.0;
    }
    let length = points.last().expect("points exist").0;
    (drop / length).max(1.0e-6)
}

fn read_storm_days(pass_parquet: &Path, fwidth_m: f64, efflen_m: f64) -> Vec<StormDay> {
    let file = File::open(pass_parquet).expect("pass parquet should open");
    let reader = SerializedFileReader::new(file).expect("pass parquet should parse");
    let mut storm_days = Vec::new();
    for (index, row) in reader
        .get_row_iter(None)
        .expect("pass parquet rows should iterate")
        .enumerate()
    {
        let row = row.expect("pass parquet row should decode");
        let runvol_m3 = row_f64_value(&row, "runvol");
        let peakro = row_f64_value(&row, "peakro");
        let runoff_depth_m = runvol_m3 / (fwidth_m * efflen_m);
        // Legacy `passby` gate: route sediment only when the event crosses
        // either bound (`contin.for:977`).
        if runoff_depth_m > 0.0
            && peakro > 0.0
            && !(runoff_depth_m <= PASSBY_RUNOFF_M && peakro <= PASSBY_PEAKRO_M_S)
        {
            storm_days.push(StormDay {
                sim_day_index: index,
                runoff_depth_m,
                peakro_m_s: peakro,
            });
        }
    }
    storm_days
}

fn row_f64_value(row: &Row, column_name: &str) -> f64 {
    let index = row
        .get_column_iter()
        .enumerate()
        .find(|(_, (name, _))| name.as_str() == column_name)
        .map_or_else(
            || panic!("missing required parquet column '{column_name}'"),
            |(index, _)| index,
        );
    if let Ok(value) = row.get_double(index) {
        return value;
    }
    if let Ok(value) = row.get_float(index) {
        return f64::from(value);
    }
    panic!("column '{column_name}' does not decode as numeric");
}

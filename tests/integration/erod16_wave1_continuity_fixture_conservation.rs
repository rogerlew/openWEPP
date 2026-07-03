//! EROD16 Wave-1 continuity fixture-forcing conservation gate
//! (SC-SED-001, erosion port Increment-1).
//!
//! Drives the Wave-1 single-OFE sediment-continuity solver with **real
//! McKenzie Bridge OR forcing**: the `forest_high_severity_clay_loam`
//! fixture is run end-to-end through `openwepp-cli-hill` (direct
//! production runtime), its storm-day runoff forcing (`runvol`,
//! `peakro`) is read back from the published pass parquet, and each
//! storm day above the legacy event-size gate is solved with operands
//! built from the fixture's own soil erodibility (`ki`/`kr`/`shcrit`),
//! texture-derived particle classes, and slope profile.
//!
//! Evidence class: **Ran** for the fixture execution and the solver;
//! the operand-construction chain in this file is a **test-harness
//! Static port** of the legacy `prtcmp.for`/`falvel.for`/`shield.for`/
//! `yalin.for`/`trcoef.for`/`shears.for`/`param.for` lineage used ONLY
//! to build test forcing. Production operand producers are the declared
//! Increment-1b scope (see the work-package implementation record).
//! Documented harness assumptions (labeled, not production math):
//!   - `kiadjf = kradjf = tcadjf = 1.0` (legacy `inidat.for:424`
//!     initialization values; the daily `soil.for` adjustment chain has
//!     no openWEPP producer yet),
//!   - bare-burn rill friction `frcsol = frctrl = 1.11`
//!     (`frcfac.for:222` Gilley bare-soil value, zero residue/live cover),
//!   - `rspace = 1.0 m`, rill width from Gilley `1.13 q^0.303`,
//!   - `effdrr = effdrn` and `effint = qi` (rainfall-excess surfaces are
//!     not yet exported by the runtime; conservative lower bound).
//!
//! Gate (hard): every storm day must solve fail-closed-clean; the
//! telescoping conservation identity `exported - inflow =
//! detachment - deposition` must hold on every active day; the fixture
//! must actually generate detachment (nonzero exported sediment on storm
//! days) — the McKenzie-class activation proof required by the handoff.

// Legacy naming continuity (`effdrr`/`effdrn`, `frac`/`fracs`, `frclyt`),
// legacy-table literals (the `cdre` drag table sits at ln(10) decades),
// and the single long fixture-driver function mirror the `.for` sources.
#![allow(
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::approx_constant,
    clippy::doc_markdown
)]

use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::{
    DirectWave1ContinuityInputs, compute_direct_wave1_continuity, derive_wave1_slope_segments,
};
use openwepp_input_contract::parsers::slope::{SlopeParserOptions, parse_slope_file};
use openwepp_input_contract::parsers::soil::{ParserMode, SoilParserOptions, parse_soil};
use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::{Row, RowAccessor};

// Legacy physical constants (`inidat.for:1054-1151`).
const ACCGAV: f64 = 9.807;
const WTDENS: f64 = 9807.0;
const KINVIS: f64 = 1.0e-6;
const MSDENS: f64 = 1000.0;
// `contin.for:977` event-size bypass bounds.
const PASSBY_RUNOFF_M: f64 = 0.010;
const PASSBY_PEAKRO_M_S: f64 = 2.78e-6;
// Harness assumptions (documented in the module header).
const HARNESS_ADJUSTMENT_FACTOR: f64 = 1.0;
const HARNESS_RILL_FRICTION: f64 = 1.11;
const HARNESS_RSPACE_M: f64 = 1.0;

#[derive(Debug, Clone, Copy)]
struct ParticleClass {
    dia_m: f64,
    spg: f64,
    frac: f64,
}

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
    assert!(
        (0.0..1.0).contains(&sand) && (0.0..1.0).contains(&clay) && silt > 0.0,
        "clay-loam texture must be a valid fraction triple"
    );

    let slope = parse_slope_file(&run_dir.join("p4.slp"), SlopeParserOptions::strict())
        .expect("fixture slope should parse");
    let slope_ofe = slope.ofes.first().expect("fixture slope has one OFE");
    let slplen_m = slope_ofe.slplen;
    let fwidth_m = slope_ofe.fwidth;
    // Fixture points carry normalized distances; scale to meters for the
    // `profil.for` fit.
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
    // Normalized slope at the toe for the `param.for:168` end-slope shear.
    let last = segments.last().expect("segments exist");
    let slpend = (last.a + last.b) * avgslp;

    // Texture-derived particle classes (`prtcmp.for` core) and the
    // effective particle surface (`param.for:558-579`).
    let classes = particle_composition(sand, clay, silt, orgmat);
    let frac_sum: f64 = classes.iter().map(|class| class.frac).sum();
    assert!(
        (frac_sum - 1.0).abs() < 1.0e-6,
        "particle class fractions must sum to 1, observed {frac_sum}"
    );
    let (diaeff, spgeff) = effective_particle(&classes);
    let veleff = falvel(spgeff, diaeff);
    assert!(veleff > 0.0);

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
        let qout = storm.peakro_m_s * slplen_m;
        let qshear = qout * HARNESS_RSPACE_M;
        let width_m = 1.13 * qshear.powf(0.303);
        let shrsol = shears(qshear, avgslp, width_m);
        let shrend = shears(qshear, slpend, width_m);
        let kt = trcoef(shrsol, &classes, sand);
        let kt2 = trcoef(0.5 * (shrend + shrsol), &classes, sand);
        let ktrato = kt2 / kt;
        let tcend = (kt * shrsol.powf(1.5)).max(1.0e-10);

        let effdrn_s = (storm.runoff_depth_m / storm.peakro_m_s).min(86_400.0);
        let effdrr_s = effdrn_s;
        let qi = storm.runoff_depth_m / effdrr_s;
        let effint = qi;
        // `param.for:482` with intdr = 1 (rif clamps to 1 at the smooth
        // rrc bound) and the harness `kiadjf`.
        let detinr =
            soil_ofe.ki * HARNESS_ADJUSTMENT_FACTOR * effint * qi * HARNESS_RSPACE_M / width_m;

        let inputs = DirectWave1ContinuityInputs {
            enabled: true,
            segments: segments.clone(),
            peakro_m_s: storm.peakro_m_s,
            runoff_depth_m: storm.runoff_depth_m,
            qin_m2_s: 0.0,
            efflen_m: slplen_m,
            slplen_m,
            cntlen_m: slplen_m,
            rspace_m: HARNESS_RSPACE_M,
            width_m,
            field_width_m: fwidth_m,
            effdrn_s,
            effdrr_s,
            kr_s_m: soil_ofe.kr,
            kradjf: HARNESS_ADJUSTMENT_FACTOR,
            shcrit_pa: soil_ofe.shcrit,
            tcadjf: HARNESS_ADJUSTMENT_FACTOR,
            detinr_kg_s_m2: detinr,
            shrsol_pa: shrsol,
            tcend_kg_s_m: tcend,
            ktrato,
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

// ---------------------------------------------------------------------
// Test-harness Static ports of the legacy operand chain (forcing
// construction only; production producers are Increment-1b).
// ---------------------------------------------------------------------

/// `prtcmp.for`: five-class particle composition from soil texture,
/// including the large-aggregate clay-enrichment correction re-entry.
fn particle_composition(sand: f64, clay: f64, silt: f64, _orgmat: f64) -> Vec<ParticleClass> {
    let mut dia_mm = [0.002_f64, 0.010, 0.030, 0.300, 0.200];
    let spg = [2.60_f64, 2.65, 1.80, 1.60, 2.65];
    if clay > 0.15 {
        dia_mm[3] = 2.0 * clay;
    }

    let frac1 = if clay > 0.0 && clay < 1.0 {
        0.26 * clay
    } else if clay <= 0.0 {
        0.0001
    } else {
        0.9996
    };
    let mut frac5 = sand * (1.0 - clay).powi(5);
    if frac5 <= 0.0 {
        frac5 = 0.0001;
    }

    // Small-aggregate class 3 diameter/fraction by clay band.
    let mut frac3;
    if clay >= 1.0 {
        dia_mm[2] = 0.180;
        frac3 = 0.0001;
    } else if clay <= 0.25 {
        dia_mm[2] = 0.030;
        frac3 = 1.8 * clay;
        if frac3 <= 0.0 {
            frac3 = 0.0001;
        }
    } else if clay < 0.60 {
        dia_mm[2] = 0.20 * (clay - 0.25) + 0.030;
        if clay >= 0.50 {
            frac3 = 0.6 * clay;
        } else {
            frac3 = 0.45 - 0.6 * (clay - 0.25);
        }
    } else {
        dia_mm[2] = 0.1;
        frac3 = 0.6 * clay;
    }

    let frcly3 = if clay > 0.0 && silt > 0.0 {
        clay / (clay + silt)
    } else {
        0.0
    };

    // Label-20 block with the single legacy `jflag` re-entry.
    let mut fracs = [0.0_f64; 5];
    for pass in 0..2 {
        let mut frac2 = silt - frac3;
        let mut frac3_local = frac3;
        if frac2 <= 0.0 {
            frac2 = 0.0001;
            frac3_local = silt - frac2;
            if frac3_local <= 0.0 {
                frac3_local = 0.0001;
            }
        }
        let mut frac4 = 1.0 - frac1 - frac2 - frac3_local - frac5;
        fracs = [frac1, frac2, frac3_local, frac4, frac5];
        if frac4 <= 0.0 {
            let crct = 1.0 / (1.0 + frac4.abs() + 0.0001);
            fracs[3] = 0.0001;
            for value in &mut fracs {
                *value *= crct;
            }
            frac4 = fracs[3];
        }

        if pass == 1 {
            break;
        }
        // Large-aggregate clay-content correction (`prtcmp.for:288-300`).
        let frcly4 = if frac4 > 0.0001 {
            let value = (clay - fracs[0] - frcly3 * fracs[2]) / frac4;
            if (0.0..=1.0).contains(&value) {
                value
            } else {
                0.0
            }
        } else {
            0.0
        };
        let frclyt = 0.5 * clay;
        let frcly1 = 0.95 * frclyt;
        if clay < 1.0 && frcly4 < frcly1 && (frcly3 - frclyt).abs() > 0.0 {
            let f1f2f5 = fracs[0] + fracs[1] + fracs[4];
            frac3 = (clay - frclyt - fracs[0] + frclyt * f1f2f5) / (frcly3 - frclyt);
            if frac3 <= 0.0 {
                frac3 = 0.0001;
            }
            continue;
        }
        break;
    }

    (0..5)
        .map(|k| ParticleClass {
            dia_m: dia_mm[k] / 1000.0,
            spg: spg[k],
            frac: fracs[k],
        })
        .collect()
}

/// `param.for:558-579`: effective particle diameter and specific gravity
/// as the fraction-weighted log means of the three smallest classes.
fn effective_particle(classes: &[ParticleClass]) -> (f64, f64) {
    let mut diaeff = 0.0;
    let mut spgeff = 0.0;
    let mut sumf = 0.0;
    for class in classes.iter().take(3) {
        diaeff += class.frac * class.dia_m.ln();
        spgeff += class.frac * class.spg.ln();
        sumf += class.frac;
    }
    ((diaeff / sumf).exp(), (spgeff / sumf).exp())
}

/// `falvel.for`: particle fall velocity from the drag-coefficient table
/// (`inidat.for:1017-1034`) with the Stokes small-particle branch.
fn falvel(spg: f64, dia_m: f64) -> f64 {
    const CDRE: [f64; 9] = [
        -6.907_75, -4.605_17, -2.302_58, 0.0, 2.302_58, 4.605_17, 6.907_75, 9.210_34, 11.512_92,
    ];
    const CDRE2: [f64; 9] = [
        -4.509_86, -1.514_13, 0.788_46, 3.126_76, 6.040_25, 9.305_65, 13.081_54, 17.504_39,
        22.291_88,
    ];
    let rtsid = ((spg - 1.0) * ACCGAV * dia_m.powi(3) / (KINVIS * KINVIS)) * (8.0 / 6.0);
    if rtsid >= 0.024 {
        let target = rtsid.ln();
        for i in 1..9 {
            if CDRE2[i] > target {
                let rey = ((target - CDRE2[i - 1]) / (CDRE2[i] - CDRE2[i - 1])
                    * (CDRE[i] - CDRE[i - 1])
                    + CDRE[i - 1])
                    .exp();
                return rey * KINVIS / dia_m;
            }
        }
        CDRE[8].exp() * KINVIS / dia_m
    } else {
        (dia_m * dia_m * (spg - 1.0) * ACCGAV) / (KINVIS * 18.0)
    }
}

/// `shield.for`: dimensionless critical shear from the Shields diagram
/// (including the legacy mixed linear/log extrapolation above the table).
fn shield(reyn: f64) -> f64 {
    const Y: [f64; 8] = [0.0772, 0.0579, 0.04, 0.035, 0.034, 0.045, 0.055, 0.057];
    const R: [f64; 8] = [1.0, 2.0, 4.0, 8.0, 12.0, 100.0, 400.0, 1000.0];
    let ycr = if reyn < R[0] {
        let slope = (Y[1].ln() - Y[0].ln()) / (R[1].ln() - R[0].ln());
        Y[0].ln() - slope * (R[0].ln() - reyn.ln())
    } else if reyn > R[7] {
        let slope = (Y[7].ln() - Y[6].ln()) / (R[7].ln() - R[6].ln());
        Y[7] + slope * (reyn.ln() - R[7].ln())
    } else {
        let mut value = Y[7].ln();
        for i in 1..8 {
            if reyn >= R[i - 1] && reyn <= R[i] {
                let slope = (Y[i].ln() - Y[i - 1].ln()) / (R[i].ln() - R[i - 1].ln());
                value = Y[i - 1].ln() + slope * (reyn.ln() - R[i - 1].ln());
                break;
            }
        }
        value
    };
    ycr.exp()
}

/// `yalin.for`: total transport capacity at a shear (kg m^-1 s^-1) with
/// the class-fraction weighting and the sandy-soil adjustment.
fn yalin(effsh: f64, classes: &[ParticleClass], sand: f64) -> f64 {
    let yalcon = 0.635;
    let vstar = (effsh / MSDENS).sqrt();
    let mut t = 0.0;
    let mut deltas = vec![0.0_f64; classes.len()];
    let mut p = vec![0.0_f64; classes.len()];
    for (k, class) in classes.iter().enumerate() {
        let reyn = vstar * class.dia_m / KINVIS;
        let ycrit = shield(reyn);
        let delta = vstar * vstar / ((class.spg - 1.0) * ACCGAV * class.dia_m * ycrit) - 1.0;
        if delta > 0.0 {
            let sigma = delta * 2.45 * class.spg.powf(-0.4) * ycrit.sqrt();
            deltas[k] = delta;
            p[k] = yalcon * delta * (1.0 - (1.0 + sigma).ln() / sigma);
            t += delta;
        }
    }
    if t == 0.0 {
        t = 1000.0;
    }
    let mut tottc = 0.0;
    #[allow(clippy::cast_precision_loss)]
    let npart = classes.len() as f64;
    for (k, class) in classes.iter().enumerate() {
        let coef = vstar * MSDENS * class.dia_m * class.spg;
        let ws = p[k] * (deltas[k] / t) * coef * (class.frac * npart);
        tottc += ws;
    }
    // Sandy transport adjustment (`yalin.for:141-145`, INV-SED-006 floor).
    if sand > 0.5 {
        let adjtc = (0.3 + 0.7 * (-12.52 * (sand - 0.5)).exp()).max(0.30);
        tottc *= adjtc;
    }
    tottc
}

/// `trcoef.for`: transport coefficient `kt = tottc / shear^1.5`.
fn trcoef(shear: f64, classes: &[ParticleClass], sand: f64) -> f64 {
    let kt = yalin(shear, classes, sand) / shear.powf(1.5);
    if kt == 0.0 { 1.0e-9 } else { kt }
}

/// `shears.for`: rill flow shear at the end of the slope via the Chezy
/// uniform-flow depth iteration (harness friction assumptions in the
/// module header).
fn shears(q: f64, sslope: f64, width_m: f64) -> f64 {
    let q = q.abs();
    let sslope = sslope.max(1.0e-6);
    let chezch = (8.0 * ACCGAV / HARNESS_RILL_FRICTION).sqrt();
    let mut depth = 0.2 * q.powf(0.36);
    if q > 0.0 {
        let u = (q / chezch / sslope.sqrt()).powf(2.0 / 3.0) / width_m;
        loop {
            let dz = depth;
            depth = u * (width_m + dz + dz).powf(1.0 / 3.0);
            if (dz / depth - 1.0).abs() <= 5.0e-6 {
                break;
            }
        }
    } else {
        depth = 0.0;
    }
    let xsarea = depth * width_m;
    let wp = width_m + 2.0 * depth;
    let hydrad = if wp > 1.0e-12 { xsarea / wp } else { 0.0 };
    let sinang = sslope.atan().sin();
    // `shears.for:134` multiplies by frcsol/frctrl; under the bare-burn
    // harness assumption (frcsol == frctrl) the partition ratio is 1.
    WTDENS * sinang * hydrad
}

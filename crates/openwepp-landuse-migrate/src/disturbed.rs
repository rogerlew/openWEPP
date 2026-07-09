//! Embedded Disturbed route-coefficient table used by migration.

use sha2::{Digest, Sha256};

use crate::LanduseMigrationError;

pub const DISTURBED_ROUTE_TABLE_ID: &str = "disturbed-route-coefficients";
pub const DISTURBED_ROUTE_TABLE_VERSION: &str = "ADR-0014-2026-07-07";
pub const DISTURBED_ROUTE_TABLE_SOURCE_AUTHORITY: &str = "ADR-0014; openWEPP WP 20260707-laned-router-d16-hybrid-disturbed-route-coeff-source-acquisition-001";

const TREATMENT_SUFFIXES: &[&str] = &[
    "-mulch_15",
    "-mulch_30",
    "-mulch_60",
    "-thinning",
    "-prescribed_fire",
];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisturbedRouteCoefficientRow {
    pub disturbed_class: &'static str,
    pub k_o: f64,
    pub form_c_d: f64,
    pub d_r_m: f64,
    pub lambda: f64,
    pub vegetation_c_d: f64,
}

const ROUTE_ROWS: &[DisturbedRouteCoefficientRow] = &[
    row("agriculture crops", 480.0, 0.25, 0.010, 0.050, 0.12),
    row("bare", 540.0, 0.00, 0.000, 0.000, 0.00),
    row("deciduous forest", 420.0, 0.90, 0.050, 0.180, 0.65),
    row("forest", 410.0, 0.95, 0.060, 0.200, 0.75),
    row("forest high sev fire", 530.0, 0.18, 0.006, 0.018, 0.08),
    row("forest low sev fire", 465.0, 0.58, 0.026, 0.085, 0.34),
    row("forest moderate sev fire", 490.0, 0.40, 0.016, 0.050, 0.20),
    row("forest prescribed fire", 450.0, 0.70, 0.035, 0.110, 0.45),
    row("grass high sev fire", 530.0, 0.08, 0.003, 0.010, 0.04),
    row("grass low sev fire", 475.0, 0.27, 0.010, 0.045, 0.15),
    row("grass moderate sev fire", 500.0, 0.18, 0.007, 0.026, 0.09),
    row("grass prescribed fire", 465.0, 0.32, 0.012, 0.055, 0.18),
    row("high use skid", 575.0, 0.03, 0.000, 0.000, 0.00),
    row("low or treated skid", 545.0, 0.12, 0.006, 0.020, 0.03),
    row("mixed forest", 415.0, 0.92, 0.055, 0.190, 0.70),
    row("mulch", 420.0, 0.85, 0.040, 0.180, 0.20),
    row("short grass", 460.0, 0.34, 0.014, 0.070, 0.24),
    row("shrub", 430.0, 0.72, 0.035, 0.120, 0.45),
    row("shrub high sev fire", 525.0, 0.14, 0.004, 0.014, 0.06),
    row("shrub low sev fire", 465.0, 0.44, 0.020, 0.065, 0.24),
    row("shrub moderate sev fire", 490.0, 0.30, 0.012, 0.038, 0.14),
    row("shrub prescribed fire", 450.0, 0.55, 0.026, 0.090, 0.32),
    row("skid", 560.0, 0.05, 0.000, 0.000, 0.00),
    row("tall grass", 440.0, 0.48, 0.020, 0.100, 0.35),
    row("thinning", 435.0, 0.90, 0.045, 0.160, 0.50),
    row("young forest", 430.0, 0.85, 0.045, 0.160, 0.60),
];

const fn row(
    disturbed_class: &'static str,
    k_o: f64,
    form_c_d: f64,
    d_r_m: f64,
    lambda: f64,
    vegetation_c_d: f64,
) -> DisturbedRouteCoefficientRow {
    DisturbedRouteCoefficientRow {
        disturbed_class,
        k_o,
        form_c_d,
        d_r_m,
        lambda,
        vegetation_c_d,
    }
}

#[must_use]
pub fn all_disturbed_classes() -> Vec<&'static str> {
    ROUTE_ROWS.iter().map(|row| row.disturbed_class).collect()
}

pub fn normalize_disturbed_class(value: &str) -> Result<&'static str, LanduseMigrationError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(LanduseMigrationError::MissingMigrationAuthority {
            site: "disturbed_class".to_string(),
        });
    }

    let mut route_class = trimmed;
    for suffix in TREATMENT_SUFFIXES {
        if let Some(stripped) = route_class.strip_suffix(suffix) {
            route_class = stripped;
            break;
        }
    }

    if row_by_exact_class(route_class).is_none() {
        if route_class.contains("mulch") {
            route_class = "mulch";
        } else if route_class.contains("thinning") {
            route_class = "young forest";
        }
    }

    row_by_exact_class(route_class)
        .map(|row| row.disturbed_class)
        .ok_or_else(|| LanduseMigrationError::UnknownDisturbedClass {
            disturbed_class: value.to_string(),
        })
}

pub fn row_for_disturbed_class(
    value: &str,
) -> Result<DisturbedRouteCoefficientRow, LanduseMigrationError> {
    let normalized = normalize_disturbed_class(value)?;
    let row = row_by_exact_class(normalized).ok_or_else(|| {
        LanduseMigrationError::UnknownDisturbedClass {
            disturbed_class: value.to_string(),
        }
    })?;
    validate_route_row(row)?;
    Ok(row)
}

#[must_use]
pub fn disturbed_route_table_checksum() -> String {
    let canonical = canonical_table_text();
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

fn canonical_table_text() -> String {
    let mut text = String::new();
    text.push_str(DISTURBED_ROUTE_TABLE_ID);
    text.push('\n');
    text.push_str(DISTURBED_ROUTE_TABLE_VERSION);
    text.push('\n');
    for row in ROUTE_ROWS {
        text.push_str(row.disturbed_class);
        text.push('|');
        text.push_str(&row.k_o.to_string());
        text.push('|');
        text.push_str(&row.form_c_d.to_string());
        text.push('|');
        text.push_str(&row.d_r_m.to_string());
        text.push('|');
        text.push_str(&row.lambda.to_string());
        text.push('|');
        text.push_str(&row.vegetation_c_d.to_string());
        text.push('\n');
    }
    text
}

fn row_by_exact_class(value: &str) -> Option<DisturbedRouteCoefficientRow> {
    ROUTE_ROWS
        .iter()
        .copied()
        .find(|row| row.disturbed_class == value)
}

fn validate_route_row(row: DisturbedRouteCoefficientRow) -> Result<(), LanduseMigrationError> {
    for (field, value) in [
        ("k_o", row.k_o),
        ("form_c_d", row.form_c_d),
        ("d_r_m", row.d_r_m),
        ("lambda", row.lambda),
        ("vegetation_c_d", row.vegetation_c_d),
    ] {
        if !value.is_finite() {
            return Err(LanduseMigrationError::InvalidRouteCoefficientRow {
                disturbed_class: row.disturbed_class.to_string(),
                detail: format!("{field} must be finite"),
            });
        }
    }
    if row.k_o <= 0.0 {
        return Err(LanduseMigrationError::InvalidRouteCoefficientRow {
            disturbed_class: row.disturbed_class.to_string(),
            detail: "k_o must be positive".to_string(),
        });
    }
    if row.form_c_d < 0.0 || row.d_r_m < 0.0 || row.vegetation_c_d < 0.0 {
        return Err(LanduseMigrationError::InvalidRouteCoefficientRow {
            disturbed_class: row.disturbed_class.to_string(),
            detail: "drag and roughness terms must be non-negative".to_string(),
        });
    }
    if !(0.0..=1.0).contains(&row.lambda) {
        return Err(LanduseMigrationError::InvalidRouteCoefficientRow {
            disturbed_class: row.disturbed_class.to_string(),
            detail: "lambda must be in 0..=1".to_string(),
        });
    }
    if (row.d_r_m == 0.0) != (row.lambda == 0.0) {
        return Err(LanduseMigrationError::InvalidRouteCoefficientRow {
            disturbed_class: row.disturbed_class.to_string(),
            detail: "d_r_m and lambda must both be zero or both positive".to_string(),
        });
    }
    Ok(())
}

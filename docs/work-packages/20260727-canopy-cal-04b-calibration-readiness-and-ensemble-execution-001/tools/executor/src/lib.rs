use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const CALIBRATION_MAGIC: &[u8; 8] = b"CAL04B03";
pub const HOLDOUT_MAGIC: &[u8; 8] = b"CAL04B02";
pub const RETAINED_START_YEAR: i32 = 1989;
pub const RETAINED_END_YEAR: i32 = 2024;
pub const DAYMET_LANE_COUNT: usize = 9;
pub const DAYMET_SOURCE_DAYS_PER_YEAR: usize = 365;
pub const CALIBRATION_DAYS_PER_YEAR: usize = 180;
pub const DAYMET_YEARS: usize = 36;
pub const DAYMET_SOURCE_DAYS_PER_LANE: usize = DAYMET_SOURCE_DAYS_PER_YEAR * DAYMET_YEARS;
pub const CALIBRATION_DAYS_PER_LANE: usize = CALIBRATION_DAYS_PER_YEAR * DAYMET_YEARS;
pub const HUBBARD_PLOT_IDS: [&str; DAYMET_LANE_COUNT] =
    ["1B", "4B", "4T", "5B", "5T", "6T", "7B", "7T", "HQ"];

#[derive(Clone)]
pub struct Config {
    pub id: String,
    pub configuration_id: String,
    pub values: [f64; 6],
    pub boundary: String,
    pub saturation: String,
}

#[derive(Clone, Copy)]
pub struct Climate {
    pub year: i32,
    pub ordinal: u16,
    pub tmin: f64,
    pub vpd: f64,
}

#[derive(Clone)]
pub struct DaymetLane {
    pub lane_index: usize,
    pub plot_id: String,
    pub latitude_degrees: f64,
    pub longitude_degrees: f64,
    pub forcing: Vec<Climate>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TraceHeader {
    pub candidate_count: usize,
    pub lane_count: usize,
    pub days_per_lane: usize,
}

impl TraceHeader {
    pub const BYTE_COUNT: u64 = 20;

    pub fn write(self, writer: &mut impl Write) -> Result<(), Box<dyn Error>> {
        writer.write_all(CALIBRATION_MAGIC)?;
        writer.write_all(
            &u32::try_from(self.candidate_count)
                .map_err(|_| "candidate count exceeds u32")?
                .to_le_bytes(),
        )?;
        writer.write_all(
            &u32::try_from(self.lane_count)
                .map_err(|_| "lane count exceeds u32")?
                .to_le_bytes(),
        )?;
        writer.write_all(
            &u32::try_from(self.days_per_lane)
                .map_err(|_| "day count exceeds u32")?
                .to_le_bytes(),
        )?;
        Ok(())
    }

    pub fn read(reader: &mut impl Read) -> Result<Self, Box<dyn Error>> {
        let mut magic = [0_u8; 8];
        reader.read_exact(&mut magic)?;
        if &magic != CALIBRATION_MAGIC {
            return Err("trace magic mismatch".into());
        }
        let mut four = [0_u8; 4];
        reader.read_exact(&mut four)?;
        let candidate_count = u32::from_le_bytes(four) as usize;
        reader.read_exact(&mut four)?;
        let lane_count = u32::from_le_bytes(four) as usize;
        reader.read_exact(&mut four)?;
        let days_per_lane = u32::from_le_bytes(four) as usize;
        Ok(Self {
            candidate_count,
            lane_count,
            days_per_lane,
        })
    }

    pub fn expected_bytes(self) -> Result<u64, Box<dyn Error>> {
        let values = u64::try_from(self.candidate_count)?
            .checked_mul(u64::try_from(self.lane_count)?)
            .and_then(|value| value.checked_mul(u64::try_from(self.days_per_lane).ok()?))
            .and_then(|value| value.checked_mul(8))
            .and_then(|value| value.checked_add(Self::BYTE_COUNT))
            .ok_or("trace size overflow")?;
        Ok(values)
    }
}

pub fn sha256(path: &Path) -> Result<String, Box<dyn Error>> {
    let output = Command::new("sha256sum").arg(path).output()?;
    if !output.status.success() {
        return Err(format!("sha256sum failed for {}", path.display()).into());
    }
    let text = String::from_utf8(output.stdout)?;
    Ok(text
        .split_whitespace()
        .next()
        .ok_or("missing sha256 output")?
        .to_string())
}

pub fn leap(year: i32) -> bool {
    year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
}

pub fn days_in_year(year: i32) -> u16 {
    if leap(year) {
        366
    } else {
        365
    }
}

pub fn ordinal(year: i32, month: usize, day: u16) -> Result<u16, Box<dyn Error>> {
    let days = [
        31_u16,
        if leap(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if !(1..=12).contains(&month) || day == 0 || day > days[month - 1] {
        return Err(format!("invalid date {year}-{month}-{day}").into());
    }
    Ok(days[..month - 1].iter().sum::<u16>() + day)
}

pub fn es(temperature_c: f64) -> f64 {
    0.6108 * (17.27 * temperature_c / (temperature_c + 237.3)).exp()
}

pub fn read_climate(path: &Path) -> Result<(f64, Vec<Climate>), Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let latitude: f64 = text
        .lines()
        .nth(4)
        .ok_or("missing climate identity line")?
        .split_whitespace()
        .next()
        .ok_or("missing latitude")?
        .parse()?;
    let mut rows = Vec::new();
    let mut daily_started = false;
    for (line_number, line) in text.lines().enumerate() {
        let fields: Vec<_> = line.split_whitespace().collect();
        if fields.len() != 13 {
            if daily_started && !fields.is_empty() {
                return Err(format!("malformed daily climate line {}", line_number + 1).into());
            }
            continue;
        }
        let parsed = fields[0]
            .parse::<u16>()
            .and_then(|day| fields[1].parse::<usize>().map(|month| (day, month)))
            .and_then(|(day, month)| fields[2].parse::<i32>().map(|year| (day, month, year)));
        let (day, month, year) = match parsed {
            Ok(value) => value,
            Err(_) if !daily_started => continue,
            Err(_) => {
                return Err(format!("malformed daily date line {}", line_number + 1).into());
            }
        };
        daily_started = true;
        let tmax: f64 = fields[7]
            .parse()
            .map_err(|_| format!("bad tmax line {}", line_number + 1))?;
        let tmin: f64 = fields[8]
            .parse()
            .map_err(|_| format!("bad tmin line {}", line_number + 1))?;
        let dewpoint: f64 = fields[12]
            .parse()
            .map_err(|_| format!("bad dewpoint line {}", line_number + 1))?;
        let vpd = (0.5 * (es(tmax) + es(tmin)) - es(dewpoint)) * 1000.0;
        if !vpd.is_finite() || vpd < 0.0 {
            return Err(format!("invalid VPD line {}", line_number + 1).into());
        }
        rows.push(Climate {
            year,
            ordinal: ordinal(year, month, day)?,
            tmin,
            vpd,
        });
    }
    if rows.is_empty() {
        return Err("empty climate".into());
    }
    for pair in rows.windows(2) {
        let expected = if pair[0].ordinal == days_in_year(pair[0].year) {
            (pair[0].year + 1, 1)
        } else {
            (pair[0].year, pair[0].ordinal + 1)
        };
        if (pair[1].year, pair[1].ordinal) != expected {
            return Err(format!(
                "nonconsecutive climate after {}-{}",
                pair[0].year, pair[0].ordinal
            )
            .into());
        }
    }
    Ok((latitude, rows))
}

pub fn read_configs(path: &Path) -> Result<Vec<Config>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if index == 0 || line.trim().is_empty() {
            continue;
        }
        let fields: Vec<_> = line.split(',').collect();
        if fields.len() != 12 {
            return Err(format!("config line {} has {} fields", index + 1, fields.len()).into());
        }
        let expected = format!("GSI-{:04}", index);
        if fields[0] != expected {
            return Err(format!("config order mismatch: {} != {expected}", fields[0]).into());
        }
        let mut values = [0.0; 6];
        for (target, source) in values.iter_mut().zip(&fields[4..10]) {
            *target = source.parse()?;
        }
        rows.push(Config {
            id: fields[0].to_string(),
            configuration_id: format!("{}|{}|{}", fields[1], fields[2], fields[3]),
            values,
            boundary: fields[10].to_string(),
            saturation: fields[11].to_string(),
        });
    }
    if rows.len() != 9261 {
        return Err(format!("expected 9261 configs, observed {}", rows.len()).into());
    }
    Ok(rows)
}

fn csv_rows(path: &Path) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let text = fs::read_to_string(path)?;
    Ok(text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split(',').map(str::to_string).collect())
        .collect())
}

fn authority_digest(
    authority_path: &Path,
    repository_root: &Path,
    input_id: &str,
    expected_path: &Path,
) -> Result<String, Box<dyn Error>> {
    let rows = csv_rows(authority_path)?;
    if rows.first().map(Vec::as_slice)
        != Some(
            [
                "input_id",
                "path",
                "role",
                "expected_sha256",
                "observed_sha256",
                "state",
            ]
            .map(str::to_string)
            .as_slice(),
        )
    {
        return Err("authority manifest header mismatch".into());
    }
    let matches = rows[1..]
        .iter()
        .filter(|row| row.first().map(String::as_str) == Some(input_id))
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(format!(
            "authority manifest has {} rows for {input_id}",
            matches.len()
        )
        .into());
    }
    let row = matches[0];
    if row.len() != 6 || row[3] != row[4] || row[5] != "PASS" {
        return Err(format!("authority manifest row {input_id} is not authenticated").into());
    }
    let recorded_path = Path::new(&row[1]);
    let recorded_path = if recorded_path.is_absolute() {
        PathBuf::from(recorded_path)
    } else {
        repository_root.join(recorded_path)
    };
    let recorded_canonical = fs::canonicalize(recorded_path)?;
    let expected_canonical = fs::canonicalize(expected_path)?;
    if !recorded_canonical.starts_with(repository_root)
        || !expected_canonical.starts_with(repository_root)
        || recorded_canonical != expected_canonical
    {
        return Err(format!("authority manifest path differs for {input_id}").into());
    }
    let observed = sha256(expected_path)?;
    if observed != row[3] {
        return Err(format!("working input digest differs for {input_id}").into());
    }
    Ok(observed)
}

fn validate_hubbard_lane_ids(ids: &[String]) -> Result<(), Box<dyn Error>> {
    if ids.len() != DAYMET_LANE_COUNT {
        return Err(format!("Hubbard lane count {} != {DAYMET_LANE_COUNT}", ids.len()).into());
    }
    for (index, (observed, expected)) in ids.iter().zip(HUBBARD_PLOT_IDS).enumerate() {
        if observed != expected {
            return Err(format!("Hubbard lane {index} identity {observed} != {expected}").into());
        }
    }
    Ok(())
}

pub fn read_authenticated_daymet(
    forcing_path: &Path,
    geometry_path: &Path,
    source_manifest_path: &Path,
    authority_path: &Path,
    authority_resolution_path: &Path,
) -> Result<Vec<DaymetLane>, Box<dyn Error>> {
    let source_manifest_canonical = fs::canonicalize(source_manifest_path)?;
    let repository_root = source_manifest_canonical
        .parent()
        .and_then(|parent| parent.ancestors().nth(4))
        .ok_or("source manifest is not under a repository docs/work-packages package")?;
    if !repository_root.join("docs/work-packages").is_dir()
        || !source_manifest_canonical.starts_with(repository_root)
    {
        return Err("source manifest repository root is invalid".into());
    }
    authority_digest(
        authority_path,
        repository_root,
        "daymet_derived",
        forcing_path,
    )?;
    authority_digest(
        authority_path,
        repository_root,
        "daymet_source_request_manifest",
        source_manifest_path,
    )?;
    authority_digest(
        authority_path,
        repository_root,
        "hubbard_plot_geometry",
        geometry_path,
    )?;
    authority_digest(
        authority_path,
        repository_root,
        "calibration_forcing_authority_resolution",
        authority_resolution_path,
    )?;

    let source_rows = csv_rows(source_manifest_path)?;
    let expected_source_header = [
        "source_id",
        "plot_id",
        "requested_latitude",
        "requested_longitude",
        "source_elevation_m",
        "daymet_grid_elevation_m",
        "years",
        "variables",
        "product",
        "doi",
        "path",
        "sha256",
        "state",
    ];
    if source_rows
        .first()
        .is_none_or(|row| row.iter().map(String::as_str).ne(expected_source_header))
    {
        return Err("Daymet source manifest header mismatch".into());
    }
    let mut source_order = Vec::new();
    let mut source_geometry = HashMap::new();
    for row in &source_rows[1..] {
        if row.len() != expected_source_header.len()
            || row[6] != "1989-2024"
            || row[7] != "tmax|tmin|vp|dayl"
            || row[8] != "Daymet V4 R1"
            || row[12] != "VERIFIED"
            || {
                let source_path = fs::canonicalize(repository_root.join(&row[10]))?;
                !source_path.starts_with(repository_root) || sha256(&source_path)? != row[11]
            }
        {
            return Err(format!("invalid Daymet source row for {:?}", row.get(1)).into());
        }
        let plot = row[1].clone();
        if source_geometry
            .insert(
                plot.clone(),
                (row[2].parse::<f64>()?, row[3].parse::<f64>()?),
            )
            .is_some()
        {
            return Err(format!("duplicate Daymet source plot {plot}").into());
        }
        source_order.push(plot);
    }
    validate_hubbard_lane_ids(&source_order)?;

    let geometry_rows = csv_rows(geometry_path)?;
    let expected_geometry_header = [
        "plot_id",
        "latitude_deg",
        "longitude_deg",
        "source_elevation_ft",
        "source_elevation_m",
        "geometry_authority",
        "role",
    ];
    if geometry_rows
        .first()
        .is_none_or(|row| row.iter().map(String::as_str).ne(expected_geometry_header))
    {
        return Err("Hubbard geometry header mismatch".into());
    }
    let mut geometry = HashMap::new();
    for row in &geometry_rows[1..] {
        if row.len() != expected_geometry_header.len()
            || row[5] != "knb-lter-hbr.51.16 EML"
            || row[6] != "calibration"
        {
            return Err("invalid Hubbard geometry row".into());
        }
        let coordinates = (row[1].parse::<f64>()?, row[2].parse::<f64>()?);
        if source_geometry.get(&row[0]) != Some(&coordinates) {
            return Err(format!("geometry/source coordinates differ for {}", row[0]).into());
        }
        if geometry.insert(row[0].clone(), coordinates).is_some() {
            return Err(format!("duplicate geometry plot {}", row[0]).into());
        }
    }
    if geometry.len() != DAYMET_LANE_COUNT {
        return Err(format!("expected {DAYMET_LANE_COUNT} geometry rows").into());
    }

    let forcing_rows = csv_rows(forcing_path)?;
    let expected_forcing_header = [
        "plot_id",
        "year",
        "yday",
        "date",
        "source_elevation_m",
        "daymet_grid_elevation_m",
        "elevation_error_m",
        "tmax_c",
        "tmin_c",
        "vp_pa",
        "derived_vpd_pa",
        "daymet_daylength_hours",
        "native_photoperiod_hours",
        "daylength_difference_minutes",
    ];
    if forcing_rows
        .first()
        .is_none_or(|row| row.iter().map(String::as_str).ne(expected_forcing_header))
    {
        return Err("Daymet derived forcing header mismatch".into());
    }
    let mut by_plot: BTreeMap<String, Vec<Climate>> = BTreeMap::new();
    for (number, row) in forcing_rows[1..].iter().enumerate() {
        if row.len() != expected_forcing_header.len() || !source_geometry.contains_key(&row[0]) {
            return Err(format!("invalid Daymet forcing row {}", number + 2).into());
        }
        let year: i32 = row[1].parse()?;
        let ordinal: u16 = row[2].parse()?;
        let tmax: f64 = row[7].parse()?;
        let tmin: f64 = row[8].parse()?;
        let vapor_pressure: f64 = row[9].parse()?;
        let vpd: f64 = row[10].parse()?;
        let independently_derived =
            (0.5 * (es(tmax) + es(tmin)) - vapor_pressure / 1000.0) * 1000.0;
        if !(1989..=2024).contains(&year)
            || !(1..=365).contains(&ordinal)
            || !tmin.is_finite()
            || !vpd.is_finite()
            || vpd < 0.0
            || (vpd - independently_derived).abs() > 0.000_001
        {
            return Err(format!("invalid Daymet forcing values at row {}", number + 2).into());
        }
        by_plot.entry(row[0].clone()).or_default().push(Climate {
            year,
            ordinal,
            tmin,
            vpd,
        });
    }

    let mut lanes = Vec::new();
    for (lane_index, plot_id) in source_order.into_iter().enumerate() {
        let forcing = by_plot
            .remove(&plot_id)
            .ok_or_else(|| format!("missing forcing plot {plot_id}"))?;
        if forcing.len() != DAYMET_SOURCE_DAYS_PER_LANE {
            return Err(format!(
                "plot {plot_id} has {} forcing rows, expected {DAYMET_SOURCE_DAYS_PER_LANE}",
                forcing.len()
            )
            .into());
        }
        for (year_index, year) in (1989..=2024).enumerate() {
            let start = year_index * DAYMET_SOURCE_DAYS_PER_YEAR;
            for (day_index, row) in forcing[start..start + DAYMET_SOURCE_DAYS_PER_YEAR]
                .iter()
                .enumerate()
            {
                if row.year != year || row.ordinal != day_index as u16 + 1 {
                    return Err(format!(
                        "plot {plot_id} calendar mismatch at {year}/{}",
                        day_index + 1
                    )
                    .into());
                }
            }
        }
        let (latitude_degrees, longitude_degrees) = geometry[&plot_id];
        lanes.push(DaymetLane {
            lane_index,
            plot_id,
            latitude_degrees,
            longitude_degrees,
            forcing,
        });
    }
    if !by_plot.is_empty() {
        return Err("forcing contains unadmitted plots".into());
    }
    Ok(lanes)
}

pub fn retained_calendar(rows: &[Climate]) -> Vec<Climate> {
    rows.iter()
        .copied()
        .filter(|row| (RETAINED_START_YEAR..=RETAINED_END_YEAR).contains(&row.year))
        .collect()
}

pub fn require_calendar_extent(
    rows: &[Climate],
    start: (i32, u16),
    end: (i32, u16),
) -> Result<(), Box<dyn Error>> {
    let observed_start = rows.first().map(|row| (row.year, row.ordinal));
    let observed_end = rows.last().map(|row| (row.year, row.ordinal));
    if observed_start != Some(start) || observed_end != Some(end) {
        return Err(format!(
            "climate extent {:?}..{:?} != {start:?}..{end:?}",
            observed_start, observed_end
        )
        .into());
    }
    let expected = (start.0..=end.0)
        .map(|year| {
            let lower = if year == start.0 { start.1 } else { 1 };
            let upper = if year == end.0 {
                end.1
            } else {
                days_in_year(year)
            };
            usize::from(upper - lower + 1)
        })
        .sum::<usize>();
    if rows.len() != expected {
        return Err(format!("climate row count {} != {expected}", rows.len()).into());
    }
    Ok(())
}

pub fn write_calendar(path: &Path, rows: &[Climate]) -> Result<(), Box<dyn Error>> {
    let mut text = String::from("year,ordinal\n");
    for row in rows {
        text.push_str(&format!("{},{}\n", row.year, row.ordinal));
    }
    fs::write(path, text)?;
    Ok(())
}

pub fn arg_value(args: &[String], name: &str) -> Result<String, Box<dyn Error>> {
    let position = args
        .iter()
        .position(|value| value == name)
        .ok_or_else(|| format!("missing {name}"))?;
    args.get(position + 1)
        .cloned()
        .ok_or_else(|| format!("missing value for {name}").into())
}

#[cfg(test)]
mod tests {
    use super::{
        days_in_year, es, leap, ordinal, read_authenticated_daymet, require_calendar_extent,
        validate_hubbard_lane_ids, Climate, CALIBRATION_MAGIC, DAYMET_LANE_COUNT,
        DAYMET_SOURCE_DAYS_PER_LANE, HOLDOUT_MAGIC,
    };
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn gregorian_leap_rules_and_ordinals_are_exact() {
        assert!(leap(2000));
        assert!(leap(2024));
        assert!(!leap(1900));
        assert!(!leap(2023));
        assert_eq!(days_in_year(2024), 366);
        assert_eq!(days_in_year(2023), 365);
        assert_eq!(ordinal(2024, 3, 1).ok(), Some(61));
        assert_eq!(ordinal(2023, 3, 1).ok(), Some(60));
        assert!(ordinal(2023, 2, 29).is_err());
    }

    #[test]
    fn vapor_pressure_curve_is_finite_and_monotone() {
        let cold = es(-20.0);
        let mild = es(0.0);
        let warm = es(20.0);
        assert!(cold.is_finite() && mild.is_finite() && warm.is_finite());
        assert!(0.0 < cold && cold < mild && mild < warm);
    }

    #[test]
    fn calendar_extent_counts_leap_days() {
        let rows = vec![
            Climate {
                year: 2024,
                ordinal: 365,
                tmin: 0.0,
                vpd: 1.0,
            },
            Climate {
                year: 2024,
                ordinal: 366,
                tmin: 0.0,
                vpd: 1.0,
            },
            Climate {
                year: 2025,
                ordinal: 1,
                tmin: 0.0,
                vpd: 1.0,
            },
        ];
        assert!(require_calendar_extent(&rows, (2024, 365), (2025, 1)).is_ok());
        assert!(require_calendar_extent(&rows[..2], (2024, 365), (2025, 1)).is_err());
    }

    #[test]
    fn hubbard_lane_identity_rejects_broadcast_swap_duplicate_and_missing() {
        let canonical = ["1B", "4B", "4T", "5B", "5T", "6T", "7B", "7T", "HQ"].map(str::to_string);
        assert!(validate_hubbard_lane_ids(&canonical).is_ok());

        let broadcast = ["1B"; 9].map(str::to_string);
        assert!(validate_hubbard_lane_ids(&broadcast).is_err());

        let mut swapped = canonical.clone();
        swapped.swap(0, 1);
        assert!(validate_hubbard_lane_ids(&swapped).is_err());

        let mut duplicate = canonical.clone();
        duplicate[8] = duplicate[7].clone();
        assert!(validate_hubbard_lane_ids(&duplicate).is_err());

        assert!(validate_hubbard_lane_ids(&canonical[..8]).is_err());
    }

    #[test]
    fn calibration_version_is_isolated_from_holdout_serialization() {
        assert_eq!(CALIBRATION_MAGIC, b"CAL04B03");
        assert_eq!(HOLDOUT_MAGIC, b"CAL04B02");
        assert_ne!(CALIBRATION_MAGIC, HOLDOUT_MAGIC);
    }

    #[test]
    fn checked_in_nine_plot_forcing_authenticates_and_retains_lane_identity() {
        let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.ancestors().nth(5).expect("repository root");
        let package = manifest.ancestors().nth(2).expect("package root");
        let predecessor = root.join(
            "docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/artifacts",
        );
        let lanes = read_authenticated_daymet(
            &predecessor.join("daymet-daily-derived.csv"),
            &predecessor.join("hubbard-plot-geometry.csv"),
            &predecessor.join("source-and-request-manifest.csv"),
            &package.join("artifacts/input-and-authority-manifest.csv"),
            &package.join("artifacts/calibration-forcing-authority-resolution.md"),
        )
        .expect("authenticated Daymet lanes");
        assert_eq!(lanes.len(), 9);
        assert_eq!(lanes[0].plot_id, "1B");
        assert_eq!(lanes[8].plot_id, "HQ");
        assert!(lanes
            .iter()
            .all(|lane| lane.forcing.len() == DAYMET_SOURCE_DAYS_PER_LANE));
    }

    #[test]
    fn source_manifest_anchors_repository_when_authority_is_externally_published() {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest_dir.ancestors().nth(5).expect("repository root");
        let package = manifest_dir.ancestors().nth(2).expect("package root");
        let predecessor = root.join(
            "docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/artifacts",
        );
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let external_root =
            std::env::temp_dir().join(format!("cal04b-external-authority-{unique}"));
        let external_artifacts =
            external_root.join("publication/docs/work-packages/cal04b/artifacts");
        fs::create_dir_all(&external_artifacts).expect("external authority directory");
        let external_authority = external_artifacts.join("input-and-authority-manifest.csv");
        fs::copy(
            package.join("artifacts/input-and-authority-manifest.csv"),
            &external_authority,
        )
        .expect("copy authority manifest");

        let lanes = read_authenticated_daymet(
            &predecessor.join("daymet-daily-derived.csv"),
            &predecessor.join("hubbard-plot-geometry.csv"),
            &predecessor.join("source-and-request-manifest.csv"),
            &external_authority,
            &package.join("artifacts/calibration-forcing-authority-resolution.md"),
        )
        .expect("external authority ledger must resolve repository inputs");
        assert_eq!(lanes.len(), DAYMET_LANE_COUNT);

        let authority_text =
            fs::read_to_string(&external_authority).expect("read external authority");
        let escaped = authority_text.replacen(
            "docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001/artifacts/daymet-daily-derived.csv",
            "../../../../etc/passwd",
            1,
        );
        fs::write(&external_authority, escaped).expect("write traversal mutation");
        assert!(read_authenticated_daymet(
            &predecessor.join("daymet-daily-derived.csv"),
            &predecessor.join("hubbard-plot-geometry.csv"),
            &predecessor.join("source-and-request-manifest.csv"),
            &external_authority,
            &package.join("artifacts/calibration-forcing-authority-resolution.md"),
        )
        .is_err());
        fs::remove_dir_all(&external_root).expect("remove bounded test directory");
    }
}

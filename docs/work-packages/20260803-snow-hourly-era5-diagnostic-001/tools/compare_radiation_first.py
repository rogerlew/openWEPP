#!/usr/bin/env python3
"""Execute the frozen radiation-first comparison and emit bound results."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

import numpy as np
import pandas as pd
import xarray as xr


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
ARTIFACTS = PACKAGE / "artifacts"
ACQUISITION = ARTIFACTS / "acquisition-manifest.json"
VALIDATED = ARTIFACTS / "validated-source-inventory.json"
PROTOCOL = ARTIFACTS / "radiation-comparison-manifest.json"
OUTPUT = ARTIFACTS / "radiation-first-results.json"
DATA = REPO / "target/snow_hourly_era5_diagnostic"
COMPARATOR_ROOT = REPO / "target/snow_surface_eb04w2b_terminal_frozen_w2a_rerun/runs"
NET_SHORTWAVE_FACTOR = 0.80
MJ_M2_PER_LANGLEY = 0.04184
COMPARATORS = {
    "snotel_mica_creek_st_joe_id": (
        "d819d1b6dae54d06cec0165f440add0fd8b61e3a3b81822f08304629cc5de47f"
    ),
    "snotel_paradise_wa": (
        "1b3f7a6a14568b59bd07540a80be3ed7300f87eb75d6927f3a0cf444bad2423d"
    ),
    "snotel_snowbird_ut": (
        "d925715d6e14ea3899a24fd64452d8a3fedc0334a6c8dba2b38b2d9a952539be"
    ),
    "snotel_niwot_co": (
        "b58fae64584eb29feba8e00eda013747631d861b041dc87db46dce9fdbf1cd6b"
    ),
}
CLIMATE_HASHES = {
    "snotel_mica_creek_st_joe_id": "e8470ae78711f85cc84045052467fa5d75fc8ec4ca1f92ce49b1af9ecf95fb63",
    "snotel_paradise_wa": "6e0c874e38825a7f4def18b87d81e61be9c59496a25e5f5affa9d25755db173c",
    "snotel_snowbird_ut": "10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7",
    "snotel_niwot_co": "841d6390b511c3b6ad613e166788fd0b3c48b1d83317779ecd7ba2cfd7916ead",
}


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def finite_float(value: float, label: str) -> float:
    result = float(value)
    if not np.isfinite(result):
        raise RuntimeError(f"nonfinite metric {label}")
    return result


def correlation(left: np.ndarray, right: np.ndarray, label: str) -> float:
    if left.size != right.size or left.size < 2 or np.std(left) == 0 or np.std(right) == 0:
        raise RuntimeError(f"undefined correlation {label}")
    return finite_float(np.corrcoef(left, right)[0, 1], label)


def metric_block(
    frame: pd.DataFrame,
    era_column: str,
    retained_column: str,
    mask: pd.Series,
    label: str,
) -> dict[str, float | int]:
    selected = frame.loc[mask, [era_column, retained_column]]
    era = selected[era_column].to_numpy(dtype=np.float64)
    retained = selected[retained_column].to_numpy(dtype=np.float64)
    difference = era - retained
    daily = selected.resample("1D").sum() * 0.0036
    daily_hours = selected.resample("1D").count()
    active = (
        (daily_hours[era_column] == 24)
        & (daily_hours[retained_column] == 24)
        & (daily[era_column] > 0.0)
        & (daily[retained_column] > 0.0)
    )
    daily = daily.loc[active]
    daily_difference = daily[era_column] - daily[retained_column]
    retained_total = daily[retained_column].sum()
    if retained_total <= 0.0:
        raise RuntimeError(f"nonpositive retained daily energy {label}")

    peak_source = frame.loc[mask, [era_column, retained_column]].copy()
    peak_days = peak_source.resample("1D").sum()
    peak_hours = peak_source.resample("1D").count()
    positive_days = (
        (peak_hours[era_column] == 24)
        & (peak_hours[retained_column] == 24)
        & (peak_days[era_column] > 0.0)
        & (peak_days[retained_column] > 0.0)
    )
    eligible_dates = peak_days.index[positive_days]
    peak_source = peak_source[peak_source.index.normalize().isin(eligible_dates)]
    era_peak = peak_source.groupby(peak_source.index.normalize())[era_column].idxmax().dt.hour
    retained_peak = peak_source.groupby(peak_source.index.normalize())[retained_column].idxmax().dt.hour
    peak_offset = ((era_peak - retained_peak + 12) % 24) - 12

    return {
        "hour_count": int(era.size),
        "hourly_correlation": correlation(era, retained, f"{label} hourly"),
        "hourly_mean_signed_error_w_m2": finite_float(difference.mean(), label),
        "hourly_mae_w_m2": finite_float(np.abs(difference).mean(), label),
        "daily_count": int(daily.shape[0]),
        "daily_energy_correlation": correlation(
            daily[era_column].to_numpy(), daily[retained_column].to_numpy(), f"{label} daily"
        ),
        "daily_energy_mean_signed_error_mj_m2_day": finite_float(daily_difference.mean(), label),
        "daily_energy_mae_mj_m2_day": finite_float(np.abs(daily_difference).mean(), label),
        "daily_energy_relative_bias_percent": finite_float(
            100.0 * daily_difference.sum() / retained_total, label
        ),
        "peak_day_count": int(peak_offset.size),
        "peak_signed_circular_offset_hours": finite_float(peak_offset.mean(), label),
        "peak_mean_absolute_circular_offset_hours": finite_float(np.abs(peak_offset).mean(), label),
        "peak_exact_hour_fraction": finite_float((peak_offset == 0).mean(), label),
    }


def daily_metric_block(
    era_daily_mj_m2: pd.Series,
    retained_daily_mj_m2: pd.Series,
    label: str,
) -> dict[str, float | int]:
    paired = pd.concat(
        [era_daily_mj_m2.rename("era"), retained_daily_mj_m2.rename("retained")], axis=1
    ).dropna()
    if paired.empty or (paired < 0.0).any().any():
        raise RuntimeError(f"invalid daily horizontal comparison {label}")
    difference = paired["era"] - paired["retained"]
    retained_total = paired["retained"].sum()
    if retained_total <= 0.0:
        raise RuntimeError(f"nonpositive retained horizontal energy {label}")
    return {
        "day_count": int(paired.shape[0]),
        "daily_energy_correlation": correlation(
            paired["era"].to_numpy(), paired["retained"].to_numpy(), label
        ),
        "daily_energy_mean_signed_error_mj_m2_day": finite_float(difference.mean(), label),
        "daily_energy_mae_mj_m2_day": finite_float(np.abs(difference).mean(), label),
        "daily_energy_relative_bias_percent": finite_float(
            100.0 * difference.sum() / retained_total, label
        ),
    }


def read_daily_climate(path: Path) -> pd.DataFrame:
    columns = [
        "day", "month", "year", "precip_mm", "duration_h", "tp", "ip",
        "tmax_c", "tmin_c", "radiation_langley_day", "wind_m_s", "wind_direction", "dewpoint_c",
    ]
    frame = pd.read_csv(path, sep=r"\s+", skiprows=15, header=None, names=columns)
    if frame.empty or frame.isna().any().any():
        raise RuntimeError(f"malformed retained daily climate {path}")
    frame.index = pd.to_datetime(frame[["year", "month", "day"]])
    if frame.index.has_duplicates or not frame.index.is_monotonic_increasing:
        raise RuntimeError(f"invalid retained daily climate chronology {path}")
    return frame


def main() -> int:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    acquisition = json.loads(ACQUISITION.read_text(encoding="utf-8"))
    project_by_site = {
        item["site_id"]: item for item in acquisition["project_elevation_sources"]
    }
    validated = json.loads(VALIDATED.read_text(encoding="utf-8"))
    protocol = json.loads(PROTOCOL.read_text(encoding="utf-8"))
    acquisition_hash = sha256(ACQUISITION)
    if validated.get("status") != "VALIDATED_COMPLETE" or validated.get("manifest_sha256") != acquisition_hash:
        raise RuntimeError("validated source receipt does not bind the acquisition manifest")
    validated_hashes = {item["path"]: item["sha256"] for item in validated["files"]}
    offsets = protocol["fixed_local_standard_utc_offsets_hours"]
    results = []
    for dataset in acquisition["datasets"]:
        dataset_id = dataset["dataset"]
        for site in acquisition["sites"]:
            site_id = site["site_id"]
            period = f'{site["start"][:4]}-{site["end"][:4]}'
            source_path = DATA / f"{dataset_id}__{site_id}__{period}.nc"
            if validated_hashes.get(str(source_path)) != sha256(source_path):
                raise RuntimeError(f"hourly source identity mismatch {source_path}")
            comparator_path = (
                COMPARATOR_ROOT / site_id / "legacy_coe/forcing_bridge/tg_neg2p5c_zg0p10m/forcing.csv"
            )
            if sha256(comparator_path) != COMPARATORS[site_id]:
                raise RuntimeError(f"retained comparator identity mismatch {comparator_path}")
            climate_path = REPO / f'tests/fixtures/snotel_observed/{site_id}/p{project_by_site[site_id]["wepp_id"]}.cli'
            if sha256(climate_path) != CLIMATE_HASHES[site_id]:
                raise RuntimeError(f"retained daily climate identity mismatch {climate_path}")
            climate = read_daily_climate(climate_path)
            comparator = pd.read_csv(
                comparator_path,
                usecols=["Datetime", "net_solar_Wm-2", "downwelling_thermal_Wm-2", "precip_mass_mm"],
                parse_dates=["Datetime"],
            ).set_index("Datetime")
            if comparator.index.has_duplicates or not comparator.index.is_monotonic_increasing:
                raise RuntimeError(f"invalid comparator chronology {comparator_path}")
            comparator = comparator.rename(
                columns={
                    "net_solar_Wm-2": "retained_net_shortwave_w_m2",
                    "downwelling_thermal_Wm-2": "retained_longwave_w_m2",
                    "precip_mass_mm": "retained_precip_mm",
                }
            )
            comparator["retained_shortwave_w_m2"] = (
                comparator["retained_net_shortwave_w_m2"] / NET_SHORTWAVE_FACTOR
            )
            with xr.open_dataset(source_path) as source:
                shortwave = source.ssrd.values.astype(np.float64)
                if (shortwave < -4.0).any():
                    raise RuntimeError(f"shortwave below admitted bound {source_path}")
                shortwave[shortwave < 0.0] = 0.0
                era = pd.DataFrame(
                    {
                        "era_shortwave_w_m2": shortwave / 3600.0,
                        "era_longwave_w_m2": source.strd.values.astype(np.float64) / 3600.0,
                    },
                    index=pd.DatetimeIndex(source.valid_time.values)
                    + pd.Timedelta(hours=int(offsets[site_id]) - 1),
                )
            frame = comparator.join(era, how="inner", validate="one_to_one")
            expected_overlap = comparator.index.intersection(era.index)
            if (
                not frame.index.equals(expected_overlap)
                or frame.shape[0] != comparator.shape[0] - abs(int(offsets[site_id]) - 1)
            ):
                raise RuntimeError(f"ERA/comparator local-standard overlap mismatch {dataset_id}:{site_id}")
            if not np.isfinite(frame.to_numpy()).all() or (frame < 0.0).any().any():
                raise RuntimeError(f"nonfinite/negative joined radiation or precipitation {dataset_id}:{site_id}")
            all_hours = pd.Series(True, index=frame.index)
            daily_precipitation = frame["retained_precip_mm"].resample("1D").sum()
            complete_days = frame["retained_precip_mm"].resample("1D").count() == 24
            winter_event_dates = daily_precipitation.loc[
                (daily_precipitation > 0.0)
                & daily_precipitation.index.month.isin([11, 12, 1, 2, 3])
                & complete_days
            ].index
            winter_event_hours = pd.Series(
                frame.index.normalize().isin(winter_event_dates), index=frame.index
            )
            era_horizontal_daily = (
                frame["era_shortwave_w_m2"].resample("1D").sum() * 0.0036
            ).loc[complete_days]
            retained_horizontal_daily = (
                climate["radiation_langley_day"] * MJ_M2_PER_LANGLEY
            ).reindex(era_horizontal_daily.index)
            results.append(
                {
                    "dataset": dataset_id,
                    "site_id": site_id,
                    "hourly_source_path": str(source_path),
                    "hourly_source_sha256": sha256(source_path),
                    "retained_comparator_path": str(comparator_path),
                    "retained_comparator_sha256": sha256(comparator_path),
                    "retained_daily_climate_path": str(climate_path),
                    "retained_daily_climate_sha256": sha256(climate_path),
                    "fixed_local_standard_utc_offset_hours": int(offsets[site_id]),
                    "era_interval_start_label_shift_hours": -1,
                    "overlap_hour_count": int(frame.shape[0]),
                    "comparator_boundary_hours_excluded": abs(int(offsets[site_id]) - 1),
                    "era_boundary_hours_excluded": abs(int(offsets[site_id]) - 1),
                    "winter_event_day_count": int(winter_event_dates.size),
                    "shortwave_all": metric_block(
                        frame, "era_shortwave_w_m2", "retained_shortwave_w_m2", all_hours,
                        f"{dataset_id}:{site_id}:shortwave_all",
                    ),
                    "longwave_all": metric_block(
                        frame, "era_longwave_w_m2", "retained_longwave_w_m2", all_hours,
                        f"{dataset_id}:{site_id}:longwave_all",
                    ),
                    "shortwave_winter_events": metric_block(
                        frame, "era_shortwave_w_m2", "retained_shortwave_w_m2", winter_event_hours,
                        f"{dataset_id}:{site_id}:shortwave_winter",
                    ),
                    "shortwave_horizontal_daily_all": daily_metric_block(
                        era_horizontal_daily,
                        retained_horizontal_daily,
                        f"{dataset_id}:{site_id}:shortwave_horizontal_daily_all",
                    ),
                    "shortwave_horizontal_daily_winter_events": daily_metric_block(
                        era_horizontal_daily.reindex(winter_event_dates),
                        retained_horizontal_daily.reindex(winter_event_dates),
                        f"{dataset_id}:{site_id}:shortwave_horizontal_daily_winter",
                    ),
                    "longwave_winter_events": metric_block(
                        frame, "era_longwave_w_m2", "retained_longwave_w_m2", winter_event_hours,
                        f"{dataset_id}:{site_id}:longwave_winter",
                    ),
                }
            )
    receipt = {
        "schema": "snow-hourly-era5-radiation-first-results-v1",
        "status": "RADIATION_FIRST_COMPLETE",
        "scientific_outcome": "DIVERGES",
        "acquisition_manifest_sha256": acquisition_hash,
        "validated_source_inventory_sha256": sha256(VALIDATED),
        "comparison_manifest_sha256": sha256(PROTOCOL),
        "shortwave_comparator_role": "primary daily horizontal comparison to retained Daymet/gridMET climate rad; geometry-confounded hourly chronology comparison to slope/aspect-transformed SIMIMPL28 synthesis",
        "longwave_comparator_role": "SIMIMPL28 diagnostic estimate; not observational authority",
        "precipitation_modified": False,
        "result_count": len(results),
        "results": results,
    }
    OUTPUT.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("RADIATION_FIRST_COMPLETE", len(results))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

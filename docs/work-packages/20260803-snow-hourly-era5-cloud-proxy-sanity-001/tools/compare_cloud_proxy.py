#!/usr/bin/env python3
"""Compare ERA5 cloud cover with the retained SIMIMPL daily cloud proxy."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path

import numpy as np
import pandas as pd
import xarray as xr


PACKAGE = Path(__file__).resolve().parents[1]
REPO = PACKAGE.parents[2]
PREDECESSOR = REPO / "docs/work-packages/20260803-snow-hourly-era5-diagnostic-001"
ARTIFACTS = PACKAGE / "artifacts"
MANIFEST = ARTIFACTS / "cloud-comparison-manifest.json"
OUTPUT = ARTIFACTS / "cloud-proxy-results.json"
ACQUISITION = PREDECESSOR / "artifacts/acquisition-manifest.json"
VALIDATED = PREDECESSOR / "artifacts/validated-source-inventory.json"
RADIATION_RESULTS = PREDECESSOR / "artifacts/radiation-first-results.json"
DATA = REPO / "target/snow_hourly_era5_diagnostic"
COMPARATOR_ROOT = REPO / "target/snow_surface_eb04w2b_terminal_frozen_w2a_rerun/runs"
SIGMA = 5.670_374_419e-8
MJ_M2_PER_LANGLEY = 0.04184
OFFSETS = {
    "snotel_mica_creek_st_joe_id": -8,
    "snotel_paradise_wa": -8,
    "snotel_snowbird_ut": -7,
    "snotel_niwot_co": -7,
}
COMPARATOR_HASHES = {
    "snotel_mica_creek_st_joe_id": "d819d1b6dae54d06cec0165f440add0fd8b61e3a3b81822f08304629cc5de47f",
    "snotel_paradise_wa": "1b3f7a6a14568b59bd07540a80be3ed7300f87eb75d6927f3a0cf444bad2423d",
    "snotel_snowbird_ut": "d925715d6e14ea3899a24fd64452d8a3fedc0334a6c8dba2b38b2d9a952539be",
    "snotel_niwot_co": "b58fae64584eb29feba8e00eda013747631d861b041dc87db46dce9fdbf1cd6b",
}
CLIMATE_HASHES = {
    "snotel_mica_creek_st_joe_id": "e8470ae78711f85cc84045052467fa5d75fc8ec4ca1f92ce49b1af9ecf95fb63",
    "snotel_paradise_wa": "6e0c874e38825a7f4def18b87d81e61be9c59496a25e5f5affa9d25755db173c",
    "snotel_snowbird_ut": "10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7",
    "snotel_niwot_co": "841d6390b511c3b6ad613e166788fd0b3c48b1d83317779ecd7ba2cfd7916ead",
}
CATEGORIES = ("clear", "mixed", "overcast")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def correlation(left: pd.Series, right: pd.Series, label: str) -> float:
    if len(left) != len(right) or len(left) < 2 or left.std() == 0 or right.std() == 0:
        raise RuntimeError(f"undefined correlation {label}")
    value = float(np.corrcoef(left.to_numpy(), right.to_numpy())[0, 1])
    if not np.isfinite(value):
        raise RuntimeError(f"nonfinite correlation {label}")
    return value


def category(values: pd.Series) -> pd.Series:
    codes = np.where(values < 0.25, "clear", np.where(values < 0.75, "mixed", "overcast"))
    return pd.Series(codes, index=values.index)


def metric_block(frame: pd.DataFrame, era_column: str, mask: pd.Series, label: str) -> dict[str, object]:
    selected = frame.loc[mask]
    era = selected[era_column]
    proxy = selected["simimpl_cloud_proxy"]
    delta = era - proxy
    era_category = category(era)
    proxy_category = category(proxy)
    confusion = {
        proxy_name: {era_name: int(((proxy_category == proxy_name) & (era_category == era_name)).sum()) for era_name in CATEGORIES}
        for proxy_name in CATEGORIES
    }
    return {
        "day_count": int(len(selected)),
        "era_mean_cloud_fraction": float(era.mean()),
        "simimpl_mean_cloud_fraction": float(proxy.mean()),
        "correlation": correlation(era, proxy, f"{label}:cloud"),
        "era_minus_proxy_mean_signed_error": float(delta.mean()),
        "mae": float(delta.abs().mean()),
        "category_exact_agreement_fraction": float((era_category == proxy_category).mean()),
        "category_confusion_proxy_rows_era_columns": confusion,
        "cloud_residual_vs_shortwave_residual_correlation": correlation(
            delta, selected["shortwave_residual_mj_m2_day"], f"{label}:cloud-shortwave"
        ),
    }


def read_daily_climate(path: Path) -> pd.DataFrame:
    columns = ["day", "month", "year", "precip_mm", "duration_h", "tp", "ip", "tmax_c", "tmin_c", "radiation_langley_day", "wind_m_s", "wind_direction", "dewpoint_c"]
    frame = pd.read_csv(path, sep=r"\s+", skiprows=15, header=None, names=columns)
    if frame.empty or frame.isna().any().any():
        raise RuntimeError(f"malformed climate file {path}")
    frame.index = pd.to_datetime(frame[["year", "month", "day"]])
    return frame


def main() -> int:
    if OUTPUT.exists():
        raise RuntimeError(f"refusing to overwrite {OUTPUT}")
    acquisition = json.loads(ACQUISITION.read_text(encoding="utf-8"))
    validated = json.loads(VALIDATED.read_text(encoding="utf-8"))
    radiation = json.loads(RADIATION_RESULTS.read_text(encoding="utf-8"))
    project_by_site = {item["site_id"]: item for item in acquisition["project_elevation_sources"]}
    validated_hashes = {item["path"]: item["sha256"] for item in validated["files"]}
    radiation_comparators = {item["site_id"]: item["retained_comparator_sha256"] for item in radiation["results"] if item["dataset"] == "reanalysis-era5-single-levels-timeseries"}
    results = []
    for site in acquisition["sites"]:
        site_id = site["site_id"]
        period = f'{site["start"][:4]}-{site["end"][:4]}'
        source_path = DATA / f"reanalysis-era5-single-levels-timeseries__{site_id}__{period}.nc"
        if validated_hashes.get(str(source_path)) != sha256(source_path):
            raise RuntimeError(f"ERA5 identity mismatch {source_path}")
        comparator_path = COMPARATOR_ROOT / site_id / "legacy_coe/forcing_bridge/tg_neg2p5c_zg0p10m/forcing.csv"
        if sha256(comparator_path) != COMPARATOR_HASHES[site_id] or radiation_comparators.get(site_id) != COMPARATOR_HASHES[site_id]:
            raise RuntimeError(f"comparator identity mismatch {site_id}")
        climate_path = REPO / f'tests/fixtures/snotel_observed/{site_id}/p{project_by_site[site_id]["wepp_id"]}.cli'
        if sha256(climate_path) != CLIMATE_HASHES[site_id]:
            raise RuntimeError(f"climate identity mismatch {site_id}")
        comparator = pd.read_csv(comparator_path, usecols=["Datetime", "downwelling_thermal_Wm-2", "temp_air_degC", "precip_mass_mm"], parse_dates=["Datetime"]).set_index("Datetime")
        if comparator.index.has_duplicates or not comparator.index.is_monotonic_increasing:
            raise RuntimeError(f"invalid comparator chronology {site_id}")
        emissivity = comparator["downwelling_thermal_Wm-2"] / (SIGMA * (comparator["temp_air_degC"] + 273.15) ** 4)
        proxy = (emissivity - 0.72) / 0.28
        if (proxy < -1e-10).any() or (proxy > 1.0 + 1e-10).any():
            raise RuntimeError(f"reconstructed proxy outside [0,1] {site_id}")
        comparator["simimpl_cloud_proxy"] = proxy.clip(0.0, 1.0)
        daily_proxy_range = comparator["simimpl_cloud_proxy"].resample("1D").agg(lambda values: values.max() - values.min())
        if (daily_proxy_range > 1e-10).any():
            raise RuntimeError(f"SIMIMPL proxy not daily constant {site_id}")
        daily_comparator = comparator.resample("1D").agg({"simimpl_cloud_proxy": "mean", "precip_mass_mm": "sum"})
        comparator_counts = comparator["simimpl_cloud_proxy"].resample("1D").count()
        climate = read_daily_climate(climate_path)
        with xr.open_dataset(source_path) as source:
            index = pd.DatetimeIndex(source.valid_time.values) + pd.Timedelta(hours=OFFSETS[site_id])
            shortwave = source.ssrd.values.astype(np.float64)
            if (shortwave < -4.0).any():
                raise RuntimeError(f"shortwave below admitted bound {site_id}")
            shortwave[shortwave < 0.0] = 0.0
            hourly = pd.DataFrame({"tcc": source.tcc.values.astype(np.float64), "ssrd_j_m2": shortwave}, index=index)
        if not np.isfinite(hourly.to_numpy()).all() or not hourly["tcc"].between(0.0, 1.0).all():
            raise RuntimeError(f"invalid ERA5 cloud/shortwave domain {site_id}")
        if (hourly.loc[hourly.index.hour == 0, "ssrd_j_m2"] != 0.0).any():
            raise RuntimeError(f"nonzero local-midnight preceding-hour ssrd sensitivity weight {site_id}")
        counts = hourly["tcc"].resample("1D").count()
        complete = counts[(counts == 24) & (comparator_counts.reindex(counts.index) == 24)].index
        daily = pd.DataFrame(index=complete)
        daily["era_24h_mean_cloud"] = hourly["tcc"].resample("1D").mean().reindex(complete)
        weighted_numerator = (hourly["tcc"] * hourly["ssrd_j_m2"]).resample("1D").sum()
        weighted_denominator = hourly["ssrd_j_m2"].resample("1D").sum()
        if (weighted_denominator.reindex(complete) <= 0.0).any():
            raise RuntimeError(f"nonpositive daily shortwave weights {site_id}")
        daily["era_shortwave_weighted_cloud"] = (weighted_numerator / weighted_denominator).reindex(complete)
        daily["era_shortwave_mj_m2_day"] = weighted_denominator.reindex(complete) / 1e6
        daily = daily.join(daily_comparator, how="inner")
        daily["retained_shortwave_mj_m2_day"] = (climate["radiation_langley_day"] * MJ_M2_PER_LANGLEY).reindex(daily.index)
        daily["shortwave_residual_mj_m2_day"] = daily["era_shortwave_mj_m2_day"] - daily["retained_shortwave_mj_m2_day"]
        if daily.isna().any().any() or not np.isfinite(daily.to_numpy()).all():
            raise RuntimeError(f"invalid joined daily cloud frame {site_id}")
        all_days = pd.Series(True, index=daily.index)
        winter_wet = pd.Series((daily.index.month.isin([11, 12, 1, 2, 3])) & (daily["precip_mass_mm"] > 0.0), index=daily.index)
        results.append({
            "site_id": site_id,
            "era5_path": str(source_path),
            "era5_sha256": sha256(source_path),
            "comparator_path": str(comparator_path),
            "comparator_sha256": sha256(comparator_path),
            "climate_path": str(climate_path),
            "climate_sha256": sha256(climate_path),
            "complete_day_count": int(len(daily)),
            "winter_wet_day_count": int(winter_wet.sum()),
            "proxy_daily_max_range": float(daily_proxy_range.max()),
            "all_24h_mean": metric_block(daily, "era_24h_mean_cloud", all_days, f"{site_id}:all:24h"),
            "all_shortwave_weighted": metric_block(daily, "era_shortwave_weighted_cloud", all_days, f"{site_id}:all:weighted"),
            "winter_wet_24h_mean": metric_block(daily, "era_24h_mean_cloud", winter_wet, f"{site_id}:winter:24h"),
            "winter_wet_shortwave_weighted": metric_block(daily, "era_shortwave_weighted_cloud", winter_wet, f"{site_id}:winter:weighted"),
        })
    receipt = {
        "schema": "snow-hourly-era5-cloud-proxy-results-v1",
        "status": "CLOUD_PROXY_SANITY_COMPLETE",
        "scientific_outcome": "DIVERGES",
        "comparison_manifest_sha256": sha256(MANIFEST),
        "acquisition_manifest_sha256": sha256(ACQUISITION),
        "validated_source_inventory_sha256": sha256(VALIDATED),
        "radiation_first_results_sha256": sha256(RADIATION_RESULTS),
        "precipitation_modified": False,
        "primary_cloud_operator": "24-hour arithmetic mean ERA5 tcc",
        "realized_ssrd_weighted_operator_role": "outcome-dependent sensitivity; mathematically coupled to shortwave residual",
        "result_count": len(results),
        "results": results,
    }
    OUTPUT.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print("CLOUD_PROXY_SANITY_COMPLETE", len(results))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

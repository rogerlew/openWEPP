#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
package_dir="$repo_root/docs/work-packages/20260726-canopy-cal-04a-best-available-evidence-daymet-001"
geometry="$package_dir/artifacts/hubbard-plot-geometry.csv"
destination="$repo_root/references/canopy_phenology/daymet_calibration/raw"
years=$(seq -s, 1989 2024)

mkdir -p "$destination"

while IFS=, read -r plot latitude longitude _rest; do
    if [[ "$plot" == "plot_id" ]]; then
        continue
    fi
    url="https://daymet.ornl.gov/single-pixel/api/data?lat=${latitude}&lon=${longitude}&vars=tmax,tmin,vp,dayl&years=${years}"
    output="$destination/hubbard_${plot}_daymet_v4r1_1989_2024.csv"
    curl --fail --silent --show-error --location --retry 2 \
        --connect-timeout 30 --max-time 300 \
        "$url" --output "$output"
    printf '%s,%s\n' "$plot" "$url"
done < "$geometry"

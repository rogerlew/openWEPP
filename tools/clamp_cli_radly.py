#!/usr/bin/env python3
"""Clamp CLIGEN `.cli` daily solar radiation (`rad`, Langleys/day) to the
openWEPP sunmap horizontal daily potential.

openWEPP's hourly-forcing sunmap rejects a run when a day's `rad` exceeds the
baseline horizontal daily extraterrestrial potential `r3`
(`CLIM-RUNTIME-E-017: radly ... out of domain (0 <= radly <= rpoth/r3)`). Older
CLIGEN outputs (produced before the generator clamp) can emit an implausibly
high `rad` on a low-sun day. This tool applies the same clamp offline so such a
climate file runs: for each day it recomputes `r3` from day-of-year + station
latitude and caps `rad` at `floor(r3)`. Compliant days are left byte-identical.

The `r3` formula and the leap-aware day-of-year are a faithful port of
`crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
(`simimpl28_sunmap` r3; `simimpl28_day_of_year`; `SIMIMPL28_SUNMAP_SOLCON = 1.94`).

Usage:
    python3 tools/clamp_cli_radly.py <file.cli> [more.cli ...] [--in-place]
    python3 tools/clamp_cli_radly.py in.cli --output out.cli
    python3 tools/clamp_cli_radly.py *.cli --in-place --quiet
"""

from __future__ import annotations

import argparse
import datetime
import math
import re
import sys
from pathlib import Path

SOLCON = 1.94  # SIMIMPL28_SUNMAP_SOLCON (Langleys / minute)
DAY_ANGLE = 0.0172  # ~2*pi/365 day-angle factor used by the sunmap
RAD_FIELD_INDEX = 9  # da mo year prcp dur tp ip tmax tmin [rad] w-vl w-dir tdew
DAILY_FIELD_COUNT = 13
NUMBER_RE = re.compile(r"-?\d+(?:\.\d+)?")
HEADER_RE = re.compile(r"^\s*da\s+mo\s+year\b")


def daily_horizontal_potential_langleys(day_of_year: int, latitude_deg: float) -> float:
    """openWEPP `simimpl28_sunmap` `r3` — daily horizontal extraterrestrial
    radiation potential (Langleys/day) for a given ordinal day and latitude."""
    radlat = math.radians(latitude_deg)
    s = float(day_of_year)
    declination = 0.00698 - 0.4067 * math.cos((s + 10.0) * DAY_ANGLE)
    eccentricity = 1.0 - 0.0167 * math.cos((s - 3.0) * DAY_ANGLE)
    r1 = (60.0 * SOLCON) / (eccentricity * eccentricity)
    x = max(-1.0, min(1.0, -(math.tan(radlat) * math.tan(declination))))
    t = math.acos(x)
    t1, t0 = t, -t
    twelve_over_pi = 12.0 / math.pi
    return r1 * (
        math.sin(declination) * math.sin(radlat) * (t1 - t0) * twelve_over_pi
        + math.cos(declination) * math.cos(radlat) * (math.sin(t1) - math.sin(t0)) * twelve_over_pi
    )


def parse_station_latitude(lines: list[str]) -> float:
    """The station latitude is the first token on the line following the
    'Latitude Longitude Elevation ...' header row."""
    for idx, line in enumerate(lines):
        if "Latitude" in line and "Longitude" in line and idx + 1 < len(lines):
            token = lines[idx + 1].split()[0]
            return float(token)
    raise ValueError("could not locate the 'Latitude Longitude ...' header row")


def daily_data_start(lines: list[str]) -> int:
    """Index of the first daily data row (2 lines past the 'da mo year ...'
    column header: the header line and the '(mm) (h) ...' units line)."""
    for idx, line in enumerate(lines):
        if HEADER_RE.match(line):
            return idx + 2
    raise ValueError("could not locate the 'da mo year ...' daily header row")


def clamp_cli_text(text: str, quiet: bool, source_name: str) -> tuple[str, int, float]:
    """Return (clamped_text, days_clamped, max_reduction_langleys)."""
    newline = "\r\n" if "\r\n" in text else "\n"
    trailing_newline = text.endswith(("\n", "\r"))
    lines = text.splitlines()

    latitude = parse_station_latitude(lines)
    start = daily_data_start(lines)

    days_clamped = 0
    max_reduction = 0.0
    for i in range(start, len(lines)):
        line = lines[i]
        if not line.strip():
            continue
        numbers = list(NUMBER_RE.finditer(line))
        if len(numbers) != DAILY_FIELD_COUNT:
            if not quiet:
                print(
                    f"{source_name}: skipping non-standard daily row {i + 1} "
                    f"({len(numbers)} fields, expected {DAILY_FIELD_COUNT})",
                    file=sys.stderr,
                )
            continue

        day = int(numbers[0].group())
        mon = int(numbers[1].group())
        year = int(numbers[2].group())
        rad = int(round(float(numbers[RAD_FIELD_INDEX].group())))
        day_of_year = datetime.date(year, mon, day).timetuple().tm_yday
        potential = daily_horizontal_potential_langleys(day_of_year, latitude)
        cap = math.floor(potential)  # floor(r3) <= r3, so radly <= r3 holds

        if rad > cap:
            field_start = numbers[RAD_FIELD_INDEX - 1].end()
            field_end = numbers[RAD_FIELD_INDEX].end()
            field = line[field_start:field_end]  # leading whitespace + rad
            lines[i] = line[:field_start] + str(cap).rjust(len(field)) + line[field_end:]
            days_clamped += 1
            max_reduction = max(max_reduction, float(rad - cap))

    out = newline.join(lines)
    if trailing_newline:
        out += newline
    return out, days_clamped, max_reduction


def process(path: Path, in_place: bool, output: Path | None, quiet: bool) -> int:
    text = path.read_text()
    clamped, days, max_reduction = clamp_cli_text(text, quiet, path.name)
    if in_place:
        destination = path
    elif output is not None:
        destination = output
    else:
        destination = None

    if destination is not None:
        destination.write_text(clamped)

    if not quiet:
        target = destination if destination is not None else "(dry run, not written)"
        print(
            f"{path}: clamped {days} day(s) to floor(r3); "
            f"max reduction {max_reduction:.0f} Langleys -> {target}"
        )
    return days


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("cli_files", nargs="+", type=Path, help="CLIGEN .cli file(s)")
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--in-place", action="store_true", help="rewrite each file in place")
    group.add_argument("--output", type=Path, help="write the (single) clamped file here")
    parser.add_argument("--quiet", action="store_true", help="suppress the per-file summary")
    args = parser.parse_args(argv)

    if args.output is not None and len(args.cli_files) != 1:
        parser.error("--output takes exactly one input .cli file")

    total = 0
    for path in args.cli_files:
        total += process(path, args.in_place, args.output, args.quiet)
    if not args.quiet:
        print(f"total days clamped: {total}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

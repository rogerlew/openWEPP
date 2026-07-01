pub(crate) fn saturation_vapor_pressure_kpa(temperature_c: f64) -> f64 {
    0.6108 * (17.27 * temperature_c / (temperature_c + 237.3)).exp()
}

pub(crate) fn legacy_sunmap_horizontal_radpot_ly(deglat: f64, sdate: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let radlat = deglat * pi / 180.0;
    let declination = 0.00698 - 0.4067 * ((sdate + 10.0) * 0.0172).cos();
    let earth_sun_distance_factor = 1.0 - 0.0167 * ((sdate - 3.0) * 0.0172).cos();
    let radiation_factor = (60.0 * 1.94) / (earth_sun_distance_factor * earth_sun_distance_factor);
    let sunset_argument = -(radlat.tan() * declination.tan()).clamp(-1.0, 1.0);
    let sunset_angle = sunset_argument.acos();
    radiation_factor
        * ((declination.sin() * radlat.sin() * (sunset_angle - -sunset_angle) * 12.0 / pi)
            + (declination.cos()
                * radlat.cos()
                * (sunset_angle.sin() - (-sunset_angle).sin())
                * 12.0
                / pi))
}

// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    // unimplemented!("calculate hourly production rate at speed: {}", speed)
    let float_speed = speed as f64;
    if speed == 0 {
        return 0.0;
    } else if (speed >= 1 && speed <= 4 ) {
        return 221.0 * float_speed * 1.;
    } else if (speed >= 5 && speed <= 8) {
        return 221.0 * float_speed * 0.9;
    } else {
        return 221.0 * float_speed * 0.77;
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    // unimplemented!("calculate the amount of working items at speed: {}", speed)
    let rate_per_hour = production_rate_per_hour(speed) / 60.;
    let int_per_minute = rate_per_hour as u32;
    return int_per_minute;
}

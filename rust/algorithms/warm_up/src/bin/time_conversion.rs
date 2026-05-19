fn time_conversion(s: &str) -> String {
    let is_pm = s.ends_with("PM");
    let hour = &s[0..2];
    let rest_of_time = &s[2..8];
    let hour_num = hour.parse().unwrap_or(0);
    let new_hour = match (is_pm, hour_num) {
        (true, 12) => 12,
        (true, h) => h + 12,
        (false, 12) => 0,
        (false, h) => h,
    };
    format!("{:02}{}", new_hour, rest_of_time)
}

fn main() {
    let time = "12:01:00PM";
    time_conversion(time);
}

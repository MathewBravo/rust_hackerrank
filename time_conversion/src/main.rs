fn timeConversion(s: &str) -> String {
    let mut is_am: bool = false;
    let mut hour_time: &str = "";
    let mut time_as_int: i32 = 0;
    if s.len() > 2 {
        hour_time = &s[..2];
        time_as_int = hour_time.parse().unwrap();
    }
    for c in s.chars() {
        if c == 'A' {
            is_am = true;
        }
    }
    if is_am && (time_as_int == 12) {
        time_as_int -= 12
    } else if !is_am && time_as_int != 12 {
        time_as_int += 12
    }
    let mut hour_as_string = time_as_int.to_string();
    if hour_as_string.len() == 1 {
        hour_as_string = format!("0{}", hour_as_string);
    }
    let mut finalstring = s.replace(hour_time, &hour_as_string);
    finalstring[..finalstring.len() - 2].to_string()
}
fn main() {
    let time = "06:40:03AM";
    let result = timeConversion(time);
    println!("{}", result)
}

use std::process::Command;
use std::env;
use std::cmp::Ordering;

fn hourify(hour: String) -> String {
    return String::from(match hour.as_ref() {
        "" => "midnight",
        "twelve" => "noon",
        _ => hour.as_ref()
    });
}

fn pretty_num(num: i8) -> String {
    match num {
        10 => return String::from("ten"),
        11 => return String::from("eleven"),
        12 => return String::from("twelve"),
        13 => return String::from("thirteen"),
        15 => return String::from("fifteen"),
        _ => ()
    };

    let tens = num / 10;
    let ones = num % 10;
    let tens_effect: Box<Fn(String) -> String> = match tens {
        0 => Box::new( |x| x ),
        1 => Box::new( |x| format!("{}teen", x) ),
        2 => Box::new( |x| format!("twenty {}", x) ),
        3 => Box::new( |x| format!("thirty {}", x) ),
        4 => Box::new( |x| format!("forty {}", x) ),
        5 => Box::new( |x| format!("fifty {}", x) ),
        6 => Box::new( |x| format!("sixty {}", x) ),
        7 => Box::new( |x| format!("seventy {}", x) ),
        8 => Box::new( |x| format!("eighty {}", x) ),
        9 => Box::new( |x| format!("ninety {}", x) ),
        _ => Box::new( |x| x )
    };
    if ones == 0 {
        return String::from(tens_effect(String::new()).trim());
    }

    let ones = match ones {
        0 => String::from(""),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        _ => String::new()
    };

    return tens_effect(ones);
}

fn fuzzy_clock(fuzziness: u8) -> String {
    assert!(fuzziness == 1 as u8);

    let time = Command::new("sh")
        .arg("-c")
        .arg("date '+%-H:%-M'")
        .output()
        .expect("Failed to read date");
    let time: String = String::from(String::from_utf8(time.stdout).expect("Failed to read time").trim());
    let mut time = time.split(':');
    let mut hour: i8 = time.next().unwrap().parse().expect("Failed to parse hour");
    let mut minute: i8 = time.next().unwrap().parse().expect("Failed to parse minute");
    minute = (minute as f32 / 5.0).round() as i8 * 5;

    let relation = match minute.cmp(&30) {
        Ordering::Greater => "to",
        _ => "past"
    };

    if relation == "to" {
        hour = (hour + 1) % 24;
        minute = 60 - minute;
    }
    let small_hour = hour % 13 + hour / 13;  // 12 hours, with separate noon and midnight

    return match (hour, minute) {
        (0, 0) => String::from("midnight"),
        (12, 0) => String::from("noon"),
        (_, 0) => format!("{} o'clock", pretty_num(small_hour)),
        (_, 15) => format!("quarter {} {}", relation, hourify(pretty_num(small_hour))),
        (_, 30) => format!("half past {}", hourify(pretty_num(small_hour))),
        _ => format!("{} {} {}", pretty_num(minute), relation, hourify(pretty_num(small_hour)))
    };
}

fn main() {
    let mut args = env::args();
    let bin_name = args.next().unwrap();
    let fuzziness: u8 = args.next()
        .unwrap_or(String::from("1"))
        .parse()
        .expect(format!("Usage: {} <fuzziness>", bin_name).as_ref());
    println!("{}", fuzzy_clock(fuzziness));
}

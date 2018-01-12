use std::process::Command;
use std::env;

fn full_clock<'a>(locale_: &'a str, date_fmt_: &'a str) -> String {
    let locale = match locale_ {
        "" => "ja_JP.UTF-8",
        x => x
    };
    let date_fmt = match date_fmt_ {
        "" => "+%d %A %H時%M分",
        x => x
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("LC_TIME='{}' date '{}'", locale, date_fmt))
        .output()
        .expect("Failed to read date");
    let stdout = String::from_utf8(output.stdout).expect("Failed to parse date");
    return stdout;
}

fn main() {
    let default_date_fmt: String = String::from("+%d %A %H時%M分");
    let default_locale: String = String::from("ja_JP.UTF-8");
    let mut args = env::args();
    args.next(); // Skip over the binary's name

    let date_fmt: String = args.next().unwrap_or(default_date_fmt);
    let locale: String = default_locale;
    println!("{}", full_clock(&locale, &date_fmt));
}

use clap::Arg;
use std::process::Command;

fn main() {
    let matches = clap::App::new("Set brightness")
        .about("Tool to set the screen brightness on arch with intel_brightness")
        .author("Felix Kaasa")
        .version("0.1")
        .arg(
            Arg::with_name("Percentage")
                .value_name("PERCENTAGE")
                .help("Number between 1-100")
                .required(true),
        )
        .get_matches();

    let percentage = match matches.value_of("Percentage").unwrap().parse::<u8>() {
        Ok(val) => val,
        Err(_) => {
            println!("ERROR: Could not parse input as an integer between 1-100");
            std::process::exit(1)
        }
    };
    if percentage > 100 {
        println!("ERROR: Percentage can not be over 100");
        std::process::exit(2);
    }
    println!("{}", percentage);
    println!("{:?}", matches);
    set_brightness(percentage.into());
    std::process::exit(0);
}

fn set_brightness(percentage: u32) {
    //TODO check if the brightness is higher than what we want to set it to before setting it.
    let max_pwr: f32 = 96000.0;
    let percentage: f32 = percentage as f32 / 100.0;
    let pwr: u32 = (percentage * max_pwr) as u32;
    let command = format!(
        "echo {} > /sys/class/backlight/intel_backlight/brightness",
        pwr
    );

    let output = Command::new("sh").args(["-c", &command]).output();
    match output {
        Ok(_) => (),
        Err(_) => std::process::exit(3),
    };
}

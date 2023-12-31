use clap::Arg;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let matches = clap::App::new("Set brightness")
        .about("Tool to set the screen brightness on arch with intel_brightness")
        .author("Felix Kaasa")
        .version("1.1")
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
            eprintln!("ERROR: Could not parse input as an integer between 1-100");
            std::process::exit(1)
        }
    };
    if percentage > 100 {
        eprintln!("ERROR: Percentage can not be over 100");
        std::process::exit(2);
    }

    let max_pwr = match get_max_power() {
        Ok(max) => max,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(4);
        }
    };
    let percentage: f32 = percentage as f32 / 100.0;
    let pwr: u32 = (percentage * max_pwr as f32) as u32;
    set_brightness(pwr);

    std::process::exit(0);
}

fn set_brightness(pwr: u32) {
    if let Ok(base_path) = get_base_path() {
        let command = format!("echo {} > {}", pwr, base_path.join("brightness").display());

        let output = Command::new("sh").args(["-c", &command]).output();
        match output {
            Ok(_) => (),
            Err(e) => {
                eprintln!("ERROR: could not write to the brightness file");
                eprintln!("{}", e);
                std::process::exit(3)
            }
        };
    }
}

fn get_max_power() -> Result<u32, Box<dyn std::error::Error>> {
    let max_power: u32 = fs::read_to_string(get_base_path()?.join("max_brightness"))?
        .trim()
        .parse()?;
    Ok(max_power)
}

fn get_base_path() -> Result<PathBuf, std::io::Error> {
    let path = PathBuf::from("/sys/class/backlight");
    for dir in path.read_dir()? {
        let dir = dir?;
        let metadata = std::fs::metadata(dir.path())?;
        if metadata.is_dir() {
            let dir_content = std::fs::read_dir(dir.path())?;
            for file in dir_content.into_iter() {
                let file = file?;
                if file.file_name() == "brightness" {
                    return Ok(file
                        .path()
                        .parent()
                        .expect("this path should always have a parent")
                        .to_path_buf());
                }
            }
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "No folder contained brightness",
    ))
}

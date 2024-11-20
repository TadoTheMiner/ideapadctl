use clap::{Parser, ValueEnum};
use color_eyre::{eyre::eyre, owo_colors::OwoColorize, Result};
use std::{fmt::Display, fs, path::PathBuf};

const PATH: &str = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/";
#[derive(Parser)]
struct Cli {
    #[arg(index = 1)]
    setting: Setting,
    #[arg(index = 2, default_value_t = Action::Get)]
    action: Action,
    #[arg(long, short, default_value_t = String::from(PATH))]
    path: String,
}

#[derive(Clone, ValueEnum)]
enum Setting {
    ConservationMode,
    FnLock,
    CameraPower,
    UsbCharging,
}

#[derive(Clone, ValueEnum)]
enum Action {
    Get,
    On,
    Off,
    Toggle,
}
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Action::Get => "get",
                Action::On => "on",
                Action::Off => "off",
                Action::Toggle => "toggle",
            }
        )
    }
}
fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let mut path = PathBuf::from(cli.path);
    path.push(match cli.setting {
        Setting::ConservationMode => "conservation_mode",
        Setting::FnLock => "fn_lock",
        Setting::CameraPower => "camera_power",
        Setting::UsbCharging => "usb_charging",
    });
    match cli.action {
        Action::Get => {
            println!(
                "{}",
                if get_setting(&path)? {
                    "on".green().to_string()
                } else {
                    "off".red().to_string()
                }
                .bold()
            );
            Ok(())
        }
        Action::On => set_setting(&path, true),
        Action::Off => set_setting(&path, false),
        Action::Toggle => set_setting(&path, !get_setting(&path)?),
    }
}

fn get_setting(path: &PathBuf) -> Result<bool> {
    match fs::read_to_string(path)?.chars().next().unwrap() {
        '1' => Ok(true),
        '0' => Ok(false),
        other => Err(eyre!("Invalid value: {}", other)),
    }
}

fn set_setting(path: &PathBuf, value: bool) -> Result<()> {
    fs::write(path, if value { "1" } else { "0" })?;
    Ok(())
}

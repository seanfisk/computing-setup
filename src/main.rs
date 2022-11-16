mod default_browser;
mod env;
mod fs;
mod git;
mod hammerspoon;
mod hg;
mod homebrew;
mod iterm2;
mod japicc;
mod karabiner;
mod logging;
mod login_items;
mod login_shells;
mod network_link_conditioner;
mod path;
mod preferences;
mod quicksilver;
mod scripts;
mod ssh;
mod user;
mod user_defaults;
mod verbose_command;

use anyhow::{anyhow, Result};
use clap::{crate_authors, crate_description, crate_name, App, AppSettings, Arg};
use log::{debug, info};
use logging::ColorMode;
use users::get_user_by_name;

fn is_root() -> bool {
    nix::unistd::Uid::current().is_root()
}

fn get_standard_username(cli_value: Option<&str>) -> Result<String> {
    debug!("Looking for standard user from CLI");
    match cli_value {
        Some(v) => {
            debug!("Standard user set to {:?} from command line", v);
            Ok(v.to_owned())
        }
        None => {
            debug!("Looking for standard user from SUDO_USER environment variable");
            match env::get("SUDO_USER")? {
                Some(v) => {
                    debug!("Standard user set to {:?} from SUDO_USER environment variable", v);
                    Ok(v)
                }
                None => Err(anyhow!("Standard user not given by --standard-user command-line option nor SUDO_USER environment variable")),
            }
        }
    }
}

fn main() -> Result<()> {
    if !is_root() {
        return Err(anyhow!("This program must be run as root!"));
    }

    const log_level_arg_name: &str = "log-level";
    let log_level_arg = Arg::with_name(log_level_arg_name)
        .short("l")
        .long("log-level")
        .possible_values(&logging::LogLevel::variants())
        .help("Set the minimum log level")
        .takes_value(true)
        .value_name("LEVEL");

    const standard_user_arg_name: &str = "username";
    let standard_user_arg = Arg::with_name(standard_user_arg_name)
        .short("u")
        .long("standard-user")
        .help("Standard user to run as; defaults to value of SUDO_USER environment variable")
        .takes_value(true)
        .value_name("USERNAME");

    const homebrew_arg_name: &str = "homebrew";
    let homebrew_arg = Arg::with_name(homebrew_arg_name)
        .short("-H")
        .long(homebrew_arg_name)
        .help("Install Homebrew formulae and casks (takes a long time)");

    const browser_arg_name: &str = "set-default-browser";
    let browser_arg = Arg::with_name(browser_arg_name)
        .short("-B")
        .long(browser_arg_name)
        .help("Set the default browser (shows a prompt every time)");

    let color_mode = logging::read_color_mode_from_env()?;

    let app = App::new(crate_name!())
        .about(crate_description!())
        .author(crate_authors!())
        .setting(AppSettings::ColoredHelp)
        .setting(match color_mode {
            ColorMode::Never => AppSettings::ColorNever,
            ColorMode::Always => AppSettings::ColorAlways,
            ColorMode::Auto => AppSettings::ColorAuto,
        })
        .arg(log_level_arg)
        .arg(standard_user_arg)
        .arg(homebrew_arg)
        .arg(browser_arg);

    let matches = app.get_matches();

    logging::init(color_mode, matches.value_of(log_level_arg_name))?;
    debug!("Logger was succesfully instantiated");

    let standard_username = get_standard_username(matches.value_of(standard_user_arg_name))?;
    let standard_user = get_user_by_name(&standard_username).ok_or_else(|| {
        anyhow!(
            "User with name {:?} does not exist on this system!",
            standard_username
        )
    })?;

    if matches.is_present(homebrew_arg_name) {
        homebrew::install_system(&standard_user)?;
        homebrew::install_deps(&standard_user)?;
    }

    login_shells::set(&standard_user)?;
    ssh::configure(&standard_user)?;
    // TODO Fix Zsh startup helper?

    iterm2::configure(&standard_user)?;
    login_items::configure(&standard_user)?;
    quicksilver::configure(&standard_user)?;
    hammerspoon::configure(&standard_user)?;
    karabiner::configure(&standard_user)?;
    git::configure(&standard_user)?;
    hg::configure(&standard_user)?;
    network_link_conditioner::install()?;
    japicc::install()?;

    if matches.is_present(browser_arg_name) {
        default_browser::set(&standard_user)?;
    }

    preferences::set(&standard_user)?;

    scripts::install(&standard_user)?;

    info!("Setup complete!");

    Ok(())
}

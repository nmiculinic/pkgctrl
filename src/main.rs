mod config;

use crate::config::Config;
use alpm::{Alpm, PackageReason};
use serde::Serialize;
use std::collections::HashSet;
use std::error;
use std::fmt;
use std::fmt::Formatter;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn error::Error>> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
    want:
    - "example"
    ignore:
    - "waat"
    ignoreGroups:
    - kde-applications
    - kde-utilities
    "#;

    let cfg: config::Config = serde_yaml::from_str(data)?;
    let system_state = analyze_system(&cfg)?;
    println!("{}", system_state);

    // let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    Ok(())
}

#[derive(Debug, Default, Serialize)]
struct SystemAnalysis {
    unwanted_found: Vec<String>,
    wanted_found: Vec<String>,
    wanted_missing: Vec<String>,
    ignored_found: Vec<String>,
}
impl fmt::Display for SystemAnalysis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match serde_yaml::to_string(self) {
            Ok(v) => writeln!(f, "{}", v),
            Err(e) => panic!("{}", e),
        }
    }
}

fn analyze_system(cfg: &Config) -> Result<SystemAnalysis, Box<dyn error::Error>> {
    let mut want = HashSet::new();
    for x in &cfg.want {
        want.insert(x);
    }
    let mut ignore = HashSet::new();
    for x in &cfg.ignore {
        ignore.insert(x);
    }
    let mut ignore_groups = HashSet::new();
    for x in &cfg.ignore_groups {
        ignore_groups.insert(x);
    }

    let pacman = pacmanconf::Config::new()?;
    let alpm = Alpm::new(pacman.root_dir, pacman.db_path)?;
    let mut sol = SystemAnalysis::default();
    'pkg: for pkg in alpm.localdb().pkgs() {
        if !(pkg.reason() == PackageReason::Explicit) {
            continue;
        }
        let pkg_name = pkg.name().to_string();
        if ignore.contains(&pkg_name) {
            sol.ignored_found.push(pkg_name);
            continue;
        }
        for group in pkg.groups() {
            if ignore_groups.contains(&group.to_string()) {
                sol.ignored_found.push(pkg_name);
                continue 'pkg;
            }
        }
        if want.contains(&pkg_name) {
            sol.wanted_found.push(pkg_name);
            continue;
        }
        sol.unwanted_found.push(pkg_name);
    }
    Ok(sol)
}

// fn print_installed(out: &mut impl WriteColor) -> std::io::Result<()> {
//     out.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
//     write!(out, "[installed]")?;
//     out.reset()?;
//     Ok(())
// }
//
// fn print_outdated(out: &mut impl WriteColor) -> std::io::Result<()> {
//     out.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
//     write!(out, "[~installed]")?;
//     out.reset()?;
//     Ok(())
// }

// fn print_pkg_with_name(
//     out: &mut impl WriteColor,
//     pkg_name: &str,
//     alpm: &Alpm,
// ) -> std::io::Result<()> {
//     let installed_pkg = alpm.localdb().pkg(pkg_name);
//     let db_list = alpm.syncdbs();
//     for db in db_list {
//         if let Ok(pkg) = db.pkg(pkg_name) {
//             print_package_details(out, alpm, &db, &pkg, &installed_pkg.ok())?;
//             break;
//         }
//     }
//     Ok(())
// }
//
// fn print_dep_list(
//     out: &mut impl WriteColor,
//     alpm: &Alpm,
//     dep_list: AlpmList<Dep>,
// ) -> std::io::Result<()> {
//     for dep in dep_list {
//         write!(out, "    {}", dep.name())?;
//         if let Some(ver) = dep.version() {
//             write!(out, " {}", ver)?;
//         }
//         if !dep.desc().is_none() {
//             write!(out, ": {}", dep.desc().unwrap())?;
//         }
//         let ip = alpm.localdb().pkgs().find_satisfier(dep.to_string());
//         if let Some(p) = ip {
//             write!(out, " ")?;
//             if p.name() == dep.name() {
//                 print_installed(out)?;
//             } else {
//                 write!(out, " [satisfied by {}]", p.name())?;
//             }
//         }
//         writeln!(out)?;
//     }
//     Ok(())
// }
//

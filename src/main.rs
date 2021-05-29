mod config;

use crate::config::Config;
use alpm::{Alpm, PackageReason};
use colored::*;
use human_panic::setup_panic;
use serde::Serialize;
use std::collections::{BTreeSet, HashSet};
use std::error;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "pkgctrl", about = "reconciled pacman packages")]
enum CLI {
    SyncConfig {
        #[structopt(short, long, parse(from_os_str))]
        config: PathBuf,

        #[structopt(long)]
        dry_run: bool,
    },
    Reconcile {
        #[structopt(short, long, parse(from_os_str))]
        config: PathBuf,

        #[structopt(long)]
        dry_run: bool,

        #[structopt(long)]
        noconfirm: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_panic!();
    let opt = CLI::from_args();
    match opt {
        CLI::SyncConfig { config, dry_run } => {
            let cfg: config::Config = {
                let file = File::open(&config)?;
                serde_yaml::from_reader(file)
            }?;
            let system_state = analyze_system(&cfg)?;
            system_state.terminal_dbg();

            if dry_run {
                println!("DRY RUN mode, quiting");
                return Ok(());
            }
            let mut new_cfg = cfg.clone();
            new_cfg.want = system_state.wanted_found;
            new_cfg.want.extend(system_state.unwanted_found);
            new_cfg.want.sort();
            {
                let file = File::create(&config)?;
                serde_yaml::to_writer(file, &new_cfg)?;
            }
            Ok(())
        }
        CLI::Reconcile {
            config,
            dry_run,
            noconfirm,
        } => {
            let cfg: config::Config = {
                let file = File::open(&config)?;
                serde_yaml::from_reader(file)
            }?;
            let system_state = analyze_system(&cfg)?;
            system_state.terminal_dbg();
            if dry_run {
                println!("DRY RUN mode, quiting");
                return Ok(());
            }

            if system_state.unwanted_found.len() > 0 {
                let mut cmd = Command::new("sudo");
                cmd.arg("pacman").arg("-R");

                if noconfirm {
                    cmd.arg("--noconfirm");
                }
                cmd.args(&system_state.unwanted_found).status()?;
            }

            if system_state.wanted_missing.len() > 0 {
                let mut cmd = Command::new("yay");
                cmd.arg("--sudoloop")
                    .arg("--nocleanmenu")
                    .arg("--nodiffmenu")
                    .arg("--noeditmenu");
                if noconfirm {
                    cmd.arg("--noconfirm");
                }
                cmd.arg("-Syu")
                    .args(&system_state.wanted_missing)
                    .status()?;
            }
            println!("{}", "Reconciliation successfully finished".green());
            Ok(())
        }
    }
}

#[derive(Debug, Default, Serialize)]
struct SystemAnalysis {
    unwanted_found: Vec<String>,
    wanted_found: Vec<String>,
    wanted_missing: Vec<String>,
    ignored_found: Vec<String>,
    groups_present: BTreeSet<String>,
}

impl SystemAnalysis {
    fn terminal_dbg(&self) {
        for pkg in &self.wanted_missing {
            println!("{}", format!("+++ {}", pkg).green());
        }
        for pkg in &self.unwanted_found {
            println!("{}", format!("--- {}", pkg).red());
        }
        // for grp in &self.groups_present {
        //     println!("{}", format!("ooo {}", grp).yellow());
        // }
    }
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
        for group in pkg.groups() {
            sol.groups_present.insert(group.to_string());
        }
        let pkg_name = pkg.name().to_string();
        for group in pkg.groups() {
            if ignore_groups.contains(&group.to_string()) {
                sol.ignored_found.push(pkg_name);
                continue 'pkg;
            }
        }
        if ignore.contains(&pkg_name) {
            sol.ignored_found.push(pkg_name);
            continue;
        }
        if want.contains(&pkg_name) {
            want.remove(&pkg_name);
            sol.wanted_found.push(pkg_name);
            continue;
        }
        sol.unwanted_found.push(pkg_name);
    }
    sol.wanted_missing = want.into_iter().map(|x| x.to_string()).collect();
    Ok(sol)
}

mod config;

use std::collections::HashSet;
use alpm::{Alpm, AlpmList, Db, Dep, Package, SigLevel, PackageReason};
use alpm_utils::DbListExt;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};


fn main() {
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

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let cfg: config::Config = serde_yaml::from_str(data).unwrap();

    let mut want = HashSet::new();
    for x in cfg.want {
        want.insert(x);
    }
    let mut ignore = HashSet::new();
    for x in cfg.ignore {
        ignore.insert(x);
    }
    let mut ignore_groups = HashSet::new();
    for x in cfg.ignore_groups {
        ignore_groups.insert(x);
    }

    let pacman = pacmanconf::Config::new().unwrap();
    let alpm = Alpm::new(pacman.root_dir, pacman.db_path).unwrap();
    for repo in pacman.repos {
        alpm.register_syncdb(repo.name, SigLevel::USE_DEFAULT)
            .unwrap();
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for pkg in alpm.localdb().pkgs() {
        if !(pkg.reason() == PackageReason::Explicit) {
            continue
        }
        if ignore.contains(pkg.name()) {
            continue
        }
        if want.contains(pkg.name()) {
            continue
        }
        println!("{}, {} {}",
                 pkg.name(),
                 pkg.install_date().unwrap_or(0),
                 pkg.groups().is_empty(),
        );
    }
}

fn print_installed(out: &mut impl WriteColor) -> std::io::Result<()> {
    out.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
    write!(out, "[installed]")?;
    out.reset()?;
    Ok(())
}

fn print_outdated(out: &mut impl WriteColor) -> std::io::Result<()> {
    out.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
    write!(out, "[~installed]")?;
    out.reset()?;
    Ok(())
}

fn print_pkg_with_name(
    out: &mut impl WriteColor,
    pkg_name: &str,
    alpm: &Alpm,
) -> std::io::Result<()> {
    let installed_pkg = alpm.localdb().pkg(pkg_name);
    let db_list = alpm.syncdbs();
    for db in db_list {
        if let Ok(pkg) = db.pkg(pkg_name) {
            print_package_details(out, alpm, &db, &pkg, &installed_pkg.ok())?;
            break;
        }
    }
    Ok(())
}

fn print_dep_list(
    out: &mut impl WriteColor,
    alpm: &Alpm,
    dep_list: AlpmList<Dep>,
) -> std::io::Result<()> {
    for dep in dep_list {
        write!(out, "    {}", dep.name())?;
        if let Some(ver) = dep.version() {
            write!(out, " {}", ver)?;
        }
        if !dep.desc().is_none() {
            write!(out, ": {}", dep.desc().unwrap())?;
        }
        let ip = alpm.localdb().pkgs().find_satisfier(dep.to_string());
        if let Some(p) = ip {
            write!(out, " ")?;
            if p.name() == dep.name() {
                print_installed(out)?;
            } else {
                write!(out, " [satisfied by {}]", p.name())?;
            }
        }
        writeln!(out)?;
    }
    Ok(())
}

fn print_package_details(
    out: &mut impl WriteColor,
    alpm: &Alpm,
    db: &Db,
    pkg: &Package,
    installed_pkg: &Option<Package>,
) -> std::io::Result<()> {
    write!(out, "{}/{} {}", db.name(), pkg.name(), pkg.version())?;
    if let Some(ip) = installed_pkg {
        write!(out, " ")?;
        if ip.version() != pkg.version() {
            print_outdated(out)?;
        } else {
            print_installed(out)?;
        }
    }
    writeln!(out)?;
    if let Some(desc) = pkg.desc() {
        writeln!(out)?;
        writeln!(out, "{}", desc)?;
    }
    writeln!(out)?;
    if let Some(ip) = installed_pkg {
        if ip.version() != pkg.version() {
            writeln!(out, "Installed Version: {}", ip.version())?;
        }
        let reason = match ip.reason() {
            alpm::PackageReason::Depend => "dependency",
            alpm::PackageReason::Explicit => "explicit",
        };
        writeln!(out, "Installed Reason: {}", reason)?;
    }
    writeln!(out, "Opt Depends:")?;
    print_dep_list(out, alpm, pkg.optdepends())?;
    writeln!(out, "Depends:")?;
    print_dep_list(out, alpm, pkg.depends())?;
    Ok(())
}

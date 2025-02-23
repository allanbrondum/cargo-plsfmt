use cargo_metadata::Package;
use clap::Parser;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub struct Opts {
    #[arg(
        short = 'p',
        long = "package",
        value_name = "package",
        num_args = 1..
    )]
    packages: Vec<String>,
}

fn main() {
    let mut found_fmt = false;
    let args = env::args().filter(|x| {
        if found_fmt {
            true
        } else {
            found_fmt = x == "plsfmt";
            x != "fmt"
        }
    });

    let opts = Opts::parse_from(args);
    let strategy = FmtStrategy::from_opts(&opts);

    format_workspace(&strategy);
}

fn format_workspace(strategy: &FmtStrategy) {
    let packages = get_packages(strategy);

    for package in packages {
        for file in files_in_package(&package) {
            format_file(&file);
        }
    }
}

fn format_file(file: &PathBuf) {
    let content = fs::read_to_string(file).unwrap();
    let formatted = cargo_plsfmt::format_file(&content);
    fs::write(file, formatted).unwrap();
}

fn files_in_package(package: &Package) -> impl Iterator<Item = PathBuf> {
    let src_dir = package.manifest_path.parent().unwrap().join("src");

    glob::glob(&format!("{}/**/*.rs", src_dir))
        .expect("Failed to read source directory")
        .filter_map(Result::ok)
}

#[derive(Debug, PartialEq, Eq)]
pub enum FmtStrategy {
    Packages(Vec<String>),
    Root,
}

impl FmtStrategy {
    pub fn from_opts(opts: &Opts) -> FmtStrategy {
        if opts.packages.is_empty() {
            FmtStrategy::Root
        } else {
            FmtStrategy::Packages(opts.packages.clone())
        }
    }
}

fn get_packages(strategy: &FmtStrategy) -> Vec<Package> {
    match *strategy {
        FmtStrategy::Root => get_packages_root_only(),
        FmtStrategy::Packages(ref hitlist) => get_packages_with_hitlist(hitlist),
    }
}

fn get_packages_root_only() -> Vec<Package> {
    let metadata = get_cargo_metadata();
    let workspace_root_path = PathBuf::from(&metadata.workspace_root)
        .canonicalize()
        .unwrap();

    let current_dir = env::current_dir().unwrap().canonicalize().unwrap();
    let in_workspace_root = workspace_root_path == current_dir;
    let current_dir_manifest = current_dir.join("Cargo.toml");

    match metadata.packages.len() {
        1 => metadata.packages,
        _ => metadata
            .packages
            .into_iter()
            .filter(|p| {
                in_workspace_root
                    || PathBuf::from(&p.manifest_path)
                        .canonicalize()
                        .unwrap_or_default()
                        == current_dir_manifest
            })
            .collect(),
    }
}

fn get_packages_with_hitlist(hitlist: &[String]) -> Vec<Package> {
    let metadata = get_cargo_metadata();
    let mut workspace_hitlist: BTreeSet<&String> = BTreeSet::from_iter(hitlist);

    let packages = metadata
        .packages
        .into_iter()
        .filter(|package| workspace_hitlist.remove(&package.name))
        .collect();

    if !workspace_hitlist.is_empty() {
        let package = workspace_hitlist.iter().next().unwrap();
        panic!("package `{package}` is not a member of the workspace");
    }

    packages
}

fn get_cargo_metadata() -> cargo_metadata::Metadata {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.no_deps();
    cmd.other_options(vec![String::from("--offline")]);
    cmd.exec().unwrap()
}

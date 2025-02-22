use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use clap::Parser;

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

    format_crate(&strategy);
}

fn format_crate(strategy: &FmtStrategy) {
    let _targets = get_targets(strategy);

    // todo
}

#[derive(Debug)]
pub struct Target {
    path: PathBuf,
}

impl Target {
    pub fn from_target(target: &cargo_metadata::Target) -> Self {
        let path = PathBuf::from(&target.src_path);
        let canonicalized = fs::canonicalize(&path).unwrap_or(path);

        Target {
            path: canonicalized,
        }
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Target) -> bool {
        self.path == other.path
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Target) -> Option<Ordering> {
        Some(self.path.cmp(&other.path))
    }
}

impl Ord for Target {
    fn cmp(&self, other: &Target) -> Ordering {
        self.path.cmp(&other.path)
    }
}

impl Eq for Target {}

impl Hash for Target {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
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

fn get_targets(strategy: &FmtStrategy) -> BTreeSet<Target> {
    let mut targets = BTreeSet::new();

    match *strategy {
        FmtStrategy::Root => get_targets_root_only(&mut targets),
        FmtStrategy::Packages(ref hitlist) => get_targets_with_hitlist(hitlist, &mut targets),
    }

    targets
}

fn get_targets_root_only(targets: &mut BTreeSet<Target>) {
    let metadata = get_cargo_metadata();
    let workspace_root_path = PathBuf::from(&metadata.workspace_root)
        .canonicalize()
        .unwrap();

    let current_dir = env::current_dir().unwrap().canonicalize().unwrap();
    let in_workspace_root = workspace_root_path == current_dir;
    let current_dir_manifest = current_dir.join("Cargo.toml");

    let package_targets = match metadata.packages.len() {
        1 => metadata.packages.into_iter().next().unwrap().targets,
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
            .flat_map(|p| p.targets)
            .collect(),
    };

    for target in package_targets {
        targets.insert(Target::from_target(&target));
    }
}

fn get_targets_with_hitlist(hitlist: &[String], targets: &mut BTreeSet<Target>) {
    let metadata = get_cargo_metadata();
    let mut workspace_hitlist: BTreeSet<&String> = BTreeSet::from_iter(hitlist);

    for package in metadata.packages {
        if workspace_hitlist.remove(&package.name) {
            for target in package.targets {
                targets.insert(Target::from_target(&target));
            }
        }
    }

    if !workspace_hitlist.is_empty() {
        let package = workspace_hitlist.iter().next().unwrap();
        panic!("package `{package}` is not a member of the workspace");
    }
}

fn get_cargo_metadata() -> cargo_metadata::Metadata {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.no_deps();
    cmd.other_options(vec![String::from("--offline")]);
    cmd.exec().unwrap()
}

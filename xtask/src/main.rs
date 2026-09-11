//! `xtask`: thin, non-interactive project automation over the `scaffold` crate.
//!
//! ```text
//! cargo xtask generate <name> [--layout X] [--with a,b] [--without c,d]
//!                       [--theme T] [--app-name "..."] [--shell-path DIR]
//!                       [--output-dir DIR]
//! cargo xtask list-presets
//! cargo xtask list-features
//! ```
//!
//! Fully deterministic (same input yields the same bytes, no timestamps) and
//! 100% non-interactive: every failure prints a clear message to stderr and
//! exits non-zero. Zero external dependencies: argument parsing is manual.

use scaffold::model::{Feature, Layout};
use scaffold::spec::{GenerateOptions, split_feature_list};
use scaffold::{VALID_FEATURES, VALID_LAYOUTS, resolve, write_project};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    std::process::exit(dispatch(std::env::args().skip(1).collect()));
}

/// Route the command. Returns the process exit code.
fn dispatch(args: Vec<String>) -> i32 {
    if args.is_empty() {
        eprintln!("error: missing command.\n\n{USAGE}");
        return 2;
    }
    match args[0].as_str() {
        "--help" | "-h" | "help" => {
            println!("{USAGE}");
            0
        }
        "--version" | "-V" | "version" => {
            println!("xtask {VERSION}");
            0
        }
        "generate" => match parse_generate(&args[1..]) {
            Ok(request) => match run_generate(request) {
                Ok(()) => 0,
                Err(message) => {
                    eprintln!("error: {message}");
                    1
                }
            },
            Err(message) => {
                eprintln!("error: {message}");
                2
            }
        },
        "list-presets" => {
            print_presets();
            0
        }
        "list-features" => {
            print_features();
            0
        }
        other => {
            eprintln!("error: unknown command '{other}'.\n\n{USAGE}");
            2
        }
    }
}

const USAGE: &str = "\
iced-desktop-shell project automation

USAGE:
    cargo xtask generate <name> [OPTIONS]
    cargo xtask list-presets
    cargo xtask list-features
    cargo xtask --help

GENERATE OPTIONS:
    --layout <preset>     technical-ribbon (default), ide, studio, operator, minimal
    --with <a,b>          add capabilities (repeatable, comma-separated)
    --without <a,b>       remove preset defaults (repeatable, comma-separated)
    --theme <theme>       dark (default) or light
    --app-name \"...\"      display name (default: Title Case of <name>)
    --shell-path <dir>    use a local desktop-shell checkout instead of the git URL
    --output-dir <dir>    target directory (default: <name> under the current dir)

FEATURES (valid --with / --without values):
    ribbon, menu, toolbar, activity-bar, explorer, inspector, bottom-panel,
    statusbar, persistence, theme

EXAMPLES:
    cargo xtask generate printnc-config --layout technical-ribbon
    cargo xtask generate omt-control --layout ide --without explorer
    cargo xtask generate converter --layout minimal --theme light
";

struct GenerateRequest {
    options: GenerateOptions,
    output_dir: PathBuf,
}

fn parse_generate(args: &[String]) -> Result<GenerateRequest, String> {
    let mut options = GenerateOptions {
        layout: "technical-ribbon".to_string(),
        theme: "dark".to_string(),
        ..Default::default()
    };
    let mut output_dir: Option<PathBuf> = None;
    let mut positionals: Vec<String> = Vec::new();

    let mut index = 0;
    while index < args.len() {
        let arg = args[index].as_str();
        if arg == "--help" || arg == "-h" {
            return Err(format!("generate usage:\n\n{GENERATE_USAGE}"));
        } else if let Some(value) = flag_value(args, &mut index, "--layout") {
            options.layout = value;
        } else if let Some(value) = flag_value(args, &mut index, "--with") {
            options.with.extend(split_feature_list(&value));
        } else if let Some(value) = flag_value(args, &mut index, "--without") {
            options.without.extend(split_feature_list(&value));
        } else if let Some(value) = flag_value(args, &mut index, "--theme") {
            options.theme = value;
        } else if let Some(value) = flag_value(args, &mut index, "--app-name") {
            options.app_name = value;
        } else if let Some(value) = flag_value(args, &mut index, "--shell-path") {
            options.shell_path = Some(value);
        } else if let Some(value) = flag_value(args, &mut index, "--output-dir") {
            output_dir = Some(PathBuf::from(value));
        } else if arg.starts_with("--") {
            return Err(format!(
                "unknown flag '{arg}' for generate. Expected one of: \
                 --layout, --with, --without, --theme, --app-name, \
                 --shell-path, --output-dir."
            ));
        } else {
            positionals.push(args[index].clone());
            index += 1;
        }
    }

    if positionals.len() > 1 {
        return Err(format!(
            "generate takes a single project name, got {} positional arguments ({}).",
            positionals.len(),
            positionals.join(", ")
        ));
    }
    let Some(name) = positionals.into_iter().next() else {
        return Err(
            "generate requires a project name, e.g. `cargo xtask generate my-app`.".to_string(),
        );
    };
    options.name = name.clone();

    let resolved_dir = match output_dir {
        Some(dir) => dir,
        None => PathBuf::from(name),
    };
    Ok(GenerateRequest {
        options,
        output_dir: resolved_dir,
    })
}

const GENERATE_USAGE: &str = "\
cargo xtask generate <name> [--layout X] [--with a,b] [--without c,d] \
[--theme T] [--app-name \"...\"] [--shell-path DIR] [--output-dir DIR]";

/// Read a `--flag value` or `--flag=value` argument. Returns `None` when the
/// current argument is not the requested flag. Advances the index on success.
fn flag_value(args: &[String], index: &mut usize, flag: &str) -> Option<String> {
    let current = args.get(*index).map(String::as_str).unwrap_or("");
    if let Some(inline) = current.strip_prefix(&format!("{flag}=")) {
        *index += 1;
        return Some(inline.to_string());
    }
    if current == flag {
        let value = args.get(*index + 1).cloned().unwrap_or_default();
        *index += 2;
        return Some(value);
    }
    None
}

fn run_generate(request: GenerateRequest) -> Result<(), String> {
    let spec = resolve(&request.options).map_err(|err| err.to_string())?;
    write_project(&spec, &request.output_dir).map_err(|err| err.to_string())?;
    println!(
        "Generated '{}' (layout {}, features: {}) at {}",
        spec.crate_name,
        spec.layout.kebab(),
        spec.feature_list(),
        request.output_dir.display()
    );
    Ok(())
}

fn print_presets() {
    println!("Available presets (defaults are added unless removed with --without):\n");
    for raw in VALID_LAYOUTS {
        let Some(layout) = Layout::parse(raw) else {
            continue;
        };
        let defaults: Vec<&str> = layout
            .default_features()
            .iter()
            .map(Feature::kebab)
            .collect();
        println!("  {:<18} [{}]", layout.kebab(), defaults.join(", "));
        println!("  {:<18} {}", "", layout.describe());
        let slots = layout.slots();
        let left = slots.left.map(|s| (s.id, s.title));
        let right = slots.right.map(|s| (s.id, s.title));
        let bottom = slots.bottom.map(|s| (s.id, s.title));
        println!(
            "  {:<18} slots: left={}, right={}, bottom={}\n",
            "",
            slot_name(left),
            slot_name(right),
            slot_name(bottom),
        );
    }
    println!(
        "Inspect panel slot ids/titles per preset above; domain content is stubbed in the generated src/demo.rs."
    );
}

fn print_features() {
    println!("Valid --with / --without values:\n");
    for feature in VALID_FEATURES {
        println!("  {feature}");
    }
    println!(
        "\nNotes:\n\
         \x20 - `workspace` always exists and must never be listed.\n\
         \x20 - `ribbon` requires --layout technical-ribbon.\n\
         \x20 - `activity-bar` requires --layout ide.\n\
         \x20 - `persistence` is opt-in everywhere (theme/panel state on disk).\n\
         \x20 - `theme` is always wired; pick the default with --theme dark|light."
    );
}

fn slot_name(slot: Option<(&str, &str)>) -> String {
    match slot {
        Some((id, title)) => format!("{id} (\"{title}\")"),
        None => "-".to_string(),
    }
}

//! Integration tests for the `scaffold` crate: layout parsing, preset
//! defaults, feature validation, combination rules, and render determinism.

use scaffold::model::{Feature, Layout};
use scaffold::spec::{GenerateOptions, ResolvedSpec, split_feature_list};
use scaffold::{VALID_FEATURES, render_project, resolve};

fn base_options(name: &str, layout: &str) -> GenerateOptions {
    GenerateOptions {
        name: name.to_string(),
        layout: layout.to_string(),
        theme: "dark".to_string(),
        ..Default::default()
    }
}

fn resolved(opts: &GenerateOptions) -> Option<ResolvedSpec> {
    match resolve(opts) {
        Ok(spec) => Some(spec),
        Err(err) => {
            panic!("unexpected resolve error: {err}");
        }
    }
}

fn expect_error(opts: &GenerateOptions, fragment: &str) {
    match resolve(opts) {
        Ok(spec) => panic!(
            "expected error containing '{fragment}', got spec: {}",
            spec.feature_list()
        ),
        Err(err) => assert!(
            err.to_string().contains(fragment),
            "error '{err}' should mention '{fragment}'"
        ),
    }
}

fn feature_set(spec: &ResolvedSpec) -> Vec<&'static str> {
    spec.features.iter().map(Feature::kebab).collect()
}

// ---------------------------------------------------------------------------
// layout parsing
// ---------------------------------------------------------------------------

#[test]
fn parses_all_kebab_layout_names() {
    let cases = [
        ("technical-ribbon", Layout::TechnicalRibbon),
        ("ide", Layout::Ide),
        ("studio", Layout::Studio),
        ("operator", Layout::Operator),
        ("minimal", Layout::Minimal),
    ];
    for (raw, expected) in cases {
        assert_eq!(Layout::parse(raw), Some(expected), "layout '{raw}'");
    }
}

#[test]
fn rejects_unknown_layout_with_valid_list() {
    assert_eq!(Layout::parse("vscode"), None);
    let mut opts = base_options("demo", "vscode");
    expect_error(&opts, "technical-ribbon");
    opts.layout = String::new();
    expect_error(&opts, "Valid layouts");
}

// ---------------------------------------------------------------------------
// defaults per preset
// ---------------------------------------------------------------------------

#[test]
fn technical_ribbon_defaults() {
    let opts = base_options("cad", "technical-ribbon");
    if let Some(spec) = resolved(&opts) {
        assert_eq!(
            feature_set(&spec),
            vec![
                "ribbon",
                "menu",
                "explorer",
                "inspector",
                "bottom-panel",
                "statusbar"
            ]
        );
    }
}

#[test]
fn ide_defaults() {
    let opts = base_options("devtool", "ide");
    if let Some(spec) = resolved(&opts) {
        assert_eq!(
            feature_set(&spec),
            vec![
                "menu",
                "activity-bar",
                "explorer",
                "bottom-panel",
                "statusbar"
            ]
        );
    }
}

#[test]
fn studio_defaults() {
    let opts = base_options("gis", "studio");
    if let Some(spec) = resolved(&opts) {
        assert_eq!(
            feature_set(&spec),
            vec![
                "menu",
                "toolbar",
                "explorer",
                "inspector",
                "bottom-panel",
                "statusbar"
            ]
        );
    }
}

#[test]
fn operator_defaults_use_navigation_slots() {
    let opts = base_options("hmi", "operator");
    if let Some(spec) = resolved(&opts) {
        assert_eq!(
            feature_set(&spec),
            vec!["menu", "explorer", "inspector", "bottom-panel", "statusbar"]
        );
        let slots = spec.layout.slots();
        let left = slots.left.map(|s| (s.id, s.title));
        let right = slots.right.map(|s| (s.id, s.title));
        let bottom = slots.bottom.map(|s| (s.id, s.title));
        assert_eq!(left, Some(("navigation", "Navigation")));
        assert_eq!(right, Some(("controls", "Controls")));
        assert_eq!(bottom, Some(("alarms", "Alarms")));
    }
}

#[test]
fn minimal_defaults() {
    let opts = base_options("converter", "minimal");
    if let Some(spec) = resolved(&opts) {
        assert_eq!(feature_set(&spec), vec!["menu", "toolbar", "statusbar"]);
    }
}

#[test]
fn studio_slots_use_domain_flavoured_names() {
    let slots = Layout::Studio.slots();
    let left = slots.left.map(|s| (s.id, s.title));
    let right = slots.right.map(|s| (s.id, s.title));
    let bottom = slots.bottom.map(|s| (s.id, s.title));
    assert_eq!(left, Some(("hierarchy", "Hierarchy")));
    assert_eq!(right, Some(("properties", "Properties")));
    assert_eq!(bottom, Some(("timeline", "Timeline")));
}

// ---------------------------------------------------------------------------
// with / without handling
// ---------------------------------------------------------------------------

#[test]
fn with_adds_and_without_removes() {
    let mut opts = base_options("cad", "technical-ribbon");
    opts.with = split_feature_list("persistence");
    opts.without = split_feature_list("inspector");
    if let Some(spec) = resolved(&opts) {
        let set = feature_set(&spec);
        assert!(set.contains(&"persistence"));
        assert!(!set.contains(&"inspector"));
        assert!(set.contains(&"ribbon"));
    }
}

#[test]
fn rejects_unknown_feature_listing_valid_ones() {
    let mut opts = base_options("cad", "technical-ribbon");
    opts.with = split_feature_list("notifications");
    expect_error(&opts, "Valid features");
    for valid in VALID_FEATURES {
        assert!(Feature::parse(valid).is_some(), "feature '{valid}'");
    }
}

#[test]
fn rejects_typo_like_console_or_tabs() {
    for unknown in [
        "tabs",
        "command-palette",
        "terminal",
        "console",
        "secondary-sidebar",
    ] {
        let mut opts = base_options("cad", "ide");
        opts.with = split_feature_list(unknown);
        expect_error(&opts, unknown);
    }
}

#[test]
fn rejects_duplicate_features() {
    let mut opts = base_options("cad", "ide");
    opts.with = split_feature_list("menu,menu");
    expect_error(&opts, "duplicate");

    let mut opts = base_options("cad", "ide");
    opts.without = split_feature_list("explorer, explorer");
    expect_error(&opts, "duplicate");

    let mut opts = base_options("cad", "ide");
    opts.with = split_feature_list("persistence");
    opts.without = split_feature_list("persistence");
    expect_error(&opts, "both --with and --without");
}

#[test]
fn rejects_removing_workspace() {
    let mut opts = base_options("cad", "minimal");
    opts.without = split_feature_list("workspace");
    expect_error(&opts, "cannot be removed");
}

#[test]
fn rejects_requesting_workspace() {
    let mut opts = base_options("cad", "minimal");
    opts.with = split_feature_list("workspace");
    expect_error(&opts, "always present");
}

#[test]
fn rejects_ribbon_outside_technical_ribbon() {
    let mut opts = base_options("tool", "minimal");
    opts.with = split_feature_list("ribbon");
    expect_error(&opts, "technical-ribbon");

    let mut opts = base_options("tool", "ide");
    opts.with = split_feature_list("ribbon");
    expect_error(&opts, "does not support the ribbon");
}

#[test]
fn rejects_activity_bar_outside_ide() {
    let mut opts = base_options("cad", "technical-ribbon");
    opts.with = split_feature_list("activity-bar");
    expect_error(&opts, "--layout ide");
}

#[test]
fn rejects_bad_names_layouts_and_themes() {
    let opts = base_options("", "ide");
    expect_error(&opts, "must not be empty");

    let opts = base_options("Bad Name!", "ide");
    expect_error(&opts, "invalid project name");

    let mut opts = base_options("ok-name", "ide");
    opts.theme = "neon".to_string();
    expect_error(&opts, "Valid themes");

    let mut opts = base_options("ok-name", "ide");
    opts.theme = "light".to_string();
    if let Some(spec) = resolved(&opts) {
        assert_eq!(spec.theme.kebab(), "light");
    }
}

// ---------------------------------------------------------------------------
// render
// ---------------------------------------------------------------------------

#[test]
fn renders_expected_files_for_every_preset() {
    for layout in ["technical-ribbon", "ide", "studio", "operator", "minimal"] {
        let opts = base_options("preset-smoke", layout);
        if let Some(spec) = resolved(&opts) {
            let files = render_project(&spec);
            let paths: Vec<&str> = files.iter().map(|file| file.path).collect();
            assert_eq!(
                paths,
                vec![
                    "Cargo.toml",
                    "src/main.rs",
                    "src/app.rs",
                    "src/demo.rs",
                    ".scaffold.toml",
                    "AGENTS.md",
                    "README.md",
                ],
                "layout '{layout}'"
            );
            for file in &files {
                assert!(!file.contents.is_empty(), "{} is empty", file.path);
            }
        }
    }
}

#[test]
fn render_is_deterministic() {
    let mut opts = base_options("stable", "operator");
    opts.with = split_feature_list("persistence");
    if let Some(spec) = resolved(&opts) {
        let first = render_project(&spec);
        let second = render_project(&spec);
        assert_eq!(first.len(), second.len());
        for (left, right) in first.iter().zip(second.iter()) {
            assert_eq!(left.path, right.path);
            assert_eq!(left.contents, right.contents);
        }
    }
}

#[test]
fn render_contains_no_unselected_feature_code() {
    let opts = base_options("converter", "minimal");
    if let Some(spec) = resolved(&opts) {
        let files = render_project(&spec);
        let demo = files
            .iter()
            .find(|file| file.path == "src/demo.rs")
            .map(|file| file.contents.as_str())
            .unwrap_or("");
        assert!(
            !demo.contains("VIEW_TOGGLE_EXPLORER"),
            "minimal must not reference explorer commands"
        );
        assert!(
            !demo.contains("ribbon_tabs"),
            "minimal must not contain ribbon code"
        );
        assert!(
            demo.contains("toolbar_items"),
            "minimal must contain its toolbar"
        );
    }
}

#[test]
fn render_wires_shell_path_override() {
    let mut opts = base_options("local", "ide");
    opts.shell_path = Some("/tmp/fake-shell".to_string());
    // Directory does not exist: resolution must fail before rendering.
    expect_error(&opts, "not an existing directory");
}

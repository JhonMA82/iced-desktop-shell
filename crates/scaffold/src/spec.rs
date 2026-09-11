//! Generation request validation: from raw CLI strings to a [`ResolvedSpec`].
//!
//! Validation is explicit and predictable: unknown names, reserved names,
//! duplicates, and incoherent combinations are rejected with a message that
//! names the valid options. The renderer never receives a broken spec.

use super::model::{
    Feature, Layout, RESERVED_ALWAYS_PRESENT, Theme, VALID_FEATURES, VALID_LAYOUTS,
};
use std::collections::BTreeSet;
use std::fmt;

/// Raw generation request, exactly as parsed from CLI flags.
#[derive(Debug, Clone, Default)]
pub struct GenerateOptions {
    /// Project (and default directory) name, e.g. `printnc-config`.
    pub name: String,
    /// `--layout` value.
    pub layout: String,
    /// `--with` entries (already split on commas by the caller).
    pub with: Vec<String>,
    /// `--without` entries (already split on commas by the caller).
    pub without: Vec<String>,
    /// `--theme` value.
    pub theme: String,
    /// `--app-name` display name. Empty means "derive from `name`".
    pub app_name: String,
    /// `--shell-path` local override for the `desktop-shell` dependency.
    pub shell_path: Option<String>,
}

/// Fully validated, renderer-ready specification.
#[derive(Debug, Clone)]
pub struct ResolvedSpec {
    /// Cargo package name (sanitized).
    pub crate_name: String,
    /// Human display name used for window title and docs.
    pub app_title: String,
    pub layout: Layout,
    /// Final feature set in canonical (enum declaration) order.
    pub features: Vec<Feature>,
    pub theme: Theme,
    /// `desktop-shell` dependency source override for local verification.
    pub shell_path: Option<String>,
}

/// Validation failure with a human- and agent-readable message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldError(pub String);

impl fmt::Display for ScaffoldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ScaffoldError {}

impl ResolvedSpec {
    pub fn has_feature(&self, feature: Feature) -> bool {
        self.features.contains(&feature)
    }

    pub fn feature_list(&self) -> String {
        self.features
            .iter()
            .map(Feature::kebab)
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Split a `--with` / `--without` flag value on commas, trimming whitespace
/// and dropping empty segments (so `--with "a, b,"` behaves sanely).
pub fn split_feature_list(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(str::to_string)
        .collect()
}

/// Validate `options` into a [`ResolvedSpec`].
pub fn resolve(options: &GenerateOptions) -> Result<ResolvedSpec, ScaffoldError> {
    let crate_name = sanitize_crate_name(&options.name)?;
    let app_title = if options.app_name.trim().is_empty() {
        title_case(&options.name)
    } else {
        options.app_name.trim().to_string()
    };

    let layout_name = options.layout.trim();
    let layout = Layout::parse(layout_name).ok_or_else(|| {
        ScaffoldError(format!(
            "unknown layout '{layout_name}'. Valid layouts: {}.",
            VALID_LAYOUTS.join(", ")
        ))
    })?;

    let theme_name = if options.theme.trim().is_empty() {
        "dark"
    } else {
        options.theme.trim()
    };
    let theme = Theme::parse(theme_name).ok_or_else(|| {
        ScaffoldError(format!(
            "unknown theme '{theme_name}'. Valid themes: dark, light."
        ))
    })?;

    let with = parse_feature_entries(&options.with, "--with")?;
    let without = parse_feature_entries(&options.without, "--without")?;

    // Duplicate inside one flag.
    for (entries, flag) in [(&with, "--with"), (&without, "--without")] {
        let mut seen = BTreeSet::new();
        for feature in entries {
            if !seen.insert(feature.kebab()) {
                return Err(ScaffoldError(format!(
                    "duplicate feature '{}' in {flag}. List each feature only once.",
                    feature.kebab()
                )));
            }
        }
    }

    // Same feature requested and removed.
    for feature in &with {
        if without.contains(feature) {
            return Err(ScaffoldError(format!(
                "feature '{}' is listed in both --with and --without. Keep exactly one.",
                feature.kebab()
            )));
        }
    }

    // Defaults must only reference implemented features; this is a renderer
    // invariant, never a user error to work around.
    for feature in layout.default_features() {
        if !feature.is_implemented() {
            return Err(ScaffoldError(format!(
                "internal error: layout '{}' requires unimplemented feature '{}'. \
                 The generator refuses to emit broken code.",
                layout.kebab(),
                feature.kebab()
            )));
        }
    }

    // Final set: (defaults + with) - without, in canonical order.
    let mut set: BTreeSet<Feature> = layout.default_features().into_iter().collect();
    for feature in &with {
        set.insert(*feature);
    }
    for feature in &without {
        set.remove(feature);
    }
    let features: Vec<Feature> = set.into_iter().collect();

    check_combination(layout, &features)?;

    let shell_path = match options.shell_path.as_ref() {
        None => None,
        Some(path) => Some(resolve_shell_path(path.trim())?),
    };

    Ok(ResolvedSpec {
        crate_name,
        app_title,
        layout,
        features,
        theme,
        shell_path,
    })
}

/// Resolve `--shell-path` to the absolute directory of the `desktop-shell`
/// crate. Accepts both the crate directory itself and the repository root
/// (which contains `crates/desktop-shell`). The stored path is canonicalized
/// so the generated `Cargo.toml` path dependency stays valid wherever the
/// project is generated from.
fn resolve_shell_path(raw: &str) -> Result<String, ScaffoldError> {
    if raw.is_empty() {
        return Err(ScaffoldError(
            "--shell-path must not be empty. Pass the desktop-shell crate directory or the repository root.".to_string(),
        ));
    }
    let base = std::path::Path::new(raw);
    if !base.is_dir() {
        return Err(ScaffoldError(format!(
            "--shell-path '{raw}' is not an existing directory."
        )));
    }
    // Accept the repository root transparently.
    let crate_dir = if base.join("crates/desktop-shell").is_dir() {
        base.join("crates/desktop-shell")
    } else {
        base.to_path_buf()
    };
    if !crate_dir.join("Cargo.toml").is_file() {
        return Err(ScaffoldError(format!(
            "--shell-path '{raw}' contains no crate (expected Cargo.toml in '{}'). Pass the desktop-shell crate directory or the repository root.",
            crate_dir.display()
        )));
    }
    match std::fs::canonicalize(&crate_dir) {
        Ok(absolute) => Ok(absolute.to_string_lossy().into_owned()),
        Err(err) => Err(ScaffoldError(format!(
            "--shell-path '{}' cannot be resolved to an absolute path: {err}.",
            crate_dir.display()
        ))),
    }
}

/// Parse one flag's entries, rejecting reserved and unknown names.
fn parse_feature_entries(entries: &[String], flag: &str) -> Result<Vec<Feature>, ScaffoldError> {
    let mut out = Vec::with_capacity(entries.len());
    for raw in entries {
        let name = raw.trim();
        if RESERVED_ALWAYS_PRESENT
            .iter()
            .any(|reserved| reserved.eq_ignore_ascii_case(name))
        {
            if flag == "--without" {
                return Err(ScaffoldError(
                    "'workspace' cannot be removed: every application has a central \
                     workspace view. Drop '--without workspace' and, if the workspace \
                     should stay empty for now, leave its stub content untouched."
                        .to_string(),
                ));
            }
            return Err(ScaffoldError(
                "'workspace' is always present: every generated application ships \
                 with a central workspace view. Remove it from --with."
                    .to_string(),
            ));
        }
        match Feature::parse(name) {
            Some(feature) => out.push(feature),
            None => {
                return Err(ScaffoldError(format!(
                    "unknown feature '{name}' in {flag}. Valid features: {}.",
                    VALID_FEATURES.join(", ")
                )));
            }
        }
    }
    Ok(out)
}

/// Reject spatially incoherent combinations explicitly instead of generating
/// a broken or misleading interface.
fn check_combination(layout: Layout, features: &[Feature]) -> Result<(), ScaffoldError> {
    let has = |feature: Feature| features.contains(&feature);
    if has(Feature::Ribbon) && layout != Layout::TechnicalRibbon {
        return Err(ScaffoldError(format!(
            "layout '{}' does not support the ribbon: the ribbon defines the \
             technical-ribbon top chrome. Use '--layout technical-ribbon' for a \
             ribbon shell, or drop '--with ribbon'.",
            layout.kebab()
        )));
    }
    if has(Feature::ActivityBar) && layout != Layout::Ide {
        return Err(ScaffoldError(format!(
            "layout '{}' does not support the activity-bar: the vertical activity \
             rail defines the ide workbench pattern. Use '--layout ide' for an \
             activity rail, or drop '--with activity-bar'.",
            layout.kebab()
        )));
    }
    Ok(())
}

/// Validate the project name as a Cargo package name.
fn sanitize_crate_name(raw: &str) -> Result<String, ScaffoldError> {
    let name = raw.trim();
    if name.is_empty() {
        return Err(ScaffoldError(
            "project name must not be empty. Pass a name such as 'my-app'.".to_string(),
        ));
    }
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err(ScaffoldError(format!(
            "invalid project name '{name}': names must be plain Cargo package names, not paths."
        )));
    }
    let mut chars = name.chars();
    let first = match chars.next() {
        Some(first) => first,
        None => {
            return Err(ScaffoldError(
                "project name must not be empty. Pass a name such as 'my-app'.".to_string(),
            ));
        }
    };
    if !first.is_ascii_alphanumeric() {
        return Err(ScaffoldError(format!(
            "invalid project name '{name}': it must start with a letter or digit and \
             contain only lowercase letters, digits, '-' and '_'."
        )));
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    {
        return Err(ScaffoldError(format!(
            "invalid project name '{name}': use only lowercase letters, digits, '-' and '_'."
        )));
    }
    Ok(name.to_string())
}

/// Derive a display name: `printnc-config` -> `Printnc Config`.
fn title_case(raw: &str) -> String {
    raw.split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let mut word = first.to_uppercase().to_string();
                    word.push_str(&chars.as_str().to_lowercase());
                    word
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    fn options(name: &str) -> GenerateOptions {
        GenerateOptions {
            name: name.to_string(),
            layout: "ide".to_string(),
            theme: "dark".to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn rejects_path_like_names() {
        let mut bad = options("../evil");
        assert!(resolve(&bad).is_err());
        bad = options("a/b");
        assert!(resolve(&bad).is_err());
    }
}

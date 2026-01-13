use crate::{BuildProfile, Target};
use anyhow::{anyhow, bail, Context, Result};
use cargo_metadata::{CrateType, Metadata, TargetKind};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::Path;
use strsim::levenshtein;

const WASM_BINDGEN_METADATA_KEY: &str = "package.metadata.wasm-bindgen";

pub struct CrateData {
    data: Metadata,
    current_idx: usize,
    manifest: CargoManifest,
    out_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CargoManifest {
    package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
    #[serde(default)]
    metadata: CargoMetadata,
}

#[derive(Default, Deserialize)]
struct CargoMetadata {
    #[serde(default, rename = "wasm-bindgen")]
    wasm_bindgen: CargoWasmBindgen,
}

#[derive(Default, Deserialize)]
pub struct CargoWasmBindgen {
    #[serde(default)]
    pub profile: CargoWasmBindgenProfiles,
    pub target: Option<Target>,
    #[serde(rename = "out-dir")]
    pub out_dir: Option<String>,
    pub scope: Option<String>,
    #[serde(rename = "no-typescript")]
    pub disable_dts: Option<bool>,
    #[serde(rename = "weak-refs")]
    pub weak_refs: Option<bool>,
    #[serde(rename = "reference-types")]
    pub reference_types: Option<bool>,
    #[serde(rename = "no-pack")]
    pub no_pack: Option<bool>,
    #[serde(rename = "no-opt")]
    pub no_opt: Option<bool>,
    #[serde(rename = "wasm-opt-version")]
    pub wasm_opt_version: Option<String>,
}

#[derive(Deserialize)]
pub struct CargoWasmBindgenProfiles {
    #[serde(default = "CargoWasmBindgenProfile::default_dev")]
    pub dev: CargoWasmBindgenProfile,
    #[serde(default = "CargoWasmBindgenProfile::default_release")]
    pub release: CargoWasmBindgenProfile,
    #[serde(default = "CargoWasmBindgenProfile::default_profiling")]
    pub profiling: CargoWasmBindgenProfile,
    #[serde(default = "CargoWasmBindgenProfile::default_custom")]
    pub custom: CargoWasmBindgenProfile,
}

impl Default for CargoWasmBindgenProfiles {
    fn default() -> CargoWasmBindgenProfiles {
        CargoWasmBindgenProfiles {
            dev: CargoWasmBindgenProfile::default_dev(),
            release: CargoWasmBindgenProfile::default_release(),
            profiling: CargoWasmBindgenProfile::default_profiling(),
            custom: CargoWasmBindgenProfile::default_custom(),
        }
    }
}

#[derive(Default, Deserialize)]
pub struct CargoWasmBindgenProfile {
    #[serde(default, rename = "wasm-bindgen")]
    wasm_bindgen: CargoWasmBindgenProfileWasmBindgen,
    #[serde(default, rename = "wasm-opt")]
    wasm_opt: Option<CargoWasmBindgenProfileWasmOpt>,
}

#[derive(Default, Deserialize)]
struct CargoWasmBindgenProfileWasmBindgen {
    #[serde(default, rename = "debug-js-glue")]
    debug_js_glue: Option<bool>,
    #[serde(default, rename = "demangle-name-section")]
    demangle_name_section: Option<bool>,
    #[serde(default, rename = "dwarf-debug-info")]
    dwarf_debug_info: Option<bool>,
    #[serde(default, rename = "omit-default-module-path")]
    omit_default_module_path: Option<bool>,
    #[serde(default, rename = "split-linked-modules")]
    split_linked_modules: Option<bool>,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
pub enum CargoWasmBindgenProfileWasmOpt {
    Enabled(bool),
    ExplicitArgs(Vec<String>),
}

impl Default for CargoWasmBindgenProfileWasmOpt {
    fn default() -> Self {
        CargoWasmBindgenProfileWasmOpt::Enabled(false)
    }
}

impl CargoWasmBindgenProfile {
    fn default_dev() -> Self {
        CargoWasmBindgenProfile {
            wasm_bindgen: CargoWasmBindgenProfileWasmBindgen {
                debug_js_glue: Some(true),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: None,
        }
    }

    fn default_release() -> Self {
        CargoWasmBindgenProfile {
            wasm_bindgen: CargoWasmBindgenProfileWasmBindgen {
                debug_js_glue: Some(false),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: Some(CargoWasmBindgenProfileWasmOpt::Enabled(true)),
        }
    }

    fn default_profiling() -> Self {
        CargoWasmBindgenProfile {
            wasm_bindgen: CargoWasmBindgenProfileWasmBindgen {
                debug_js_glue: Some(false),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: Some(CargoWasmBindgenProfileWasmOpt::Enabled(true)),
        }
    }

    fn default_custom() -> Self {
        CargoWasmBindgenProfile {
            wasm_bindgen: CargoWasmBindgenProfileWasmBindgen {
                debug_js_glue: Some(false),
                demangle_name_section: Some(true),
                dwarf_debug_info: Some(false),
                omit_default_module_path: Some(false),
                split_linked_modules: Some(false),
            },
            wasm_opt: Some(CargoWasmBindgenProfileWasmOpt::Enabled(true)),
        }
    }

    pub fn wasm_bindgen_debug_js_glue(&self) -> bool {
        self.wasm_bindgen.debug_js_glue.unwrap_or(false)
    }

    pub fn wasm_bindgen_demangle_name_section(&self) -> bool {
        self.wasm_bindgen.demangle_name_section.unwrap_or(true)
    }

    pub fn wasm_bindgen_dwarf_debug_info(&self) -> bool {
        self.wasm_bindgen.dwarf_debug_info.unwrap_or(false)
    }

    pub fn wasm_bindgen_omit_default_module_path(&self) -> bool {
        self.wasm_bindgen.omit_default_module_path.unwrap_or(false)
    }

    pub fn wasm_bindgen_split_linked_modules(&self) -> bool {
        self.wasm_bindgen.split_linked_modules.unwrap_or(false)
    }

    pub fn wasm_opt_args(&self) -> Option<Vec<String>> {
        match self.wasm_opt.as_ref()? {
            CargoWasmBindgenProfileWasmOpt::Enabled(false) => None,
            CargoWasmBindgenProfileWasmOpt::Enabled(true) => Some(vec!["-O".to_string()]),
            CargoWasmBindgenProfileWasmOpt::ExplicitArgs(s) => Some(s.clone()),
        }
    }
}

impl CrateData {
    pub fn new(crate_path: &Path, out_name: Option<String>) -> Result<CrateData> {
        let manifest_path = crate_path.join("Cargo.toml");
        if !manifest_path.is_file() {
            bail!(
                "crate directory is missing a `Cargo.toml` file; is `{}` the \\
                 wrong directory?",
                crate_path.display()
            )
        }

        let data = cargo_metadata::MetadataCommand::new()
            .manifest_path(&manifest_path)
            .exec()?;

        let manifest = CrateData::parse_crate_data(&manifest_path)?;

        let current_idx = data
            .packages
            .iter()
            .position(|pkg| pkg.name == manifest.package.name)
            .ok_or_else(|| anyhow!("failed to find package in metadata"))?;

        Ok(CrateData {
            data,
            manifest,
            current_idx,
            out_name,
        })
    }

    pub fn parse_crate_data(manifest_path: &Path) -> Result<CargoManifest> {
        let manifest_content = fs::read_to_string(manifest_path)
            .with_context(|| anyhow!("failed to read: {}", manifest_path.display()))?;

        let deserializer = toml::Deserializer::parse(&manifest_content)
            .with_context(|| anyhow!("failed to create TOML deserializer"))?;

        let mut unused_keys = BTreeSet::new();
        let levenshtein_threshold = 1;

        let manifest: CargoManifest = serde_ignored::deserialize(deserializer, |path| {
            let path_string = path.to_string();
            if path_string.starts_with("package.metadata")
                && (path_string.contains("wasm-bindgen")
                    || levenshtein(WASM_BINDGEN_METADATA_KEY, &path_string)
                        <= levenshtein_threshold)
            {
                unused_keys.insert(path_string);
            }
        })
        .with_context(|| anyhow!("failed to parse manifest: {}", manifest_path.display()))?;

        Ok(manifest)
    }

    pub fn target_directory(&self) -> std::path::PathBuf {
        self.data.target_directory.clone().into_std_path_buf()
    }

    pub fn configured_profile(&self, profile: BuildProfile) -> &CargoWasmBindgenProfile {
        match profile {
            BuildProfile::Dev => &self.manifest.package.metadata.wasm_bindgen.profile.dev,
            BuildProfile::Profiling => {
                &self
                    .manifest
                    .package
                    .metadata
                    .wasm_bindgen
                    .profile
                    .profiling
            }
            BuildProfile::Release => &self.manifest.package.metadata.wasm_bindgen.profile.release,
            BuildProfile::Custom(_) => &self.manifest.package.metadata.wasm_bindgen.profile.custom,
        }
    }

    pub fn wasm_bindgen_config(&self) -> &CargoWasmBindgen {
        &self.manifest.package.metadata.wasm_bindgen
    }

    pub fn check_crate_config(&self) -> Result<()> {
        self.check_crate_type()?;
        Ok(())
    }

    fn check_crate_type(&self) -> Result<()> {
        let pkg = &self.data.packages[self.current_idx];
        let any_cdylib = pkg
            .targets
            .iter()
            .filter(|target| target.kind.iter().any(|k| k == &TargetKind::CDyLib))
            .any(|target| target.crate_types.iter().any(|s| s == &CrateType::CDyLib));
        if any_cdylib {
            return Ok(());
        }
        bail!(
            "crate-type must be cdylib to compile to wasm32-unknown-unknown. Add the following to your \
             Cargo.toml file:\n\n\
             [lib]\n\
             crate-type = [\"cdylib\", \"rlib\"]"
        )
    }

    pub fn crate_name(&self) -> String {
        let pkg = &self.data.packages[self.current_idx];
        match pkg
            .targets
            .iter()
            .find(|t| t.kind.iter().any(|k| k == &TargetKind::CDyLib))
        {
            Some(lib) => lib.name.replace("-", "_"),
            None => pkg.name.replace("-", "_"),
        }
    }

    pub fn name_prefix(&self) -> String {
        match &self.out_name {
            Some(value) => value.clone(),
            None => self.crate_name(),
        }
    }

    pub fn write_package_json(
        &self,
        out_dir: &Path,
        scope: &Option<String>,
        disable_dts: bool,
        target: Target,
    ) -> Result<()> {
        let pkg_file_path = out_dir.join("package.json");
        let existing_deps = if pkg_file_path.exists() {
            Some(
                serde_json::from_str::<HashMap<String, String>>(&fs::read_to_string(
                    &pkg_file_path,
                )?)
                .context("error reading existing package.json")?,
            )
        } else {
            None
        };

        let npm_data = match target {
            Target::Nodejs => self.to_commonjs(scope, disable_dts, existing_deps, out_dir),
            Target::NoModules => self.to_nomodules(scope, disable_dts, existing_deps, out_dir),
            Target::Bundler => self.to_esmodules(scope, disable_dts, existing_deps, out_dir),
            Target::Web => self.to_web(scope, disable_dts, existing_deps, out_dir),
            Target::Deno => return Ok(()),
        };

        let npm_json = serde_json::to_string_pretty(&npm_data)?;
        fs::write(&pkg_file_path, npm_json)
            .with_context(|| anyhow!("failed to write: {}", pkg_file_path.display()))?;
        Ok(())
    }

    fn npm_data(
        &self,
        scope: &Option<String>,
        add_js_bg_to_package_json: bool,
        disable_dts: bool,
        out_dir: &Path,
    ) -> NpmData {
        let name_prefix = self.name_prefix();
        let wasm_file = format!("{}_bg.wasm", name_prefix);
        let js_file = format!("{}.js", name_prefix);
        let mut files = vec![wasm_file];

        files.push(js_file.clone());
        if add_js_bg_to_package_json {
            let js_bg_file = format!("{}_bg.js", name_prefix);
            files.push(js_bg_file);
        }

        let pkg = &self.data.packages[self.current_idx];
        let npm_name = match scope {
            Some(s) => format!("@{}/{}", s, pkg.name),
            None => pkg.name.to_string(),
        };

        let dts_file = if !disable_dts {
            let file = format!("{}.d.ts", name_prefix);
            files.push(file.to_string());
            Some(file)
        } else {
            None
        };

        let keywords = if !pkg.keywords.is_empty() {
            Some(pkg.keywords.clone())
        } else {
            None
        };

        if let Ok(entries) = fs::read_dir(out_dir) {
            let file_names = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.metadata().map(|m| m.is_file()).unwrap_or(false))
                .filter_map(|e| e.file_name().into_string().ok())
                .filter(|f| f.starts_with("LICENSE"))
                .filter(|f| f != "LICENSE");
            for file_name in file_names {
                files.push(file_name);
            }
        }

        NpmData {
            name: npm_name,
            dts_file,
            files,
            main: js_file,
            homepage: pkg.homepage.clone(),
            keywords,
        }
    }

    fn license(&self) -> Option<String> {
        let pkg = &self.data.packages[self.current_idx];
        pkg.license.clone().or_else(|| {
            pkg.license_file
                .clone()
                .map(|file| format!("SEE LICENSE IN {}", file))
        })
    }

    fn to_commonjs(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        deps: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, false, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        NpmPackage::CommonJSPackage(CommonJSPackage {
            name: data.name,
            collaborators: pkg.authors.clone(),
            description: pkg.description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: pkg.repository.clone().map(|url| Repository {
                ty: "git".into(),
                url,
            }),
            files: data.files,
            main: data.main,
            homepage: data.homepage,
            types: data.dts_file,
            keywords: data.keywords,
            dependencies: deps,
        })
    }

    fn to_esmodules(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        deps: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, true, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        NpmPackage::ESModulesPackage(ESModulesPackage {
            name: data.name,
            ty: "module".into(),
            collaborators: pkg.authors.clone(),
            description: pkg.description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: pkg.repository.clone().map(|url| Repository {
                ty: "git".into(),
                url,
            }),
            files: data.files,
            main: data.main.clone(),
            homepage: data.homepage,
            types: data.dts_file,
            side_effects: vec![format!("./{}", data.main), "./snippets/*".to_owned()],
            keywords: data.keywords,
            dependencies: deps,
        })
    }

    fn to_web(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        deps: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, false, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        NpmPackage::ESModulesPackage(ESModulesPackage {
            name: data.name,
            ty: "module".into(),
            collaborators: pkg.authors.clone(),
            description: pkg.description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: pkg.repository.clone().map(|url| Repository {
                ty: "git".into(),
                url,
            }),
            files: data.files,
            main: data.main,
            homepage: data.homepage,
            types: data.dts_file,
            side_effects: vec!["./snippets/*".to_owned()],
            keywords: data.keywords,
            dependencies: deps,
        })
    }

    fn to_nomodules(
        &self,
        scope: &Option<String>,
        disable_dts: bool,
        deps: Option<HashMap<String, String>>,
        out_dir: &Path,
    ) -> NpmPackage {
        let data = self.npm_data(scope, false, disable_dts, out_dir);
        let pkg = &self.data.packages[self.current_idx];

        NpmPackage::NoModulesPackage(NoModulesPackage {
            name: data.name,
            collaborators: pkg.authors.clone(),
            description: pkg.description.clone(),
            version: pkg.version.to_string(),
            license: self.license(),
            repository: pkg.repository.clone().map(|url| Repository {
                ty: "git".into(),
                url,
            }),
            files: data.files,
            browser: data.main,
            homepage: data.homepage,
            types: data.dts_file,
            keywords: data.keywords,
            dependencies: deps,
        })
    }
}

struct NpmData {
    name: String,
    files: Vec<String>,
    dts_file: Option<String>,
    main: String,
    homepage: Option<String>,
    keywords: Option<Vec<String>>,
}

#[allow(clippy::enum_variant_names)]
#[derive(Serialize)]
#[serde(untagged)]
pub enum NpmPackage {
    CommonJSPackage(CommonJSPackage),
    ESModulesPackage(ESModulesPackage),
    NoModulesPackage(NoModulesPackage),
}

#[derive(Serialize)]
pub struct CommonJSPackage {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub collaborators: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    pub main: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize)]
pub struct ESModulesPackage {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub collaborators: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    pub main: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(rename = "sideEffects")]
    pub side_effects: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize)]
pub struct NoModulesPackage {
    pub name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub collaborators: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    pub browser: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Serialize)]
pub struct Repository {
    #[serde(rename = "type")]
    pub ty: String,
    pub url: String,
}

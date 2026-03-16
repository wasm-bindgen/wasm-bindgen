use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::Parser;
use walkdir::WalkDir;

/// Generate wasm-bindgen Rust bindings from TypeScript .d.ts files.
#[derive(Parser, Debug)]
#[command(name = "ts-gen", version, about)]
struct Cli {
    /// One or more .d.ts files or directories to process.
    #[arg(short, long, required = true, num_args = 1..)]
    input: Vec<PathBuf>,

    /// Output file path for the generated .rs file (e.g., `src/bindings.rs`).
    #[arg(short, long)]
    output: PathBuf,

    /// JS module specifier (required when .d.ts has top-level exports).
    #[arg(long)]
    lib_name: Option<String>,

    /// External type mappings for imported types.
    ///
    /// Format: `LHS=RHS` where:
    ///   - `Blob=::web_sys::Blob` maps a specific type
    ///   - `node:buffer=node_buffer_sys` maps a module
    ///   - `node:*=node_sys::*` maps a module prefix with wildcard
    ///
    /// Multiple mappings can be comma-separated or specified multiple times.
    #[arg(short = 'e', long = "external", num_args = 1..)]
    external: Vec<String>,

    /// Print the parsed IR for debugging.
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Resolve input files
    let input_files = resolve_input_files(&cli.input)?;
    if input_files.is_empty() {
        bail!("No .d.ts files found in the specified input paths");
    }

    eprintln!("[ts-gen] Processing {} file(s)...", input_files.len());

    // Parse
    let (module, mut gctx) = ts_gen::parse::parse_dts_files(&input_files, cli.lib_name.as_deref())?;

    // Apply external type mappings
    for mapping in &cli.external {
        gctx.external_map.add_mappings(mapping);
    }

    gctx.diagnostics.emit();

    if cli.verbose {
        eprintln!("[ts-gen] Parsed {} declarations", module.types.len());
        for &type_id in &module.types {
            let decl = gctx.get_type(type_id);
            print_declaration(decl, 0);
        }
    }

    // Generate Rust code
    let rust_source = ts_gen::codegen::generate(&module, &gctx)?;

    // Write output file, creating parent directories if needed
    if let Some(parent) = cli.output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }

    std::fs::write(&cli.output, &rust_source)
        .with_context(|| format!("Failed to write {}", cli.output.display()))?;

    eprintln!("[ts-gen] Wrote {}", cli.output.display());
    eprintln!("[ts-gen] Done");

    Ok(())
}

/// Resolve input paths: expand directories to find all .d.ts files.
fn resolve_input_files(paths: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in paths {
        if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = entry?;
                if entry.file_type().is_file()
                    && entry.file_name().to_string_lossy().ends_with(".d.ts")
                {
                    files.push(entry.into_path());
                }
            }
        } else if path.is_file() {
            files.push(path.clone());
        } else {
            bail!("Input path does not exist: {}", path.display());
        }
    }
    Ok(files)
}

/// Pretty-print a declaration for debugging.
fn print_declaration(decl: &ts_gen::ir::TypeDeclaration, indent: usize) {
    let prefix = "  ".repeat(indent);
    let ctx = match &decl.module_context {
        ts_gen::ir::ModuleContext::Global => "global".to_string(),
        ts_gen::ir::ModuleContext::Module(m) => format!("module={m}"),
    };

    match &decl.kind {
        ts_gen::ir::TypeKind::Class(c) => {
            let extends = c
                .extends
                .as_ref()
                .map(|e| format!(" extends {:?}", e))
                .unwrap_or_default();
            let abstract_ = if c.is_abstract { " abstract" } else { "" };
            eprintln!(
                "{prefix}[{ctx}]{abstract_} class {} ({} members){extends}",
                c.name,
                c.members.len(),
            );
            for member in &c.members {
                print_member(member, indent + 1);
            }
        }
        ts_gen::ir::TypeKind::Interface(i) => {
            eprintln!(
                "{prefix}[{ctx}] interface {} ({} members, {:?})",
                i.name,
                i.members.len(),
                i.classification,
            );
            for member in &i.members {
                print_member(member, indent + 1);
            }
        }
        ts_gen::ir::TypeKind::TypeAlias(t) => {
            eprintln!("{prefix}[{ctx}] type {} = {:?}", t.name, t.target);
        }
        ts_gen::ir::TypeKind::StringEnum(e) => {
            let variants: Vec<_> = e.variants.iter().map(|v| &v.js_value).collect();
            eprintln!("{prefix}[{ctx}] string enum {} = {:?}", e.name, variants);
        }
        ts_gen::ir::TypeKind::NumericEnum(e) => {
            let variants: Vec<_> = e
                .variants
                .iter()
                .map(|v| format!("{} = {}", v.rust_name, v.value))
                .collect();
            eprintln!(
                "{prefix}[{ctx}] numeric enum {} = [{}]",
                e.name,
                variants.join(", ")
            );
        }
        ts_gen::ir::TypeKind::Function(f) => {
            let params: Vec<_> = f.params.iter().map(|p| &p.name).collect();
            eprintln!(
                "{prefix}[{ctx}] function {}({:?}) -> {:?}",
                f.name, params, f.return_type
            );
        }
        ts_gen::ir::TypeKind::Variable(v) => {
            let const_ = if v.is_const { "const" } else { "var" };
            eprintln!("{prefix}[{ctx}] {const_} {}: {:?}", v.name, v.type_ref);
        }
        ts_gen::ir::TypeKind::Namespace(ns) => {
            eprintln!(
                "{prefix}[{ctx}] namespace {} ({} declarations)",
                ns.name,
                ns.declarations.len()
            );
            for d in &ns.declarations {
                print_declaration(d, indent + 1);
            }
        }
    }
}

/// Pretty-print a member for debugging.
fn print_member(member: &ts_gen::ir::Member, indent: usize) {
    let prefix = "  ".repeat(indent);
    match member {
        ts_gen::ir::Member::Getter(g) => {
            let optional = if g.optional { "?" } else { "" };
            eprintln!("{prefix}getter {}{optional}: {:?}", g.js_name, g.type_ref);
        }
        ts_gen::ir::Member::Setter(s) => {
            eprintln!("{prefix}setter {}: {:?}", s.js_name, s.type_ref);
        }
        ts_gen::ir::Member::Method(m) => {
            let params: Vec<_> = m
                .params
                .iter()
                .map(|p| format!("{}: {:?}", p.name, p.type_ref))
                .collect();
            eprintln!(
                "{prefix}method {}({}): {:?}",
                m.js_name,
                params.join(", "),
                m.return_type
            );
        }
        ts_gen::ir::Member::Constructor(c) => {
            let params: Vec<_> = c
                .params
                .iter()
                .map(|p| format!("{}: {:?}", p.name, p.type_ref))
                .collect();
            eprintln!("{prefix}constructor({})", params.join(", "));
        }
        ts_gen::ir::Member::IndexSignature(i) => {
            eprintln!("{prefix}index [{:?}]: {:?}", i.key_type, i.value_type);
        }
        ts_gen::ir::Member::StaticGetter(g) => {
            eprintln!("{prefix}static getter {}: {:?}", g.js_name, g.type_ref);
        }
        ts_gen::ir::Member::StaticSetter(s) => {
            eprintln!("{prefix}static setter {}: {:?}", s.js_name, s.type_ref);
        }
        ts_gen::ir::Member::StaticMethod(m) => {
            let params: Vec<_> = m
                .params
                .iter()
                .map(|p| format!("{}: {:?}", p.name, p.type_ref))
                .collect();
            eprintln!(
                "{prefix}static method {}({}): {:?}",
                m.js_name,
                params.join(", "),
                m.return_type
            );
        }
    }
}

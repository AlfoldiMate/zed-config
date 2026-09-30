use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use keycook::parser::Severity;
use keycook::{catalog, compile, emit, lsp, manifest, parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "keycook", version, about = "Cook a Zed keymap.json from a hierarchical .kc file")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Compile a .kc file to keymap.json
    Build {
        /// Source file (default: keymap.kc)
        file: Option<PathBuf>,
        /// Output file (default: keymap.json next to the source)
        #[arg(short, long)]
        out: Option<PathBuf>,
        /// Action manifest from `zed --dump-all-actions` (default: embedded Zed 1.21.0)
        #[arg(long)]
        manifest: Option<PathBuf>,
        /// Do not insert `prefix: null` where a chord would otherwise wait
        #[arg(long)]
        no_auto_null: bool,
        /// Treat warnings as errors
        #[arg(long)]
        strict: bool,
        /// Keep source order instead of sorting sections by node depth
        #[arg(long)]
        source_order: bool,
    },
    /// Check a .kc file without writing anything
    Check {
        file: Option<PathBuf>,
        #[arg(long)]
        manifest: Option<PathBuf>,
        #[arg(long)]
        strict: bool,
    },
    /// Run the language server on stdin/stdout
    Lsp {
        #[arg(long)]
        manifest: Option<PathBuf>,
    },
    /// Search the action manifest
    Actions {
        /// Substring to match (case-insensitive), empty lists everything
        query: Option<String>,
    },
}

fn load_manifest(path: Option<PathBuf>) -> anyhow::Result<manifest::Manifest> {
    match path {
        Some(p) => {
            let text = std::fs::read_to_string(&p).with_context(|| format!("reading {}", p.display()))?;
            manifest::Manifest::parse(&text).with_context(|| format!("parsing {}", p.display()))
        }
        None => Ok(manifest::Manifest::embedded()),
    }
}

fn report(src: &str, path: &std::path::Path, diags: &[parser::Diagnostic]) -> (usize, usize) {
    let idx = lsp::LineIndex::new(src);
    let mut errors = 0;
    let mut warnings = 0;
    for d in diags {
        let pos = idx.position(d.span.start);
        let tag = match d.severity {
            Severity::Error => {
                errors += 1;
                "error"
            }
            Severity::Warning => {
                warnings += 1;
                "warning"
            }
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        eprintln!("{}:{}:{}: {tag}: {}", path.display(), pos.line + 1, pos.character + 1, d.message);
    }
    (errors, warnings)
}

fn compile_file(file: Option<PathBuf>, manifest_path: Option<PathBuf>, auto_null: bool, sort: bool) -> anyhow::Result<(PathBuf, String, compile::Output)> {
    let path = file.unwrap_or_else(|| PathBuf::from("keymap.kc"));
    let src = std::fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
    let manifest = load_manifest(manifest_path)?;
    let catalog = catalog::Catalog::embedded();
    let out = compile::compile(&src, &manifest, &catalog, &compile::Options { auto_null, sort });
    Ok((path, src, out))
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Build { file, out, manifest, no_auto_null, strict, source_order } => {
            let (path, src, output) = compile_file(file, manifest, !no_auto_null, !source_order)?;
            let (errors, warnings) = report(&src, &path, &output.diags);
            if errors > 0 || (strict && warnings > 0) {
                bail!("{errors} error(s), {warnings} warning(s); nothing written");
            }
            let out_path = out.unwrap_or_else(|| path.with_file_name("keymap.json"));
            let json = emit::render(&output.sections, &path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default());
            std::fs::write(&out_path, json).with_context(|| format!("writing {}", out_path.display()))?;
            let bindings: usize = output.sections.iter().map(|s| s.bindings.len()).sum();
            let sections = output.sections.iter().filter(|s| !s.bindings.is_empty()).count();
            eprintln!("{}: {bindings} bindings in {sections} sections ({warnings} warning(s))", out_path.display());
        }
        Cmd::Check { file, manifest, strict } => {
            let (path, src, output) = compile_file(file, manifest, true, true)?;
            let (errors, warnings) = report(&src, &path, &output.diags);
            let bindings: usize = output.sections.iter().map(|s| s.bindings.len()).sum();
            eprintln!("{}: {bindings} bindings, {errors} error(s), {warnings} warning(s)", path.display());
            if errors > 0 || (strict && warnings > 0) {
                std::process::exit(1);
            }
        }
        Cmd::Lsp { manifest } => {
            let m = load_manifest(manifest)?;
            let c = catalog::Catalog::embedded();
            tokio::runtime::Builder::new_multi_thread().enable_all().build()?.block_on(lsp::run(m, c));
        }
        Cmd::Actions { query } => {
            let m = manifest::Manifest::embedded();
            let q = query.unwrap_or_default().to_lowercase();
            for name in &m.sorted_names {
                if q.is_empty() || name.to_lowercase().contains(&q) {
                    let a = &m.actions[name];
                    let sig = a.schema.as_ref().map(|s| format!("({})", manifest::summarize(s))).unwrap_or_default();
                    let doc = a.doc.lines().next().unwrap_or("");
                    println!("{name}{sig}  {doc}");
                }
            }
        }
    }
    Ok(())
}

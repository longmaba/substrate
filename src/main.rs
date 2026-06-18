use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Instant;

const STORE_DIR: &str = ".substrate";
const STORE_METADATA: &str = "store.toml";
const STORE_VERSION: u32 = 1;
const BENCH_CHUNK_SIZE: usize = 128;
const DEFAULT_IGNORED_DIRS: &[&str] = &[
    STORE_DIR,
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    "coverage",
    ".cache",
    ".next",
    ".turbo",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct StoreMetadata {
    version: u32,
    root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    Init {
        path: PathBuf,
    },
    Status {
        path: PathBuf,
    },
    Bench {
        fixture_path: PathBuf,
    },
    Ingest {
        path: PathBuf,
    },
    Project {
        state_id: String,
        out: PathBuf,
    },
    Diff {
        left: PathBuf,
        right: PathBuf,
    },
    State {
        state_id: String,
    },
    Verify {
        state_id: String,
        out: PathBuf,
        bench: PathBuf,
    },
}

fn main() {
    let exit_code = run(env::args().skip(1), &mut io::stdout(), &mut io::stderr());
    std::process::exit(exit_code);
}

fn run<I, S>(args: I, stdout: &mut dyn Write, stderr: &mut dyn Write) -> i32
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    match parse_command(args.into_iter().map(Into::into)) {
        Ok(Command::Init { path }) => match init_store(&path) {
            Ok(metadata) => {
                let _ = writeln!(stdout, "initialized: yes");
                let _ = writeln!(stdout, "root: {}", metadata.root.display());
                let _ = writeln!(stdout, "store: {}", store_dir(&metadata.root).display());
                0
            }
            Err(error) => {
                let _ = writeln!(stderr, "error: {error}");
                1
            }
        },
        Ok(Command::Status { path }) => match status(&path) {
            Ok(Some(metadata)) => {
                let _ = writeln!(stdout, "initialized: yes");
                let _ = writeln!(stdout, "root: {}", metadata.root.display());
                let _ = writeln!(stdout, "store: {}", store_dir(&metadata.root).display());
                let _ = writeln!(stdout, "version: {}", metadata.version);
                0
            }
            Ok(None) => {
                let root = normalize_status_root(&path);
                let _ = writeln!(stdout, "initialized: no");
                let _ = writeln!(stdout, "root: {}", root.display());
                let _ = writeln!(stdout, "store: {}", store_dir(&root).display());
                0
            }
            Err(error) => {
                let _ = writeln!(stderr, "error: {error}");
                1
            }
        },
        Ok(Command::Bench { fixture_path }) => match run_storage_benchmark(&fixture_path) {
            Ok(report) => {
                let _ = write!(stdout, "{}", report.to_text());
                0
            }
            Err(error) => {
                let _ = writeln!(stderr, "error: {error}");
                1
            }
        },
        Ok(Command::Ingest { path }) => match ingest_working_tree(&path) {
            Ok(report) => {
                let _ = write!(stdout, "{}", report.to_text());
                0
            }
            Err(error) => {
                let _ = writeln!(stderr, "error: {error}");
                1
            }
        },
        Ok(Command::Project { state_id, out }) => {
            let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            match project_state(&root, &state_id, &out) {
                Ok(report) => {
                    let _ = write!(stdout, "{}", report.to_text());
                    0
                }
                Err(error) => {
                    let _ = writeln!(stderr, "error: {error}");
                    1
                }
            }
        }
        Ok(Command::Diff { left, right }) => match diff_paths(&left, &right) {
            Ok(report) => {
                let _ = write!(stdout, "{}", report.to_text());
                0
            }
            Err(error) => {
                let _ = writeln!(stderr, "error: {error}");
                1
            }
        },
        Ok(Command::State { state_id }) => {
            let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            match state_report(&root, &state_id) {
                Ok(report) => {
                    let _ = write!(stdout, "{}", report.to_text());
                    0
                }
                Err(error) => {
                    let _ = writeln!(stderr, "error: {error}");
                    1
                }
            }
        }
        Ok(Command::Verify {
            state_id,
            out,
            bench,
        }) => {
            let root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            match verify_state(&root, &state_id, &out, &bench) {
                Ok(report) => {
                    let _ = write!(stdout, "{}", report.to_text());
                    if report.label == StateLabel::VerifiedGreen {
                        0
                    } else {
                        1
                    }
                }
                Err(error) => {
                    let _ = writeln!(stderr, "error: {error}");
                    1
                }
            }
        }
        Err(error) => {
            let _ = writeln!(stderr, "error: {error}");
            let _ = writeln!(
                stderr,
                "usage: substrate <init <path>|status [path]|ingest <path>|project <state-id> --out <path>|state <state-id>|verify <state-id> --out <path> --bench <fixture-path>|diff <left> <right>|bench <fixture-path>>"
            );
            2
        }
    }
}

fn parse_command<I>(mut args: I) -> Result<Command, String>
where
    I: Iterator<Item = String>,
{
    let Some(command) = args.next() else {
        return Err("missing command".to_string());
    };

    match command.as_str() {
        "init" => {
            let Some(path) = args.next() else {
                return Err("init requires <path>".to_string());
            };
            reject_extra_args(args)?;
            Ok(Command::Init {
                path: PathBuf::from(path),
            })
        }
        "status" => {
            let path = args.next().unwrap_or_else(|| ".".to_string());
            reject_extra_args(args)?;
            Ok(Command::Status {
                path: PathBuf::from(path),
            })
        }
        "bench" => {
            let Some(path) = args.next() else {
                return Err("bench requires <fixture-path>".to_string());
            };
            reject_extra_args(args)?;
            Ok(Command::Bench {
                fixture_path: PathBuf::from(path),
            })
        }
        "ingest" => {
            let Some(path) = args.next() else {
                return Err("ingest requires <path>".to_string());
            };
            reject_extra_args(args)?;
            Ok(Command::Ingest {
                path: PathBuf::from(path),
            })
        }
        "project" => {
            let Some(state_id) = args.next() else {
                return Err("project requires <state-id>".to_string());
            };
            let Some(flag) = args.next() else {
                return Err("project requires --out <path>".to_string());
            };
            if flag != "--out" {
                return Err(format!("project expected --out, got `{flag}`"));
            }
            let Some(path) = args.next() else {
                return Err("project requires --out <path>".to_string());
            };
            reject_extra_args(args)?;
            Ok(Command::Project {
                state_id,
                out: PathBuf::from(path),
            })
        }
        "diff" => {
            let Some(left) = args.next() else {
                return Err("diff requires <left> <right>".to_string());
            };
            let Some(right) = args.next() else {
                return Err("diff requires <left> <right>".to_string());
            };
            reject_extra_args(args)?;
            Ok(Command::Diff {
                left: PathBuf::from(left),
                right: PathBuf::from(right),
            })
        }
        "state" => {
            let Some(state_id) = args.next() else {
                return Err("state requires <state-id>".to_string());
            };
            reject_extra_args(args)?;
            Ok(Command::State { state_id })
        }
        "verify" => {
            let Some(state_id) = args.next() else {
                return Err("verify requires <state-id>".to_string());
            };
            let mut out = None;
            let mut bench = None;
            while let Some(flag) = args.next() {
                let Some(value) = args.next() else {
                    return Err(format!("verify requires a value after `{flag}`"));
                };
                match flag.as_str() {
                    "--out" => out = Some(PathBuf::from(value)),
                    "--bench" => bench = Some(PathBuf::from(value)),
                    _ => return Err(format!("verify unknown option `{flag}`")),
                }
            }
            Ok(Command::Verify {
                state_id,
                out: out.ok_or_else(|| "verify requires --out <path>".to_string())?,
                bench: bench.ok_or_else(|| "verify requires --bench <fixture-path>".to_string())?,
            })
        }
        _ => Err(format!("unknown command `{command}`")),
    }
}

fn reject_extra_args<I>(mut args: I) -> Result<(), String>
where
    I: Iterator<Item = String>,
{
    match args.next() {
        Some(extra) => Err(format!("unexpected argument `{extra}`")),
        None => Ok(()),
    }
}

fn init_store(path: &Path) -> Result<StoreMetadata, String> {
    fs::create_dir_all(path)
        .map_err(|error| format!("failed to create root `{}`: {error}", path.display()))?;
    let root = path
        .canonicalize()
        .map_err(|error| format!("failed to resolve root `{}`: {error}", path.display()))?;
    let store = store_dir(&root);
    let metadata_path = metadata_path(&root);

    if metadata_path.exists() {
        return read_metadata(&root);
    }

    if store.exists() && !store.is_dir() {
        return Err(format!(
            "store path is not a directory: {}",
            store.display()
        ));
    }

    fs::create_dir_all(&store)
        .map_err(|error| format!("failed to create store `{}`: {error}", store.display()))?;

    let metadata = StoreMetadata {
        version: STORE_VERSION,
        root,
    };
    fs::write(&metadata_path, metadata.to_store_toml()).map_err(|error| {
        format!(
            "failed to write store metadata `{}`: {error}",
            metadata_path.display()
        )
    })?;
    Ok(metadata)
}

fn status(path: &Path) -> Result<Option<StoreMetadata>, String> {
    let root = normalize_status_root(path);
    let metadata = metadata_path(&root);
    if !metadata.exists() {
        return Ok(None);
    }
    read_metadata(&root).map(Some)
}

fn read_metadata(root: &Path) -> Result<StoreMetadata, String> {
    let metadata = metadata_path(root);
    let raw = fs::read_to_string(&metadata)
        .map_err(|error| format!("failed to read metadata `{}`: {error}", metadata.display()))?;
    StoreMetadata::from_store_toml(&raw)
        .map_err(|error| format!("invalid store metadata `{}`: {error}", metadata.display()))
}

fn normalize_status_root(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn store_dir(root: &Path) -> PathBuf {
    root.join(STORE_DIR)
}

fn metadata_path(root: &Path) -> PathBuf {
    store_dir(root).join(STORE_METADATA)
}

fn objects_dir(root: &Path) -> PathBuf {
    store_dir(root).join("objects")
}

fn states_dir(root: &Path) -> PathBuf {
    store_dir(root).join("states")
}

fn object_path(root: &Path, content_id: &str) -> PathBuf {
    objects_dir(root).join(content_id)
}

fn state_path(root: &Path, state_id: &str) -> PathBuf {
    states_dir(root).join(format!("{state_id}.manifest"))
}

fn verification_dir(root: &Path) -> PathBuf {
    store_dir(root).join("verification")
}

fn verification_path(root: &Path, state_id: &str) -> PathBuf {
    verification_dir(root).join(format!("{state_id}.txt"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestEntry {
    path: String,
    content_id: String,
    byte_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Manifest {
    entries: Vec<ManifestEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IngestReport {
    root: PathBuf,
    state_id: String,
    file_count: usize,
    object_count: usize,
    skipped_binary_count: usize,
    manifest: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectReport {
    root: PathBuf,
    out: PathBuf,
    state_id: String,
    file_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StateLabel {
    Candidate,
    VerifiedGreen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerificationGate {
    name: String,
    passed: bool,
    detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerificationMetadata {
    state_id: String,
    label: StateLabel,
    gates: Vec<VerificationGate>,
    evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StateReport {
    root: PathBuf,
    metadata: VerificationMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerifyReport {
    root: PathBuf,
    state_id: String,
    label: StateLabel,
    gates: Vec<VerificationGate>,
    metadata: PathBuf,
}

fn ingest_working_tree(path: &Path) -> Result<IngestReport, String> {
    let root = path
        .canonicalize()
        .map_err(|error| format!("failed to resolve root `{}`: {error}", path.display()))?;
    if !metadata_path(&root).exists() {
        return Err(format!("not initialized: {}", root.display()));
    }
    read_metadata(&root)?;

    fs::create_dir_all(objects_dir(&root)).map_err(|error| {
        format!(
            "failed to create objects directory `{}`: {error}",
            objects_dir(&root).display()
        )
    })?;
    fs::create_dir_all(states_dir(&root)).map_err(|error| {
        format!(
            "failed to create states directory `{}`: {error}",
            states_dir(&root).display()
        )
    })?;

    let mut entries = Vec::new();
    let mut skipped_binary_count = 0usize;
    let mut object_ids = HashSet::new();

    for file in collect_working_tree_files(&root)? {
        let bytes = fs::read(&file)
            .map_err(|error| format!("failed to read `{}`: {error}", file.display()))?;
        if !is_supported_text(&bytes) {
            skipped_binary_count += 1;
            continue;
        }

        let content_id = content_id(&bytes);
        let object = object_path(&root, &content_id);
        if !object.exists() {
            fs::write(&object, &bytes).map_err(|error| {
                format!("failed to write object `{}`: {error}", object.display())
            })?;
        }
        object_ids.insert(content_id.clone());
        entries.push(ManifestEntry {
            path: relative_manifest_path(&root, &file)?,
            content_id,
            byte_len: bytes.len() as u64,
        });
    }

    entries.sort_by(|left, right| left.path.cmp(&right.path));
    let manifest = Manifest { entries };
    let manifest_text = manifest.to_text();
    let state_id = state_id(&manifest_text);
    let manifest_path = state_path(&root, &state_id);
    fs::write(&manifest_path, manifest_text).map_err(|error| {
        format!(
            "failed to write manifest `{}`: {error}",
            manifest_path.display()
        )
    })?;
    write_verification_metadata(&root, &candidate_metadata(&state_id))?;

    Ok(IngestReport {
        root,
        state_id,
        file_count: manifest.entries.len(),
        object_count: object_ids.len(),
        skipped_binary_count,
        manifest: manifest_path,
    })
}

fn project_state(root: &Path, state_id: &str, out: &Path) -> Result<ProjectReport, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("failed to resolve root `{}`: {error}", root.display()))?;
    read_metadata(&root)?;
    let manifest_path = state_path(&root, state_id);
    let manifest_text = fs::read_to_string(&manifest_path).map_err(|error| {
        format!(
            "failed to read manifest `{}`: {error}",
            manifest_path.display()
        )
    })?;
    let manifest = Manifest::from_text(&manifest_text)?;

    fs::create_dir_all(out)
        .map_err(|error| format!("failed to create output `{}`: {error}", out.display()))?;
    let out = out
        .canonicalize()
        .map_err(|error| format!("failed to resolve output `{}`: {error}", out.display()))?;

    for entry in &manifest.entries {
        let bytes = fs::read(object_path(&root, &entry.content_id)).map_err(|error| {
            format!(
                "failed to read object `{}`: {error}",
                object_path(&root, &entry.content_id).display()
            )
        })?;
        if bytes.len() as u64 != entry.byte_len {
            return Err(format!(
                "object length mismatch for {}: expected {}, got {}",
                entry.path,
                entry.byte_len,
                bytes.len()
            ));
        }
        let target = out.join(path_from_manifest(&entry.path)?);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create projection directory `{}`: {error}",
                    parent.display()
                )
            })?;
        }
        fs::write(&target, bytes).map_err(|error| {
            format!("failed to write projection `{}`: {error}", target.display())
        })?;
    }

    Ok(ProjectReport {
        root,
        out,
        state_id: state_id.to_string(),
        file_count: manifest.entries.len(),
    })
}

fn collect_working_tree_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let ignore_rules = IgnoreRules::load(root);
    collect_working_tree_files_into(root, root, &ignore_rules, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_working_tree_files_into(
    root: &Path,
    current: &Path,
    ignore_rules: &IgnoreRules,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(current)
        .map_err(|error| format!("failed to read directory `{}`: {error}", current.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read directory entry: {error}"))?;
        let path = entry.path();
        if path.is_dir() {
            if should_ignore_working_tree_path(root, &path, true, ignore_rules)? {
                continue;
            }
            collect_working_tree_files_into(root, &path, ignore_rules, files)?;
        } else if path.is_file() {
            if should_ignore_working_tree_path(root, &path, false, ignore_rules)? {
                continue;
            }
            files.push(path);
        }
    }
    Ok(())
}

fn should_ignore_working_tree_path(
    root: &Path,
    path: &Path,
    is_dir: bool,
    ignore_rules: &IgnoreRules,
) -> Result<bool, String> {
    let relative = relative_manifest_path(root, path)?;
    if relative
        .split('/')
        .any(|part| DEFAULT_IGNORED_DIRS.contains(&part))
    {
        return Ok(true);
    }
    Ok(ignore_rules.is_ignored(&relative, is_dir))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IgnoreRules {
    patterns: Vec<IgnorePattern>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IgnorePattern {
    pattern: String,
    negated: bool,
    directory_only: bool,
    anchored: bool,
    has_slash: bool,
}

impl IgnoreRules {
    fn load(root: &Path) -> Self {
        let raw = fs::read_to_string(root.join(".gitignore")).unwrap_or_default();
        Self::parse(&raw)
    }

    fn parse(raw: &str) -> Self {
        let patterns = raw.lines().filter_map(IgnorePattern::parse).collect();
        Self { patterns }
    }

    fn is_ignored(&self, relative: &str, is_dir: bool) -> bool {
        let mut ignored = false;
        for pattern in &self.patterns {
            if pattern.matches(relative, is_dir) {
                ignored = !pattern.negated;
            }
        }
        ignored
    }
}

impl IgnorePattern {
    fn parse(line: &str) -> Option<Self> {
        let mut pattern = line.trim();
        if pattern.is_empty() || pattern.starts_with('#') {
            return None;
        }

        let negated = pattern.starts_with('!');
        if negated {
            pattern = pattern[1..].trim_start();
        }
        if pattern.is_empty() {
            return None;
        }

        let anchored = pattern.starts_with('/');
        if anchored {
            pattern = &pattern[1..];
        }
        let directory_only = pattern.ends_with('/');
        if directory_only {
            pattern = pattern.trim_end_matches('/');
        }
        if pattern.is_empty() {
            return None;
        }

        let pattern = pattern.replace('\\', "/");
        let has_slash = pattern.contains('/');
        Some(Self {
            pattern,
            negated,
            directory_only,
            anchored,
            has_slash,
        })
    }

    fn matches(&self, relative: &str, is_dir: bool) -> bool {
        if self.directory_only && !is_dir {
            return false;
        }
        if self.has_slash || self.anchored {
            return glob_match(&self.pattern, relative);
        }

        relative
            .split('/')
            .any(|part| glob_match(&self.pattern, part))
    }
}

fn glob_match(pattern: &str, text: &str) -> bool {
    let pattern = pattern.as_bytes();
    let text = text.as_bytes();
    let (mut p, mut t) = (0usize, 0usize);
    let mut star = None;
    let mut star_text = 0usize;

    while t < text.len() {
        if p < pattern.len() && (pattern[p] == b'?' || pattern[p] == text[t]) {
            p += 1;
            t += 1;
        } else if p < pattern.len() && pattern[p] == b'*' {
            star = Some(p);
            p += 1;
            star_text = t;
        } else if let Some(star_index) = star {
            p = star_index + 1;
            star_text += 1;
            t = star_text;
        } else {
            return false;
        }
    }

    while p < pattern.len() && pattern[p] == b'*' {
        p += 1;
    }
    p == pattern.len()
}

fn is_supported_text(bytes: &[u8]) -> bool {
    !bytes.contains(&0)
}

fn content_id(bytes: &[u8]) -> String {
    format!("{:016x}-{}", stable_hash(bytes), bytes.len())
}

fn state_id(manifest_text: &str) -> String {
    format!("s{:016x}", stable_hash(manifest_text.as_bytes()))
}

fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in bytes {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn relative_manifest_path(root: &Path, file: &Path) -> Result<String, String> {
    let relative = file.strip_prefix(root).map_err(|error| {
        format!(
            "failed to make `{}` relative to `{}`: {error}",
            file.display(),
            root.display()
        )
    })?;
    let parts: Vec<String> = relative
        .components()
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect();
    Ok(parts.join("/"))
}

fn path_from_manifest(path: &str) -> Result<PathBuf, String> {
    if path.is_empty() || path.contains("..") || path.starts_with('/') || path.starts_with('\\') {
        return Err(format!("invalid manifest path `{path}`"));
    }
    let mut output = PathBuf::new();
    for part in path.split('/') {
        if part.is_empty() || part == "." || part == ".." {
            return Err(format!("invalid manifest path `{path}`"));
        }
        output.push(part);
    }
    Ok(output)
}

fn write_verification_metadata(
    root: &Path,
    metadata: &VerificationMetadata,
) -> Result<PathBuf, String> {
    fs::create_dir_all(verification_dir(root)).map_err(|error| {
        format!(
            "failed to create verification directory `{}`: {error}",
            verification_dir(root).display()
        )
    })?;
    let path = verification_path(root, &metadata.state_id);
    fs::write(&path, metadata.to_text()).map_err(|error| {
        format!(
            "failed to write verification metadata `{}`: {error}",
            path.display()
        )
    })?;
    Ok(path)
}

fn read_verification_metadata(root: &Path, state_id: &str) -> Result<VerificationMetadata, String> {
    let path = verification_path(root, state_id);
    let raw = fs::read_to_string(&path).map_err(|error| {
        format!(
            "failed to read verification metadata `{}`: {error}",
            path.display()
        )
    })?;
    VerificationMetadata::from_text(&raw)
}

fn candidate_metadata(state_id: &str) -> VerificationMetadata {
    VerificationMetadata {
        state_id: state_id.to_string(),
        label: StateLabel::Candidate,
        gates: Vec::new(),
        evidence: vec!["created_by=ingest".to_string()],
    }
}

fn state_report(root: &Path, state_id: &str) -> Result<StateReport, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("failed to resolve root `{}`: {error}", root.display()))?;
    read_metadata(&root)?;
    let metadata = read_verification_metadata(&root, state_id)?;
    Ok(StateReport { root, metadata })
}

fn verify_state(
    root: &Path,
    state_id: &str,
    out: &Path,
    bench: &Path,
) -> Result<VerifyReport, String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("failed to resolve root `{}`: {error}", root.display()))?;
    read_metadata(&root)?;

    let mut gates = Vec::new();
    let manifest_path = state_path(&root, state_id);
    let manifest_text = fs::read_to_string(&manifest_path);
    let manifest = match manifest_text {
        Ok(raw) => match Manifest::from_text(&raw) {
            Ok(manifest) => {
                gates.push(pass_gate("manifest_parse", "manifest parsed"));
                Some(manifest)
            }
            Err(error) => {
                gates.push(fail_gate("manifest_parse", &error));
                None
            }
        },
        Err(error) => {
            gates.push(fail_gate(
                "manifest_parse",
                &format!(
                    "failed to read manifest `{}`: {error}",
                    manifest_path.display()
                ),
            ));
            None
        }
    };

    if let Some(manifest) = &manifest {
        gates.push(check_object_integrity(&root, manifest));
        if gates.iter().all(|gate| gate.passed) {
            gates.push(check_projection_stability(&root, state_id, out, manifest));
        } else {
            gates.push(fail_gate(
                "projection_stability",
                "skipped because object integrity failed",
            ));
        }
    } else {
        gates.push(fail_gate(
            "object_integrity",
            "skipped because manifest parse failed",
        ));
        gates.push(fail_gate(
            "projection_stability",
            "skipped because manifest parse failed",
        ));
    }

    match run_storage_benchmark(bench) {
        Ok(report) => gates.push(pass_gate(
            "benchmark_completion",
            &format!(
                "revision_count={} file_count={} dedup_ratio={:.4}",
                report.revision_count, report.file_count, report.dedup_ratio
            ),
        )),
        Err(error) => gates.push(fail_gate("benchmark_completion", &error)),
    }

    let label = if gates.iter().all(|gate| gate.passed) {
        StateLabel::VerifiedGreen
    } else {
        StateLabel::Candidate
    };
    let metadata = VerificationMetadata {
        state_id: state_id.to_string(),
        label,
        gates: gates.clone(),
        evidence: vec![
            format!("bench={}", bench.display()),
            format!("projection_out={}", out.display()),
        ],
    };
    let metadata_path = write_verification_metadata(&root, &metadata)?;

    Ok(VerifyReport {
        root,
        state_id: state_id.to_string(),
        label,
        gates,
        metadata: metadata_path,
    })
}

fn pass_gate(name: &str, detail: &str) -> VerificationGate {
    VerificationGate {
        name: name.to_string(),
        passed: true,
        detail: detail.to_string(),
    }
}

fn fail_gate(name: &str, detail: &str) -> VerificationGate {
    VerificationGate {
        name: name.to_string(),
        passed: false,
        detail: detail.to_string(),
    }
}

fn check_object_integrity(root: &Path, manifest: &Manifest) -> VerificationGate {
    for entry in &manifest.entries {
        let object = object_path(root, &entry.content_id);
        let bytes = match fs::read(&object) {
            Ok(bytes) => bytes,
            Err(error) => {
                return fail_gate(
                    "object_integrity",
                    &format!("failed to read object `{}`: {error}", object.display()),
                );
            }
        };
        if bytes.len() as u64 != entry.byte_len {
            return fail_gate(
                "object_integrity",
                &format!(
                    "{} byte length mismatch: expected {}, got {}",
                    entry.path,
                    entry.byte_len,
                    bytes.len()
                ),
            );
        }
        let actual_content_id = content_id(&bytes);
        if actual_content_id != entry.content_id {
            return fail_gate(
                "object_integrity",
                &format!(
                    "{} content id mismatch: expected {}, got {}",
                    entry.path, entry.content_id, actual_content_id
                ),
            );
        }
    }
    pass_gate(
        "object_integrity",
        &format!("{} manifest entries verified", manifest.entries.len()),
    )
}

fn check_projection_stability(
    root: &Path,
    state_id: &str,
    out: &Path,
    manifest: &Manifest,
) -> VerificationGate {
    if let Err(error) = project_state(root, state_id, out) {
        return fail_gate("projection_stability", &error);
    }
    let out = match out.canonicalize() {
        Ok(path) => path,
        Err(error) => {
            return fail_gate(
                "projection_stability",
                &format!("failed to resolve output `{}`: {error}", out.display()),
            );
        }
    };
    for entry in &manifest.entries {
        let relative = match path_from_manifest(&entry.path) {
            Ok(path) => path,
            Err(error) => return fail_gate("projection_stability", &error),
        };
        let source = root.join(&relative);
        let projected = out.join(&relative);
        let source_bytes = match fs::read(&source) {
            Ok(bytes) => bytes,
            Err(error) => {
                return fail_gate(
                    "projection_stability",
                    &format!("failed to read source `{}`: {error}", source.display()),
                );
            }
        };
        let projected_bytes = match fs::read(&projected) {
            Ok(bytes) => bytes,
            Err(error) => {
                return fail_gate(
                    "projection_stability",
                    &format!(
                        "failed to read projected `{}`: {error}",
                        projected.display()
                    ),
                );
            }
        };
        if source_bytes != projected_bytes {
            return fail_gate(
                "projection_stability",
                &format!("{} differs between source and projection", entry.path),
            );
        }
    }
    pass_gate(
        "projection_stability",
        &format!("{} projected files matched source", manifest.entries.len()),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiffReport {
    left: PathBuf,
    right: PathBuf,
    pair_count: usize,
    rust_pair_count: usize,
    typescript_pair_count: usize,
    javascript_pair_count: usize,
    python_pair_count: usize,
    csharp_pair_count: usize,
    line_diff_changed_lines: usize,
    normalized_changed_node_count: usize,
    unsupported_file_fallback_count: usize,
    pairs: Vec<DiffPairReport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiffPairReport {
    path: String,
    classification: String,
    note: String,
    line_changed_lines: usize,
    normalized_changed_nodes: usize,
    unsupported_fallback: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiffNote {
    classification: String,
    note: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiffLanguage {
    Rust,
    TypeScript,
    JavaScript,
    Python,
    CSharp,
}

#[derive(Clone, Copy)]
struct DiffLanguageSpec {
    language: DiffLanguage,
    extensions: &'static [&'static str],
    changed_node_count: fn(&[u8], &[u8]) -> Result<usize, String>,
}

const DIFF_LANGUAGE_REGISTRY: &[DiffLanguageSpec] = &[
    DiffLanguageSpec {
        language: DiffLanguage::Rust,
        extensions: &[".rs"],
        changed_node_count: normalized_rust_changed_node_count_result,
    },
    DiffLanguageSpec {
        language: DiffLanguage::TypeScript,
        extensions: &[".ts", ".tsx"],
        changed_node_count: normalized_typescript_changed_node_count,
    },
    DiffLanguageSpec {
        language: DiffLanguage::JavaScript,
        extensions: &[".js", ".jsx"],
        changed_node_count: normalized_javascript_changed_node_count,
    },
    DiffLanguageSpec {
        language: DiffLanguage::Python,
        extensions: &[".py"],
        changed_node_count: normalized_python_changed_node_count,
    },
    DiffLanguageSpec {
        language: DiffLanguage::CSharp,
        extensions: &[".cs"],
        changed_node_count: normalized_csharp_changed_node_count,
    },
];

fn diff_paths(left: &Path, right: &Path) -> Result<DiffReport, String> {
    let left = left
        .canonicalize()
        .map_err(|error| format!("failed to resolve left `{}`: {error}", left.display()))?;
    let right = right
        .canonicalize()
        .map_err(|error| format!("failed to resolve right `{}`: {error}", right.display()))?;
    let notes = load_diff_notes(&left, &right);
    let pairs = diff_file_pairs(&left, &right)?;

    let mut pair_reports = Vec::new();
    let mut rust_pair_count = 0usize;
    let mut typescript_pair_count = 0usize;
    let mut javascript_pair_count = 0usize;
    let mut python_pair_count = 0usize;
    let mut csharp_pair_count = 0usize;
    let mut line_diff_changed_lines = 0usize;
    let mut normalized_changed_node_count = 0usize;
    let mut unsupported_file_fallback_count = 0usize;

    for pair in pairs {
        let left_bytes = fs::read(&pair.left)
            .map_err(|error| format!("failed to read `{}`: {error}", pair.left.display()))?;
        let right_bytes = fs::read(&pair.right)
            .map_err(|error| format!("failed to read `{}`: {error}", pair.right.display()))?;
        let line_changes = changed_line_count(&left_bytes, &right_bytes);
        let note = notes.get(&pair.relative).cloned().unwrap_or(DiffNote {
            classification: "unclassified".to_string(),
            note: String::new(),
        });

        let (normalized_changes, unsupported) = match diff_language_for_path(&pair.relative) {
            Some(spec) => {
                match spec.language {
                    DiffLanguage::Rust => rust_pair_count += 1,
                    DiffLanguage::TypeScript => typescript_pair_count += 1,
                    DiffLanguage::JavaScript => javascript_pair_count += 1,
                    DiffLanguage::Python => python_pair_count += 1,
                    DiffLanguage::CSharp => csharp_pair_count += 1,
                }
                ((spec.changed_node_count)(&left_bytes, &right_bytes)?, false)
            }
            None => {
                unsupported_file_fallback_count += 1;
                (0, true)
            }
        };

        line_diff_changed_lines += line_changes;
        normalized_changed_node_count += normalized_changes;
        pair_reports.push(DiffPairReport {
            path: pair.relative,
            classification: note.classification,
            note: note.note,
            line_changed_lines: line_changes,
            normalized_changed_nodes: normalized_changes,
            unsupported_fallback: unsupported,
        });
    }

    Ok(DiffReport {
        left,
        right,
        pair_count: pair_reports.len(),
        rust_pair_count,
        typescript_pair_count,
        javascript_pair_count,
        python_pair_count,
        csharp_pair_count,
        line_diff_changed_lines,
        normalized_changed_node_count,
        unsupported_file_fallback_count,
        pairs: pair_reports,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DiffFilePair {
    relative: String,
    left: PathBuf,
    right: PathBuf,
}

fn diff_file_pairs(left: &Path, right: &Path) -> Result<Vec<DiffFilePair>, String> {
    if left.is_file() && right.is_file() {
        let relative = left
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| "file".to_string());
        return Ok(vec![DiffFilePair {
            relative,
            left: left.to_path_buf(),
            right: right.to_path_buf(),
        }]);
    }
    if !left.is_dir() || !right.is_dir() {
        return Err("diff inputs must both be files or both be directories".to_string());
    }

    let mut left_files = collect_relative_files(left)?;
    let right_files = collect_relative_files(right)?;
    left_files.retain(|relative| right.join(path_from_manifest(relative).unwrap()).is_file());
    let mut pairs = Vec::new();
    for relative in left_files {
        if right_files.contains(&relative) {
            pairs.push(DiffFilePair {
                left: left.join(path_from_manifest(&relative)?),
                right: right.join(path_from_manifest(&relative)?),
                relative,
            });
        }
    }
    pairs.sort_by(|a, b| a.relative.cmp(&b.relative));
    Ok(pairs)
}

fn collect_relative_files(root: &Path) -> Result<Vec<String>, String> {
    let mut files = collect_files(root)?;
    let mut relative = Vec::new();
    files.sort();
    for file in files {
        relative.push(relative_manifest_path(root, &file)?);
    }
    relative.sort();
    Ok(relative)
}

fn load_diff_notes(left: &Path, right: &Path) -> std::collections::HashMap<String, DiffNote> {
    let mut candidates = Vec::new();
    if let Some(parent) = left.parent() {
        candidates.push(parent.join("NOTES.tsv"));
    }
    if let Some(parent) = right.parent() {
        candidates.push(parent.join("NOTES.tsv"));
    }

    for candidate in candidates {
        if let Ok(raw) = fs::read_to_string(candidate) {
            return parse_diff_notes(&raw);
        }
    }
    std::collections::HashMap::new()
}

fn parse_diff_notes(raw: &str) -> std::collections::HashMap<String, DiffNote> {
    let mut notes = std::collections::HashMap::new();
    for line in raw.lines().skip(1) {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 3 {
            notes.insert(
                parts[0].to_string(),
                DiffNote {
                    classification: parts[1].to_string(),
                    note: parts[2].to_string(),
                },
            );
        }
    }
    notes
}

fn changed_line_count(left: &[u8], right: &[u8]) -> usize {
    let left = String::from_utf8_lossy(left);
    let right = String::from_utf8_lossy(right);
    let left_lines: Vec<&str> = left.lines().collect();
    let right_lines: Vec<&str> = right.lines().collect();
    let max_len = left_lines.len().max(right_lines.len());
    let mut changed = 0usize;
    for index in 0..max_len {
        if left_lines.get(index) != right_lines.get(index) {
            changed += 1;
        }
    }
    changed
}

fn normalized_rust_changed_node_count(left: &[u8], right: &[u8]) -> usize {
    let left = rust_function_blocks(&String::from_utf8_lossy(left));
    let right = rust_function_blocks(&String::from_utf8_lossy(right));
    let mut names: HashSet<String> = left.keys().cloned().collect();
    names.extend(right.keys().cloned());
    names
        .into_iter()
        .filter(|name| left.get(name) != right.get(name))
        .count()
}

fn normalized_rust_changed_node_count_result(left: &[u8], right: &[u8]) -> Result<usize, String> {
    Ok(normalized_rust_changed_node_count(left, right))
}

fn diff_language_for_path(path: &str) -> Option<&'static DiffLanguageSpec> {
    DIFF_LANGUAGE_REGISTRY.iter().find(|spec| {
        spec.extensions
            .iter()
            .any(|extension| path.ends_with(extension))
    })
}

fn normalized_typescript_changed_node_count(left: &[u8], right: &[u8]) -> Result<usize, String> {
    let left = parser_node_fingerprints(
        left,
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "TypeScript",
    )?;
    let right = parser_node_fingerprints(
        right,
        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        "TypeScript",
    )?;
    changed_fingerprint_count(&left, &right)
}

fn normalized_javascript_changed_node_count(left: &[u8], right: &[u8]) -> Result<usize, String> {
    let left =
        parser_node_fingerprints(left, tree_sitter_javascript::LANGUAGE.into(), "JavaScript")?;
    let right =
        parser_node_fingerprints(right, tree_sitter_javascript::LANGUAGE.into(), "JavaScript")?;
    changed_fingerprint_count(&left, &right)
}

fn normalized_python_changed_node_count(left: &[u8], right: &[u8]) -> Result<usize, String> {
    let left = parser_node_fingerprints(left, tree_sitter_python::LANGUAGE.into(), "Python")?;
    let right = parser_node_fingerprints(right, tree_sitter_python::LANGUAGE.into(), "Python")?;
    changed_fingerprint_count(&left, &right)
}

fn normalized_csharp_changed_node_count(left: &[u8], right: &[u8]) -> Result<usize, String> {
    let left = parser_node_fingerprints(left, tree_sitter_c_sharp::LANGUAGE.into(), "C#")?;
    let right = parser_node_fingerprints(right, tree_sitter_c_sharp::LANGUAGE.into(), "C#")?;
    changed_fingerprint_count(&left, &right)
}

fn changed_fingerprint_count(
    left: &HashMap<String, usize>,
    right: &HashMap<String, usize>,
) -> Result<usize, String> {
    let mut keys: HashSet<String> = left.keys().cloned().collect();
    keys.extend(right.keys().cloned());
    let mut changed = 0usize;
    for key in keys {
        let left_count = left.get(&key).copied().unwrap_or(0);
        let right_count = right.get(&key).copied().unwrap_or(0);
        changed += left_count.abs_diff(right_count);
    }
    Ok(changed)
}

fn parser_node_fingerprints(
    source: &[u8],
    language: tree_sitter::Language,
    language_name: &str,
) -> Result<HashMap<String, usize>, String> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&language)
        .map_err(|error| format!("failed to load {language_name} grammar: {error}"))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| format!("failed to parse {language_name} source"))?;
    if tree.root_node().has_error() {
        return Err(format!("{language_name} parse tree contains errors"));
    }

    let mut fingerprints = HashMap::new();
    collect_parser_node_fingerprints(tree.root_node(), source, &mut fingerprints);
    Ok(fingerprints)
}

fn collect_parser_node_fingerprints(
    node: tree_sitter::Node,
    source: &[u8],
    fingerprints: &mut HashMap<String, usize>,
) {
    if node.is_named() && node.parent().is_some() {
        let text = node.utf8_text(source).unwrap_or("");
        let normalized = normalize_parser_text(text);
        let fingerprint = format!("{}:{}", node.kind(), normalized);
        *fingerprints.entry(fingerprint).or_insert(0) += 1;
    }

    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        collect_parser_node_fingerprints(child, source, fingerprints);
    }
}

fn normalize_parser_text(raw: &str) -> String {
    let mut normalized: String = raw.chars().filter(|ch| !ch.is_whitespace()).collect();
    loop {
        let next = normalized
            .replace(",)", ")")
            .replace(",]", "]")
            .replace(",}", "}");
        if next == normalized {
            return normalized;
        }
        normalized = next;
    }
}

fn rust_function_blocks(raw: &str) -> std::collections::HashMap<String, String> {
    let mut blocks = std::collections::HashMap::new();
    let bytes = raw.as_bytes();
    let mut cursor = 0usize;
    while let Some(offset) = raw[cursor..].find("fn ") {
        let start = cursor + offset;
        let name_start = start + 3;
        let mut name_end = name_start;
        while name_end < bytes.len() {
            let ch = bytes[name_end] as char;
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name_end += 1;
            } else {
                break;
            }
        }
        if name_end == name_start {
            cursor = name_start;
            continue;
        }
        let name = raw[name_start..name_end].to_string();
        let Some(open_offset) = raw[name_end..].find('{') else {
            break;
        };
        let open = name_end + open_offset;
        let Some(close) = matching_brace(raw, open) else {
            break;
        };
        blocks.insert(name, normalize_rust_block(&raw[start..=close]));
        cursor = close + 1;
    }
    blocks
}

fn matching_brace(raw: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (offset, ch) in raw[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

fn normalize_rust_block(raw: &str) -> String {
    let mut without_comments = String::new();
    for line in raw.lines() {
        let line = line
            .split_once("//")
            .map(|(before, _)| before)
            .unwrap_or(line);
        without_comments.push_str(line);
    }
    let mut normalized: String = without_comments
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .collect();
    loop {
        let next = normalized
            .replace(",)", ")")
            .replace(",]", "]")
            .replace(",}", "}");
        if next == normalized {
            return normalized;
        }
        normalized = next;
    }
}

impl DiffReport {
    fn to_text(&self) -> String {
        let mut output = format!(
            "diff_report: yes\nleft: {}\nright: {}\npair_count: {}\nrust_pair_count: {}\ntypescript_pair_count: {}\njavascript_pair_count: {}\npython_pair_count: {}\ncsharp_pair_count: {}\nline_diff_changed_lines: {}\nnormalized_changed_node_count: {}\nunsupported_file_fallback_count: {}\nsemantic_equivalence_claimed: no\npairs:\n",
            self.left.display(),
            self.right.display(),
            self.pair_count,
            self.rust_pair_count,
            self.typescript_pair_count,
            self.javascript_pair_count,
            self.python_pair_count,
            self.csharp_pair_count,
            self.line_diff_changed_lines,
            self.normalized_changed_node_count,
            self.unsupported_file_fallback_count,
        );
        for pair in &self.pairs {
            output.push_str(&format!(
                "{}\t{}\tline_changed_lines={}\tnormalized_changed_nodes={}\tunsupported_fallback={}\tnote={}\n",
                pair.path,
                pair.classification,
                pair.line_changed_lines,
                pair.normalized_changed_nodes,
                pair.unsupported_fallback,
                pair.note
            ));
        }
        output
    }
}

#[derive(Debug, Clone, PartialEq)]
struct StorageBenchmarkReport {
    fixture: PathBuf,
    revision_count: usize,
    file_count: usize,
    whole_file_baseline_bytes: u64,
    substrate_stored_bytes: u64,
    substrate_delta_stored_bytes: u64,
    chunk_count: usize,
    unique_chunk_count: usize,
    dedup_ratio: f64,
    delta_dedup_ratio: f64,
    ingest_time_ms: u128,
}

fn run_storage_benchmark(fixture_path: &Path) -> Result<StorageBenchmarkReport, String> {
    let started = Instant::now();
    let fixture = fixture_path.canonicalize().map_err(|error| {
        format!(
            "failed to resolve fixture `{}`: {error}",
            fixture_path.display()
        )
    })?;
    let revisions = collect_revision_dirs(&fixture)?;
    if revisions.is_empty() {
        return Err(format!(
            "fixture has no revision directories: {}",
            fixture.display()
        ));
    }

    let mut file_count = 0usize;
    let mut whole_file_baseline_bytes = 0u64;
    let mut substrate_stored_bytes = 0u64;
    let mut chunk_count = 0usize;
    let mut unique_chunks: HashSet<Vec<u8>> = HashSet::new();

    for revision in &revisions {
        for file in collect_files(revision)? {
            let bytes = fs::read(&file).map_err(|error| {
                format!("failed to read fixture file `{}`: {error}", file.display())
            })?;
            file_count += 1;
            whole_file_baseline_bytes += bytes.len() as u64;

            for chunk in bytes.chunks(BENCH_CHUNK_SIZE) {
                chunk_count += 1;
                let chunk = chunk.to_vec();
                if unique_chunks.insert(chunk.clone()) {
                    substrate_stored_bytes += chunk.len() as u64;
                }
            }
        }
    }

    let dedup_ratio = if substrate_stored_bytes == 0 {
        0.0
    } else {
        whole_file_baseline_bytes as f64 / substrate_stored_bytes as f64
    };
    let substrate_delta_stored_bytes = delta_encoded_unique_chunk_bytes(&unique_chunks);
    let delta_dedup_ratio = if substrate_delta_stored_bytes == 0 {
        0.0
    } else {
        whole_file_baseline_bytes as f64 / substrate_delta_stored_bytes as f64
    };

    Ok(StorageBenchmarkReport {
        fixture,
        revision_count: revisions.len(),
        file_count,
        whole_file_baseline_bytes,
        substrate_stored_bytes,
        substrate_delta_stored_bytes,
        chunk_count,
        unique_chunk_count: unique_chunks.len(),
        dedup_ratio,
        delta_dedup_ratio,
        ingest_time_ms: started.elapsed().as_millis(),
    })
}

fn delta_encoded_unique_chunk_bytes(unique_chunks: &HashSet<Vec<u8>>) -> u64 {
    let mut chunks: Vec<&Vec<u8>> = unique_chunks.iter().collect();
    chunks.sort();

    let mut stored = 0u64;
    let mut previous: Option<&[u8]> = None;
    for chunk in chunks {
        let raw_cost = raw_chunk_record_bytes(chunk);
        let cost = if let Some(previous) = previous {
            raw_cost.min(delta_chunk_record_bytes(previous, chunk))
        } else {
            raw_cost
        };
        stored += cost as u64;
        previous = Some(chunk);
    }
    stored
}

fn raw_chunk_record_bytes(chunk: &[u8]) -> usize {
    chunk.len()
}

fn delta_chunk_record_bytes(previous: &[u8], current: &[u8]) -> usize {
    let prefix = common_prefix_len(previous, current);
    let suffix = common_suffix_len_after_prefix(previous, current, prefix);
    let middle_len = current.len().saturating_sub(prefix + suffix);
    middle_len
}

fn common_prefix_len(left: &[u8], right: &[u8]) -> usize {
    left.iter()
        .zip(right.iter())
        .take_while(|(left, right)| left == right)
        .count()
}

fn common_suffix_len_after_prefix(left: &[u8], right: &[u8], prefix_len: usize) -> usize {
    let max_suffix = left.len().min(right.len()).saturating_sub(prefix_len);
    left.iter()
        .rev()
        .zip(right.iter().rev())
        .take(max_suffix)
        .take_while(|(left, right)| left == right)
        .count()
}

fn collect_revision_dirs(fixture: &Path) -> Result<Vec<PathBuf>, String> {
    if !fixture.is_dir() {
        return Err(format!(
            "fixture path is not a directory: {}",
            fixture.display()
        ));
    }

    let mut revisions = Vec::new();
    for entry in fs::read_dir(fixture)
        .map_err(|error| format!("failed to read fixture `{}`: {error}", fixture.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read fixture entry: {error}"))?;
        let path = entry.path();
        if path.is_dir() {
            revisions.push(path);
        }
    }
    revisions.sort();
    Ok(revisions)
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    collect_files_into(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_into(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to read directory `{}`: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read directory entry: {error}"))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_into(&path, files)?;
        } else if path.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

impl StorageBenchmarkReport {
    fn to_text(&self) -> String {
        let mut output = format!(
            "fixture: {}\nrevision_count: {}\nfile_count: {}\nwhole_file_baseline_bytes: {}\nsubstrate_stored_bytes: {}\nchunk_size_bytes: {}\nchunk_count: {}\nunique_chunk_count: {}\ndedup_ratio: {:.4}\ningest_time_ms: {}\n",
            self.fixture.display(),
            self.revision_count,
            self.file_count,
            self.whole_file_baseline_bytes,
            self.substrate_stored_bytes,
            BENCH_CHUNK_SIZE,
            self.chunk_count,
            self.unique_chunk_count,
            self.dedup_ratio,
            self.ingest_time_ms
        );
        output.push_str("substrate_delta_stored_bytes: ");
        output.push_str(&self.substrate_delta_stored_bytes.to_string());
        output.push('\n');
        output.push_str("delta_dedup_ratio: ");
        output.push_str(&format!("{:.4}", self.delta_dedup_ratio));
        output.push('\n');
        output.push_str("delta_encoding: sorted-unique-chunk-prefix-suffix-experiment\n");
        output
    }
}

impl Manifest {
    fn to_text(&self) -> String {
        let mut output = String::new();
        output.push_str("substrate-manifest-v1\n");
        for entry in &self.entries {
            output.push_str(&entry.path);
            output.push('\t');
            output.push_str(&entry.content_id);
            output.push('\t');
            output.push_str(&entry.byte_len.to_string());
            output.push('\n');
        }
        output
    }

    fn from_text(raw: &str) -> Result<Self, String> {
        let mut lines = raw.lines();
        match lines.next() {
            Some("substrate-manifest-v1") => {}
            _ => return Err("invalid manifest header".to_string()),
        }

        let mut entries = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() != 3 {
                return Err(format!("invalid manifest entry `{line}`"));
            }
            let byte_len = parts[2]
                .parse::<u64>()
                .map_err(|error| format!("invalid manifest byte length: {error}"))?;
            entries.push(ManifestEntry {
                path: parts[0].to_string(),
                content_id: parts[1].to_string(),
                byte_len,
            });
        }
        entries.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(Self { entries })
    }
}

impl IngestReport {
    fn to_text(&self) -> String {
        format!(
            "ingested: yes\nroot: {}\nstate_id: {}\nfile_count: {}\nobject_count: {}\nskipped_binary_count: {}\nmanifest: {}\n",
            self.root.display(),
            self.state_id,
            self.file_count,
            self.object_count,
            self.skipped_binary_count,
            self.manifest.display()
        )
    }
}

impl ProjectReport {
    fn to_text(&self) -> String {
        format!(
            "projected: yes\nroot: {}\nstate_id: {}\nout: {}\nfile_count: {}\n",
            self.root.display(),
            self.state_id,
            self.out.display(),
            self.file_count
        )
    }
}

impl StateLabel {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Candidate => "candidate",
            Self::VerifiedGreen => "verified-green",
        }
    }

    fn from_str(raw: &str) -> Result<Self, String> {
        match raw {
            "candidate" => Ok(Self::Candidate),
            "verified-green" => Ok(Self::VerifiedGreen),
            _ => Err(format!("unknown state label `{raw}`")),
        }
    }
}

impl VerificationMetadata {
    fn to_text(&self) -> String {
        let mut output = format!(
            "substrate-verification-v1\nstate_id={}\nlabel={}\n",
            self.state_id,
            self.label.as_str()
        );
        for gate in &self.gates {
            output.push_str(&format!(
                "gate\t{}\t{}\t{}\n",
                gate.name,
                gate.passed,
                gate.detail.replace('\t', " ").replace('\n', " ")
            ));
        }
        for evidence in &self.evidence {
            output.push_str(&format!(
                "evidence\t{}\n",
                evidence.replace('\t', " ").replace('\n', " ")
            ));
        }
        output
    }

    fn from_text(raw: &str) -> Result<Self, String> {
        let mut lines = raw.lines();
        match lines.next() {
            Some("substrate-verification-v1") => {}
            _ => return Err("invalid verification metadata header".to_string()),
        }
        let state_id = read_prefixed_line(lines.next(), "state_id=")?;
        let label = StateLabel::from_str(&read_prefixed_line(lines.next(), "label=")?)?;
        let mut gates = Vec::new();
        let mut evidence = Vec::new();
        for line in lines {
            let parts: Vec<&str> = line.split('\t').collect();
            match parts.as_slice() {
                ["gate", name, passed, detail] => gates.push(VerificationGate {
                    name: (*name).to_string(),
                    passed: passed
                        .parse::<bool>()
                        .map_err(|error| format!("invalid gate pass value: {error}"))?,
                    detail: (*detail).to_string(),
                }),
                ["evidence", value] => evidence.push((*value).to_string()),
                _ => return Err(format!("invalid verification metadata line `{line}`")),
            }
        }
        Ok(Self {
            state_id,
            label,
            gates,
            evidence,
        })
    }
}

fn read_prefixed_line(line: Option<&str>, prefix: &str) -> Result<String, String> {
    line.and_then(|line| line.strip_prefix(prefix))
        .map(|value| value.to_string())
        .ok_or_else(|| format!("missing {prefix}"))
}

impl StateReport {
    fn to_text(&self) -> String {
        let mut output = format!(
            "state_report: yes\nroot: {}\nstate_id: {}\nlabel: {}\n",
            self.root.display(),
            self.metadata.state_id,
            self.metadata.label.as_str()
        );
        for gate in &self.metadata.gates {
            output.push_str(&format!(
                "gate: {}={} ({})\n",
                gate.name, gate.passed, gate.detail
            ));
        }
        for evidence in &self.metadata.evidence {
            output.push_str(&format!("evidence: {}\n", evidence));
        }
        output
    }
}

impl VerifyReport {
    fn to_text(&self) -> String {
        let mut output = format!(
            "verified: {}\nroot: {}\nstate_id: {}\nlabel: {}\nmetadata: {}\n",
            self.label == StateLabel::VerifiedGreen,
            self.root.display(),
            self.state_id,
            self.label.as_str(),
            self.metadata.display()
        );
        for gate in &self.gates {
            output.push_str(&format!(
                "gate: {}={} ({})\n",
                gate.name, gate.passed, gate.detail
            ));
        }
        output
    }
}

impl StoreMetadata {
    fn to_store_toml(&self) -> String {
        format!(
            "version = {}\nroot = {}\n",
            self.version,
            quote_toml_string(&self.root.to_string_lossy())
        )
    }

    fn from_store_toml(raw: &str) -> Result<Self, String> {
        let version = read_toml_u32(raw, "version")?;
        if version != STORE_VERSION {
            return Err(format!("unsupported store version {version}"));
        }
        let root = PathBuf::from(read_toml_string(raw, "root")?);
        Ok(Self { version, root })
    }
}

fn read_toml_u32(raw: &str, key: &str) -> Result<u32, String> {
    let value = read_toml_value(raw, key)?;
    value
        .parse::<u32>()
        .map_err(|error| format!("{key} must be an integer: {error}"))
}

fn read_toml_string(raw: &str, key: &str) -> Result<String, String> {
    let value = read_toml_value(raw, key)?;
    unquote_toml_string(value).ok_or_else(|| format!("{key} must be a string"))
}

fn read_toml_value<'a>(raw: &'a str, key: &str) -> Result<&'a str, String> {
    raw.lines()
        .filter_map(|line| line.split_once('='))
        .map(|(left, right)| (left.trim(), right.trim()))
        .find(|(left, _)| *left == key)
        .map(|(_, right)| right)
        .ok_or_else(|| format!("missing {key}"))
}

fn quote_toml_string(value: &str) -> String {
    let mut quoted = String::with_capacity(value.len() + 2);
    quoted.push('"');
    for character in value.chars() {
        match character {
            '\\' => quoted.push_str("\\\\"),
            '"' => quoted.push_str("\\\""),
            '\n' => quoted.push_str("\\n"),
            '\r' => quoted.push_str("\\r"),
            '\t' => quoted.push_str("\\t"),
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

fn unquote_toml_string(value: &str) -> Option<String> {
    let inner = value.strip_prefix('"')?.strip_suffix('"')?;
    let mut output = String::new();
    let mut chars = inner.chars();
    while let Some(character) = chars.next() {
        if character != '\\' {
            output.push(character);
            continue;
        }
        match chars.next()? {
            '\\' => output.push('\\'),
            '"' => output.push('"'),
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
            _ => return None,
        }
    }
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after unix epoch")
                .as_nanos();
            let path =
                env::temp_dir().join(format!("substrate-{name}-{}-{unique}", std::process::id()));
            fs::create_dir_all(&path).expect("test dir should be created");
            Self { path }
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn parse_rejects_missing_and_unknown_commands() {
        assert_eq!(
            parse_command(Vec::<String>::new().into_iter()),
            Err("missing command".into())
        );
        assert_eq!(
            parse_command(["nope".to_string()].into_iter()),
            Err("unknown command `nope`".into())
        );
        assert_eq!(
            parse_command(["init".to_string()].into_iter()),
            Err("init requires <path>".into())
        );
        assert_eq!(
            parse_command(["bench".to_string()].into_iter()),
            Err("bench requires <fixture-path>".into())
        );
        assert_eq!(
            parse_command(["ingest".to_string()].into_iter()),
            Err("ingest requires <path>".into())
        );
        assert_eq!(
            parse_command(["project".to_string()].into_iter()),
            Err("project requires <state-id>".into())
        );
        assert_eq!(
            parse_command(["project".to_string(), "s123".to_string()].into_iter()),
            Err("project requires --out <path>".into())
        );
        assert_eq!(
            parse_command(["diff".to_string()].into_iter()),
            Err("diff requires <left> <right>".into())
        );
        assert_eq!(
            parse_command(["state".to_string()].into_iter()),
            Err("state requires <state-id>".into())
        );
        assert_eq!(
            parse_command(["verify".to_string()].into_iter()),
            Err("verify requires <state-id>".into())
        );
    }

    #[test]
    fn init_creates_repo_local_store_metadata() {
        let test_dir = TestDir::new("init");
        let repo = test_dir.path.join("repo");

        let metadata = init_store(&repo).expect("store should initialize");

        assert!(repo.join(STORE_DIR).is_dir());
        assert!(repo.join(STORE_DIR).join(STORE_METADATA).is_file());
        assert_eq!(metadata.version, STORE_VERSION);
        assert_eq!(metadata.root, repo.canonicalize().unwrap());
    }

    #[test]
    fn init_is_idempotent_for_existing_valid_metadata() {
        let test_dir = TestDir::new("idempotent");
        let repo = test_dir.path.join("repo");

        let first = init_store(&repo).expect("first init should succeed");
        let second = init_store(&repo).expect("second init should succeed");

        assert_eq!(first, second);
    }

    #[test]
    fn status_reports_initialized_and_uninitialized_roots() {
        let test_dir = TestDir::new("status");
        let initialized = test_dir.path.join("initialized");
        let plain = test_dir.path.join("plain");
        fs::create_dir_all(&plain).unwrap();

        init_store(&initialized).expect("store should initialize");

        assert!(status(&initialized).unwrap().is_some());
        assert!(status(&plain).unwrap().is_none());
    }

    #[test]
    fn run_returns_non_zero_for_argument_errors() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();

        let code = run(["init"], &mut stdout, &mut stderr);

        assert_eq!(code, 2);
        assert!(stdout.is_empty());
        assert!(String::from_utf8(stderr)
            .unwrap()
            .contains("init requires <path>"));
    }

    #[test]
    fn metadata_round_trips_windows_style_paths() {
        let root = PathBuf::from(r"C:\workspace\repo with spaces");
        let metadata = StoreMetadata {
            version: STORE_VERSION,
            root: root.clone(),
        };

        let parsed = StoreMetadata::from_store_toml(&metadata.to_store_toml()).unwrap();

        assert_eq!(parsed.root, root);
    }

    #[test]
    fn storage_benchmark_reports_required_fields() {
        let test_dir = TestDir::new("bench");
        write_fixture_file(
            &test_dir.path,
            "rev-00/src/generated.rs",
            "pub fn stable() -> u32 { 1 }\npub fn changed() -> u32 { 10 }\n",
        );
        write_fixture_file(
            &test_dir.path,
            "rev-01/src/generated.rs",
            "pub fn stable() -> u32 { 1 }\npub fn changed() -> u32 { 11 }\n",
        );

        let report = run_storage_benchmark(&test_dir.path).expect("benchmark should run");
        let output = report.to_text();

        assert_eq!(report.revision_count, 2);
        assert_eq!(report.file_count, 2);
        assert!(report.whole_file_baseline_bytes > 0);
        assert!(report.substrate_stored_bytes > 0);
        assert!(report.substrate_delta_stored_bytes > 0);
        assert!(report.substrate_delta_stored_bytes <= report.substrate_stored_bytes);
        assert!(report.chunk_count >= report.unique_chunk_count);
        assert!(output.contains("whole_file_baseline_bytes:"));
        assert!(output.contains("substrate_stored_bytes:"));
        assert!(output.contains("substrate_delta_stored_bytes:"));
        assert!(output.contains("chunk_count:"));
        assert!(output.contains("unique_chunk_count:"));
        assert!(output.contains("dedup_ratio:"));
        assert!(output.contains("delta_dedup_ratio:"));
        assert!(output.contains("delta_encoding: sorted-unique-chunk-prefix-suffix-experiment"));
        assert!(output.contains("ingest_time_ms:"));
    }

    #[test]
    fn delta_storage_estimate_counts_only_changed_middle_bytes() {
        assert_eq!(common_prefix_len(b"abc-old-tail", b"abc-new-tail"), 4);
        assert_eq!(
            common_suffix_len_after_prefix(b"abc-old-tail", b"abc-new-tail", 4),
            5
        );
        assert_eq!(
            delta_chunk_record_bytes(b"abc-old-tail", b"abc-new-tail"),
            3
        );

        let chunks: HashSet<Vec<u8>> = [
            b"handler stable body rev 001".to_vec(),
            b"handler stable body rev 002".to_vec(),
            b"handler stable body rev 003".to_vec(),
        ]
        .into_iter()
        .collect();
        let raw_bytes: u64 = chunks.iter().map(|chunk| chunk.len() as u64).sum();
        let delta_bytes = delta_encoded_unique_chunk_bytes(&chunks);

        assert!(delta_bytes < raw_bytes);
    }

    #[test]
    fn committed_storage_fixture_has_required_revision_count() {
        let fixture = Path::new("fixtures/storage-agent-churn");
        let revisions = collect_revision_dirs(fixture).expect("fixture should be readable");
        let report = run_storage_benchmark(fixture).expect("committed fixture should benchmark");

        assert_eq!(revisions.len(), 25);
        assert_eq!(report.revision_count, 25);
        assert!(report.file_count >= 50);
        assert!(report.whole_file_baseline_bytes > report.substrate_stored_bytes);
        assert!(report.substrate_delta_stored_bytes <= report.substrate_stored_bytes);
    }

    #[test]
    fn manifest_serializes_deterministically() {
        let manifest = Manifest {
            entries: vec![
                ManifestEntry {
                    path: "src/a.rs".to_string(),
                    content_id: "abc-1".to_string(),
                    byte_len: 12,
                },
                ManifestEntry {
                    path: "README.md".to_string(),
                    content_id: "def-2".to_string(),
                    byte_len: 20,
                },
            ],
        };

        let parsed = Manifest::from_text(&manifest.to_text()).expect("manifest should parse");

        assert_eq!(parsed.entries.len(), 2);
        assert_eq!(parsed.entries[0].path, "README.md");
        assert_eq!(parsed.entries[1].path, "src/a.rs");
        assert_eq!(state_id(&manifest.to_text()), state_id(&manifest.to_text()));
    }

    #[test]
    fn ingest_requires_initialized_store() {
        let test_dir = TestDir::new("not-init");

        let error = ingest_working_tree(&test_dir.path).unwrap_err();

        assert!(error.contains("not initialized"));
    }

    #[test]
    fn ingest_writes_objects_and_manifest_and_skips_binary() {
        let test_dir = TestDir::new("ingest");
        let repo = test_dir.path.join("repo");
        init_store(&repo).expect("store should initialize");
        write_fixture_file(&repo, "src/lib.rs", "pub fn answer() -> u32 { 42 }\n");
        write_fixture_file(&repo, "README.md", "# Demo\n");
        let binary = repo.join("binary.dat");
        fs::write(&binary, [0, 1, 2, 3]).expect("binary file should be written");

        let report = ingest_working_tree(&repo).expect("ingest should succeed");
        let manifest_text = fs::read_to_string(&report.manifest).unwrap();
        let manifest = Manifest::from_text(&manifest_text).unwrap();

        assert_eq!(report.file_count, 2);
        assert_eq!(report.skipped_binary_count, 1);
        assert!(manifest
            .entries
            .iter()
            .any(|entry| entry.path == "README.md"));
        assert!(manifest
            .entries
            .iter()
            .any(|entry| entry.path == "src/lib.rs"));
        assert!(!manifest
            .entries
            .iter()
            .any(|entry| entry.path == "binary.dat"));
        for entry in manifest.entries {
            assert!(object_path(&repo.canonicalize().unwrap(), &entry.content_id).is_file());
        }
    }

    #[test]
    fn ingest_honors_gitignore_and_default_local_directory_skips() {
        let test_dir = TestDir::new("ignore-ingest");
        let repo = test_dir.path.join("repo");
        init_store(&repo).expect("store should initialize");
        write_fixture_file(
            &repo,
            ".gitignore",
            "ignored.txt\nlogs/\n*.tmp\n!important.tmp\n",
        );
        write_fixture_file(&repo, "src/lib.rs", "pub fn kept() -> bool { true }\n");
        write_fixture_file(&repo, "ignored.txt", "ignore me\n");
        write_fixture_file(&repo, "notes.tmp", "ignore tmp\n");
        write_fixture_file(&repo, "important.tmp", "keep tmp\n");
        write_fixture_file(&repo, "logs/app.log", "ignore log\n");
        write_fixture_file(&repo, ".git/config", "ignore git metadata\n");
        write_fixture_file(&repo, "node_modules/pkg/index.js", "ignore dependency\n");
        write_fixture_file(&repo, "target/debug/output.txt", "ignore build output\n");
        write_fixture_file(&repo, "dist/app.js", "ignore dist output\n");

        let report = ingest_working_tree(&repo).expect("ingest should succeed");
        let manifest_text = fs::read_to_string(&report.manifest).unwrap();
        let manifest = Manifest::from_text(&manifest_text).unwrap();
        let paths: HashSet<&str> = manifest
            .entries
            .iter()
            .map(|entry| entry.path.as_str())
            .collect();

        assert!(paths.contains(".gitignore"));
        assert!(paths.contains("src/lib.rs"));
        assert!(paths.contains("important.tmp"));
        assert!(!paths.contains("ignored.txt"));
        assert!(!paths.contains("notes.tmp"));
        assert!(!paths.contains("logs/app.log"));
        assert!(!paths.contains(".git/config"));
        assert!(!paths.contains("node_modules/pkg/index.js"));
        assert!(!paths.contains("target/debug/output.txt"));
        assert!(!paths.contains("dist/app.js"));
    }

    #[test]
    fn ingest_then_project_recreates_supported_text_files() {
        let test_dir = TestDir::new("round-trip");
        let repo = test_dir.path.join("repo");
        let out = test_dir.path.join("out");
        init_store(&repo).expect("store should initialize");
        write_fixture_file(&repo, "src/main.rs", "fn main() { println!(\"hi\"); }\n");
        write_fixture_file(&repo, "docs/note.txt", "line one\r\nline two\n");

        let ingest = ingest_working_tree(&repo).expect("ingest should succeed");
        let project = project_state(&repo, &ingest.state_id, &out).expect("project should succeed");

        assert_eq!(project.file_count, 2);
        assert_eq!(
            fs::read(repo.join("src/main.rs")).unwrap(),
            fs::read(out.join("src/main.rs")).unwrap()
        );
        assert_eq!(
            fs::read(repo.join("docs/note.txt")).unwrap(),
            fs::read(out.join("docs/note.txt")).unwrap()
        );
        assert!(!out.join(STORE_DIR).exists());
    }

    #[test]
    fn normalized_rust_diff_ignores_formatting_and_reordering() {
        let left = b"fn alpha() -> u32 { 1 }\nfn beta() -> u32 { 2 }\n";
        let right = b"fn beta() -> u32\n{\n    2\n}\nfn alpha() -> u32 { 1 }\n";

        assert!(changed_line_count(left, right) > 0);
        assert_eq!(normalized_rust_changed_node_count(left, right), 0);

        let compact = b"fn pair(a: u32, b: u32) -> (u32, u32) { (a, b) }\n";
        let formatted = b"fn pair(\n    a: u32,\n    b: u32,\n) -> (u32, u32) {\n    (\n        a,\n        b,\n    )\n}\n";
        assert_eq!(normalized_rust_changed_node_count(compact, formatted), 0);
    }

    #[test]
    fn normalized_rust_diff_counts_changed_function_blocks() {
        let left = b"pub fn ready(count: u32) -> bool { count >= 3 }\n";
        let right = b"pub fn ready(count: u32) -> bool { count > 3 }\n";

        assert_eq!(normalized_rust_changed_node_count(left, right), 1);
    }

    #[test]
    fn diff_report_counts_unsupported_fallbacks() {
        let test_dir = TestDir::new("diff");
        let before = test_dir.path.join("before");
        let after = test_dir.path.join("after");
        write_fixture_file(&before, "same.rs", "fn same() -> u32 { 1 }\n");
        write_fixture_file(&after, "same.rs", "fn same() -> u32 { 1 }\n");
        write_fixture_file(&before, "note.txt", "alpha\n");
        write_fixture_file(&after, "note.txt", "beta\n");

        let report = diff_paths(&before, &after).expect("diff should run");

        assert_eq!(report.pair_count, 2);
        assert_eq!(report.rust_pair_count, 1);
        assert_eq!(report.typescript_pair_count, 0);
        assert_eq!(report.javascript_pair_count, 0);
        assert_eq!(report.python_pair_count, 0);
        assert_eq!(report.csharp_pair_count, 0);
        assert_eq!(report.unsupported_file_fallback_count, 1);
        assert_eq!(report.normalized_changed_node_count, 0);
        assert!(report
            .to_text()
            .contains("semantic_equivalence_claimed: no"));
    }

    #[test]
    fn diff_language_registry_detects_supported_extensions() {
        assert_eq!(
            diff_language_for_path("src/lib.rs").map(|spec| spec.language),
            Some(DiffLanguage::Rust)
        );
        assert_eq!(
            diff_language_for_path("src/component.tsx").map(|spec| spec.language),
            Some(DiffLanguage::TypeScript)
        );
        assert_eq!(
            diff_language_for_path("src/widget.jsx").map(|spec| spec.language),
            Some(DiffLanguage::JavaScript)
        );
        assert_eq!(
            diff_language_for_path("src/tool.py").map(|spec| spec.language),
            Some(DiffLanguage::Python)
        );
        assert_eq!(
            diff_language_for_path("src/Widget.cs").map(|spec| spec.language),
            Some(DiffLanguage::CSharp)
        );
        assert!(diff_language_for_path("README.md").is_none());
    }

    #[test]
    fn committed_diff_fixture_produces_required_report() {
        let report = diff_paths(
            Path::new("fixtures/diff-rust-pairs/before"),
            Path::new("fixtures/diff-rust-pairs/after"),
        )
        .expect("committed diff fixture should report");
        let output = report.to_text();

        assert_eq!(report.pair_count, 11);
        assert_eq!(report.rust_pair_count, 10);
        assert_eq!(report.unsupported_file_fallback_count, 1);
        assert!(report.line_diff_changed_lines > 0);
        assert!(report.normalized_changed_node_count > 0);
        assert!(output.contains("pair-01-formatting.rs\tformatting-only"));
        assert!(output.contains("pair-11-unsupported.txt\tunsupported-fallback"));
    }

    #[test]
    fn typescript_tree_sitter_diff_ignores_formatting_and_counts_logic() {
        let formatted_left = b"export function label(id: number, name: string): string { return `${id}:${name}`; }\n";
        let formatted_right = b"export function label(\n  id: number,\n  name: string,\n): string {\n  return `${id}:${name}`;\n}\n";
        assert!(changed_line_count(formatted_left, formatted_right) > 0);
        assert_eq!(
            normalized_typescript_changed_node_count(formatted_left, formatted_right).unwrap(),
            0
        );

        let logic_left = b"export function ready(count: number): boolean { return count >= 3; }\n";
        let logic_right = b"export function ready(count: number): boolean { return count > 3; }\n";
        assert!(normalized_typescript_changed_node_count(logic_left, logic_right).unwrap() > 0);
    }

    #[test]
    fn committed_typescript_fixture_produces_parser_backed_report() {
        let report = diff_paths(
            Path::new("fixtures/diff-typescript-pairs/before"),
            Path::new("fixtures/diff-typescript-pairs/after"),
        )
        .expect("committed TypeScript fixture should report");
        let output = report.to_text();

        assert_eq!(report.pair_count, 5);
        assert_eq!(report.rust_pair_count, 0);
        assert_eq!(report.typescript_pair_count, 4);
        assert_eq!(report.javascript_pair_count, 0);
        assert_eq!(report.python_pair_count, 0);
        assert_eq!(report.csharp_pair_count, 0);
        assert_eq!(report.unsupported_file_fallback_count, 1);
        assert!(report.line_diff_changed_lines > 0);
        assert!(report.normalized_changed_node_count > 0);
        assert!(output.contains("typescript_pair_count: 4"));
        assert!(output.contains("pair-01-formatting.ts\tformatting-only"));
        assert!(output.contains("pair-05-unsupported.txt\tunsupported-fallback"));
    }

    #[test]
    fn javascript_tree_sitter_diff_ignores_formatting_and_counts_logic() {
        let formatted_left = b"export function label(id, name) { return `${id}:${name}`; }\n";
        let formatted_right =
            b"export function label(\n  id,\n  name,\n) {\n  return `${id}:${name}`;\n}\n";
        assert!(changed_line_count(formatted_left, formatted_right) > 0);
        assert_eq!(
            normalized_javascript_changed_node_count(formatted_left, formatted_right).unwrap(),
            0
        );

        let logic_left = b"export function ready(count) { return count >= 3; }\n";
        let logic_right = b"export function ready(count) { return count > 3; }\n";
        assert!(normalized_javascript_changed_node_count(logic_left, logic_right).unwrap() > 0);

        let jsx_left =
            b"export function Badge() { return <span className=\"ready\">Ready</span>; }\n";
        let jsx_right =
            b"export function Badge() { return <span className=\"blocked\">Blocked</span>; }\n";
        assert!(normalized_javascript_changed_node_count(jsx_left, jsx_right).unwrap() > 0);
    }

    #[test]
    fn committed_javascript_fixture_produces_parser_backed_report() {
        let report = diff_paths(
            Path::new("fixtures/diff-javascript-pairs/before"),
            Path::new("fixtures/diff-javascript-pairs/after"),
        )
        .expect("committed JavaScript fixture should report");
        let output = report.to_text();

        assert_eq!(report.pair_count, 5);
        assert_eq!(report.rust_pair_count, 0);
        assert_eq!(report.typescript_pair_count, 0);
        assert_eq!(report.javascript_pair_count, 4);
        assert_eq!(report.python_pair_count, 0);
        assert_eq!(report.csharp_pair_count, 0);
        assert_eq!(report.unsupported_file_fallback_count, 1);
        assert!(report.line_diff_changed_lines > 0);
        assert!(report.normalized_changed_node_count > 0);
        assert!(output.contains("javascript_pair_count: 4"));
        assert!(output.contains("pair-01-formatting.js\tformatting-only"));
        assert!(output.contains("pair-04-jsx.jsx\tlocalized-logic-edit"));
        assert!(output.contains("pair-05-unsupported.txt\tunsupported-fallback"));
    }

    #[test]
    fn python_tree_sitter_diff_ignores_formatting_and_counts_logic() {
        let formatted_left = b"def label(identifier, name):\n    return f'{identifier}:{name}'\n";
        let formatted_right =
            b"def label(\n    identifier,\n    name,\n):\n    return f'{identifier}:{name}'\n";
        assert!(changed_line_count(formatted_left, formatted_right) > 0);
        assert_eq!(
            normalized_python_changed_node_count(formatted_left, formatted_right).unwrap(),
            0
        );

        let logic_left = b"def ready(count):\n    return count >= 3\n";
        let logic_right = b"def ready(count):\n    return count > 3\n";
        assert!(normalized_python_changed_node_count(logic_left, logic_right).unwrap() > 0);

        let reorder_left = b"def alpha():\n    return 1\n\n\ndef beta():\n    return 2\n";
        let reorder_right = b"def beta():\n    return 2\n\n\ndef alpha():\n    return 1\n";
        assert_eq!(
            normalized_python_changed_node_count(reorder_left, reorder_right).unwrap(),
            0
        );
    }

    #[test]
    fn committed_python_fixture_produces_parser_backed_report() {
        let report = diff_paths(
            Path::new("fixtures/diff-python-pairs/before"),
            Path::new("fixtures/diff-python-pairs/after"),
        )
        .expect("committed Python fixture should report");
        let output = report.to_text();

        assert_eq!(report.pair_count, 5);
        assert_eq!(report.rust_pair_count, 0);
        assert_eq!(report.typescript_pair_count, 0);
        assert_eq!(report.javascript_pair_count, 0);
        assert_eq!(report.python_pair_count, 4);
        assert_eq!(report.csharp_pair_count, 0);
        assert_eq!(report.unsupported_file_fallback_count, 1);
        assert!(report.line_diff_changed_lines > 0);
        assert!(report.normalized_changed_node_count > 0);
        assert!(output.contains("python_pair_count: 4"));
        assert!(output.contains("pair-01-formatting.py\tformatting-only"));
        assert!(output.contains("pair-05-unsupported.txt\tunsupported-fallback"));
    }

    #[test]
    fn csharp_tree_sitter_diff_ignores_formatting_and_counts_logic() {
        let formatted_left =
            b"public static string Label(int id, string name) { return $\"{id}:{name}\"; }\n";
        let formatted_right = b"public static string Label(\n    int id,\n    string name\n)\n{\n    return $\"{id}:{name}\";\n}\n";
        assert!(changed_line_count(formatted_left, formatted_right) > 0);
        assert_eq!(
            normalized_csharp_changed_node_count(formatted_left, formatted_right).unwrap(),
            0
        );

        let logic_left = b"public static bool Ready(int count) { return count >= 3; }\n";
        let logic_right = b"public static bool Ready(int count) { return count > 3; }\n";
        assert!(normalized_csharp_changed_node_count(logic_left, logic_right).unwrap() > 0);

        let reorder_left = b"public class AlphaValue { public int Read() { return 1; } }\npublic class BetaValue { public int Read() { return 2; } }\n";
        let reorder_right = b"public class BetaValue { public int Read() { return 2; } }\npublic class AlphaValue { public int Read() { return 1; } }\n";
        assert_eq!(
            normalized_csharp_changed_node_count(reorder_left, reorder_right).unwrap(),
            0
        );
    }

    #[test]
    fn committed_csharp_fixture_produces_parser_backed_report() {
        let report = diff_paths(
            Path::new("fixtures/diff-csharp-pairs/before"),
            Path::new("fixtures/diff-csharp-pairs/after"),
        )
        .expect("committed C# fixture should report");
        let output = report.to_text();

        assert_eq!(report.pair_count, 5);
        assert_eq!(report.rust_pair_count, 0);
        assert_eq!(report.typescript_pair_count, 0);
        assert_eq!(report.javascript_pair_count, 0);
        assert_eq!(report.python_pair_count, 0);
        assert_eq!(report.csharp_pair_count, 4);
        assert_eq!(report.unsupported_file_fallback_count, 1);
        assert!(report.line_diff_changed_lines > 0);
        assert!(report.normalized_changed_node_count > 0);
        assert!(output.contains("csharp_pair_count: 4"));
        assert!(output.contains("pair-01-formatting.cs\tformatting-only"));
        assert!(output.contains("pair-05-unsupported.txt\tunsupported-fallback"));
    }

    #[test]
    fn ingest_writes_candidate_verification_metadata() {
        let test_dir = TestDir::new("candidate");
        let repo = test_dir.path.join("repo");
        init_store(&repo).expect("store should initialize");
        write_fixture_file(&repo, "src/lib.rs", "pub fn answer() -> u32 { 42 }\n");

        let ingest = ingest_working_tree(&repo).expect("ingest should succeed");
        let report = state_report(&repo, &ingest.state_id).expect("state report should load");

        assert_eq!(report.metadata.label, StateLabel::Candidate);
        assert_eq!(report.metadata.state_id, ingest.state_id);
        assert!(report.to_text().contains("label: candidate"));
    }

    #[test]
    fn verify_state_upgrades_valid_state_to_verified_green() {
        let test_dir = TestDir::new("verify-pass");
        let repo = test_dir.path.join("repo");
        let out = test_dir.path.join("out");
        let bench = test_dir.path.join("bench");
        write_bench_fixture(&bench);
        init_store(&repo).expect("store should initialize");
        write_fixture_file(&repo, "src/lib.rs", "pub fn answer() -> u32 { 42 }\n");

        let ingest = ingest_working_tree(&repo).expect("ingest should succeed");
        let verify = verify_state(&repo, &ingest.state_id, &out, &bench)
            .expect("verification should complete");
        let state = state_report(&repo, &ingest.state_id).expect("state should load");

        assert_eq!(verify.label, StateLabel::VerifiedGreen);
        assert!(verify.gates.iter().all(|gate| gate.passed));
        assert_eq!(state.metadata.label, StateLabel::VerifiedGreen);
        assert!(verify.to_text().contains("verified: true"));
    }

    #[test]
    fn verify_state_keeps_tampered_object_candidate() {
        let test_dir = TestDir::new("verify-fail");
        let repo = test_dir.path.join("repo");
        let out = test_dir.path.join("out");
        let bench = test_dir.path.join("bench");
        write_bench_fixture(&bench);
        init_store(&repo).expect("store should initialize");
        write_fixture_file(&repo, "src/lib.rs", "pub fn answer() -> u32 { 42 }\n");

        let ingest = ingest_working_tree(&repo).expect("ingest should succeed");
        let manifest = Manifest::from_text(&fs::read_to_string(&ingest.manifest).unwrap()).unwrap();
        let object = object_path(
            &repo.canonicalize().unwrap(),
            &manifest.entries[0].content_id,
        );
        fs::write(object, b"tampered").expect("object should be tampered");

        let verify = verify_state(&repo, &ingest.state_id, &out, &bench)
            .expect("verification should complete with failed gates");
        let state = state_report(&repo, &ingest.state_id).expect("state should load");

        assert_eq!(verify.label, StateLabel::Candidate);
        assert!(verify.gates.iter().any(|gate| !gate.passed));
        assert_eq!(state.metadata.label, StateLabel::Candidate);
        assert!(verify.to_text().contains("verified: false"));
    }

    fn write_fixture_file(root: &Path, relative: &str, contents: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).expect("fixture parent should be created");
        fs::write(path, contents).expect("fixture file should be written");
    }

    fn write_bench_fixture(root: &Path) {
        write_fixture_file(root, "rev-00/src/lib.rs", "pub fn value() -> u32 { 1 }\n");
        write_fixture_file(root, "rev-01/src/lib.rs", "pub fn value() -> u32 { 2 }\n");
    }
}

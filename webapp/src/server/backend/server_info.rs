use std::hash::{DefaultHasher, Hash, Hasher};
use once_cell::sync::Lazy;

const MAGIC_NUMBER: u64 = 666666;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn run_git(args: &str) -> String {
    let mut cmd = std::process::Command::new("git.exe");
    for arg in args.split(" ") {
        cmd.arg(arg);
    }
    let output = cmd.output().expect("failed to execute git");

    let stdout = output.stdout;
    String::from_utf8_lossy(&stdout)
        .to_string()
        .trim()
        .to_string()
}

fn _fetch_git_version() -> String {
    let branch_name = run_git("rev-parse --abbrev-ref HEAD");
    let commit_hash = run_git("rev-parse --short HEAD");
    let dirty_state = {
        let _untracked = run_git("ls-files --exclude-standard --others");
        let _diff1 = run_git("diff-index HEAD --");
        let _diff2 = run_git("diff-index --cached HEAD --");
        format!("{_untracked}-{_diff1}-{_diff2}")
    };
    let dirty_hash = calculate_hash(&dirty_state) % MAGIC_NUMBER;
    let ts_hash = get_timestamp_hash();
    let version = format!("branch={branch_name} commit={commit_hash} dh={dirty_hash} ts={ts_hash}");
    log::info!("backend found git version = {version}");
    version
}

fn get_timestamp_hash() -> u64 {
    (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros()
        % MAGIC_NUMBER as u128) as u64
}

pub static GIT_VERSION: Lazy<String> = Lazy::new(|| _fetch_git_version());

pub const FILE_URL: &str = "https://file.sfx.xyz";
pub const DEFAULT_FILE_URL: &str = "https://res.sfx.xyz/images/default.png";

pub fn mode() -> String {
    let machine_kind = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    machine_kind.to_string()
}

pub fn is_debug() -> bool {
    mode() == "debug"
}

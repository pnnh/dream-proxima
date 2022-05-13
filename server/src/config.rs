pub fn mode() -> String {
    let machine_kind = if cfg!(debug_assertions) {
        "debug"
    } else { "release" };
    machine_kind.to_string()
}

pub fn is_debug() -> bool {
    mode() == "debug"
}

pub fn is_release() -> bool {
    !is_debug()
}
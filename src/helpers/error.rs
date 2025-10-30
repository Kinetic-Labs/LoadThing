use crate::helpers::ansi;

// failed to get uri path
pub const ERROR_1: i32 = 0x000001;
// failed to bind
pub const ERROR_2: i32 = 0x000002;
// failed to read file
pub const ERROR_3: i32 = 0x000003;
// failed to fetch
pub const ERROR_4: i32 = 0x000004;
// failed to preform/initialize TLS handshake
pub const ERROR_5: i32 = 0x000005;
// net write error
pub const ERROR_6: i32 = 0x000006;
// net read error
pub const ERROR_7: i32 = 0x000007;
// net connection error
pub const ERROR_8: i32 = 0x000008;
// thread messaging error
pub const ERROR_9: i32 = 0x000009;
// thread join error
pub const ERROR_10: i32 = 0x00010;

pub fn send_error(error: i32, extra: String) {
    eprintln!(
        "{}\n  {}{extra}{}",
        fmt_error(error),
        ansi::GRAY,
        ansi::RESET
    );
}

pub fn fmt_error(error: i32) -> String {
    let base = format!("ERROR RAISED WITH CODE: {error}!");
    let get_message: &str = match error {
        ERROR_1 => "Failed to get URI path",
        ERROR_2 => "Failed to bind address",
        ERROR_3 => "Failed to read file!",
        ERROR_4 => "Failed to fetch!",
        ERROR_5 => "Failed to preform and/or initialize TLS handshake!",
        ERROR_6 => "Failed to write!",
        ERROR_7 => "Failed to read!",
        ERROR_8 => "Failed to connect!",
        ERROR_9 => "Failed to communicate between threads!",
        ERROR_10 => "Failed to join thread(s)!",
        _ => &format!("UKNOWN ERROR WITH CODE: {error}!"),
    };

    format!("{}{base} MESSAGE: {get_message}{}", ansi::RED, ansi::RESET)
}

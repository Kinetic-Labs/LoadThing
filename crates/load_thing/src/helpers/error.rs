use crate::helpers::ansi;

// failed to get uri path
pub const ERROR_1_FAILED_TO_GET_URI_PATH: i32 = 0x000001;
// failed to bind
pub const ERROR_2_BIND_ERROR: i32 = 0x000002;
// failed to read file
pub const ERROR_3_READ_FILE_ERROR: i32 = 0x000003;
// failed to fetch
pub const ERROR_4_FETCH_ERROR: i32 = 0x000004;
// failed to preform/initialize TLS handshake
pub const ERROR_5_FAILURE_TLS_HANDSHAKE: i32 = 0x000005;
// net write error
pub const ERROR_6_NET_WRITE_ERROR: i32 = 0x000006;
// net read error
pub const ERROR_7_NET_READ_ERROR: i32 = 0x000007;
// net connection error
pub const ERROR_8_NET_CONN_ERROR: i32 = 0x000008;
// thread messaging error
pub const ERROR_9_THREAD_MESSAGING_ERROR: i32 = 0x000009;
// thread join error
pub const ERROR_10_THREAD_JOIN_ERROR: i32 = 0x00010;

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
        ERROR_1_FAILED_TO_GET_URI_PATH => "Failed to get URI path",
        ERROR_2_BIND_ERROR => "Failed to bind address",
        ERROR_3_READ_FILE_ERROR => "Failed to read file!",
        ERROR_4_FETCH_ERROR => "Failed to fetch!",
        ERROR_5_FAILURE_TLS_HANDSHAKE => "Failed to preform and/or initialize TLS handshake!",
        ERROR_6_NET_WRITE_ERROR => "Failed to write!",
        ERROR_7_NET_READ_ERROR => "Failed to read!",
        ERROR_8_NET_CONN_ERROR => "Failed to connect!",
        ERROR_9_THREAD_MESSAGING_ERROR => "Failed to communicate between threads!",
        ERROR_10_THREAD_JOIN_ERROR => "Failed to join thread(s)!",
        _ => &format!("UKNOWN ERROR WITH CODE: {error}!"),
    };

    format!("{}{base} MESSAGE: {get_message}{}", ansi::RED, ansi::RESET)
}

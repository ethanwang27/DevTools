mod error;
mod utils;
use utils::administrative_division::get_all_provinces;
use utils::base64util::{base64_decode, base64_encode};
use utils::hashes::hash_generate;
use utils::hmac_util::generate_hmac;
use utils::id_no::{get_id_no, parse_id_no};
use utils::qr_code::get_qr_code;
use utils::url_utils::{url_decode, url_encode};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .max_file_size(500_000)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .level(log::LevelFilter::Info)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] [{}] {} {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"), //解决timezone_strategy不生效的问题
                        record.level(),
                        message,
                        record.target()
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_qr_code,
            get_all_provinces,
            get_id_no,
            parse_id_no,
            hash_generate,
            generate_hmac,
            base64_encode,
            base64_decode,
            url_decode,
            url_encode,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

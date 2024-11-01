mod utils;
use utils::qr_code::get_qr_code;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use tauri_plugin_log::{Target, TargetKind};
    let log_targets = vec![
        Target::new(TargetKind::Stdout),
        Target::new(TargetKind::Webview),
        Target::new(TargetKind::LogDir {
            file_name: Some("logs".to_string()),
        }),
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets(log_targets)
                .max_file_size(500_000)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .level(log::LevelFilter::Debug)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![get_qr_code])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

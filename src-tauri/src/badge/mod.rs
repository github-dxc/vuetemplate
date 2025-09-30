pub mod linux_badge;
pub mod macos_badge;
pub mod windows_badge;

// 来消息
pub async fn set_message_notify(window: tauri::Window, count: i64) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        if count > 0 {
            windows_badge::flash_taskbar_icon(&window)?;
            windows_badge::show_message_notification(&window)?;
        } else {
            windows_badge::clear_taskbar_status(&window)?;
        }
    }
    #[cfg(target_os = "macos")]
    {
        if count > 0 {
            macos_badge::flash_taskbar_icon_with_type(&window, macos_badge::FlashType::Critical)?;
            macos_badge::set_message_count_badge(count as u32)?;
        } else {
            macos_badge::stop_flash_taskbar_icon()?;
        }
    }
    Ok(())
}

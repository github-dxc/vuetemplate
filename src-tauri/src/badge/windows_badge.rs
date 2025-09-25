use std::path::Path;
use tauri::Window;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{LoadIconW, SetClassLongPtrW, GCLP_HICON};

#[cfg(target_os = "windows")]
pub fn set_badge(window: &Window, icon_path: Option<&str>) -> Result<(), String> {
    // 获取窗口句柄
    let hwnd = window.hwnd().map_err(|e| e.to_string())?;
    let handle = hwnd.0 as isize;

    unsafe {
        if let Some(path) = icon_path {
            // 确保图标文件存在
            if !Path::new(path).exists() {
                return Err(format!("Icon file not found: {}", path));
            }

            // 加载新图标
            use windows::core::PCWSTR;
            let wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
            let icon = LoadIconW(None, PCWSTR(wide.as_ptr()));
            if let Ok(icon) = icon {
                // 设置窗口类的图标
                SetClassLongPtrW(
                    HWND(handle as isize),
                    GCLP_HICON,
                    icon.0,
                );
            }
        } else {
            // 恢复默认图标
            SetClassLongPtrW(HWND(handle as isize), GCLP_HICON, 0);
        }
    }

    Ok(())
}

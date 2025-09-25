use std::path::Path;
use tauri::Window;

// 检查是否在 Linux 环境下
#[cfg(target_os = "linux")]
pub fn set_badge(window: &Window, icon_path: Option<&str>) -> Result<(), String> {
    {
        if let Some(path) = icon_path {
            // 验证图标文件是否存在
            if !Path::new(path).exists() {
                return Err(format!("图标文件未找到: {}", path));
            }

            // 获取窗口的 GTK 句柄
            if let Some(window_handle) = window.gtk_window() {
                unsafe {
                    // 加载图标
                    if let Ok(pixbuf) = gtk::gdk_pixbuf::Pixbuf::from_file(path) {
                        // 设置窗口图标
                        window_handle.set_icon(Some(&pixbuf));
                    }
                }
            }
        } else {
            // 恢复默认图标
            if let Some(window_handle) = window.gtk_window() {
                window_handle.set_icon(None);
            }
        }
    }

    Ok(())
}
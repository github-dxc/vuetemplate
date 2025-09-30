use tauri::Window;
use windows::core::{GUID, PCWSTR};
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_INPROC_SERVER};
use windows::Win32::UI::Shell::{ITaskbarList3, TBPF_ERROR, TBPF_NOPROGRESS};
use windows::Win32::UI::WindowsAndMessaging::{
    FlashWindowEx, GetForegroundWindow, LoadImageW, SendMessageW, FLASHWINFO, FLASHW_ALL,
    FLASHW_TIMERNOFG, ICON_BIG, ICON_SMALL, IMAGE_ICON, LR_DEFAULTSIZE, LR_LOADFROMFILE,
    WM_SETICON,
};

// 修改任务栏图标
#[cfg(target_os = "windows")]
pub fn set_taskbar_icon(window: &Window, icon_path: Option<&str>) -> Result<(), String> {
    let hwnd = window.hwnd().map_err(|e| e.to_string())?;
    let hwnd = HWND(hwnd.0 as isize);

    unsafe {
        use windows::Win32::Foundation::{LPARAM, WPARAM};

        let h_icon = if let Some(path) = icon_path {
            if !std::path::Path::new(path).exists() {
                return Err(format!("Icon file not found: {}", path));
            }
            let wide_path: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
            let icon_handle = LoadImageW(
                None,
                PCWSTR(wide_path.as_ptr()),
                IMAGE_ICON,
                0,
                0,
                LR_LOADFROMFILE | LR_DEFAULTSIZE,
            )
            .map_err(|e| format!("Failed to load icon: {:?}", e))?;
            icon_handle.0
        } else {
            0
        };

        // THE FIX: Cast ICON_BIG and ICON_SMALL to usize.
        // This ensures the type matches what IntoParam<WPARAM> expects.
        SendMessageW(hwnd, WM_SETICON, WPARAM(ICON_BIG as usize), LPARAM(h_icon));
        SendMessageW(
            hwnd,
            WM_SETICON,
            WPARAM(ICON_SMALL as usize),
            LPARAM(h_icon),
        );
    }
    Ok(())
}

// 让任务栏图标闪烁（如果窗口不在前台）
#[cfg(target_os = "windows")]
pub fn flash_taskbar_icon(window: &Window) -> Result<(), String> {
    let hwnd = window.hwnd().map_err(|e| e.to_string())?;
    let hwnd = HWND(hwnd.0 as isize);

    unsafe {
        // 检查窗口是否是前台窗口

        let foreground_hwnd = GetForegroundWindow();

        // 只有当窗口不在前台时才闪烁
        if foreground_hwnd != hwnd {
            let mut flash_info = FLASHWINFO {
                cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
                hwnd,
                dwFlags: FLASHW_ALL | FLASHW_TIMERNOFG, // 闪烁标题栏和任务栏
                uCount: 3,                              // 0 表示持续闪烁直到窗口变为前台
                dwTimeout: 200,                         // 闪烁间隔，单位为毫秒
            };

            let result = FlashWindowEx(&mut flash_info);
            if !result.as_bool() {
                return Err("Failed to flash window".into());
            }
        }
    }

    Ok(())
}

const CLSID_TaskbarList: GUID = GUID::from_u128(0x56FDF344_FD6D_11d0_958A_006097C9A090);

// 显示消息通知状态（红色进度条）
#[cfg(target_os = "windows")]
pub fn show_message_notification(window: &Window) -> Result<(), String> {
    unsafe {
        // 初始化 COM
        CoInitialize(None).map_err(|e| format!("Failed to initialize COM: {:?}", e))?;

        // 创建 TaskbarList 实例
        let taskbar_list: ITaskbarList3 =
            CoCreateInstance(&CLSID_TaskbarList, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| format!("Failed to create TaskbarList: {:?}", e))?;

        // 初始化任务栏列表
        taskbar_list
            .HrInit()
            .map_err(|e| format!("Failed to initialize TaskbarList: {:?}", e))?;

        // 获取窗口句柄
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        let hwnd = HWND(hwnd.0 as isize);

        // 设置错误状态（红色进度条）
        taskbar_list
            .SetProgressState(hwnd, TBPF_ERROR)
            .map_err(|e| format!("Failed to set progress state: {:?}", e))?;

        // 设置进度值（这会让红色更明显）
        taskbar_list
            .SetProgressValue(hwnd, 100, 100) // 100% 进度
            .map_err(|e| format!("Failed to set progress value: {:?}", e))?;
    }

    Ok(())
}

// 清除所有任务栏状态
#[cfg(target_os = "windows")]
pub fn clear_taskbar_status(window: &Window) -> Result<(), String> {
    unsafe {
        // 初始化 COM
        CoInitialize(None).map_err(|e| format!("Failed to initialize COM: {:?}", e))?;

        // 创建 TaskbarList 实例
        let taskbar_list: ITaskbarList3 =
            CoCreateInstance(&CLSID_TaskbarList, None, CLSCTX_INPROC_SERVER)
                .map_err(|e| format!("Failed to create TaskbarList: {:?}", e))?;

        // 初始化任务栏列表
        taskbar_list
            .HrInit()
            .map_err(|e| format!("Failed to initialize TaskbarList: {:?}", e))?;

        // 获取窗口句柄
        let hwnd = window.hwnd().map_err(|e| e.to_string())?;
        let hwnd = HWND(hwnd.0 as isize);

        // 清除进度状态
        taskbar_list
            .SetProgressState(hwnd, TBPF_NOPROGRESS)
            .map_err(|e| format!("Failed to clear progress state: {:?}", e))?;

        // 停止闪烁
        let mut flash_info = FLASHWINFO {
            cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
            hwnd,
            dwFlags: windows::Win32::UI::WindowsAndMessaging::FLASHW_STOP,
            uCount: 0,
            dwTimeout: 0,
        };
        FlashWindowEx(&mut flash_info);
    }

    Ok(())
}

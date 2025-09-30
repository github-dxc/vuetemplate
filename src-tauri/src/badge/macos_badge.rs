use std::path::Path;
use tauri::Window;

#[cfg(target_os = "macos")]
use cocoa::{
    appkit::{NSApp, NSApplication, NSImage, NSRequestUserAttentionType},
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSString},
};
#[cfg(target_os = "macos")]
use core_foundation::string::CFString;
#[cfg(target_os = "macos")]
use objc::{
    msg_send,
    runtime::{Class, Object, BOOL, NO, YES},
    sel, sel_impl,
};

/// 设置 Dock 图标
#[cfg(target_os = "macos")]
pub fn set_taskbar_icon<P: AsRef<Path>>(window: &Window, icon_path: P) -> Result<(), String> {
    let icon_path = icon_path.as_ref();

    if !icon_path.exists() {
        return Err(format!("图标文件不存在: {:?}", icon_path));
    }

    let path_str = icon_path.to_str().ok_or("无效的文件路径")?;

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // 获取 NSApplication 实例
        let app = NSApp();

        // 创建 NSString 路径
        let ns_path = NSString::alloc(nil).init_str(path_str);

        // 创建 NSImage
        let image = NSImage::alloc(nil).initWithContentsOfFile_(ns_path);

        if image == nil {
            return Err("无法加载图标文件".to_string());
        }

        // 设置应用图标
        app.setApplicationIconImage_(image);

        // 释放资源
        let _: () = msg_send![image, release];
        let _: () = msg_send![ns_path, release];
    }

    Ok(())
}

/// 重置 Dock 图标为默认图标
#[cfg(target_os = "macos")]
pub fn reset_taskbar_icon(window: &Window) -> Result<(), String> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        // 设置为 nil 会恢复默认图标
        app.setApplicationIconImage_(nil);
    }

    Ok(())
}

/// 闪烁 Dock 图标以引起用户注意
#[cfg(target_os = "macos")]
pub fn flash_taskbar_icon(window: &Window) -> Result<(), String> {
    flash_taskbar_icon_with_type(window, FlashType::Informational)
}

/// 闪烁类型枚举
#[cfg(target_os = "macos")]
pub enum FlashType {
    /// 信息性请求 - 温和的提醒
    Informational,
    /// 紧急请求 - 持续闪烁直到用户响应
    Critical,
}

/// 使用指定类型闪烁 Dock 图标
#[cfg(target_os = "macos")]
pub fn flash_taskbar_icon_with_type(window: &Window, flash_type: FlashType) -> Result<(), String> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        let request_type = match flash_type {
            FlashType::Informational => NSRequestUserAttentionType::NSInformationalRequest,
            FlashType::Critical => NSRequestUserAttentionType::NSCriticalRequest,
        };

        // 请求用户注意
        let request_id: i32 = app.requestUserAttention_(request_type);

        if request_id == 0 {
            return Err("无法请求用户注意".to_string());
        }
    }

    Ok(())
}

/// 停止 Dock 图标闪烁
#[cfg(target_os = "macos")]
pub fn stop_flash_taskbar_icon(window: &Window) -> Result<(), String> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        // 取消所有用户注意请求
        app.cancelUserAttentionRequest_(0);
    }

    Ok(())
}

/// 显示消息通知
#[cfg(target_os = "macos")]
pub fn show_message_notification(
    window: &Window,
    title: &str,
    message: &str,
    subtitle: Option<&str>,
) -> Result<(), String> {
    show_notification_with_options(
        window,
        NotificationOptions {
            title: title.to_string(),
            message: message.to_string(),
            subtitle: subtitle.map(|s| s.to_string()),
            sound: true,
            flash_dock: true,
            user_info: None,
        },
    )
}

/// 通知选项结构体
#[cfg(target_os = "macos")]
pub struct NotificationOptions {
    pub title: String,
    pub message: String,
    pub subtitle: Option<String>,
    pub sound: bool,
    pub flash_dock: bool,
    pub user_info: Option<std::collections::HashMap<String, String>>,
}

/// 使用详细选项显示通知
#[cfg(target_os = "macos")]
pub fn show_notification_with_options(
    window: &Window,
    options: NotificationOptions,
) -> Result<(), String> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        // 获取用户通知中心
        let notification_center_class =
            Class::get("NSUserNotificationCenter").ok_or("无法获取 NSUserNotificationCenter 类")?;
        let default_center: id =
            msg_send![notification_center_class, defaultUserNotificationCenter];

        // 创建用户通知
        let notification_class =
            Class::get("NSUserNotification").ok_or("无法获取 NSUserNotification 类")?;
        let notification: id = msg_send![notification_class, alloc];
        let notification: id = msg_send![notification, init];

        // 设置标题
        let title_ns = NSString::alloc(nil).init_str(&options.title);
        let _: () = msg_send![notification, setTitle: title_ns];

        // 设置消息内容
        let message_ns = NSString::alloc(nil).init_str(&options.message);
        let _: () = msg_send![notification, setInformativeText: message_ns];

        // 设置副标题（如果提供）
        if let Some(subtitle) = &options.subtitle {
            let subtitle_ns = NSString::alloc(nil).init_str(subtitle);
            let _: () = msg_send![notification, setSubtitle: subtitle_ns];
            let _: () = msg_send![subtitle_ns, release];
        }

        // 设置声音
        if options.sound {
            let sound_class =
                Class::get("NSUserNotificationDefaultSoundName").ok_or("无法获取默认声音")?;
            let default_sound: id = msg_send![sound_class, defaultUserNotificationSound];
            let _: () = msg_send![notification, setSoundName: default_sound];
        }

        // 设置用户信息（如果提供）
        if let Some(user_info) = &options.user_info {
            let dict_class =
                Class::get("NSMutableDictionary").ok_or("无法获取 NSMutableDictionary 类")?;
            let dict: id = msg_send![dict_class, alloc];
            let dict: id = msg_send![dict, init];

            for (key, value) in user_info {
                let key_ns = NSString::alloc(nil).init_str(key);
                let value_ns = NSString::alloc(nil).init_str(value);
                let _: () = msg_send![dict, setObject:value_ns forKey:key_ns];
                let _: () = msg_send![key_ns, release];
                let _: () = msg_send![value_ns, release];
            }

            let _: () = msg_send![notification, setUserInfo: dict];
            let _: () = msg_send![dict, release];
        }

        // 发送通知
        let _: () = msg_send![default_center, deliverNotification: notification];

        // 如果需要闪烁 Dock 图标
        if options.flash_dock {
            flash_taskbar_icon(window)?;
        }

        // 释放资源
        let _: () = msg_send![notification, release];
        let _: () = msg_send![title_ns, release];
        let _: () = msg_send![message_ns, release];
    }

    Ok(())
}

/// 设置 Dock 图标徽章（数字提示）
#[cfg(target_os = "macos")]
pub fn set_dock_badge(badge_text: Option<&str>) -> Result<(), String> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        let app = NSApp();

        match badge_text {
            Some(text) => {
                let badge_ns = NSString::alloc(nil).init_str(text);
                let _: () = msg_send![app, dockTile];
                let dock_tile: id = msg_send![app, dockTile];
                let _: () = msg_send![dock_tile, setBadgeLabel: badge_ns];
                let _: () = msg_send![badge_ns, release];
            }
            None => {
                let dock_tile: id = msg_send![app, dockTile];
                let _: () = msg_send![dock_tile, setBadgeLabel: nil];
            }
        }
    }

    Ok(())
}

/// 设置消息计数徽章
#[cfg(target_os = "macos")]
pub fn set_message_count_badge(count: u32) -> Result<(), String> {
    let badge_text = if count > 0 {
        Some(count.to_string())
    } else {
        None
    };

    set_dock_badge(badge_text.as_deref())
}

/// 检查通知权限
#[cfg(target_os = "macos")]
pub fn check_notification_permission() -> Result<bool, String> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let notification_center_class =
            Class::get("NSUserNotificationCenter").ok_or("无法获取 NSUserNotificationCenter 类")?;
        let default_center: id =
            msg_send![notification_center_class, defaultUserNotificationCenter];

        // 检查通知中心是否可用
        if default_center == nil {
            return Ok(false);
        }

        // 在 macOS 10.14+ 中，需要检查通知权限
        // 这里简化处理，假设有权限（实际应用中可能需要更复杂的权限检查）
        Ok(true)
    }
}

/// 请求通知权限（用于 macOS 10.14+）
#[cfg(target_os = "macos")]
pub fn request_notification_permission() -> Result<(), String> {
    // 在真实应用中，这里应该使用 UNUserNotificationCenter
    // 为了简化，这里只是一个占位符
    println!("请在系统偏好设置中允许通知权限");
    Ok(())
}

/// 综合示例：显示带有所有选项的通知
#[cfg(target_os = "macos")]
pub fn show_rich_notification(
    window: &Window,
    title: &str,
    message: &str,
    count: Option<u32>,
) -> Result<(), String> {
    // 设置徽章计数
    if let Some(count) = count {
        set_message_count_badge(count)?;
    }

    // 显示通知
    let mut user_info = std::collections::HashMap::new();
    user_info.insert(
        "timestamp".to_string(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
            .to_string(),
    );

    show_notification_with_options(
        window,
        NotificationOptions {
            title: title.to_string(),
            message: message.to_string(),
            subtitle: count.map(|c| format!("总计 {} 条消息", c)),
            sound: true,
            flash_dock: true,
            user_info: Some(user_info),
        },
    )?;

    Ok(())
}

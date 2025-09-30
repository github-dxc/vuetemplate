import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { LogicalSize } from '@tauri-apps/api/window';
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart';
import { isRegistered, register, unregister } from '@tauri-apps/plugin-global-shortcut';
import { writeText, readText } from '@tauri-apps/plugin-clipboard-manager';


// 创建新窗口
export const createNewWindow = async (label, option, DOMContentLoadedCallback, successCallback, errorCallback) => {
    // 首先检查窗口是否已经存在
    const existingWindow = await WebviewWindow.getByLabel(label);

    if (existingWindow) {
        console.log(existingWindow);

        // 显示窗口（如果被隐藏）
        const focused = await existingWindow.isFocused();
        if (!focused) {
            existingWindow.setFocus()
        }
        DOMContentLoadedCallback()
        return existingWindow;
    }
    const webview = new WebviewWindow(label, option);

    // 监听窗口创建完成
    webview.once('tauri://created', function () {
        console.log('窗口创建成功');
        if (successCallback) {
            successCallback(webview);
        }
    });

    // 监听窗口创建错误
    webview.once('tauri://error', function (e) {
        console.error('窗口创建失败:', e);
        if (errorCallback) {
            errorCallback(e);
        }
    });

    // 监听窗口DOM加载完成事件
    webview.once('DOMContentLoaded', function (e) {
        if (DOMContentLoadedCallback) {
            DOMContentLoadedCallback();
        }
    });
}

// 修改窗口大小
export const changeSize = async ({ label, width, hight, center = false, onTop = false }) => {
    const webview = await WebviewWindow.getByLabel(label);
    if (webview) {
        //把窗口设置为保持在页面上
        await webview.setSize(new LogicalSize(width, hight));
        await webview.setAlwaysOnTop(onTop);
        if (center) {
            await webview.center();
        }
    } else {
        console.error(`窗口 ${label} 不存在`);
    }
}

// 自动启动
export const autoStart = async (autoStart = false) => {
    const enabled = await isEnabled();
    if (autoStart) {
        if (!enabled) {
            enable();
            console.log('Auto start enabled');
        }
    } else {
        if (enabled) {
            disable();
            console.log('Auto start disabled');
        };
    }
};

// 注册快捷键事件
export const registerShortcut = async (key, hook) => {
    const b = await isRegistered(key);
    if (!b) {
        await register(key, hook);
    }
}

// 注销快捷键事件
export const unregisterShortcut = async (key) => {
    const b = await isRegistered(key);
    if (b) {
        await unregister(key);
    }
}

// 读取剪切板信息
export const readClipboard = async () => {
    return await readText();
}

// 写入剪切板信息
export const writeClipboard = async (text='') => {
    await writeText(text);
}
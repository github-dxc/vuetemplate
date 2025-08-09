import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

// 创建新窗口
export const createNewWindow = async function(label, option) {
    // 首先检查窗口是否已经存在
    const existingWindow = await WebviewWindow.getByLabel(label);
    
    if (existingWindow) {
        console.log(existingWindow);

        // 显示窗口（如果被隐藏）
        const focused = await existingWindow.isFocused();
        if (!focused) {
            existingWindow.setFocus()
        }
        
        return existingWindow;
    }
    const webview = new WebviewWindow(label, option);

    // 监听窗口创建完成
    webview.once('tauri://created', function () {
        console.log('窗口创建成功');
    });

    // 监听窗口创建错误
    webview.once('tauri://error', function (e) {
        console.error('窗口创建失败:', e);
    });
}

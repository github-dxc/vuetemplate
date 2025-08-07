import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

// 创建新窗口
export const createNewWindow = async function() {
    const label = 'theUniqueLabel'; // 窗口的唯一标识符

    // 首先检查窗口是否已经存在
    const existingWindow = await WebviewWindow.getByLabel(label);
    
    if (existingWindow) {
        console.log(existingWindow);

        // 显示窗口（如果被隐藏）
        const focused = await existingWindow.isFocused();
        if (!focused) {
            existingWindow.setFocus()
        }
        // await existingWindow.show();
        
        // // 将窗口置于前台并获得焦点
        // await existingWindow.setFocus();
        
        // 如果窗口被最小化，则恢复窗口
        // if (await existingWindow.isMinimized()) {
        //     await existingWindow.unminimize();
        // }
        
        return existingWindow;
    }
    const webview = new WebviewWindow(label, {
        url: '/login',
        title: '新窗口',
        width: 1600,
        height: 900,
        resizable: true,
        center: true
    });

    // 监听窗口创建完成
    webview.once('tauri://created', function () {
        console.log('窗口创建成功');
    });

    // 监听窗口创建错误
    webview.once('tauri://error', function (e) {
        console.error('窗口创建失败:', e);
    });
}

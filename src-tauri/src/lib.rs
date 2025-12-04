// 这是一个注释，指向 Tauri 命令的官方文档链接，用于学习如何从前端调用 Rust 函数
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// 导入 tauri::Manager trait，这是一个核心 trait，用于管理 Tauri 应用的状态、窗口和资源
// 它提供了如 get_webview_window() 等方法，使我们能够在 setup 钩子中访问和操作窗口
use tauri::Manager;
// 这是 Manager trait 的 API 文档链接，包含所有可用方法和示例
// https://docs.rs/tauri/latest/tauri/trait.Manager.html


// 在前端点击开始游戏时调用此命令以启动指定路径的程序
// path: 可执行文件的完整路径
// admin: 是否以管理员权限运行
#[tauri::command]
fn launch(path: String, args: Option<String>, admin: bool) -> Result<(), String> {
    use std::process::Command;
    use std::os::windows::process::CommandExt;

    if admin {
        let mut cmd = Command::new("powershell");
        cmd.arg("-Command");
        
        // Escape single quotes for PowerShell
        let safe_path = path.replace("'", "''");
        let mut ps_cmd = format!("Start-Process -FilePath '{}'", safe_path);
        
        if let Some(a) = args {
            let safe_args = a.replace("'", "''");
            ps_cmd.push_str(&format!(" -ArgumentList '{}'", safe_args));
        }
        
        ps_cmd.push_str(" -Verb RunAs");
        
        cmd.arg(ps_cmd);
        
        // CREATE_NO_WINDOW to hide the powershell console
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);

        cmd.spawn()
            .map(|_child| ())
            .map_err(|e| e.to_string())
    } else {
        let mut cmd = Command::new(&path);
        if let Some(a) = args {
            // 简单按空白分割参数 —— 对常见场景有效。若需更复杂解析可改进。
            let parts = a.split_whitespace();
            cmd.args(parts);
        }

        // 尝试 spawn 子进程，不阻塞当前线程
        cmd.spawn().map(|_child| ()).map_err(|e| e.to_string())
    }
}

// 这是一个条件编译属性，用于移动端（Android/iOS）入口点
// 当编译为移动端时，此函数将成为应用的入口点，而不是 main.rs 中的 main 函数
// mobile 是 Tauri 定义的编译目标，表示移动平台
#[cfg_attr(mobile, tauri::mobile_entry_point)]
// 定义一个公开的（pub）函数 run，没有参数，返回值隐含为 ()
// 这个函数是 Tauri 应用的真正入口，负责构建和运行应用
pub fn run() {
    // 使用 tauri::Builder::default() 创建一个默认的 Tauri 应用构建器
    // 这是一个流式 API，用于配置应用的各个方面
    tauri::Builder::default()
        // 添加 tauri-plugin-opener 插件，这个插件允许应用打开外部链接或文件
        // init() 是插件的初始化函数，返回插件实例
        .plugin(tauri_plugin_opener::init())
        // 添加 tauri-plugin-dialog 插件，用于显示文件/文件夹选择对话框
        .plugin(tauri_plugin_dialog::init())
        // 设置一个 setup 钩子，这是一个闭包，在应用启动时执行
        // 它接收一个可变引用到 App 实例，用于配置窗口和其他初始化逻辑
        .setup(|app| {
            // 从应用实例中获取名为 "main" 的窗口
            // get_webview_window() 方法来自 Manager trait，返回一个窗口实例
            // unwrap() 用于解包 Result，如果失败则 panic（在开发中用于快速失败）
            let window = app.get_webview_window("main").unwrap();
            // 获取应用的包信息，包括版本号等
            // package_info() 返回 PackageInfo 结构体，我们取其 version 字段的引用
            let version = &app.package_info().version;
            // 设置窗口标题，使用 format! 宏将 "SSMT4 v" 和版本号拼接
            // set_title() 方法设置窗口的标题栏文本
            window.set_title(&format!("SSMT4 v{}", version)).unwrap();
            // 设置窗口大小，使用 LogicalSize 结构体指定逻辑像素尺寸
            // LogicalSize { width: 1000.0, height: 671.0 } 表示宽度 1000 像素，高度 671 像素
            // set_size() 方法应用这个尺寸到窗口
            window.set_size(tauri::Size::Logical(tauri::LogicalSize { width: 1000.0, height: 671.0 })).unwrap();
            // 设置窗口是否可调整大小，true 表示用户可以拖拽边框改变窗口大小
            // set_resizable() 方法控制这个属性
            window.set_resizable(true).unwrap();
            // 将窗口居中显示在屏幕上
            // center() 方法自动计算屏幕尺寸并调整窗口位置
            window.center().unwrap();
            // 返回 Ok(()) 表示 setup 钩子执行成功
            // 如果有错误，可以返回 Err，但这里我们假设所有操作都成功
            Ok(())
        })
        // 注册命令处理器，使用 generate_handler! 宏自动生成处理命令的代码
        // 这允许前端通过 invoke 调用 Rust 中标记为 #[tauri::command] 的函数
        .invoke_handler(tauri::generate_handler![launch])
        // 运行应用，使用 generate_context!() 宏生成应用上下文（包括配置等）
        // run() 方法启动事件循环，开始监听前端调用和系统事件
        .run(tauri::generate_context!())
        // 如果运行失败，使用 expect() 提供错误消息并 panic
        // 这确保了应用启动失败时能给出有意义的错误信息
        .expect("error while running tauri application");
}

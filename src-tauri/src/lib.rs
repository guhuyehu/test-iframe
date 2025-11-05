// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::{Manager, webview::DownloadEvent, WebviewWindowBuilder};
use tauri_plugin_dialog::DialogExt;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 关闭默认创建的主窗口（如果存在）
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.close();
            }
            
            let handle = app.handle().clone();
            let dialog_handle = handle.clone();
            
            // 创建窗口构建器（从配置或手动创建）
            let window_builder = if let Some(window_config) = app.config().app.windows.first() {
                // 如果有配置，使用配置创建
                WebviewWindowBuilder::from_config(&handle, window_config)?
            } else {
                // 如果没有配置，手动创建窗口
                WebviewWindowBuilder::new(
                    &handle,
                    "main",
                    tauri::WebviewUrl::App("index.html".into())
                )
                .title("test-iframe")
                .inner_size(800.0, 600.0)
            };
            
            let _webview_window = window_builder
                .on_download(move |_webview, event| {
                    if let DownloadEvent::Requested { url, destination: _ } = event {
                        println!("检测到下载请求: {}", url);
                        
                        // 立即阻止默认下载，我们将在对话框中手动下载
                        // 从 URL 中提取文件名
                        let url_str = url.as_str().to_string();
                        let url_clone = url_str.clone();
                        let filename = url_str
                            .split('/')
                            .last()
                            .unwrap_or("download")
                            .split('?')
                            .next()
                            .unwrap_or("download")
                            .to_string();
                        
                        let app_handle = dialog_handle.clone();
                        
                        // 在后台异步显示对话框，不阻塞主线程
                        std::thread::spawn(move || {
                            // 显示文件保存对话框
                            app_handle.dialog().file()
                                .set_title("选择保存位置")
                                .set_file_name(&filename)
                                .save_file(move |path| {
                                    if let Some(file_path) = path {
                                        if let Some(path_ref) = file_path.as_path() {
                                            let save_path = std::path::PathBuf::from(path_ref);
                                            println!("用户选择的保存路径: {:?}", save_path);
                                            
                                            // 在后台线程中下载文件
                                            let url_for_download = url_clone.clone();
                                            std::thread::spawn(move || {
                                                match download_file(&url_for_download, &save_path) {
                                                    Ok(_) => {
                                                        println!("下载完成: {:?}", save_path);
                                                    }
                                                    Err(e) => {
                                                        eprintln!("下载失败: {}", e);
                                                    }
                                                }
                                            });
                                        }
                                    } else {
                                        println!("用户取消了下载");
                                    }
                                });
                        });
                        
                        // 立即返回 false，阻止默认下载
                        false
                    } else {
                        true // 允许其他下载事件继续
                    }
                })
                .build()?;
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 手动下载文件的函数
fn download_file(url: &str, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("开始下载: {} -> {:?}", url, path);
    
    // 使用 reqwest 下载文件
    let response = reqwest::blocking::get(url)?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP 错误: {}", response.status()).into());
    }
    
    // 创建文件
    let mut file = std::fs::File::create(path)?;
    
    // 写入文件内容
    std::io::copy(&mut response.bytes()?.as_ref(), &mut file)?;
    
    println!("下载完成: {:?}", path);
    Ok(())
}

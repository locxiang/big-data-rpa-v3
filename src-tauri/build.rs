fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut windows = tauri_build::WindowsAttributes::new();
        // 使用外部manifest文件
        windows = windows.app_manifest("app.manifest");
        
        let attrs = tauri_build::Attributes::new().windows_attributes(windows);
        tauri_build::try_build(attrs).expect("Build failed");
    }

    #[cfg(not(target_os = "windows"))]
    {
        tauri_build::build()
    }
}

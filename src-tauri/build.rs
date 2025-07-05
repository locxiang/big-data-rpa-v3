fn main() {
    //tauri_build::build()

    let mut windows = tauri_build::WindowsAttributes::new();
    windows = windows.app_manifest(r#"
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
    "#);

    let attrs = tauri_build::Attributes::new().windows_attributes(windows);
    tauri_build::try_build(attrs).expect("Build failed");
}

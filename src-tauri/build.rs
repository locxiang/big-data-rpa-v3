use std::env;
use std::path::PathBuf;

fn main() {
    // 生成 Tauri 构建配置
    tauri_build::build();
    
    // Windows 特定构建配置
    if cfg!(target_os = "windows") {
        configure_windows_build();
    }
}

fn configure_windows_build() {
    println!("cargo:rerun-if-changed=app.manifest");
    
    // 链接 Windows 清单文件
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("app.manifest");
    
    if manifest_path.exists() {
        println!("cargo:rustc-link-arg-bin=big-data-rpa-v3=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bin=big-data-rpa-v3=/MANIFESTINPUT:{}", manifest_path.display());
    }
    
    // 配置 npcap/pcap 库链接
    configure_pcap_linking();
    
    // 配置静态链接 CRT
    configure_static_crt();
    
    // 配置 Windows 系统库
    configure_windows_libs();
}

fn configure_pcap_linking() {
    // 检查是否有 npcap SDK
    if let Ok(npcap_sdk) = env::var("NPCAP_SDK_PATH") {
        let lib_path = PathBuf::from(npcap_sdk).join("Lib/x64");
        if lib_path.exists() {
            println!("cargo:rustc-link-search=native={}", lib_path.display());
            println!("cargo:rustc-link-lib=static=wpcap");
            println!("cargo:rustc-link-lib=static=Packet");
        }
    } else {
        // 尝试使用系统安装的 npcap
        println!("cargo:rustc-link-lib=dylib=wpcap");
        println!("cargo:rustc-link-lib=dylib=Packet");
    }
}

fn configure_static_crt() {
    // 检查是否启用静态链接
    if env::var("RUSTFLAGS").unwrap_or_default().contains("crt-static") {
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcrt");
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcmt");
        println!("cargo:rustc-link-lib=static=libcmt");
    }
}

fn configure_windows_libs() {
    // 链接必要的 Windows 系统库
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=oleaut32");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=ws2_32");
    println!("cargo:rustc-link-lib=iphlpapi");
    
    // 网络相关库
    println!("cargo:rustc-link-lib=netapi32");
    println!("cargo:rustc-link-lib=userenv");
    println!("cargo:rustc-link-lib=psapi");
}

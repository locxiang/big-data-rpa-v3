use anyhow::{anyhow, Result};
use etherparse::{InternetSlice, SlicedPacket, TransportSlice};
use log::{debug, error, info};
use once_cell::sync::OnceCell;
use pcap::Capture;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::ipc::Channel;

// 运行状态控制
static CAPTURE_RUNNING: OnceCell<Arc<AtomicBool>> = OnceCell::new();
static CAPTURE_THREAD: OnceCell<Arc<Mutex<Option<thread::JoinHandle<()>>>>> = OnceCell::new();
static CAPTURE_STATUS: OnceCell<Arc<Mutex<CaptureStatus>>> = OnceCell::new();
static APP_HANDLE: OnceCell<tauri::AppHandle> = OnceCell::new();
static STATUS_CHANNEL: OnceCell<Arc<Mutex<Option<Channel<CaptureStatus>>>>> = OnceCell::new();
static HTTP_CHANNEL: OnceCell<Arc<Mutex<Option<Channel<HttpRequest>>>>> = OnceCell::new();

// 捕获状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureStatus {
    pub running: bool,
    pub message: String,
    pub device_name: String,
    pub start_time: u64,
}

// HTTP 请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    pub id: u64,
    pub timestamp: u64,
    pub src_ip: String,
    pub src_port: u16,
    pub dst_ip: String,
    pub dst_port: u16,
    pub method: String,
    pub path: String,
    pub version: String,
    pub host: String,
    pub content_type: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

// 初始化 AppHandle 以便发送事件
pub fn init_app_handle(app_handle: tauri::AppHandle) -> Result<()> {
    APP_HANDLE
        .set(app_handle)
        .map_err(|_| anyhow!("已经初始化过 AppHandle"))?;
    Ok(())
}

// 设置状态通道
pub fn set_status_channel(channel: Channel<CaptureStatus>) -> Result<()> {
    if let Some(channels) = STATUS_CHANNEL.get() {
        let mut guard = channels.lock().unwrap();
        *guard = Some(channel);
        Ok(())
    } else {
        let channels = Arc::new(Mutex::new(Some(channel)));
        STATUS_CHANNEL
            .set(channels)
            .map_err(|_| anyhow!("已经初始化过状态通道"))?;
        Ok(())
    }
}

// 设置 HTTP 请求通道
pub fn set_http_channel(channel: Channel<HttpRequest>) -> Result<()> {
    if let Some(channels) = HTTP_CHANNEL.get() {
        let mut guard = channels.lock().unwrap();
        *guard = Some(channel);
        Ok(())
    } else {
        let channels = Arc::new(Mutex::new(Some(channel)));
        HTTP_CHANNEL
            .set(channels)
            .map_err(|_| anyhow!("已经初始化过 HTTP 请求通道"))?;
        Ok(())
    }
}

pub fn init_packet_capture() -> Result<()> {
    // 初始化运行状态标志
    let running = Arc::new(AtomicBool::new(true));
    CAPTURE_RUNNING
        .set(running.clone())
        .map_err(|_| anyhow!("已经初始化过运行状态标志"))?;

    // 初始化线程句柄
    let thread_handle = Arc::new(Mutex::new(None));
    CAPTURE_THREAD
        .set(thread_handle.clone())
        .map_err(|_| anyhow!("已经初始化过线程句柄"))?;
        
    // 初始化捕获状态
    let status = Arc::new(Mutex::new(CaptureStatus {
        running: false,
        message: "正在初始化...".to_string(),
        device_name: "未知".to_string(),
        start_time: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    }));
    CAPTURE_STATUS
        .set(status.clone())
        .map_err(|_| anyhow!("已经初始化过捕获状态"))?;
        
    // 初始化通道存储
    if STATUS_CHANNEL.get().is_none() {
        STATUS_CHANNEL
            .set(Arc::new(Mutex::new(None)))
            .map_err(|_| anyhow!("已经初始化过状态通道存储"))?;
    }
    
    if HTTP_CHANNEL.get().is_none() {
        HTTP_CHANNEL
            .set(Arc::new(Mutex::new(None)))
            .map_err(|_| anyhow!("已经初始化过 HTTP 请求通道存储"))?;
    }

    // 启动捕获线程
    let running_clone = running.clone();
    let status_clone = status.clone();
    let capture_thread = thread::spawn(move || {
        if let Err(e) = start_capture(running_clone, status_clone) {
            error!("数据包捕获出错: {}", e);
            if let Some(status) = CAPTURE_STATUS.get() {
                let mut status_guard = status.lock().unwrap();
                status_guard.running = false;
                status_guard.message = format!("捕获失败: {}", e);
            }
            // 发送状态更新
            send_status_update();
        }
    });

    // 保存线程句柄
    *thread_handle.lock().unwrap() = Some(capture_thread);
    info!("数据包捕获线程已启动");
    Ok(())
}

fn start_capture(running: Arc<AtomicBool>, status: Arc<Mutex<CaptureStatus>>) -> Result<()> {
    info!("开始初始化数据包捕获...");
    
    // 更新状态
    {
        let mut status_guard = status.lock().unwrap();
        status_guard.message = "正在初始化网络捕获...".to_string();
    }
    send_status_update();

    // 获取可用的网络设备列表
    let list = match pcap::Device::list() {
        Ok(list) => list,
        Err(e) => {
            let err = anyhow!("获取网络设备列表失败: {}", e);
            {
                let mut status_guard = status.lock().unwrap();
                status_guard.running = false;
                status_guard.message = err.to_string();
            }
            send_status_update();
            return Err(err);
        }
    };
    
    if list.is_empty() {
        let err = anyhow!("没有找到可用的网络设备");
        {
            let mut status_guard = status.lock().unwrap();
            status_guard.running = false;
            status_guard.message = err.to_string();
        }
        send_status_update();
        return Err(err);
    }
    
    // 尝试找到一个非回环设备
    let device = match list.iter().find(|d| !d.flags.is_loopback()) {
        Some(device) => device,
        None => {
            let err = anyhow!("没有找到非回环网络设备");
            {
                let mut status_guard = status.lock().unwrap();
                status_guard.running = false;
                status_guard.message = err.to_string();
            }
            send_status_update();
            return Err(err);
        }
    };
    
    info!("使用网络设备: {}", device.name);
    
    // 更新状态
    {
        let mut status_guard = status.lock().unwrap();
        status_guard.device_name = device.name.clone();
    }
    send_status_update();
    
    let mut cap = match Capture::from_device(device.clone()) {
        Ok(cap) => match cap.promisc(true).timeout(1000).immediate_mode(true).open() {
            Ok(cap) => cap,
            Err(e) => {
                let err = anyhow!("打开网络设备失败: {}. 请确保已安装ChmodBPF", e);
                {
                    let mut status_guard = status.lock().unwrap();
                    status_guard.running = false;
                    status_guard.message = err.to_string();
                }
                send_status_update();
                return Err(err);
            }
        },
        Err(e) => {
            let err = anyhow!("创建捕获句柄失败: {}. 请确保已安装ChmodBPF", e);
            {
                let mut status_guard = status.lock().unwrap();
                status_guard.running = false;
                status_guard.message = err.to_string();
            }
            send_status_update();
            return Err(err);
        }
    };

    // 设置过滤器，只捕获 HTTP 流量
    if let Err(e) = cap.filter("tcp port 80 or tcp port 8080 or tcp port 443", true) {
        let err = anyhow!("设置过滤器失败: {}", e);
        {
            let mut status_guard = status.lock().unwrap();
            status_guard.running = false;
            status_guard.message = err.to_string();
        }
        send_status_update();
        return Err(err);
    }
    
    // 更新状态为运行中
    {
        let mut status_guard = status.lock().unwrap();
        status_guard.running = true;
        status_guard.message = "正在捕获 HTTP 请求...".to_string();
    }
    send_status_update();
    
    info!("开始捕获 HTTP 请求数据包...");

    // 简化的捕获循环
    while running.load(Ordering::Relaxed) {
        match cap.next_packet() {
            Ok(packet) => {
                debug!("捕获到数据包: {} 字节", packet.data.len());
                match SlicedPacket::from_ethernet(packet.data) {
                    Ok(sliced) => process_packet(sliced),
                    Err(e) => debug!("解析数据包错误: {:?}", e)
                }
            },
            Err(pcap::Error::TimeoutExpired) => continue, // 超时是正常的
            Err(e) => {
                error!("捕获数据包错误: {:?}", e);
                if !running.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }

    // 更新状态为已停止
    {
        let mut status_guard = status.lock().unwrap();
        status_guard.running = false;
        status_guard.message = "数据包捕获已停止".to_string();
    }
    send_status_update();

    info!("数据包捕获已停止");
    Ok(())
}

fn process_packet(sliced: SlicedPacket) {
    // 提取 IP 地址信息
    let (src_ip, dst_ip) = match sliced.ip {
        Some(InternetSlice::Ipv4(ipv4, _)) => (
            IpAddr::V4(ipv4.source_addr()),
            IpAddr::V4(ipv4.destination_addr()),
        ),
        Some(InternetSlice::Ipv6(ipv6, _)) => (
            IpAddr::V6(ipv6.source_addr()),
            IpAddr::V6(ipv6.destination_addr()),
        ),
        None => return,
    };

    // 提取端口信息
    let (src_port, dst_port) = match sliced.transport {
        Some(TransportSlice::Tcp(tcp)) => (tcp.source_port(), tcp.destination_port()),
        _ => return,
    };

    // 只处理有效载荷
    if !sliced.payload.is_empty() {
        // 检查是否是 HTTP 请求
        if is_http_request(sliced.payload) {
            // 解析 HTTP 请求
            if let Some(mut http_request) = parse_http_request(sliced.payload) {
                // 添加网络信息
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                http_request.timestamp = timestamp;
                http_request.src_ip = src_ip.to_string();
                http_request.src_port = src_port;
                http_request.dst_ip = dst_ip.to_string();
                http_request.dst_port = dst_port;
                
                // 生成唯一ID
                http_request.id = timestamp * 1000 + (src_port as u64 % 1000);
                
                // 输出格式化的 HTTP 请求信息到日志
                info!("捕获 HTTP 请求: {}:{} -> {}:{}", src_ip, src_port, dst_ip, dst_port);
                info!("请求方法: {}", http_request.method);
                info!("请求路径: {}", http_request.path);
                
                // 发送 HTTP 请求到前端
                send_http_request(http_request);
            }
        }
    }
}

// 检查是否是 HTTP 请求
fn is_http_request(data: &[u8]) -> bool {
    if data.len() < 4 {
        return false;
    }

    data.starts_with(b"GET ")
        || data.starts_with(b"POST ")
        || data.starts_with(b"PUT ")
        || data.starts_with(b"DELETE ")
}

// 解析 HTTP 请求
fn parse_http_request(data: &[u8]) -> Option<HttpRequest> {
    let http_text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = http_text.split("\r\n").collect();
    
    if lines.is_empty() {
        return None;
    }
    
    // 解析请求行
    let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
    if request_line_parts.len() < 3 {
        return None;
    }
    
    let method = request_line_parts[0].to_string();
    let path = request_line_parts[1].to_string();
    let version = request_line_parts[2].to_string();
    
    let mut host = String::new();
    let mut content_type = String::new();
    let mut headers = Vec::new();
    let mut body = String::new();
    
    // 找到请求头和请求体的分隔位置
    let mut body_start = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            body_start = i + 1;
            break;
        }
        
        // 解析请求头
        if i > 0 {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                let header_name = parts[0].to_string();
                let header_value = parts[1].to_string();
                
                // 提取特定的头信息
                if header_name.eq_ignore_ascii_case("Host") {
                    host = header_value.clone();
                } else if header_name.eq_ignore_ascii_case("Content-Type") {
                    content_type = header_value.clone();
                }
                
                headers.push((header_name, header_value));
            }
        }
    }
    
    // 提取请求体
    if body_start < lines.len() {
        body = lines[body_start..].join("\r\n");
    }
    
    Some(HttpRequest {
        id: 0, // 将在 process_packet 中设置
        timestamp: 0, // 将在 process_packet 中设置
        src_ip: String::new(), // 将在 process_packet 中设置
        src_port: 0, // 将在 process_packet 中设置
        dst_ip: String::new(), // 将在 process_packet 中设置
        dst_port: 0, // 将在 process_packet 中设置
        method,
        path,
        version,
        host,
        content_type,
        headers,
        body,
    })
}

pub fn stop_packet_capture() -> Result<()> {
    // 设置运行标志为 false，通知捕获线程停止
    if let Some(running) = CAPTURE_RUNNING.get() {
        running.store(false, Ordering::Relaxed);
        info!("已发送停止数据包捕获的信号");
    }

    // 尝试等待线程结束
    if let Some(handle) = CAPTURE_THREAD.get() {
        let mut guard = handle.lock().unwrap();
        if let Some(thread) = guard.take() {
            if thread.is_finished() {
                let _ = thread.join();
                info!("数据包捕获线程已正常结束");
            } else {
                info!("数据包捕获线程正在运行，已发送停止信号");
            }
        }
    }
    
    // 更新状态
    if let Some(status) = CAPTURE_STATUS.get() {
        let mut status_guard = status.lock().unwrap();
        status_guard.running = false;
        status_guard.message = "数据包捕获已停止".to_string();
    }
    send_status_update();
    
    Ok(())
}

// 获取捕获状态
pub fn get_capture_status() -> CaptureStatus {
    if let Some(status) = CAPTURE_STATUS.get() {
        let status_guard = status.lock().unwrap();
        status_guard.clone()
    } else {
        CaptureStatus {
            running: false,
            message: "捕获未初始化".to_string(),
            device_name: "未知".to_string(),
            start_time: 0,
        }
    }
}

// 通过 Channel 发送状态更新
fn send_status_update() {
    if let Some(channels) = STATUS_CHANNEL.get() {
        let guard = channels.lock().unwrap();
        if let Some(channel) = &*guard {
            let status = get_capture_status();
            info!("通过 Channel 发送状态更新: {:?}", status);
            if let Err(e) = channel.send(status) {
                error!("发送状态更新失败: {}", e);
            }
        }
    }
}

// 通过 Channel 发送 HTTP 请求
fn send_http_request(request: HttpRequest) {
    if let Some(channels) = HTTP_CHANNEL.get() {
        let guard = channels.lock().unwrap();
        if let Some(channel) = &*guard {
            info!("通过 Channel 发送 HTTP 请求: {:?}", request);
            if let Err(e) = channel.send(request) {
                error!("发送 HTTP 请求失败: {}", e);
            }
        }
    }
}
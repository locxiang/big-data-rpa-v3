import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke, Channel } from '@tauri-apps/api/core';

// 定义数据包类型
export interface PacketData {
  id: string;
  timestamp: number;
  type: 'request' | 'response' | 'tcp';
  protocol: string;
  srcIp: string;
  srcPort: number;
  dstIp: string;
  dstPort: number;
  length: number;
  http?: {
    method?: string;
    url?: string;
    host?: string;
    headers?: Record<string, string>;
    body?: string;
  };
}

// 定义网络设备类型
export interface NetworkDevice {
  name: string;
  description: string;
  is_loopback: boolean;
  addresses: string[];
}

// 定义捕获状态类型
export interface CaptureStatus {
  running: boolean;
  message: string;
  device_name: string;
  start_time: number;
}

// 定义 HTTP 请求类型
export interface HttpRequest {
  id: number;
  timestamp: number;
  src_ip: string;
  src_port: number;
  dst_ip: string;
  dst_port: number;
  method: string;
  path: string;
  version: string;
  host: string;
  content_type: string;
  headers: [string, string][];
  body: string;
}

export const useProxyStore = defineStore('proxy', () => {
  // 状态
  const captureStatus = ref<CaptureStatus>({
    running: false,
    message: '未初始化',
    device_name: '未知',
    start_time: 0
  });
  
  const packets = ref<PacketData[]>([]);
  const devices = ref<NetworkDevice[]>([]);
  const selectedDevice = ref<string>('');
  const error = ref<string>('');
  const isLoading = ref<boolean>(false);
  
  // 计算属性
  const isCapturing = computed(() => captureStatus.value.running);
  const packetCount = computed(() => packets.value.length);
  
  // 初始化
  const initialize = async () => {
    try {
      isLoading.value = true;
      error.value = '';
      
      // 设置状态通道
      const statusChannel = new Channel<CaptureStatus>();
      statusChannel.onmessage = (status: CaptureStatus) => {
        console.log('收到状态更新:', status);
        captureStatus.value = status;
      };
      
      // 设置HTTP请求通道
      const httpChannel = new Channel<HttpRequest>();
      httpChannel.onmessage = (httpRequest: HttpRequest) => {
        console.log('收到 HTTP 请求:', httpRequest);
        const packet: PacketData = {
          id: httpRequest.id.toString(),
          timestamp: httpRequest.timestamp * 1000, // 转换为毫秒
          type: 'request',
          protocol: 'HTTP',
          srcIp: httpRequest.src_ip,
          srcPort: httpRequest.src_port,
          dstIp: httpRequest.dst_ip,
          dstPort: httpRequest.dst_port,
          length: httpRequest.body.length,
          http: {
            method: httpRequest.method,
            url: httpRequest.path,
            host: httpRequest.host,
            headers: Object.fromEntries(httpRequest.headers),
            body: httpRequest.body
          }
        };
        
        packets.value.unshift(packet);
        
        // 限制包数量，避免内存溢出
        if (packets.value.length > 1000) {
          packets.value = packets.value.slice(0, 1000);
        }
      };
      
      // 发送通道到后端
      await invoke('set_status_channel', { channel: statusChannel });
      await invoke('set_http_channel', { channel: httpChannel });
      
      // 获取初始状态
      await getCaptureStatus();
      
      // 获取网络设备列表
      await getNetworkDevices();
      
    } catch (err) {
      error.value = `初始化失败: ${err}`;
      console.error('初始化失败:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  // 开始捕获
  const startCapture = async () => {
    try {
      isLoading.value = true;
      error.value = '';
      
      await invoke('init_packet_capture');
      
    } catch (err) {
      error.value = `启动失败: ${err}`;
      console.error('启动捕获失败:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  // 停止捕获
  const stopCapture = async () => {
    try {
      isLoading.value = true;
      error.value = '';
      
      await invoke('stop_packet_capture');
      
    } catch (err) {
      error.value = `停止失败: ${err}`;
      console.error('停止捕获失败:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  // 清空数据包
  const clearPackets = () => {
    packets.value = [];
  };
  
  // 获取捕获状态
  const getCaptureStatus = async () => {
    try {
      const status = await invoke('get_capture_status') as CaptureStatus;
      captureStatus.value = status;
      return status;
    } catch (err) {
      console.error('获取状态失败:', err);
      throw err;
    }
  };
  
  // 检查权限
  const checkPermissions = async () => {
    try {
      const hasPermission = await invoke('has_chmodbpf') as boolean;
      if (!hasPermission) {
        error.value = '缺少网络捕获权限，请安装 ChmodBPF 权限包';
      }
      return hasPermission;
    } catch (err) {
      console.error('权限检查失败:', err);
      throw err;
    }
  };
  
  // 获取网络设备列表
  const getNetworkDevices = async () => {
    try {
      isLoading.value = true;
      error.value = '';
      
      const deviceList = await invoke('get_network_devices') as NetworkDevice[];
      devices.value = deviceList;
      
      // 自动选择第一个非回环设备
      const nonLoopbackDevice = deviceList.find(device => !device.is_loopback);
      if (nonLoopbackDevice && !selectedDevice.value) {
        selectedDevice.value = nonLoopbackDevice.name;
      }
      
      return deviceList;
    } catch (err) {
      error.value = `获取网络设备失败: ${err}`;
      console.error('获取网络设备失败:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  };
  
    // 清理资源
  const cleanup = async () => {
    try {
      if (captureStatus.value.running) {
        localStorage.clear();
        await stopCapture();
      }
    } catch (err) {
      console.error('清理失败:', err);
    }
  };

  // 保存数据包到 localStorage 用于新窗口访问
  const savePacketForWindow = (packet: PacketData) => {
    try {
      localStorage.setItem(`packet-${packet.id}`, JSON.stringify(packet));
    } catch (err) {
      console.error('保存数据包失败:', err);
    }
  };

  // 从 localStorage 获取数据包
  const getPacketForWindow = (id: string): PacketData | null => {
    try {
      
      const packetData = localStorage.getItem(`packet-${id}`);
      console.log('获取数据包》getPacketForWindow', id, packetData);
      if (packetData) {
        return JSON.parse(packetData) as PacketData;
      }
    } catch (err) {
      console.error('获取数据包失败:', err);
    }
    return null;
  };

  // 根据ID查找数据包
  const findPacketById = (id: string): PacketData | undefined => {
    return packets.value.find(packet => packet.id === id);
  };

  return {
    // 状态
    captureStatus,
    packets,
    devices,
    selectedDevice,
    error,
    isLoading,
    
    // 计算属性
    isCapturing,
    packetCount,
    
    // 方法
    initialize,
    startCapture,
    stopCapture,
    clearPackets,
    getCaptureStatus,
    checkPermissions,
    getNetworkDevices,
    cleanup,
    savePacketForWindow,
    getPacketForWindow,
    findPacketById
  };
}); 
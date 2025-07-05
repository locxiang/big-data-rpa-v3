<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";

// 定义类型
interface CaptureStatus {
  running: boolean;
  message: string;
  device_name: string;
  start_time: number;
}

interface HttpRequest {
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

// 状态和数据
const captureStatus = ref<CaptureStatus>({
  running: false,
  message: "正在加载...",
  device_name: "",
  start_time: 0
});
const httpRequests = ref<HttpRequest[]>([]);
const selectedRequest = ref<HttpRequest | null>(null);
const showDetails = ref(false);

// ChmodBPF状态
const hasChmodBPF = ref(false);
const showChmodBPFWarning = ref(false);

// 通道
let statusChannel: Channel<CaptureStatus> | null = null;
let httpChannel: Channel<HttpRequest> | null = null;

// 检查ChmodBPF状态
async function checkChmodBPF() {
  try {
    hasChmodBPF.value = await invoke("has_chmodbpf");
    console.log("ChmodBPF状态:", hasChmodBPF.value);
    if (!hasChmodBPF.value) {
      showChmodBPFWarning.value = true;
    }
  } catch (error) {
    console.error("检查ChmodBPF状态失败:", error);
  }
}

// 获取初始捕获状态
async function fetchCaptureStatus() {
  try {
    captureStatus.value = await invoke("get_capture_status");
  } catch (error) {
    console.error("获取捕获状态失败:", error);
  }
}

// 设置状态更新通道
async function setupStatusChannel() {
  statusChannel = new Channel<CaptureStatus>();
  
  // 设置消息处理函数
  statusChannel.onmessage = (status) => {
    console.log("收到状态更新:", status);
    captureStatus.value = status;
  };
  
  // 发送通道到后端
  try {
    await invoke("set_status_channel", { channel: statusChannel });
    console.log("状态通道已设置");
  } catch (error) {
    console.error("设置状态通道失败:", error);
  }
}

// 设置 HTTP 请求通道
async function setupHttpChannel() {
  httpChannel = new Channel<HttpRequest>();
  
  // 设置消息处理函数
  httpChannel.onmessage = (request) => {
    console.log("收到 HTTP 请求:", request);
    // 添加新请求到列表开头
    httpRequests.value.unshift(request);
    
    // 限制列表长度，避免过长
    if (httpRequests.value.length > 100) {
      httpRequests.value = httpRequests.value.slice(0, 100);
    }
  };
  
  // 发送通道到后端
  try {
    await invoke("set_http_channel", { channel: httpChannel });
    console.log("HTTP 请求通道已设置");
  } catch (error) {
    console.error("设置 HTTP 请求通道失败:", error);
  }
}

// 启动捕获
async function startCapture() {
  try {
    // 如果没有安装ChmodBPF，显示警告
    if (!hasChmodBPF.value) {
      showChmodBPFWarning.value = true;
      return;
    }
    
    // 调用后端初始化捕获
    await invoke("init_packet_capture");
    
    // 状态更新会通过通道接收
  } catch (error) {
    console.error("启动捕获失败:", error);
  }
}

// 停止捕获
async function stopCapture() {
  try {
    await invoke("stop_packet_capture");
  } catch (error) {
    console.error("停止捕获失败:", error);
  }
}

// 清除 HTTP 请求
function clearHttpRequests() {
  httpRequests.value = [];
  selectedRequest.value = null;
  showDetails.value = false;
}

// 查看请求详情
function viewRequestDetails(request: HttpRequest) {
  selectedRequest.value = request;
  showDetails.value = true;
}

// 关闭详情
function closeDetails() {
  showDetails.value = false;
}

// 关闭ChmodBPF警告
function closeChmodBPFWarning() {
  showChmodBPFWarning.value = false;
}

// 格式化时间戳
function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString();
}

// 格式化持续时间
function formatDuration(startTime: number): string {
  if (!startTime) return "未知";
  const now = Math.floor(Date.now() / 1000);
  const seconds = now - startTime;
  
  if (seconds < 60) return `${seconds}秒`;
  if (seconds < 3600) return `${Math.floor(seconds / 60)}分${seconds % 60}秒`;
  return `${Math.floor(seconds / 3600)}小时${Math.floor((seconds % 3600) / 60)}分`;
}

// 组件挂载时设置通道
onMounted(async () => {
  // 检查ChmodBPF状态
  await checkChmodBPF();
  
  // 获取初始状态
  await fetchCaptureStatus();
  
  // 设置通道
  await setupStatusChannel();
  await setupHttpChannel();
});

// 组件卸载时清理通道
onUnmounted(() => {
  // 在 Tauri v2 中，Channel 不需要手动关闭
  // 当组件卸载时，通道会自动被垃圾回收
  statusChannel = null;
  httpChannel = null;
});
</script>

<template>
  <div class="container">
    <h1>HTTP 数据包捕获</h1>
    
    <!-- ChmodBPF警告对话框 -->
    <div v-if="showChmodBPFWarning" class="chmodbpf-warning">
      <div class="warning-content">
        <div class="warning-icon">⚠️</div>
        <div class="warning-message">
          <h3>未检测到ChmodBPF</h3>
          <p>您需要安装ChmodBPF才能使用抓包功能。请安装ChmodBPF后重启应用。</p>
          <p>ChmodBPF可以使普通用户获得抓包权限，无需超级用户权限。</p>
        </div>
        <div class="warning-actions">
          <button class="btn btn-primary" @click="closeChmodBPFWarning">我知道了</button>
        </div>
      </div>
    </div>
    
    <!-- ChmodBPF状态 -->
    <div class="chmodbpf-status">
      <span :class="{ 'chmodbpf-installed': hasChmodBPF, 'chmodbpf-missing': !hasChmodBPF }">
        {{ hasChmodBPF ? 'ChmodBPF已安装' : '未安装ChmodBPF，无法使用抓包功能' }}
      </span>
    </div>
    
    <!-- 捕获状态卡片 -->
    <div class="status-card" :class="{ 'status-running': captureStatus.running, 'status-stopped': !captureStatus.running }">
      <div class="status-header">
        <h2>捕获状态</h2>
        <span class="status-badge" :class="{ 'status-running': captureStatus.running, 'status-stopped': !captureStatus.running }">
          {{ captureStatus.running ? '运行中' : '已停止' }}
        </span>
      </div>
      <div class="status-info">
        <p><strong>状态消息:</strong> {{ captureStatus.message }}</p>
        <p><strong>网络设备:</strong> {{ captureStatus.device_name }}</p>
        <p v-if="captureStatus.start_time">
          <strong>开始时间:</strong> {{ formatTimestamp(captureStatus.start_time) }}
          (运行时间: {{ formatDuration(captureStatus.start_time) }})
        </p>
      </div>
      <div class="status-actions">
        <button 
          class="action-btn start-btn" 
          @click="startCapture" 
          :disabled="captureStatus.running || !hasChmodBPF">
          开始捕获
        </button>
        <button 
          class="action-btn stop-btn" 
          @click="stopCapture" 
          :disabled="!captureStatus.running">
          停止捕获
        </button>
      </div>
    </div>
    
    <!-- HTTP 请求列表 -->
    <div class="http-requests-section">
      <div class="section-header">
        <h2>HTTP 请求列表</h2>
        <button class="clear-btn" @click="clearHttpRequests">清除列表</button>
      </div>
      
      <div class="requests-container">
        <div v-if="httpRequests.length === 0" class="no-requests">
          <p>暂无捕获的 HTTP 请求</p>
        </div>
        <table v-else class="requests-table">
          <thead>
            <tr>
              <th>时间</th>
              <th>方法</th>
              <th>主机</th>
              <th>路径</th>
              <th>来源</th>
              <th>操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="request in httpRequests" :key="request.id">
              <td>{{ formatTimestamp(request.timestamp) }}</td>
              <td class="method-cell" :class="`method-${request.method.toLowerCase()}`">{{ request.method }}</td>
              <td>{{ request.host || request.dst_ip }}</td>
              <td class="path-cell">{{ request.path }}</td>
              <td>{{ request.src_ip }}:{{ request.src_port }}</td>
              <td>
                <button class="view-btn" @click="viewRequestDetails(request)">查看</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    
    <!-- 请求详情弹窗 -->
    <div v-if="showDetails && selectedRequest" class="request-details-overlay">
      <div class="request-details-modal">
        <div class="modal-header">
          <h3>HTTP 请求详情</h3>
          <button class="close-btn" @click="closeDetails">×</button>
        </div>
        <div class="modal-content">
          <div class="detail-group">
            <h4>基本信息</h4>
            <p><strong>时间:</strong> {{ formatTimestamp(selectedRequest.timestamp) }}</p>
            <p><strong>方法:</strong> <span :class="`method-${selectedRequest.method.toLowerCase()}`">{{ selectedRequest.method }}</span></p>
            <p><strong>路径:</strong> {{ selectedRequest.path }}</p>
            <p><strong>HTTP 版本:</strong> {{ selectedRequest.version }}</p>
            <p><strong>主机:</strong> {{ selectedRequest.host }}</p>
          </div>
          
          <div class="detail-group">
            <h4>网络信息</h4>
            <p><strong>来源:</strong> {{ selectedRequest.src_ip }}:{{ selectedRequest.src_port }}</p>
            <p><strong>目标:</strong> {{ selectedRequest.dst_ip }}:{{ selectedRequest.dst_port }}</p>
          </div>
          
          <div class="detail-group">
            <h4>请求头</h4>
            <div class="headers-list">
              <div v-for="(header, index) in selectedRequest.headers" :key="index" class="header-item">
                <span class="header-name">{{ header[0] }}:</span>
                <span class="header-value">{{ header[1] }}</span>
              </div>
            </div>
          </div>
          
          <div v-if="selectedRequest.body" class="detail-group">
            <h4>请求体</h4>
            <pre class="request-body">{{ selectedRequest.body }}</pre>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style>
.container {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

h1 {
  text-align: center;
  margin-bottom: 20px;
}

/* ChmodBPF警告对话框样式 */
.chmodbpf-warning {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.warning-content {
  background-color: white;
  padding: 20px;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  width: 100%;
  text-align: center;
}

.warning-icon {
  font-size: 48px;
  margin-bottom: 20px;
  color: #ff9800;
}

.warning-message h3 {
  color: #ff9800;
  margin-bottom: 10px;
}

.warning-actions {
  margin-top: 20px;
  display: flex;
  justify-content: center;
}

.btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
}

.btn-primary {
  background-color: #2196f3;
  color: white;
}

/* ChmodBPF状态样式 */
.chmodbpf-status {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  margin-bottom: 10px;
}

.chmodbpf-installed {
  color: #4caf50;
  font-weight: bold;
}

.chmodbpf-missing {
  color: #f44336;
  font-weight: bold;
}

/* 捕获状态卡片样式 */
.status-card {
  background-color: #f8f9fa;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  margin-bottom: 20px;
  overflow: hidden;
  border-left: 5px solid #ccc;
}

.status-card.status-running {
  border-left-color: #4caf50;
}

.status-card.status-stopped {
  border-left-color: #f44336;
}

.status-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  background-color: #f1f3f5;
  border-bottom: 1px solid #e9ecef;
}

.status-header h2 {
  margin: 0;
  font-size: 18px;
}

.status-badge {
  padding: 5px 10px;
  border-radius: 4px;
  font-weight: bold;
  font-size: 14px;
}

.status-badge.status-running {
  background-color: #4caf50;
  color: white;
}

.status-badge.status-stopped {
  background-color: #f44336;
  color: white;
}

.status-info {
  padding: 15px 20px;
}

.status-info p {
  margin: 8px 0;
}

.status-actions {
  display: flex;
  gap: 10px;
  padding: 0 20px 15px 20px;
}

.action-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  font-weight: bold;
  cursor: pointer;
  transition: background-color 0.2s;
}

.start-btn {
  background-color: #4caf50;
  color: white;
}

.start-btn:hover:not(:disabled) {
  background-color: #3d9140;
}

.stop-btn {
  background-color: #f44336;
  color: white;
}

.stop-btn:hover:not(:disabled) {
  background-color: #d32f2f;
}

.action-btn:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

/* HTTP 请求列表样式 */
.http-requests-section {
  background-color: #f8f9fa;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  overflow: hidden;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  background-color: #f1f3f5;
  border-bottom: 1px solid #e9ecef;
}

.section-header h2 {
  margin: 0;
  font-size: 18px;
}

.clear-btn {
  padding: 5px 10px;
  background-color: #e9ecef;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.clear-btn:hover {
  background-color: #dee2e6;
}

.requests-container {
  max-height: 500px;
  overflow-y: auto;
}

.no-requests {
  padding: 30px;
  text-align: center;
  color: #6c757d;
}

.requests-table {
  width: 100%;
  border-collapse: collapse;
}

.requests-table th, .requests-table td {
  padding: 10px 15px;
  text-align: left;
  border-bottom: 1px solid #e9ecef;
}

.requests-table th {
  background-color: #f8f9fa;
  font-weight: bold;
}

.method-cell {
  font-weight: bold;
  padding: 3px 8px;
  border-radius: 4px;
  display: inline-block;
  min-width: 60px;
  text-align: center;
}

.method-get {
  background-color: #4caf50;
  color: white;
}

.method-post {
  background-color: #2196f3;
  color: white;
}

.method-put {
  background-color: #ff9800;
  color: white;
}

.method-delete {
  background-color: #f44336;
  color: white;
}

.path-cell {
  max-width: 250px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.view-btn {
  padding: 3px 8px;
  background-color: #2196f3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.view-btn:hover {
  background-color: #0b7dda;
}

/* 请求详情弹窗样式 */
.request-details-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.request-details-modal {
  background-color: white;
  border-radius: 8px;
  width: 80%;
  max-width: 800px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  background-color: #f8f9fa;
  border-bottom: 1px solid #e9ecef;
}

.modal-header h3 {
  margin: 0;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #6c757d;
}

.close-btn:hover {
  color: #343a40;
}

.modal-content {
  padding: 20px;
  overflow-y: auto;
}

.detail-group {
  margin-bottom: 20px;
}

.detail-group h4 {
  margin-top: 0;
  margin-bottom: 10px;
  padding-bottom: 5px;
  border-bottom: 1px solid #e9ecef;
}

.headers-list {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.header-item {
  display: flex;
}

.header-name {
  font-weight: bold;
  margin-right: 8px;
  min-width: 120px;
}

.request-body {
  background-color: #f8f9fa;
  padding: 10px;
  border-radius: 4px;
  overflow-x: auto;
  font-family: monospace;
  margin: 0;
}
</style>
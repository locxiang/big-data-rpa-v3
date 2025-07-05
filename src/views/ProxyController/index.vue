<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import PacketTable from './components/PacketTable.vue';
import { useProxyStore } from '../../stores/proxyStore.ts';
import './animations.css';

const router = useRouter();
const proxyStore = useProxyStore();

const goBack = () => {
  router.push('/');
};

onMounted(() => {
  proxyStore.initialize();
});

onUnmounted(() => {
  // 关闭窗口但是不关闭抓包
  // proxyStore.cleanup();
});
</script>

<template>
  <div class="h-screen overflow-hidden relative text-slate-200 bg-gradient-to-br from-slate-900 via-slate-800 to-slate-700">
    <!-- 粒子动画背景 -->
    <div class="absolute inset-0 z-0 pointer-events-none particles-bg animate-particle-float"></div>
    
    <!-- 动态背景网格 -->
    <div class="absolute inset-0 z-[1] pointer-events-none grid-pattern animate-grid-float"></div>
    <div class="absolute inset-0 z-[2] pointer-events-none bg-gradient-to-br from-blue-500/5 via-purple-500/3 to-cyan-500/5 animate-overlay-pulse"></div>
    
    <!-- 科技装饰元素 -->
    <div class="absolute inset-0 z-[3] pointer-events-none">
      <div class="absolute top-[10%] right-[15%] w-48 h-48 circuit-pattern opacity-60 animate-circuit-glow rotate-[15deg]"></div>
      <div class="absolute bottom-[20%] left-[10%] w-48 h-48 circuit-pattern opacity-60 animate-circuit-glow -rotate-[25deg] animation-delay-2s"></div>
      <div class="absolute top-[20%] left-[25%] w-0.5 h-24 data-stream animate-data-flow"></div>
      <div class="absolute top-[60%] right-[30%] w-0.5 h-24 data-stream animate-data-flow animation-delay-1_5s"></div>
    </div>

    <!-- 顶部导航栏 -->
    <div class="h-15 flex items-center justify-between px-6 bg-gradient-to-r from-slate-900/95 to-slate-800/90 border-b border-blue-500/25 backdrop-blur-2xl relative z-[100] shadow-2xl">
      <!-- 动画扫描线 -->
      <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-blue-500/80 to-transparent animate-header-scan"></div>
      
      <div class="flex items-center gap-6">
        <button 
          @click="goBack" 
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-500/10 to-cyan-500/10 border border-blue-500/30 rounded-md text-slate-200 hover:from-blue-500/20 hover:to-cyan-500/20 hover:border-blue-500/50 hover:-translate-y-0.5 transition-all duration-300 relative overflow-hidden group"
        >
          <div class="text-lg font-bold">←</div>
          <span class="text-sm">返回控制台</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>
        
        <div class="flex items-center gap-3">
          <div class="text-2xl animate-icon-glow">📡</div>
          <h1 class="text-xl font-bold text-slate-200 font-mono tracking-wide">网络抓包分析器</h1>
        </div>
      </div>
      
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-3 px-4 py-2 bg-gradient-to-r from-slate-900/80 to-slate-800/60 border border-blue-500/30 rounded-md backdrop-blur-lg">
          <div :class="['w-3 h-3 rounded-full relative border-2 shadow-lg', 
                        proxyStore.captureStatus.running ? 'bg-green-500 border-green-500/50 shadow-green-500/50' : 'bg-red-500 border-red-500/50 shadow-red-500/50']">
            <div v-if="proxyStore.captureStatus.running" class="absolute inset-0 bg-green-500 rounded-full animate-pulse-custom"></div>
            <div v-else class="absolute inset-0 bg-red-500 rounded-full animate-status-blink"></div>
          </div>
          <span class="text-sm font-semibold">{{ proxyStore.captureStatus.running ? '抓包中' : '已停止' }}</span>
        </div>
        
        <button 
          @click="proxyStore.clearPackets" 
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded-md text-slate-200 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 hover:-translate-y-0.5 transition-all duration-300 relative overflow-hidden group"
        >
          <span class="text-lg">🧹</span>
          <span class="text-sm">清空记录</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="h-[calc(100vh-60px)] overflow-y-auto p-6 relative z-10">
      <!-- 错误提示 -->
      <div v-if="proxyStore.error" class="bg-gradient-to-br from-red-500/20 to-pink-500/20 backdrop-blur-2xl border border-red-500/30 rounded-xl p-4 mb-6 shadow-2xl flex items-center gap-4">
        <div class="text-2xl animate-icon-pulse">⚠️</div>
        <div class="flex flex-col">
          <span class="font-semibold text-red-300">发生错误</span>
          <span class="text-red-400 text-sm">{{ proxyStore.error }}</span>
        </div>
      </div>

      <!-- 控制面板 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-blue-500/20 rounded-xl p-6 mb-6 shadow-2xl hover:shadow-blue-500/10 hover:-translate-y-1 transition-all duration-300">
        <div class="mb-6">
          <div class="flex items-center gap-3 relative">
            <div class="text-2xl animate-icon-glow">🎛️</div>
            <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">抓包控制</h2>
            <div class="flex-1 h-0.5 bg-gradient-to-r from-blue-500/60 via-cyan-500/60 to-transparent ml-4 relative overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent animate-line-scan"></div>
            </div>
          </div>
        </div>
        
        <div class="flex items-center gap-6">
          <!-- 网络设备选择 - 占50%宽度 -->
          <div class="w-1/2 bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/10 rounded-lg p-4">
            <label class="block text-sm font-semibold text-slate-200 mb-3">选择网络设备</label>
            <select 
              v-model="proxyStore.selectedDevice"
              :disabled="proxyStore.captureStatus.running || proxyStore.isLoading"
              class="w-full px-4 py-3 bg-gradient-to-r from-slate-900/80 to-slate-800/60 border border-blue-500/30 rounded-lg text-slate-200 focus:outline-none focus:border-blue-500/60 focus:ring-2 focus:ring-blue-500/20 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed"
            >
              <option v-if="proxyStore.devices.length === 0" value="" disabled>未找到网络设备</option>
              <option v-for="device in proxyStore.devices" :key="device.name" :value="device.name">
                {{ device.name }} - {{ device.description }}
                {{ device.is_loopback ? ' (回环)' : '' }}
              </option>
            </select>
          </div>
          
          <!-- 控制按钮 - 占剩余50%宽度 -->
          <div class="flex-1 flex items-center gap-4">
            <button 
              @click="proxyStore.startCapture" 
              :disabled="!proxyStore.selectedDevice || proxyStore.captureStatus.running || proxyStore.isLoading"
              class="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-green-500/10 to-emerald-500/10 border border-green-500/30 rounded-lg text-slate-200 hover:from-green-500/20 hover:to-emerald-500/20 hover:border-green-500/50 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:translate-y-0 relative overflow-hidden group"
            >
              <span class="text-lg">▶️</span>
              <span class="font-semibold">{{ proxyStore.isLoading ? '启动中...' : '开始抓包' }}</span>
              <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
            </button>
            
            <button 
              @click="proxyStore.stopCapture" 
              :disabled="!proxyStore.captureStatus.running || proxyStore.isLoading"
              class="flex items-center gap-2 px-6 py-3 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded-lg text-slate-200 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:translate-y-0 relative overflow-hidden group"
            >
              <span class="text-lg">⏹️</span>
              <span class="font-semibold">{{ proxyStore.isLoading ? '停止中...' : '停止抓包' }}</span>
              <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
            </button>
          </div>
        </div>
      </div>



      <!-- 实时数据包列表 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-blue-500/20 rounded-xl shadow-2xl hover:shadow-blue-500/10 hover:-translate-y-1 transition-all duration-300 relative overflow-hidden">
        <div class="absolute top-0 left-0 right-0 h-0.5 bg-gradient-to-r from-transparent via-blue-500/80 to-transparent animate-packet-scan"></div>
        
        <div class="p-6 border-b border-blue-500/20">
          <div class="flex items-center justify-between">
            <div class="flex items-center gap-3">
              <div class="text-2xl animate-icon-glow">📡</div>
              <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">实时HTTP数据包</h2>
              <div class="flex items-center gap-2 px-3 py-1 bg-gradient-to-r from-blue-500/20 to-cyan-500/20 border border-blue-500/30 rounded-full">
                <div class="w-2 h-2 bg-cyan-400 rounded-full animate-pulse-custom"></div>
                <span class="text-sm font-semibold text-cyan-400">{{ proxyStore.packets.length }}</span>
              </div>
            </div>
            
            <div class="flex items-center gap-4">
              <div v-if="proxyStore.captureStatus.running" class="text-sm text-slate-400 font-mono">
                实时监控中...
              </div>
            </div>
          </div>
        </div>
        
        <PacketTable :packets="proxyStore.packets" />
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 动画导入已在 script 中处理 */
</style>
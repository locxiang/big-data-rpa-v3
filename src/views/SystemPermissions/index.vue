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
          <div class="text-2xl animate-icon-glow">🔐</div>
          <h1 class="text-xl font-bold text-slate-200 font-mono tracking-wide">系统权限管理</h1>
        </div>
      </div>
      
      <div class="flex items-center gap-4">
        <button 
          @click="handleRefresh" 
          :disabled="authStore.isLoading"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-blue-500/10 to-cyan-500/10 border border-blue-500/30 rounded-md text-slate-200 hover:from-blue-500/20 hover:to-cyan-500/20 hover:border-blue-500/50 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed relative overflow-hidden group"
        >
          <span class="text-lg">🔄</span>
          <span class="text-sm">{{ authStore.isLoading ? '刷新中...' : '刷新状态' }}</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>
        
        <button 
          @click="handleClearAll" 
          :disabled="authStore.isLoading"
          class="flex items-center gap-2 px-4 py-2 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded-md text-slate-200 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed relative overflow-hidden group"
        >
          <span class="text-lg">🗑️</span>
          <span class="text-sm">清除所有Token</span>
          <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
        </button>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="h-[calc(100vh-40px)] overflow-y-auto p-6 relative z-10">
      <!-- 权限概览 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-blue-500/20 rounded-xl p-6 mb-6 shadow-2xl hover:shadow-blue-500/10 hover:-translate-y-1 transition-all duration-300">
        <div class="mb-6">
          <div class="flex items-center gap-3 relative">
            <div class="text-2xl animate-icon-glow">📊</div>
            <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">权限概览</h2>
            <div class="flex-1 h-0.5 bg-gradient-to-r from-blue-500/60 via-cyan-500/60 to-transparent ml-4 relative overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent animate-line-scan"></div>
            </div>
          </div>
        </div>
        
        <div class="grid grid-cols-4 gap-4">
          <!-- 活跃Token -->
          <div class="bg-gradient-to-br from-slate-900/80 to-slate-800/60 backdrop-blur-lg border border-blue-500/15 rounded-lg p-4 hover:shadow-lg hover:shadow-blue-500/20 hover:-translate-y-1 transition-all duration-300 cursor-pointer relative overflow-hidden group">
            <div class="absolute inset-0 bg-gradient-to-r from-blue-500/5 to-cyan-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            <div class="flex items-center gap-3 mb-3 relative z-10">
              <div class="text-2xl animate-icon-pulse">✅</div>
              <div class="w-3 h-3 bg-green-500 rounded-full relative border-2 border-green-500/50 shadow-lg shadow-green-500/50">
                <div class="absolute inset-0 bg-green-500 rounded-full animate-pulse-custom"></div>
              </div>
            </div>
            <div class="relative z-10">
              <div class="text-2xl font-bold text-green-400 font-mono drop-shadow-lg">{{ authStore.activeTokensCount }}</div>
              <div class="text-slate-400 text-sm font-medium">活跃Token</div>
            </div>
          </div>
          
          <!-- 过期Token -->
          <div class="bg-gradient-to-br from-slate-900/80 to-slate-800/60 backdrop-blur-lg border border-blue-500/15 rounded-lg p-4 hover:shadow-lg hover:shadow-amber-500/20 hover:-translate-y-1 transition-all duration-300 cursor-pointer relative overflow-hidden group">
            <div class="absolute inset-0 bg-gradient-to-r from-amber-500/5 to-orange-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            <div class="flex items-center gap-3 mb-3 relative z-10">
              <div class="text-2xl animate-icon-pulse">⏰</div>
              <div class="w-3 h-3 bg-amber-500 rounded-full border-2 border-amber-500/50 shadow-lg shadow-amber-500/50"></div>
            </div>
            <div class="relative z-10">
              <div class="text-2xl font-bold text-amber-400 font-mono drop-shadow-lg">{{ authStore.expiredTokensCount }}</div>
              <div class="text-slate-400 text-sm font-medium">过期Token</div>
            </div>
          </div>
          
          <!-- 等待获取 -->
          <div class="bg-gradient-to-br from-slate-900/80 to-slate-800/60 backdrop-blur-lg border border-blue-500/15 rounded-lg p-4 hover:shadow-lg hover:shadow-cyan-500/20 hover:-translate-y-1 transition-all duration-300 cursor-pointer relative overflow-hidden group">
            <div class="absolute inset-0 bg-gradient-to-r from-cyan-500/5 to-blue-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            <div class="flex items-center gap-3 mb-3 relative z-10">
              <div class="text-2xl animate-icon-pulse">⏳</div>
              <div class="w-3 h-3 bg-cyan-500 rounded-full border-2 border-cyan-500/50 shadow-lg shadow-cyan-500/50"></div>
            </div>
            <div class="relative z-10">
              <div class="text-2xl font-bold text-cyan-400 font-mono drop-shadow-lg">{{ authStore.waitingTokensCount }}</div>
              <div class="text-slate-400 text-sm font-medium">等待获取</div>
            </div>
          </div>
          
          <!-- 失败Token -->
          <div class="bg-gradient-to-br from-slate-900/80 to-slate-800/60 backdrop-blur-lg border border-blue-500/15 rounded-lg p-4 hover:shadow-lg hover:shadow-red-500/20 hover:-translate-y-1 transition-all duration-300 cursor-pointer relative overflow-hidden group">
            <div class="absolute inset-0 bg-gradient-to-r from-red-500/5 to-pink-500/5 opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
            <div class="flex items-center gap-3 mb-3 relative z-10">
              <div class="text-2xl animate-icon-pulse">❌</div>
              <div class="w-3 h-3 bg-red-500 rounded-full border-2 border-red-500/50 shadow-lg shadow-red-500/50"></div>
            </div>
            <div class="relative z-10">
              <div class="text-2xl font-bold text-red-400 font-mono drop-shadow-lg">{{ authStore.failedTokensCount }}</div>
              <div class="text-slate-400 text-sm font-medium">失败Token</div>
            </div>
          </div>
        </div>
      </div>

      <!-- 系统状态列表 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-blue-500/20 rounded-xl p-6 mb-6 shadow-2xl hover:shadow-blue-500/10 hover:-translate-y-1 transition-all duration-300">
        <div class="mb-6">
          <div class="flex items-center gap-3 relative">
            <div class="text-2xl animate-icon-glow">🖥️</div>
            <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">系统状态</h2>
            <div class="flex-1 h-0.5 bg-gradient-to-r from-blue-500/60 via-cyan-500/60 to-transparent ml-4 relative overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent animate-line-scan"></div>
            </div>
          </div>
        </div>
        
        <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/10 rounded-lg overflow-hidden">
          <table class="w-full">
            <thead>
              <tr class="bg-gradient-to-r from-slate-900/80 to-slate-800/60 backdrop-blur-lg">
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">系统名称</th>
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">状态</th>
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">剩余时间</th>
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">获取时间</th>
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="system in authStore.tokenStatuses" :key="system.system_id" class="hover:bg-blue-500/5 transition-colors duration-200">
                <td class="px-4 py-3 border-b border-blue-500/5">{{ system.system_name }}</td>
                <td class="px-4 py-3 border-b border-blue-500/5">
                  <span 
                    :class="[
                      'inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wide border transition-all duration-300 relative overflow-hidden',
                      getStatusClasses(system.status)
                    ]"
                  >
                    {{ authStore.formatTokenStatus(system.status) }}
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full hover:translate-x-full transition-transform duration-500"></div>
                  </span>
                </td>
                <td class="px-4 py-3 border-b border-blue-500/5">
                  <span 
                    v-if="system.status === 'Active'" 
                    :class="[
                      'font-semibold font-mono transition-all duration-300',
                      getRemainingTimeClass(system.system_id)
                    ]"
                  >
                    {{ getRemainingTimeText(system.system_id) }}
                  </span>
                  <span v-else class="text-slate-500 italic">-</span>
                </td>
                <td class="px-4 py-3 border-b border-blue-500/5">
                  <span v-if="system.token_acquired_at" class="text-cyan-400 font-mono text-sm">
                    {{ formatTokenAcquiredTime(system.token_acquired_at) }}
                  </span>
                  <span v-else class="text-slate-500 italic">未获取</span>
                </td>
                <td class="px-4 py-3 border-b border-blue-500/5">
                  <button 
                    @click="handleClearSystem(system.system_id)"
                    :disabled="authStore.isLoading"
                    class="flex items-center gap-1 px-3 py-1 bg-gradient-to-r from-red-500/10 to-pink-500/10 border border-red-500/30 rounded text-xs text-slate-200 hover:from-red-500/20 hover:to-pink-500/20 hover:border-red-500/50 hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-60 disabled:cursor-not-allowed relative overflow-hidden group"
                  >
                    <span class="text-sm">🗑️</span>
                    <span>清除</span>
                    <div class="absolute inset-0 bg-gradient-to-r from-white/0 via-white/5 to-white/0 -translate-x-full group-hover:translate-x-full transition-transform duration-500"></div>
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Token历史记录 -->
      <div class="bg-gradient-to-br from-slate-900/95 to-slate-800/90 backdrop-blur-2xl border border-blue-500/20 rounded-xl p-6 shadow-2xl hover:shadow-blue-500/10 hover:-translate-y-1 transition-all duration-300">
        <div class="mb-6">
          <div class="flex items-center gap-3 relative">
            <div class="text-2xl animate-icon-glow">📚</div>
            <h2 class="text-lg font-semibold text-slate-200 font-mono tracking-wide">Token历史记录</h2>
            <div class="flex-1 h-0.5 bg-gradient-to-r from-blue-500/60 via-cyan-500/60 to-transparent ml-4 relative overflow-hidden">
              <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/40 to-transparent animate-line-scan"></div>
            </div>
          </div>
        </div>
        
        <div class="bg-gradient-to-br from-slate-900/60 to-slate-800/40 backdrop-blur-lg border border-blue-500/10 rounded-lg overflow-hidden">
          <table class="w-full">
            <thead>
              <tr class="bg-gradient-to-r from-slate-900/80 to-slate-800/60 backdrop-blur-lg">
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">时间</th>
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">系统</th>
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">事件</th>
                <th class="px-4 py-3 text-left font-semibold text-slate-200 font-mono tracking-wide border-b border-blue-500/10">详情</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="(event, index) in authStore.tokenEvents" :key="index" class="hover:bg-blue-500/5 transition-colors duration-200">
                <td class="px-4 py-3 border-b border-blue-500/5">{{ formatEventTime(event) }}</td>
                <td class="px-4 py-3 border-b border-blue-500/5">{{ getEventSystemName(event) }}</td>
                <td class="px-4 py-3 border-b border-blue-500/5">
                  <span 
                    :class="[
                      'inline-flex items-center px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wide border transition-all duration-300 relative overflow-hidden',
                      getEventClasses(event)
                    ]"
                  >
                    {{ getEventType(event) }}
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -translate-x-full hover:translate-x-full transition-transform duration-500"></div>
                  </span>
                </td>
                <td class="px-4 py-3 border-b border-blue-500/5">{{ getEventDetails(event) }}</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <!-- 错误信息 -->
    <div 
      v-if="authStore.error" 
      class="fixed top-5 right-5 bg-gradient-to-r from-red-500/90 to-pink-500/90 backdrop-blur-lg text-red-50 px-5 py-4 rounded-lg border border-red-500/50 shadow-2xl shadow-red-500/30 max-w-md z-[1000] animate-slide-in"
    >
      <div class="flex items-center gap-3">
        <span class="text-xl">⚠️</span>
        <span class="flex-1">{{ authStore.error }}</span>
        <button 
          @click="authStore.clearError" 
          class="text-red-50 hover:text-red-200 text-2xl transition-colors duration-300"
        >
          ×
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '@/stores/authStore';
import type { TokenEvent, TokenState } from '@/stores/authStore';

const router = useRouter();
const authStore = useAuthStore();

// 当前时间用于实时倒计时
const currentTime = ref(Math.floor(Date.now() / 1000));
let timeUpdateInterval: number | null = null;

// 生命周期
onMounted(async () => {
  await authStore.initialize();
  
  // 启动定时器，每秒更新当前时间
  timeUpdateInterval = window.setInterval(() => {
    currentTime.value = Math.floor(Date.now() / 1000);
  }, 1000);
});

onUnmounted(() => {
  // 清理定时器
  if (timeUpdateInterval) {
    clearInterval(timeUpdateInterval);
    timeUpdateInterval = null;
  }
});

// 返回控制台
const goBack = () => {
  router.push('/');
};

// 刷新状态
const handleRefresh = async () => {
  try {
    await authStore.refreshTokenStatuses();
  } catch (error) {
    console.error('刷新失败:', error);
  }
};

// 清除所有Token
const handleClearAll = async () => {
  if (confirm('确定要清除所有系统的Token吗？')) {
    try {
      await authStore.clearAllTokens();
    } catch (error) {
      console.error('清除失败:', error);
    }
  }
};

// 清除单个系统Token
const handleClearSystem = async (systemId: string) => {
  if (confirm(`确定要清除该系统的Token吗？`)) {
    try {
      await authStore.clearSystemToken(systemId);
    } catch (error) {
      console.error('清除失败:', error);
    }
  }
};

// 获取状态样式类
const getStatusClasses = (status: TokenState): string => {
  if (status === 'Active') {
    return 'bg-green-500/20 border-green-500/30 text-green-400 shadow-green-500/20';
  }
  if (status === 'Expired') {
    return 'bg-amber-500/20 border-amber-500/30 text-amber-400 shadow-amber-500/20';
  }
  if (status === 'Waiting') {
    return 'bg-cyan-500/20 border-cyan-500/30 text-cyan-400 shadow-cyan-500/20';
  }
  if (typeof status === 'object' && 'Failed' in status) {
    return 'bg-red-500/20 border-red-500/30 text-red-400 shadow-red-500/20';
  }
  return 'bg-slate-500/20 border-slate-500/30 text-slate-400 shadow-slate-500/20';
};

// 获取事件样式类
const getEventClasses = (event: TokenEvent): string => {
  if ('TokenAcquired' in event) {
    return 'bg-green-500/20 border-green-500/30 text-green-400 shadow-green-500/20';
  }
  if ('TokenExpired' in event) {
    return 'bg-amber-500/20 border-amber-500/30 text-amber-400 shadow-amber-500/20';
  }
  if ('TokenFailed' in event) {
    return 'bg-red-500/20 border-red-500/30 text-red-400 shadow-red-500/20';
  }
  return 'bg-slate-500/20 border-slate-500/30 text-slate-400 shadow-slate-500/20';
};

// 获取剩余时间文本 (实时更新)
const getRemainingTimeText = (systemId: string): string => {
  // 找到对应的系统状态
  const systemStatus = authStore.tokenStatuses.find(status => status.system_id === systemId);
  
  if (!systemStatus || !systemStatus.token_expires_at || systemStatus.status !== 'Active') {
    return '-';
  }
  
  // 使用响应式的当前时间计算剩余时间
  const remaining = systemStatus.token_expires_at - currentTime.value;
  
  if (remaining <= 0) {
    return '已过期';
  }
  
  return authStore.formatRemainingTime(remaining);
};

// 获取剩余时间样式类 (根据紧迫程度)
const getRemainingTimeClass = (systemId: string): string => {
  // 找到对应的系统状态
  const systemStatus = authStore.tokenStatuses.find(status => status.system_id === systemId);
  
  if (!systemStatus || !systemStatus.token_expires_at || systemStatus.status !== 'Active') {
    return 'text-slate-500';
  }
  
  // 使用响应式的当前时间计算剩余时间
  const remaining = systemStatus.token_expires_at - currentTime.value;
  
  if (remaining <= 0) {
    return 'text-red-400 animate-pulse';
  } else if (remaining <= 60) { // 少于1分钟
    return 'text-red-400 animate-pulse drop-shadow-lg';
  } else if (remaining <= 300) { // 少于5分钟
    return 'text-amber-400 drop-shadow-lg';
  } else if (remaining <= 1800) { // 少于30分钟
    return 'text-yellow-400';
  } else {
    return 'text-green-400 drop-shadow-lg';
  }
};

// 格式化事件时间
const formatEventTime = (event: TokenEvent): string => {
  let timestamp: number;
  
  if ('TokenAcquired' in event) {
    timestamp = event.TokenAcquired.acquired_at;
  } else if ('TokenExpired' in event) {
    timestamp = event.TokenExpired.expired_at;
  } else if ('TokenFailed' in event) {
    timestamp = event.TokenFailed.failed_at;
  } else {
    return '-';
  }
  
  return new Date(timestamp * 1000).toLocaleString('zh-CN');
};

// 获取事件系统名
const getEventSystemName = (event: TokenEvent): string => {
  if ('TokenAcquired' in event) return event.TokenAcquired.system_name;
  if ('TokenExpired' in event) return event.TokenExpired.system_name;
  if ('TokenFailed' in event) return event.TokenFailed.system_name;
  return '-';
};

// 获取事件类型
const getEventType = (event: TokenEvent): string => {
  if ('TokenAcquired' in event) return 'Token获取';
  if ('TokenExpired' in event) return 'Token过期';
  if ('TokenFailed' in event) return 'Token失败';
  return '未知';
};

// 获取事件详情
const getEventDetails = (event: TokenEvent): string => {
  if ('TokenAcquired' in event) {
    return `从 ${event.TokenAcquired.source_url} 获取成功`;
  }
  if ('TokenExpired' in event) {
    return '已过期';
  }
  if ('TokenFailed' in event) {
    return event.TokenFailed.error;
  }
  return '-';
};

// 格式化Token获取时间
const formatTokenAcquiredTime = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
};
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@400;700;900&family=Exo+2:wght@300;400;500;600;700&display=swap');
@import './animations.css';
</style> 
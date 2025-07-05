<template>
  <div class="w-[1200px] h-[800px] mx-auto bg-slate-900 text-white font-sans overflow-hidden">
    <!-- 顶部导航栏 -->
    <header class="h-[60px] bg-gradient-to-r from-slate-800 to-slate-700 border-b border-slate-600 flex items-center justify-between px-6">
      <!-- 左侧品牌区域 -->
      <div class="flex items-center space-x-4">
        <div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-lg flex items-center justify-center">
          <span class="text-xl font-bold">RPA</span>
        </div>
        <div>
          <h1 class="text-lg font-bold">数字重庆业务数据巡查自动化系统</h1>
          <p class="text-xs text-slate-400">Advanced Automation Platform v2.1.0</p>
        </div>
      </div>
      
      <!-- 右侧操作区域 -->
      <div class="flex items-center">
        <div class="text-sm text-slate-300">
          {{ currentTime }}
        </div>
      </div>
    </header>

    <!-- 状态概览区域 -->
    <section class="h-[120px] bg-slate-800 border-b border-slate-600 p-4">
      <div class="grid grid-cols-4 gap-4 h-full">
        <!-- 系统权限状态 -->
        <div @click="openPermissionsPage"
             class="bg-slate-700 hover:bg-slate-600 rounded-lg p-4 cursor-pointer transition-colors duration-200 flex items-center space-x-3"
             :class="{
               'border-l-4 border-green-500': systemAuthStatus.color === 'green',
               'border-l-4 border-yellow-500': systemAuthStatus.color === 'yellow',
               'border-l-4 border-gray-500': systemAuthStatus.color === 'gray'
             }">
          <div class="text-3xl">🔐</div>
          <div class="flex-1">
            <div class="text-sm font-medium text-white">系统权限</div>
            <div class="text-xs mt-1" 
                 :class="{
                   'text-green-400': systemAuthStatus.color === 'green',
                   'text-yellow-400': systemAuthStatus.color === 'yellow',
                   'text-gray-400': systemAuthStatus.color === 'gray'
                 }">
              {{ systemAuthStatus.text }} ({{ systemAuthStatus.statusText }})
            </div>
          </div>
          <div class="w-3 h-3 rounded-full" 
               :class="{
                 'bg-green-500': systemAuthStatus.color === 'green',
                 'bg-yellow-500': systemAuthStatus.color === 'yellow',
                 'bg-gray-500': systemAuthStatus.color === 'gray'
               }"></div>
        </div>

        <!-- 运行状态 -->
        <div @click="openProxyWindow"
             class="bg-slate-700 hover:bg-slate-600 rounded-lg p-4 cursor-pointer transition-colors duration-200 flex items-center space-x-3"
             :class="{
               'border-l-4 border-green-500': runningStatus.color === 'green',
               'border-l-4 border-gray-500': runningStatus.color === 'gray'
             }">
          <div class="text-3xl">⚡</div>
          <div class="flex-1">
            <div class="text-sm font-medium text-white">运行状态</div>
            <div class="text-xs mt-1"
                 :class="{
                   'text-green-400': runningStatus.color === 'green',
                   'text-gray-400': runningStatus.color === 'gray'
                 }">
              {{ runningStatus.text }}
            </div>
          </div>
          <div class="w-3 h-3 rounded-full"
               :class="{
                 'bg-green-500': runningStatus.color === 'green',
                 'bg-gray-500': runningStatus.color === 'gray'
               }"></div>
        </div>

        <!-- 运行时长 -->
        <div class="bg-slate-700 rounded-lg p-4 flex items-center space-x-3 border-l-4 border-green-500">
          <div class="text-3xl">⏰</div>
          <div class="flex-1">
            <div class="text-sm font-medium text-white">运行时长</div>
            <div class="text-xs text-green-400 mt-1">{{ systemStatus.runTime }}</div>
          </div>
          <div class="w-3 h-3 rounded-full bg-green-500"></div>
        </div>

        <!-- 活跃模块 -->
        <div class="bg-slate-700 rounded-lg p-4 flex items-center space-x-3 border-l-4 border-purple-500">
          <div class="text-3xl">📊</div>
          <div class="flex-1">
            <div class="text-sm font-medium text-white">活跃模块</div>
            <div class="text-xs text-purple-400 mt-1">{{ systemStatus.activeModules }} 个运行中</div>
          </div>
          <div class="w-3 h-3 rounded-full bg-purple-500"></div>
        </div>
      </div>
    </section>

    <!-- 功能模块区域 -->
    <section class="h-[550px] bg-slate-850 p-6">
      <div class="mb-4">
        <h2 class="text-xl font-bold text-white flex items-center space-x-2">
          <span>🚀</span>
          <span>自动化模块</span>
        </h2>
        <p class="text-sm text-slate-400 mt-1">点击启动或管理自动化模块</p>
      </div>
      
      <div class="grid grid-cols-3 gap-4 h-[300px]">
        <div 
          v-for="module in automationModules.slice(0, 6)" 
          :key="module.id"
          class="bg-slate-700 hover:bg-slate-600 rounded-lg p-6 cursor-pointer transition-all duration-200 
                 flex flex-col items-center justify-center text-center space-y-3 group"
          :class="{
            'border-2 border-green-500 bg-green-500/10': module.status === 'running',
            'border-2 border-blue-500 bg-blue-500/10': module.status === 'ready',
            'border-2 border-slate-500': module.status === 'stopped'
          }"
          @click="handleModuleClick(module)">
          
          <!-- 模块图标 -->
          <div class="text-4xl mb-2">{{ module.icon }}</div>
          
          <!-- 模块信息 -->
          <div class="flex-1">
            <h3 class="text-lg font-bold text-white mb-2">{{ module.name }}</h3>
            <p class="text-sm text-slate-400 leading-relaxed">{{ module.description }}</p>
          </div>
          
          <!-- 状态和操作 -->
          <div class="flex items-center justify-between w-full">
            <div class="px-3 py-1 rounded-full text-xs font-medium"
                 :class="{
                   'bg-green-500/20 text-green-400': module.status === 'running',
                   'bg-blue-500/20 text-blue-400': module.status === 'ready',
                   'bg-slate-500/20 text-slate-400': module.status === 'stopped'
                 }">
              {{ getStatusText(module.status) }}
            </div>
            
            <button @click.stop="startModule(module)"
                    class="px-3 py-1 bg-blue-600 hover:bg-blue-500 text-white text-xs rounded-md 
                           transition-colors duration-200 flex items-center space-x-1">
              <span>启动</span>
              <span>→</span>
            </button>
          </div>
        </div>
      </div>
    </section>

    <!-- 底部授权信息区域 -->
    <section class="h-[40px] w-full bg-slate-800 border-t border-slate-600 px-6 flex items-center justify-between">
      <!-- 左侧授权信息 -->
      <div class="flex items-center space-x-6">
        <div class="flex items-center space-x-3">
          <div class="text-2xl">🛡️</div>
          <div>
            <span class="text-sm font-medium text-white">授权状态：</span>
            <span class="text-sm font-medium" :class="authInfo.isAuthorized ? 'text-green-400' : 'text-red-400'">
              {{ authInfo.isAuthorized ? '已授权' : '未授权' }}
            </span>
          </div>
        </div>
        
        <div class="text-sm text-slate-400">|</div>
        
        <div class="flex items-center space-x-4">
          <div>
            <span class="text-sm text-slate-400">许可类型：</span>
            <span class="text-sm font-medium text-white">{{ authInfo.licenseType }}</span>
          </div>
          
          <div>
            <span class="text-sm text-slate-400">版本：</span>
            <span class="text-sm font-medium text-blue-400">{{ authInfo.version }}</span>
          </div>
          
          <div>
            <span class="text-sm text-slate-400">到期时间：</span>
            <span class="text-sm font-medium text-white">{{ authInfo.expiryDate }}</span>
          </div>
          
          <div>
            <span class="text-sm text-slate-400">剩余：</span>
            <span class="text-sm font-medium" :class="authInfo.remainingDays < 30 ? 'text-red-400' : 'text-green-400'">
              {{ authInfo.remainingDays }} 天
            </span>
          </div>
        </div>
      </div>
      
      <!-- 右侧版权信息 -->
      <div class="text-sm text-slate-400">
        <span class="mr-2">©</span>
        <span class="font-medium">重庆秫米科技技术有限公司</span>
        <span class="ml-2">{{ new Date().getFullYear() }} All Rights Reserved</span>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { useProxyStore } from '../../stores/proxyStore';
import { useAuthStore } from '../../stores/authStore';

const router = useRouter();
const proxyStore = useProxyStore();
const authStore = useAuthStore();

// 当前时间
const currentTime = ref('');

// 系统状态数据
const systemStatus = ref({
  hasSystemAuth: true,
  runTime: '2天 15小时 32分钟',
  automationStatus: '运行中',
  activeModules: 2
});

// 系统权限状态计算属性
const systemAuthStatus = computed(() => {
  const totalSystems = authStore.tokenStatuses.length;
  const activeSystems = authStore.activeTokensCount;
  
  let status = 'unknown';
  let color = 'gray';
  let text = '未知';
  
  if (totalSystems === 0) {
    status = 'no-systems';
    color = 'gray';
    text = '无系统';
  } else if (activeSystems === 0) {
    status = 'no-tokens';
    color = 'gray';
    text = '无有效授权';
  } else if (activeSystems === totalSystems) {
    status = 'full-authorized';
    color = 'green';
    text = '全部授权';
  } else {
    status = 'partial-authorized';
    color = 'yellow';
    text = '部分授权';
  }
  
  return {
    status,
    color,
    text,
    totalSystems,
    activeSystems,
    statusText: totalSystems > 0 ? `${activeSystems}/${totalSystems}` : '0/0'
  };
});

// 运行状态计算属性
const runningStatus = computed(() => {
  const isCapturing = proxyStore.isCapturing;
  
  return {
    color: isCapturing ? 'green' : 'gray',
    text: isCapturing ? '网卡监控中' : '未开启监控',
    status: isCapturing ? 'active' : 'inactive'
  };
});

// 授权信息数据
const authInfo = ref({
  isAuthorized: true,
  expiryDate: '2024-12-31',
  version: 'v2.1.0',
  licenseType: '企业版',
  remainingDays: 125
});

// 自动化模块数据
const automationModules = ref([
  {
    id: 1,
    name: '自动化处理指标上屏',
    icon: '📊',
    status: 'ready',
    description: '自动化处理指标上屏'
  },
  {
    id: 2,
    name: '自动化巡捡体证指标',
    icon: '📈',
    status: 'running',
    description: '自动化巡捡体证指标检查数据是否有异常'
  },
  {
    id: 3,
    name: '数据同步异常处理',
    icon: '📁',
    status: 'stopped',
    description: '根据BI系统统计的异常数据，进行自动化处理'
  },
  {
    id: 4,
    name: 'xxxx 自动化处理',
    icon: '✅',
    status: 'stopped',
    description: 'xxxxxx 自动化处理'
  },
  {
    id: 5,
    name: 'xxxx 自动化处理',
    icon: '🚨',
    status: 'stopped',
    description: 'xxxxx 处理自动化'
  },
  {
    id: 6,
    name: 'xxxx 自动化处理',
    icon: '📋',
    status: 'ready',
    description: 'xxxxx 处理自动化'
  }
]);

// 时间更新定时器
let timeTimer: number | null = null;

// 更新时间
const updateTime = () => {
  const now = new Date();
  currentTime.value = now.toLocaleTimeString('zh-CN', { 
    hour12: false,
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit'
  });
};

// 获取状态文本
const getStatusText = (status: string) => {
  switch (status) {
    case 'running': return '运行中';
    case 'ready': return '就绪';
    case 'stopped': return '已停止';
    default: return '未知';
  }
};

// 处理模块点击
const handleModuleClick = (module: any) => {
  console.log('点击了模块:', module.name);
  // 可以根据不同模块进行不同的处理
  if (module.id === 1) {
    // 数据申报系统 - 跳转到代理工具
    openProxyWindow();
  } else {
    // 其他模块的处理逻辑
    console.log(`模块 ${module.name} 功能开发中...`);
  }
};

// 启动模块
const startModule = (module: any) => {
  console.log('启动模块:', module.name);
  // 这里可以添加启动模块的逻辑
  // 例如调用后端的自动化脚本
  if (module.status === 'ready') {
    // 更新模块状态为运行中
    module.status = 'running';
    systemStatus.value.activeModules++;
    // 这里可以调用 Tauri 命令执行自动化脚本
    // invoke('start_automation_module', { moduleId: module.id });
  }
};

// 打开代理窗口 - 使用 Vue Router 替代 Electron API
const openProxyWindow = () => {
  router.push('/proxy');
};

// 打开系统权限页面
const openPermissionsPage = () => {
  router.push('/permissions');
};

// 检查系统权限和状态
const checkSystemStatus = async () => {
  try {
    // 初始化 authStore
    if (!authStore.tokenStatuses.length) {
      await authStore.initialize();
    }
    
    // 刷新 token 状态
    await authStore.refreshTokenStatuses();
    
    // 检查网络捕获权限
    const hasPermission = await proxyStore.checkPermissions();
    systemStatus.value.hasSystemAuth = hasPermission;
    
    // 获取捕获状态
    await proxyStore.getCaptureStatus();
    
    // 更新系统状态
    systemStatus.value.automationStatus = proxyStore.isCapturing ? '运行中' : '已停止';
    systemStatus.value.activeModules = proxyStore.isCapturing ? 1 : 0;
    
  } catch (error) {
    console.error('检查系统状态失败:', error);
    systemStatus.value.hasSystemAuth = false;
  }
};

onMounted(async () => {
  updateTime();
  timeTimer = setInterval(updateTime, 1000);
  
  // 检查系统状态
  await checkSystemStatus();
  
  // 定期刷新系统权限状态和运行状态
  setInterval(async () => {
    try {
      await authStore.refreshTokenStatuses();
      await proxyStore.getCaptureStatus();
    } catch (error) {
      console.error('刷新系统状态失败:', error);
    }
  }, 10000); // 每10秒刷新一次
});

onUnmounted(() => {
  if (timeTimer) {
    clearInterval(timeTimer);
  }
});
</script>

<style scoped>
/* 自定义样式 */
.bg-slate-850 {
  background-color: #1e293b;
}

/* 自定义滚动条样式 */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: rgba(51, 65, 85, 0.3);
  border-radius: 3px;
}

::-webkit-scrollbar-thumb {
  background: linear-gradient(to bottom, rgba(59, 130, 246, 0.6), rgba(6, 182, 212, 0.6));
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(to bottom, rgba(59, 130, 246, 0.8), rgba(6, 182, 212, 0.8));
}
</style> 
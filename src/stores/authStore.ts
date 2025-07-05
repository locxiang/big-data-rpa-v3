import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke, Channel } from '@tauri-apps/api/core';

// Token状态枚举
export type TokenState = 
  | 'Waiting'           // 等待获取
  | 'Active'            // 已获取，有效
  | 'Expired'           // 已过期
  | { Failed: string }; // 获取失败

// Token状态接口
export interface TokenStatus {
  system_id: string;
  system_name: string;
  has_token: boolean;
  token_acquired_at?: number;
  token_expires_at?: number;
  last_seen_url?: string;
  status: TokenState;
}

// Token事件类型
export type TokenEvent = 
  | {
      TokenAcquired: {
        system_id: string;
        system_name: string;
        token: string;
        acquired_at: number;
        expires_at: number;
        source_url: string;
      };
    }
  | {
      TokenExpired: {
        system_id: string;
        system_name: string;
        expired_at: number;
      };
    }
  | {
      TokenFailed: {
        system_id: string;
        system_name: string;
        error: string;
        failed_at: number;
      };
    };

export const useAuthStore = defineStore('auth', () => {
  // 状态
  const tokenStatuses = ref<TokenStatus[]>([]);
  const tokenEvents = ref<TokenEvent[]>([]);
  const error = ref<string>('');
  const isLoading = ref<boolean>(false);
  
  // 计算属性
  const activeTokensCount = computed(() => 
    tokenStatuses.value.filter(status => status.status === 'Active').length
  );
  
  const expiredTokensCount = computed(() =>
    tokenStatuses.value.filter(status => status.status === 'Expired').length
  );
  
  const waitingTokensCount = computed(() =>
    tokenStatuses.value.filter(status => status.status === 'Waiting').length
  );
  
  const failedTokensCount = computed(() =>
    tokenStatuses.value.filter(status => 
      typeof status.status === 'object' && 'Failed' in status.status
    ).length
  );
  
  // 获取特定系统的状态
  const getSystemStatus = computed(() => (systemId: string) => 
    tokenStatuses.value.find(status => status.system_id === systemId)
  );
  
  // 检查系统是否有有效token
  const hasValidToken = computed(() => (systemId: string) => {
    const status = getSystemStatus.value(systemId);
    return status?.status === 'Active';
  });
  
  // 初始化
  const initialize = async () => {
    try {
      isLoading.value = true;
      error.value = '';
      
      console.log('🔐 开始初始化Auth Store...');
      
      // 设置Token事件通道
      const tokenEventChannel = new Channel<TokenEvent>();
      tokenEventChannel.onmessage = async (event: TokenEvent) => {
        console.log('收到Token事件:', event);
        tokenEvents.value.unshift(event);
        
        // 限制事件历史数量
        if (tokenEvents.value.length > 100) {
          tokenEvents.value = tokenEvents.value.slice(0, 100);
        }
        
        // 更新对应系统的状态
        await refreshTokenStatuses();
      };
      
      // 发送通道到后端
      await invoke('set_token_event_channel', { channel: tokenEventChannel });
      console.log('✅ Token事件通道设置成功');
      
      // 获取初始状态
      await refreshTokenStatuses();
      
      // 获取事件历史
      await loadTokenEventHistory();
      
      console.log('🎉 Auth Store初始化完成');
      
    } catch (err) {
      error.value = `Auth Store初始化失败: ${err}`;
      console.error('Auth Store初始化失败:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  // 刷新所有系统的token状态
  const refreshTokenStatuses = async () => {
    try {
      console.log('🔄 刷新所有系统token状态...');
      const statuses = await invoke('get_all_token_status') as TokenStatus[];
      tokenStatuses.value = statuses;
      console.log(`📊 获取到 ${statuses.length} 个系统状态`);
      return statuses; // 返回获取到的状态数据
    } catch (err) {
      console.error('刷新token状态失败:', err);
      throw err;
    }
  };
  
  // 获取特定系统的token
  const getSystemToken = async (systemId: string): Promise<string | null> => {
    try {
      console.log(`🔍 获取系统 [${systemId}] 的token...`);
      const token = await invoke('get_system_token', { systemId }) as string | null;
      
      if (token) {
        console.log(`✅ 系统 [${systemId}] token可用，长度: ${token.length}`);
      } else {
        console.log(`❌ 系统 [${systemId}] token不可用`);
      }
      
      return token;
    } catch (err) {
      console.error(`获取系统 [${systemId}] token失败:`, err);
      throw err;
    }
  };
  
  // 清除特定系统的token
  const clearSystemToken = async (systemId: string) => {
    try {
      isLoading.value = true;
      console.log(`🗑️ 清除系统 [${systemId}] 的token...`);
      
      await invoke('clear_system_token', { systemId });
      console.log(`✅ 系统 [${systemId}] token已清除`);
      
      // 刷新状态
      await refreshTokenStatuses();
      
    } catch (err) {
      error.value = `清除系统 [${systemId}] token失败: ${err}`;
      console.error(`清除系统 [${systemId}] token失败:`, err);
    } finally {
      isLoading.value = false;
    }
  };
  
  // 清除所有系统的token
  const clearAllTokens = async () => {
    try {
      isLoading.value = true;
      console.log('🗑️ 清除所有系统token...');
      
      await invoke('clear_all_tokens');
      console.log('✅ 所有系统token已清除');
      
      // 刷新状态
      await refreshTokenStatuses();
      
    } catch (err) {
      error.value = `清除所有token失败: ${err}`;
      console.error('清除所有token失败:', err);
    } finally {
      isLoading.value = false;
    }
  };
  
  // 加载token事件历史
  const loadTokenEventHistory = async () => {
    try {
      console.log('📚 加载token事件历史...');
      const history = await invoke('get_token_event_history') as TokenEvent[];
      tokenEvents.value = history;
      console.log(`📖 加载了 ${history.length} 个历史事件`);
    } catch (err) {
      console.error('加载token事件历史失败:', err);
      throw err;
    }
  };
  
  // 清空错误信息
  const clearError = () => {
    error.value = '';
  };
  
  // 格式化token状态显示
  const formatTokenStatus = (status: TokenState): string => {
    if (status === 'Active') return '有效';
    if (status === 'Expired') return '已过期';
    if (status === 'Waiting') return '等待获取';
    if (typeof status === 'object' && 'Failed' in status) {
      return `失败: ${status.Failed}`;
    }
    return '未知';
  };
  
  // 格式化token事件显示
  const formatTokenEvent = (event: TokenEvent): string => {
    if ('TokenAcquired' in event) {
      const data = event.TokenAcquired;
      return `[${data.system_name}] Token获取成功`;
    }
    if ('TokenExpired' in event) {
      const data = event.TokenExpired;
      return `[${data.system_name}] Token已过期`;
    }
    if ('TokenFailed' in event) {
      const data = event.TokenFailed;
      return `[${data.system_name}] Token获取失败: ${data.error}`;
    }
    return '未知事件';
  };
  
  // 获取token剩余时间（秒）
  const getTokenRemainingTime = computed(() => (systemId: string): number | null => {
    const status = getSystemStatus.value(systemId);
    if (!status || !status.token_expires_at || status.status !== 'Active') {
      return null;
    }
    
    const now = Math.floor(Date.now() / 1000);
    const remaining = status.token_expires_at - now;
    return remaining > 0 ? remaining : 0;
  });
  
  // 格式化剩余时间显示
  const formatRemainingTime = (seconds: number): string => {
    if (seconds <= 0) return '已过期';
    
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    
    if (hours > 0) {
      return `${hours}小时${minutes}分钟`;
    } else if (minutes > 0) {
      return `${minutes}分${secs}秒`;
    } else {
      return `${secs}秒`;
    }
  };
  
  return {
    // 状态
    tokenStatuses,
    tokenEvents,
    error,
    isLoading,
    
    // 计算属性
    activeTokensCount,
    expiredTokensCount,
    waitingTokensCount,
    failedTokensCount,
    getSystemStatus,
    hasValidToken,
    getTokenRemainingTime,
    
    // 方法
    initialize,
    refreshTokenStatuses,
    getSystemToken,
    clearSystemToken,
    clearAllTokens,
    loadTokenEventHistory,
    clearError,
    formatTokenStatus,
    formatTokenEvent,
    formatRemainingTime,
  };
}); 
<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { useProxyStore } from '../../stores/proxyStore';
import type { PacketData } from '../../stores/proxyStore';
import PacketDetail from './components/PacketDetail.vue';

const route = useRoute();
const proxyStore = useProxyStore();

const packet = ref<PacketData | null>(null);
const loading = ref(true);
const error = ref('');

onMounted(async () => {
  try {
    // 从路由参数获取数据包ID
    const packetId = route.params.id as string;
    
    if (!packetId) {
      error.value = '缺少数据包ID参数';
      loading.value = false;
      return;
    }
    
    // 获取数据包
    let packetData = proxyStore.getPacketForWindow(packetId);
    
    if (packetData) {
      packet.value = packetData;
    } else {
      error.value = '未找到指定的数据包';
    }
    
  } catch (err) {
    console.error('加载数据包详情失败:', err);
    error.value = '加载数据包详情失败';
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-900/95 via-slate-800/90 to-slate-700/85">
    <!-- 加载状态 -->
    <div v-if="loading" class="flex flex-col items-center justify-center min-h-screen gap-4 px-5 text-center">
      <div class="w-10 h-10 border-3 border-blue-500/30 border-t-blue-500 rounded-full animate-spin"></div>
      <p class="text-lg text-slate-400 m-0">正在加载数据包详情...</p>
    </div>
    
    <!-- 错误状态 -->
    <div v-else-if="error" class="flex flex-col items-center justify-center min-h-screen gap-4 px-5 text-center">
      <div class="text-5xl">⚠️</div>
      <p class="text-lg text-slate-400 m-0">{{ error }}</p>
    </div>
    
    <!-- 数据包详情 -->
    <PacketDetail v-if="packet" :packet="packet" />
  </div>
</template>

<style scoped>
/* 自定义径向渐变背景 - Tailwind 不支持复杂的径向渐变 */
.bg-gradient-radial {
  background: radial-gradient(ellipse at center, rgba(59, 130, 246, 0.08), transparent 60%);
}
</style> 
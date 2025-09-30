<template>
  <div class="setting-group">
    <h3>全局快捷键</h3>
    <div class="shortcut-item" v-for="sc in shortcuts" :key="sc.id">
      <span class="shortcut-label">{{ sc.label }}</span>
      <el-input 
        v-model="sc.key" 
        readonly 
        class="shortcut-input"
        @click=""
      />
      <el-switch v-model="shortcut.timestamp" @change="changeShortcutSetting" />
    </div>
  </div>
</template>

<script setup vapor>
import { ref } from 'vue';
import { useUserStore } from '../../store';

const userStore = useUserStore()

const shortcut = ref({
  timestamp: userStore.settingInfo.shortcut.timestamp || false,
});

const shortcuts = ref([
  { id: 1, label: '查看时间', key: 'Ctrl + C' },
]);

const changeShortcutSetting = () => {
  userStore.updateSetting({ shortcut: shortcut.value });
};

</script>

<style scoped>
/* Shortcut settings styles from original file */
.setting-group { margin-bottom: 30px; }
.setting-group h3 { margin: 0 0 15px 0; font-size: 16px; color: #333; font-weight: 500; }
.shortcut-item { display: flex; align-items: center; justify-content: space-between; margin-bottom: 15px; }
.shortcut-label { font-weight: 500; color: #333; }
.shortcut-input { width: 150px; }
</style>
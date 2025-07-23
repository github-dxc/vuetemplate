<template>
  <div class="plugin-header">
    <h3>插件管理</h3>
    <el-badge value="NEW" class="new-badge" />
  </div>
  <div class="plugin-list">
    <div class="plugin-item" v-for="plugin in localPlugins" :key="plugin.id">
      <div class="plugin-info">
        <h4>{{ plugin.name }}</h4>
        <p>{{ plugin.description }}</p>
      </div>
      <el-switch v-model="plugin.enabled" />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  plugins: Array
});
const emit = defineEmits(['update:plugins']);

const localPlugins = computed({
  get: () => props.plugins,
  set: (val) => emit('update:plugins', val)
});
</script>

<style scoped>
/* Plugin settings styles from original file */
.plugin-header { display: flex; align-items: center; gap: 10px; margin-bottom: 20px; }
.plugin-header h3 { margin: 0; }
.new-badge { --el-badge-bg-color: #ff4d4f; }
.plugin-list { display: flex; flex-direction: column; gap: 15px; }
.plugin-item { display: flex; align-items: center; justify-content: space-between; padding: 15px; border: 1px solid #e8e8e8; border-radius: 8px; background: #fafafa; }
.plugin-info h4 { margin: 0 0 5px 0; color: #333; }
.plugin-info p { margin: 0; color: #666; font-size: 14px; }
</style>
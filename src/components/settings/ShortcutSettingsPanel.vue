<template>
  <div class="setting-group">
    <h3>全局快捷键</h3>
    <div class="shortcut-item" v-for="shortcut in shortcuts" :key="shortcut.id">
      <span class="shortcut-label">{{ shortcut.label }}</span>
      <el-input 
        v-model="shortcut.key" 
        readonly 
        class="shortcut-input"
        @click="$emit('edit-shortcut', shortcut)"
      />
    </div>
  </div>
</template>

<script setup>
defineProps({
  shortcuts: Array
});
defineEmits(['edit-shortcut']);
</script>

<style scoped>
/* Shortcut settings styles from original file */
.setting-group { margin-bottom: 30px; }
.setting-group h3 { margin: 0 0 15px 0; font-size: 16px; color: #333; font-weight: 500; }
.shortcut-item { display: flex; align-items: center; justify-content: space-between; margin-bottom: 15px; }
.shortcut-label { font-weight: 500; color: #333; }
.shortcut-input { width: 150px; }
</style>
<template>
  <div class="setting-group">
    <h3>启动设置</h3>
    <el-checkbox v-model="localSettings.autoStart">开机自动启动微信</el-checkbox>
    <el-checkbox v-model="localSettings.minimizeToTray">启动时最小化到系统托盘</el-checkbox>
  </div>
  <div class="setting-group">
    <h3>消息设置</h3>
    <el-checkbox v-model="localSettings.soundNotification">声音提醒</el-checkbox>
    <el-checkbox v-model="localSettings.desktopNotification">桌面通知</el-checkbox>
    <el-checkbox v-model="localSettings.showMessagePreview">显示消息预览</el-checkbox>
  </div>
  <div class="setting-group">
    <h3>界面设置</h3>
    <div class="setting-item">
      <span>界面缩放</span>
      <el-select v-model="localSettings.uiScale" style="width: 120px;">
        <el-option label="75%" value="0.75" />
        <el-option label="100%" value="1" />
        <el-option label="125%" value="1.25" />
        <el-option label="150%" value="1.5" />
      </el-select>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  settings: Object
});
const emit = defineEmits(['update:settings']);

const localSettings = computed({
  get: () => props.settings,
  set: (val) => emit('update:settings', val)
});
</script>

<style scoped>
/* General settings styles from original file */
.setting-group { margin-bottom: 30px; }
.setting-group h3 { margin: 0 0 15px 0; font-size: 16px; color: #333; font-weight: 500; }
.setting-group .el-checkbox { display: block; margin-bottom: 12px; }
.setting-item { display: flex; align-items: center; justify-content: space-between; margin-bottom: 15px; }
</style>
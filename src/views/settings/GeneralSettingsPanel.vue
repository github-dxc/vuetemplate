<template>
  <div class="setting-group">
    <div class="setting-item">
      <h3>启动设置</h3>
      <el-form-item label="开机自动启动">
        <el-switch v-model="startSettings.autoStart" @change="changeStartSetting" />
      </el-form-item>
    </div>
    <div class="setting-item">
      <h3>更新设置</h3>
      <el-form-item label="自动更新">
        <el-switch v-model="updateSettings.autoUpdate" @change="changeUpdateSetting" />
      </el-form-item>
    </div>
  </div>
  <!-- <div class="setting-group">
    <h3>消息设置</h3>
    <el-checkbox v-model="localSettings.soundNotification">声音提醒</el-checkbox>
    <el-checkbox v-model="localSettings.desktopNotification">桌面通知</el-checkbox>
    <el-checkbox v-model="localSettings.showMessagePreview">显示消息预览</el-checkbox>
  </div> -->
  <!-- <div class="setting-group">
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
  </div> -->
</template>

<script setup vapor>
import { ref } from 'vue';
import { useUserStore } from '../../store';
import { autoStart } from '../../windows';

const userStore = useUserStore()

const updateSettings = ref({
  autoUpdate: userStore.settingInfo.update.autoUpdate || false,
});

const startSettings = ref({
  autoStart: userStore.settingInfo.start.autoStart || false,
});

const changeUpdateSetting = () => {
  userStore.updateSetting({ update: updateSettings.value })
};

const changeStartSetting = async () => {
  userStore.updateSetting({ start: startSettings.value })
  autoStart(startSettings.value.autoStart);
};

</script>

<style scoped>
.setting-group {
  margin-bottom: 30px;
}

.setting-group h3 {
  margin: 0 0 15px 0;
  font-size: 16px;
  color: #333;
  font-weight: 500;
}

</style>
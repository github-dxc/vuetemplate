<template>
  <div class="setting-group">
    <h3>文件存储</h3>
    <div class="setting-item">
      <span>文件保存位置</span>
      <div class="file-path">
        <el-input :model-value="settings.filePath" readonly />
        <el-button @click="">更改</el-button>
      </div>
    </div>
    <div class="setting-item">
      <span>自动下载文件大小限制</span>
      <el-select v-model="settings.autoDownloadLimit" style="width: 150px;">
        <el-option label="不自动下载" value="0" />
        <el-option label="小于10MB" value="10" />
        <el-option label="小于50MB" value="50" />
        <el-option label="小于100MB" value="100" />
      </el-select>
    </div>
  </div>
  <div class="setting-group">
    <h3>清理缓存</h3>
    <div class="cache-info">
      <p>当前缓存大小：156MB</p>
      <el-button type="primary" @click="">清理缓存</el-button>
    </div>
  </div>
</template>

<script setup vapor>
import { ref } from 'vue';

const settings = ref({filePath: 'C:\\Users\\Username\\Documents\\WeChat Files', autoDownloadLimit: '10'});
</script>

<style scoped>
/* File settings styles from original file */
.setting-group { margin-bottom: 30px; }
.setting-group h3 { margin: 0 0 15px 0; font-size: 16px; color: #333; font-weight: 500; }
.setting-item { display: flex; align-items: center; justify-content: space-between; margin-bottom: 15px; }
.file-path { display: flex; gap: 10px; align-items: center; }
.file-path .el-input { width: 300px; }
.cache-info { display: flex; align-items: center; gap: 20px; }
</style>
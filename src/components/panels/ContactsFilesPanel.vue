<template>
  <div class="left-panel">
    <div class="panel-header">
      <h3>{{ currentMenu.title }}</h3>
      <el-input 
        v-model="localSearchText" 
        placeholder="搜索..." 
        size="small"
        style="width: 200px;"
      >
        <template #prefix>
          <el-icon><Search /></el-icon>
        </template>
      </el-input>
    </div>
    <div class="panel-content">
      <div v-if="currentMenu.id === 'contacts'" class="contact-list">
        <div 
          v-for="contact in filteredContacts" 
          :key="contact.id"
          class="contact-item"
          :class="{ active: selectedContact?.id === contact.id }"
          @click="$emit('select-contact', contact)"
        >
          <el-avatar :size="40" :src="contact.avatar" />
          <div class="contact-info">
            <div class="contact-name">{{ contact.name }}</div>
            <div class="contact-status">{{ contact.status }}</div>
          </div>
          <div class="contact-time">{{ contact.lastSeen }}</div>
        </div>
      </div>
      
      <div v-else-if="currentMenu.id === 'files'" class="file-list">
        <div 
          v-for="file in files" 
          :key="file.id"
          class="file-item"
          @click="$emit('select-file', file)"
        >
          <el-icon :size="32" class="file-icon">
            <Document />
          </el-icon>
          <div class="file-info">
            <div class="file-name">{{ file.name }}</div>
            <div class="file-meta">{{ file.size }} • {{ file.date }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <div class="right-panel">
    <div class="panel-content">
      <div v-if="currentMenu.id === 'contacts' && selectedContact" class="contact-detail">
        <div class="detail-header">
          <el-avatar :size="80" :src="selectedContact.avatar" />
          <div class="detail-info">
            <h2>{{ selectedContact.name }}</h2>
            <p>{{ selectedContact.status }}</p>
          </div>
        </div>
        <div class="detail-content">
          <el-descriptions title="联系人信息" :column="1">
            <el-descriptions-item label="微信号">{{ selectedContact.wechatId }}</el-descriptions-item>
            <el-descriptions-item label="地区">{{ selectedContact.region }}</el-descriptions-item>
            <el-descriptions-item label="最后在线">{{ selectedContact.lastSeen }}</el-descriptions-item>
          </el-descriptions>
        </div>
      </div>
      
      <div v-else-if="currentMenu.id === 'files' && selectedFile" class="file-detail">
        <div class="detail-header">
          <el-icon :size="60">
            <Document />
          </el-icon>
          <div class="detail-info">
            <h2>{{ selectedFile.name }}</h2>
            <p>{{ selectedFile.size }} • {{ selectedFile.date }}</p>
          </div>
        </div>
        <div class="detail-actions">
          <el-button type="primary">打开文件</el-button>
          <el-button>下载</el-button>
          <el-button>分享</el-button>
        </div>
      </div>
      
      <div v-else class="empty-detail">
        <el-empty description="请选择一个项目查看详情" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { Search, Document } from '@element-plus/icons-vue';

const props = defineProps({
  currentMenu: Object,
  contacts: Array,
  files: Array,
  selectedContact: Object,
  selectedFile: Object
});

defineEmits(['select-contact', 'select-file']);

const localSearchText = ref('');

const filteredContacts = computed(() => {
  if (!localSearchText.value) return props.contacts;
  return props.contacts.filter(contact => 
    contact.name.toLowerCase().includes(localSearchText.value.toLowerCase())
  );
});
</script>

<style scoped>
/* Double-panel styles from original file */
.left-panel { width: 300px; background: white; border-right: 1px solid #e8e8e8; display: flex; flex-direction: column; }
.right-panel { flex: 1; background: white; display: flex; flex-direction: column; }
.panel-header { padding: 16px 20px; border-bottom: 1px solid #e8e8e8; display: flex; align-items: center; justify-content: space-between; }
.panel-header h3 { margin: 0; font-size: 16px; color: #333; }
.panel-content { flex: 1; padding: 20px; overflow-y: auto; }
.contact-list, .file-list { padding: 0; }
.contact-item, .file-item { display: flex; align-items: center; padding: 12px 16px; cursor: pointer; border-bottom: 1px solid #f0f0f0; transition: background 0.3s; }
.contact-item:hover, .file-item:hover { background: #f8f8f8; }
.contact-item.active { background: #e6f7ff; }
.contact-info { flex: 1; margin-left: 12px; }
.contact-name, .file-name { font-weight: 500; color: #333; margin-bottom: 4px; }
.contact-status, .file-meta { color: #999; font-size: 12px; }
.contact-time { color: #999; font-size: 12px; }
.file-icon { color: #1890ff; margin-right: 12px; }
.contact-detail, .file-detail { padding: 20px; }
.detail-header { display: flex; align-items: center; gap: 16px; margin-bottom: 24px; padding-bottom: 16px; border-bottom: 1px solid #e8e8e8; }
.detail-info h2 { margin: 0 0 8px 0; color: #333; }
.detail-info p { margin: 0; color: #666; }
.detail-actions { margin-top: 20px; display: flex; gap: 12px; }
.empty-detail { display: flex; align-items: center; justify-content: center; height: 100%; }
</style>
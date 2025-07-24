<template>
  <div class="app-container">
    <Sidebar 
      :menu-list="menuList"
      :active-menu="activeMenu"
      :user-avatar="userAvatar"
      @switch-menu="handleSwitchMenu"
      @open-settings="openSettings"
    />
    
    <ContentArea 
      :current-menu="currentMenu"
      :messages="messages"
      :user-avatar="userAvatar"
      :contacts="contacts"
      :files="files"
      :selected-contact="selectedContact"
      :selected-file="selectedFile"
      @send-message="sendMessage"
      @select-contact="selectContact"
      @select-file="selectFile"
    />

    <SettingsDialog
      v-model:visible="settingsVisible"
      :user-avatar="userAvatar"
      :active-setting-menu="activeSettingMenu"
      :settings="settings"
      :shortcuts="shortcuts"
      :plugins="plugins"
      @update:active-setting-menu="activeSettingMenu = $event"
      @update:settings="settings = $event"
      @update:plugins="plugins = $event"
      @close="closeSettings"
      @logout="logout"
      @choose-file-path="chooseFilePath"
      @clear-cache="clearCache"
      @edit-shortcut="editShortcut"
    />
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { 
  ChatDotRound, User, Document, Star, View
} from '@element-plus/icons-vue';
import Sidebar from '../components/Sidebar.vue';
import ContentArea from '../components/ContentArea.vue';
import SettingsDialog from '../components/SettingsDialog.vue';

// All state from the original file is moved here to be the single source of truth.
const userAvatar = ref('https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png');

const activeMenu = ref('chat');
const menuList = ref([  
  { id: 'subscribe', title: '订阅', icon: View, layout: 'single' },
  { id: 'chat', title: '聊天', icon: ChatDotRound, layout: 'single', badge: 3 },
  { id: 'contacts', title: '通讯录', icon: User, layout: 'double' },
  { id: 'files', title: '文件传输', icon: Document, layout: 'double' },
  { id: 'favorites', title: '收藏', icon: Star, layout: 'single' }
]);

const messages = ref([
  { id: 1, sender: '小明', avatar: 'https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png', content: '你好，最近怎么样？', time: '14:30' },
  { id: 2, sender: '小红', avatar: 'https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png', content: '还不错，工作挺忙的', time: '14:32' }
]);
const inputMessage = ref('');

const contacts = ref([
  { id: 1, name: '文件传输助手', avatar: 'https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png', status: '在线', lastSeen: '刚刚', wechatId: 'filehelper', region: '北京' },
  { id: 2, name: '哈哈姐', avatar: 'https://shadow.elemecdn.com/app/element/hamburger.9cf7b091-55e9-11e9-a976-7f4d0b07eef6.png', status: '离线', lastSeen: '2小时前', wechatId: 'hahajie', region: '上海' }
]);
const selectedContact = ref(null);

const files = ref([
  { id: 1, name: '项目文档.docx', size: '2.3MB', date: '2024-01-15' },
  { id: 2, name: '会议记录.pdf', size: '1.8MB', date: '2024-01-14' }
]);
const selectedFile = ref(null);

const settingsVisible = ref(false);
const activeSettingMenu = ref('account');
const settings = ref({
  autoStart: true,
  minimizeToTray: false,
  soundNotification: true,
  desktopNotification: true,
  showMessagePreview: false,
  uiScale: '1',
  filePath: 'C:\\Users\\Username\\Documents\\WeChat Files',
  autoDownloadLimit: '10'
});
const shortcuts = ref([
  { id: 1, label: '截取屏幕', key: 'Alt + A' },
  { id: 2, label: '搜索', key: 'Ctrl + F' },
  { id: 3, label: '切换会话', key: 'Ctrl + Tab' },
  { id: 4, label: '快速回复', key: 'Ctrl + Enter' }
]);
const plugins = ref([
  { id: 1, name: '消息助手', description: '自动回复和消息管理工具', enabled: true },
  { id: 2, name: '文件整理', description: '自动整理接收的文件', enabled: false }
]);

const currentMenu = computed(() => {
  return menuList.value.find(menu => menu.id === activeMenu.value);
});

// Methods
const handleSwitchMenu = (menu) => {
  activeMenu.value = menu.id;
  selectedContact.value = null;
  selectedFile.value = null;
};
const openSettings = () => {
  settingsVisible.value = true;
};
const closeSettings = () => {
  settingsVisible.value = false;
};
const sendMessage = (message) => {
  if (message.trim()) {
    messages.value.push({
      id: Date.now(),
      sender: '我',
      avatar: userAvatar.value,
      content: message,
      time: new Date().toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' })
    });
  }
};
const selectContact = (contact) => {
  selectedContact.value = contact;
};
const selectFile = (file) => {
  selectedFile.value = file;
};
const logout = () => {
  console.log('退出登录');
};
const chooseFilePath = () => {
  console.log('选择文件路径');
};
const clearCache = () => {
  console.log('清理缓存');
};
const editShortcut = (shortcut) => {
  console.log('编辑快捷键:', shortcut);
};
</script>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  background: #f5f5f5;
}
</style>
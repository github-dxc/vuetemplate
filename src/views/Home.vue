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
      :user-avatar="userAvatar"
    />

    <SettingsDialog
      v-model:visible="settingsVisible"
      :user-avatar="userAvatar"
      :active-setting-menu="activeSettingMenu"
      :settings="settings"
      :shortcuts="shortcuts"
      @update:active-setting-menu="activeSettingMenu = $event"
      @update:settings="settings = $event"
      @close="closeSettings"
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
import Subscribe from '../components/panels/Subscribe.vue';
import ChatPanel from '../components/panels/ChatPanel.vue';
import ContactsFilesPanel from '../components/panels/ContactsFilesPanel.vue';


// All state from the original file is moved here to be the single source of truth.
const userAvatar = ref('https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png');

const activeMenu = ref('subscribe');
const menuList = ref([  
  { id: 'subscribe', title: '订阅', icon: View, component: Subscribe },
  { id: 'chat', title: '聊天', icon: ChatDotRound, badge: 3, component: ChatPanel },
  { id: 'contacts', title: '通讯录', icon: User },
  { id: 'files', title: '文件传输', icon: Document, component: ContactsFilesPanel },
  { id: 'favorites', title: '收藏', icon: Star }
]);

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
const currentMenu = computed(() => {
  return menuList.value.find(menu => menu.id === activeMenu.value);
});

// Methods
const handleSwitchMenu = (menu) => {
  activeMenu.value = menu.id;
};
const openSettings = () => {
  settingsVisible.value = true;
};
const closeSettings = () => {
  settingsVisible.value = false;
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
  width: 100%;
  background: #e9e7e6;
}
</style>
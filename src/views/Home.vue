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
      @close="closeSettings"
    />
    <Update />
  </div>
</template>

<script setup>
import { ref, computed, markRaw } from 'vue';
import { 
  ChatDotRound, User, Document, Star, Bell
} from '@element-plus/icons-vue';

import Sidebar from '../components/Sidebar.vue';
import ContentArea from '../components/ContentArea.vue';
import SettingsDialog from '../components/SettingsDialog.vue';
import SubscribePanel from '../components/panels/SubscribePanel.vue';
import ChatPanel from '../components/panels/ChatPanel.vue';
import Update from '../components/Update.vue';

// All state from the original file is moved here to be the single source of truth.
const userAvatar = ref('https://cube.elemecdn.com/0/88/03b0d39583f48206768a7534e55bcpng.png');

const activeMenu = ref('subscribe');

const menuList = ref([  
  { id: 'subscribe', title: '订阅', icon: markRaw(Bell), component: markRaw(SubscribePanel) },
  { id: 'chat', title: '聊天', icon: markRaw(ChatDotRound), badge: 3, component: markRaw(ChatPanel) },
  { id: 'contacts', title: '通讯录', icon: markRaw(User) },
  { id: 'files', title: '文件传输', icon: markRaw(Document) },
  { id: 'favorites', title: '收藏', icon: markRaw(Star) }
]);

const settingsVisible = ref(false);

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
</script>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  width: 100%;
  background: #e9e7e6;
}
</style>
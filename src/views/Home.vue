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
      :bug-list="bugList"
      :bug-msgs="bugMsgs"
      :bug-total="bugTotal"
      :enums="enums"
    />

    <SettingsDialog
      v-model:visible="settingsVisible"
      @close="closeSettings"
    />
    <Update />
  </div>
</template>

<script setup>
import { ref, computed, markRaw, onMounted } from 'vue';
import { 
  ChatDotRound, User, Document, Star, Bell
} from '@element-plus/icons-vue';
import Sidebar from '../components/Sidebar.vue';
import ContentArea from '../components/ContentArea.vue';
import SettingsDialog from '../components/SettingsDialog.vue';
import SubscribePanel from '../components/panels/SubscribePanel.vue';
import ChatPanel from '../components/panels/ChatPanel.vue';
import Update from '../components/Update.vue';
import { useRouter } from 'vue-router';
import { listen, emit } from '@tauri-apps/api/event';
import { initBugs, initData, initMsgs } from '../api';

const router = useRouter()

// 变量
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
const bugList = ref([]);
const bugMsgs = ref([]);
const bugTotal = ref(0);
const enums = ref({});

// 计算属性
const currentMenu = computed(() => {
  return menuList.value.find(menu => menu.id === activeMenu.value);
});

// 方法
const handleSwitchMenu = (menu) => {
  activeMenu.value = menu.id;
};
const openSettings = () => {
  settingsVisible.value = true;
};
const closeSettings = () => {
  settingsVisible.value = false;
}; 
const api_bug_list = async () => {
  try {
    // 获取bug列表
    let data = await initBugs();
    console.log("api_init_bugs:", data);
    if (data) {
      bugList.value = data || [];
      bugTotal.value = bugList.value.length;
    }
  } catch (error) {
    console.log(error);
    router.push("/login");
  }
}
const api_msg_list = async () => {
  try {
    // 获取msg列表
    let data = await initMsgs();
    console.log("api_init_msgs:", data);
    if (data) {
      bugMsgs.value = data || [];
    }
  } catch (error) {
    console.log(error);
  }
}
const api_init_data = async () => {
  try {
    let data = await initData();
    console.log("api_init_data:", data);
    let enumsData = JSON.parse(data)
    if (enumsData) {
      enums.value = enumsData;
    }
  } catch (error) {
    console.log(error);
  }
}


// ------------------监听数据------------------

// 监听rust发送的消息
listen('sub_bugs', (event) => {
  console.log('sub_bugs:', event.payload)
  try {
    const obj = event.payload;
    if (obj) {
      bugList.value = obj;
      bugTotal.value = bugList.value.length;
    };
  } catch (error) {
    console.error('解析 JSON 失败:', error);
    return;
  }
});
listen('sub_msgs', (event) => {
  console.log('sub_msgs:', event.payload)
  try {
    const obj = event.payload;
    if (obj) {
      bugMsgs.value = bugMsgs.value.push(...obj);
    };
  } catch (error) {
    console.error('解析 JSON 失败:', error);
    return;
  }
});


// ------------------初始化------------------

onMounted(async () => {
  // 初始化枚举数据
  await api_init_data();
  // 查询bug列表
  await api_bug_list();
  // 初始化消息数据
  await api_msg_list();
});
</script>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  width: 100%;
  background: #e9e7e6;
}
</style>
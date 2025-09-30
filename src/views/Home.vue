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
      :group-msgs="groupMsgs"
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

<script setup vapor>
import { ref, computed, markRaw, onMounted, watch } from 'vue';
import { 
  ChatDotRound, CirclePlus, Document, Star, Bell
} from '@element-plus/icons-vue';
import Sidebar from './Sidebar.vue';
import ContentArea from './ContentArea.vue';
import SettingsDialog from './SettingsDialog.vue';
import SubscribePanel from './panels/SubscribePanel.vue';
import ChatPanel from './panels/ChatPanel.vue';
import Update from '../components/Update.vue';
import ListPanel from './panels/ListPanel.vue';
import AddBug from './panels/AddBug.vue';
import ToolPanel from './panels/ToolPanel.vue';
import { useRouter } from 'vue-router';
import { useUserStore } from "../store";
import { listen, emit } from '@tauri-apps/api/event';
import { initBugs, initData, initMsgs, setWindowMessageNotify } from '../api';
import { formatDate } from '../util';
import { readClipboard, registerShortcut, unregisterShortcut } from '../windows';

const router = useRouter()
const userStore = useUserStore();

// ------------------变量------------------
const userAvatar = computed(() => userStore.userInfo.avatar);
const activeMenu = ref('subscribe');
const menuList = ref([
  { id: 'subscribe', title: '订阅', icon: markRaw(ChatDotRound), component: markRaw(SubscribePanel) },
  { id: 'chat', title: '消息', icon: markRaw(Bell), badge: 0, component: markRaw(ChatPanel) },
  { id: 'files', title: '列表', icon: markRaw(Document), component: markRaw(ListPanel) },
  { id: 'contacts', title: '新增', icon: markRaw(CirclePlus), component: markRaw(AddBug) },
  { id: 'favorites', title: '收藏', icon: markRaw(Star), component: markRaw(ToolPanel) }
]);
const settingsVisible = ref(false);
const bugList = ref([]);
const bugMsgs = ref([]);
const bugTotal = ref(0);
const enums = ref({});

// ------------------计算属性------------------
const shortcut_timestamp = computed(() => userStore.settingInfo.shortcut.timestamp);

const currentMenu = computed(() => {
  return menuList.value.find(menu => menu.id === activeMenu.value);
});

const readMsg = computed(() => userStore.userInfo.read_msg);

const groupMsgs = computed(() => {
  let msgs = [];
  let unreadCount = 0;
  let is_last_msg = readMsg.value === '';
  for (let i = 0; i < bugMsgs.value.length; i++) {
    const e = bugMsgs.value[i];
    let item = msgs.find(h => h.user_id === e.handler_id && h.timestamp === e.updated_at && h.bug_id === e.bug_id)
    if (item) {
      item.operations.push(`${e.field} ${e.change}`);
    }else {
      let msg = {
        bug_id: e.bug_id,
        user_id: e.handler_id,
        username: e.handler,
        timestamp: e.updated_at,
        timestr: formatDate(e.updated_at),
        operations: [`${e.field} ${e.change}`]
      };
      if (is_last_msg) {
        msg.is_new = true;
        unreadCount++;
      }
      if (readMsg.value === `${e.updated_at}-${e.bug_id}-${e.handler_id}`) {
        is_last_msg = true;
        msg.is_last_msg = true;
      };
      msgs.push(msg);
    }
  }
  menuList.value[1].badge = unreadCount;
  if (!unreadCount) {
    // 清除消息通知
    NewMessageNotify(0);
  }
  return msgs;
});

// ------------------方法------------------
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

const NewMessageNotify = async (count) => {
  try {
    await setWindowMessageNotify(count);
  } catch (error) {
    console.error("setWindowNessageNotify error:", error);
  }
};

const registerShortcutTimestamp = async () => {
  if (shortcut_timestamp.value) {
    // 注册快捷键
    // TODO 重新使用rust的rdev库实现
    await registerShortcut('CommandOrControl+C', async () => {
      const text = await readClipboard();
      console.log("Clipboard text:", text);
    });
  } else {
    // 注销快捷键
    await unregisterShortcut('CommandOrControl+C');
  }
};

// ------------------订阅数据------------------

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
  // 新消息通知
  NewMessageNotify(event.payload.length);
  try {
    const obj = event.payload;
    if (obj) {
      bugMsgs.value.push(...obj);
    };
  } catch (error) {
    console.error('解析 JSON 失败:', error);
    return;
  }
});

// ------------------监听数据------------------
// 监听快捷键配置变化
watch(shortcut_timestamp, (newVal) => {
  console.log("shortcut_timestamp changed:", newVal);
  registerShortcutTimestamp();
});

// ------------------初始化------------------

onMounted(async () => {
  userStore.getUserInfo();
  console.log("userStore.userInfo:",userStore.userInfo);
  userStore.changeGetHost("");
  console.log("userStore.host:",userStore.serverHost);
  // 初始化枚举数据
  await api_init_data();
  // 查询bug列表
  await api_bug_list();
  // 初始化消息数据
  await api_msg_list();
  // 注册快捷键
  registerShortcutTimestamp();
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
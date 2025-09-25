<template>
  <div class="sidebar">
    <div class="avatar-section">
      <el-avatar shape="square" :size="50" :src="userAvatar" />
    </div>

    <div class="menu-section">
      <div v-for="menu in menuList" :key="menu.id" class="menu-item" :class="{ active: activeMenu === menu.id }"
        @click="$emit('switch-menu', menu)">
        <el-icon :size="24">
          <component :is="menu.icon" />
        </el-icon>
        <div class="menu-badge" v-if="menu.badge">{{ menu.badge }}</div>
      </div>
    </div>

    <div class="bottom-section">
      <div class="menu-item" @click="$emit('open-settings')">
        <el-icon :size="24">
          <Setting />
        </el-icon>
      </div>
    </div>
  </div>
</template>

<script setup vapor>
import { Setting } from '@element-plus/icons-vue';

defineProps({
  menuList: {
    type: Array,
    required: true
  },
  activeMenu: {
    type: String,
    required: true
  },
  userAvatar: {
    type: String,
    required: true
  }
});

defineEmits(['switch-menu', 'open-settings']);
</script>

<style scoped>
/* Sidebar styles from original file */
.sidebar {
  width: 64px;
  background: #2e2e2e;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px 0;
  position: relative;
  flex-shrink: 0;
}

.avatar-section {
  margin-bottom: 20px;
}

.menu-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.menu-item {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  border-radius: 4px;
  color: #b2b2b2;
  position: relative;
  transition: all 0.3s;
}

.menu-item:hover {
  background: #404040;
  color: #fff;
}

.menu-item.active {
  background: #07c160;
  color: #fff;
}

.menu-badge {
  position: absolute;
  top: -5px;
  right: -5px;
  background: #ff4d4f;
  color: white;
  border-radius: 50%;
  width: 16px;
  height: 16px;
  font-size: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.bottom-section {
  margin-top: auto;
}

:deep(.el-avatar) {
  background-color: #ffffff;
}
</style>
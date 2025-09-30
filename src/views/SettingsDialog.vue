<template>
  <el-dialog 
    :model-value="visible" 
    title="" 
    width="800px"
    :show-close="false"
    class="settings-dialog"
    append-to-body
    @close="$emit('close')"
  >
    <div class="settings-container">
      <div class="settings-sidebar">
        <div 
          v-for="setting in settingsMenu" 
          :key="setting.id"
          class="settings-menu-item"
          :class="{ active: activeSettingMenu === setting.id }"
          @click="activeSettingMenu = setting.id"
        >
          <el-icon class="menu-icon">
            <component :is="setting.icon" />
          </el-icon>
          <span class="menu-text">{{ setting.title }}</span>
          <el-badge 
            v-if="setting.badge" 
            :value="setting.badge" 
            class="menu-badge"
          />
        </div>
      </div>

      <div class="settings-content">
        <div class="settings-header">
          <div class="header-actions">
            <el-button text @click="$emit('close')">
              <el-icon><Close /></el-icon>
            </el-button>
          </div>
        </div>

        <div v-if="activeSettingMenu === 'account'" class="setting-panel">
          <AccountSettingsPanel />
        </div>
        <div v-else-if="activeSettingMenu === 'general'" class="setting-panel">
          <GeneralSettingsPanel />
        </div>
        <div v-else-if="activeSettingMenu === 'files'" class="setting-panel">
          <FileSettingsPanel />
        </div>
        <div v-else-if="activeSettingMenu === 'shortcuts'" class="setting-panel">
          <ShortcutSettingsPanel />
        </div>
        <div v-else-if="activeSettingMenu === 'about'" class="setting-panel">
          <AboutPanel />
        </div>
      </div>
    </div>
  </el-dialog>
</template>

<script setup vapor>
import { ref, markRaw } from 'vue';
import { 
  UserFilled, Tools, FolderOpened, School, InfoFilled, Close, Minus 
} from '@element-plus/icons-vue';
import AccountSettingsPanel from './settings/AccountSettingsPanel.vue';
import GeneralSettingsPanel from './settings/GeneralSettingsPanel.vue';
import FileSettingsPanel from './settings/FileSettingsPanel.vue';
import ShortcutSettingsPanel from './settings/ShortcutSettingsPanel.vue';
import AboutPanel from './settings/AboutPanel.vue';

defineProps({
  visible: Boolean,
});

defineEmits([
  'close',
]);

const activeSettingMenu = ref('account');

const settingsMenu = ref([
  { id: 'account', title: '账号设置', icon: markRaw(UserFilled) },
  { id: 'general', title: '通用设置', icon: markRaw(Tools) },
  // { id: 'files', title: '文件管理', icon: markRaw(FolderOpened) },
  { id: 'shortcuts', title: '快捷键', icon: markRaw(School) },
  { id: 'about', title: '关于我们', icon: markRaw(InfoFilled) }
]);

</script>

<style scoped>
/* Settings dialog styles from original file */
.settings-dialog { --el-dialog-padding-primary: 0; }
.settings-dialog .el-dialog__body { padding: 0; }
.settings-container { display: flex; height: 500px; background: #f5f5f5; }
.settings-sidebar { width: 200px; background: white; border-right: 1px solid #e8e8e8; padding: 20px 0; }
.settings-menu-item { display: flex; align-items: center; padding: 12px 20px; cursor: pointer; transition: background 0.3s; position: relative; }
.settings-menu-item:hover { background: #f8f8f8; }
.settings-menu-item.active { background: #e6f7ff; border-right: 2px solid #1890ff; }
.menu-icon { margin-right: 12px; color: #666; }
.settings-menu-item.active .menu-icon { color: #1890ff; }
.menu-text { flex: 1; color: #333; }
.settings-menu-item.active .menu-text { color: #1890ff; font-weight: 500; }
.settings-content { flex: 1; background: white; position: relative; }
.settings-header { position: absolute; top: 10px; right: 10px; z-index: 10; }
.header-actions { display: flex; gap: 5px; }
.header-actions .el-button { width: 30px; height: 30px; padding: 0; color: #999; }
.header-actions .el-button:hover { background: #f0f0f0; color: #333; }
.setting-panel { padding: 40px 30px 30px; height: 100%; overflow-y: auto; }
</style>
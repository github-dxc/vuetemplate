<template>
  <div class="content-area">
    <div v-if="currentMenu?.layout === 'single'" class="single-layout">
      <component 
        :is="singlePanelComponent" 
        :current-menu="currentMenu"
        :messages="messages"
        :user-avatar="userAvatar"
        @send-message="handleSendMessage"
      />
    </div>
    
    <div v-else-if="currentMenu?.layout === 'double'" class="double-layout">
      <component 
        :is="doublePanelComponent" 
        :current-menu="currentMenu"
        :contacts="contacts"
        :files="files"
        :selected-contact="selectedContact"
        :selected-file="selectedFile"
        @select-contact="$emit('select-contact', $event)"
        @select-file="$emit('select-file', $event)"
      />
    </div>

    <div v-else class="welcome-page">
      <el-empty description="欢迎使用应用" />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import ChatPanel from './panels/ChatPanel.vue';
import ContactsFilesPanel from './panels/ContactsFilesPanel.vue';

const props = defineProps({
  currentMenu: Object,
  messages: Array,
  userAvatar: String,
  contacts: Array,
  files: Array,
  selectedContact: Object,
  selectedFile: Object
});

const emit = defineEmits(['send-message', 'select-contact', 'select-file']);

const singlePanelComponent = computed(() => {
  if (props.currentMenu?.id === 'chat') {
    return ChatPanel;
  }
  return null;
});

const doublePanelComponent = computed(() => {
  if (props.currentMenu?.id === 'contacts' || props.currentMenu?.id === 'files') {
    return ContactsFilesPanel;
  }
  return null;
});

const handleSendMessage = (message) => {
  emit('send-message', message);
};
</script>

<style scoped>
/* Content area styles from original file */
.content-area { flex: 1; display: flex; flex-direction: column; }
.single-layout { height: 100%; display: flex; flex-direction: column; }
.double-layout { height: 100%; display: flex; }
.welcome-page { display: flex; align-items: center; justify-content: center; height: 100%; background: white; }
</style>
<template>
  <div class="content-panel">
    <div class="panel-header">
      <h3>聊天</h3>
    </div>
    <div class="panel-content chat-content">
      <div class="message-list">
        <div v-for="msg in messages" :key="msg.id" class="message-item">
          <el-avatar :size="32" :src="msg.avatar" />
          <div class="message-content">
            <div class="message-info">
              <span class="sender">{{ msg.sender }}</span>
              <span class="time">{{ msg.time }}</span>
            </div>
            <div class="message-text">{{ msg.content }}</div>
          </div>
        </div>
      </div>
      <div class="input-area">
        <el-input 
          v-model="localInputMessage" 
          type="textarea" 
          :rows="3" 
          placeholder="输入消息..."
          @keyup.ctrl.enter="handleSendMessage"
        />
        <el-button type="primary" @click="handleSendMessage" style="margin-top: 8px;">
          发送 (Ctrl+Enter)
        </el-button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

defineProps({
  messages: Array,
  userAvatar: String
});

const emit = defineEmits(['send-message']);

const localInputMessage = ref('');

const handleSendMessage = () => {
  emit('send-message', localInputMessage.value);
  localInputMessage.value = '';
};
</script>

<style scoped>
/* Chat panel styles from original file */
.content-panel { flex: 1; background: white; display: flex; flex-direction: column; }
.panel-header { padding: 16px 20px; border-bottom: 1px solid #e8e8e8; display: flex; align-items: center; justify-content: space-between; }
.panel-header h3 { margin: 0; font-size: 16px; color: #333; }
.panel-content { flex: 1; padding: 20px; overflow-y: auto; }
.chat-content { height: 100%; display: flex; flex-direction: column; }
.message-list { flex: 1; overflow-y: auto; margin-bottom: 20px; }
.message-item { display: flex; gap: 12px; margin-bottom: 16px; }
.message-content { flex: 1; }
.message-info { display: flex; gap: 8px; margin-bottom: 4px; }
.sender { font-weight: 500; color: #333; }
.time { color: #999; font-size: 12px; }
.message-text { background: #f0f0f0; padding: 8px 12px; border-radius: 8px; max-width: 60%; }
.input-area { border-top: 1px solid #e8e8e8; padding-top: 16px; }
</style>
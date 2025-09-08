<template>
  <div class="message-container">
    <!-- 头部工具栏 -->
    <div class="message-header">
      <div class="header-left">
        <div class="title-section">
          <h3 class="conversation-title">消息记录</h3>
        </div>
      </div>
      <div class="header-right">
        <el-button text @click="searchMessages">
          <el-icon><Search /></el-icon>
        </el-button>
        <el-button text @click="showInfo">
          <el-icon><InfoFilled /></el-icon>
        </el-button>
        <el-dropdown @command="handleMenuCommand">
          <el-button text>
            <el-icon><MoreFilled /></el-icon>
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="mute">静音通知</el-dropdown-item>
              <el-dropdown-item command="pin">置顶对话</el-dropdown-item>
              <el-dropdown-item command="archive">归档对话</el-dropdown-item>
              <el-dropdown-item command="delete" divided>删除对话</el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>

    <!-- 消息列表区域 -->
    <div class="message-list" ref="messageListRef">
      <div class="message-content" v-for="(group, date) in groupedMessages" :key="date">
        <!-- 日期分隔符 -->
        <div class="date-divider">
          <div class="date-line"></div>
          <span class="date-text">{{ formatTimelineDate(date) }}</span>
          <div class="date-line"></div>
        </div>

        <!-- 消息项 -->
        <div class="message-group">
          <div class="operation-history-list">
            <OperationCard v-for="(history, index) in group" :key="index" :username="history.username" :timestamp="history.timestr" :operations="history.operations"></OperationCard>
          </div>
        </div>
      </div>

      <!-- 滚动到底部按钮 -->
      <transition name="fade">
        <div 
          v-show="showScrollToBottom" 
          class="scroll-to-bottom"
          @click="scrollToBottom"
        >
          <el-button circle type="primary" size="small">
            <el-icon><ArrowDown /></el-icon>
          </el-button>
          <span v-if="unreadCount > 0" class="unread-badge">{{ unreadCount }}</span>
        </div>
      </transition>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  Search, InfoFilled, MoreFilled, ArrowDown,
} from '@element-plus/icons-vue'
import OperationCard from '../OperationCard.vue';
import { initMsgs } from '../../api';
import { formatDate, formatDateDay } from '../../util';

// 响应式数据
const showScrollToBottom = ref(false)
const unreadCount = ref(0)
const messageListRef = ref(null)
const historys = ref([])

// 按日期分组消息
const groupedMessages = computed(() => {
  const groups = {}
  historys.value.forEach(message => {
    let day_str = formatDateDay(message.timestamp);
    const date = new Date(day_str).getTime()/1000;
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(message)
  })
  console.log("groups:",groups);
  return groups
})

// 格式化日期
const formatTimelineDate = (timestamp) => {
  const date = new Date(timestamp*1000)
  const today = new Date()
  const yesterday = new Date(today.getTime() - 24 * 60 * 60 * 1000)
  
  if (date.toDateString() === today.toDateString()) {
    return '今天'
  } else if (date.toDateString() === yesterday.toDateString()) {
    return '昨天'
  } else {
    return date.toLocaleDateString('zh-CN', { month: 'long', day: 'numeric' })
  }
}

// 事件处理函数
const searchMessages = () => {
  console.log('搜索消息')
}

const showInfo = () => {
  console.log('显示会话信息')
}

const handleMenuCommand = (command) => {
  console.log('菜单命令:', command)
}

const scrollToBottom = () => {
  if (messageListRef.value) {
    messageListRef.value.scrollTop = messageListRef.value.scrollHeight
  }
  showScrollToBottom.value = false
  unreadCount.value = 0
}

// 获取历史记录
const getMsgs = async () => {
  let result = await initMsgs();
  console.log(result);
  let change_history = result || [];
  for (let i = 0; i < change_history.length; i++) {
    const e = change_history[i];
    let item = historys.value.find(h => h.username === e.handler && h.timestamp === e.updated_at)
    if (item) {
      item.operations.push(`${e.field} ${e.change}`);
    }else {
      historys.value.push({
        user_id: e.handler_id,
        username: e.handler,
        timestamp: e.updated_at,
        timestr: formatDate(e.updated_at),
        operations: [`${e.field} ${e.change}`]
      });
    }
  }
}

// 监听滚动事件
const handleScroll = () => {
  if (!messageListRef.value) return
  
  const { scrollTop, scrollHeight, clientHeight } = messageListRef.value
  const isAtBottom = scrollTop + clientHeight >= scrollHeight - 10
  
  showScrollToBottom.value = !isAtBottom
}

onMounted(() => {
  if (messageListRef.value) {
    messageListRef.value.addEventListener('scroll', handleScroll)
    scrollToBottom()
  }
  // 获取历史消息
  getMsgs();
})
</script>

<style scoped>
.message-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #f5f5f5;
}

/* 头部工具栏 */
.message-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #ffffff;
  border-bottom: 1px solid #e4e7ed;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.title-section {
  display: flex;
  flex-direction: column;
}

.conversation-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
  line-height: 1.2;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

/* 消息列表区域 */
.message-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px;
  position: relative;
}

.message-content {
  width: 95%;
  margin: 0 auto;
}

.date-divider {
  display: flex;
  align-items: center;
  margin: 24px 0 16px 0;
  gap: 16px;
}

.date-line {
  flex: 1;
  height: 1px;
  background-color: #dcdfe6;
}

.date-text {
  font-size: 12px;
  color: #909399;
  background: #f5f5f5;
  padding: 4px 12px;
  border-radius: 12px;
  white-space: nowrap;
}

/* 滚动到底部按钮 */
.scroll-to-bottom {
  position: absolute;
  bottom: 20px;
  right: 20px;
  z-index: 100;
}

.unread-badge {
  position: absolute;
  top: -8px;
  right: -8px;
  background-color: #f56c6c;
  color: white;
  font-size: 12px;
  padding: 2px 6px;
  border-radius: 10px;
  min-width: 16px;
  text-align: center;
}

/* 消息列表 */
.operation-history-list {
  justify-content: center;
}

.operation-history-list .operation-card {
  margin-top: 20px;
  margin-bottom: 20px;
}

.operation-history-list .other {
  margin-left: 20px;
}

.operation-history-list .self {
  margin-right: 20px;
}

/* 过渡效果 */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* 滚动条样式 */
.message-list::-webkit-scrollbar {
  width: 6px;
}

.message-list::-webkit-scrollbar-track {
  background: transparent;
}

.message-list::-webkit-scrollbar-thumb {
  background-color: #c0c4cc;
  border-radius: 3px;
}

.message-list::-webkit-scrollbar-thumb:hover {
  background-color: #909399;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .message-list {
    padding: 12px;
  }
  
  .message-bubble-section {
    max-width: 80%;
  }
  
  .conversation-title {
    font-size: 14px;
  }
  
  .participant-count {
    font-size: 11px;
  }
}
</style>
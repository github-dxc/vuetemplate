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
              <el-dropdown-item command="mute">测试1</el-dropdown-item>
              <el-dropdown-item command="pin">测试2</el-dropdown-item>
              <el-dropdown-item command="archive">测试3</el-dropdown-item>
              <el-dropdown-item command="delete" divided>测试4</el-dropdown-item>
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
          <div class="operation-history-list" v-for="(history, index) in group" :key="index" >
            <OperationCard 
              :bug_id="history.bug_id"
              :host="host"
              :username="history.username" 
              :timestr="history.timestr" 
              :timestamp="history.timestamp"
              :operations="history.operations"
              :class="getMsgClass(history)"
              @click="openBugDetails(history.timestamp,history.bug_id,history.user_id)"
            ></OperationCard>
            
            <!-- 上次阅读分隔符 -->
            <div v-if="!!history.is_last_msg" class="date-divider" id="last-read-line">
              <div class="date-line"></div>
              <span class="date-text">上次阅读</span>
              <div class="date-line"></div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 明细展示 -->
    <div v-if="bugDetailsVisible">
      <el-drawer v-model="bugDetailsVisible" :title="bugDetailsTitle" :show-close="false" direction="rtl" size="71%">
        <BugDetails :bug-id="bugId" :enums="enums" @set-title="setTitle"/>
      </el-drawer>
    </div>

    <!-- 滚动到底部按钮 -->
    <div class="scroll-to-last-read" @click="scrollToLastRead">
      <el-button circle plain type="primary" size="small">
        <el-icon><CaretTop /></el-icon>
      </el-button>
    </div>
    <transition name="fade">
      <div 
        v-show="showScrollToBottom" 
        class="scroll-to-bottom"
        @click="scrollToBottom"
      >
        <el-button circle plain type="primary" size="small">
          <el-icon><CaretBottom /></el-icon>
        </el-button>
      </div>
    </transition>
  </div>
</template>

<script setup vapor>
import { ref, computed, onMounted, onActivated } from 'vue'
import { 
  Search, InfoFilled, MoreFilled, CaretBottom, CaretTop
} from '@element-plus/icons-vue'
import OperationCard from '../../components/OperationCard.vue';
import { formatDateDay } from '../../util';
import { useUserStore } from "../../store";
import BugDetails from '../../components/BugDetails.vue';

const props = defineProps({
  enums: {
    type: Object,
    required: true,
    default: {}
  },
  groupMsgs: {
    type: Array,
    required: true,
    default: []
  }
}); 

const userStore = useUserStore();

// 响应式数据
const showScrollToBottom = ref(false)
const messageListRef = ref(null)
const bugDetailsVisible = ref(false);
const bugDetailsTitle = ref("Bug明细");
const bugId = ref(0);

const user_id = computed(() => userStore.userInfo.user_id || 0);

const host = computed(() => userStore.serverHost || '');

const last_read_msg = computed(() => userStore.userInfo.read_msg);

// 按日期分组消息
const groupedMessages = computed(() => {
  const groups = {}
  props.groupMsgs.forEach(message => {
    let day_str = formatDateDay(message.timestamp);
    const date = new Date(day_str).getTime()/1000;
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(message)
  })
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

// 获取消息的class
const getMsgClass = (history) => {
  let class_arr = [];
  class_arr.push(user_id.value === history.user_id? 'self' : 'other')
  class_arr.push(history.is_new? 'new' : 'old')
  return class_arr.join(' ');
}

// 事件处理函数
const searchMessages = () => {
  console.log('搜索消息')
}

const showInfo = () => {
  console.log('显示会话信息')
}

// 处理菜单事件
const handleMenuCommand = (command) => {
  console.log('菜单命令:', command)
}

// 滚动到底部
const scrollToBottom = () => {
  if (messageListRef.value) {
    messageListRef.value.scrollTop = messageListRef.value.scrollHeight
  }
  showScrollToBottom.value = false
}

// 滚动到上次阅读
const scrollToLastRead = () => {
  const lastReadElement = document.querySelector('#last-read-line')
  if (lastReadElement) {
    lastReadElement.scrollIntoView({
      behavior: 'smooth',
      block: 'center',
      inline: 'nearest'
    })
  } else {
    console.log('没有找到上次阅读位置')
  }
}

// 打开bug详情
const openBugDetails = async (timestamp,bug_id,user_id) => {
  bugDetailsVisible.value = true;
  bugId.value = bug_id;
  //设置已读
  let arr = last_read_msg.value.split('-');
  if (parseInt(timestamp)<parseInt(arr[0]) 
  || (parseInt(timestamp)===parseInt(arr[0])&&parseInt(bug_id)<parseInt(arr[1])) 
  || (parseInt(timestamp)===parseInt(arr[0])&&parseInt(bug_id)===parseInt(arr[1])&&parseInt(user_id)<parseInt(arr[2]))) {
    console.log("忽略设置已读");
    return;
  }
  userStore.readMsg(timestamp,bug_id,user_id);
}
const setTitle = (title) => {
  bugDetailsTitle.value = title;
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
  }
})

onActivated(() => {
  scrollToBottom()
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

.scroll-to-last-read {
  position: absolute;
  bottom: 60px;
  right: 20px;
  z-index: 100;
}

/* 滚动到底部按钮 */
.scroll-to-bottom {
  position: absolute;
  bottom: 20px;
  right: 20px;
  z-index: 100;
}

/* 消息列表 */
.operation-history-list {
  display: flex;
  flex-direction: column;
  padding: 10px;
}

.operation-history-list .operation-card {
  width: 50%;
}

.operation-history-list .other {
  align-self: flex-start;
}

.operation-history-list .self {
  align-self: flex-end;
}

.new {
  position: relative; /* 为了让伪元素绝对定位相对于这个元素 */
}

.new::after {
  content: '';
  position: absolute;
  top: 10px;
  right: 10px;
  width: 10px;
  height: 10px;
  background-color: #ff4757; /* 红色小圆点 */
  border-radius: 50%; /* 圆形 */
  z-index: 999; /* 确保在最上层 */
}

.new::after {
  animation: pulse 1.5s infinite;
}

@keyframes pulse {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.7;
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
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

:deep(.el-drawer__header) {
  margin-bottom: 0px;
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
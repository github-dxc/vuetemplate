<template>
  <div class="top-menu">
    <el-menu
      :default-active="activeIndex"
      mode="horizontal"
      @select="handleMenuSelect"
      class="menu-container"
    >
      <!-- 左侧菜单项 -->
      <div class="menu-left">
        <!-- 文件菜单 -->
        <el-sub-menu index="file">
          <template #title>
            <el-icon><Document /></el-icon>
            <span>文件</span>
          </template>
          <el-menu-item index="file-new">
            <el-icon><Plus /></el-icon>
            <span>新建</span>
          </el-menu-item>
          <el-menu-item index="file-open">
            <el-icon><FolderOpened /></el-icon>
            <span>打开</span>
          </el-menu-item>
          <el-menu-item index="file-save">
            <el-icon><Download /></el-icon>
            <span>保存</span>
          </el-menu-item>
          <el-menu-item index="file-save-as">
            <el-icon><DocumentCopy /></el-icon>
            <span>另存为</span>
          </el-menu-item>
          <el-menu-item index="file-recent" disabled>
            <el-icon><Clock /></el-icon>
            <span>最近文件</span>
          </el-menu-item>
          <el-menu-item index="file-exit">
            <el-icon><Close /></el-icon>
            <span>退出</span>
          </el-menu-item>
        </el-sub-menu>

        <!-- 编辑菜单 -->
        <el-sub-menu index="edit">
          <template #title>
            <el-icon><Edit /></el-icon>
            <span>编辑</span>
          </template>
          <el-menu-item index="edit-undo">
            <el-icon><RefreshLeft /></el-icon>
            <span>撤销</span>
          </el-menu-item>
          <el-menu-item index="edit-redo">
            <el-icon><RefreshRight /></el-icon>
            <span>重做</span>
          </el-menu-item>
          <el-menu-item index="edit-cut">
            <el-icon><Scissors /></el-icon>
            <span>剪切</span>
          </el-menu-item>
          <el-menu-item index="edit-copy">
            <el-icon><DocumentCopy /></el-icon>
            <span>复制</span>
          </el-menu-item>
          <el-menu-item index="edit-paste">
            <el-icon><CopyDocument /></el-icon>
            <span>粘贴</span>
          </el-menu-item>
          <el-menu-item index="edit-select-all">
            <el-icon><Select /></el-icon>
            <span>全选</span>
          </el-menu-item>
          <el-menu-item index="edit-find">
            <el-icon><Search /></el-icon>
            <span>查找</span>
          </el-menu-item>
          <el-menu-item index="edit-replace">
            <el-icon><Switch /></el-icon>
            <span>替换</span>
          </el-menu-item>
        </el-sub-menu>

        <!-- 帮助菜单 -->
        <el-sub-menu index="help">
          <template #title>
            <el-icon><QuestionFilled /></el-icon>
            <span>帮助</span>
          </template>
          <el-menu-item index="help-docs">
            <el-icon><Reading /></el-icon>
            <span>帮助文档</span>
          </el-menu-item>
          <el-menu-item index="help-shortcuts">
            <el-icon><Keyboard /></el-icon>
            <span>快捷键</span>
          </el-menu-item>
          <el-menu-item index="help-feedback">
            <el-icon><ChatDotSquare /></el-icon>
            <span>反馈建议</span>
          </el-menu-item>
          <el-menu-item index="help-check-update">
            <el-icon><Refresh /></el-icon>
            <span>检查更新</span>
          </el-menu-item>
          <el-menu-item index="help-about">
            <el-icon><InfoFilled /></el-icon>
            <span>关于</span>
          </el-menu-item>
        </el-sub-menu>
      </div>

      <!-- 右侧用户菜单 -->
      <div class="menu-right">
        <el-sub-menu index="user">
          <template #title>
            <div class="user-info">
              <el-avatar 
                :size="32" 
                :src="userInfo.avatar" 
                class="user-avatar"
              >
                <el-icon><User /></el-icon>
              </el-avatar>
              <span class="username">{{ userInfo.username }}</span>
              <el-icon class="dropdown-icon"><ArrowDown /></el-icon>
            </div>
          </template>
          <el-menu-item index="user-profile">
            <el-icon><User /></el-icon>
            <span>个人资料</span>
          </el-menu-item>
          <el-menu-item index="user-settings">
            <el-icon><Setting /></el-icon>
            <span>设置</span>
          </el-menu-item>
          <el-menu-item index="user-theme">
            <el-icon><Moon /></el-icon>
            <span>主题切换</span>
          </el-menu-item>
          <el-menu-item index="user-logout" class="logout-item">
            <el-icon><SwitchButton /></el-icon>
            <span>注销</span>
          </el-menu-item>
        </el-sub-menu>
      </div>
    </el-menu>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import {
  Document,
  Edit,
  QuestionFilled,
  User,
  Plus,
  FolderOpened,
  Download,
  DocumentCopy,
  Clock,
  Close,
  RefreshLeft,
  RefreshRight,
  Scissors,
  CopyDocument,
  Select,
  Search,
  Switch,
  Reading,
  Keyboard,
  ChatDotSquare,
  Refresh,
  InfoFilled,
  Setting,
  Moon,
  SwitchButton,
  ArrowDown
} from '@element-plus/icons-vue'

// 组件属性
const props = defineProps({
  // 用户信息
  userInfo: {
    type: Object,
    default: () => ({
      username: '用户名',
      avatar: '',
      email: 'user@example.com'
    })
  },
  // 当前激活的菜单项
  activeIndex: {
    type: String,
    default: ''
  }
})

// 组件事件
const emit = defineEmits([
  'menu-select',
  'user-profile',
  'user-logout',
  'file-new',
  'file-open',
  'file-save',
  'edit-undo',
  'edit-redo',
  'help-about'
])

// 响应式数据
const activeIndex = ref(props.activeIndex)

// 用户信息
const userInfo = reactive({
  username: props.userInfo.username || '用户名',
  avatar: props.userInfo.avatar || '',
  email: props.userInfo.email || 'user@example.com'
})

// 菜单选择处理
const handleMenuSelect = (index, indexPath) => {
  console.log('菜单选择:', index, indexPath)
  
  // 触发通用菜单选择事件
  emit('menu-select', index, indexPath)
  
  // 根据菜单项执行不同的操作
  switch (index) {
    // 文件菜单
    case 'file-new':
      handleFileNew()
      break
    case 'file-open':
      handleFileOpen()
      break
    case 'file-save':
      handleFileSave()
      break
    case 'file-save-as':
      handleFileSaveAs()
      break
    case 'file-exit':
      handleFileExit()
      break
    
    // 编辑菜单
    case 'edit-undo':
      handleEditUndo()
      break
    case 'edit-redo':
      handleEditRedo()
      break
    case 'edit-cut':
      handleEditCut()
      break
    case 'edit-copy':
      handleEditCopy()
      break
    case 'edit-paste':
      handleEditPaste()
      break
    case 'edit-select-all':
      handleEditSelectAll()
      break
    case 'edit-find':
      handleEditFind()
      break
    case 'edit-replace':
      handleEditReplace()
      break
    
    // 帮助菜单
    case 'help-docs':
      handleHelpDocs()
      break
    case 'help-shortcuts':
      handleHelpShortcuts()
      break
    case 'help-feedback':
      handleHelpFeedback()
      break
    case 'help-check-update':
      handleHelpCheckUpdate()
      break
    case 'help-about':
      handleHelpAbout()
      break
    
    // 用户菜单
    case 'user-profile':
      handleUserProfile()
      break
    case 'user-settings':
      handleUserSettings()
      break
    case 'user-theme':
      handleUserTheme()
      break
    case 'user-logout':
      handleUserLogout()
      break
    
    default:
      ElMessage.info(`点击了菜单项: ${index}`)
  }
}

// 文件菜单处理函数
const handleFileNew = () => {
  ElMessage.success('新建文件')
  emit('file-new')
}

const handleFileOpen = () => {
  ElMessage.success('打开文件')
  emit('file-open')
}

const handleFileSave = () => {
  ElMessage.success('保存文件')
  emit('file-save')
}

const handleFileSaveAs = () => {
  ElMessage.success('另存为')
}

const handleFileExit = async () => {
  try {
    await ElMessageBox.confirm('确定要退出应用吗？', '退出确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    ElMessage.success('退出应用')
    // 这里可以调用 Tauri 的退出 API
    // await invoke('exit_app')
  } catch {
    ElMessage.info('取消退出')
  }
}

// 编辑菜单处理函数
const handleEditUndo = () => {
  ElMessage.success('撤销操作')
  emit('edit-undo')
}

const handleEditRedo = () => {
  ElMessage.success('重做操作')
  emit('edit-redo')
}

const handleEditCut = () => {
  ElMessage.success('剪切')
}

const handleEditCopy = () => {
  ElMessage.success('复制')
}

const handleEditPaste = () => {
  ElMessage.success('粘贴')
}

const handleEditSelectAll = () => {
  ElMessage.success('全选')
}

const handleEditFind = () => {
  ElMessage.success('查找')
}

const handleEditReplace = () => {
  ElMessage.success('替换')
}

// 帮助菜单处理函数
const handleHelpDocs = () => {
  ElMessage.success('打开帮助文档')
  // 可以打开外部链接或显示帮助对话框
}

const handleHelpShortcuts = () => {
  ElMessage.success('显示快捷键')
  // 可以显示快捷键对话框
}

const handleHelpFeedback = () => {
  ElMessage.success('反馈建议')
  // 可以打开反馈表单或外部链接
}

const handleHelpCheckUpdate = () => {
  ElMessage.success('检查更新')
  // 可以调用更新检查功能
}

const handleHelpAbout = () => {
  ElMessage.success('关于应用')
  emit('help-about')
  // 可以显示关于对话框
}

// 用户菜单处理函数
const handleUserProfile = () => {
  ElMessage.success('个人资料')
  emit('user-profile')
}

const handleUserSettings = () => {
  ElMessage.success('设置')
}

const handleUserTheme = () => {
  ElMessage.success('主题切换')
  // 可以实现主题切换功能
}

const handleUserLogout = async () => {
  try {
    await ElMessageBox.confirm('确定要注销登录吗？', '注销确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning'
    })
    ElMessage.success('注销成功')
    emit('user-logout')
  } catch {
    ElMessage.info('取消注销')
  }
}

// 更新用户信息
const updateUserInfo = (newUserInfo) => {
  Object.assign(userInfo, newUserInfo)
}

// 暴露给父组件的方法
defineExpose({
  updateUserInfo
})

// 生命周期
onMounted(() => {
  console.log('顶部菜单组件已挂载')
})
</script>

<style scoped>
.top-menu {
  width: 100%;
  border-bottom: 1px solid #e4e7ed;
  background: #ffffff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.menu-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 16px;
  border-bottom: none;
}

.menu-left {
  display: flex;
  align-items: center;
  flex: 1;
}

.menu-right {
  display: flex;
  align-items: center;
  margin-left: auto;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 8px;
  cursor: pointer;
}

.user-avatar {
  border: 2px solid #e4e7ed;
  transition: border-color 0.3s;
}

.user-info:hover .user-avatar {
  border-color: #409eff;
}

.username {
  font-size: 14px;
  font-weight: 500;
  color: #303133;
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dropdown-icon {
  font-size: 12px;
  color: #909399;
  transition: transform 0.3s;
}

.user-info:hover .dropdown-icon {
  transform: rotate(180deg);
}

/* 菜单项样式 */
:deep(.el-menu-item) {
  height: 48px;
  line-height: 48px;
  padding: 0 16px;
  font-size: 14px;
  transition: all 0.3s;
}

:deep(.el-menu-item:hover) {
  background-color: #f5f7fa;
}

:deep(.el-sub-menu__title) {
  height: 48px;
  line-height: 48px;
  padding: 0 16px;
  font-size: 14px;
  font-weight: 500;
}

:deep(.el-sub-menu__title:hover) {
  background-color: #f5f7fa;
}

/* 用户菜单特殊样式 */
:deep(.el-sub-menu .el-sub-menu__title) {
  border-bottom: none;
}

.logout-item {
  color: #f56c6c !important;
}

.logout-item:hover {
  background-color: #fef0f0 !important;
}

/* 图标样式 */
:deep(.el-icon) {
  margin-right: 6px;
  font-size: 16px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .menu-container {
    padding: 0 8px;
  }
  
  .username {
    display: none;
  }
  
  :deep(.el-menu-item),
  :deep(.el-sub-menu__title) {
    padding: 0 8px;
    font-size: 13px;
  }
}

/* 深色主题支持 */
@media (prefers-color-scheme: dark) {
  .top-menu {
    background: #1f2937;
    border-bottom-color: #374151;
  }
  
  .username {
    color: #f3f4f6;
  }
  
  :deep(.el-menu-item:hover),
  :deep(.el-sub-menu__title:hover) {
    background-color: #374151;
  }
}
</style>
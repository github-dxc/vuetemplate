<template>
  <div class="layout-container">
    <!-- 左侧菜单栏 -->
    <div class="left-sidebar">
      <!-- 搜索框 -->
      <div class="search-wrapper">
        <el-input
          v-model="searchText"
          placeholder="搜索菜单..."
          :prefix-icon="Search"
          clearable
          size = "large"
        />
      </div>

      <!-- 菜单列表 -->
      <el-scrollbar class="menu-scrollbar">
        <el-menu
          :default-active="activeMenu"
          @select="handleMenuSelect"
          class="menu-list"
        >
          <el-menu-item
            v-for="item in filteredMenuList"
            :key="item.id"
            :index="item.id"
          >
            <el-icon>
              <component :is="item.icon" />
            </el-icon>
            <span>{{ item.name }}</span>
          </el-menu-item>
        </el-menu>

        <!-- 无搜索结果提示 -->
        <el-empty
          v-if="filteredMenuList.length === 0"
          description="未找到相关功能"
          :image-size="80"
        />
      </el-scrollbar>
    </div>

    <!-- 右侧内容区 -->
    <div class="right-content">
      <!-- 头部标题 -->
      <div class="content-header">
        <el-icon class="header-icon">
          <component :is="currentMenu?.icon" />
        </el-icon>
        <h2>{{ currentMenu?.name || '请选择功能' }}</h2>
      </div>

      <!-- 主要内容区域 -->
      <div class="content-main">
        <div>
          <!-- 这里放置你的菜单组件内容 -->
          <component
            v-if="currentMenu"
            :is="currentMenu.component"
          />
          
          <!-- 默认空状态 -->
          <div v-else class="empty-state">
            <el-icon :size="80" color="#909399">
              <Document />
            </el-icon>
            <p>请从左侧菜单选择一个功能开始使用</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, markRaw } from 'vue'
import { Search, Clock, Picture, VideoCamera, Document } from '@element-plus/icons-vue'
import TimeTool from '../tools/TimeTool.vue'

// 搜索文本
const searchText = ref('')

// 当前选中的菜单
const activeMenu = ref('time-converter')

// 菜单列表数据
const menuList = ref([
  {
    id: 'time-converter',
    name: '时间转换',
    icon: markRaw(Clock),
    component: markRaw(TimeTool),
  },
  {
    id: 'image-converter',
    name: '图片格式转换',
    icon: markRaw(Picture),
    component: null
  },
  {
    id: 'video-converter',
    name: '视频格式转换',
    icon: markRaw(VideoCamera),
    component: null
  }
])

// 搜索筛选后的菜单列表
const filteredMenuList = computed(() => {
  if (!searchText.value) {
    return menuList.value
  }
  return menuList.value.filter(item =>
    item.name.toLowerCase().includes(searchText.value.toLowerCase())
  )
})

// 当前菜单信息
const currentMenu = computed(() => {
  return menuList.value.find(item => item.id === activeMenu.value)
})

// 处理菜单选择
const handleMenuSelect = (index) => {
  activeMenu.value = index
}
</script>

<style scoped>
.layout-container {
  display: flex;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  overflow: hidden;
}

/* 左侧菜单栏 */
.left-sidebar {
  width: 200px;
  background: #f0f3f7;
  border-right: 1px solid #e4e7ed;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.05);
}

.search-wrapper {
  padding: 16px;
  background: #f0f3f7;
}

:deep(.search-wrapper .el-input__wrapper) {
  background: #e5e8ec;
  border-radius: 8px;
}

:deep(.search-wrapper .el-input__wrapper:hover) {
  background: #d9dce1;
}

:deep(.search-wrapper .el-input__wrapper.is-focus) {
  background: #ffffff;
  border: #4184dd;
}
:deep(.search-wrapper .el-input__wrapper:not(.is-focus)) {
  box-shadow: none;
}

.menu-scrollbar {
  flex: 1;
  height: 0;
}

.menu-list {
  border: none;
  background: #f0f3f7;
}

.menu-list .el-menu-item {
  height: 56px;
  line-height: 56px;
  margin: 4px 8px;
  border-radius: 8px;
  transition: all 0.3s;
}

.menu-list .el-menu-item:hover {
  background: #e4e7eb;
}

.menu-list .el-menu-item.is-active {
  background: #4890f9;
  color: #ffffff;
}

.menu-list .el-menu-item .el-icon {
  margin-right: 12px;
  font-size: 18px;
}

/* 右侧内容区 */
.right-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-header {
  height: 64px;
  background: #f5f7fa;
  border-bottom: 1px solid #e4e7ed;
  display: flex;
  align-items: center;
  padding: 0 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.content-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 500;
  color: #303133;
}

.header-icon {
  margin-right: 12px;
  font-size: 24px;
  color: #409eff;
}

.content-main {
  flex: 1;
  height: 0;
  background: #f5f7fa;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.08);
  overflow: hidden;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #909399;
}

.empty-state p {
  margin-top: 16px;
  font-size: 14px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .left-sidebar {
    width: 240px;
  }
  
  .content-header h2 {
    font-size: 18px;
  }
}
</style>
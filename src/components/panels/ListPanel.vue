<template>
  <div class="demo-container">
    <!-- 使用横向菜单组件 -->
    <HorizontalMenu 
      mode="vertical"
      :menu-items="menuData"
      @menu-click="handleMenuClick"
      @submenu-click="handleSubmenuClick"
    />
  </div>
</template>

<script setup>
import { ref, reactive, markRaw } from 'vue'
import HorizontalMenu from '../HorizontalMenu.vue' // 引入上面的组件
import {
  User,
  Setting,
  Document,
  Monitor,
  View,
} from '@element-plus/icons-vue'

// 菜单数据 - 作为 prop 传入
const menuData = reactive([
  {
    id: 'user',
    label: '用户管理',
    icon: markRaw(User),
    children: [
      { 
        id: 'user-list', 
        label: '用户列表',
        checked: true // 默认选中第一个
      },
      { 
        id: 'user-add', 
        label: '添加用户',
        tag: 'New', 
        tagType: 'success',
        checked: false
      },
      { 
        id: 'user-import', 
        label: '批量导入',
        checked: false
      },
      { 
        id: 'user-export', 
        label: '导出数据',
        checked: false
      }
    ]
  },
  {
    id: 'content',
    label: '内容管理',
    icon: markRaw(Document),
    children: [
      { 
        id: 'article-list', 
        label: '文章列表',
        checked: false
      },
      { 
        id: 'article-edit', 
        label: '编辑文章',
        checked: true // 默认选中
      },
      { 
        id: 'article-delete', 
        label: '删除文章',
        checked: false
      },
      { 
        id: 'article-share', 
        label: '分享文章',
        tag: 'Hot', 
        tagType: 'danger',
        checked: false
      },
      { 
        id: 'article-edit', 
        label: '编辑文章',
        checked: true // 默认选中
      },
      { 
        id: 'article-delete', 
        label: '删除文章',
        checked: false
      },
      { 
        id: 'article-share', 
        label: '分享文章',
        tag: 'Hot', 
        tagType: 'danger',
        checked: false
      },
      { 
        id: 'article-edit', 
        label: '编辑文章',
        checked: true // 默认选中
      },
      { 
        id: 'article-delete', 
        label: '删除文章',
        checked: false
      },
      { 
        id: 'article-share', 
        label: '分享文章',
        tag: 'Hot', 
        tagType: 'danger',
        checked: false
      },
      { 
        id: 'article-edit', 
        label: '编辑文章',
        checked: true // 默认选中
      },
      { 
        id: 'article-delete', 
        label: '删除文章',
        checked: false
      },
      { 
        id: 'article-share', 
        label: '分享文章',
        tag: 'Hot', 
        tagType: 'danger',
        checked: false
      }
    ]
  },
  {
    id: 'system',
    label: '系统设置',
    icon: markRaw(Setting),
    children: [
      { 
        id: 'general', 
        label: '常规设置',
        checked: false
      },
      { 
        id: 'security', 
        label: '安全设置',
        tag: '重要', 
        tagType: 'warning',
        checked: false
      }
    ]
  },
  {
    id: 'monitor',
    label: '系统监控',
    icon: markRaw(Monitor),
    children: [
      { 
        id: 'cpu', 
        label: 'CPU监控',
        checked: false
      },
      { 
        id: 'memory', 
        label: '内存监控',
        checked: false
      },
      { 
        id: 'disk', 
        label: '磁盘监控',
        checked: false
      }
    ]
  },
  {
    id: 'single',
    label: '单独菜单',
    icon: markRaw(View)
    // 没有子菜单
  }
])

// 事件处理
const handleMenuClick = (menuItem) => {
  console.log('点击了菜单:', menuItem)
}

const handleSubmenuClick = (data) => {
  console.log('点击了子菜单:', data)
}
</script>

<style scoped>
.demo-container {
  min-height: 100vh;
  background: #f5f7fa;
}

.selection-display {
  padding: 20px;
  max-width: 400px;
  margin: 0 auto;
}

.selection-display h3 {
  margin-bottom: 16px;
  color: #303133;
}

.selection-display p {
  margin: 8px 0;
  color: #606266;
}
</style>
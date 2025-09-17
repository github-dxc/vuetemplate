<template>
  <div class="horizontal-menu-container">
    <nav class="horizontal-menu">
      <template v-for="(item, index) in menuItems" :key="item.id">
        <!-- 一级菜单项 -->
        <div
          class="menu-item"
          :class="{ active: activeMenu === item.id }"
          @click="handleMenuClick(item)"
        >
          <el-button
            :type="activeMenu === item.id ? 'primary' : 'default'"
            class="menu-button"
          >
            <el-icon><component :is="item.icon" /></el-icon>
            {{ item.label }}
            <el-icon v-if="item.children" class="arrow-icon">
              <ArrowRight />
            </el-icon>
          </el-button>
        </div>

        <!-- 子菜单 - 插入到当前菜单项后面 -->
        <transition
          name="submenu-slide"
          @enter="onEnter"
          @leave="onLeave"
        >
          <div
            v-if="item.children && activeMenu === item.id"
            class="submenu-horizontal"
            :key="`submenu-${item.id}`"
          >
            <div
              v-for="child in item.children"
              :key="child.id"
              class="submenu-item"
              @click="handleSubmenuClick(item, child)"
            >
              <el-button
                type="info"
                class="submenu-button"
                :class="{ selected: child.checked }"
              >
                <el-icon v-if="child.icon">
                  <component :is="child.icon" />
                </el-icon>
                <span>{{ child.label }}</span>
                <el-tag
                  v-if="child.tag"
                  :type="child.tagType || 'primary'"
                  size="small"
                  class="submenu-tag"
                >
                  {{ child.tag }}
                </el-tag>
              </el-button>
            </div>
          </div>
        </transition>
      </template>
    </nav>
  </div>
</template>

<script setup>
import { ref, nextTick, computed } from 'vue'
import { ArrowRight } from '@element-plus/icons-vue'

// Props 定义
const props = defineProps({
  menuItems: {
    type: Array,
    required: true,
    default: () => []
  }
})

// 响应式数据
const activeMenu = ref(null)

// 事件定义
const emit = defineEmits(['menu-click', 'submenu-click'])

// 事件处理函数
const handleMenuClick = (item) => {
  if (item.children) {
    // 切换子菜单显示/隐藏
    activeMenu.value = activeMenu.value === item.id ? null : item.id
  } else {
    // 没有子菜单的项目直接选中
    activeMenu.value = null
    emit('menu-click', item)
  }
}

const handleSubmenuClick = (parentItem, childItem) => {
  // 清除该父菜单下所有子菜单的选中状态
  if (parentItem.children) {
    parentItem.children.forEach(child => {
      child.checked = false
    })
  }
  
  // 设置当前点击的子菜单为选中状态
  childItem.checked = true
  
  emit('submenu-click', {
    parent: parentItem,
    child: childItem
  })
}

// 动画钩子
const onEnter = (el) => {
  el.style.width = '0px'
  el.style.overflow = 'hidden'
  el.style.transform = 'scaleX(0)'
  el.style.transformOrigin = 'left center'
  
  nextTick(() => {
    el.style.transform = 'scaleX(1)'
    
    const buttons = el.querySelectorAll('.submenu-button')
    let totalWidth = 32 // 左右 padding
    
    buttons.forEach((button, index) => {
      totalWidth += button.offsetWidth
      if (index < buttons.length - 1) {
        totalWidth += 8 // gap 间距
      }
    })
    
    el.style.width = (totalWidth + 10) + 'px'
  })
}

const onLeave = (el) => {
  el.classList.add('submenu-leaving')
  
  const buttons = el.querySelectorAll('.submenu-button')
  buttons.forEach((button, index) => {
    setTimeout(() => {
      button.style.transform = 'translateX(20px)'
      button.style.opacity = '0'
    }, index * 80)
  })
  
  setTimeout(() => {
    el.style.width = '0px'
    el.style.transform = 'scaleX(0)'
  }, buttons.length * 80 + 100)
}
</script>

<style scoped>
.horizontal-menu-container {
  width: 100%;
  background: #f5f7fa;
  min-height: 100vh;
  padding: 20px;
}

.horizontal-menu {
  display: flex;
  width: fit-content;
  align-items: center;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  padding: 0;
  position: relative;
  overflow: hidden;
}

.menu-item {
  position: relative;
  flex-shrink: 0;
}

.menu-item.active .menu-button {
  background: linear-gradient(135deg, #409eff, #5a9cf8);
  color: white;
  border-color: #409eff;
}

.menu-button {
  border: none;
  border-radius: 0;
  height: 60px;
  padding: 0 24px;
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  transition: all 0.3s ease;
  cursor: pointer;
}

.menu-button:hover {
  background-color: #ecf5ff;
  color: #409eff;
}

.arrow-icon {
  margin-left: 4px;
  transition: transform 0.3s ease;
}

.menu-item.active .arrow-icon {
  transform: rotate(90deg);
}

.submenu-horizontal {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 16px;
  background: linear-gradient(90deg, #f0f9ff, #e0f2fe);
  border-right: 3px solid #409eff;
  height: 60px;
  transition: all 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  overflow: hidden;
  transform-origin: left center;
}

.submenu-item {
  flex-shrink: 0;
}

.submenu-button {
  height: 40px;
  padding: 0 16px;
  border-radius: 10px;
  font-size: 13px;
  border: 1px solid #d9ecff;
  background: white;
  color: #606266;
  transition: all 0.4s cubic-bezier(0.25, 0.46, 0.45, 0.94);
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  position: relative;
  overflow: hidden;
  transform: translateX(-20px);
  opacity: 0;
  animation: slideInRight 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94) forwards;
  animation-delay: calc(var(--index) * 0.1s);
}

.submenu-button::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
  transition: left 0.5s ease;
}

.submenu-button:hover::before {
  left: 100%;
}

.submenu-button:hover {
  background: #409eff;
  color: white;
  border-color: #409eff;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.3);
}

.submenu-button.selected {
  background: #67c23a;
  color: white;
  border-color: #67c23a;
  box-shadow: 0 2px 8px rgba(103, 194, 58, 0.3);
}

.submenu-tag {
  margin-left: 4px;
  transform: scale(0.85);
}

/* 子菜单滑入动画 */
.submenu-slide-enter-active {
  transition: all 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.submenu-slide-leave-active {
  transition: all 0.4s cubic-bezier(0, 0.25, 0.75, 1);
}

.submenu-slide-enter-from {
  width: 0px !important;
  opacity: 0;
  transform: scaleX(0);
}

.submenu-slide-leave-to {
  width: 0px !important;
  opacity: 0;
  transform: scaleX(0);
}

/* 子菜单按钮的渐次出现动画 */
@keyframes slideInRight {
  0% {
    transform: translateX(-20px);
    opacity: 0;
  }
  100% {
    transform: translateX(0);
    opacity: 1;
  }
}

/* 离场状态的按钮样式 */
.submenu-leaving .submenu-button {
  transition: all 0.3s cubic-bezier(0.55, 0.085, 0.68, 0.53);
}

/* 增强的视觉效果 */
.menu-item:not(:last-child)::after {
  content: '';
  position: absolute;
  right: 0;
  top: 20%;
  height: 60%;
  width: 1px;
  background: linear-gradient(to bottom, transparent, #e4e7ed, transparent);
}

.menu-item.active::after {
  display: none;
}

/* 响应式设计 */
@media (max-width: 1024px) {
  .horizontal-menu {
    flex-wrap: wrap;
    height: auto;
  }
  
  .submenu-horizontal {
    width: 100% !important;
    order: 999;
    height: auto;
    padding: 16px;
    flex-wrap: wrap;
    gap: 12px;
  }
  
  .menu-button {
    padding: 0 16px;
  }
}

@media (max-width: 768px) {
  .horizontal-menu {
    flex-direction: column;
    align-items: stretch;
  }
  
  .menu-item {
    width: 100%;
  }
  
  .menu-button {
    justify-content: space-between;
    width: 100%;
    border-radius: 0;
  }
  
  .submenu-horizontal {
    width: 100% !important;
    height: auto;
    padding: 16px;
    margin: 0;
    flex-direction: column;
    align-items: stretch;
    gap: 8px;
  }
  
  .submenu-button {
    width: 100%;
    justify-content: flex-start;
    border-radius: 6px;
  }
}
</style>
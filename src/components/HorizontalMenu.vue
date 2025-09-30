<template>
  <div class="menu-container" :class="`${mode}-container`">
    <nav class="menu" :class="`${mode}-menu`">
      <template v-for="(item, index) in menuItems" :key="item.id">
        <!-- 一级菜单项 -->
        <div
          class="menu-item"
          :class="{ active: activeMenu === item.id, 'has-selected': !(activeMenu === item.id)&&hasSelectedChild(item), [`${mode}-menu-item`]: true }"
          @click="handleMenuClick(item)"
        >
          <el-button
            :type="activeMenu === item.id || hasSelectedChild(item) ? 'primary' : 'default'"
            class="menu-button"
            :class="`${mode}-menu-button`"
          >
            <el-icon><component :is="item.icon" /></el-icon>
            {{ item.label }}
            <el-icon v-if="item.children" class="arrow-icon" :class="`${mode}-arrow-icon`">
              <ArrowRight />
            </el-icon>
          </el-button>
        </div>

        <!-- 子菜单 - 横向模式插入到当前菜单项后面，纵向模式插入到下方 -->
        <transition
          :name="`submenu-${mode}`"
          @enter="onEnter"
          @leave="onLeave"
        >
          <div
            v-if="item.children && activeMenu === item.id"
            class="submenu"
            :class="`submenu-${mode}`"
            :key="`submenu-${item.id}`"
            @wheel="handleWheelScroll"
          >
            <div
              v-for="child in item.children"
              :key="child.id"
              class="submenu-item"
              :class="`submenu-item-${mode}`"
              @click="handleSubmenuClick(item, child)"
            >
              <el-button
                type="info"
                class="submenu-button"
                :class="[`submenu-button-${mode}`, { selected: child.checked }]"
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

<script setup vapor>
import { ref, nextTick, computed } from 'vue'
import { ArrowRight } from '@element-plus/icons-vue'

// Props 定义
const props = defineProps({
  menuItems: {
    type: Array,
    required: true,
    default: () => []
  },
  mode: {
    type: String,
    default: 'horizontal',
    validator: (value) => ['horizontal', 'vertical'].includes(value)
  }
})

// 事件定义
const emit = defineEmits(['menu-click', 'submenu-click'])

// 响应式数据
const activeMenu = ref(null)

// 计算属性
const hasSelectedChild = (item) => {
  return item.children?.some(child => child.checked);
};

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
      if (child.id !== childItem.id){
        child.checked = false
      }else{
        childItem.checked = !childItem.checked
      }
    })
  }
  
  emit('submenu-click', {
    parent: parentItem,
    child: childItem
  })
}

// 动画钩子
const onEnter = (el) => {
  if (props.mode === 'horizontal') {
    // 横向模式动画
    el.style.width = '0px'
    el.style.transform = 'scaleX(0)'
    el.style.transformOrigin = 'left center'
    
    nextTick(() => {
      el.style.transform = 'scaleX(1)'
      
      const buttons = el.querySelectorAll('.submenu-button')
      let totalWidth = 0
      
      buttons.forEach((button, index) => {
        totalWidth += button.offsetWidth
        if (index < buttons.length - 1) {
          totalWidth += 8 // gap 间距
        }
      })
      
      el.style.width = totalWidth + 'px'
    })
  } else {
    // 纵向模式动画
    el.style.height = '0px'
    el.style.transform = 'scaleY(0)'
    el.style.transformOrigin = 'top center'
    
    nextTick(() => {
      el.style.transform = 'scaleY(1)'
      
      const buttons = el.querySelectorAll('.submenu-button')
      let totalHeight = 0
      
      buttons.forEach((button, index) => {
        totalHeight += button.offsetHeight
        if (index < buttons.length - 1) {
          totalHeight += 8 // gap 间距
        }
      })
      
      el.style.height = totalHeight + 'px'
    })
  }
}

const onLeave = (el) => {
  el.classList.add('submenu-leaving')
  
  const buttons = el.querySelectorAll('.submenu-button')
  buttons.forEach((button, index) => {
    setTimeout(() => {
      button.style.opacity = '0'
    }, index * 80)
  })
  
  setTimeout(() => {
    if (props.mode === 'horizontal') {
      el.style.width = '0px'
      el.style.transform = 'scaleX(0)'
    } else {
      el.style.height = '0px'
      el.style.transform = 'scaleY(0)'
    }
  }, buttons.length * 80)
}

const handleWheelScroll = (event) => {
  const target = event.currentTarget
  
  if (target.classList.contains('submenu-horizontal')) {
    event.preventDefault() // 阻止默认垂直滚动
    const delta = event.deltaY || event.deltaX
    target.scrollLeft += delta // 转换为水平滚动
  }
}
</script>

<style scoped>
/* 基础容器样式 */
.menu-container {
  width: 100%;
  height: 100%;
}

.horizontal-container {
  max-width: 100%;
}

.vertical-container {
  max-height: 100%;
}

/* 菜单基础样式 */
.menu {
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  padding: 0;
  position: relative;
  overflow: hidden;
}

.horizontal-menu {
  display: flex;
  width: fit-content;
  align-items: center;
}

.vertical-menu {
  display: flex;
  flex-direction: column;
  width: 132px;
  align-items: stretch;
}

/* 菜单项样式 */
.menu-item {
  position: relative;
  flex-shrink: 0;
}

.horizontal-menu-item {
  /* 横向菜单项特定样式 */
}

.vertical-menu-item {
  width: 100%;
}

.menu-item.active .menu-button {
  background: linear-gradient(135deg, #409eff, #5a9cf8);
  color: white;
  border-color: #409eff;
}

/* 添加有选中子菜单的菜单项样式 */
.menu-item.has-selected .menu-button {
  background-color: #ecf5ff;
  color: #409eff;
}

/* 当有选中子菜单时，hover 效果保持一致 */
.menu-item.has-selected:hover .menu-button {
  background: linear-gradient(135deg, #409eff, #5a9cf8);
  color: white;
  border-color: #409eff;
}

/* 菜单按钮样式 */
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

.horizontal-menu-button {
  /* 横向按钮样式保持原有样式 */
}

.vertical-menu-button {
  width: 100%;
  justify-content: flex-start;
  border-radius: 0;
}

.menu-button:hover {
  background-color: #ecf5ff;
  color: #409eff;
}

/* 箭头图标样式 */
.arrow-icon {
  margin-left: auto;
  transition: transform 0.3s ease;
}

.horizontal-arrow-icon {
  margin-left: 4px;
}

.vertical-arrow-icon {
  margin-left: auto;
}

.menu-item.active .horizontal-arrow-icon {
  transform: rotate(90deg);
}

.menu-item.active .vertical-arrow-icon {
  transform: rotate(90deg);
}

/* 子菜单容器样式 */
.submenu {
  transition: all 0.3s cubic-bezier(0.12, 0.92, 0.25, 1.00);
  overflow: hidden;
}

.submenu-horizontal {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 16px;
  background: linear-gradient(90deg, #f0f9ff, #e0f2fe);
  border-right: 3px solid #409eff;
  height: 60px;
  max-width: 400px;    /* 限制最大宽度 */
  overflow-x: auto;    /* 水平方向溢出时显示滚动条 */
}

.submenu-vertical {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 16px;
  background: linear-gradient(180deg, #f0f9ff, #e0f2fe);
  border-left: 3px solid #409eff;
  max-height: 420px;   /* 限制最大高度 */
  overflow-y: auto;    /* 垂直方向溢出时显示滚动条 */
}

/* 子菜单项样式 */
.submenu-item {
  flex-shrink: 0;
}

.submenu-item-vertical {
  width: 100%;
}

/* 子菜单按钮样式 */
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

.submenu-button-vertical {
  width: 100%;
  /* justify-content: flex-start; */
  transform: translateY(-10px);
  animation: slideInDown 0.6s cubic-bezier(0.25, 0.46, 0.45, 0.94) forwards;
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

.submenu-horizontal::-webkit-scrollbar,
.submenu-vertical::-webkit-scrollbar {
  display: none; /* Webkit内核浏览器隐藏滚动条 */
}

/* 横向子菜单过渡动画 */
.submenu-horizontal-enter-active {
  transition: all 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.submenu-horizontal-leave-active {
  transition: all 0.4s cubic-bezier(0, 0.25, 0.75, 1);
}

.submenu-horizontal-enter-from {
  width: 0px !important;
  opacity: 0;
  transform: scaleX(0);
}

.submenu-horizontal-leave-to {
  width: 0px !important;
  opacity: 0;
  transform: scaleX(0);
}

/* 纵向子菜单过渡动画 */
.submenu-vertical-enter-active {
  transition: all 0.5s cubic-bezier(0.25, 0.46, 0.45, 0.94);
}

.submenu-vertical-leave-active {
  transition: all 0.4s cubic-bezier(0, 0.25, 0.75, 1);
}

.submenu-vertical-enter-from {
  height: 0px !important;
  opacity: 0;
  transform: scaleY(0);
}

.submenu-vertical-leave-to {
  height: 0px !important;
  opacity: 0;
  transform: scaleY(0);
}

/* 子菜单按钮动画 */
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

@keyframes slideInDown {
  0% {
    transform: translateY(-10px);
    opacity: 0;
  }
  100% {
    transform: translateY(0);
    opacity: 1;
  }
}

/* 离场状态的按钮样式 */
.submenu-leaving .submenu-button {
  transition: all 0.1s cubic-bezier(0.12, 0.92, 0.25, 1.00);
}

/* 横向模式的分隔线 */
.horizontal-menu .menu-item:not(:last-child)::after {
  content: '';
  position: absolute;
  right: 0;
  top: 20%;
  height: 60%;
  width: 1px;
  background: linear-gradient(to bottom, transparent, #e4e7ed, transparent);
}

.horizontal-menu .menu-item.active::after {
  display: none;
}

/* 纵向模式的分隔线 */
.vertical-menu .menu-item:not(:last-child)::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 10%;
  width: 80%;
  height: 1px;
  background: linear-gradient(to right, transparent, #e4e7ed, transparent);
}

.vertical-menu .menu-item.active::after {
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
  
  .horizontal-menu .menu-item {
    width: 100%;
  }
  
  .horizontal-menu .menu-button {
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
  
  .submenu-horizontal .submenu-button {
    width: 100%;
    justify-content: flex-start;
    border-radius: 6px;
  }
  
  .vertical-container {
    width: 100%;
  }
}
</style>
<template>
  <div class="demo-container">
    <!-- 使用横向菜单组件 -->
    <el-container>
      <el-aside width="auto">
        <HorizontalMenu mode="vertical" :menu-items="verticalMenu" @menu-click="handleMenuClick"
          @submenu-click="handleSubmenuClick" />
      </el-aside>

      <el-container>
        <el-header>
          <HorizontalMenu mode="horizontal" :menu-items="horizontalMenu" @menu-click="handleMenuClick"
            @submenu-click="handleSubmenuClick" />
        </el-header>
        <el-main>
          <div class="panel-bug-list">
            <BugList :bug-list="bugs" :bug-total="bugTotal" :enums="enums" :page="bugPage" :limit="bugLimit"
              @change-page="changeBugPage"></BugList>
          </div>
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup vapor>
import { ref, reactive, markRaw, computed, onMounted } from 'vue'
import HorizontalMenu from '../../components/HorizontalMenu.vue' // 引入上面的组件
import {
  User,
  Sort,
  Document,
  Odometer,
  View,
  SetUp,
  VideoPause,
  Star,
  Notebook,
} from '@element-plus/icons-vue'
import { bugList } from '../../api';
import BugList from "../../components/BugList.vue"

const props = defineProps({
  enums: {
    type: Object,
    required: true,
    default: {}
  }
});

const params = reactive({
  page: 1,
  limit: 9,
  view_state: 0,
  priority: [],
  severity: [],
  status: [],
  resolution: [],
  project_id: '',
  reporter_id: [],
  handler_id: [],
  category_id: []
});

const bugTotal = ref(0);
const bugPage = ref(0);
const bugLimit = ref(0);
const bugs = ref([]);

const horizontalMenu = reactive([
  {
    id: 'project_id',
    label: '项目',
    icon: markRaw(Document),
    children: props.enums.Project.map(ver => ({
      id: ver.key,
      label: ver.value,
      checked: false
    }))
  }, {
    id: 'view_state',
    label: '查看权限',
    icon: markRaw(View),
    children: props.enums.ViewStatus.map(ver => ({
      id: parseInt(ver.key),
      label: ver.value,
      checked: false
    }))
  }, {
    id: 'priority',
    label: '优先级',
    icon: markRaw(Sort),
    children: props.enums.Priority.map(ver => ({
      id: parseInt(ver.key),
      label: ver.value,
      checked: false
    }))
  }, {
    id: 'severity',
    label: '严重性',
    icon: markRaw(Odometer),
    children: props.enums.Severity.map(ver => ({
      id: parseInt(ver.key),
      label: ver.value,
      checked: false
    }))
  },
]);

const verticalMenu = reactive([
  {
    id: 'handler_id',
    label: '处理人',
    icon: markRaw(Star),
    children: props.enums.Users.map(ver => ({
      id: parseInt(ver.key),
      label: ver.value,
      checked: false
    }))
  }, {
    id: 'reporter_id',
    label: '上报人',
    icon: markRaw(User),
    children: props.enums.Users.map(ver => ({
      id: parseInt(ver.key),
      label: ver.value,
      checked: false
    }))
  }, {
    id: 'category_id',
    label: '分类',
    icon: markRaw(Notebook),
    children: props.enums.Category.map(ver => ({
      id: ver.key,
      label: ver.value,
      checked: false
    }))
  }, {
    id: 'status',
    label: '状态',
    icon: markRaw(SetUp),
    children: props.enums.Status.map(ver => ({
      id: parseInt(ver.key),
      label: ver.value,
      checked: false
    }))
  }, {
    id: 'resolution',
    label: '处理情况',
    icon: markRaw(VideoPause),
    children: props.enums.Resolution.map(ver => ({
      id: parseInt(ver.key),
      label: ver.value,
      checked: false
    }))
  },
])

const fetchBugList = async () => {
  try {
    const res = await bugList(params);
    bugs.value = res.bugs;
    bugTotal.value = res.total;
    bugPage.value = res.page;
    bugLimit.value = res.limit;
  } catch (error) {
    console.error('获取bug列表失败:', error);
  }
}

// 事件处理
const changeBugPage = (newPage) => {
  params.page = newPage;
  bugPage.value = newPage;
  fetchBugList();
}

const handleMenuClick = (menuItem) => {
  console.log('点击了菜单:', menuItem)
}

const handleSubmenuClick = (data) => {
  console.log('点击了子菜单:', data);

  // 判断是否为数组属性
  if (Array.isArray(params[data.parent.id])) {
    // 是数组，则添加/移除元素
    const index = params[data.parent.id].indexOf(data.child.id);
    if (index > -1) {
      params[data.parent.id].splice(index, 1);
    } else {
      params[data.parent.id].push(data.child.id);
    }
  } else {
    // 不是数组，直接赋值
    params[data.parent.id] = data.child.id;
  }

  fetchBugList();
}

onMounted(() => {
  fetchBugList();
});
</script>

<style scoped>
.el-container {
  height: 100%;
}

.demo-container {
  width: 100%;
  height: 100%;
  background: #f5f7fa;
}

.panel-bug-list {
  width: 100%;
  height: 100%;
}

:deep(.el-header) {
  margin-top: 20px;
}

:deep(.el-aside) {
  padding: 20px 0 0 20px;
}
</style>
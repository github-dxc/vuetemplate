<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { useRouter } from 'vue-router';

const router = useRouter()
const bugList = ref([]);
const total = ref(0);
const limit = ref(0);
const page = ref(0);
const enums = ref({
  Project: [],
  Priority: [],
  Severity: [],
  Reproducibility: [],
  ViewStatus: [],
  Resolution: [],
  Status: [],
  Category: []
});

// 定义每行的class
const tableRowClassName = ({
  row,
  rowIndex,
}) => {
  const status = row.status;
  if (status <= 50 || status === 85) {
    return 'primary-row';
  } else if (status === 80 || status === 81 || status === 82) {
    return 'success-row';
  } else if (status === 83 || status === 84) {
    return 'warning-row';
  }
}

// 根据传入enums的key获取value enum=[{key:1,value:"asd"}]
function getEnumValue( _enum, key) {
  if (_enum && _enum.length > 0) {
    const item = _enum.find(e => e.key === key);
    console.log("获取枚举值:", key, item);
    return item ? item.value : key; // 如果找不到对应的key，返回key本身
  }
  return key; // 如果_enum为空或未定义，返回key本身
}

// 根据状态获取颜色
function getStatusColor(status) {
  if (status <= 50 || status === 85) {
    return 'primary';
  } else if (status === 80 || status === 81 || status === 82) {
    return 'success';
  } else if (status === 83 || status === 84) {
    return 'warning';
  } else if (status === 90) {
    return 'info';
  }
}

// 一键处理已修正
async function revisedHandle(bug_id) {
  try {
    const result = await invoke("api_update_bug", { bug_id: bug_id, status: 81, resolution: 20 });
    console.log("更新成功", result);
  } catch (error) {
    // 更新成功
    console.error("更新成功");
  }
}

// 一键处理不修改
async function notReviseHandle(bug_id) {
  try {
    const result = await invoke("api_update_bug", { bug_id: bug_id, status: 83, resolution: 70 });
    console.log("更新成功", result);
  } catch (error) {
    // 更新成功
    console.error("更新成功");
  }
}

// 初始化
onMounted(async () => {
  try {
    // 获取枚举数据
    const enumsData = await invoke("api_init_data", { name: "enums" });
    console.log("枚举数据:", enumsData);
    const v = JSON.parse(enumsData)
    if (enumsData) {
      enums.value = v;
    }
    const obj = await invoke("api_bug_list", { });
    console.log(obj);
    if (obj) {
        total.value = parseInt(obj.total) || 0;
        limit.value = parseInt(obj.limit) || 0;
        page.value = parseInt(obj.page) || 0;
        bugList.value = obj.bugs || [];
    }
  } catch (error) {
    console.log(error);
    router.push("/login");
  }
});

// 监听rust发送的消息
listen('timer-tick', (event) => {
  console.log('收到定时消息:', event.payload)
  try {
    const obj = event.payload;
    if (obj) {
      total.value = parseInt(obj.total) || 0;
      limit.value = parseInt(obj.limit) || 0;
      page.value = parseInt(obj.page) || 0;
      bugList.value = obj.bugs || [];
    };
  } catch (error) {
    console.error('解析 JSON 失败:', error);
    return;
  }
})
</script>

<template>
  <main class="container">
    <el-card class="box-card">
      <div slot="header" class="clearfix">
        <span>Bug 列表</span>
      </div>

      <el-table :data="bugList" style="width: 100%;font-size: 12px" :row-class-name="tableRowClassName">
        <el-table-column label="ID" width="60" >
          <template #default="scope">
            <el-link type="primary" :href="'http://bug.test.com/view.php?id=' + scope.row.bug_id" target="_blank">{{ scope.row.bug_id }}</el-link>
          </template>
        </el-table-column>

        <el-table-column prop="project" label="项目名称" width="100" />
        <el-table-column prop="handler" label="处理人" width="80" />
        <el-table-column prop="summary" label="摘要" width="200" show-overflow-tooltip>
          <template #default="scope">
            <div class="multi-line-ellipsis">
              {{ scope.row.summary }}
            </div>
          </template>
        </el-table-column>
        <el-table-column prop="priority" label="状态" width="80">
          <template #default="scope">
            <el-tag :type="getStatusColor(scope.row.status)">{{ getEnumValue(enums.Status,scope.row.status) }}</el-tag>
          </template>
        </el-table-column>
        <el-table-column label="操作" width="180">
          <template #default="scope">
            <el-dropdown split-button type="primary" @click="handleClick" @command="handleCommand" size="small">
              已修正
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item>不修改</el-dropdown-item>
                  <el-dropdown-item>Action 2</el-dropdown-item>
                  <el-dropdown-item>Action 3</el-dropdown-item>
                  <el-dropdown-item divided>Action 4</el-dropdown-item>
                  <el-dropdown-item>Action 5</el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
            <el-button type="success" size="small" @click="revisedHandle(scope.row.bug_id)" plain>已修正</el-button>
            <el-button type="warning" size="small" @click="notReviseHandle(scope.row.bug_id)" plain>不修改</el-button>
          </template>
        </el-table-column>
      </el-table>

      <div class="pagination">
        <el-pagination background layout="prev, pager, next" :total="total" :page="page" />
      </div>
    </el-card>
  </main>
</template>

<style scoped>
.box-card {
  max-width: 800px;
  margin: 40px auto;
}

.pagination {
  text-align: center;
  margin-top: 20px;
}

.multi-line-ellipsis {
  display: -webkit-box;
  -webkit-line-clamp: 2; /* 限制显示两行 */
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: normal; /* 允许换行 */
}

::v-deep(.warning-row) {
  --el-table-tr-bg-color: var(--el-color-warning-light-9);
}

::v-deep(.success-row) {
  --el-table-tr-bg-color: var(--el-color-success-light-9);
}

::v-deep(.primary-row) {
  --el-table-tr-bg-color: var(--el-color-primary-light-9);
}

</style>
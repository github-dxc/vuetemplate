<template>
  <main class="container">
    <el-card class="box-card">
      <div slot="header" class="clearfix">
        <span>Bug 列表</span>
      </div>

      <el-table :data="bugList" style="width: 100%;font-size: 12px" :row-class-name="tableRowClassName">
        <el-table-column label="ID" width="60" header-align="center" >
          <template #default="scope">
            <el-link type="primary" :href="'http://bug.test.com/view.php?id=' + scope.row.bug_id" target="_blank">{{ scope.row.bug_id }}</el-link>
          </template>
        </el-table-column>

        <el-table-column prop="project" label="项目名称" width="100" header-align="center" />

        <el-table-column label="状态" width="80" header-align="center">
          <template #default="scope">
            <el-tag :type="getStatusColor(scope.row.status)">{{ bugStatus.get(scope.row.status) }}</el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="handler" label="处理人" width="60" header-align="center" />

        <el-table-column label="摘要" width="200" show-overflow-tooltip header-align="center" >
          <template #default="scope">
            <div class="multi-line-ellipsis">
              {{ scope.row.summary }}
            </div>
          </template>
        </el-table-column>

        <el-table-column label="附件" width="60" header-align="center" >
          <template #default="scope">
            <div v-if="scope.row.attachments>0">
              <el-link icon="PictureFilled">
                *{{ scope.row.attachments }}
              </el-link>
            </div>
          </template>
        </el-table-column>

        <el-table-column label="优先级" width="70" header-align="center" >
          <template #default="scope">
            <el-row :gutter="20">
              <el-col style="text-align: center;">
                {{ priorityText[scope.row.priority] || '-' }}
              </el-col>
            </el-row>
            <el-row :gutter="20">
              <el-col>
                <el-tag :type="getSeverityColor(scope.row.severity)">{{ bugSeverity.get(scope.row.severity) || '-' }}</el-tag>
              </el-col>
            </el-row>
          </template>
        </el-table-column>

        <el-table-column label="操作" width="150" header-align="center" >
          <template #default="scope">
            <el-dropdown split-button type="primary" 
              @click="handleCommand({status: workableStatus[scope.row.status][0],bug_id: scope.row.bug_id})" @command="handleCommand">
              {{ bugStatus.get(workableStatus[scope.row.status][0]) }}
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item v-for="(s, i) in workableStatus[scope.row.status]" :key="s" :command="{status: s,bug_id: scope.row.bug_id}" :disabled="i === 0">
                    {{ bugStatus.get(s) }}
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </template>
        </el-table-column>
      </el-table>

      <div class="pagination">
        <el-pagination background layout="prev, pager, next" :total="total" :page="page" />
      </div>
    </el-card>
  </main>
</template>
<script setup vapor>
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { useRouter } from 'vue-router';

//------------------data-------------------//

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

// 获取Status的值
const bugStatus = computed(() => {
  const myMap = new Map();
  enums.value.Status.forEach(item => {
    myMap.set(item.key, item.value);
  });
  return myMap;
})

// 获取Severity的值
const bugSeverity = computed(() => {
  const myMap = new Map();
  enums.value.Severity.forEach(item => {
    myMap.set(item.key, item.value);
  });
  return myMap;
})

// 获取Status颜色
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

// 获取Severity颜色
function getSeverityColor(severity) {
  if (severity <= 30) {
    return 'primary';
  } else if (severity <= 50) {
    return 'warning';
  } else if (severity >= 60) {
    return 'error';
  }
}

// 获取可操作的状态
const workableStatus =  {
  10: [50, 90],
  20: [50, 90],
  30: [50, 90],
  40: [50, 90],
  50: [80, 81, 83, 84, 90],
  80: [90, 85],
  81: [90, 85],
  82: [90, 85],
  83: [90, 85],
  84: [90, 85],
  85: [80, 81, 83, 84],
  90: [85],
}

// 获取优先级自定义显示文本
const priorityText = {
  10: '-',
  20: '🔥',
  30: '🔥',
  40: '🔥🔥',
  50: '🔥🔥🔥',
  60: '🔥🔥🔥🔥',
}

// 定义每行的class
const tableRowClassName = ({
  row,
  rowIndex,
}) => {
  const status = row.status;
  if (status <= 50 || status === 85) {
    return 'warning-row';
  } else if (status === 80 || status === 81) {
    return 'primary-row';
  } else if (status === 82) {
    return 'success-row';
  }
}

//------------------api-------------------//

async function api_init_data() {
  try {
    let data = await invoke("api_init_data", { name: "enums" });
    console.log("api_init_data:", data);
    let enumsData = JSON.parse(data)
    if (enumsData) {
      enums.value = enumsData;
    }
  } catch (error) {
    console.log(error);
  }
}

async function api_bug_list(params) {
  try {
    // 获取bug列表
    let data = await invoke("api_bug_list",params);
    console.log("api_bug_list:", data);
    if (data) {
        total.value = parseInt(data.total) || 0;
        limit.value = parseInt(data.limit) || 0;
        page.value = parseInt(data.page) || 0;
        bugList.value = data.bugs || [];
    }
  } catch (error) {
    console.log(error);
    router.push("/login");
  }
}

async function handleCommand(command) {
  if (!(command.status && command.bug_id)){
    return
  }
  console.log("处理命令:", command);
  let status = command.status;
  let bug_id = command.bug_id;
  try {
    let resolution = 0;
    if (status === 80 || status === 81 || status === 82) {
      resolution = 20;
    }else if (status === 83) {
      resolution = 90;
    }else if (status === 84) {
      resolution = 80;
    }else if (status === 85) {
      resolution = 30;
    }
    const result = await invoke("api_update_bug", { bug_id: bug_id, status: status, resolution: resolution });
    console.log("更新成功", result);
    api_bug_list({});
  } catch (error) {
    console.error("更新失败", error);
  }
}

//------------------vue/tauri-------------------//

// 初始化
onMounted(async () => {
    // 初始化枚举数据
    api_init_data()
    // 查询bug列表
    api_bug_list({})
});

// 监听rust发送的消息
listen('sub_bugs', (event) => {
  console.log('收到定时消息:', event.payload)
  try {
    const obj = event.payload;
    if (obj) {
      // total.value = parseInt(obj.total) || 0;
      // limit.value = parseInt(obj.limit) || 0;
      // page.value = parseInt(obj.page) || 0;
      // bugList.value = obj.bugs || [];
      bugList.value = obj;
    };
  } catch (error) {
    console.error('解析 JSON 失败:', error);
    return;
  }
})
</script>

<style>
.el-button-group .el-button--primary:first-child {
  width: 75px;
}
</style>

<style scoped>
.box-card {
  max-width: 1000px;
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
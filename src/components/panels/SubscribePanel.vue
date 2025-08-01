<template>
  <div class="content-panel">
    <div class="panel-header">
      <div class="header-content">
        <h3>🐛 订阅信息</h3>
      </div>
    </div>

    <div class="table-container">
      <el-table 
        :data="bugList" 
        class="custom-table"
        :row-class-name="tableRowClassName"
        stripe
        border
        size="default"
        :header-cell-style="{ background: '#f8fafc', color: '#374151', fontWeight: '600' }"
      >
        <el-table-column label="BugID" width="80" header-align="center">
          <template #default="scope">
            <el-link 
              type="primary" 
              :href="'http://bug.test.com/view.php?id=' + scope.row?.bug_id" 
              target="_blank"
              class="bug-id-link"
            >
              #{{ scope.row.bug_id }}
            </el-link>
          </template>
        </el-table-column>

        <el-table-column prop="project" label="项目名称" width="120" header-align="center">
          <template #default="scope">
            <div class="project-name">
              <el-icon class="project-icon"><Folder /></el-icon>
              {{ scope.row.project }}
            </div>
          </template>
        </el-table-column>

        <el-table-column label="状态" width="100" header-align="center">
          <template #default="scope">
            <el-tag 
              :type="getStatusColor(scope.row.status)"
            >
              {{ bugStatus.get(scope.row.status) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="handler" label="处理人" width="110" header-align="center">
          <template #default="scope">
            <div class="handler-info">
              <span class="handler-name">{{ scope.row.handler || '未分配' }}</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column label="摘要" min-width="auto" show-overflow-tooltip header-align="center">
          <template #default="scope">
            <div class="summary-content">
              <el-icon class="summary-icon"><Document /></el-icon>
              <span class="summary-text">{{ scope.row.summary }}</span>
            </div>
          </template>
        </el-table-column>

        <el-table-column label="附件" width="80" align="center" header-align="center">
          <template #default="scope">
            <div v-if="scope.row.attachments > 0" class="attachment-info">
              <el-badge :value="scope.row.attachments" type="primary" class="attachment-badge" :max="9">
                <el-link class="attachment-icon" icon="PictureFilled" ></el-link>
              </el-badge>
            </div>
            <span v-else class="no-attachment">-</span>
          </template>
        </el-table-column>

        <el-table-column label="优先级" width="90" align="center" header-align="center">
          <template #default="scope">
            <div class="priority-container">
              <div class="priority-level">
                {{ priorityText[scope.row.priority] || '-' }}
              </div>
              <el-tag 
                :type="getSeverityColor(scope.row.severity)" 
                size="small"
                effect="plain"
                class="severity-tag"
              >
                {{ bugSeverity.get(scope.row.severity) || '-' }}
              </el-tag>
            </div>
          </template>
        </el-table-column>

        <el-table-column label="操作" width="160" align="center" header-align="center">
          <template #default="scope">
            <el-dropdown 
              split-button 
              type="primary"
              size="default"
              class="action-dropdown"
              @click="handleCommand({ status: workableStatus[scope.row.status][0], bug_id: scope.row.bug_id })"
              @command="handleCommand"
            >
              {{ bugStatus.get(workableStatus[scope.row.status][0]) }}
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item 
                    v-for="(s, i) in workableStatus[scope.row.status]" 
                    :key="s"
                    :command="{ status: s, bug_id: scope.row.bug_id }" 
                    :disabled="i === 0"
                    class="dropdown-item"
                  >
                    {{ bugStatus.get(s) }}
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </template>
        </el-table-column>
      </el-table>
    </div>

    <div class="pagination-container">
      <el-pagination 
        background 
        layout="prev, pager, next, jumper, total" 
        :total="total" 
        :current-page="page"
        :page-size="limit"
        class="custom-pagination"
        @current-change="handlePageChange"
      />
    </div>
  </div>
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
const limit = ref(10);
const page = ref(1);
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
    return 'warning';
  } else if (status === 80 || status === 81 || status === 82) {
    return 'success';
  } else if (status === 83 || status === 84) {
    return 'danger';
  } else if (status === 90) {
    return 'info';
  }
  return 'primary';
}

// 获取Severity颜色
function getSeverityColor(severity) {
  if (severity <= 30) {
    return 'success';
  } else if (severity <= 50) {
    return 'warning';
  } else if (severity >= 60) {
    return 'danger';
  }
  return 'primary';
}

// 获取可操作的状态
const workableStatus = {
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
  10: '⚪',
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
    return 'success-row';
  } else if (status === 82) {
    return 'completed-row';
  } else if (status === 83 || status === 84) {
    return 'danger-row';
  }
  return '';
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
    let data = await invoke("api_init_bugs", params);
    console.log("api_init_bugs:", data);
    if (data) {
      bugList.value = data || [];
    }
  } catch (error) {
    console.log(error);
    router.push("/login");
  }
}

async function handleCommand(command) {
  if (!(command.status && command.bug_id)) {
    return
  }
  console.log("处理命令:", command);
  let status = command.status;
  let bug_id = command.bug_id;
  try {
    let resolution = 0;
    if (status === 80 || status === 81 || status === 82) {
      resolution = 20;
    } else if (status === 83) {
      resolution = 90;
    } else if (status === 84) {
      resolution = 80;
    } else if (status === 85) {
      resolution = 30;
    }
    const result = await invoke("api_update_bug", { bug_id: bug_id, status: status, resolution: resolution });
    console.log("更新成功", result);
    api_bug_list({});
  } catch (error) {
    console.error("更新失败", error);
  }
}

function handlePageChange(currentPage) {
  page.value = currentPage;
  api_bug_list({ page: currentPage });
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
      bugList.value = obj;
    };
  } catch (error) {
    console.error('解析 JSON 失败:', error);
    return;
  }
})
</script>

<style scoped>
.content-panel {
  flex: 1;
  max-width: 100%;
  max-height: 100%;
  width: 100%; 
  height: 100%;
  overflow: auto;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.panel-header {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border: none;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.04);
  padding: 20px 24px;
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.panel-header h3 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #1f2937;
  display: flex;
  align-items: center;
  gap: 8px;
}

.table-container {
  flex: 1;
  padding: 20px;
  height: 60%;
  background: transparent;
}

.table-container .el-table {
  height: 100%;
}

.custom-table {
  border-radius: 12px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  background: white;
}

.custom-table :deep(.el-table__header) {
  background: #f8fafc;
}

.custom-table :deep(.el-table__row) {
  transition: all 0.3s ease;
}

.custom-table :deep(.el-table__row:hover) {
  background: #f8fafc !important;
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.bug-id-link {
  font-weight: 600;
  font-family: 'Monaco', 'Menlo', monospace;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  text-decoration: none;
}

.project-name {
  display: flex;
  align-items: center;
  gap: 6px;
  font-weight: 500;
}

.project-icon {
  color: #6366f1;
}

.handler-info {
  display: flex;
  align-items: center;
  gap: 8px;
  justify-content: center;
}

.handler-name {
  font-size: 12px;
  color: #4b5563;
  font-weight: 500;
}

.summary-content {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
}

.summary-icon {
  color: #10b981;
  flex-shrink: 0;
}

.summary-text {
  flex: 1;
  font-size: 13px;
  line-height: 1.4;
  color: #374151;
  /* 限制显示两行 */
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: normal;
}

.attachment-info {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 30px;
  margin-top: 4px;
}

.attachment-badge :deep(.el-badge__content) {
  background: #eb6969;
  margin-top: 5px;
  width: 14px;
  height: 14px;
  padding: 0;
  border: none;
}

.attachment-icon {
  color: #10b981;
  margin-top: 2px;
  font-size: 25px;
}

.no-attachment {
  color: #9ca3af;
  font-size: 14px;
}

.priority-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}

.priority-level {
  font-size: 16px;
  line-height: 1;
}

.severity-tag {
  font-size: 10px;
  min-width: 40px;
  text-align: center;
}

.action-dropdown {
  border-radius: 8px;
}

.action-dropdown :deep(.el-button--primary) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  border-radius: 8px;
  font-weight: 500;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.3);
  transition: all 0.3s ease;
}

.action-dropdown :deep(.el-button--primary:hover) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 6px;
}

.current-status {
  color: #10b981;
}

.pagination-container {
  padding: 20px 24px;
  text-align: center;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  border-top: 1px solid rgba(0, 0, 0, 0.06);
}

.custom-pagination :deep(.el-pager li) {
  background: white;
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  margin: 0 2px;
  transition: all 0.3s ease;
}

.custom-pagination :deep(.el-pager li:hover) {
  background: #667eea;
  color: white;
  transform: translateY(-1px);
}

.custom-pagination :deep(.el-pager li.is-active) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-color: transparent;
}

/* 行状态颜色 */
:deep(.warning-row) {
  background: linear-gradient(90deg, rgba(251, 191, 36, 0.05) 0%, rgba(251, 191, 36, 0.02) 100%) !important;
  border-left: 3px solid #f59e0b;
}

:deep(.success-row) {
  background: linear-gradient(90deg, rgba(16, 185, 129, 0.05) 0%, rgba(16, 185, 129, 0.02) 100%) !important;
  border-left: 3px solid #10b981;
}

:deep(.completed-row) {
  background: linear-gradient(90deg, rgba(59, 130, 246, 0.05) 0%, rgba(59, 130, 246, 0.02) 100%) !important;
  border-left: 3px solid #3b82f6;
}

:deep(.danger-row) {
  background: linear-gradient(90deg, rgba(239, 68, 68, 0.05) 0%, rgba(239, 68, 68, 0.02) 100%) !important;
  border-left: 3px solid #ef4444;
}

/* Tag样式优化 */
:deep(.el-tag) {
  border: none;
  font-weight: 500;
  letter-spacing: 0.5px;
}

:deep(.el-tag--success) {
  background: linear-gradient(135deg, #10f794 0%, #059669 100%);
  color: white;
}

:deep(.el-tag--warning) {
  background: linear-gradient(135deg, #fbbf24 0%, #d97706 100%);
  color: white;
}

:deep(.el-tag--danger) {
  background: linear-gradient(135deg, #f87171 0%, #dc2626 100%);
  color: white;
}

:deep(.el-tag--primary) {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

:deep(.el-tag--info) {
  background: linear-gradient(135deg, #6b7280 0%, #4b5563 100%);
  color: white;
}
</style>
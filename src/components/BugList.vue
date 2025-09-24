<template>
    <div class="table-panel">
        <div class="table-container">
            <el-table :data="bugList" class="custom-table" :row-class-name="tableRowClassName" stripe border
                size="default" :header-cell-style="{ background: '#f8fafc', color: '#374151', fontWeight: '600' }">
                <el-table-column label="BugID" width="85" header-align="center">
                    <template #default="scope">
                        <el-link type="primary" class="bug-id-link"
                            @click="browserOpen(`/view.php?id=${scope.row.bug_id}`)">
                            #{{ scope.row.bug_id }}
                        </el-link>
                        <el-link class="summary-icon"
                            @click="copyMessage(`http://${host}/view.php?id=${scope.row.bug_id}`)"
                            icon="Document"></el-link>
                    </template>
                </el-table-column>

                <el-table-column prop="project" label="项目" width="120" header-align="center">
                    <template #default="scope">
                        <div class="project-name">
                            {{ scope.row.project }}
                        </div>
                    </template>
                </el-table-column>

                <el-table-column label="状态" width="100" header-align="center">
                    <template #default="scope">
                        <el-tag :type="getStatusColor(scope.row.status)">
                            {{ bugStatus.get(String(scope.row.status)) }}
                        </el-tag>
                    </template>
                </el-table-column>

                <el-table-column prop="handler" label="处理人" width="110" header-align="center">
                    <template #default="scope">
                        <div class="handler-info">
                            <el-popover placement="bottom-start" :width="400" trigger="click" :hide-after="10">
                                <template #reference>
                                    <el-button type="warning" size="small" plain>{{
                                        bugUsers.get(String(scope.row.handler_id)) }}</el-button>
                                </template>
                                <div class="flex gap-2 mt-4">
                                    <el-check-tag class="option_check_tag" v-for="item in bugUsers"
                                        :checked="item[0] === String(scope.row.handler_id)" type="warning"
                                        @change="changeBug({ bug_id: scope.row.bug_id, handler_id: Number(item[0]) })">{{
                                        item[1] }}</el-check-tag>
                                </div>
                            </el-popover>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column label="摘要" width="auto" show-overflow-tooltip header-align="center">
                    <template #default="scope">
                        <div class="summary-content">
                            <el-link class="summary-icon" @click="copyMessage(scope.row.summary)"
                                icon="Document"></el-link>
                            <span class="summary-text">{{ scope.row.summary }}</span>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column label="附件" width="60" align="center" header-align="center">
                    <template #default="scope">
                        <div v-if="scope.row.attachments > 0" class="attachment-info">
                            <el-badge :value="scope.row.attachments" type="primary" class="attachment-badge" :max="9">
                                <el-link class="attachment-icon" icon="PictureFilled"
                                    @click="openImagePreview(scope.row.bug_id)"></el-link>
                            </el-badge>
                        </div>
                        <span v-else class="no-attachment">-</span>
                    </template>
                </el-table-column>

                <el-table-column label="明细" width="60" align="center" header-align="center">
                    <template #default="scope">
                        <div class="attachment-info">
                            <el-badge v-if="scope.row.issue_notes_count" :value="scope.row.issue_notes_count"
                                type="primary" class="attachment-badge" :max="9">
                                <el-link class="attachment-icon" icon="Memo"
                                    @click="openBugDetails(scope.row.bug_id)"></el-link>
                            </el-badge>
                            <el-link v-else class="attachment-icon" icon="Memo"
                                @click="openBugDetails(scope.row.bug_id)"></el-link>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column label="优先级" width="90" align="center" header-align="center">
                    <template #default="scope">
                        <div class="priority-container">
                            <div class="priority-level">
                                {{ priorityText[scope.row.priority] || '-' }}
                            </div>
                            <el-tag :type="getSeverityColor(scope.row.severity)" size="small" effect="plain"
                                class="severity-tag">
                                {{ bugSeverity.get(String(scope.row.severity)) || '-' }}
                            </el-tag>
                        </div>
                    </template>
                </el-table-column>

                <el-table-column label="快捷操作" width="130" align="center" header-align="center">
                    <template #default="scope">
                        <el-dropdown split-button type="primary" size="default" class="action-dropdown"
                            @click="handleCommand({ status: workableStatus[scope.row.status][0], bug_id: scope.row.bug_id })"
                            @command="handleCommand">
                            {{ bugStatus.get(String(workableStatus[scope.row.status][0])) }}
                            <template #dropdown>
                                <el-dropdown-menu>
                                    <el-dropdown-item v-for="(s, i) in workableStatus[scope.row.status]" :key="s"
                                        :command="{ status: s, bug_id: scope.row.bug_id }" :disabled="i === 0"
                                        class="dropdown-item">
                                        {{ bugStatus.get(String(s)) }}
                                    </el-dropdown-item>
                                    <el-dropdown-item divided :command="{ type: 'comment', bug_id: scope.row.bug_id }"
                                        class="dropdown-item">
                                        <el-icon>
                                            <EditPen />
                                        </el-icon>
                                        评论
                                    </el-dropdown-item>
                                </el-dropdown-menu>
                            </template>
                        </el-dropdown>
                    </template>
                </el-table-column>
            </el-table>

            <div class="pagination-container">
                <el-pagination background layout="prev, pager, next, jumper, total" :total="bugTotal"
                    :current-page="page" :page-size="limit" class="custom-pagination"
                    @current-change="handlePageChange" />
            </div>

        </div>

        <!-- 明细展示 -->
        <div v-if="bugDetailsVisible">
            <el-drawer v-model="bugDetailsVisible" :title="bugDetailsTitle" :show-close="false" direction="rtl"
                size="71%">
                <BugDetails :bug-id="bugId" :enums="enums" @set-title="setTitle" />
            </el-drawer>
        </div>
        <!-- 评论展示 -->
        <Annotation :bug-id="bugId" @success="annotationDialogVisible = false" @close="annotationDialogVisible = false"
            :dialog-visible="annotationDialogVisible" />
    </div>
</template>

<script setup vapor>
import { ref, onMounted, computed } from "vue";
import { listen, emit } from '@tauri-apps/api/event';
import { createNewWindow } from "../windows";
import { apiBugInfo, browserOpen, updateBug } from "../api";
import { ElMessage } from "element-plus";
import { useUserStore } from "../store";
import BugDetails from "./BugDetails.vue";
import { EditPen } from "@element-plus/icons-vue";
import Annotation from "./Annotation.vue";
import { copyMessage } from "../util";

const props = defineProps({
    page: {
        type: Number,
        required: true,
        default: 1
    },
    limit: {
        type: Number,
        required: true,
        default: 10
    },
    bugList: {
        type: Array,
        required: true,
        default: []
    },
    bugTotal: {
        type: Number,
        required: true,
        default: 0
    },
    enums: {
        type: Object,
        required: true,
        default: {}
    }
});

const emits = defineEmits(['change-page']);

//------------------data-------------------//
const userStore = useUserStore();
const bugDetailsVisible = ref(false);
const bugDetailsTitle = ref("Bug明细");
const bugId = ref(0);
const annotationDialogVisible = ref(false);

const host = computed(() => {
    return userStore.serverHost;
})

const bugUsers = computed(() => {
    const myMap = new Map();
    props.enums?.Users?.forEach(item => {
        if (item.key !== "0") {
            myMap.set(item.key, item.value);
        }
    });
    return myMap;
})

const bugStatus = computed(() => {
    const myMap = new Map();
    props.enums?.Status?.forEach(item => {
        myMap.set(item.key, item.value);
    });
    return myMap;
})

const bugSeverity = computed(() => {
    const myMap = new Map();
    props.enums?.Severity?.forEach(item => {
        myMap.set(item.key, item.value);
    });
    return myMap;
})

// 获取Status颜色
const getStatusColor = (status) => {
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
const getSeverityColor = (severity) => {
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

const handleCommand = async (command) => {
    if (command.type) {
        switch (command.type) {
            //评论
            case 'comment':
                bugId.value = command.bug_id;
                annotationDialogVisible.value = true;
        }
        return;
    }
    if (!(command.status && command.bug_id)) {
        return;
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
        const result = await updateBug({ bug_id: bug_id, status: status, resolution: resolution });
        console.log("更新成功", result);
    } catch (error) {
        ElMessage({
            message: '更新失败，请稍后重试',
            type: 'error',
        });
        console.error("更新失败", error);
    }
}

const changeBug = function (data) {
    updateBug(data).then(result => {
        console.log("更新成功:", result);
    }).catch(error => {
        ElMessage({
            message: '更新失败，请稍后重试',
            type: 'error',
        });
        console.error("更新失败:", error);
    });
}

const handlePageChange = (currentPage) => {
    emits('change-page', currentPage);
}

// 打开图片预览
const openImagePreview = async (bug_id) => {
    const DOMContentLoadedCallback = () => {
        // 发送图片信息列表给图片预览窗口
        apiBugInfo(bug_id).then(result => {
            console.log("成功:", result);
            let sendData = result.bugnote_notes || [];
            if (result.attachments && result.attachments.length > 0) {
                let attNote = { note_id: 0, time: 0, handler_id: 0, handler: "", text: "", attachments: result.attachments };
                sendData = [attNote, ...result.bugnote_notes];
            }
            emit('web_images', { bugnote_notes: sendData });

        }).catch(error => {
            console.error("错误:", error);
        });
    };
    // label需要在capabilities/default.json中声明权限
    await createNewWindow('image', {
        url: '/image', // 窗口加载的URL
        title: 'image',
        width: 1600,
        height: 900,
        visible: false,
        resizable: true,
        center: true,
        transparent: false,//背景是否透明
        decorations: true,//是否有边框
    }, DOMContentLoadedCallback);
}

// 打开bug详情
const openBugDetails = async (bug_id) => {
    bugDetailsVisible.value = true;
    bugId.value = bug_id;
}
const setTitle = (title) => {
    bugDetailsTitle.value = title;
}

// 初始化
onMounted(async () => {
});

</script>

<style scoped>
.table-panel {
    flex: 1;
    width: 100%;
    height: 100%;
    overflow: auto;
    display: flex;
    flex-direction: column;
    background: transparent;
}

.table-container {
    flex: 1;
    height: 60%;
    background: transparent;
}

.table-container .el-table {
    height: 90%;
    border-radius: 14px 14px 0 0;
}

.custom-table {
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
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.bug-id-link {
    font-weight: 600;
    font-family: 'Monaco', 'Menlo', monospace;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    margin: 0 2px;
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

.handler-info {
    display: flex;
    align-items: center;
    gap: 8px;
    justify-content: center;
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
    letter-spacing: -8px;
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
    display: flex;
    align-items: center;
    /* 垂直居中 */
    height: 10%;
    text-align: center;
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    border-radius: 0 0 14px 14px;
}

.el-pagination {
    margin: 0 5px;
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

:deep(.el-drawer__header) {
    margin-bottom: 0px !important;
}

.option_check_tag {
    position: relative;
    margin: 3px;
}
</style>
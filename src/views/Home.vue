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

// 根据传入enums的key获取value enum=[{key:1,value:"asd"}]
function getEnumValue( _enum, key) {
  if (_enum && _enum.length > 0) {
    const item = _enum.find(e => e.key === key);
    console.log("获取枚举值:", key, item);
    return item ? item.value : key; // 如果找不到对应的key，返回key本身
  }
  return key; // 如果_enum为空或未定义，返回key本身
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

      <el-table :data="bugList" style="width: 100%">
        <el-table-column label="ID" width="60" >
          <template #default="scope">
            <el-link type="primary" :href="'http://bug.test.com/view.php?id=' + scope.row.bug_id" target="_blank">{{ scope.row.bug_id }}</el-link>
          </template>
        </el-table-column>

        <el-table-column prop="project" label="项目名称" width="80" />
        <el-table-column prop="handler" label="处理人" width="80" />
        <el-table-column prop="summary" label="摘要" width="200" />
        <el-table-column prop="priority" label="状态" width="80">
          <template #default="scope">
            {{ getEnumValue(enums.Status,scope.row.status) }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="180">
          <template #default="scope">
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

</style>
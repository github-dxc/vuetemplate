<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event'

const bugList = ref([
  { id: '1', system: 'Windows', title: '无法启动应用' },
  { id: '2', system: 'Linux', title: '权限错误' },
  { id: '3', system: 'macOS', title: '界面显示异常' }
]);
const testStr = ref("asd");

// 一键处理已修正
async function revisedHandle() {
  
}

// 一键处理不修改
async function notReviseHandle() {
  
}

// 监听rust发送的消息
listen('timer-tick', (event) => {
  console.log('收到定时消息:', event.payload)
  testStr.value = event.payload;
})
</script>

<template>
  <main class="container">
    <el-card class="box-card">
      <div slot="header" class="clearfix">
        <span>Bug 列表{{ testStr }}</span>
      </div>

      <el-table :data="bugList" style="width: 100%">
        <el-table-column prop="id" label="ID" width="180" />
        <el-table-column prop="system" label="系统" width="180" />
        <el-table-column prop="title" label="标题" />
        <el-table-column label="操作" width="180">
          <template #default="scope">
            <el-button type="success" size="small" @click="revisedHandle" plain>已修正</el-button>
            <el-button type="warning" size="small" @click="notReviseHandle" plain>不修改</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </main>
</template>

<style scoped>
.box-card {
  max-width: 800px;
  margin: 40px auto;
}

</style>
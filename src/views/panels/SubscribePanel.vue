<template>
  <div class="content-panel">
    <div class="panel-header">
      <div class="header-content">
        <h3>🐛 订阅信息</h3>
      </div>
    </div>
    <div class="panel-bug-list">
      <BugList :bug-list="bugList" :bug-total="bugTotal" :enums="enums" :page="1" :limit="999"></BugList>
    </div>
  </div>
</template>

<script setup vapor>
import { onMounted } from "vue";
import BugList from "../../components/BugList.vue"

const props = defineProps({
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

// 初始化
onMounted(async () => {
});

</script>

<style scoped>
.content-panel {
  flex: 1;
  width: 100%; 
  height: 100%;
  overflow: auto;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  display: flex;
  flex-direction: column;
  max-height: 100vh;
}

.panel-header {
  height: auto;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  border: none;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.04);
}

.header-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin: 20px 24px;
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

.panel-bug-list {
  height: 85%;
  padding: 25px 20px;
}
</style>
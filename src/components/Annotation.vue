<template>
  <div>
    <!-- 反馈按钮 -->
    <div 
      class="feedback-button"
      @click="openDialog"
      style="width: 50px; height: 60px;"
    >
      <el-icon :size="16">
        <ChatRound />
      </el-icon>
      <span class="feedback-text">反馈</span>
    </div>

    <!-- 反馈对话框 -->
    <el-dialog
      v-model="dialogVisible"
      title="反馈"
      width="750px"
      :before-close="handleClose"
      append-to-body
    >
      <el-form
        ref="feedbackFormRef"
        :model="feedbackForm"
        :rules="formRules"
        label-width="80px"
      >
        <el-form-item label="内容" prop="content">
          <el-input
            v-model="feedbackForm.content"
            type="textarea"
            :rows="4"
            placeholder="请详细描述您的问题或建议..."
            :maxlength="500"
            show-word-limit
          />
        </el-form-item>

        <el-form-item label="附件">
          <el-upload
            ref="uploadRef"
            :file-list="fileList"
            :on-change="handleFileChange"
            :on-remove="handleFileRemove"
            :before-upload="() => false"
            multiple
            drag
          >
            <el-icon class="el-icon--upload">
              <UploadFilled />
            </el-icon>
            <div class="el-upload__text">
              将文件拖到此处，或<em>点击上传</em>
            </div>
            <template #tip>
              <div class="el-upload__tip">
                支持jpg/png/gif文件，且不超过5MB
              </div>
            </template>
          </el-upload>
        </el-form-item>
      </el-form>

      <template #footer>
        <div class="dialog-footer">
          <el-button @click="handleClose">取消</el-button>
          <el-button
            type="primary"
            @click="handleSubmit"
            :loading="submitLoading"
          >
            提交反馈
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { ChatRound, UploadFilled } from '@element-plus/icons-vue'

// Props 定义
const props = defineProps({
})

// Emits 定义
const emit = defineEmits(['submit', 'close', 'open'])

// 响应式数据
const dialogVisible = ref(false)
const submitLoading = ref(false)
const feedbackFormRef = ref()
const uploadRef = ref()
const fileList = ref([])
const formRules = {
  content: [
    { required: true, message: '反馈内容不能为空', trigger: 'blur' },
    { min: 10, max: 500, message: '内容长度应在10到500字符之间', trigger: 'blur' }
  ]
}

// 表单数据
const feedbackForm = reactive({
  content: ''
})

// 方法
const openDialog = () => {
  dialogVisible.value = true
  emit('open')
}

const handleClose = () => {
  dialogVisible.value = false
  resetForm()
  emit('close')
}

const resetForm = () => {
  feedbackForm.content = ''
  fileList.value = []
  if (feedbackFormRef.value) {
    feedbackFormRef.value.clearValidate()
  }
}

const handleSubmit = async () => {
  if (!feedbackFormRef.value) return
  
  try {
    const valid = await feedbackFormRef.value.validate()
    if (!valid) return
    
    submitLoading.value = true
    
    // 准备提交数据
    const submitData = {
      ...feedbackForm,
      files: fileList.value,
      timestamp: new Date().toISOString()
    }
    
    // 触发提交事件
    emit('submit', submitData)
    
    // 模拟提交延迟
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    ElMessage.success('反馈提交成功！')
    handleClose()
    
  } catch (error) {
    console.error('表单验证失败:', error)
  } finally {
    submitLoading.value = false
  }
}

const handleFileChange = (file, fileList) => {
  // 文件大小限制 5MB
  const maxSize = 5 * 1024 * 1024
  if (file.size > maxSize) {
    ElMessage.error('文件大小不能超过5MB')
    return false
  }
  
  // 文件类型限制
  const allowedTypes = ['image/jpeg', 'image/png', 'image/gif']
  if (!allowedTypes.includes(file.raw.type)) {
    ElMessage.error('只支持 jpg/png/gif 格式的图片')
    return false
  }
  
  fileList.value = fileList
}

const handleFileRemove = (file, fileList) => {
  fileList.value = fileList
}
</script>

<style scoped>
.feedback-button {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 60px;
  height: 80px;
  background: #ffffff;
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  font-size: 12px;
  color: #666;
  user-select: none;
}

.feedback-button:hover {
  background: #f5f5f5;
  border-color: #409eff;
  color: #409eff;
  box-shadow: 0 4px 12px rgba(64, 158, 255, 0.15);
  transform: translateY(-1px);
}

.feedback-text {
  margin-top: 4px;
  font-size: 12px;
  line-height: 1;
}

.el-button.is-circle {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transition: all 0.3s ease;
}

.el-button.is-circle:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
}

.dialog-footer {
  text-align: right;
}

.el-upload__tip {
  color: #999;
  font-size: 12px;
}
</style>
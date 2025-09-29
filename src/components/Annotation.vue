<template>
  <div>
    <!-- 评论对话框 -->
    <el-dialog
      v-model="dialogVisible"
      title="评论"
      width="750px"
      :before-close="handleClose"
      append-to-body
    >
      <el-form
        ref="formRef"
        :model="formData"
        :rules="formRules"
        label-width="80px"
      >
        <el-form-item label="内容" prop="content">
          <el-input
            v-model="formData.content"
            type="textarea"
            :rows="4"
            placeholder="请详细描述您的问题或建议..."
            :maxlength="500"
            show-word-limit
            :autofocus="true"
          />
        </el-form-item>

        <el-form-item label="附件">
          <el-upload
            ref="uploadRef"
            :file-list="fileList"
            list-type="picture-card"
            :on-change="handleFileChange"
            :on-remove="handleFileRemove"
            :on-preview="handlePictureCardPreview"
            :auto-upload="false"
            :multiple="true"
          >
            <el-icon><Plus /></el-icon>
            <template #tip>
              <div class="el-upload__tip">
                点击或粘贴添加图片，单个文件不超过5MB，支持jpg/png/gif格式
              </div>
            </template>
          </el-upload>
          <el-dialog v-model="dialogVisiblePreview" width="80%" :append-to-body="true">
            <el-image style="width: 100%; height: 100%" :src="dialogImageUrlPreview" fit="fill" />
          </el-dialog>
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
            提交评论
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup vapor>
import { ref, reactive, onMounted, onBeforeUnmount, computed } from 'vue'
import { ElMessage } from 'element-plus'
import { bugNoteAdd } from '../api'

// Props 定义
const props = defineProps({
  dialogVisible: {
    type: Boolean,
    required: true,
    default: false
  },
  bugId: {
    type: Number,
    required: true,
    default: 0
  },
})

// Emits 定义
const emits = defineEmits(['success', 'failed', 'close'])

// 响应式数据
const dialogVisible = computed(() => props.dialogVisible)
const dialogVisiblePreview = ref(false)
const dialogImageUrlPreview = ref('')
const submitLoading = ref(false)
const formRef = ref()
const uploadRef = ref()
const fileList = ref([])
const formRules = {
  content: [
    { required: true, message: '评论内容不能为空', trigger: 'blur' },
    { min: 1, max: 500, message: '内容长度应在1到500字符之间', trigger: 'blur' }
  ]
}

// 表单数据
const formData = reactive({
  content: ''
})

// 方法
const uploadAnotation = async (submitData)=> {
  let bug_id = props.bugId;
  let bugnote_text = submitData.content;
  let binary_file = [];
  for (let i = 0; i < submitData.files.length; i++) {
    let file = submitData.files[i];
    const arrayBuffer = await file.raw.arrayBuffer();
    binary_file.push([file.name, Array.from(new Uint8Array(arrayBuffer))]);
  }
  bugNoteAdd({bug_id,bugnote_text,binary_file}).then(result => {
    ElMessage.success('评论提交成功！');
    emits('success');
  }).catch(error => {
    ElMessage.error('更新失败，请稍后重试' + error);
    emits('failed');
  });
}

const handleClose = () => {
  resetForm()
  emits('close')
}

const resetForm = () => {
  formData.content = ''
  fileList.value = []
  if (formRef.value) {
    formRef.value.clearValidate()
  }
}

const handleSubmit = async () => {
  if (!formRef.value) return
  
  try {
    const valid = await formRef.value.validate()
    if (!valid) return
    
    submitLoading.value = true
    
    // 准备提交数据
    const submitData = {
      ...formData,
      files: fileList.value,
      timestamp: new Date().toISOString()
    }
    
    // 触发提交事件
    await uploadAnotation(submitData)
    
  } catch (error) {
    console.error('表单验证失败:', error)
  } finally {
    submitLoading.value = false
  }
}

const handleFileChange = (file, newFileList) => {
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
    newFileList.splice(newFileList.indexOf(file), 1)
    return false
  }
  
  fileList.value = newFileList
}

const handleFileRemove = (file, newFileList) => {
  fileList.value = newFileList
}

const handlePictureCardPreview = (uploadFile) => {
  dialogImageUrlPreview.value = uploadFile.url
  dialogVisiblePreview.value = true
}

const handlePaste = (event) => {
  const items = event.clipboardData && event.clipboardData.items
  if (!items) return
  for (const item of items) {
    if (item.type.indexOf('image') !== -1) {
      const file = item.getAsFile()
      if (file) {
        // 构造 el-upload 需要的文件对象
        const uploadFile = {
          name: file.name || `pasted-image-${Date.now()}.png`,
          url: URL.createObjectURL(file),
          status: 'ready',
          size: file.size,
          percentage: 0,
          raw: file,
          uid: Date.now() + Math.random()
        }
        fileList.value.push(uploadFile)
      }
    }
  }
}

onMounted(() => {
  // 只在对话框打开时监听
  window.addEventListener('paste', handlePaste)
})
onBeforeUnmount(() => {
  window.removeEventListener('paste', handlePaste)
})
</script>

<style scoped>
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
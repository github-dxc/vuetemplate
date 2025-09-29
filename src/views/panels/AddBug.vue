<template>
  <div class="content-panel">
    <div class="panel-header">
      <div class="header-content">
        <h3>🐛 添加问题</h3>
      </div>
      <div class="header-right">
        <el-button text @click="handleSubmit" :loading="submitLoading">
          <el-icon size="20"><SuccessFilled /></el-icon>
        </el-button>
      </div>
    </div>
    <div class="panel-content">
      <div class="overlap" ref="contentRef">
        <el-form ref="formRef" :model="bugInfo" :rules="formRules" label-width="80px">

          <el-form-item prop="project_id" label="问题分配">
            <div class="option_tags">
              <!--项目-->
              <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
                <template #reference>
                  <el-button type="primary" plain round>{{ bugProject.get(String(bugInfo.project_id)) }}</el-button>
                </template>
                <div class="flex gap-2 mt-4">
                  <el-check-tag class="option_check_tag" v-for="item in bugProject"
                    :checked="item[0] === String(bugInfo.project_id)" type="primary"
                    @change="changeBug({ project_id: String(item[0]) })">{{ item[1] }}</el-check-tag>
                </div>
              </el-popover>

              <!--分组-->
              <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
                <template #reference>
                  <el-button type="success" plain round>{{ bugCategory.get(String(bugInfo.category_id)) }}</el-button>
                </template>
                <div class="flex gap-2 mt-4">
                  <el-check-tag class="option_check_tag" v-for="item in bugCategory"
                    :checked="item[0] === String(bugInfo.category_id)" type="success"
                    @change="changeBug({ category_id: Number(item[0]) })">{{ item[1] }}</el-check-tag>
                </div>
              </el-popover>

              <!--出现频率-->
              <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
                <template #reference>
                  <el-button type="warning" plain round>{{ bugReproducibility.get(String(bugInfo.reproducibility))
                    }}</el-button>
                </template>
                <div class="flex gap-2 mt-4">
                  <el-check-tag class="option_check_tag" v-for="item in bugReproducibility"
                    :checked="item[0] === String(bugInfo.reproducibility)" type="warning"
                    @change="changeBug({ reproducibility: Number(item[0]) })">{{ item[1] }}</el-check-tag>
                </div>
              </el-popover>

              <!--严重性-->
              <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
                <template #reference>
                  <el-button type="danger" plain round>{{ bugSeverity.get(String(bugInfo.severity)) }}</el-button>
                </template>
                <div class="flex gap-2 mt-4">
                  <el-check-tag class="option_check_tag" v-for="item in bugSeverity"
                    :checked="item[0] === String(bugInfo.severity)" type="danger"
                    @change="changeBug({ severity: Number(item[0]) })">{{ item[1] }}</el-check-tag>
                </div>
              </el-popover>

              <!--优先级-->
              <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
                <template #reference>
                  <el-button type="danger" plain round>{{ bugPriority.get(String(bugInfo.priority)) }}</el-button>
                </template>
                <div class="flex gap-2 mt-4">
                  <el-check-tag class="option_check_tag" v-for="item in bugPriority"
                    :checked="item[0] === String(bugInfo.priority)" type="danger"
                    @change="changeBug({ priority: Number(item[0]) })">{{ item[1] }}</el-check-tag>
                </div>
              </el-popover>

              <!--处理人-->
              <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
                <template #reference>
                  <el-button type="primary" plain round>{{ bugUsers.get(String(bugInfo.handler_id)) }}</el-button>
                </template>
                <div class="flex gap-2 mt-4">
                  <el-check-tag class="option_check_tag" v-for="item in bugUsers"
                    :checked="item[0] === String(bugInfo.handler_id)" type="primary"
                    @change="changeBug({ handler_id: Number(item[0]) })">{{ item[1] }}</el-check-tag>
                </div>
              </el-popover>
            </div>
          </el-form-item>

          <el-form-item prop="summary" label="问题摘要">
            <el-input v-model="bugInfo.summary" placeholder="请填写问题摘要" style="width: 750px;" />
          </el-form-item>

          <el-form-item prop="description" label="问题描述">
            <div class="text-content">
              <div class="editor-container">
                <QuillEditor ref="descriptionEditor" v-model:content="bugInfo.description" contentType="html"
                  :toolbar="toolbarOptions" />
              </div>
            </div>
          </el-form-item>

          <el-form-item prop="steps_to_reproduce" label="重现步骤">
            <div class="text-content">
              <div class="editor-container">
                <QuillEditor ref="stepsEditor" v-model:content="bugInfo.steps_to_reproduce" contentType="html"
                  :toolbar="toolbarOptions" />
              </div>
            </div>
          </el-form-item>

          <el-form-item label="附件">
            <el-upload ref="uploadRef" :file-list="fileList" list-type="picture-card" :on-change="handleFileChange"
              :on-remove="handleFileRemove" :on-preview="handlePictureCardPreview" :auto-upload="false"
              :multiple="true">
              <el-icon>
                <Plus />
              </el-icon>
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
      </div>
    </div>

  </div>
</template>

<script setup vapor>
import { ref, reactive, onMounted, onBeforeUnmount, computed } from 'vue'
import { QuillEditor, Quill } from '@vueup/vue-quill';
import '@vueup/vue-quill/dist/vue-quill.snow.css';
import { ElMessage } from 'element-plus'
import { bugReport } from '../../api'

// Props 定义
const props = defineProps({
  enums: {
    type: Object,
    required: true,
    default: () => ({})
  },
})

// 响应式数据
const dialogVisiblePreview = ref(false)
const dialogImageUrlPreview = ref('')
const submitLoading = ref(false)
const formRef = ref()
const uploadRef = ref()
const fileList = ref([])
const contentRef = ref(null);
const toolbarOptions = [
  [{ header: [1, 2, 3, false] }],
  [{ list: 'ordered' }, { list: 'bullet' }],
  ['bold', 'italic', 'underline', 'strike'],
  ['link', 'blockquote', 'code-block'],
];
const formRules = {
  project_id: [
    { required: true, message: '项目不能为空', trigger: 'blur' }
  ],
  category_id: [
    { required: true, message: '分组不能为空', trigger: 'blur' }
  ],
  summary: [
    { required: true, message: '摘要内容不能为空', trigger: 'blur' },
    { min: 1, max: 500, message: '内容长度应在1到500字符之间', trigger: 'blur' }
  ],
  description: [
    { required: true, message: '评论内容不能为空', trigger: 'blur' },
    { min: 1, max: 500, message: '内容长度应在1到500字符之间', trigger: 'blur' }
  ]
}

// 表单数据
const bugInfo = reactive({
  project_id: '0',
  category_id: 0,
  reproducibility: 0,
  severity: 0,
  priority: 0,
  handler_id: 0,
  summary: '',
  description: '',
  steps_to_reproduce: ''
})

const bugCategory = computed(() => {
  const myMap = new Map();
  props.enums.Category.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
    if (myMap.size === 1) {
      bugInfo.category_id = Number(item.key);
    }
  });
  return myMap;
})
const bugPriority = computed(() => {
  const myMap = new Map();
  props.enums.Priority.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
    if (myMap.size === 1) {
      bugInfo.priority = Number(item.key);
    }
  });
  return myMap;
})
const bugReproducibility = computed(() => {
  const myMap = new Map();
  props.enums.Reproducibility.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
    if (myMap.size === 1) {
      bugInfo.reproducibility = Number(item.key);
    }
  });
  return myMap;
})
const bugProject = computed(() => {
  const myMap = new Map();
  props.enums.Project.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
    if (myMap.size === 1) {
      bugInfo.project_id = String(item.key);
    }
  });
  return myMap;
})
const bugSeverity = computed(() => {
  const myMap = new Map();
  props.enums.Severity.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
    if (myMap.size === 1) {
      bugInfo.severity = Number(item.key);
    }
  });
  return myMap;
})
const bugUsers = computed(() => {
  const myMap = new Map();
  props.enums.Users.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
    if (myMap.size === 1) {
      bugInfo.handler_id = Number(item.key);
    }
  });
  return myMap;
})

// 方法
const changeBug = function (data) {
  Object.assign(bugInfo, data);
}

const uploadBug = async (submitData, files=[]) => {
  console.log('提交数据:', submitData);
  let binary_file = [];
  for (let i = 0; i < files.length; i++) {
    let file = files[i];
    const arrayBuffer = await file.raw.arrayBuffer();
    binary_file.push([file.name, Array.from(new Uint8Array(arrayBuffer))]);
  }
  bugReport({ ...submitData, binary_file }).then(result => {
    ElMessage.success('提交成功！');
  }).catch(error => {
    ElMessage.error('更新失败，请稍后重试' + error);
  });
}

const resetForm = () => {
  bugInfo.summary = ''
  bugInfo.description = ''
  bugInfo.steps_to_reproduce = ''
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

    // 触发提交事件
    await uploadBug(bugInfo, fileList.value)

  } catch (error) {
    console.error('表单验证失败:', error)
  } finally {
    submitLoading.value = false
    resetForm();
  }
}

const handleFileChange = (file, newFileList) => {
  console.log('文件变化:', file, newFileList);
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
    // 把file从newFileList中移除
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
  display: flex;
  justify-content: space-between;
  align-items: center;
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

.header-right {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  margin-right: 40px;
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

.panel-content {
  height: 85%;
  padding: 25px 20px;
}

.overlap {
  padding: 20px;
  background-color: #ffffff;
  border-color: #e5e7eb;
  border-radius: 8px;
  box-shadow:
    0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 1px 2px #0000000f, 0px 1px 3px #0000001a;
  position: relative;
  height: 95%;
  overflow: auto;
}

.option_tags {
  position: relative;
  height: 32px;
}

.option_tags>button {
  font-weight: bold;
}

.option_check_tag {
  position: relative;
  margin: 3px;
}

.option_text {
  position: relative;
  width: 780px;
  margin-top: 20px;
  margin-bottom: 20px;
  margin-left: 25px;
}

.text-content {
  margin-top: 10px;
  position: relative;
  width: 750px;
}

.text-field {
  color: #111827;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 16px;
  font-weight: 600;
  letter-spacing: 0;
  line-height: 24px;
  position: relative;
  white-space: nowrap;
}

.text-content .editor-container {
  position: relative;
}

.el-button.is-circle {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  transition: all 0.3s ease;
}

.el-button.is-circle:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
}

.el-upload__tip {
  color: #999;
  font-size: 12px;
}
</style>
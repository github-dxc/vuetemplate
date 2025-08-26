<template>
  <div class="generated-design">
    <div class="overlap-group">
      <div class="overlap" ref="contentRef">
        <!--展示提交员信息-->
        <div class="option_reporter">
          <div class="rectangle" :style="{backgroundColor: getColorByUnicPalette(getFirstChar(bugInfo.reporter)).textColor}"/>
          <div class="frame">
            <div class="text-wrapper-0">{{ getFirstChar(bugInfo.reporter) }}</div>
          </div>
          <div class="rectangle-2" />
          <div class="text-wrapper">{{ bugInfo.reporter }}</div>
          <div class="text-wrapper-3">报告员</div>
          <div class="rectangle-3" />
          <div class="vector-wrapper">
            <el-icon >
              <Briefcase />
            </el-icon>
          </div>
          <div class="text-wrapper-4">{{ bugInfo.project }}</div>
          <div class="text-wrapper-5">提交时间</div>
          <div class="text-wrapper-6">{{ formatDate(bugInfo.date_submitted) }}</div>
        </div>
        <!--展示选项类状态-->
        <div class="option_tags">
          <!--解决情况-->
          <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
            <template #reference>
              <el-button type="danger" plain round>{{ bugSeverity.get(String(bugInfo.severity)) }}</el-button>
            </template>
            <div class="flex gap-2 mt-4">
              <el-check-tag class="option_check_tag" v-for="item in bugSeverity" :checked="item[0]===String(bugInfo.severity)" type="danger" @change="changeBug({severity:Number(item[0])})">{{ item[1] }}</el-check-tag>
            </div>
          </el-popover>
          
          <!--解决情况-->
          <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
            <template #reference>
              <el-button type="warning" plain round>{{ bugStatus.get(String(bugInfo.status)) }}</el-button>
            </template>
            <div class="flex gap-2 mt-4">
              <el-check-tag class="option_check_tag" v-for="item in bugStatus" :checked="item[0]===String(bugInfo.status)" type="warning" @change="changeBug({status:Number(item[0])})">{{ item[1] }}</el-check-tag>
            </div>
          </el-popover>
          
          <!--处理情况-->
          <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
            <template #reference>
              <el-button type="primary" plain round>{{ bugResolution.get(String(bugInfo.resolution)) }}</el-button>
            </template>
            <div class="flex gap-2 mt-4">
              <el-check-tag class="option_check_tag" v-for="item in bugResolution" :checked="item[0]===String(bugInfo.resolution)" type="primary" @change="changeBug({resolution:Number(item[0])})">{{ item[1] }}</el-check-tag>
            </div>
          </el-popover>
          
          <!--分组-->
          <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
            <template #reference>
              <el-button type="success" plain round>{{ bugCategory.get(String(bugInfo.category_id)) }}</el-button>
            </template>
            <div class="flex gap-2 mt-4">
              <el-check-tag class="option_check_tag" v-for="item in bugCategory" :checked="item[0]===String(bugInfo.category_id)" type="success" @change="changeBug({category_id:Number(item[0])})">{{ item[1] }}</el-check-tag>
            </div>
          </el-popover>
          
          <!--处理人-->
          <el-popover placement="bottom-start" :width="400" trigger="hover" :hide-after="10">
            <template #reference>
              <el-button type="info" plain round>{{ bugUsers.get(String(bugInfo.handler_id)) }}</el-button>
            </template>
            <div class="flex gap-2 mt-4">
              <el-check-tag class="option_check_tag" v-for="item in bugUsers" :checked="item[0]===String(bugInfo.handler_id)" type="primary" @change="changeBug({handler_id:Number(item[0])})">{{ item[1] }}</el-check-tag>
            </div>
          </el-popover>
          
        </div>
        <!--展示描述和重现步骤-->
        <div class="option_text">
          <div class="text-field">问题描述</div>
          <div class="text-content">
            <div class="text" v-if="!showDescriptionEdit" v-html="bugInfo.description||'双击此处添加描述'" @dblclick="focusDescriptionEditor"></div>
            <div v-else class="editor-container">
              <QuillEditor
                ref="descriptionEditor"
                v-model:content="bugInfo.description"
                contentType="html"
                :toolbar="toolbarOptions"
                @ready="onEditorReady"
              />
            </div>
          </div>
          
        </div>
        <div class="option_text">
          <div class="text-field">重现步骤</div>
          <div class="text-content">
            <div class="text" v-if="!showStepsEdit" v-html="bugInfo.steps_to_reproduce||'双击此处添加重现步骤'" @dblclick="focusStepsEditor"></div>
            <div v-else class="editor-container">
              <QuillEditor
                ref="stepsEditor"
                v-model:content="bugInfo.steps_to_reproduce"
                contentType="html"
                :toolbar="toolbarOptions"
                @ready="onEditorReady"
              />
            </div>
          </div>
        </div>
        <!--最后更新时间-->
        <div class="option_last_updated">
          <div class="frame-2">
            <el-icon class="vector-3">
              <Edit />
            </el-icon>
          </div>
          <div class="text-wrapper-15">最后更新: {{ formatDate(bugInfo.last_updated) }}</div>
        </div>
      </div>

      <!--评论区/时间线-->
      <div class="overlap-comment">
        <div class="overlap-9">
          <div class="frame-3">
            <el-icon class="vector-4" :size="20">
              <Comment />
            </el-icon>
          </div>
          <div class="text-wrapper-comment">评论 ({{ bugNotes.length }})</div>
        </div>
        <div v-for="bugNote in bugNotes" class="overlap-comment-detail">
          <div class="overlap-background" :style="{backgroundColor: getColorByUnicPalette(getFirstChar(bugNote.handler)).backgroundColor}">
            <div class="text-wrapper-surname" :style="{color: getColorByUnicPalette(getFirstChar(bugNote.handler)).textColor}">{{ getFirstChar(bugNote.handler) }}</div>
          </div>
          <div class="text-wrapper-header">
            <div class="text-wrapper-name">{{ bugNote.handler }}</div>
            <div class="text-wrapper-time">{{ formatDate(bugNote.time) }}</div>
          </div>
          <div class="text-wrapper-content">
            <div v-html="bugNote.text"></div>
          </div>
          
          <div v-for="(img, index) in bugNote.attachments" class="demo-image__error">
            <div class="block">
              <span class="demonstration">{{ img.name }}</span>
              <el-image :src="img.url_base64" @click="openImagePreview(index)" fit="contain"/>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount } from 'vue';
import { QuillEditor,Quill } from '@vueup/vue-quill';
import '@vueup/vue-quill/dist/vue-quill.snow.css';
import { formatDate, getFirstChar, getColorByUnicPalette, byteArrayToBase64Image } from '../util';
import { apiBugInfo, bugNoteAdd, imageBase64, updateBug } from '../api';
import { ElMessage } from "element-plus";
import { createNewWindow } from '../windows';
import { emit } from '@tauri-apps/api/event';

const props = defineProps({
  bugId: {
    type: Number,
    required: true,
    default: 0
  },
  enums: {
    type: Object,
    required: true,
    default: () => ({})
  }
}); 

const emits = defineEmits(['alabelClickFn']);

const contentRef = ref(null);
const toolbarOptions = [
  [{ header: [1, 2, 3, false] }],
  [{ list: 'ordered' }, { list: 'bullet' }],
  ['bold', 'italic', 'underline'],
  ['link', 'blockquote', 'code-block', 'image', 'video'],
  ['save']  // 添加保存按钮
];
const descriptionEditor = ref(null);
const stepsEditor = ref(null);
const showDescriptionEdit = ref(false);
const showStepsEdit = ref(false);
const focusDescriptionEditor = (e) => {
  if (e.target.tagName.toLowerCase() === 'a') {
    return;
  }
  showDescriptionEdit.value=!showDescriptionEdit.value;
  setTimeout(()=>{
    if (descriptionEditor.value) {
      let q = descriptionEditor.value.getQuill();
      q.focus();
      const length = q.getLength();
      q.setSelection(length, 0);
    }
  },200)
};
const focusStepsEditor = (e) => {
  if (e.target.tagName.toLowerCase() === 'a') {
    return;
  }
  showStepsEdit.value=!showStepsEdit.value;
  setTimeout(()=>{
    if (stepsEditor.value) {
      let q = stepsEditor.value.getQuill();
      q.focus();
      const length = q.getLength();
      q.setSelection(length, 0);
    }
  },200)
};
const onEditorReady = (quill) => {
  const toolbar = quill.getModule('toolbar');
  toolbar.addHandler('save', () => {
    // 获取当前编辑器实例
    const currentEditor = quill;
    // 获取当前编辑器的内容
    const content = currentEditor.root.innerHTML;
    
    // 判断是哪个编辑器并保存相应内容
    if (currentEditor === descriptionEditor.value?.getQuill()) {
      changeBug({ description: content });
      showDescriptionEdit.value = false;
    } else if (currentEditor === stepsEditor.value?.getQuill()) {
      changeBug({ steps_to_reproduce: content });
      showStepsEdit.value = false;
    }
  });
};

const bugInfo = ref({});
const oldBugInfo = ref({});
const bugNotes = ref({});

const bugProject = computed(() => {
  const myMap = new Map();
  props.enums.Project.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
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
  });
  return myMap;
})
const bugSeverity = computed(() => {
  const myMap = new Map();
  props.enums.Severity.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
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
  });
  return myMap;
})
const bugViewStatus = computed(() => {
  const myMap = new Map();
  props.enums.ViewStatus.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
  });
  return myMap;
})
const bugResolution = computed(() => {
  const myMap = new Map();
  props.enums.Resolution.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
  });
  return myMap;
})
const bugStatus = computed(() => {
  const myMap = new Map();
  props.enums.Status.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
    }
  });
  return myMap;
})
const bugCategory = computed(() => {
  const myMap = new Map();
  props.enums.Category.forEach(item => {
    if (item.key !== "0") {
      myMap.set(item.key, item.value);
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
  });
  return myMap;
})

const addBugNote = async function() {
  let bug_id = props.bugId;
  let bugnote_text = "dxcdxc";
  let file_path = "D:/15520/Pictures/wechat_2025-08-21_214419_925.png";
  bugNoteAdd({bug_id,bugnote_text,file_path}).then(result => {
    console.log("成功:", result);
    getBugInfo();
  }).catch(error => {
    ElMessage({
      message: '更新失败，请稍后重试',
      type: 'error',
    });
    console.error("错误:", error);
  });
}

const changeBug = function(data) {
  console.log("更新数据:", data);
  // 如果data中的数据与oldBugInfo中的数据相同，则不进行更新
  let isChanged = false;
  for (let key in data) {
    if (data[key] !== oldBugInfo.value[key]) {
      isChanged = true;
      break;
    }
  }
  console.log("旧数据:", oldBugInfo.value);
  if (!isChanged) {
    console.log("数据未改变，跳过更新");
    return;
  }
  if (!bugInfo.last_updated_sec) {
    ElMessage({
      message: '无法更新，可能是因为没有权限，请刷新后重试',
      type: 'warning',
    });
    return;
  }
  data.bug_id = props.bugId;
  updateBug(data).then(result => {
    console.log("更新成功:", result);
    getBugInfo();
  }).catch(error => {
    ElMessage({
      message: '更新失败，请稍后重试',
      type: 'error',
    });
    console.error("更新失败:", error);
  });
}

// 打开图片预览
const openImagePreview = async function(index) {
  const DOMContentLoadedCallback = () => {
    // 发送图片信息列表给图片预览窗口
    emit('web_images', { bugnote_notes: bugInfo.value.bugnote_notes, show_index: index});
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
  },DOMContentLoadedCallback);
}

const getBugInfo = function() {
  apiBugInfo(props.bugId).then(result => {
    console.log("成功:", result);
    bugInfo.value = result;
    oldBugInfo.value = {...result};
    const allNotes = result.bugnote_notes || [];
    if (result.attachments.length > 0) {
      allNotes.push({
        handler: result.reporter,
        handler_id: result.reporter_id,
        time: result.date_submitted,
        attachments: result.attachments,
      });
    }
    bugNotes.value = allNotes;
    bugNotes.value.forEach((i) => {
      i.attachments.forEach((item) => {
        imageBase64(item.url).then((bytes) => {
          if (bytes) {
            item.url_base64 = byteArrayToBase64Image(bytes, item.name);
          }
        });
      });
    });
  }).catch(error => {
    console.error("错误:", error);
  });
}

onMounted(() => {
  getBugInfo();

  // 监听内容区域的a标签点击事件，传递到外面用外部浏览器打开
  const clickHandler = (e) => {
    const a = e.target.closest('a');
    //如果a标签是editor-container的子元素且不是.ql-preview，则不处理
    if (a && e.target.closest('.editor-container') && !e.target.closest('.ql-preview')) {
      return;
    }
    if (a) {
      e.preventDefault(); // 阻止默认行为
      emits('alabelClickFn', a.href);
    }
  };
  contentRef.value.addEventListener('click', clickHandler);
  onBeforeUnmount(() => {
    contentRef.value?.removeEventListener('click', clickHandler);
  });

  
  // 注册富文本自定义格式
  const Inline = Quill.import('blots/inline');
  class SaveBlot extends Inline {}
  SaveBlot.blotName = 'save';
  SaveBlot.tagName = 'span';
  Quill.register(SaveBlot);
  // 注册富文本自定义图标
  const icons = Quill.import('ui/icons');
  icons['save'] = `<svg viewBox="0 0 24 24" width="18" height="18">
    <path fill="currentColor" d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/>
  </svg>`;

})

</script>

<style>
.ql-tooltip.ql-editing {
  left: 20px !important;
}
</style>

<style scoped>
.generated-design {
  align-items: start;
  background-color: #ffffff;
  display: grid;
  justify-items: center;
}

.generated-design .overlap-group {
  background-color: #ffffff;
  height: 100%;
  position: relative;
  width: 100%;
}

.generated-design .overlap {
  background-color: #ffffff;
  border: 1px solid;
  border-color: #e5e7eb;
  border-radius: 8px;
  box-shadow:
    0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 1px 2px #0000000f, 0px 1px 3px #0000001a;
  left: 10px;
  right: 10px;
  position: relative;
  width: 780px;
}

.generated-design .option_reporter {
  height: 52px;
  margin-left: 25px;
  position: relative;
  margin-top: 30px;
  width: 760px;
}

.generated-design .rectangle {
  border-radius: 33554400px;
  box-shadow:
    0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 2px 8px #3b82f64c;
  height: 48px;
  left: 0;
  position: absolute;
  top: 0;
  width: 48px;
}

.generated-design .frame {
  height: 24px;
  left: 12px;
  position: absolute;
  top: 12px;
  width: 24px;
}

.generated-design .vector {
  height: 18px;
  left: 4px;
  position: absolute;
  top: 4px;
  width: 17px;
}

.generated-design .rectangle-2 {
  background-color: #10b981;
  border: 2px solid;
  border-color: #ffffff;
  border-radius: 33554400px;
  height: 16px;
  left: 32px;
  position: absolute;
  top: 32px;
  width: 16px;
}

.generated-design .text-wrapper-0 {
  color: #ffffff;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 18px;
  font-weight: 600;
  letter-spacing: 0;
  line-height: 28px;
  margin-left: 4px;
  position: absolute;
  white-space: nowrap;
}

.generated-design .text-wrapper {
  color: #111827;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 18px;
  font-weight: 600;
  left: 58px;
  letter-spacing: 0;
  line-height: 28px;
  position: absolute;
  top: 2px;
  white-space: nowrap;
}

.generated-design .text-wrapper-3 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 58px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 34px;
  white-space: nowrap;
}

.generated-design .rectangle-3 {
  background-color: #d1d5db;
  border-radius: 33554400px;
  height: 4px;
  left: 108px;
  position: absolute;
  top: 43px;
  width: 4px;
}

.generated-design .vector-wrapper {
  color: #6b7280;
  height: 12px;
  left: 120px;
  position: absolute;
  top: 35px;
  width: 12px;
}

.generated-design .text-wrapper-4 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 138px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 35px;
  white-space: nowrap;
}

.generated-design .text-wrapper-5 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 670px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  text-align: right;
  top: 2px;
  white-space: nowrap;
}

.generated-design .text-wrapper-6 {
  color: #111827;
  font-family: "Inter-Medium", Helvetica;
  font-size: 16px;
  font-weight: 500;
  left: 600px;
  letter-spacing: 0;
  line-height: 24px;
  position: absolute;
  text-align: right;
  top: 34px;
  white-space: nowrap;
}

.option_tags {
  position: relative;
  width: 780px;
  height: 32px;
  margin-top: 20px;
  margin-left: 25px;
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

.generated-design .text-field {
  color: #111827;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 16px;
  font-weight: 600;
  letter-spacing: 0;
  line-height: 24px;
  position: relative;
  white-space: nowrap;
}

.generated-design .text-content {
  margin-top: 10px;
  margin-left: 5px;
  position: relative;
  width: 750px;
}

.generated-design .text-content .text {
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 13px;
  font-weight: 400;
  letter-spacing: 0;
  line-height: 26px;
}

:deep(.text-content .text p) {
  margin: 0px;
}

.generated-design .text-content .editor-container {
  position: relative;
  left: -15px;
}

.generated-design .option_last_updated {
  border-color: #e5e7eb;
  border-top-style: solid;
  border-top-width: 1px;
  height: 56px;
  margin-left: 25px;
  position: relative;
  margin-top: 10px;
  width: 720px;
}

.generated-design .frame-2 {
  height: 16px;
  left: -1px;
  position: relative;
  top: 18px;
  width: 16px;
}

.generated-design .vector-3 {
  height: 15px;
  left: 2px;
  position: relative;
  bottom: 3px;
  width: 15px;
}

.generated-design .text-wrapper-15 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 23px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 14px;
  white-space: nowrap;
}

.generated-design .overlap-comment {
  background-color: #ffffff;
  border: 1px solid;
  border-color: #e5e7eb;
  border-radius: 8px;
  box-shadow:
    0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 1px 2px #0000000f, 0px 1px 3px #0000001a;
  left: 10px;
  right: 10px;
  position: relative;
  top: 15px;
  width: 780px;
  margin-bottom: 10px;
}

.generated-design .overlap-9 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #e5e7eb;
  height: 65px;
  left: 1px;
  position: relative;
  top: 1px;
  width: 780px;
}

.generated-design .frame-3 {
  height: 20px;
  left: 23px;
  position: relative;
  top: 22px;
  width: 20px;
}

.generated-design .vector-4 {
  height: 20px;
  left: 2px;
  position: relative;
  bottom: 2px;
  width: 20px;
}

.generated-design .text-wrapper-comment {
  color: #111827;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 18px;
  font-weight: 600;
  left: 55px;
  letter-spacing: 0;
  line-height: 27px;
  position: absolute;
  top: 17px;
  white-space: nowrap;
}

.generated-design .overlap-comment-detail {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #f3f4f6;
  left: 1px;
  position: relative;
  top: 0px;
  width: 700px;
}

.generated-design .overlap-background {
  background-color: #eff6ff;
  border-radius: 33554400px;
  height: 32px;
  left: 24px;
  position: relative;
  top: 24px;
  width: 32px;
}

.generated-design .text-wrapper-surname {
  color: #2563eb;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 14px;
  font-weight: 600;
  left: 9px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 6px;
  white-space: nowrap;
}
.text-wrapper-header {
  position: relative;
  display: flex;
  align-items: center;
  top: 0px;
}

.generated-design .text-wrapper-name {
  color: #111827;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 67px;
  letter-spacing: 0;
  line-height: 21px;
  position: relative;
  white-space: nowrap;
}

.generated-design .text-wrapper-time {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 12px;
  font-weight: 400;
  margin-left: 8px;
  left: 66px;
  letter-spacing: 0;
  line-height: 18px;
  position: relative;
  white-space: nowrap;
}

.generated-design .text-wrapper-content {
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 67px;
  letter-spacing: 0;
  line-height: 22.8px;
  position: relative;
  margin-top: 15px;
  margin-bottom: 20px;
  white-space: nowrap;
}

.demo-image__error {
  position: relative;
  height: 200px;
  margin-top: -20px;
  margin-left: 20px;
  margin-bottom: 10px;
}

.demo-image__error .block {
  text-align: center;
  display: inline-block;
  width: 49%;
  box-sizing: border-box;
  vertical-align: top;
}
.demo-image__error .demonstration {
  display: block;
  color: var(--el-text-color-secondary);
  font-size: 14px;
}
.demo-image__error .el-image {
  padding: 0 5px;
  max-width: 300px;
  max-height: 170px;
  width: 100%;
  height: 170px;
}
</style>
<template>
  <div class="generated-design">
    <div class="overlap-group">
      <div class="overlap">
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
          <div class="text-wrapper-4">活跃</div>
          <div class="text-wrapper-5">提交时间</div>
          <div class="text-wrapper-6">{{ formatDate(bugInfo.date_submitted) }}</div>
        </div>
        <!--展示选项类状态-->
        <div class="option_tags">
          <el-button type="primary" plain round>高优先级</el-button>
          <el-button type="success" plain round>UI问题</el-button>
          <el-button type="info" plain round>UI问题</el-button>
          <el-button type="warning" plain round>Warning</el-button>
          <el-button type="danger" plain round>Danger</el-button>
        </div>
        <!--展示描述和重现步骤-->
        <div class="option_text">
          <div class="text-field">问题描述</div>
          <div class="text-content">
            <div v-html="bugInfo.description"></div>
          </div>
        </div>
        <div class="option_text">
          <div class="text-field">重现步骤</div>
          <div class="text-content">
            <div v-html="bugInfo.steps_to_reproduce"></div>
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
          <div class="text-wrapper-comment">评论 (5)</div>
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
              <el-image :src="img.url_base64" @click="openImagePreview(index)"/>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { formatDate, getFirstChar, getColorByUnicPalette, byteArrayToBase64Image } from '../util';
import { apiBugInfo, imageBase64 } from '../api';
import { createNewWindow } from '../windows';
import { emit } from '@tauri-apps/api/event';

const props = defineProps({
  bugId: {
    type: Number,
    required: true,
    default: 0
  }
});

const bugInfo = ref({});
const bugNotes = ref({});

// 打开图片预览
const openImagePreview = async function(index) {
  const DOMContentLoadedCallback = () => {
    // 发送图片信息列表给图片预览窗口
    emit('web_images', { attachments: bugInfo.value.attachments, bugnote_notes: bugInfo.value.bugnote_notes, show_index: index});
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

onMounted(() => {
  try {
    apiBugInfo(props.bugId).then(result => {
      console.log("成功:", result);
      bugInfo.value = result;
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
  } catch (error) {
    console.error('下载图片失败:', error);
    return;
  }
})

</script>

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
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  margin-top: 10px;
  margin-left: 5px;
  letter-spacing: 0;
  line-height: 26px;
  position: relative;
  width: 750px;
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
  top: 20px;
  width: 780px;
  margin-bottom: 10px;
}

.generated-design .overlap-9 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #e5e7eb;
  height: 76px;
  left: 1px;
  position: relative;
  top: 1px;
  width: 780px;
}

.generated-design .frame-3 {
  height: 20px;
  left: 23px;
  position: relative;
  top: 27px;
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
  top: 22px;
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
  top: 5px;
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
  height: 232px;
  margin-top: -10px;
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
  max-height: 200px;
  width: 100%;
  height: 200px;
}
</style>
<template>
  <div class="transparent-background">
    <el-image-viewer v-if="showPreview" :url-list="srcList" show-progress :close-on-press-escape="true" :initial-index="imageInitIndex"
      @close="closeHandler">
      <template #progress="{ activeIndex, total }">
        <span>{{ activeIndex + 1 + ' - ' + total }}</span>
        <div class="image-nodes" v-if="imageNotes[activeIndex]">【注释】：{{ imageNotes[activeIndex] }}</div>
      </template>
      <template #toolbar="{ actions, prev, next, reset, activeIndex, setActiveItem }">
        <div class="image-names">{{ imageNames[activeIndex] }}</div>
        <el-icon @click="changeWindowSize">
          <FullScreen />
        </el-icon>
        <el-icon @click="prev">
          <Back />
        </el-icon>
        <el-icon @click="next">
          <Right />
        </el-icon>
        <el-icon @click="actions('zoomOut')" v-if="!isMinimized">
          <ZoomOut />
        </el-icon>
        <el-icon @click="actions('zoomIn', { enableTransition: false, zoomRate: 2 })" v-if="!isMinimized">
          <ZoomIn />
        </el-icon>
        <el-icon @click="actions('clockwise', { rotateDeg: 180, enableTransition: false })" v-if="!isMinimized">
          <RefreshRight />
        </el-icon>
        <el-icon @click="actions('anticlockwise')" v-if="!isMinimized">
          <RefreshLeft />
        </el-icon>
        <el-icon @click="reset">
          <Refresh />
        </el-icon>
        <el-icon @click="download(activeIndex)">
          <Download />
        </el-icon>
      </template>
    </el-image-viewer>
  </div>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue'
import { ElIcon } from 'element-plus'
import {
  Back,
  Download,
  Refresh,
  RefreshLeft,
  RefreshRight,
  Right,
  ZoomIn,
  ZoomOut,
  FullScreen,
} from '@element-plus/icons-vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { listen } from '@tauri-apps/api/event';
import { imageBase64 } from '../api';
import { byteArrayToBase64Image } from '../util';
import { changeSize } from '../windows';

document.addEventListener('DOMContentLoaded', () => {
  // 获取当前窗口的实例
  const appWindow = WebviewWindow.getCurrent();
  // 在 DOM 加载完成后显示窗口
  appWindow.show();
  appWindow.emit('DOMContentLoaded', { message: 'DOM fully loaded and parsed' });
});

const srcList = ref([])
const imageNotes = ref([]);
const imageNames = ref([]);
const imageInitIndex = ref(0);
const showPreview = ref(true)
const isMinimized = ref(false)

const closeHandler = () => {
  showPreview.value = false
  const appWindow = WebviewWindow.getCurrent();
  appWindow.close();
}

const download = (index: number) => {
  const url = srcList.value[index]
  const filename = imageNames.value[index]

  // 检查是否为 base64 图片
  if (url.startsWith('data:')) {
    // 处理 base64 图片
    const link = document.createElement('a')
    link.href = url
    link.download = filename
    document.body.appendChild(link)
    link.click()
    link.remove()
  } else {
    // 处理普通 URL
    fetch(url)
      .then((response) => response.blob())
      .then((blob) => {
        const blobUrl = URL.createObjectURL(new Blob([blob]))
        const link = document.createElement('a')
        link.href = blobUrl
        link.download = filename
        document.body.appendChild(link)
        link.click()
        URL.revokeObjectURL(blobUrl)
        link.remove()
      })
      .catch((error) => {
        console.error('下载失败:', error)
      })
  }
}

const changeWindowSize = () => {
  isMinimized.value = !isMinimized.value;
  // appWindow.setSize({ width: 400, height: 300 });
  if (isMinimized.value) {
    changeSize({label:'image', width:400, hight:300, onTop:true});
  }else {
    changeSize({label:'image', width:1600, hight:900, center:true});
  }
}

// 监听展示的图片列表
listen('web_images', async (event) => {
  console.log('web_images:', event.payload)
  const payloadValue = event.payload;
  srcList.value = [];
  imageNames.value = [];
  imageNotes.value = [];
  imageInitIndex.value = payloadValue.show_index || 0;
  try {
    const count1 = payloadValue.attachments?.length || 0;
    for (let i = 0; i < count1; i++) {
      const item = payloadValue.attachments[i];
      const bytes = await imageBase64(item.url);
      if (bytes) {
        srcList.value.push(byteArrayToBase64Image(bytes, item.name));
        imageNotes.value.push("");
        imageNames.value.push(item.name);
      }
    }
    const count2 = payloadValue.bugnote_notes?.length || 0;
    for (let i = 0; i < payloadValue.bugnote_notes.length; i++) {
      const note = payloadValue.bugnote_notes[i];
      for (let j = 0; j < note.attachments.length; j++) {
        const item = note.attachments[j];
        const bytes = await imageBase64(item.url);
        if (bytes) {
          srcList.value.push(byteArrayToBase64Image(bytes, item.name));
          imageNotes.value.push(note.text);
          imageNames.value.push(item.name);
        }
      }
    }
  } catch (error) {
    console.error('下载图片失败:', error);
    return;
  }
})

onMounted(() => {
})

</script>

<style scoped>
body {
  background-color: transparent;
}

.transparent-background {
  /* 设置背景为透明 */
  background-color: transparent;
}

.image-nodes {
  margin-left: 5px;
}

.image-names {
  font-size: medium;
}

/* 添加半透明凸显文字的弧边背景，类似小米图标 */
::v-deep(.el-image-viewer__progress) {
  background: rgba(66, 66, 66, 0.8);
  border-radius: 20px;
  padding: 5px;
  backdrop-filter: blur(5px);
}

::v-deep(.el-image-viewer__close) {
  display: none;
}
</style>
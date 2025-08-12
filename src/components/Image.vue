<template>
  <div class="transparent-background">
    <el-image-viewer v-if="showPreview" :url-list="srcList" show-progress :close-on-press-escape="true"
      @close="closeHandler">
      <template #progress="{ activeIndex, total }">
        <span>{{ activeIndex + 1 + ' - ' + total }}</span>
        <div class="image-nodes" v-if="imageNotes[activeIndex]">【描述】：{{ imageNotes[activeIndex] }}</div>
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

const srcList = ref([
  // 'https://fuss10.elemecdn.com/a/3f/3302e58f9a181d2509f3dc0fa68b0jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/1/34/19aa98b1fcb2781c4fba33d850549jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/0/6f/e35ff375812e6b0020b6b4e8f9583jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/9/bb/e27858e973f5d7d3904835f46abbdjpeg.jpeg',
  // 'https://fuss10.elemecdn.com/d/e6/c4d93a3805b3ce3f323f7974e6f78jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/3/28/bbf893f792f03a54408b3b7a7ebf0jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/2/11/6535bcfb26e4c79b48ddde44f4b6fjpeg.jpeg',
])
const imageNotes = ref([]);
const imageNames = ref([]);
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
    changeSize('image', 400, 300, true);
  }else {
    changeSize('image', 1600, 900, false);
  }
}

// 监听展示的图片列表
listen('web_images', (event) => {
  console.log('web_images:', event.payload)
  const payloadValue = event.payload;
  imageNotes.value = [];
  try {
    payloadValue.attachments.forEach((item) => {
      imageBase64(item.url).then((bytes) => {
        if (bytes) {
          srcList.value.push(byteArrayToBase64Image(bytes, item.name));
          imageNotes.value.push("");
          imageNames.value.push(item.name);
        }
      });
    });
    payloadValue.bugnote_notes.forEach((i) => {
      i.attachments.forEach((item) => {
        imageBase64(item.url).then((bytes) => {
          if (bytes) {
            srcList.value.push(byteArrayToBase64Image(bytes, item.name));
            imageNotes.value.push(i.text);
            imageNames.value.push(item.name);
          }
        });
      });
    });
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
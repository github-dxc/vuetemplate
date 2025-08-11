<template>
  <div class="transparent-background">
    <el-image-viewer v-if="showPreview" :url-list="srcList" show-progress :initial-index="4" :close-on-press-escape="true"
      @close="closeHandler">
      <template #toolbar="{ actions, prev, next, reset, activeIndex, setActiveItem }">
        {{ payloadValue }}
        <el-icon @click="prev">
          <Back />
        </el-icon>
        <el-icon @click="next">
          <Right />
        </el-icon>
        <el-icon @click="setActiveItem(srcList.length - 1)">
          <DArrowRight />
        </el-icon>
        <el-icon @click="actions('zoomOut')">
          <ZoomOut />
        </el-icon>
        <el-icon @click="actions('zoomIn', { enableTransition: false, zoomRate: 2 })">
          <ZoomIn />
        </el-icon>
        <el-icon @click="
          actions('clockwise', { rotateDeg: 180, enableTransition: false })
          ">
          <RefreshRight />
        </el-icon>
        <el-icon @click="actions('anticlockwise')">
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
  DArrowRight,
  Download,
  Refresh,
  RefreshLeft,
  RefreshRight,
  Right,
  ZoomIn,
  ZoomOut,
} from '@element-plus/icons-vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { listen } from '@tauri-apps/api/event';
import { imageBase64 } from '../api';
import { byteArrayToBase64Image } from '../util';

document.addEventListener('DOMContentLoaded', () => {
  // 获取当前窗口的实例
  const appWindow = WebviewWindow.getCurrent();
  // 在 DOM 加载完成后显示窗口
  appWindow.show();
  appWindow.emit('DOMContentLoaded', { message: 'DOM fully loaded and parsed' });
});

const srcList = ref([
  'https://fuss10.elemecdn.com/a/3f/3302e58f9a181d2509f3dc0fa68b0jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/1/34/19aa98b1fcb2781c4fba33d850549jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/0/6f/e35ff375812e6b0020b6b4e8f9583jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/9/bb/e27858e973f5d7d3904835f46abbdjpeg.jpeg',
  // 'https://fuss10.elemecdn.com/d/e6/c4d93a3805b3ce3f323f7974e6f78jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/3/28/bbf893f792f03a54408b3b7a7ebf0jpeg.jpeg',
  // 'https://fuss10.elemecdn.com/2/11/6535bcfb26e4c79b48ddde44f4b6fjpeg.jpeg',
])

const showPreview = ref(true)

const closeHandler = () => {
  showPreview.value = false
  const appWindow = WebviewWindow.getCurrent();
  appWindow.close();
}

const download = (index: number) => {
  const url = srcList.value[index]
  const suffix = url.slice(url.lastIndexOf('.'))
  const filename = Date.now() + suffix

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
}

onMounted(() => {
})

const payloadValue = ref([]);
// 监听展示的图片列表
listen('web_images', (event) => {
  console.log('web_images:', event.payload)
  payloadValue.value = event.payload;
  try {
    payloadValue.value.attachments.forEach((item) => {
      imageBase64(item.url).then((bytes) => {
        srcList.value.push(byteArrayToBase64Image(bytes, item.name));
      });
    });
    payloadValue.value.bugnote_notes.forEach((item) => {
      item.attachments.forEach((item) => {
        imageBase64(item.url).then((bytes) => {
          srcList.value.push(byteArrayToBase64Image(bytes, item.name));
        });
      });
    });
  } catch (error) {
    console.error('下载图片失败:', error);
    return;
  }
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
</style>
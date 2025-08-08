<template>
  <el-container>
    <el-main class="list-container">
      <el-card>
        <div v-for="(item, index) in paginatedData" :key="index" class="list-item">
          <div class="item-content">
            <el-row :gutter="20">
              <el-col :span="18">
                <div class="item-meta">
                  <el-avatar :size="48" :src="item.avatar" />
                  <div class="meta-content">
                    <h4 class="item-title">{{ item.title }}</h4>
                    <p class="item-description">{{ item.description }}</p>
                  </div>
                </div>
                <div class="item-actions">
                  <span class="action-item"><el-icon><i class="el-icon-heart"></i></el-icon>83</span>
                  <span class="action-item"><el-icon><i class="el-icon-star-on"></i></el-icon>{{ item.index }}</span>
                  <span class="action-item"><el-icon><i class="el-icon-message"></i></el-icon>Reply</span>
                </div>
              </el-col>
              <el-col :span="6">
                <div class="image-area">
                  <img :src="item.imageSrc" alt="item image" />
                </div>
              </el-col>
            </el-row>
          </div>
        </div>
        <div class="pagination-container">
          <el-pagination
            background
            layout="prev, pager, next"
            :total="dataSource.length"
            :page-size="pageSize"
            :current-page="currentPage"
            @current-change="handlePageChange"
          />
        </div>
      </el-card>
    </el-main>
  </el-container>
</template>

<script setup>
import { reactive, ref, computed } from 'vue';

const names = ['Socrates', 'Balzac', 'Plato'];
const avatarSrc = [
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/a8c8cdb109cb051163646151a4a5083b.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/e278888093bef8910e829486fb45dd69.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/9eeb1800d9b78349b24682c3518ac4a3.png~tplv-uwbnlip3yd-webp.webp',
];
const imageSrc = [
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/29c1f9d7d17c503c5d7bf4e538cb7c4f.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/04d7bc31dd67dcdf380bc3f6aa07599f.png~tplv-uwbnlip3yd-webp.webp',
  '//p1-arco.byteimg.com/tos-cn-i-uwbnlip3yd/1f61854a849a076318ed527c8fca1bbf.png~tplv-uwbnlip3yd-webp.webp',
];

const dataSource = new Array(15).fill(null).map((_, index) => {
  return {
    index: index,
    avatar: avatarSrc[index % avatarSrc.length],
    title: names[index % names.length],
    description:
      'Beijing ByteDance Technology Co., Ltd. is an enterprise located in China. ByteDance has products such as TikTok, Toutiao, volcano video and Douyin (the Chinese version of TikTok).',
    imageSrc: imageSrc[index % imageSrc.length],
  };
});

const currentPage = ref(1);
const pageSize = 3;

const paginatedData = computed(() => {
  const start = (currentPage.value - 1) * pageSize;
  const end = start + pageSize;
  return dataSource.slice(start, end);
});

const handlePageChange = (val) => {
  currentPage.value = val;
};
</script>

<style scoped>
.list-container {
  padding: 20px;
}

.list-item {
  padding: 20px 0;
  border-bottom: 1px solid #e9e9eb;
}
.list-item:last-child {
  border-bottom: none;
}

.item-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.item-meta {
  display: flex;
  align-items: flex-start;
  margin-bottom: 10px;
}

.meta-content {
  margin-left: 10px;
}

.item-title {
  font-size: 16px;
  font-weight: bold;
  margin: 0 0 5px 0;
}

.item-description {
  font-size: 14px;
  color: #606266;
  margin: 0;
}

.item-actions {
  display: flex;
  align-items: center;
  margin-top: 10px;
}

.action-item {
  display: flex;
  align-items: center;
  margin-right: 20px;
  color: #909399;
  cursor: pointer;
}

.action-item .el-icon {
  margin-right: 4px;
}

.image-area {
  width: 183px;
  height: 119px;
  border-radius: 2px;
  overflow: hidden;
  display: flex;
  align-items: center;
  justify-content: center;
}

.image-area img {
  max-width: 100%;
  height: auto;
  display: block;
}

.pagination-container {
  margin-top: 20px;
  text-align: right;
}
</style>
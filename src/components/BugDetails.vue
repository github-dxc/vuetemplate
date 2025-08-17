<template>
  <el-container class="list-container">
    <el-container>
      <!-- Header: 基本信息 -->
      <el-header class="bug-header" height="180px">
        <el-card class="header-card" shadow="hover">
          <el-row :gutter="20">
            <el-col :span="12">
              <div class="info-item">
                <el-icon>
                  <User />
                </el-icon>
                <span class="label">报告员:</span>
                <span class="value">{{ bugData.reporter }}</span>
              </div>
              <div class="info-item">
                <el-icon>
                  <Calendar />
                </el-icon>
                <span class="label">提交时间:</span>
                <span class="value">{{ formatDate(bugData.date_submitted) }}</span>
              </div>
              <div class="info-item">
                <el-icon>
                  <PriceTag />
                </el-icon>
                <span class="label">标签:</span>
                <span class="value">{{ bugData.tags || '无' }}</span>
              </div>
            </el-col>
            <el-col :span="12">
              <div class="info-item">
                <el-icon>
                  <Document />
                </el-icon>
                <span class="label">描述:</span>
                <p class="description">{{ bugData.description }}</p>
              </div>
              <div class="info-item">
                <el-icon>
                  <List />
                </el-icon>
                <span class="label">重现步骤:</span>
                <p class="steps">{{ bugData.steps_to_reproduce }}</p>
              </div>
            </el-col>
          </el-row>
        </el-card>
      </el-header>
      <el-main>
        <el-card>
          <div v-for="(item, index) in dataSource" :key="index" class="list-item">
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
                    <span class="action-item"><el-icon><i class="el-icon-star-on"></i></el-icon>{{ item.index }}</span>
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
        </el-card>
      </el-main>
    </el-container>

    <el-aside width="200px">
      <el-card>
        操作历史
      </el-card>
    </el-aside>
  </el-container>
</template>

<script setup>
import { reactive, ref, computed } from 'vue';
import { formatDate } from '../util'

const props = defineProps({
  bugDetails: {
    type: Object,
    required: true,
    default: {}
  }
});
// 使用传入的数据，如果没有则使用示例数据
const bugData = computed(() => {
  return props.bugDetails || {
    "bug_id": 1,
    "project_id": "1",
    "project": "测试项目",
    "category_id": 1,
    "view_state": 10,
    "date_submitted": 1752508800,
    "last_updated": 1754750538,
    "reporter_id": 1,
    "reporter": "administrator",
    "handler_id": 1,
    "handler": "administrator",
    "priority": 30,
    "severity": 50,
    "reproducibility": 70,
    "status": 80,
    "resolution": 20,
    "summary": "测试摘要",
    "description": "测试描述1",
    "additional_information": "附注",
    "steps_to_reproduce": "问题步骤",
    "tags": "没加标签.",
    "bugnote_notes": [
      {
        "note_id": 0,
        "time": 1752508800,
        "text": "",
        "handler_id": 1,
        "handler": "administrator",
        "attachments": []
      },
      {
        "note_id": 1,
        "time": 1754668800,
        "text": "cestasd",
        "handler_id": 1,
        "handler": "administrator",
        "attachments": [
          {
            "size": 67078,
            "url": "file_download.php?file_id=2&type=bug",
            "name": "wechat_2025-08-09_224129_298.png"
          }
        ]
      }
    ],
    "attachments": [],
    "change_history": [
      {
        "bug_id": 1,
        "updated_at": 1752508800,
        "handler_id": 1,
        "handler": "administrator",
        "field": "新建问题",
        "change": ""
      },
      {
        "bug_id": 1,
        "updated_at": 1752508800,
        "handler_id": 1,
        "handler": "administrator",
        "field": "添加了以下文件：: 微信截图_20250715215845.png",
        "change": ""
      },
      {
        "bug_id": 1,
        "updated_at": 1752595200,
        "handler_id": 1,
        "handler": "administrator",
        "field": "分派给",
        "change": "=> administrator"
      },
      {
        "bug_id": 1,
        "updated_at": 1752595200,
        "handler_id": 1,
        "handler": "administrator",
        "field": "状态",
        "change": "新建 => 已分配"
      },
      {
        "bug_id": 1,
        "updated_at": 1754323200,
        "handler_id": 1,
        "handler": "administrator",
        "field": "描述已修改",
        "change": "<a href=\"bug_revision_view_page.php?rev_id=2#r2\">查看修订历史</a>"
      },
      {
        "bug_id": 1,
        "updated_at": 1754409600,
        "handler_id": 1,
        "handler": "administrator",
        "field": "状态",
        "change": "已分配 => 已解决"
      },
      {
        "bug_id": 1,
        "updated_at": 1754409600,
        "handler_id": 1,
        "handler": "administrator",
        "field": "处理状况",
        "change": "未处理 => 已修正"
      },
      {
        "bug_id": 1,
        "updated_at": 1754668800,
        "handler_id": 1,
        "handler": "administrator",
        "field": "添加了以下文件：: wechat_2025-08-09_224129_298.png",
        "change": ""
      },
      {
        "bug_id": 1,
        "updated_at": 1754668800,
        "handler_id": 1,
        "handler": "administrator",
        "field": "注释已添加: 0000001",
        "change": ""
      }
    ]
  };
});

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

const dataSource = new Array(3).fill(null).map((_, index) => {
  return {
    index: index,
    avatar: avatarSrc[index % avatarSrc.length],
    title: names[index % names.length],
    description:
      'Beijing ByteDance Technology Co., Ltd. is an enterprise located in China. ByteDance has products such as TikTok, Toutiao, volcano video and Douyin (the Chinese version of TikTok).',
    imageSrc: imageSrc[index % imageSrc.length],
  };
});

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
</style>
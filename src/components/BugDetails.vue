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
      <div class="overlap-comment">
        <div class="overlap-9">
          <div class="frame-3">
            <el-icon class="vector-4" :size="20">
              <Comment />
            </el-icon>
          </div>
          <div class="text-wrapper-18">评论 (5)</div>
        </div>
        <div class="overlap-10">
          <div class="overlap-11" :style="{backgroundColor: getColorByUnicPalette('李四').backgroundColor}">
            <div class="text-wrapper-19" :style="{color: getColorByUnicPalette('李四').textColor}">李</div>
          </div>
          <div class="text-wrapper-20">李四</div>
          <div class="text-wrapper-21">2024-01-15 15:45</div>
          <div class="text-wrapper-22">
            我也遇到了同样的问题，在Safari浏览器上也能重现。建议检查一下前端的错误处理逻辑。
          </div>
        </div>
        <div class="overlap-12">
          <div class="overlap-13">
            <div class="text-wrapper-23">王</div>
          </div>
          <div class="text-wrapper-20">王五</div>
          <div class="text-wrapper-21">2024-01-15 16:20</div>
          <div class="text-wrapper-22">
            已经定位到问题了，是API返回的错误信息格式有变化，前端没有正确解析。附上相关截图：
          </div>
          <img class="IMAGE" alt="Image" :src="IMAGE" />
        </div>
        <div class="overlap-14">
          <div class="rectangle-4" />
          <div class="rectangle-5" />
          <div class="text-wrapper-24">赵</div>
          <div class="text-wrapper-25">赵六</div>
          <div class="text-wrapper-26">2024-01-15 17:10</div>
          <div class="text-wrapper-27">
            我来负责修复这个问题。预计今天晚上就能提交修复代码。
          </div>
          <div class="rectangle-6" />
          <div class="rectangle-7" />
          <div class="text-wrapper-28">孙</div>
          <div class="text-wrapper-29">孙七</div>
          <div class="text-wrapper-30">2024-01-16 09:30</div>
          <div class="text-wrapper-31">
            测试环境已经修复了，可以验证一下。这是修复后的效果图：
          </div>
          <img class="IMAGE-2" alt="Image" :src="image1" />
        </div>
        <div class="overlap-15">
          <div class="text-wrapper-32">周</div>
        </div>
        <div class="text-wrapper-33">周八</div>
        <div class="text-wrapper-34">2024-01-16 14:15</div>
        <div class="text-wrapper-35">
          验证通过！现在错误提示显示正常了，用户体验得到了很大改善。可以关闭这个问题了。
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { formatDate, getFirstChar, getColorByUnicPalette } from '../util';

const props = defineProps({
  bugDetails: {
    type: Object,
    required: true,
    default: {}
  }
});

const bugInfo = computed(() => props.bugDetails);

const IMAGE = ref('path/to/IMAGE.png');
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
  height: 913px;
  left: 10px;
  right: 10px;
  position: relative;
  top: 20px;
  width: 780px;
}

.generated-design .overlap-9 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #e5e7eb;
  height: 76px;
  left: 1px;
  position: absolute;
  top: 1px;
  width: 780px;
}

.generated-design .frame-3 {
  height: 20px;
  left: 23px;
  position: absolute;
  top: 27px;
  width: 20px;
}

.generated-design .vector-4 {
  height: 20px;
  left: 2px;
  position: absolute;
  bottom: 2px;
  width: 20px;
}

.generated-design .text-wrapper-18 {
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

.generated-design .overlap-10 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #f3f4f6;
  height: 97px;
  left: 1px;
  position: absolute;
  top: 77px;
  width: 720px;
}

.generated-design .overlap-11 {
  background-color: #eff6ff;
  border-radius: 33554400px;
  height: 32px;
  left: 24px;
  position: absolute;
  top: 24px;
  width: 32px;
}

.generated-design .text-wrapper-19 {
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

.generated-design .text-wrapper-20 {
  color: #111827;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 67px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 22px;
  white-space: nowrap;
}

.generated-design .text-wrapper-21 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 12px;
  font-weight: 400;
  left: 103px;
  letter-spacing: 0;
  line-height: 18px;
  position: absolute;
  top: 23px;
  white-space: nowrap;
}

.generated-design .text-wrapper-22 {
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 67px;
  letter-spacing: 0;
  line-height: 22.8px;
  position: absolute;
  top: 46px;
  white-space: nowrap;
}

.generated-design .overlap-12 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #f3f4f6;
  height: 263px;
  left: 1px;
  position: absolute;
  top: 174px;
  width: 720px;
}

.generated-design .overlap-13 {
  background-color: #f0fdf4;
  border-radius: 33554400px;
  height: 32px;
  left: 24px;
  position: absolute;
  top: 24px;
  width: 32px;
}

.generated-design .text-wrapper-23 {
  color: #16a34a;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 14px;
  font-weight: 600;
  left: 9px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 4px;
  white-space: nowrap;
}

.generated-design .IMAGE {
  height: 154px;
  left: 67px;
  position: absolute;
  top: 83px;
  width: 304px;
}

.generated-design .overlap-14 {
  height: 379px;
  left: 1px;
  position: absolute;
  top: 437px;
  width: 720px;
}

.generated-design .rectangle-4 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #f3f4f6;
  height: 97px;
  left: 0;
  position: absolute;
  top: 0;
  width: 720px;
}

.generated-design .rectangle-5 {
  background-color: #fef2f2;
  border-radius: 33554400px;
  height: 32px;
  left: 24px;
  position: absolute;
  top: 24px;
  width: 32px;
}

.generated-design .text-wrapper-24 {
  color: #dc2626;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 14px;
  font-weight: 600;
  left: 33px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 28px;
  white-space: nowrap;
}

.generated-design .text-wrapper-25 {
  color: #111827;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 68px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 23px;
  white-space: nowrap;
}

.generated-design .text-wrapper-26 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 12px;
  font-weight: 400;
  left: 104px;
  letter-spacing: 0;
  line-height: 18px;
  position: absolute;
  top: 24px;
  white-space: nowrap;
}

.generated-design .text-wrapper-27 {
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 68px;
  letter-spacing: 0;
  line-height: 22.8px;
  position: absolute;
  top: 47px;
  white-space: nowrap;
}

.generated-design .rectangle-6 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #f3f4f6;
  height: 283px;
  left: 0;
  position: absolute;
  top: 96px;
  width: 720px;
}

.generated-design .rectangle-7 {
  background-color: #f0f9ff;
  border-radius: 33554400px;
  height: 32px;
  left: 24px;
  position: absolute;
  top: 120px;
  width: 32px;
}

.generated-design .text-wrapper-28 {
  color: #0ea5e9;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 14px;
  font-weight: 600;
  left: 33px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 125px;
  white-space: nowrap;
}

.generated-design .text-wrapper-29 {
  color: #111827;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 68px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 119px;
  white-space: nowrap;
}

.generated-design .text-wrapper-30 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 12px;
  font-weight: 400;
  left: 104px;
  letter-spacing: 0;
  line-height: 18px;
  position: absolute;
  top: 120px;
  white-space: nowrap;
}

.generated-design .text-wrapper-31 {
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 68px;
  letter-spacing: 0;
  line-height: 22.8px;
  position: absolute;
  top: 143px;
  white-space: nowrap;
}

.generated-design .IMAGE-2 {
  height: 174px;
  left: 68px;
  position: absolute;
  top: 180px;
  width: 288px;
}

.generated-design .overlap-15 {
  background-color: #fef3c7;
  border-radius: 33554400px;
  height: 32px;
  left: 25px;
  position: absolute;
  top: 840px;
  width: 32px;
}

.generated-design .text-wrapper-32 {
  color: #d97706;
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

.generated-design .text-wrapper-33 {
  color: #111827;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 68px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 838px;
  white-space: nowrap;
}

.generated-design .text-wrapper-34 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 12px;
  font-weight: 400;
  left: 104px;
  letter-spacing: 0;
  line-height: 18px;
  position: absolute;
  top: 839px;
  white-space: nowrap;
}

.generated-design .text-wrapper-35 {
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 68px;
  letter-spacing: 0;
  line-height: 22.8px;
  position: absolute;
  top: 862px;
  white-space: nowrap;
}
</style>
<template>
  <div class="generated-design">
    <div class="overlap-group-wrapper">
      <div class="overlap-group">
        <div class="overlap">
          <div class="div">
            <div class="rectangle" />
            <div class="frame">
              <img class="vector" alt="Vector" :src="vector" />
            </div>
            <div class="rectangle-2" />
          </div>
          <div class="text-wrapper">张三</div>
          <div class="div-wrapper">
            <div class="text-wrapper-2">高级开发</div>
          </div>
          <div class="text-wrapper-3">报告员</div>
          <div class="rectangle-3" />
          <div class="vector-wrapper">
            <img class="img" alt="Vector" :src="image" />
          </div>
          <div class="text-wrapper-4">活跃</div>
          <div class="text-wrapper-5">提交时间</div>
          <div class="text-wrapper-6">2024-01-15 14:30</div>
          <div class="overlap-2">
            <div class="text-wrapper-7">高优先级</div>
          </div>
          <div class="overlap-3">
            <div class="text-wrapper-8">UI问题</div>
          </div>
          <div class="overlap-4">
            <div class="text-wrapper-9">前端</div>
          </div>
          <div class="text-wrapper-10">问题描述</div>
          <div class="text-wrapper-11">
            在用户登录页面，当输入错误的用户名和密码时，错误提示信息没有正确显示。页面只是简单地刷新，没有给用户任何反馈，导致用户体验很差。这个问题在Chrome和Firefox浏览器上都能重现。
          </div>
          <div class="text-wrapper-12">重现步骤</div>
          <div class="text-wrapper-13">展开</div>
          <div class="img-wrapper">
            <img class="vector-2" alt="Vector" :src="vector2" />
          </div>
          <div class="text-wrapper-14">点击展开查看详细的重现步骤...</div>
          <div class="overlap-5">
            <div class="frame-2">
              <img class="vector-3" alt="Vector" :src="vector3" />
            </div>
            <div class="text-wrapper-15">最后更新: 2024-01-15</div>
            <div class="overlap-6">
              <div class="text-wrapper-16">编辑</div>
            </div>
            <div class="overlap-7">
              <div class="text-wrapper-17">分配处理</div>
            </div>
          </div>
        </div>
        <div class="overlap-8">
          <div class="overlap-9">
            <div class="frame-3">
              <img class="vector-4" alt="Vector" :src="vector4" />
            </div>
            <div class="text-wrapper-18">评论 (5)</div>
          </div>
          <div class="overlap-10">
            <div class="overlap-11">
              <div class="text-wrapper-19">李</div>
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
  </div>
</template>

<script>
// import IMAGE from './IMAGE.png';
// import image1 from './image.png';
// import image from './image.svg';
// import vector2 from './vector-2.svg';
// import vector3 from './vector-3.svg';
// import vector4 from './vector-4.svg';
// import vector from './vector.svg';

export default {
  name: 'GeneratedDesign',
  data() {
    return {
      vector: '',
      image,
      vector2,
      vector3,
      vector4,
      IMAGE,
    };
  },
};
</script>

<style>
.generated-design {
  align-items: start;
  background-color: #ffffff;
  display: grid;
  justify-items: center;
  width: 100vw;
}

.generated-design .overlap-group-wrapper {
  background-color: #ffffff;
  height: 1441px;
  width: 1271px;
}

.generated-design .overlap-group {
  background-color: #ffffff;
  height: 1441px;
  position: relative;
  width: 1256px;
}

.generated-design .overlap {
  background-color: #ffffff;
  border: 1px solid;
  border-color: #e5e7eb;
  border-radius: 8px;
  box-shadow:
    0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 1px 2px #0000000f, 0px 1px 3px #0000001a;
  height: 440px;
  left: 180px;
  position: absolute;
  top: 32px;
  width: 896px;
}

.generated-design .div {
  height: 52px;
  left: 25px;
  position: absolute;
  top: 31px;
  width: 52px;
}

.generated-design .rectangle {
  background: linear-gradient(
    90deg,
    rgba(29, 78, 216, 1) 100%,
    rgba(59, 130, 246, 1) 0%
  );
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
  left: 36px;
  position: absolute;
  top: 36px;
  width: 16px;
}

.generated-design .text-wrapper {
  color: #111827;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 18px;
  font-weight: 600;
  left: 88px;
  letter-spacing: 0;
  line-height: 28px;
  position: absolute;
  top: 26px;
  white-space: nowrap;
}

.generated-design .div-wrapper {
  background-color: #eff6ff;
  border: 1px solid;
  border-color: #dbeafe;
  border-radius: 33554400px;
  height: 34px;
  left: 133px;
  position: absolute;
  top: 25px;
  width: 62px;
}

.generated-design .text-wrapper-2 {
  color: #2563eb;
  font-family: "Inter-Medium", Helvetica;
  font-size: 11px;
  font-weight: 500;
  left: 8px;
  letter-spacing: 0;
  line-height: 16.5px;
  position: absolute;
  top: 9px;
  white-space: nowrap;
}

.generated-design .text-wrapper-3 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 88px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 61px;
  white-space: nowrap;
}

.generated-design .rectangle-3 {
  background-color: #d1d5db;
  border-radius: 33554400px;
  height: 4px;
  left: 138px;
  position: absolute;
  top: 71px;
  width: 4px;
}

.generated-design .vector-wrapper {
  height: 12px;
  left: 150px;
  position: absolute;
  top: 67px;
  width: 12px;
}

.generated-design .img {
  height: 10px;
  left: 1px;
  position: absolute;
  top: 1px;
  width: 10px;
}

.generated-design .text-wrapper-4 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 166px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 61px;
  white-space: nowrap;
}

.generated-design .text-wrapper-5 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 814px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  text-align: right;
  top: 23px;
  white-space: nowrap;
}

.generated-design .text-wrapper-6 {
  color: #111827;
  font-family: "Inter-Medium", Helvetica;
  font-size: 16px;
  font-weight: 500;
  left: 737px;
  letter-spacing: 0;
  line-height: 24px;
  position: absolute;
  text-align: right;
  top: 48px;
  white-space: nowrap;
}

.generated-design .overlap-2 {
  background-color: #fef2f2;
  border: 1px solid;
  border-color: #fecaca;
  border-radius: 33554400px;
  height: 34px;
  left: 25px;
  position: absolute;
  top: 108px;
  width: 74px;
}

.generated-design .text-wrapper-7 {
  color: #dc2626;
  font-family: "Inter-Medium", Helvetica;
  font-size: 12px;
  font-weight: 500;
  left: 12px;
  letter-spacing: 0;
  line-height: 18px;
  position: absolute;
  top: 8px;
  white-space: nowrap;
}

.generated-design .overlap-3 {
  background-color: #eff6ff;
  border: 1px solid;
  border-color: #dbeafe;
  border-radius: 33554400px;
  height: 34px;
  left: 107px;
  position: absolute;
  top: 108px;
  width: 62px;
}

.generated-design .text-wrapper-8 {
  color: #2563eb;
  font-family: "Inter-Medium", Helvetica;
  font-size: 12px;
  font-weight: 500;
  left: 12px;
  letter-spacing: 0;
  line-height: 18px;
  position: absolute;
  top: 8px;
  white-space: nowrap;
}

.generated-design .overlap-4 {
  background-color: #f0fdf4;
  border: 1px solid;
  border-color: #bbf7d0;
  border-radius: 33554400px;
  height: 34px;
  left: 177px;
  position: absolute;
  top: 108px;
  width: 50px;
}

.generated-design .text-wrapper-9 {
  color: #16a34a;
  font-family: "Inter-Medium", Helvetica;
  font-size: 12px;
  font-weight: 500;
  left: 12px;
  letter-spacing: 0;
  line-height: 18px;
  position: absolute;
  top: 8px;
  white-space: nowrap;
}

.generated-design .text-wrapper-10 {
  color: #111827;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 16px;
  font-weight: 600;
  left: 24px;
  letter-spacing: 0;
  line-height: 24px;
  position: absolute;
  top: 164px;
  white-space: nowrap;
}

.generated-design .text-wrapper-11 {
  color: #374151;
  font-family: "Inter-Regular", Helvetica;
  font-size: 16px;
  font-weight: 400;
  left: 24px;
  letter-spacing: 0;
  line-height: 26px;
  position: absolute;
  top: 203px;
  width: 832px;
}

.generated-design .text-wrapper-12 {
  color: #111827;
  font-family: "Inter-SemiBold", Helvetica;
  font-size: 16px;
  font-weight: 600;
  left: 24px;
  letter-spacing: 0;
  line-height: 24px;
  position: absolute;
  top: 276px;
  white-space: nowrap;
}

.generated-design .text-wrapper-13 {
  color: #2563eb;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 822px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  text-align: center;
  top: 278px;
  white-space: nowrap;
}

.generated-design .img-wrapper {
  height: 16px;
  left: 854px;
  position: absolute;
  top: 281px;
  width: 16px;
}

.generated-design .vector-2 {
  height: 7px;
  left: 2px;
  position: absolute;
  top: 5px;
  width: 11px;
}

.generated-design .text-wrapper-14 {
  color: #6b7280;
  font-family: "Inter-Regular", Helvetica;
  font-size: 14px;
  font-weight: 400;
  left: 24px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  top: 312px;
  white-space: nowrap;
}

.generated-design .overlap-5 {
  border-color: #e5e7eb;
  border-top-style: solid;
  border-top-width: 1px;
  height: 56px;
  left: 25px;
  position: absolute;
  top: 359px;
  width: 846px;
}

.generated-design .frame-2 {
  height: 16px;
  left: -1px;
  position: absolute;
  top: 28px;
  width: 16px;
}

.generated-design .vector-3 {
  height: 13px;
  left: 2px;
  position: absolute;
  top: 1px;
  width: 13px;
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
  top: 24px;
  white-space: nowrap;
}

.generated-design .overlap-6 {
  background-color: #f9fafb;
  border: 1px solid;
  border-color: #d1d5db;
  border-radius: 6px;
  height: 39px;
  left: 686px;
  position: absolute;
  top: 17px;
  width: 62px;
}

.generated-design .text-wrapper-16 {
  color: #374151;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 16px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  text-align: center;
  top: 7px;
  white-space: nowrap;
}

.generated-design .overlap-7 {
  background-color: #2563eb;
  border-radius: 6px;
  height: 39px;
  left: 756px;
  position: absolute;
  top: 17px;
  width: 90px;
}

.generated-design .text-wrapper-17 {
  color: #ffffff;
  font-family: "Inter-Medium", Helvetica;
  font-size: 14px;
  font-weight: 500;
  left: 17px;
  letter-spacing: 0;
  line-height: 21px;
  position: absolute;
  text-align: center;
  top: 8px;
  white-space: nowrap;
}

.generated-design .overlap-8 {
  background-color: #ffffff;
  border: 1px solid;
  border-color: #e5e7eb;
  border-radius: 8px;
  box-shadow:
    0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 0px 0px transparent, 0px 1px 2px #0000000f, 0px 1px 3px #0000001a;
  height: 913px;
  left: 180px;
  position: absolute;
  top: 496px;
  width: 896px;
}

.generated-design .overlap-9 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #e5e7eb;
  height: 76px;
  left: 1px;
  position: absolute;
  top: 1px;
  width: 894px;
}

.generated-design .frame-3 {
  height: 20px;
  left: 23px;
  position: absolute;
  top: 27px;
  width: 20px;
}

.generated-design .vector-4 {
  height: 16px;
  left: 2px;
  position: absolute;
  top: 2px;
  width: 17px;
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
  width: 894px;
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
  width: 894px;
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
  width: 894px;
}

.generated-design .rectangle-4 {
  border-bottom-style: solid;
  border-bottom-width: 1px;
  border-color: #f3f4f6;
  height: 97px;
  left: 0;
  position: absolute;
  top: 0;
  width: 894px;
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
  width: 894px;
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
<template>
  <div class="setting-group">
    <div class="setting-item">
      <h3>启动设置</h3>
      <el-form-item label="开机自动启动">
        <el-switch v-model="startSettings.autoStart" @change="changeStartSetting" />
      </el-form-item>
    </div>
    <div class="setting-item">
      <h3>更新设置</h3>
      <el-form-item label="自动更新">
        <el-switch v-model="updateSettings.autoUpdate" @change="changeUpdateSetting" />
      </el-form-item>
    </div>
    <div class="setting-item">
      <h3>MantisBT设置</h3>
      <el-form-item label="HOST地址">
        <el-input v-model="hostSetting" style="width: 240px" placeholder="Please input MantisBT HOST" @change="changeHostSetting" />
      </el-form-item>
    </div>
    <div class="setting-item">
      <h3>订阅设置</h3>
      <el-form-item label="订阅参数">
        <el-input-tag
          v-model="subParamsSetting"
          placeholder="Please input"
          draggable
          aria-label="Please click the Enter key after input"
          @change="changeSubParamsSetting"
        />
      </el-form-item>
    </div>
  </div>
</template>

<script setup vapor>
import { onMounted, ref } from 'vue';
import { useUserStore } from '../../store';
import { autoStart } from '../../windows';
import { changeSubParams, subParamsInfo } from '../../api';
import { useRouter } from 'vue-router';

const router = useRouter()
const userStore = useUserStore()

const updateSettings = ref({
  autoUpdate: userStore.settingInfo.update.autoUpdate || false,
});

const startSettings = ref({
  autoStart: userStore.settingInfo.start.autoStart || false,
});

const subParamsSetting = ref([]);

const hostSetting = ref(userStore.settingInfo.host || '');

const changeUpdateSetting = () => {
  userStore.updateSetting({ update: updateSettings.value })
};

const changeStartSetting = async () => {
  userStore.updateSetting({ start: startSettings.value })
  autoStart(startSettings.value.autoStart);
};

const changeSubParamsSetting = async () => {
  try {
    // 把数组中的编码过的参数格式化为正常字符串
    subParamsSetting.value = subParamsSetting.value.map(item => decodeURIComponent(item));
    await changeSubParams(subParamsSetting.value);
  } catch (error) {
    console.error("Failed to change sub params:", error);
  }
  getParamsInfo();
};

const changeHostSetting = async () => {
  try {
    await userStore.changeGetHost(hostSetting.value);
    userStore.logout();
    router.push("Login");
  } catch (error) {
    console.error("Failed to change host:", error);
  }
};

const getParamsInfo = async () => {
  try {
    const res = await subParamsInfo(subParamsSetting.value);
    subParamsSetting.value = res || [];
  } catch (error) {
    console.error("Failed to fetch sub params info:", error);
  }
  
};

onMounted(async () => {
  getParamsInfo();
});

</script>

<style scoped>
.setting-group {
  margin-bottom: 30px;
}

.setting-group h3 {
  margin: 0 0 15px 0;
  font-size: 16px;
  color: #333;
  font-weight: 500;
}

.el-input-tag {
  width: 400px;
}

</style>
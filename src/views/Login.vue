<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vitejs.dev" target="_blank">
        <img src="/vite.svg" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <img src="../assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click to log in to the bug system.</p>

    <div class="row">
      <div v-if="showUsername" class="input-group">
        <input
          class="greet-input"
          ref="usernameInputRef"
          v-model="username"
          placeholder="Enter a username..."
          @keyup.enter="nextEvent"
        />
      </div>
      <div v-if="showPassword" class="input-group">
        <input
          class="greet-input"
          ref="passwordInputRef"
          v-model="password"
          type="password"
          placeholder="Enter a password..."
          @keyup.enter="nextEvent"
        />
      </div>
      <button @click="nextEvent">
        <div v-if="showUsername && password == ''">Next</div>
        <div v-if="showPassword && password == ''">Back</div>
        <div v-if="showPassword && password !== ''">Login</div>
      </button>
          <button @click="testLogin">test</button>
    </div>

    <!-- 设置按钮 -->
    <div class="settings-button" @click="toggleSettings" :class="{ active: showSettings }">
      <svg class="settings-icon" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M12 15C13.6569 15 15 13.6569 15 12C15 10.3431 13.6569 9 12 9C10.3431 9 9 10.3431 9 12C9 13.6569 10.3431 15 12 15Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M19.4 15C19.2669 15.3016 19.2272 15.6362 19.286 15.9606C19.3448 16.285 19.4995 16.5843 19.73 16.82L19.79 16.88C19.976 17.0657 20.1235 17.2863 20.2241 17.5291C20.3248 17.7719 20.3766 18.0322 20.3766 18.295C20.3766 18.5578 20.3248 18.8181 20.2241 19.0609C20.1235 19.3037 19.976 19.5243 19.79 19.71C19.6043 19.896 19.3837 20.0435 19.1409 20.1441C18.8981 20.2448 18.6378 20.2966 18.375 20.2966C18.1122 20.2966 17.8519 20.2448 17.6091 20.1441C17.3663 20.0435 17.1457 19.896 16.96 19.71L16.9 19.65C16.6663 19.4195 16.367 19.2648 16.0426 19.206C15.7182 19.1472 15.3836 19.1869 15.082 19.32C14.7869 19.4468 14.532 19.6572 14.3439 19.9255C14.1558 20.1938 14.0417 20.5082 14.013 20.836L14.013 21C14.013 21.5304 13.8022 22.0391 13.4271 22.4142C13.052 22.7893 12.5433 23 12.013 23C11.4826 23 10.9739 22.7893 10.5988 22.4142C10.2237 22.0391 10.013 21.5304 10.013 21V20.91C9.97745 20.5682 9.84924 20.2413 9.64151 19.9671C9.43378 19.6929 9.15476 19.4815 8.833 19.355C8.53142 19.2219 8.19682 19.1822 7.87244 19.241C7.54805 19.2998 7.24874 19.4545 7.015 19.685L6.955 19.745C6.76929 19.931 6.54871 20.0785 6.30589 20.1791C6.06307 20.2798 5.80275 20.3316 5.53995 20.3316C5.27714 20.3316 5.01682 20.2798 4.774 20.1791C4.53118 20.0785 4.3106 19.931 4.12489 19.745C3.93894 19.5593 3.79142 19.3387 3.69076 19.0959C3.59011 18.8531 3.53832 18.5928 3.53832 18.33C3.53832 18.0672 3.59011 17.8069 3.69076 17.5641C3.79142 17.3213 3.93894 17.1007 4.12489 16.915L4.18489 16.855C4.41544 16.6213 4.57011 16.322 4.62891 15.9976C4.68771 15.6732 4.64799 15.3386 4.51489 15.037C4.38811 14.7419 4.17771 14.487 3.90943 14.2989C3.64115 14.1108 3.32676 13.9967 2.99889 13.968H2.88889C2.35845 13.968 1.84978 13.7572 1.47467 13.3821C1.09956 13.007 0.888886 12.4983 0.888886 11.968C0.888886 11.4376 1.09956 10.9289 1.47467 10.5538C1.84978 10.1787 2.35845 9.96799 2.88889 9.96799H2.97889C3.32063 9.93244 3.64757 9.81423 3.92178 9.6265C4.19599 9.43877 4.40739 9.15975 4.53389 8.83799C4.66699 8.53641 4.70671 8.20181 4.64791 7.87743C4.58911 7.55304 4.43444 7.25373 4.20389 7.01999L4.14389 6.95999C3.95794 6.77428 3.81042 6.5537 3.70977 6.31088C3.60911 6.06806 3.55732 5.80774 3.55732 5.54494C3.55732 5.28213 3.60911 5.02181 3.70977 4.77899C3.81042 4.53617 3.95794 4.31559 4.14389 4.12988C4.3296 3.94393 4.55018 3.79641 4.793 3.69575C5.03582 3.5951 5.29614 3.54331 5.55894 3.54331C5.82175 3.54331 6.08207 3.5951 6.32489 3.69575C6.56771 3.79641 6.78829 3.94393 6.974 4.12988L7.034 4.18988C7.26774 4.42043 7.56705 4.5751 7.89144 4.6339C8.21582 4.6927 8.55042 4.65298 8.852 4.51988H8.911C9.20611 4.3931 9.46096 4.18271 9.64906 3.91443C9.83715 3.64615 9.95127 3.33176 9.98 3.00399V2.89399C9.98 2.36355 10.1907 1.85488 10.5658 1.47977C10.9409 1.10466 11.4496 0.893982 11.98 0.893982C12.5104 0.893982 13.0191 1.10466 13.3942 1.47977C13.7693 1.85488 13.98 2.36355 13.98 2.89399V2.98399C14.0087 3.31176 14.1228 3.62615 14.3109 3.89443C14.499 4.16271 14.7539 4.3731 15.049 4.49988C15.3506 4.63298 15.6852 4.6727 16.0096 4.6139C16.334 4.5551 16.6333 4.40043 16.867 4.16988L16.927 4.10988C17.1127 3.92393 17.3333 3.77641 17.5761 3.67575C17.8189 3.5751 18.0793 3.52331 18.3421 3.52331C18.6049 3.52331 18.8652 3.5751 19.108 3.67575C19.3508 3.77641 19.5714 3.92393 19.7571 4.10988C19.943 4.29559 20.0905 4.51617 20.1912 4.75899C20.2918 5.00181 20.3436 5.26213 20.3436 5.52494C20.3436 5.78774 20.2918 6.04806 20.1912 6.29088C20.0905 6.5337 19.943 6.75428 19.7571 6.93999L19.6971 6.99999C19.4665 7.23373 19.3118 7.53304 19.253 7.85743C19.1942 8.18181 19.2339 8.51641 19.367 8.81799V8.87699C19.4938 9.17211 19.7042 9.42696 19.9725 9.61505C20.2408 9.80315 20.5552 9.91727 20.883 9.94599H20.993C21.5234 9.94599 22.0321 10.1567 22.4072 10.5318C22.7823 10.9069 22.993 11.4156 22.993 11.946C22.993 12.4764 22.7823 12.9851 22.4072 13.3602C22.0321 13.7353 21.5234 13.946 20.993 13.946H20.903C20.5751 13.9747 20.2607 14.0888 19.9924 14.2769C19.7241 14.465 19.5137 14.7199 19.387 15.015V15.015Z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </div>

    <!-- 设置弹出框 -->
    <div v-if="showSettings" class="settings-modal" @click="closeSettingsModal">
      <div class="settings-content" @click.stop>
        <h3>Configuration</h3>
        <div class="input-group">
          <label for="host-input">HOST:</label>
          <input
            id="host-input"
            ref="hostInputRef"
            v-model="hostConfig"
            placeholder="Enter HOST URL..."
            class="host-input"
            @keyup.enter="saveHostConfig"
          />
        </div>
        <div class="settings-buttons">
          <button @click="saveHostConfig" class="save-btn">Save</button>
          <button @click="closeSettings" class="cancel-btn">Cancel</button>
        </div>
      </div>
    </div>
  </main>
</template>

<script setup vapor>
import { ref, onMounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from 'vue-router';
import { useUserStore } from "../store";
import { changeHost } from "../api";
import { ElMessage } from 'element-plus';

const router = useRouter()
const username = ref("");
const password = ref("");
const showUsername = ref(true);
const showPassword = ref(false);
const usernameInputRef = ref(null);
const passwordInputRef = ref(null);

// 设置相关的状态
const showSettings = ref(false);
const hostConfig = ref("");
const hostInputRef = ref(null);

async function testLogin() {
  try {
    const result = await invoke("api_login", { username: 'administrator', password: 'abcd.1234' });
    console.log("登录成功", result);
    router.push("/home");
  } catch (error) {
    console.error("Test login failed");
    ElMessage({
      showClose: true,
      message: 'Test login failed. ' + error,
      type: 'error',
    });
  }
}

async function login() {
  try {
    const result = await invoke("api_login", { username: username.value, password: password.value });
    console.log("登录成功", result);
    router.push("/home");
  } catch (error) {
    // 登录失败
    console.error("登录失败");
    ElMessage({
      showClose: true,
      message: 'Login failed. ' + error,
      type: 'error',
    });
  }
}

function nextEvent() {
  if (showUsername.value) {
    if (username.value == "") {
      return;
    }
    showUsername.value = false;
    showPassword.value = true;
    nextTick(() => {
      passwordInputRef.value?.focus();
    });
  } else if (showPassword.value) {
    if (password.value == "") {
      showPassword.value = false;
      showUsername.value = true;
      nextTick(() => {
        usernameInputRef.value?.focus();
      });
      return;
    }
    login();
  }
}

// 设置相关的方法
function toggleSettings() {
  showSettings.value = !showSettings.value;
  if (showSettings.value) {
    nextTick(() => {
      hostInputRef.value?.focus();
    });
  }
}

function closeSettings() {
  showSettings.value = false;
}

function closeSettingsModal(event) {
  if (event.target === event.currentTarget) {
    closeSettings();
  }
}

async function saveHostConfig() {
  try {
    await changeHost({host: hostConfig.value});
  } catch (error) {
    ElMessage({
      showClose: true,
      message: 'HOST configuration change failed!' + error,
      type: 'error',
    });
  }
  
  ElMessage({
    showClose: true,
    message: 'HOST configuration saved successfully!',
    type: 'success',
  });
  closeSettings();
}

onMounted(async () => {
  const userStore = useUserStore();
  if (userStore.isLoggedIn) {
    router.push("Login");
  }

  try {
    hostConfig.value = await changeHost({host: ""});
  } catch (error) {
    ElMessage({
      showClose: true,
      message: 'HOST configuration get failed!' + error,
      type: 'error',
    });
  }
})
</script>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.container {
  padding-top: 20vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  position: relative;
}

input,button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

/* 设置按钮样式 */
.settings-button {
  position: fixed;
  bottom: 30px;
  right: 30px;
  width: 44px;
  height: 44px;
  background-color: rgba(255, 255, 255, 0.9);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  z-index: 100;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.08);
}

.settings-button:hover {
  background-color: rgba(255, 255, 255, 1);
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.settings-button.active {
  background-color: rgba(240, 240, 240, 1);
  transform: scale(0.95);
}

.settings-icon {
  width: 20px;
  height: 20px;
  color: #666;
  transition: transform 0.3s ease;
}

.settings-button:hover .settings-icon {
  transform: rotate(90deg);
}

.settings-button.active .settings-icon {
  transform: rotate(180deg);
}

/* 设置弹出框样式 */
.settings-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease-out;
}

.settings-content {
  background-color: #ffffff;
  border-radius: 12px;
  padding: 24px;
  min-width: 400px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.3s ease-out;
}

.settings-content h3 {
  margin: 0 0 20px 0;
  color: #0f0f0f;
  font-size: 1.2em;
}

.settings-content .input-group {
  margin-bottom: 20px;
}

.settings-content label {
  display: block;
  margin-bottom: 8px;
  color: #0f0f0f;
  font-weight: 500;
}

.host-input {
  width: 100%;
  box-sizing: border-box;
}

.settings-buttons {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.save-btn {
  background-color: #646cff;
  color: white;
  border: none;
}

.save-btn:hover {
  background-color: #535bf2;
}

.cancel-btn {
  background-color: #f0f0f0;
  color: #0f0f0f;
  border: none;
}

.cancel-btn:hover {
  background-color: #e0e0e0;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* 深色模式适配 */
@media (prefers-color-scheme: dark) {
  .settings-button {
    background-color: rgba(47, 47, 47, 0.9);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .settings-button:hover {
    background-color: rgba(47, 47, 47, 1);
  }

  .settings-button.active {
    background-color: rgba(64, 64, 64, 1);
  }

  .settings-icon {
    color: #ccc;
  }

  .settings-content {
    background-color: #2f2f2f;
    color: #f6f6f6;
  }

  .settings-content h3 {
    color: #f6f6f6;
  }

  .settings-content label {
    color: #f6f6f6;
  }

  .host-input {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  .cancel-btn {
    background-color: #404040;
    color: #f6f6f6;
  }

  .cancel-btn:hover {
    background-color: #505050;
  }
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

.greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
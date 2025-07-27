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

    <form class="row" @submit.prevent="handleSubmit">
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
    </form>
  </main>
</template>

<script setup vapor>
import { ref, onMounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useRouter } from 'vue-router';
import { useUserStore } from "../store";

const router = useRouter()
const username = ref("");
const password = ref("");
const showUsername = ref(true);
const showPassword = ref(false);
const usernameInputRef = ref(null);
const passwordInputRef = ref(null);

async function login() {
  try {
    const result = await invoke("api_login", { username: username.value, password: password.value });
    console.log("登录成功", result);
    router.push("/home");
  } catch (error) {
    // 登录失败
    console.error("登录失败");
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
function handleSubmit(event) {
  event.preventDefault(); // 阻止默认提交行为
  nextEvent();
}

onMounted(async () => {
  const userStore = useUserStore();
  if (userStore.isLoggedIn) {
    router.push("Login");
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

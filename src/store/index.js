import { createPinia, defineStore } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { login, logout, loginInfo, changeHost, readMsg } from '../api/index'
import avatar from '../assets/avatar.png';

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)//持久化
export default pinia

export const useUserStore = defineStore('user', {
  persist: true,
  state: () => ({
    // 用户信息
    user: {
      user_id: null,
      logined: false,
      username: '',
      read_msg: '',
      avatar: avatar,
      role: '',
      createdAt: null
    },

    // 认证token
    token: '',

    // 加载状态
    loading: false,

    // 版本信息
    version: {
      currentVersion: "0.1.0",
      lastVersion: "0.1.0",
      updateTime: Math.floor(Date.now() / 1000),
    },

    // 系统设置
    setting: {
      theme: 'light', // light | dark
      language: 'zh-CN', // zh-CN | en-US
      host: 'localhost:8989',
      sidebar: {
        collapsed: false,
        fixed: true
      },
      notifications: {
        email: true,
        push: true,
        sound: false
      },
      layout: {
        header: true,
        footer: true,
        breadcrumb: true
      },
      update: {
        skipVersion: '0.0.0',
        autoUpdate: false,
      },
      start: {
        autoStart: false,
      },
      shortcut: {
        timestamp: false,
      }
    }
  }),

  getters: {
    // 用户信息
    userInfo: (state) => state.user,

    // 用户token
    userToken: (state) => state.token,

    // host设置
    serverHost: (state) => state.setting.host,

    // 当前主题
    currentTheme: (state) => state.setting.theme,

    // 是否加载中
    isLoading: (state) => state.loading,

    // 获取配置
    settingInfo: (state) => state.setting,

    // 版本信息
    versionInfo: (state) => state.version,
  },

  actions: {
    // 设置用户信息
    setUser(userInfo) {
      this.user = { ...this.user, ...userInfo }
    },

    // 设置token
    setToken(token) {
      console.log('Setting token:', token)
      this.token = token
    },

    // 设置加载状态
    setLoading(status) {
      this.loading = status
    },

    // 设置版本信息
    setVersion(version) {
      this.version = version || {}
    },

    // 批量更新设置，只更新非null和非undefined的值（包括对象和数组内部）
    updateSetting(setting) {
      const isValid = (val) => val !== null && val !== undefined;
      const deepUpdate = (target, source) => {
        for (const key in source) {
          if (Object.prototype.hasOwnProperty.call(source, key)) {
            const val = source[key];
            if (Array.isArray(val)) {
              if (!Array.isArray(target[key])) target[key] = [];
              for (let i = 0; i < val.length; i++) {
                if (isValid(val[i])) {
                  target[key][i] = val[i];
                }
              }
            } else if (typeof val === 'object' && val !== null) {
              if (!target[key] || typeof target[key] !== 'object') target[key] = {};
              deepUpdate(target[key], val);
            } else if (isValid(val)) {
              target[key] = val;
            }
          }
        }
      };
      deepUpdate(this.setting, setting);
    },

    // 切换主题
    toggleTheme() {
      this.setting.theme = this.setting.theme === 'light' ? 'dark' : 'light'
    },

    // 切换侧边栏
    toggleSidebar() {
      this.setting.sidebar.collapsed = !this.setting.sidebar.collapsed
    },

    // 登录
    async login(username, password) {
      this.setLoading(true)
      try {
        // 这里调用API进行登录
        const token = await login(username, password)
        this.setToken(token)
        const user = await loginInfo()
        const userInfo = {
          ...user,
          avatar: avatar,
          createdAt: new Date().toISOString()
        }
        this.setUser(userInfo)
        return { success: true, data: userInfo }
      } catch (error) {
        this.resetUser()
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },

    // 登出
    async logout() {
      this.setLoading(true)
      try {
        // 调用登出API
        await logout();
        this.resetUser();
        return { success: true }
      } catch (error) {
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },

    // 同步用户信息
    async getUserInfo() {
      this.setLoading(true)
      try {
        const user = await loginInfo()
        const userInfo = {
          ...user,
          createdAt: new Date().toISOString()
        }
        this.setUser(userInfo)
        return { success: true, data: userInfo }
      } catch (error) {
        // token可能已过期，清除登录状态
        this.resetUser()
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },

    // 设置读取消息
    async readMsg(updated_at, bug_id, handler_id) {
      let read_msg = `${updated_at}-${bug_id}-${handler_id}`
      this.setLoading(true)
      try {
        await readMsg(read_msg)
        const user = await loginInfo()
        const userInfo = {
          ...user,
          createdAt: new Date().toISOString()
        }
        this.setUser(userInfo)
        return { success: true, data: userInfo }
      } catch (error) {
        // token可能已过期，清除登录状态
        this.resetUser()
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },

    // 更新获取host
    async changeGetHost(host) {
      this.setLoading(true)
      try {
        const newHost = await changeHost(host)
        this.updateSetting({ host: newHost })
        return { success: true, data: newHost }
      } catch (error) {
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },

    // 重置user到初始状态
    resetUser() {
      this.user = {
        user_id: null,
        logined: false,
        username: '',
        avatar: '',
        role: '',
        createdAt: null
      }
      this.setToken('')
      this.loading = false
      // 设置保持不变，不重置
    }
  }
})

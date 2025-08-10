import { createPinia, defineStore } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { login, logout } from '../api/index'

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)//持久化
export default pinia

export const useUserStore = defineStore('user', {
  state: () => ({
    // 用户信息
    user: {
      id: null,
      username: '',
      email: '',
      avatar: '',
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
      }
    }
  }),

  getters: {
    // 是否已登录
    isLoggedIn: (state) => !!state.token && !!state.user.id,
    
    // 用户全名或用户名
    displayName: (state) => state.user.username || state.user.email,
    
    // 用户头像（带默认值）
    userAvatar: (state) => state.user.avatar || '/default-avatar.png',

    // 用户token
    userToken: (state) => state.token,
    
    // 是否为管理员
    isAdmin: (state) => state.user.role === 'admin',
    
    // 当前主题
    currentTheme: (state) => state.setting.theme,
    
    // 是否加载中
    isLoading: (state) => state.loading,

    // 获取更新配置
    updateInfo: (state) => state.setting.update,

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
        const cookie = await login(username, password)
        const user = {
          token: 'mock-jwt-token',
          user: {
            username,
            createdAt: new Date().toISOString()
          }
        }
        
        this.setToken(cookie)
        this.setUser(user)
        
        return { success: true, data: user }
      } catch (error) {
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
        
        this.setToken('')
        this.setUser({
          id: null,
          username: '',
          email: '',
          avatar: '',
          role: '',
          createdAt: null
        })
        
        return { success: true }
      } catch (error) {
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },
    
    // 获取用户信息
    async fetchUserInfo() {
      if (!this.token) return { success: false, error: 'No token' }
      
      this.setLoading(true)
      try {
        // 调用获取用户信息API
        // const response = await getUserInfoApi()
        // 模拟API响应
        const response = {
          id: 1,
          username: 'testuser',
          email: 'test@example.com',
          avatar: '/avatar.jpg',
          role: 'user',
          createdAt: '2024-01-01T00:00:00Z'
        }
        
        this.setUser(response)
        return { success: true, data: response }
      } catch (error) {
        // token可能已过期，清除登录状态
        this.logout()
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },
    
    // 更新用户资料
    async updateProfile(profileData) {
      this.setLoading(true)
      try {
        // 调用更新资料API
        // const response = await updateProfileApi(profileData)
        // 模拟API响应
        const response = { ...this.user, ...profileData }
        
        this.setUser(response)
        return { success: true, data: response }
      } catch (error) {
        return { success: false, error: error.message }
      } finally {
        this.setLoading(false)
      }
    },

    // 重置store到初始状态
    resetStore() {
      this.user = {
        id: null,
        username: '',
        email: '',
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

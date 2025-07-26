import { createPinia, defineStore } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import { logout } from '../api/index'

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
    token: localStorage.getItem('token') || '',
    
    // 加载状态
    loading: false,
    
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
    
    // 是否为管理员
    isAdmin: (state) => state.user.role === 'admin',
    
    // 当前主题
    currentTheme: (state) => state.setting.theme,
    
    // 是否加载中
    isLoading: (state) => state.loading
  },

  actions: {
    // 设置用户信息
    setUser(userInfo) {
      this.user = { ...this.user, ...userInfo }
    },
    
    // 设置token
    setToken(token) {
      this.token = token
      if (token) {
        localStorage.setItem('token', token)
      } else {
        localStorage.removeItem('token')
      }
    },
    
    // 设置加载状态
    setLoading(status) {
      this.loading = status
    },
    
    // 更新设置
    updateSetting(settingPath, value) {
      const keys = settingPath.split('.')
      let current = this.setting
      
      for (let i = 0; i < keys.length - 1; i++) {
        if (!current[keys[i]]) {
          current[keys[i]] = {}
        }
        current = current[keys[i]]
      }
      
      current[keys[keys.length - 1]] = value
      
      // 保存到localStorage
      localStorage.setItem('userSetting', JSON.stringify(this.setting))
    },
    
    // 切换主题
    toggleTheme() {
      this.setting.theme = this.setting.theme === 'light' ? 'dark' : 'light'
      localStorage.setItem('userSetting', JSON.stringify(this.setting))
    },
    
    // 切换侧边栏
    toggleSidebar() {
      this.setting.sidebar.collapsed = !this.setting.sidebar.collapsed
      localStorage.setItem('userSetting', JSON.stringify(this.setting))
    },
    
    // 登录
    async login(credentials) {
      this.setLoading(true)
      try {
        // 这里调用API进行登录
        // const response = await loginApi(credentials)
        // 模拟API响应
        const response = {
          token: 'mock-jwt-token',
          user: {
            id: 1,
            username: credentials.username,
            email: credentials.email,
            avatar: '',
            role: 'user',
            createdAt: new Date().toISOString()
          }
        }
        
        this.setToken(response.token)
        this.setUser(response.user)
        
        return { success: true, data: response }
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
    
    // 初始化store（从localStorage恢复设置）
    initStore() {
      // 恢复设置
      const savedSetting = localStorage.getItem('userSetting')
      if (savedSetting) {
        try {
          this.setting = { ...this.setting, ...JSON.parse(savedSetting) }
        } catch (error) {
          console.warn('Failed to parse saved settings:', error)
        }
      }
      
      // 如果有token，尝试获取用户信息
      if (this.token) {
        this.fetchUserInfo()
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

<template>
    <div class="container">
        <!-- 输入框 -->
        <div class="input-section">
            <el-input ref="inputRef" v-model="inputValue" placeholder="输入时间戳或时间字符串"
                clearable size="small">
                <template #prefix>
                    <el-icon>
                        <Clock />
                    </el-icon>
                </template>
            </el-input>
        </div>

        <!-- 控制选项 -->
        <el-row :gutter="20" class="control-section">
            <el-col :span="8">
                <el-select v-model="timestampUnit" placeholder="选择时间戳单位" size="small" style="width: 100%">
                    <el-option label="s" value="seconds" />
                    <el-option label="ms" value="milliseconds" />
                </el-select>
            </el-col>
            <el-col :span="16">
                <el-select v-model="timezone" placeholder="选择时区" size="small" style="width: 100%">
                    <el-option label="CST/UTC+8" value="Asia/Shanghai" />
                    <el-option label="UTC/UTC+0" value="UTC" />
                    <el-option label="EST/UTC-5" value="America/New_York" />
                    <el-option label="PST/UTC-8" value="America/Los_Angeles" />
                    <el-option label="GMT/UTC+0" value="Europe/London" />
                    <el-option label="JST/UTC+9" value="Asia/Tokyo" />
                </el-select>
            </el-col>
        </el-row>

        <!-- 结果展示栏 -->
        <div class="result-section">
            <el-input v-model="resultValue" placeholder="转换结果" readonly size="small">
                <template #prefix>
                    <el-icon>
                        <AlarmClock />
                    </el-icon>
                </template>
                <template #append>
                    <el-button :icon="CopyDocument" @click="copyMessage(resultValue)"
                        :disabled="!resultValue || resultValue.includes('无效')">
                        复制
                    </el-button>
                </template>
            </el-input>
        </div>

        <!-- 提示信息 -->
        <el-alert v-if="resultValue && resultValue.includes('无效')" title="转换失败" type="error" :description="resultValue"
            show-icon :closable="false" class="error-alert" />
    </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { Clock, CopyDocument, AlarmClock } from '@element-plus/icons-vue'
import { copyMessage } from '../util'
import { closeWebWindow } from '../windows'

const inputValue = ref('')
const timestampUnit = ref('seconds')
const timezone = ref('Asia/Shanghai')
const resultValue = ref('')
const inputRef = ref(null)


// 监听输入变化
watch([inputValue, timestampUnit, timezone], () => {
    if (!inputValue.value.trim()) {
        resultValue.value = ''
        return
    }

    if (isTimestamp(inputValue.value)) {
        // 输入是时间戳,转换为时间字符串
        resultValue.value = timestampToDate(inputValue.value)
    } else {
        // 输入是时间字符串,转换为时间戳
        resultValue.value = dateToTimestamp(inputValue.value)
    }
})


// 2秒内鼠标没移动到页面上则执行 closeWebWindow，鼠标离开后重新计算2秒时间。
let closeTimer = null
const startCloseTimer = () => {
  clearCloseTimer()
  // 2 秒后关闭当前窗口（label 固定为 time-trans）
  closeTimer = window.setTimeout(() => {
    closeWebWindow('time-trans', true).catch((e) => {
      console.error('closeWebWindow failed:', e)
    })
  }, 1000)
}
const clearCloseTimer = () => {
  if (closeTimer) {
    clearTimeout(closeTimer)
    closeTimer = null
  }
}
const onMouseMove = () => {
  // 鼠标移动到页面则取消计时
  clearCloseTimer()
}
const onMouseLeave = () => {
  // 鼠标离开页面则重新开始 2 秒计时
  startCloseTimer()
}

// 组件挂载时
onMounted(() => {
    // 输入框自动聚焦
    if (inputRef.value) {
        inputRef.value.focus()
    }

    // 监听鼠标移动与离开
    window.addEventListener('mousemove', onMouseMove)
    document.addEventListener('mouseout', onMouseLeave)
    

    // 初始启动计时：2 秒内没有鼠标移动到此窗口则关闭
    startCloseTimer()
})

// 组件卸载时清理
onUnmounted(() => {
  clearCloseTimer()
  window.removeEventListener('mousemove', onMouseMove)
  document.removeEventListener('mouseout', onMouseLeave)
  
})



// 判断输入是时间戳还是时间字符串
const isTimestamp = (value) => {
    return /^\d+$/.test(value.trim())
}

// 时间戳转时间字符串
const timestampToDate = (timestamp) => {
    try {
        let ts = parseInt(timestamp)
        // 如果是秒,转换为毫秒
        if (timestampUnit.value === 'seconds') {
            ts = ts * 1000
        }

        const date = new Date(ts)

        // 使用 Intl.DateTimeFormat 格式化时间并应用时区
        const formatter = new Intl.DateTimeFormat('zh-CN', {
            timeZone: timezone.value,
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hour12: false
        })

        return formatter.format(date).replace(/\//g, '-')
    } catch (e) {
        return '无效的时间戳'
    }
}

// 时间字符串转时间戳
const dateToTimestamp = (dateString) => {
    try {
        // 解析时间字符串时考虑选择的时区
        // 将输入的时间字符串视为指定时区的时间
        const cleanDateString = dateString.trim()

        // 创建一个临时日期对象
        const tempDate = new Date(cleanDateString)
        if (isNaN(tempDate.getTime())) {
            return '无效的时间格式'
        }

        // 获取输入时间在指定时区的本地时间表示
        // 使用 Intl.DateTimeFormat 获取时区偏移
        const formatter = new Intl.DateTimeFormat('en-US', {
            timeZone: timezone.value,
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hour12: false
        })

        // 获取当前系统时区下的Date对象表示的UTC时间
        const localDate = new Date(cleanDateString)

        // 获取该时间在目标时区的字符串表示
        const parts = formatter.formatToParts(localDate)
        const tzYear = parts.find(p => p.type === 'year').value
        const tzMonth = parts.find(p => p.type === 'month').value
        const tzDay = parts.find(p => p.type === 'day').value
        const tzHour = parts.find(p => p.type === 'hour').value
        const tzMinute = parts.find(p => p.type === 'minute').value
        const tzSecond = parts.find(p => p.type === 'second').value

        // 计算时区偏移
        const tzDate = new Date(`${tzYear}-${tzMonth}-${tzDay}T${tzHour}:${tzMinute}:${tzSecond}`)
        const offset = localDate.getTime() - tzDate.getTime()

        // 将输入视为指定时区的本地时间,计算对应的UTC时间戳
        const utcTimestamp = localDate.getTime() - offset

        // 根据选择的单位返回秒或毫秒
        if (timestampUnit.value === 'seconds') {
            return Math.floor(utcTimestamp / 1000).toString()
        } else {
            return utcTimestamp.toString()
        }
    } catch (e) {
        return '无效的时间格式'
    }
}

</script>

<style scoped>
.container {
    width: 240px;
    padding: 10px 5px 0px 5px;
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
}

.card-header {
    display: flex;
    justify-content: center;
    align-items: center;
}

.current-time-section {
    margin-bottom: 20px;
    padding: 16px;
    background: #f4f4f5;
    border-radius: 8px;
    display: flex;
    align-items: center;
    gap: 10px;
}

.current-time-section .el-link {
    font-size: 18px;
    font-weight: 600;
    font-family: 'Consolas', 'Monaco', monospace;
}

.current-time-section .hint {
    font-size: 12px;
    color: #909399;
}

.current-time-section .summary-icon {
    font-size: 14px;
}

.input-section {
    margin-bottom: 5px;
}

.control-section {
    margin-bottom: 5px;
}

.result-section {
    margin-bottom: 16px;
}

.error-alert {
    margin-top: 16px;
}

:deep(.el-input__inner) {
    font-family: 'Consolas', 'Monaco', monospace;
}

</style>
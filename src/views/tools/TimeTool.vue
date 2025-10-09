<template>
    <div class="container">
        <el-card class="converter-card">
            <template #header>
                <div class="card-header">
                    <span class="title">时间戳转换工具</span>
                </div>
            </template>

            <!-- 当前时间戳 -->
            <div class="current-time-section">
                <span class="label">当前时间戳：</span>
                <el-link type="primary" @click="useCurrentTimestamp" underline="never">
                    {{ currentTimestamp }}
                </el-link>
                <el-link class="summary-icon" @click="copyMessage(currentTimestamp)"
                    icon="Document"></el-link>
            </div>

            <!-- 输入框 -->
            <div class="input-section">
                <el-input ref="inputRef" v-model="inputValue"
                    placeholder="输入时间戳或时间字符串,例如:1609459200或2021-01-01 00:00:00" clearable size="large">
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
                    <el-select v-model="timestampUnit" placeholder="选择时间戳单位" size="large" style="width: 100%">
                        <el-option label="秒 (s)" value="seconds" />
                        <el-option label="毫秒 (ms)" value="milliseconds" />
                    </el-select>
                </el-col>
                <el-col :span="16">
                    <el-select v-model="timezone" placeholder="选择时区" size="large" style="width: 100%">
                        <el-option label="中国标准时间 (CST) UTC+8" value="Asia/Shanghai" />
                        <el-option label="协调世界时 (UTC) UTC+0" value="UTC" />
                        <el-option label="美国东部时间 (EST) UTC-5" value="America/New_York" />
                        <el-option label="美国太平洋时间 (PST) UTC-8" value="America/Los_Angeles" />
                        <el-option label="英国时间 (GMT) UTC+0" value="Europe/London" />
                        <el-option label="日本标准时间 (JST) UTC+9" value="Asia/Tokyo" />
                    </el-select>
                </el-col>
            </el-row>

            <!-- 结果展示栏 -->
            <div class="result-section">
                <el-input v-model="resultValue" placeholder="转换结果将在这里显示" readonly size="large">
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
            <el-alert v-if="resultValue && resultValue.includes('无效')" title="转换失败" type="error"
                :description="resultValue" show-icon :closable="false" class="error-alert" />
        </el-card>
    </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { Clock, CopyDocument, AlarmClock } from '@element-plus/icons-vue'
import { ElMessage } from 'element-plus'
import { copyMessage } from '../../util'

const inputValue = ref('')
const timestampUnit = ref('seconds')
const timezone = ref('Asia/Shanghai')
const resultValue = ref('')
const currentTimestamp = ref('')
const inputRef = ref(null)

let timer = null

// 更新当前时间戳
const updateCurrentTimestamp = () => {
    const now = Date.now()
    if (timestampUnit.value === 'seconds') {
        currentTimestamp.value = Math.floor(now / 1000).toString()
    } else {
        currentTimestamp.value = now.toString()
    }
}

// 使用当前时间戳
const useCurrentTimestamp = () => {
    inputValue.value = currentTimestamp.value
}

// 组件挂载时
onMounted(() => {
    // 初始化当前时间戳
    updateCurrentTimestamp()

    // 每秒更新一次时间戳
    timer = setInterval(updateCurrentTimestamp, 1000)

    // 输入框自动聚焦
    if (inputRef.value) {
        inputRef.value.focus()
    }
})

// 组件卸载时清除定时器
onUnmounted(() => {
    if (timer) {
        clearInterval(timer)
    }
})

// 监听时间戳单位变化,更新当前时间戳显示
watch(timestampUnit, () => {
    updateCurrentTimestamp()
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

</script>

<style scoped>
.container {
    max-width: 450px;
    margin: 60px auto;
    padding: 30px;
}

.converter-card {
    border-radius: 12px;
    box-shadow: 0 4px 20px 0 rgba(0, 0, 0, 0.12);
}

.card-header {
    display: flex;
    justify-content: center;
    align-items: center;
}

.title {
    font-size: 24px;
    font-weight: 600;
    color: #303133;
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

.current-time-section .label {
    font-size: 14px;
    color: #606266;
    font-weight: 500;
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
    margin-bottom: 24px;
}

.control-section {
    margin-bottom: 24px;
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

:deep(.el-card__body) {
    padding: 30px;
}

:deep(.el-card__header) {
    padding: 24px 30px;
}
</style>
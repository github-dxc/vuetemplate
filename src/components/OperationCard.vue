<template>
    <div class="operation-card">
        <div class="floating-elements">
            <div class="floating-circle" :style="getRandomPosition(1)"></div>
            <div class="floating-circle" :style="getRandomPosition(2)"></div>
        </div>
        
        <div class="card-content">
            <div class="card-header">
                <div class="user-info">
                    <div class="avatar" :style="{backgroundColor: getColorByUnicPalette(getFirstChar(username)).backgroundColor}">
                        <div :style="{color: getColorByUnicPalette(getFirstChar(username)).textColor}">{{ getFirstChar(username) }}</div>
                    </div>
                    <h3 class="user-name">{{ username }}</h3>
                    <div v-if="bug_id" class="bug-link" @click.stop>
                        <el-link 
                            type="primary"
                            class="bug-id-link"
                            @click="browserOpen(`/view.php?id=${bug_id}`)"
                        >
                            #{{ bug_id }}
                        </el-link>
                        <el-link class="summary-icon" @click="copyMessage(`http://${host}/view.php?id=${bug_id}`)" icon="Document"></el-link>
                    </div>
                </div>

                <div class="timestamp">
                    <svg class="timestamp-icon" viewBox="0 0 24 24" fill="currentColor">
                        <path
                            d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z" />
                        <path d="M13 7h-2v5.414l3.293 3.293 1.414-1.414L13 11.586z" />
                    </svg>
                    {{ timestr }}
                </div>
            </div>

            <div class="operations-container" ref="contentRef">
                <div class="operations-list">
                    <div v-for="(operation, index) in operations" :key="index" class="operation-item">
                        <div class="operation-icon"></div>
                        <div class="operation-text"><div v-html="operation"></div></div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup vapor>
import { onBeforeUnmount, onMounted, ref } from 'vue';
import { getColorByUnicPalette, getFirstChar, copyMessage } from '../util';
import { browserOpen } from '../api';

const props = defineProps({
    bug_id: {
        type: Number,
        required: false
    },
    host: {
        type: String,
        required: false,
        default: ''
    },
    username: {
        type: String,
        required: true
    },
    timestr: {
        type: String,
        required: true
    },
    timestamp:{
        type: Number,
        required: true
    },
    operations: {
        type: Array,
        required: true
    }
});

const contentRef = ref(null);

const getRandomPosition = (index) => {
    const top = Math.random() * 80 + 10; // 10% 到 90% 之间
    const left = Math.random() * 80 + 10;
    const size = index === 1 ? 60 : 40; // 第一个圆圈 60px，第二个 40px

    return {
        top: `${top}%`,
        left: `${left}%`,
        width: `${size}px`,
        height: `${size}px`,
        animationDelay: `${-index * 2}s`
    };
};

onMounted(() => {
    // 监听内容区域的a标签点击事件，传递到外面用外部浏览器打开
    const clickHandler = (e) => {
        const a = e.target.closest('a');
        //如果a标签是editor-container的子元素且不是.ql-preview，则不处理
        if (a && e.target.closest('.editor-container') && !e.target.closest('.ql-preview')) {
            return;
        }
        if (a) {
            //只获取a的路径部分
            const uri = a.href.split('localhost:8080')[1] || a.href;
            e.preventDefault(); // 阻止默认行为
            browserOpen(uri);
        }
    };
    contentRef.value.addEventListener('click', clickHandler);
    onBeforeUnmount(() => {
    contentRef.value?.removeEventListener('click', clickHandler);
    });
});
</script>

<style scoped>
.summary-icon {
  color: #10b981;
  flex-shrink: 0;
}

.bug-id-link {
  font-weight: 600;
  font-family: 'Monaco', 'Menlo', monospace;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  margin: 0 2px;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  text-decoration: none;
}

.operation-card {
    width: 680px;
    background: rgba(255, 255, 255, 0.95);
    backdrop-filter: blur(10px);
    border-radius: 20px;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
    border: 1.5px solid rgba(226, 232, 240, 0.8);
    overflow: hidden;
    position: relative;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.operation-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 30px 60px rgba(0, 0, 0, 0.15);
}

.card-content {
    padding: 20px 24px;
    height: 100%;
    display: flex;
    flex-direction: column;
}

.card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
}

.user-info {
    display: flex;
    align-items: center;
    gap: 12px;
}

.avatar {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    /* background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); */
    display: flex;
    align-items: center;
    justify-content: center;
    color: white;
    font-weight: 600;
    font-size: 16px;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.user-name {
    font-size: 18px;
    font-weight: 600;
    color: #334155;
    margin: 0;
}

.timestamp {
    display: flex;
    align-items: center;
    gap: 6px;
    color: #64748b;
    font-size: 13px;
    background: rgba(148, 163, 184, 0.1);
    padding: 6px 10px;
    border-radius: 16px;
}

.timestamp-icon {
    width: 12px;
    height: 12px;
    fill: currentColor;
}

.operations-container {
    flex: 1;
    overflow: hidden;
}

.operations-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    max-height: 200px;
    overflow-y: auto;
}

.operations-list::-webkit-scrollbar {
    width: 3px;
}

.operations-list::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 2px;
}

.operations-list::-webkit-scrollbar-thumb {
    background: #cbd5e1;
    border-radius: 2px;
}

.operation-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    background: rgba(255, 255, 255, 0.6);
    border-radius: 8px;
    border: 1px solid rgba(226, 232, 240, 0.5);
}

.operation-item:nth-child(even) {
    background: rgba(248, 250, 252, 0.8);
}

.operation-icon {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #94a3b8;
    flex-shrink: 0;
}

.operation-text {
    flex: 1;
    font-size: 14px;
    color: #475569;
    line-height: 1.4;
}

.floating-elements {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    pointer-events: none;
    overflow: hidden;
}

.floating-circle {
    position: absolute;
    border-radius: 50%;
    background: rgba(102, 126, 234, 0.1);
    animation: float 6s ease-in-out infinite;
}

@keyframes float {
    0%, 100% {
        transform: translateY(0px) rotate(0deg);
    }
    33% {
        transform: translateY(-10px) rotate(120deg);
    }
    66% {
        transform: translateY(5px) rotate(240deg);
    }
}
</style>
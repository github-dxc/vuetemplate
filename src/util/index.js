import { ElMessage } from "element-plus";

export const byteArrayToBase64Image = function(byteArray, fileName) {
  // A simple mapping of file extensions to MIME types.
  // You can expand this list as needed.
  const mimeMap = {
    'jpeg': 'image/jpeg',
    'jpg': 'image/jpeg',
    'png': 'image/png',
    'gif': 'image/gif',
    'bmp': 'image/bmp',
    'webp': 'image/webp',
    'svg': 'image/svg+xml'
  };

  // Get the file extension from the file name.
  const fileExtension = fileName.split('.').pop().toLowerCase();

  // Determine the MIME type using the map. Default to 'application/octet-stream' if not found.
  const mimeType = mimeMap[fileExtension] || 'application/octet-stream';

  // Ensure byteArray is a Uint8Array.
  const uint8Array = new Uint8Array(byteArray);

  // Convert the Uint8Array to a binary string.
  let binaryString = '';
  for (let i = 0; i < uint8Array.length; i++) {
    binaryString += String.fromCharCode(uint8Array[i]);
  }

  // Convert the binary string to Base64.
  const base64 = btoa(binaryString);

  // Return the complete data URL.
  return `data:${mimeType};base64,${base64}`;
}

// 格式化时间戳
export const formatDate = (timestamp) => {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  });
};

export const formatDateDay = (timestamp) => {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
  });
};

export const getFirstChar = function(name) {
  if (!name || typeof name !== 'string') {
    return '';
  }
  return name.trim().charAt(0);
}

// 获取一个字符的颜色：预定义调色板（确保颜色美观）
export const getColorByUnicPalette = function(unic) {
  // 定义颜色对：每个数组包含 [深色(字体), 浅色(背景)]
  const colorPairs = [
    ['#D32F2F', '#FFEBEE'], // 红色系
    ['#1976D2', '#E3F2FD'], // 蓝色系
    ['#388E3C', '#E8F5E8'], // 绿色系
    ['#F57C00', '#FFF3E0'], // 橙色系
    ['#7B1FA2', '#F3E5F5'], // 紫色系
    ['#0097A7', '#E0F2F1'], // 青色系
    ['#5D4037', '#EFEBE9'], // 棕色系
    ['#C62828', '#FFEBEE'], // 深红色系
    ['#303F9F', '#E8EAF6'], // 靛蓝色系
    ['#689F38', '#F1F8E9'], // 浅绿色系
    ['#E65100', '#FFF3E0'], // 深橙色系
    ['#8E24AA', '#F3E5F5'], // 深紫色系
    ['#00695C', '#E0F2F1'], // 深青色系
    ['#4E342E', '#EFEBE9'], // 深棕色系
    ['#AD1457', '#FCE4EC'], // 粉红色系
    ['#1565C0', '#E1F5FE'], // 浅蓝色系
    ['#2E7D32', '#E8F5E8'], // 深绿色系
    ['#EF6C00', '#FFF8E1'], // 琥珀色系
    ['#6A1B9A', '#F3E5F5'], // 中紫色系
    ['#00838F', '#E0F7FA'], // 浅青色系
    ['#FF8F00', '#FFFDE7'], // 黄色系
    ['#C2185B', '#FCE4EC'], // 深粉色系
    ['#1976D2', '#E3F2FD'], // 标准蓝色系
    ['#388E3C', '#E8F5E8'], // 标准绿色系
    ['#F57C00', '#FFF3E0'], // 标准橙色系
  ];
  
  if (!unic) return { textColor: colorPairs[0][0], backgroundColor: colorPairs[0][1] };
  
  // 计算哈希值
  let hash = 0;
  for (let i = 0; i < unic.length; i++) {
    hash = unic.charCodeAt(i) + ((hash << 5) - hash);
  }
  
  // 根据哈希值选择颜色对
  const index = Math.abs(hash) % colorPairs.length;
  const [textColor, backgroundColor] = colorPairs[index];
  
  return {
    textColor,
    backgroundColor
  };
}


// 复制数据到剪切板
export const copyMessage = (message) => {
  navigator.clipboard.writeText(message).then(() => {
    ElMessage({
      message: '复制成功',
      type: 'success',
    })
  }).catch(err => {
    console.error("复制失败:", err);
  });
}

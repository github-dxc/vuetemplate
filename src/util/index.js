import { ElMessage, ElMessageBox } from 'element-plus'

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

export const getFirstChar = function(name) {
  if (!name || typeof name !== 'string') {
    return '';
  }
  return name.trim().charAt(0);
}
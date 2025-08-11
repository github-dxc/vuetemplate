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
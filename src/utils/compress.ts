/**
 * 将图片Data URI转为Image对象
 * @param dataUri 图片的Data URI
 * @returns  Image对象
 */
function dataURLToImage(dataUri: string): Promise<HTMLImageElement> {
  return new Promise((resolve) => {
    const image = new Image();
    image.src = dataUri;
    image.onload = () => resolve(image);
  });
}

function canvasToFile(
  canvas: HTMLCanvasElement,
  type: string,
  quality: number
): Promise<Blob | null> {
  return new Promise((resolve) => {
    canvas.toBlob(
      (blob) => {
        resolve(blob);
      },
      type,
      quality
    );
  });
}

/**
 * 将图片文件转换为Data URI
 * @param file 图片文件
 * @returns 图片的Data URI
 */
export function fileToDataURL(file: Blob): Promise<any> {
  return new Promise((resolve) => {
    const reader = new FileReader();
    reader.onload = (e) => resolve((e.target as FileReader).result);
    reader.readAsDataURL(file);
  });
}

/**
 * 压缩图片
 * @param file 需要压缩的图片文件
 * @param quality 压缩效率（值越小压缩效率越好，0.2左右时可以压缩最大化，且对图片质量影响不会太大）
 * @returns
 */
export async function compressImage(
  file: File,
  quality: number = 0.5
): Promise<File> {
  const canvas = document.createElement("canvas");
  const context = canvas.getContext("2d") as CanvasRenderingContext2D;
  const base64 = await fileToDataURL(file);
  const img = await dataURLToImage(base64);
  canvas.width = img.width;
  canvas.height = img.height;
  context.clearRect(0, 0, img.width, img.height);
  context.drawImage(img, 0, 0, img.width, img.height);

  const type: string = "image/jpeg"; // 图片格式为type时压缩效果最好
  const blob = (await canvasToFile(canvas, type, quality)) as Blob; // quality:0.5可根据实际情况计算

  let filename = file.name.substring(0, file.name.lastIndexOf(".") + 1);
  let suffix = type.substring(type.lastIndexOf("/") + 1);
  filename = `${filename}${suffix}`;
  return new File([blob], filename, {
    type: type,
  });
}

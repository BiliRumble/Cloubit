/**
 * 将网络图片转换为base64
 *
 * @param img 网络图片地址
 *
 * @returns base64编码
 */
export function imageToBase64(img: string): Promise<string> {
	return new Promise((resolve, reject) => {
		const image = new Image();
		image.crossOrigin = 'anonymous';
		image.src = img;
		image.onload = () => {
			const canvas = document.createElement('canvas');
			canvas.width = image.width;
			canvas.height = image.height;
			const ctx = canvas.getContext('2d');
			if (ctx) {
				ctx.drawImage(image, 0, 0, image.width, image.height);
				resolve(canvas.toDataURL('image/png'));
			} else {
				reject(new Error('canvas error'));
			}
		};
		image.onerror = () => {
			reject(new Error('image error'));
		};
	});
}

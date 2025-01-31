import { LoginCheckQRResult, LoginCreateQRResult, LoginQRKeyResult } from '../models/auth';
import request from '../utils/request';

const { get } = request;

/**
 * 获取二维码key
 *
 * @returns LoginQRKeyResult | null
 */
async function createQRKey(): Promise<LoginQRKeyResult | null> {
	const response = (await get('login/qr/key', { key: Math.random().toString(36).substr(2, 15) }))
		.data as LoginQRKeyResult;

	if (response.code === 200) {
		console.debug('🌐 Get QR Key Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Get QR Key!');
	return null;
}

/**
 * 获取二维码
 *
 * @returns LoginCreateQRResult | null
 */
export async function createQR(): Promise<LoginCreateQRResult | null> {
	const code = await createQRKey();
	if (code === null) {
		console.error('🌐 QR Key is null!');
		return null;
	}

	const unikey = code.data.unikey;
	const response = (await get('login/qr/create', { key: unikey, qrimg: true }))
		.data as LoginCreateQRResult;

	if (response.code === 200) {
		console.debug('🌐 Get QR Code Success: ', response);
		sessionStorage.setItem('unikey', unikey);
		return response;
	}
	console.error('🌐 Cannot Get QR Code!');
	return null;
}

/**
 * 检查二维码
 *
 * @returns LoginCheckQRResult | null
 */
export async function checkQR(): Promise<LoginCheckQRResult | null> {
	const unikey = sessionStorage.getItem('unikey');
	if (unikey === null) {
		console.error('🌐 QR Key is null!');
		return null;
	}

	const response = (await get('login/qr/check', { key: unikey, timestamp: Date.now() }))
		.data as LoginCheckQRResult;

	// 只要是8xx就表示成功获取
	if (response.data.code >= 800 && response.data.code <= 900) {
		console.debug('🌐 QR Code Checked Success: ', response);
		return response;
	}
	console.error('🌐 Cannot Check QR Code!');
	return null;
}

/**
 * 登出
 *
 * @return boolean
 * */
export async function logout(): Promise<boolean> {
	const response = (await get('logout')).data as any;
	console.debug(response);
	return response.code === 200;
}

/**
 * 获取登录状态
 *
 * @todo 实现
 *
 * @returns any | null
 * */
export async function getLoginStatus(): Promise<any | null> {
	const response = (await get('login/status')).data;
	console.debug(response);
	return response;
}

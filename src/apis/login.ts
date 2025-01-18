import { LoginCheckQRResult, LoginCreateQRResult, LoginQRKeyResult } from '../models/main';
import request from '../utils/request';

const { get } = request;

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

export async function getLoginStatus(): Promise<any | null> {
	const response = (await get('login/status')).data;
	console.debug(response);
	return response;
}

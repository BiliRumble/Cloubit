import { invoke } from '@tauri-apps/api/core';

/**
 * 登出
 *
 * @returns any | null
 */
export async function logout(): Promise<any> {
	const response = await invoke('api_logout');
	console.debug(response);
	return response;
}

/**
 * 获取登录状态
 *
 * @todo 实现
 *
 * @returns any | null
 * */
export async function getLoginStatus(): Promise<any | null> {
	const response = await invoke('api_login_status');
	console.debug(response);
	return response;
}

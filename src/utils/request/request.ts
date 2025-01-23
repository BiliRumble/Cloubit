import axios from 'axios';
import { useAuthStore } from '../../store/auth';

// 创建新的axios实例
const service = axios.create({
	baseURL: 'http://localhost:8080',
	timeout: 5000,
	withCredentials: true,
});

// 添加real_ip param
service.interceptors.request.use((config) => {
	const isLogin = useAuthStore.getState().isLogin;
	const authCookieRaw = isLogin
		? (useAuthStore.getState().cookie as { [key: string]: string })['MUSIC_U']
		: '';
	const cookie = isLogin ? btoa(JSON.stringify({ MUSIC_U: authCookieRaw })) : null;
	config.params = {
		...config.params,
		real_ip: '116.25.146.177',
		cookie,
	};
	return config;
});

export default service;

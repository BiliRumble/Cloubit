import axios from 'axios';
import { useAuthStore } from '../../store/auth';
import { useSettingStore } from '../../store/setting';

// 创建新的axios实例
const service = axios.create({
	baseURL: 'http://localhost:3000',
	timeout: 5000,
	withCredentials: true,
});

service.interceptors.request.use((config) => {
	config.params = {
		...config.params,
		real_ip: useSettingStore.getState().realIP,
	};
	return config;
});

export default service;

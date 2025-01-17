import axios from 'axios';

// 创建新的axios实例
const service = axios.create({
	baseURL: 'http://localhost:8080',
	timeout: 5000,
	withCredentials: true,
});

export default service;

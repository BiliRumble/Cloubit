import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useEffect, useState } from 'react';
import LazyImage from '@/components/atoms/Image';
import styles from './Login.module.scss';

interface QrInfo {
	avatarUrl: string;
	nickname: string;
}

interface LoginResponse {
	code: number;
	data: {
		unikey: string;
		qrUrl: string;
	};
}

interface QrStatusEvent {
	code: number;
	message: string;
	status: 'waiting' | 'scanned' | 'success' | 'expired' | 'error' | 'unknown';
	data?: any;
}

interface LoginCompleteEvent {
	code: number;
	message: string;
	cookie: string;
	userInfo: any;
}

const Login = () => {
	const [qrimg, setQrimg] = useState('');
	const [stage, setState] = useState(0); // 0: 等待扫码, 1: 已扫码等待确认, 2: 登录成功
	const [info, setInfo] = useState<QrInfo | null>(null);
	const [loading, setLoading] = useState(false);
	const [error, setError] = useState<string | null>(null);

	useEffect(() => {
		let unlistenQrStatus: (() => void) | null = null;
		let unlistenLoginComplete: (() => void) | null = null;

		const initLogin = async () => {
			try {
				setLoading(true);
				setError(null);

				// 监听登录状态变化事件
				unlistenQrStatus = await listen<QrStatusEvent>('qr-status', (event: any) => {
					console.log('登录状态更新:', event.payload);
					const { status, message, data } = event.payload;

					switch (status) {
						case 'waiting':
							setState(0);
							setInfo(null);
							setError(null);
							break;

						case 'scanned':
							setState(1);
							setError(null);

							// 从返回数据中提取用户信息
							if (data && data.profile) {
								const profile = data.profile;
								setInfo({
									avatarUrl: profile.avatarUrl || '',
									nickname: profile.nickname || '用户',
								});
							} else {
								// 如果没有用户信息，使用默认显示
								setInfo({
									avatarUrl: '',
									nickname: '用户',
								});
							}
							break;

						case 'success':
							setState(2);
							setError(null);
							break;

						case 'expired':
							setState(0);
							setInfo(null);
							setError('二维码已过期，正在重新生成...');
							// 自动重新开始登录流程
							setTimeout(() => {
								startLogin();
							}, 2000);
							break;

						case 'error':
							setState(0);
							setInfo(null);
							setError(message || '登录出错，请重试');
							break;

						case 'unknown':
							console.warn('未知登录状态:', data);
							break;
					}
				});

				// 监听登录完成事件
				unlistenLoginComplete = await listen<LoginCompleteEvent>(
					'login-complete',
					(event: any) => {
						console.log('登录完成:', event.payload);
						const { userInfo } = event.payload;

						setState(2);

						// 更新用户信息
						if (userInfo && userInfo.avatarUrl && userInfo.nickname) {
							setInfo({
								avatarUrl: userInfo.avatarUrl,
								nickname: userInfo.nickname,
							});
						}

						// 登录成功后的处理
						setTimeout(() => {
							console.log('登录成功，准备跳转到主页面...');
							// 这里可以添加页面跳转逻辑
							// 例如：navigate('/home');
						}, 2000);
					}
				);

				// 启动登录流程
				await startLogin();
			} catch (err) {
				console.error('初始化登录失败:', err);
				setError('初始化登录失败，请重试');
				setLoading(false);
			}
		};

		const startLogin = async () => {
			try {
				setLoading(true);
				setError(null);

				// 调用后端登录接口
				const response = await invoke<LoginResponse>('api_login');
				console.log('登录流程启动:', response);

				// 从返回数据中提取二维码图片
				if (response.code === 200 && response.data.qrUrl) {
					setQrimg(response.data.qrUrl);
					setLoading(false);
				} else {
					throw new Error('无法获取二维码数据');
				}
			} catch (err) {
				console.error('启动登录失败:', err);
				setError(typeof err === 'string' ? err : '启动登录失败，请重试');
				setLoading(false);
			}
		};

		// 初始化登录
		initLogin();

		// 清理函数
		return () => {
			if (unlistenQrStatus) unlistenQrStatus();
			if (unlistenLoginComplete) unlistenLoginComplete();
		};
	}, []);

	// 手动重试登录
	const handleRetry = async () => {
		try {
			setLoading(true);
			setError(null);
			setState(0);
			setInfo(null);

			const response = await invoke<LoginResponse>('api_login');
			console.log('重新启动登录流程:', response);

			if (response.code === 200 && response.data.qrUrl) {
				setQrimg(response.data.qrUrl);
				setLoading(false);
			}
		} catch (err) {
			console.error('重试登录失败:', err);
			setError('重试失败，请稍后再试');
			setLoading(false);
		}
	};

	// 根据stage显示不同的提示信息
	const getStatusText = () => {
		switch (stage) {
			case 0:
				return loading ? '正在生成二维码...' : '请使用网易云音乐APP扫描二维码';
			case 1:
				return `等待 ${info?.nickname || '用户'} 授权...`;
			case 2:
				return '登录成功！';
			default:
				return '请扫描二维码登录';
		}
	};

	return (
		<div className={styles.login}>
			<div className={styles.login__qrbox + ` ${stage === 2 ? 'success' : ''}`}>
				{loading && !qrimg ? (
					<div className="loading-placeholder">
						<p>正在生成二维码...</p>
					</div>
				) : (
					<LazyImage src={qrimg} alt="登录二维码" />
				)}

				<div
					className={styles.login__qrbox__tip}
					style={{ display: stage > 0 ? 'flex' : 'none' }}
				>
					{info?.avatarUrl && (
						<LazyImage
							src={info.avatarUrl}
							alt={(info.nickname || '') + '的头像'}
							className={styles.login__qrbox__tip__avatar}
						/>
					)}
					<p>等待 {info?.nickname} 授权...</p>
				</div>

				{/* 错误提示和重试按钮 */}
				{error && (
					<div className={styles.login__error}>
						<p>{error}</p>
						{!loading && (
							<button onClick={handleRetry} className={styles.login__retry}>
								重试
							</button>
						)}
					</div>
				)}

				{/* 状态提示 */}
				{!error && stage === 0 && qrimg && !loading && (
					<div className={styles.login__status}>
						<p>{getStatusText()}</p>
					</div>
				)}

				{/* 登录成功提示 */}
				{stage === 2 && (
					<div className={styles.login__success}>
						<p>✓ 登录成功！</p>
					</div>
				)}
			</div>
		</div>
	);
};

export default Login;

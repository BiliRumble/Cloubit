import { useEffect, useState } from 'react';
import { checkQR, createQR, getLoginStatus } from '../../../apis/login';
import { getUserAccount } from '../../../apis/user';
import { useAuthStore } from '../../../store/auth';
import { setCookies } from '../../../utils/cookie';
import styles from './Login.module.scss';

interface QrInfo {
	avatarUrl: string;
	nickname: string;
}

const Login = () => {
	const [qr, setQr] = useState('');
	// 0 等待 | 1 扫码 | 2 登录
	const [qrStatus, setQrStatus] = useState(0);
	const [qrInfo, setQrInfo] = useState<QrInfo | null>(null);

	// 解析cookie字符串的函数
	const parseCookies = (cookieString: string): { [key: string]: string } => {
		const cookies: { [key: string]: string } = {};
		cookieString.split(';').forEach((cookie) => {
			const [key, value] = cookie.split('=').map((x) => x.trim());
			cookies[key] = value;
		});
		return cookies;
	};

	useEffect(() => {
		createQR().then((res) => {
			setQr(res?.data.qrimg || '');
		});
		// 轮询 - 4s
		const timer = setInterval(() => {
			// 添加标记
			checkQR().then(async (res) => {
				switch (res?.data.code) {
					case 800:
						createQR().then((res) => setQr(res?.data.qrimg || ''));
						break;
					case 801:
						break;
					case 802:
						setQrStatus(1);
						setQrInfo(res?.data as QrInfo);
						break;
					case 803:
						setQrStatus(2);
						clearInterval(timer);
						if (res.data.cookie && res.data.cookie.includes('MUSIC_U')) {
							const parsedCookie = parseCookies(res.data.cookie);
							useAuthStore.setState({
								isLogin: true,
								cookie: parsedCookie,
							});
							setCookies(res.data.cookie);

							if (useAuthStore.getState().isLogin) {
								await getUserAccount().then((res) => {
									useAuthStore.setState({ userData: res });
								});
								window.location.reload();
							}
						} else {
							console.error('登录失败，未获取到cookie');
						}
						getLoginStatus();
						break;
					default:
						break;
				}
			});
		}, 5000);
	}, []);

	return (
		<div className={styles.login}>
			<div className={styles.login__qrbox + ` ${''}`}>
				<img src={qr} alt="qr" />
				<div
					className={styles.login__qrbox__tip}
					style={{ display: qrStatus > 0 ? 'block' : 'none' }}
				>
					<div className={styles.login__qrbox__mask__info}>
						<p>等待 {qrInfo?.nickname} 授权...</p>
					</div>
				</div>
			</div>
		</div>
	);
};

export default Login;

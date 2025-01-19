import { useEffect, useState } from 'react';
import { checkQR, createQR, getLoginStatus } from '../../../apis/login';
//import { useAuthStore } from '../../../store/auth';
import { getCookie, setCookies } from '../../../utils/cookie';
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

	//const authStore = useAuthStore();

	useEffect(() => {
		createQR().then((res) => {
			setQr(res?.data.qrimg || '');
		});
		// 轮询 - 4s
		const timer = setInterval(() => {
			// 添加标记
			checkQR().then((res) => {
				switch (res?.data.code) {
					case 800:
						// 更新qr
						createQR().then((res) => setQr(res?.data.qrimg || ''));
						console.debug('更新qr', qr);
						break;
					case 801:
						// 等待扫码
						break;
					case 802:
						// 扫码成功, 等待授权
						setQrStatus(1);
						setQrInfo(res?.data as QrInfo);
						break;
					case 803:
						// 登录成功
						setQrStatus(2);
						clearInterval(timer);
						if (res.data.cookie && res.data.cookie.includes('MUSIC_U')) {
							setCookies(res.data.cookie);
							console.log(getCookie('MUSIC_U'));
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

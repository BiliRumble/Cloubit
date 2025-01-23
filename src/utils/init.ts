import { getUserAccount } from '../apis/user';
import { useAuthStore } from '../store/auth';

export async function init() {
	const isLogin = useAuthStore.getState().isLogin;
	//const userData = useAuthStore().userData;

	if (isLogin) {
		await getUserAccount().then((res) => {
			useAuthStore.setState({ userData: res });
		});
	}
}

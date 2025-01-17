import Cookies from 'js-cookie';

// 获取 Cookie
export const getCookie = (key: string) => {
	return Cookies.get(key) ?? localStorage.getItem(`cookie-${key}`);
};

// 移除 Cookie
export const removeCookie = (key: string) => {
	Cookies.remove(key);
	localStorage.removeItem(`cookie-${key}`);
};

// 设置 Cookie
export const setCookies = (cookieValue: string) => {
	const cookies = cookieValue.split(';');
	const date = new Date();
	date.setFullYear(date.getFullYear() + 50);
	const expires = `expires=${date.toUTCString()}`;

	cookies.forEach((cookie) => {
		const cookieParts = cookie.split(';');
		const nameValuePair = cookieParts[0].split('=');
		const name = nameValuePair[0]?.trim();
		const value = nameValuePair[1]?.trim();
		console.info(`name: ${name}, value: ${value}`);
		document.cookie = `${name}=${value}; ${expires}; path=/`;
		localStorage.setItem(`cookie-${name}`, value);
	});
};

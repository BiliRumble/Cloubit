export type LoginQRKeyResult = {
	code: number;
	data: {
		code: number;
		unikey: string;
	};
};

export type LoginCreateQRResult = {
	code: number;
	data: {
		code: number;
		qrimg: string;
		qrurl: string;
	};
};

export type LoginCheckQRResult = {
	code: number;
	message: string;
	cookie: string;
	nickname?: string;
	avatarUrl?: string;
};

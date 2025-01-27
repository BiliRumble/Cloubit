export interface MusicFile {
	br: number;
	canExtend: boolean;
	channelLayout: string | null;
	closedGain: number;
	closedPeak: number;
	code: number;
	effectTypes: string | null;
	encodeType: string;
	expi: number;
	fee: number;
	flag: number;
	freeTimeTrialPrivilege: {
		remainTime: number;
		resConsumable: boolean;
		type: number;
		userConsumable: boolean;
	};
	freeTrialInfo: string | null;
	freeTrialPrivilege: {
		cannotListenReason: string | null;
		freeLimitTagType: string | null;
		listenType: string | null;
		playReason: string | null;
		resConsumable: boolean;
		userConsumable: boolean;
	};
	gain: number;
	id: number;
	level: string;
	levelConfuse: string | null;
	md5: string;
	message: string | null;
	musicId: string;
	payed: number;
	peak: number;
	podcastCtrp: string | null;
	rightSource: number;
	size: number;
	time: number;
	type: string;
	uf: string | null;
	url: string;
	urlSource: number;
}

export interface getSongURLResult {
	code: number;
	data: MusicFile[];
}

// 播放列表
export interface PlayList {
	count: number;
	data: PlayListItem[];
}

export interface PlayListItem {
	index: number;
	id: number;
	source: number;
	name: string;
	cover?: string;
	artists?: string[];
}

// 歌词
export interface Lyric {
	code: number;
	klyric?: LyricContent; // ?
	lrc: LyricContent; // 原歌词
	qfy?: boolean;
	romalrc?: LyricContent; // ?
	sfy?: boolean;
	sgc?: boolean;
	tlyric?: LyricContent; // 翻译
}

// lyric内容
export interface LyricContent {
	lyric: string; // [00:00.000] 内容
	version: number;
}

export interface DailySong {
	a: any;
	al: {
		id: number;
		name: string;
		pic: number;
		picUrl: string;
		pic_str: string;
		tns: string[];
	};
	alg: string;
	alia: string[];
	ar: {
		alias: string[];
		id: number;
		name: string;
		tns: string[];
	}[];
	cd: string;
	cf: string;
	copyright: number;
	cp: number;
	crbt: any;
	djId: number;
	dt: number;
	entertainmentTags: any;
	fee: number;
	ftype: number;
	h: {
		br: number;
		fid: number;
		size: number;
		sr: number;
		vd: number;
	};
	hr: any;
	id: number;
	l: {
		br: number;
		fid: number;
		size: number;
		sr: number;
		vd: number;
	};
	m: {
		br: number;
		fid: number;
		size: number;
		sr: number;
		vd: number;
	};
	mark: number;
	mst: number;
	mv: number;
	name: string;
	no: number;
	noCopyrightRcmd: any;
	originCoverType: number;
	originSongSimpleData: any;
	pop: number;
	privilege: {
		chargeInfoList: {
			chargeMessage: any;
			chargeType: number;
			chargeUrl: any;
			rate: number;
		}[];
		code: number;
		cp: number;
		cs: boolean;
		dl: number;
		dlLevel: string;
		downloadMaxBrLevel: string;
		downloadMaxbr: number;
		fee: number;
		fl: number;
		flLevel: string;
		flag: number;
		freeTrialPrivilege: {
			cannotListenReason: any;
			freeLimitTagType: any;
			listenType: any;
			playReason: any;
			resConsumable: boolean;
			userConsumable: boolean;
		};
		id: number;
		maxBrLevel: string;
		maxbr: number;
		message: any;
		paidBigBang: boolean;
		payed: number;
		pc: any;
		pl: number;
		plLevel: string;
		playMaxBrLevel: string;
		playMaxbr: number;
		preSell: boolean;
		realPayed: number;
		rightSource: number;
		rscl: any;
		sp: number;
		st: number;
		subp: number;
		toast: boolean;
	};
	pst: number;
	publishTime: number;
	reason: any;
	recommendReason: any;
	resourceState: boolean;
	rt: string;
	rtUrl: any;
	rtUrls: any[];
	rtype: number;
	rurl: any;
	s_id: number;
	single: number;
	songJumpInfo: any;
	sq: {
		br: number;
		fid: number;
		size: number;
		sr: number;
		vd: number;
	};
	st: number;
	t: number;
	tagPicList: any;
	tns: string[];
	v: number;
	version: number;
	videoInfo: {
		moreThanOne: boolean;
		video: {
			alias: any;
			artists: any;
			coverUrl: string;
			playTime: number;
			publishTime: number;
			title: string;
			type: number;
			vid: string;
		};
	};
}

export interface RecommendReason {
	reason: string;
	reasonId: string;
	songId: number;
	targetUrl: any;
}

export interface DailySongsResult {
	code: number;
	data: {
		dailySongs: DailySong[];
		demote: boolean;
		mvResourceInfos: any;
		orderSongs: any[];
		recommendReasons: RecommendReason[];
	};
}

interface RecommendItem {
	alg: string;
	copywriter: string;
	createTime: number;
	creator: {
		avatarImgId: number;
		avatarImgIdStr: string;
		avatarUrl: string;
		backgroundImgId: number;
		backgroundImgIdStr: string;
		backgroundUrl: string;
		birthday: number;
		city: number;
		defaultAvatar: boolean;
		description: string;
		followed: boolean;
		gender: number;
		mutual: boolean;
		nickname: string;
		signature: string;
		userId: number;
	};
	id: number;
	name: string;
	picUrl: string;
	playcount: number;
	trackCount: number;
	type: number;
	userId: number;
}

export interface recommendPlaylist {
	code: number;
	featureFirst: boolean;
	haveRcmdSongs: boolean;
	recommend: RecommendItem[];
}

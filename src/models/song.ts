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

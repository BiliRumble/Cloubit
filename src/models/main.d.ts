// 热搜
export type HotSearchResult = {
	code: number;
	data: {
		alg: string;
		content: string;
		iconType: number;
		iconUrl: string;
		score: number;
		searchWord: string;
		source: number;
		url: string;
	}[];
};

// 搜索建议
export interface Artist {
	albumSize: number;
	alias: string[];
	fansGroup: string | null;
	id: number;
	img1v1: number;
	img1v1Url: string;
	name: string;
	picId: number;
	picUrl: string;
	trans: string | null;
}

export interface Album {
	artist: Artist;
	copyrightId: number;
	id: number;
	mark: number;
	name: string;
	picId: number;
	publishTime: number;
	size: number;
	status: number;
}

export interface Song {
	[x: string]: any;
	album: Album;
	alias: string[];
	artists: Artist[];
	copyrightId: number;
	duration: number;
	fee: number;
	ftype: number;
	id: number;
	mark: number;
	mvid: number;
	name: string;
	rUrl: string | null;
	rtype: number;
	status: number;
	transNames?: string[];
}

export interface Playlist {
	action: any;
	actionType: any;
	bookCount: number;
	coverImgUrl: string;
	creator: any;
	description: string;
	highQuality: boolean;
	id: number;
	name: string;
	officialTags: any;
	playCount: number;
	recommendText: any;
	score: any;
	specialType: number;
	subscribed: boolean;
	trackCount: number;
	userId: number;
}

export interface SuggestSearchResult {
	code: number;
	result: {
		albums: Album[];
		artists: Artist[];
		order: string[];
		playlists: Playlist[];
		songs: Song[];
	};
}

// 默认搜索内容,显示于搜索框，默认填充内容
export type DefaultSearchResult = {
	code: number;
	data: {
		action: number;
		alg: string;
		bizQueryInfo: string;
		gap: number;
		imageUrl: any;
		logInfo: any;
		realkeyword: string;
		searchType: number;
		showKeyword: string;
		source: number;
		styleKeyword: {
			descWord: string;
			keyWord: string;
		};
		trp_id: any;
		trp_type: any;
	};
	message: string | null;
};

// 通用结果接口
type searchType = 'song' | 'album' | 'playlist' | 'artist';
export interface SearchResult {
	code: number;
	result: {
		songCount?: number;
		songs?: Song[];
		albumCount?: number;
		albums?: Album[];
		artistCount?: number;
		artists?: Artist[];
		playlistCount?: number;
		playlists?: Playlist[];
	};
}

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
	data: {
		code: number;
		message: string;
		cookie: string;
		nickname?: string;
		avatarUrl?: string;
	};
};

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

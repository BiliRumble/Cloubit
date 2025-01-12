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

// TODO: 搜索结果
// 搜索结果 - 单曲
export interface SongSearchResult {
	code: number;
	result: {
		searchQcReminder: any;
		songCount: number;
		songs: Song[];
	};
}

// 搜索结果 - 歌单
export interface PlaylistSearchResult {
	code: number;
	result: {
		playlistCount: number;
		playlists: Playlist[];
		searchQcReminder: any;
	};
}

// 搜索结果 - 专辑
export interface AlbumSearchResult {
	code: number;
	result: {
		albumCount: number;
		albums: Album[];
	};
}

// 搜索结果 - 歌手
export interface ArtistSearchResult {
	code: number;
	result: {
		artistCount: number;
		artists: Artist[];
		searchQcReminder: any;
	};
}

type searchType = 'song' | 'album' | 'playlist' | 'artist';
type SearchResult =
	| SongSearchResult
	| PlaylistSearchResult
	| AlbumSearchResult
	| ArtistSearchResult;

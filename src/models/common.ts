// 基础通用类型
export interface BaseEntity {
	id: number;
	name: string;
}

// 扩展的艺术家类型
export interface Artist extends BaseEntity {
	albumSize: number;
	alias: string[];
	alia?: string[]; // 专辑搜索中的别名字段
	briefDesc?: string;
	fansGroup?: string | null;
	followed?: boolean; // 艺术家搜索特有
	img1v1?: number;
	img1v1Id?: number;
	img1v1Id_str?: string;
	img1v1Url: string;
	musicSize?: number;
	mvSize?: number; // 艺术家搜索特有
	picId?: number;
	picId_str?: string;
	picUrl?: string | null;
	topicPerson?: number;
	trans?: string | null;
	alg?: string; // 算法标识
}

// 音频质量信息
export interface MusicQuality {
	bitrate: number;
	dfsId: number;
	extension: string;
	id: number;
	name?: string | null;
	playTime: number;
	size: number;
	sr: number;
	volumeDelta: number;
}

// 免费试听时间权限
export interface FreeTimeTrialPrivilege {
	remainTime: number;
	resConsumable: boolean;
	type: number;
	userConsumable: boolean;
}

// 免费试听信息
export interface FreeTrialInfo {
	algData: {
		fragSource: string;
	};
	end: number;
	fragmentType: number;
	start: number;
}

// 免费试听权限
export interface FreeTrialPrivilege {
	cannotListenReason: string | null;
	freeLimitTagType: string | null;
	listenType: string | null;
	playReason: string | null;
	resConsumable: boolean;
	userConsumable: boolean;
}

// 歌曲URL数据
export interface SongUrlData {
	accompany: string | null;
	auEff: any | null;
	br: number; // 比特率
	canExtend: boolean;
	channelLayout: string | null;
	closedGain: number;
	closedPeak: number;
	code: number;
	effectTypes: any | null;
	encodeType: string; // 编码类型，如 "mp3"
	expi: number; // 过期时间（秒）
	fee: number; // 费用类型
	flag: number;
	freeTimeTrialPrivilege: FreeTimeTrialPrivilege;
	freeTrialInfo: FreeTrialInfo;
	freeTrialPrivilege: FreeTrialPrivilege;
	gain: number;
	id: number; // 歌曲ID
	level: string; // 音质等级，如 "standard", "higher", "exhigh", "lossless"
	levelConfuse: string | null;
	md5: string;
	message: string | null;
	musicId: string;
	payed: number;
	peak: number;
	podcastCtrp: any | null;
	rightSource: number;
	size: number; // 文件大小（字节）
	sr: number; // 采样率
	time: number; // 时长（毫秒）
	type: string; // 文件类型，如 "mp3"
	uf: any | null;
	url: string; // 播放URL
	urlSource: number;
}

// 音质等级枚举
export enum AudioLevel {
	STANDARD = 'standard', // 标准音质
	HIGHER = 'higher', // 较高音质
	EXHIGH = 'exhigh', // 极高音质
	LOSSLESS = 'lossless', // 无损音质
	HIRES = 'hires', // Hi-Res音质
	JYEFFECT = 'jyeffect', // 高清环绕声
	SKY = 'sky', // 沉浸环绕声
	JYMASTER = 'jymaster', // 超清母带
}

// 比特率枚举
export enum Bitrate {
	STANDARD = 128000, // 128kbps 标准音质
	HIGH = 320000, // 320kbps 高音质
	LOSSLESS = 999000, // 999kbps 无损音质
	HIRES = 1999000, // Hi-Res音质
}

// 扩展的专辑类型
export interface Album extends BaseEntity {
	alg?: string;
	alias: string[];
	artist: Artist;
	artists?: Artist[];
	blurPicUrl?: string;
	briefDesc?: string;
	commentThreadId?: string;
	company?: string | null;
	companyId?: number;
	containedSong?: string;
	copyrightId: number;
	description?: string;
	idStr?: string | null;
	mark?: number;
	onSale?: boolean;
	paid?: boolean;
	pic?: number;
	picId: number;
	picId_str?: string;
	picUrl: string;
	img1v1: number;
	img1v1Url: string;
	publishTime: number;
	size: number;
	songs?: Song[] | null;
	status: number;
	tags?: string;
	type?: string; // "专辑" | "Single" 等
}

// 用户/创建者类型
export interface User {
	authStatus?: number;
	avatarUrl: string;
	expertTags?: any;
	experts?: any;
	nickname: string;
	userId: number;
	userType?: number;
}

// 扩展的歌曲类型
export interface Song extends BaseEntity {
	album: Album;
	alias: string[];
	artists: Artist[];
	audition?: any;
	bMusic?: MusicQuality;
	commentThreadId?: string;
	copyFrom?: string;
	copyright?: number;
	copyrightId: number;
	crbt?: any;
	dayPlays?: number;
	disc?: string;
	duration: number;
	fee: number;
	ftype: number;
	hMusic?: MusicQuality | null;
	hearTime?: number;
	lMusic?: MusicQuality;
	mMusic?: MusicQuality;
	mark?: number;
	mp3Url?: string | null;
	mvid: number;
	no?: number;
	playedNum?: number;
	popularity?: number;
	position?: number;
	ringtone?: string;
	rtUrl?: string | null;
	rtUrls?: string[];
	rtype?: number;
	rUrl?: string | null;
	rurl?: string | null;
	score?: number;
	starred?: boolean;
	starredNum?: number;
	status: number;
	transNames?: string[];
}

// 带URL信息的歌曲类型
export interface SongWithUrl extends Song {
	url?: string; // 播放URL
	urlData?: SongUrlData; // 完整的URL数据
	currentBitrate?: number; // 当前比特率
	currentLevel?: AudioLevel; // 当前音质等级
}

// 歌单类型
export interface Playlist extends BaseEntity {
	action?: string;
	actionType?: string;
	alg?: string;
	bookCount?: number;
	coverImgUrl: string;
	creator: User;
	description?: string | null;
	highQuality?: boolean;
	officialTags?: any;
	playCount: number;
	recommendText?: string | null;
	score?: string | null;
	specialType?: number;
	subscribed?: boolean;
	track?: Song; // 代表性歌曲
	trackCount: number;
	userId: number;
}

// 搜索结果类型
export interface SongSearchResult {
	hasMore: boolean;
	songCount: number;
	songs: Song[];
	hlWords?: string[];
}

export interface AlbumSearchResult {
	albumCount: number;
	albums: Album[];
	hlWords?: string[];
}

export interface PlaylistSearchResult {
	hasMore: boolean;
	playlistCount: number;
	playlists: Playlist[];
	hlWords?: string[];
	searchQcReminder?: any;
}

export interface ArtistSearchResult {
	artistCount: number;
	artists: Artist[];
	hasMore: boolean;
	hlWords?: string[];
	searchQcReminder?: any;
}

// 通用 API 响应类型
export interface ApiResponse<T = any> {
	code: number;
	data?: T; // 修改为data字段，与API返回保持一致
	result?: T; // 保留result字段以兼容其他API
	message?: string;
}

// 歌曲URL响应类型
export interface SongUrlResponse extends ApiResponse<SongUrlData[]> {}

// 各类型搜索响应
export interface MusicSearchResponse extends ApiResponse<SongSearchResult> {}
export interface AlbumSearchResponse extends ApiResponse<AlbumSearchResult> {}
export interface PlaylistSearchResponse extends ApiResponse<PlaylistSearchResult> {}
export interface ArtistSearchResponse extends ApiResponse<ArtistSearchResult> {}

// 搜索类型枚举
export enum SearchType {
	SONG = 1, // 单曲
	ALBUM = 10, // 专辑
	ARTIST = 100, // 歌手
	PLAYLIST = 1000, // 歌单
	USER = 1002, // 用户
	MV = 1004, // MV
	LYRIC = 1006, // 歌词
	RADIO = 1009, // 电台
	VIDEO = 1014, // 视频
}

// 搜索参数接口
export interface SearchParams {
	keyword: string;
	type?: SearchType;
	limit?: number;
	offset?: number;
}

// 歌曲URL请求参数
export interface SongUrlParams {
	id: number | number[]; // 歌曲ID，可以是单个或数组
	br?: Bitrate | number; // 比特率
	level?: AudioLevel; // 音质等级
}

// 播放模式枚举
export enum PlayMode {
	SEQUENCE = 'sequence', // 顺序播放
	LOOP = 'loop', // 列表循环
	SINGLE = 'single', // 单曲循环
	RANDOM = 'random', // 随机播放
}

// 播放列表项
export interface PlaylistItem extends SongWithUrl {
	index: number; // 在播放列表中的索引
	isPlaying?: boolean; // 是否正在播放
	isPaused?: boolean; // 是否暂停
}

// 播放状态
export interface PlayState {
	currentSong: PlaylistItem | null;
	playlist: PlaylistItem[];
	currentIndex: number;
	isPlaying: boolean;
	isPaused: boolean;
	volume: number;
	currentTime: number;
	duration: number;
	playMode: PlayMode;
}

// 联合搜索结果类型
export type SearchResult =
	| SongSearchResult
	| AlbumSearchResult
	| PlaylistSearchResult
	| ArtistSearchResult;

// 联合搜索响应类型
export type SearchResponse =
	| MusicSearchResponse
	| AlbumSearchResponse
	| PlaylistSearchResponse
	| ArtistSearchResponse;

// 搜索结果项联合类型
export type SearchResultItem = Song | Album | Playlist | Artist;

// 通用分页参数
export interface PaginationParams {
	limit?: number;
	offset?: number;
}

// 通用分页结果
export interface PaginatedResult<T> {
	items: T[];
	total: number;
	hasMore: boolean;
	offset: number;
	limit: number;
}

// 错误响应类型
export interface ErrorResponse {
	code: number;
	message: string;
	details?: any;
}

// 歌曲URL错误类型
export enum SongUrlErrorCode {
	NOT_AVAILABLE = 404, // 歌曲不可用
	NO_COPYRIGHT = 403, // 无版权
	VIP_ONLY = 402, // 需要VIP
	REGION_RESTRICTED = 451, // 地区限制
	NETWORK_ERROR = 500, // 网络错误
	UNKNOWN_ERROR = -1, // 未知错误
}

export interface SongUrlError extends ErrorResponse {
	code: SongUrlErrorCode;
	songId: number;
	requestedBitrate?: number;
}

// 音乐服务接口定义
export interface MusicService {
	searchSongs(params: SearchParams): Promise<MusicSearchResponse>;
	searchAlbums(params: SearchParams): Promise<AlbumSearchResponse>;
	searchPlaylists(params: SearchParams): Promise<PlaylistSearchResponse>;
	searchArtists(params: SearchParams): Promise<ArtistSearchResponse>;
	search(params: SearchParams): Promise<SearchResponse>;
	getSongDetail(id: number): Promise<ApiResponse<Song>>;
	getAlbumDetail(id: number): Promise<ApiResponse<Album>>;
	getArtistDetail(id: number): Promise<ApiResponse<Artist>>;
	getPlaylist(id: number): Promise<ApiResponse<Playlist>>;
	// 新增歌曲URL相关方法
	getSongUrl(id: number, br?: Bitrate): Promise<SongUrlResponse>;
	getSongUrls(ids: number[], br?: Bitrate): Promise<SongUrlResponse>;
	getSongUrlByLevel(id: number, level: AudioLevel): Promise<SongUrlResponse>;
	checkSongAvailability(id: number): Promise<ApiResponse<boolean>>;
}

// 类型守卫函数
export function isSong(item: any): item is Song {
	return (
		item &&
		typeof item.id === 'number' &&
		typeof item.name === 'string' &&
		item.artists &&
		item.album
	);
}

export function isArtist(item: any): item is Artist {
	return (
		item &&
		typeof item.id === 'number' &&
		typeof item.name === 'string' &&
		typeof item.albumSize === 'number'
	);
}

export function isAlbum(item: any): item is Album {
	return item && typeof item.id === 'number' && typeof item.name === 'string' && item.artist;
}

export function isPlaylist(item: any): item is Playlist {
	return (
		item &&
		typeof item.id === 'number' &&
		typeof item.name === 'string' &&
		item.creator &&
		typeof item.trackCount === 'number'
	);
}

export function isSongSearchResult(result: any): result is SongSearchResult {
	return result && Array.isArray(result.songs) && typeof result.songCount === 'number';
}

export function isAlbumSearchResult(result: any): result is AlbumSearchResult {
	return result && Array.isArray(result.albums) && typeof result.albumCount === 'number';
}

export function isPlaylistSearchResult(result: any): result is PlaylistSearchResult {
	return result && Array.isArray(result.playlists) && typeof result.playlistCount === 'number';
}

export function isArtistSearchResult(result: any): result is ArtistSearchResult {
	return result && Array.isArray(result.artists) && typeof result.artistCount === 'number';
}

// 新增歌曲URL相关类型守卫
export function isSongUrlData(item: any): item is SongUrlData {
	return (
		item &&
		typeof item.id === 'number' &&
		typeof item.url === 'string' &&
		typeof item.br === 'number' &&
		typeof item.size === 'number' &&
		typeof item.code === 'number'
	);
}

export function isSongWithUrl(item: any): item is SongWithUrl {
	return isSong(item) && (item.url !== undefined || item.urlData !== undefined);
}

export function isPlaylistItem(item: any): item is PlaylistItem {
	return isSongWithUrl(item) && typeof item.index === 'number';
}

export function isSongUrlResponse(response: any): response is SongUrlResponse {
	return (
		response &&
		typeof response.code === 'number' &&
		Array.isArray(response.data) &&
		response.data.every((item: any) => isSongUrlData(item))
	);
}

export type searchType = 'song' | 'album' | 'playlist' | 'artist';

// 工具类型
export type PartialSong = Partial<Song>;
export type PartialAlbum = Partial<Album>;
export type PartialPlaylist = Partial<Playlist>;
export type PartialArtist = Partial<Artist>;
export type PartialSongUrlData = Partial<SongUrlData>;

export type SongKeys = keyof Song;
export type AlbumKeys = keyof Album;
export type PlaylistKeys = keyof Playlist;
export type ArtistKeys = keyof Artist;
export type SongUrlDataKeys = keyof SongUrlData;

// 搜索结果映射类型
export type SearchResultMap = {
	[SearchType.SONG]: SongSearchResult;
	[SearchType.ALBUM]: AlbumSearchResult;
	[SearchType.PLAYLIST]: PlaylistSearchResult;
	[SearchType.ARTIST]: ArtistSearchResult;
};

// 响应映射类型
export type SearchResponseMap = {
	[SearchType.SONG]: MusicSearchResponse;
	[SearchType.ALBUM]: AlbumSearchResponse;
	[SearchType.PLAYLIST]: PlaylistSearchResponse;
	[SearchType.ARTIST]: ArtistSearchResponse;
};

// 工具函数
export function formatFileSize(bytes: number): string {
	if (bytes === 0) return '0 B';
	const k = 1024;
	const sizes = ['B', 'KB', 'MB', 'GB'];
	const i = Math.floor(Math.log(bytes) / Math.log(k));
	return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

export function formatDuration(milliseconds: number): string {
	const seconds = Math.floor(milliseconds / 1000);
	const minutes = Math.floor(seconds / 60);
	const remainingSeconds = seconds % 60;
	return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`;
}

export function formatBitrate(br: number): string {
	return `${Math.floor(br / 1000)}kbps`;
}

export function getAudioLevelText(level: AudioLevel | string): string {
	const levelMap: Record<string, string> = {
		[AudioLevel.STANDARD]: '标准',
		[AudioLevel.HIGHER]: '较高',
		[AudioLevel.EXHIGH]: '极高',
		[AudioLevel.LOSSLESS]: '无损',
		[AudioLevel.HIRES]: 'Hi-Res',
		[AudioLevel.JYEFFECT]: '高清环绕声',
		[AudioLevel.SKY]: '沉浸环绕声',
		[AudioLevel.JYMASTER]: '超清母带',
	};
	return levelMap[level] || level;
}

export function canPlaySong(songData: SongUrlData): boolean {
	return songData.code === 200 && !!songData.url && songData.url !== '';
}

export function getSongQualityInfo(songData: SongUrlData): {
	quality: string;
	size: string;
	bitrate: string;
	format: string;
	duration: string;
} {
	return {
		quality: getAudioLevelText(songData.level),
		size: formatFileSize(songData.size),
		bitrate: formatBitrate(songData.br),
		format: songData.encodeType.toUpperCase(),
		duration: formatDuration(songData.time),
	};
}

export function getBitrateFromLevel(level: AudioLevel): Bitrate {
	const levelToBitrateMap: Record<AudioLevel, Bitrate> = {
		[AudioLevel.STANDARD]: Bitrate.STANDARD,
		[AudioLevel.HIGHER]: Bitrate.HIGH,
		[AudioLevel.EXHIGH]: Bitrate.HIGH,
		[AudioLevel.LOSSLESS]: Bitrate.LOSSLESS,
		[AudioLevel.HIRES]: Bitrate.HIRES,
		[AudioLevel.JYEFFECT]: Bitrate.LOSSLESS,
		[AudioLevel.SKY]: Bitrate.LOSSLESS,
		[AudioLevel.JYMASTER]: Bitrate.HIRES,
	};
	return levelToBitrateMap[level] || Bitrate.HIGH;
}

export function getLevelFromBitrate(bitrate: number): AudioLevel {
	if (bitrate <= 128000) return AudioLevel.STANDARD;
	if (bitrate <= 320000) return AudioLevel.HIGHER;
	if (bitrate <= 999000) return AudioLevel.LOSSLESS;
	return AudioLevel.HIRES;
}

// 歌曲费用类型枚举
export enum SongFeeType {
	FREE = 0, // 免费
	VIP = 1, // VIP歌曲
	PAID = 4, // 付费歌曲
	ALBUM_PAID = 8, // 专辑付费
}

// 歌曲可用性检查
export function checkSongPlayability(songData: SongUrlData): {
	canPlay: boolean;
	reason?: string;
	needVip?: boolean;
	needPay?: boolean;
} {
	if (songData.code !== 200) {
		return {
			canPlay: false,
			reason: songData.message || '歌曲不可用',
		};
	}

	if (!songData.url || songData.url === '') {
		return {
			canPlay: false,
			reason: '无播放链接',
		};
	}

	if (songData.fee === SongFeeType.VIP) {
		return {
			canPlay: false,
			reason: '需要VIP权限',
			needVip: true,
		};
	}

	if (songData.fee === SongFeeType.PAID || songData.fee === SongFeeType.ALBUM_PAID) {
		return {
			canPlay: false,
			reason: '需要付费购买',
			needPay: true,
		};
	}

	// 检查试听限制
	if (songData.freeTrialInfo && songData.freeTrialInfo.end > 0) {
		return {
			canPlay: true,
			reason: `试听${songData.freeTrialInfo.end}秒`,
		};
	}

	return { canPlay: true };
}

// 音频格式枚举
export enum AudioFormat {
	MP3 = 'mp3',
	FLAC = 'flac',
	OGG = 'ogg',
	AAC = 'aac',
	M4A = 'm4a',
}

// 扩展的音频信息
export interface AudioInfo {
	format: AudioFormat;
	bitrate: number;
	sampleRate: number;
	channels: number;
	duration: number;
	size: number;
	quality: AudioLevel;
}

// 播放历史记录
export interface PlayHistory {
	id: string;
	song: Song;
	playedAt: Date;
	duration: number; // 播放时长（毫秒）
	completed: boolean; // 是否播放完成
}

// 收藏夹类型
export interface Favorite {
	id: string;
	name: string;
	description?: string;
	songs: Song[];
	createdAt: Date;
	updatedAt: Date;
}

// 下载任务状态
export enum DownloadStatus {
	PENDING = 'pending',
	DOWNLOADING = 'downloading',
	COMPLETED = 'completed',
	FAILED = 'failed',
	PAUSED = 'paused',
}

// 下载任务
export interface DownloadTask {
	id: string;
	song: Song;
	urlData: SongUrlData;
	status: DownloadStatus;
	progress: number; // 0-100
	filePath?: string;
	createdAt: Date;
	completedAt?: Date;
	error?: string;
}

// 应用设置
export interface AppSettings {
	audioQuality: AudioLevel;
	downloadQuality: AudioLevel;
	autoPlay: boolean;
	crossfade: boolean;
	crossfadeDuration: number; // 秒
	volume: number; // 0-100
	playMode: PlayMode;
	downloadPath: string;
	cacheSize: number; // MB
	enableLyrics: boolean;
	enableNotifications: boolean;
}

// 统计信息
export interface Statistics {
	totalPlayTime: number; // 总播放时长（毫秒）
	totalSongs: number; // 总歌曲数
	totalArtists: number; // 总艺术家数
	totalAlbums: number; // 总专辑数
	favoriteGenres: string[]; // 喜爱的音乐类型
	mostPlayedSongs: Song[]; // 最常播放的歌曲
	recentlyPlayed: PlayHistory[]; // 最近播放
}

// 导出所有类型以便在其他文件中使用
export type {
	BaseEntity,
	Artist,
	MusicQuality,
	FreeTimeTrialPrivilege,
	FreeTrialInfo,
	FreeTrialPrivilege,
	SongUrlData,
	Album,
	User,
	Song,
	SongWithUrl,
	Playlist,
	SongSearchResult,
	AlbumSearchResult,
	PlaylistSearchResult,
	ArtistSearchResult,
	ApiResponse,
	SongUrlResponse,
	MusicSearchResponse,
	AlbumSearchResponse,
	PlaylistSearchResponse,
	ArtistSearchResponse,
	SearchParams,
	SongUrlParams,
	PlaylistItem,
	PlayState,
	SearchResult,
	SearchResponse,
	SearchResultItem,
	PaginationParams,
	PaginatedResult,
	ErrorResponse,
	SongUrlError,
	MusicService,
	AudioInfo,
	PlayHistory,
	Favorite,
	DownloadTask,
	AppSettings,
	Statistics,
};

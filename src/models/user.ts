export interface BaseUser {
	userId: number;
	nickname: string;
	avatarUrl: string;
	authStatus: number;
	userType: number;
}

interface Account {
	anonimousUser: boolean;
	ban: number;
	baoyueVersion: number;
	createTime: number;
	donateVersion: number;
	id: number;
	paidFee: boolean;
	status: number;
	tokenVersion: number;
	type: number;
	userName: string;
	vipType: number;
	whitelistAuthority: number;
}

export interface Profile extends BaseUser {
	accountStatus: number;
	accountType: number;
	anchor: boolean;
	authStatus: number;
	authenticated: boolean;
	authenticationTypes: number;
	authority: number;
	avatarDetail: any; // 这里可以是具体类型，但JSON中为null
	avatarImgId: number;
	avatarUrl: string;
	backgroundImgId: number;
	backgroundUrl: string;
	birthday: number;
	city: number;
	createTime: number;
	defaultAvatar: boolean;
	description: string | null;
	detailDescription: string | null;
	djStatus: number;
	expertTags: any; // 这里可以是具体类型，但JSON中为null
	experts: any; // 这里可以是具体类型，但JSON中为null
	followed: boolean;
	gender: number;
	lastLoginIP: string;
	lastLoginTime: number;
	locationStatus: number;
	mutual: boolean;
	nickname: string;
	province: number;
	remarkName: string | null;
	shortUserName: string;
	signature: string | null;
	userId: number;
	userName: string;
	userType: number;
	vipType: number;
	viptypeVersion: number;
}

export interface UserAccountResult {
	account: Account;
	code: number;
	profile: Profile;
}

interface Binding {
	bindingTime: number;
	expired: boolean;
	expiresIn: number;
	id: number;
	refreshTime: number;
	tokenJsonStr: any; // 这里可以是具体类型，但JSON中为null
	type: number;
	url: string;
	userId: number;
}

interface PrivacyItemUnlimit {
	age: boolean;
	area: boolean;
	college: boolean;
	gender: boolean;
	villageAge: boolean;
}

interface ProfileVillageInfo {
	imageUrl: string | null;
	targetUrl: string;
	title: string;
}

interface UserPoint {
	balance: number;
	blockBalance: number;
	status: number;
	updateTime: number;
	userId: number;
	version: number;
}

export interface UserDetailResult {
	adValid: boolean;
	bindings: Binding[];
	code: number;
	createDays: number;
	createTime: number;
	level: number;
	listenSongs: number;
	mobileSign: boolean;
	newUser: boolean;
	pcSign: boolean;
	peopleCanSeeMyPlayRecord: boolean;
	profile: {
		accountStatus: number;
		allSubscribedCount: number;
		artistIdentity: any[]; // 这里可以是具体类型，但JSON中为空数组
		authStatus: number;
		authority: number;
		avatarDetail: any; // 这里可以是具体类型，但JSON中为null
		avatarImgId: number;
		avatarImgIdStr: string;
		avatarUrl: string;
		backgroundImgId: number;
		backgroundImgIdStr: string;
		backgroundUrl: string;
		birthday: number;
		blacklist: boolean;
		cCount: number;
		city: number;
		createTime: number;
		defaultAvatar: boolean;
		description: string;
		detailDescription: string;
		djStatus: number;
		eventCount: number;
		expertTags: any; // 这里可以是具体类型，但JSON中为null
		experts: object; // 这里可以是具体类型，但JSON中为空对象
		followMe: boolean;
		followTime: any; // 这里可以是具体类型，但JSON中为null
		followed: boolean;
		followeds: number;
		follows: number;
		gender: number;
		inBlacklist: boolean;
		mutual: boolean;
		newFollows: number;
		nickname: string;
		playlistBeSubscribedCount: number;
		playlistCount: number;
		privacyItemUnlimit: PrivacyItemUnlimit;
		province: number;
		remarkName: string | null;
		sCount: number;
		sDJPCount: number;
		signature: string;
		userId: number;
		userType: number;
		vipType: number;
	};
	profileVillageInfo: ProfileVillageInfo;
	recallUser: boolean;
	userPoint: UserPoint;
}

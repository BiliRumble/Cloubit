export interface CommentResponse {
	cnum: number;
	code: number;
	commentBanner: null;
	comments: Comment[];
	hotComments: Comment[];
	isMusician: boolean;
	more: boolean;
	moreHot: boolean;
	topComments: any[];
	total: number;
	userId: number;
}

export interface Comment {
	beReplied: any[];
	commentId: number;
	commentLocationType: number;
	content: string;
	contentResource: null;
	decoration: Record<string, unknown>;
	expressionUrl: null;
	grade: null;
	ipLocation: IpLocation;
	likeAnimationMap: Record<string, unknown>;
	liked: boolean;
	likedCount: number;
	medal: null;
	needDisplayTime: boolean;
	owner: boolean;
	parentCommentId: number;
	pendantData: PendantData | null;
	repliedMark: null;
	richContent: string | null;
	showFloorComment: ShowFloorComment;
	status: number;
	time: number;
	timeStr: string;
	user: CommentUser;
	userBizLevels: null;
}

interface IpLocation {
	ip: null;
	location: string;
	userId: number | null;
}

interface PendantData {
	id: number;
	imageUrl: string;
}

interface ShowFloorComment {
	comments: null;
	replyCount: number;
	showReplyCount: boolean;
}

// 复用用户基础类型
interface BaseUser {
	userId: number;
	nickname: string;
	avatarUrl: string;
	authStatus: number;
	userType: number;
}

export interface CommentUser extends BaseUser {
	anonym: number;
	avatarDetail: AvatarDetail | null;
	vipRights: CommentVipRights;
}

interface AvatarDetail {
	identityIconUrl: string;
	identityLevel: number;
	userType: number;
}

interface CommentVipRights {
	associator: Associator | null;
	musicPackage: MusicPackage | null;
	redVipAnnualCount: number;
	redVipLevel: number;
	redplus: null;
	relationType: number;
}

interface Associator {
	iconUrl: string;
	rights: boolean;
	vipCode: number;
}

interface MusicPackage {
	iconUrl: string;
	rights: boolean;
	vipCode: number;
}

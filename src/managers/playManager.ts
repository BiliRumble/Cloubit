const data = {
	name: '测试歌曲aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa',
	cover: 'http://y.gtimg.cn/music/photo_new/T002R500x500M000000vpEZf4eAeeG.jpg?pcachetim',
	song: '/Aoharu Band Arrange(remix).mp3',
	artist: 'Dashi',
	isFromPlayList: false,
};

let playstatus = {
	isPlaying: false,
	isMuted: false,
	mode: 'loop', // loop, random, single
	volume: 0.5,
	playTime: 0,
	maxTime: 238,
};

export function getPlayStatus() {
	return { info: data, status: playstatus };
}

export function setPlayStatus(status: typeof playstatus) {
	playstatus = status;
}

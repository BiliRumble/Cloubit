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

export type SuggestSearchResult = {};

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

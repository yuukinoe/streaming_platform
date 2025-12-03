export type IDWrapper = {
	tb: string;
	id: {
		String: string;
	};
};

export type TagOrStudio = {
	id: IDWrapper;
	name: string;
	description: string | null;
	date: string;
	slug: string;
};

export type Anime = {
	id: IDWrapper;
	mal: string;
	discord_id: number | null;
	discord_role_id: string | null;
	background_position: number;
	shinden: string | null;
	title: string;
	description: string | null;
	anime_type: string;
	episodes: number;
	status: string;
	in_progress: boolean;
	release_time: string;
	aired: string;
	source: string;
	tags: TagOrStudio[];
	studio: TagOrStudio[];
	image: string;
	date: string;
	slug: string;
	alternative_title: string;
};

export type PublicRole = {
	name: string;
	administrative_role: boolean;
	visible: boolean;
	color: string;
	hierarchy: number;
	icon: string;
	date: string | null;
};

export type UserPublic = {
	id: IDWrapper;
	name: string;
	avatar: string;
	description: string;
	translator: boolean;
	proofreader: boolean;
	uploader: boolean;
	editor: boolean;
	roles: PublicRole[];
	created_at: string;
	is_active: boolean;
	slug: string;
};

export type Member = {
	id: IDWrapper;
	name: string;
	avatar: string;
	description: string;
	translator: boolean;
	proofreader: boolean;
	uploader: boolean;
	editor: boolean;
	roles: PublicRole[];
	is_active: boolean;
	administrative_role: PublicRole;
	slug: string;
};

export type EpisodesPublic = {
	id: IDWrapper;
	title: string;
	discord_id: number | null;
	translators: UserPublic[];
	proofreaders: UserPublic[];
	uploaders: UserPublic[];
	typesetters: UserPublic[];
	anime: Anime;
	episode: number;
	length: number;
	image: string;
	description: string;
	color: string;
	video_players: string[];
	discord_ping: boolean;
	date: string;
};

export type CarouselItem = {
	alt: string;
	src: string;
	title: string;
	anime_title: string;
	anime_slug: string;
	episode_number: number;
	episode_description: string;
	item_type: string;
};

export type SSRData = {
	data: {
		anime: Anime[];
		episodes: EpisodesPublic[];
	};
};
export type Episodes = {
	id: IDWrapper;
	title: string;
	discord_id: number | null;
	translators: UserPublic[];
	proofreaders: UserPublic[];
	uploaders: UserPublic[];
	typesetters: UserPublic[];
	anime: Anime;
	episode: number;
	length: number;
	image: string;
	description: string;
	color: string;
	subtitles: string;
	torrent: string;
	video_players: string[];
	discord_ping: boolean;
	webhook: boolean;
	date: string;
	slug: string;
};

export type NewsCategoryPublic = {
	id: IDWrapper;
	name: string;
	allowed_everyone: boolean;
	visible: boolean;
	slug: string;
	date: string;
};

export type NewsPublic = {
	id: IDWrapper;
	discord_id: number | null;
	author: UserPublic | null;
	website: boolean;
	category: NewsCategoryPublic;
	hyperlink: string;
	pinned: boolean;
	color: string;
	text_header: string;
	thumbnail: string;
	image: string;
	description: string;
	slug: string;
	date: string;
};
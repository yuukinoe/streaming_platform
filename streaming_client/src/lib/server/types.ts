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

export type GenericResponse = string | Anime | UserModelSuperUser;

export interface RResponse {
	status: number;
	message: GenericResponse;
}

export type AnimeList = Anime[];

export type MultiSelectType = {
	value: string;
	name: string;
	color: string;
};

export type ComboBoxType = {
	value: string;
	label: string;
};

export type UserResponse = {
	status: number;
	message: UserModelSuperUser;
};

export type UserModelSuperUser = {
	id: IDWrapper;
	name: string;
	password: string;
	avatar: string;
	email: string;
	description: string;
	translator: boolean;
	proofreader: boolean;
	uploader: boolean;
	editor: boolean;
	published: boolean;
	created_at: string;
	updated_at: string;
	is_superuser: boolean;
	is_staff: boolean;
	is_active: boolean;
	is_banned: boolean;
	roles: Role[] | null;
	slug: string;
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




export type LogsPublic = {
	id: IDWrapper;
	user_id: UserPublic;
	action: string;
	object: string;
	object_id: string;
	description: string | null;
	date: string;
};

export type Episodes = {
	id: IDWrapper;
	title: string | null;
	discord_id: number | null;
	translators: UserPublic[] | null;
	proofreaders: UserPublic[] | null;
	uploaders: UserPublic[] | null;
	typesetters: UserPublic[] | null;
	anime: Anime;
	episode: number;
	length: number;
	image: string | null;
	description: string | null;
	color: string;
	subtitles: string | null;
	torrent: string | null;
	video_players: string[] | null;
	discord_ping: boolean;
	webhook: boolean;
	date: string | null;
	slug: string;
};

export type UserProfile = {
	name: string;
	password: string;
	avatar: string;
	description: string;
	email: string;
	created_at: string;
	updated_at: string;
};

export type Role = {
	id: IDWrapper;
	name: string;
	permissions: PermissionsMap;
	administrative_role: boolean;
	visible: boolean;
	color: string;
	hierarchy: number;
	icon: string;
	date: string | null;
};

export type PublicRole = {
	id: IDWrapper;
	name: string;
	administrative_role: boolean;
	visible: boolean;
	color: string;
	hierarchy: number;
	icon: string;
	date: string | null;
};


export type PermissionFlags = {
	add: boolean;
	read: boolean;
	edit: boolean;
	self_action: boolean;
	post_requests: boolean;
	delete: boolean;
};

export type LogsTable = {
	id: string;
	object: string;
	user: string;
	action: string;
	date: string;
};

export type NewsCategoryStaff = {
	id: IDWrapper;
	name: string;
	discord_webhook: string;
	allowed_everyone: boolean;
	visible: boolean;
	slug: string;
	date: string | null;
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






export type PermissionsMap = Record<string, PermissionFlags>;

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
	webhook: boolean;
	date: string;
};



export type AnalyticsAnime = {
	id: IDWrapper;
	kind: string;
	object_id: Anime;
	date: string;
	views: number;
	ip: string | null;
};

export type AnalyticsOverall = {
	id: IDWrapper;
	kind: string;
	object_id: IDWrapper;
	date: string;
	views: number;
	ip: string | null;
};


export const Dummy = {
	IDWrapper(): IDWrapper {
		return {
			tb: '',
			id: { String: '' }
		};
	},
	TagOrStudio(): TagOrStudio {
		return {
			id: Dummy.IDWrapper(),
			name: '',
			description: null,
			date: '',
			slug: ''
		};
	},
	Anime(): Anime {
		return {
			id: Dummy.IDWrapper(),
			mal: '',
			discord_id: null,
			shinden: null,
			discord_role_id: null,
			background_position: 7,
			title: '',
			description: null,
			anime_type: '',
			episodes: 0,
			status: '',
			in_progress: false,
			release_time: '',
			aired: '',
			source: '',
			tags: [],
			studio: [],
			image: '',
			date: '',
			slug: '',
			alternative_title: ''
		};
	},
	UserModelSuperUser(): UserModelSuperUser {
		return {
			id: Dummy.IDWrapper(),
			name: '',
			password: '',
			avatar: '',
			email: '',
			description: '',
			translator: false,
			proofreader: false,
			uploader: false,
			editor: false,
			published: false,
			created_at: '',
			updated_at: '',
			is_superuser: false,
			is_staff: false,
			is_active: false,
			is_banned: false,
			roles: [Dummy.Role()],
			slug: ''
		};
	},
	UserPublic(): UserPublic {
		return {
			id: Dummy.IDWrapper(),
			name: '',
			avatar: '',
			description: '',
			translator: false,
			proofreader: false,
			uploader: false,
			editor: false,
			roles: [Dummy.PublicRole()],
			created_at: '',
			is_active: false,
			slug: ''
		};
	},
	LogsPublic(): LogsPublic {
		return {
			id: Dummy.IDWrapper(),
			user_id: Dummy.UserPublic(),
			action: '',
			object: '',
			object_id: '',
			description: null,
			date: ''
		};
	},
	Episodes(): Episodes {
		return {
			id: Dummy.IDWrapper(),
			title: null,
			discord_id: null,
			translators: null,
			proofreaders: null,
			uploaders: null,
			typesetters: null,
			anime: Dummy.Anime(),
			episode: 0,
			length: 0,
			image: '',
			description: null,
			color: '',
			subtitles: '',
			torrent: '',
			video_players: null,
			discord_ping: false,
			webhook: false,
			date: null,
			slug: ''
		};
	},
	MultiSelectType(): MultiSelectType {
		return {
			value: '',
			name: '',
			color: ''
		};
	},
	getUserProfile(): UserProfile {
		return {
			name: '',
			password: '',
			avatar: '',
			description: '',
			email: '',
			created_at: '',
			updated_at: ''
		};
	},

	Role(): Role {
		return {
			id: Dummy.IDWrapper(),
			name: '',
			permissions: {},
			administrative_role: false,
			visible: false,
			color: '',
			hierarchy: 0,
			icon: '',
			date: ''
		};
	},
	PublicRole(): PublicRole {
		return {
			id: Dummy.IDWrapper(),
			name: '',
			administrative_role: false,
			visible: false,
			color: '',
			hierarchy: 0,
			icon: '',
			date: null
		};
	},
	PermissionFlags(): PermissionFlags {
		return {
			add: false,
			read: false,
			edit: false,
			self_action: false,
			post_requests: false,
			delete: false
		};
	}
};

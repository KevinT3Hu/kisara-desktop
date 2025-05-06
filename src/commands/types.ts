export interface Anime {
	id: number;
	name: string;
	name_cn: string;
	image: string;
	release_date?: string | null;
}

export type SortType = "Match" | "Heat" | "Rank" | "Score";

export interface Paginated<T> {
	total: number;
	limit: number;
	offset: number;
	data: T[];
}

export interface AnimeSearchResultItemImages {
	large: string;
	common: string;
	medium: string;
	small: string;
	grid: string;
}

export interface AnimeTag {
	name: string;
	count: number;
}

export interface AnimeRating {
	rank: number;
	total: number;
	count: Record<string, number>;
	score: number;
}

export interface AnimeSearchResultItem {
	id: number;
	name: string;
	name_cn: string;
	summary: string;
	date?: string | null;
	images: AnimeSearchResultItemImages;
	meta_tags: string[];
	tags: AnimeTag[];
	rating: AnimeRating;
}

export interface EpisodeSearchResultItem {
	id: number;
	name: string;
	name_cn: string;
	sort: number;
	ep?: number | null;
	air_date: string;
}

export interface Episode {
	id: number;
	anime_id: number;
	sort: number;
	ep?: number | null;
	name: string;
	name_cn: string;
	air_date?: string | null;
	progress: number;
	last_watch_time?: string | null;
	torrent_id?: string | null;
}

export interface TorrentInfo {
	name: string;
	size?: string | null;
	url?: string | null;
	magnet: string;
	date?: string | null;
	seeders?: number | null;
	leechers?: number | null;
	uploader?: string | null;
}

export interface TorrentStat {
	anime_name: string;
	ep: number;
	info: ManagedTorrentInfo;
	torrent_id: string;
}

export interface ManagedTorrentInfo {
	name: string;
	stats: TorrentStats;
}

export interface DurationWithHumanReadable {
	duration: string; // ISO 8601 duration format
	human_readable: string;
}

export interface Speed {
	mbps: number;
	human_readable: string;
}

export interface LiveStats {
	snapshot: StatsSnapshot;
	average_piece_download_time?: DurationWithHumanReadable | null;
	download_speed: Speed;
	upload_speed: Speed;
	time_remaining?: DurationWithHumanReadable | null;
}

export interface TorrentStats {
	state: TorrentStatsState;
	file_progress: number[];
	error?: string | null;
	progress_bytes: number;
	uploaded_bytes: number;
	total_bytes: number;
	finished: boolean;
	live?: LiveStats | null;
}

export type TorrentStatsState = "initializing" | "live" | "paused" | "error";

export interface StatsSnapshot {
	downloaded_and_checked_bytes: number;
	fetched_bytes: number;
	uploaded_bytes: number;
	downloaded_and_checked_pieces: number;
	total_piece_download_ms: number;
	peer_stats: AggregatePeerStats;
}

export interface AggregatePeerStats {
	queued: number;
	connecting: number;
	live: number;
	seen: number;
	dead: number;
	not_needed: number;
	steals: number;
}

export interface PlayInfo {
	video: string;
	subtitles: string[];
	ep: Episode;
	anime: Anime;
}

export interface DashboardSummary {
	today: Anime[];
	last_watched: [Anime, Episode][];
}

export interface Config {
	download_config: {
		download_path: string;
	};
	network_config: {
		bgm_proxy?: string;
		torrents_proxy?: string;
		bgm_proxy_enabled: boolean;
		torrents_proxy_enabled: boolean;
	};
	locale: string;
	debug_config: {
		log_level: LogLevelFilter;
	};
}

export type LogLevelFilter = "info" | "warn" | "error" | "debug" | "trace";

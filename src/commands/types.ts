export interface Anime {
	id: number;
	name: string;
	name_cn: string;
	image: string;
	release_date?: string;
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
	date?: string;
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
	ep?: number;
	air_date: string;
}

export interface Episode {
	id: number;
	anime_id: number;
	sort: number;
	ep?: number;
	name: string;
	name_cn: string;
	air_date?: string;
	progress: number;
	last_watch_time?: string;
	torrent_id?: string;
}

export interface TorrentInfo {
	name: string;
	size?: string;
	url?: string;
	magnet: string;
	date?: string;
	seeders?: number;
	leechers?: number;
	uploader?: string;
}

export interface TorrentStat {
	ep_display: string;
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
	average_piece_download_time?: DurationWithHumanReadable;
	download_speed: Speed;
	upload_speed: Speed;
	time_remaining?: DurationWithHumanReadable;
}

export interface TorrentStats {
	state: TorrentStatsState;
	file_progress: number[];
	error?: string;
	progress_bytes: number;
	uploaded_bytes: number;
	total_bytes: number;
	finished: boolean;
	live?: LiveStats;
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

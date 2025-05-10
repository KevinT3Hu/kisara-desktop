import { invoke } from "@tauri-apps/api/core";
import type {
	Anime,
	AnimeSearchResultItem,
	Paginated,
	SortType,
	Episode,
	TorrentInfo,
	TorrentStat,
	PlayInfo,
	DashboardSummary,
	Config,
	LogLevelFilter,
} from "./types";

export async function currentSeasonAnimes(): Promise<Anime[]> {
	return invoke<Anime[]>("current_season_animes");
}

export async function currentSeason(): Promise<string> {
	return invoke<string>("current_season");
}

export async function searchAnimes(
	keyword: string,
	sort: SortType,
	page?: number,
	limit?: number,
): Promise<Paginated<AnimeSearchResultItem>> {
	return invoke<Paginated<AnimeSearchResultItem>>("search_animes", {
		keyword,
		sort,
		page,
		limit,
	});
}

export async function searchSuggestions(keyword: string): Promise<string[]> {
	return invoke<string[]>("search_suggestions", { keyword });
}

export async function animesInList(animeIds: number[]): Promise<boolean[]> {
	return invoke<boolean[]>("animes_in_list", { animeIds });
}

export async function addAnime(anime: AnimeSearchResultItem): Promise<void> {
	return invoke<void>("add_anime", { anime });
}

export async function getEpisodes(animeId: number): Promise<Episode[]> {
	return invoke<Episode[]>("get_episodes", { animeId });
}

export async function getAnime(
	animeId: number,
): Promise<AnimeSearchResultItem> {
	return invoke<AnimeSearchResultItem>("get_anime", { animeId });
}

export async function initSearchTorrents(
	epId: number,
): Promise<Record<string, TorrentInfo[]>> {
	return invoke<Record<string, TorrentInfo[]>>("init_search_torrents", {
		epId,
	});
}

export async function getDownloadingTorrentsNum(): Promise<number> {
	return invoke<number>("get_downloading_torrents_num");
}

export async function addTorrent(magnet: string, epId: number): Promise<void> {
	return invoke<void>("add_torrent", { magnet, epId });
}

export async function getTorrentStats(): Promise<TorrentStat[]> {
	return invoke<TorrentStat[]>("get_torrent_stats");
}

export async function getWindowIsMaximized(): Promise<boolean> {
	return invoke<boolean>("get_window_is_maximized");
}

export async function maximizeWindow(): Promise<void> {
	return invoke<void>("maximize_window");
}

export async function unmaximizeWindow(): Promise<void> {
	return invoke<void>("unmaximize_window");
}

export async function minimizeWindow(): Promise<void> {
	return invoke<void>("minimize_window");
}

export async function closeWindow(): Promise<void> {
	return invoke<void>("close_window");
}

export async function openUrl(url: string): Promise<void> {
	return invoke<void>("open_url", { url });
}

export async function parseTorrentPlayInfo(
	torrentId: string,
): Promise<PlayInfo> {
	return invoke<PlayInfo>("parse_torrent_play_info_v2", { torrentId });
}

export async function setProgress(
	epId: number,
	progress: number,
): Promise<void> {
	const intProgress = Math.floor(progress);
	return invoke<void>("set_progress", { epId, progress: intProgress });
}

export async function fullscreenWindow(): Promise<void> {
	return invoke<void>("fullscreen_window");
}

export async function unfullscreenWindow(): Promise<void> {
	return invoke<void>("unfullscreen_window");
}

export async function getHistory(): Promise<[Anime, Episode][]> {
	return invoke<[Anime, Episode][]>("get_history");
}

export async function removeTorrent(torrentId: string): Promise<void> {
	return invoke<void>("remove_torrent", { torrentId });
}

export async function torrentIsPresent(epId: number): Promise<string | null> {
	return invoke<string | null>("torrent_is_present", { epId });
}

export async function listAnimes(): Promise<Record<string, Anime[]>> {
	return invoke<Record<string, Anime[]>>("list_animes");
}

export async function getLastWatchedEp(
	animeId: number,
): Promise<number | null> {
	return invoke<number | null>("get_last_watched_ep", { animeId });
}

export async function getDashboardSummary(): Promise<DashboardSummary> {
	return invoke<DashboardSummary>("get_dashboard_summary");
}

export async function getConfig(): Promise<Config> {
	return invoke<Config>("get_config");
}

export async function changeLocale(locale: string): Promise<Config> {
	return invoke<Config>("change_locale", { locale });
}

export async function setBangumiProxy(
	enabled: boolean,
	proxy?: string,
): Promise<Config> {
	return invoke<Config>("set_bangumi_proxy", { proxy, enabled });
}

export async function setTorrentsProxy(
	enabled: boolean,
	proxy?: string,
): Promise<Config> {
	return invoke<Config>("set_torrents_proxy", { proxy, enabled });
}

export async function selectDownloadPath(): Promise<Config> {
	return invoke<Config>("select_download_path");
}

export async function setLogLevel(level: LogLevelFilter): Promise<Config> {
	return invoke<Config>("set_log_level", { level });
}

export async function getAirCalendar(): Promise<[Anime, Episode][][]> {
	return invoke<[Anime, Episode][][]>("get_air_calendar");
}

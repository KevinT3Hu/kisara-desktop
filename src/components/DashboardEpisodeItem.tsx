import type { Anime, Episode } from "@/commands/types";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

export default function DashboardEpisodeItem({
    anime,
    episode,
    navToAnime = false,
}: {
    anime: Anime;
    episode: Episode;
    navToAnime?: boolean;
}) {
    const { t } = useTranslation();
    const navigate = useNavigate();

    function navigateEpisode() {
        if (navToAnime) {
            navigate(`/addedAnime/${anime.id}`);
        } else {
            navigate(`/play/${episode.id}`);
        }
    }

    return (
        <div className="w-[145px] flex flex-col">
            <img
                src={anime.image}
                alt={anime.name}
                className="w-[140px] h-[198px] object-cover hover:cursor-pointer"
                onClick={navigateEpisode}
            />
            <p className="text-lg text-gray-700 line-clamp-2">
                {anime.name_cn}
            </p>
            <p className="text-sm text-gray-500 line-clamp-2">
                {t("episode_num", {
                    num: episode.ep ?? episode.sort,
                })}
            </p>
        </div>
    );
}

import type { Anime, Episode } from "@/commands/types";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

export default function CalendarGridItem({
    anime,
    episode,
}: {
    anime: Anime;
    episode: Episode;
    classNames?: string;
}) {
    const { t } = useTranslation();
    const navigate = useNavigate();

    function navigateToAnime() {
        navigate(`/addedAnime/${anime.id}`);
    }

    return (
        <div
            onClick={navigateToAnime}
            className="flex flex-row justify-start items-start w-full py-1 cursor-pointer hover:bg-gray-100 transition-colors duration-200 ease-in-out"
        >
            <img
                src={anime.image}
                alt={anime.name_cn}
                className="w-16 h-auto object-fill"
            />
            <div className="ml-4">
                <h3 className="text-md font-semibold line-clamp-3">
                    {anime.name_cn}
                </h3>
                <p className="text-sm text-gray-600">
                    {t("episode_num", { num: episode.ep ?? episode.sort })}
                </p>
            </div>
        </div>
    );
}

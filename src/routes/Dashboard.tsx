import { getDashboardSummary } from "@/commands/commands";
import type { DashboardSummary, Episode } from "@/commands/types";
import { useCurrentTitle } from "@/states";
import dayjs from "dayjs";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router";

export default function Dashboard() {
    const { t } = useTranslation();
    const [dashboardSummary, setDashboardSummary] = useState<
        DashboardSummary | undefined
    >(undefined);
    const [loading, setLoading] = useState(true);
    const setTitle = useCurrentTitle((state) => state.updateTitle);
    const navigate = useNavigate();

    const todayDate = useMemo(() => {
        const d = dayjs();
        const date = d.format(t("date_format_short"));
        const weekday = d.day();
        const weekdayName = t(`weekdays.${weekday}`);
        return `${date} ${weekdayName}`;
    }, [t]);

    useEffect(() => {
        getDashboardSummary()
            .then((summary) => {
                setDashboardSummary(summary);
                setLoading(false);
            })
            .catch((err) => {
                console.error(err);
                setLoading(false);
            });
    }, []);

    useEffect(() => {
        setTitle(t("dashboard_title"));
    }, [setTitle, t]);

    if (loading) {
        return <div>{t("loading")}</div>;
    }
    if (!dashboardSummary) {
        return <div>{t("dashboard_error")}</div>;
    }

    function navigateAnime(animeId: number) {
        navigate(`/addedAnime/${animeId}`);
    }

    function navigateEpisode(ep: Episode) {
        navigate(`/play/${ep.torrent_id}`);
    }

    return (
        <div className="flex flex-col gap-2">
            <div className="flex flex-col justify-start items-start">
                <p className="select-none text-lg">{t("today_is")}</p>
                <p className="select-none text-5xl">{todayDate}</p>
            </div>
            {dashboardSummary.today.length > 0 && (
                <div className="flex flex-col justify-start items-start rounded-sm shadow-sm px-2 py-1 mt-2 hover:bg-gray-100">
                    <p className="text-xl py-2">
                        {t("dashboard_today", {
                            num: dashboardSummary.today.length,
                        })}
                    </p>
                    <div className="flex flex-row w-full overflow-x-auto gap-2 ">
                        {dashboardSummary.today.map((anime) => (
                            <div
                                key={anime.id}
                                className="w-[145px] flex flex-col"
                            >
                                <img
                                    src={anime.image}
                                    alt={anime.name}
                                    className="w-[140px] h-[198px] object-cover hover:cursor-pointer"
                                    onClick={() => navigateAnime(anime.id)}
                                />
                                <p className="text-lg text-gray-700 line-clamp-2">
                                    {anime.name}
                                </p>
                                <p className="text-sm text-gray-500 line-clamp-2">
                                    {anime.name_cn}
                                </p>
                            </div>
                        ))}
                    </div>
                </div>
            )}
            {dashboardSummary.last_watched.length > 0 && (
                <div className="flex flex-col justify-start items-start rounded-sm shadow-sm px-2 py-1 mt-2 hover:bg-gray-100">
                    <p className="text-xl py-2">{t("dashboard_continue")}</p>
                    <div className="flex flex-row w-full overflow-x-auto gap-2 ">
                        {dashboardSummary.last_watched.map(
                            ([anime, episode]) => (
                                <div
                                    key={episode.id}
                                    className="w-[145px] flex flex-col"
                                >
                                    <img
                                        src={anime.image}
                                        alt={anime.name}
                                        className="w-[140px] h-[198px] object-cover hover:cursor-pointer"
                                        onClick={() => navigateEpisode(episode)}
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
                            )
                        )}
                    </div>
                </div>
            )}
        </div>
    );
}

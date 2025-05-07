import { getDashboardSummary } from "@/commands/commands";
import type { DashboardSummary } from "@/commands/types";
import DashboardAnimeItem from "@/components/DashboardAnimeItem";
import DashboardEpisodeItem from "@/components/DashboardEpisodeItem";
import { useCurrentTitle } from "@/states";
import dayjs from "dayjs";
import { useEffect, useMemo, useState } from "react";
import { useTranslation } from "react-i18next";

export default function Dashboard() {
    const { t } = useTranslation();
    const [dashboardSummary, setDashboardSummary] = useState<
        DashboardSummary | undefined
    >(undefined);
    const [loading, setLoading] = useState(true);
    const setTitle = useCurrentTitle((state) => state.updateTitle);

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
                            <DashboardAnimeItem key={anime.id} anime={anime} />
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
                                <DashboardEpisodeItem
                                    key={episode.id}
                                    anime={anime}
                                    episode={episode}
                                />
                            )
                        )}
                    </div>
                </div>
            )}
            {dashboardSummary.watch_next.length > 0 && (
                <div className="flex flex-col justify-start items-start rounded-sm shadow-sm px-2 py-1 mt-2 hover:bg-gray-100">
                    <p className="text-xl py-2">{t("dashboard_watch_next")}</p>
                    <div className="flex flex-row w-full overflow-x-auto gap-2 ">
                        {dashboardSummary.watch_next.map(([anime, episode]) => (
                            <DashboardEpisodeItem
                                key={episode.id}
                                anime={anime}
                                episode={episode}
                                navToAnime
                            />
                        ))}
                    </div>
                </div>
            )}
        </div>
    );
}

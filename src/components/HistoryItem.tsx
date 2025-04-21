import type { Anime, Episode } from "@/commands/types";
import dayjs from "dayjs";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";

function secsToHMS(secs: number) {
    const h = Math.floor(secs / 3600);
    const m = Math.floor((secs % 3600) / 60);
    const s = Math.floor(secs % 60);
    return `${h}:${m.toString().padStart(2, "0")}:${s
        .toString()
        .padStart(2, "0")}`;
}

export default function HistoryItem({
    ep,
    anime,
}: {
    ep: Episode;
    anime: Anime;
}) {
    const { t } = useTranslation();

    const formatDate = useMemo(() => {
        return (date: string) => {
            const d = dayjs(date);
            return d.format(t("date_format"));
        };
    }, [t]);

    return (
        <div className="flex flex-row justify-between rounded-lg shadow-sm p-2 m-1 hover:bg-gray-100 transition-all duration-200 select-none">
            <div className="flex flex-col items-start">
                <h2 className="font-bold">
                    {anime.name_cn} - E{ep.ep ?? ep.sort}
                </h2>
                <p className="text-sm text-gray-600">
                    {ep.last_watch_time ? formatDate(ep.last_watch_time) : ""}{" "}
                    {secsToHMS(ep.progress)}
                </p>
            </div>
        </div>
    );
}

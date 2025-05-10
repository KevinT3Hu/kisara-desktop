import { listAnimes } from "@/commands/commands";
import type { Anime } from "@/commands/types";
import GridAnimeItem from "@/components/GridAnimeItem";
import { Divider } from "@mantine/core";
import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

export default function List() {
    const { t } = useTranslation();

    const [data, setData] = useState<Record<string, Anime[]>>({});

    useEffect(() => {
        listAnimes().then((v) => {
            setData(v);
        });
    }, []);

    const getSeasonStr = useCallback(
        (season: string) => {
            const [year, seasonIdx] = season.split(",");
            const yearStr = t("year", { year });
            const seasonStr = t(`seasons.${seasonIdx}`);
            return `${yearStr} ${seasonStr}`;
        },
        [t]
    );

    return (
        <div className="flex flex-col justify-start items-start gap-2">
            <div className="w-full h-full flex flex-col flex-wrap gap-2">
                {Object.entries(data)
                    .reverse()
                    .map(([season, animes]) => (
                        <div
                            className="flex flex-col justify-start items-start"
                            key={season}
                        >
                            <h3 className="text-2xl mb-2">
                                {getSeasonStr(season)}
                            </h3>
                            <div className="flex flex-row flex-wrap gap-2">
                                {animes.map((anime) => (
                                    <GridAnimeItem
                                        key={anime.id}
                                        anime={anime}
                                    />
                                ))}
                            </div>
                            <Divider dir="horizontal" className="w-full my-2" />
                        </div>
                    ))}
            </div>
        </div>
    );
}

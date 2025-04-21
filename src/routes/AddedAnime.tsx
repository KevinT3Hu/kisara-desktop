import {
    getAnime,
    getEpisodes,
    getLastWatchedEp,
    initSearchTorrents,
} from "@/commands/commands";
import type {
    AnimeSearchResultItem,
    Episode,
    TorrentInfo,
} from "@/commands/types";
import AnimeSummary from "@/components/AnimeSummary";
import EpisodeItem from "@/components/EpisodeItem";
import TorrentsTable from "@/components/TorrentsTable";
import { useCurrentTitle } from "@/states";
import { Card, Drawer, Loader } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useParams } from "react-router";

export default function AddedAnime() {
    const { t } = useTranslation();

    const { animeId } = useParams();
    const [lastWatchedEpId, setLastWatchedEpId] = useState<number | undefined>(
        undefined
    );
    const setCurrentTitle = useCurrentTitle((state) => state.updateTitle);

    const [animeInfo, setAnimeInfo] = useState<
        AnimeSearchResultItem | undefined
    >(undefined);

    const [episodes, setEpisodes] = useState<Episode[]>([]);

    const [drawerOpen, { open, close }] = useDisclosure(false);

    useEffect(() => {
        setCurrentTitle("......");
    }, [setCurrentTitle]);

    useEffect(() => {
        getAnime(Number(animeId)).then((v) => {
            const name = v.name_cn.length > 0 ? v.name_cn : v.name;
            setCurrentTitle(name);
            setAnimeInfo(v);
        });
        getEpisodes(Number(animeId)).then((v) => {
            setEpisodes(v);
        });
        getLastWatchedEp(Number(animeId)).then((v) => {
            setLastWatchedEpId(v);
        });
    }, [animeId, setCurrentTitle]);

    const [searchingTorrents, setSearchingTorrents] = useState(false);
    const [torrentResults, setTorrentResults] = useState<
        Partial<{
            [x: string]: TorrentInfo[];
        }>
    >({});
    const [torrentEpId, setTorrentEpId] = useState<number | null>(null);
    function searchTorrents(epId: number) {
        setSearchingTorrents(true);
        setTorrentEpId(epId);
        open();
        initSearchTorrents(epId)
            .then((v) => {
                setTorrentResults(v);
            })
            .finally(() => {
                setSearchingTorrents(false);
            });
    }

    return (
        <>
            <div className="flex flex-col justify-start items-start w-full h-full px-4 gap-4">
                {animeInfo ? (
                    <>
                        <AnimeSummary anime={animeInfo} />
                        <Card
                            shadow="sm"
                            padding="lg"
                            radius="lg"
                            withBorder
                            className="w-full"
                        >
                            <Card.Section>
                                <h2 className="text-xl text-gray-700 m-4">
                                    {t("episodes")}
                                </h2>
                            </Card.Section>
                            <div className="flex flex-col">
                                {episodes.map((ep) => (
                                    <EpisodeItem
                                        ep={ep}
                                        key={ep.id}
                                        onClickMagnet={() =>
                                            searchTorrents(ep.id)
                                        }
                                        isLastWatched={
                                            lastWatchedEpId === ep.id
                                        }
                                    />
                                ))}
                            </div>
                        </Card>
                    </>
                ) : (
                    <div>Loading...</div>
                )}
            </div>
            <Drawer
                opened={drawerOpen}
                position="right"
                onClose={close}
                offset={8}
                radius="lg"
                title={t("search_results")}
                padding="xl"
                size="xl"
            >
                {searchingTorrents ? (
                    <div className="flex flex-col justify-center items-center w-full h-full">
                        <Loader size="lg" />
                    </div>
                ) : (
                    <div>
                        {Object.keys(torrentResults).length > 0 ? (
                            Object.entries(torrentResults).map(
                                ([sourceName, torrents]) => {
                                    if (torrents === undefined) return null;
                                    return (
                                        <TorrentsTable
                                            epId={torrentEpId!}
                                            source={sourceName}
                                            torrents={torrents}
                                            key={sourceName}
                                        />
                                    );
                                }
                            )
                        ) : (
                            <div>{t("search_results_none")}</div>
                        )}
                    </div>
                )}
            </Drawer>
        </>
    );
}

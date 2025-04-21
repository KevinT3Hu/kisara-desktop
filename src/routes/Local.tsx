import { getTorrentStats, removeTorrent as rT } from "@/commands/commands";
import type { TorrentStat } from "@/commands/types";
import TorrentItem from "@/components/TorrentItem";
import { Collapse, Modal, Button } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ChevronDown, ChevronRight } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { Item, Menu, useContextMenu } from "react-contexify";
import "react-contexify/dist/ReactContexify.css";
import { useTranslation } from "react-i18next";

const CONTEXT_MENU_ID = "torrent-context-menu";

export default function Local() {
    const { t } = useTranslation();

    const [torrents, setTorrents] = useState<TorrentStat[]>([]);

    const { show: showContextMenu } = useContextMenu({
        id: CONTEXT_MENU_ID,
    });

    const [completed, downloading] = useMemo(() => {
        const completed: TorrentStat[] = [];
        const downloading: TorrentStat[] = [];
        for (const torrent of torrents) {
            if (torrent.info.stats.finished) {
                completed.push(torrent);
            } else {
                downloading.push(torrent);
            }
        }
        return [completed, downloading];
    }, [torrents]);

    const [modalOpen, { open: openModal, close: closeModal }] =
        useDisclosure(false);

    const [downloadingOpen, { toggle: toggleDownloading }] =
        useDisclosure(false);

    const [completedOpen, { toggle: toggleCompleted }] = useDisclosure(true);

    useEffect(() => {
        getTorrentStats().then((torrents) => {
            setTorrents(torrents);
        });

        const interval = setInterval(() => {
            getTorrentStats().then((torrents) => {
                setTorrents(torrents);
            });
        }, 1000);

        return () => {
            clearInterval(interval);
        };
    }, []);

    const [removeTorrentId, setRemoveTorrentId] = useState<string | null>(null);
    function removeTorrent(torrentId: string) {
        setRemoveTorrentId(torrentId);
        openModal();
    }

    function doRemoveTorrent() {
        if (removeTorrentId) {
            rT(removeTorrentId)
                .then(() => {
                    setTorrents((prev) =>
                        prev.filter(
                            (torrent) => torrent.torrent_id !== removeTorrentId
                        )
                    );
                    setRemoveTorrentId(null);
                })
                .finally(() => {
                    closeModal();
                });
        }
    }

    useEffect(() => {
        if (!modalOpen) {
            setRemoveTorrentId(null);
        }
    }, [modalOpen]);

    function handleTorrentContextMenu(
        torrentId: string,
        event: React.MouseEvent<HTMLDivElement>
    ) {
        setRemoveTorrentId(torrentId);
        showContextMenu({
            event,
        });
    }

    return (
        <div className="flex flex-col w-full h-full">
            <div
                className="flex flex-row w-full items-center hover:cursor-pointer"
                onClick={toggleCompleted}
            >
                <div>{completedOpen ? <ChevronDown /> : <ChevronRight />}</div>
                <p className="text-xl select-none">{t("completed")}</p>
            </div>
            <Collapse in={completedOpen}>
                <div className="flex flex-col w-full">
                    {completed.map((torrent) => {
                        return (
                            <TorrentItem
                                key={torrent.torrent_id}
                                torrent={torrent.info}
                                epDisplay={torrent.ep_display}
                                torrentId={torrent.torrent_id}
                                onContextMenu={(e) =>
                                    handleTorrentContextMenu(
                                        torrent.torrent_id,
                                        e
                                    )
                                }
                            />
                        );
                    })}
                </div>
            </Collapse>
            <div
                className="flex flex-row w-full items-center hover:cursor-pointer"
                onClick={toggleDownloading}
            >
                <div>
                    {downloadingOpen ? <ChevronDown /> : <ChevronRight />}
                </div>
                <p className="text-xl select-none">{t("downloading")}</p>
            </div>
            <Collapse in={downloadingOpen}>
                <div className="flex flex-col w-full">
                    {downloading.map((torrent) => {
                        return (
                            <TorrentItem
                                key={torrent.torrent_id}
                                torrent={torrent.info}
                                epDisplay={torrent.ep_display}
                                torrentId={torrent.torrent_id}
                                onContextMenu={(e) =>
                                    handleTorrentContextMenu(
                                        torrent.torrent_id,
                                        e
                                    )
                                }
                            />
                        );
                    })}
                </div>
            </Collapse>

            <Modal
                opened={modalOpen}
                onClose={closeModal}
                title={t("confirm_remove_torrent")}
                centered
            >
                <div className="flex flex-col w-full h-full">
                    <p>{t("confirm_remove_torrent_text")}</p>
                    <div className="flex flex-row gap-2 mt-4 w-full justify-end">
                        <Button
                            color="red"
                            className="rounded-lg px-4 py-2"
                            onClick={doRemoveTorrent}
                        >
                            {t("remove")}
                        </Button>
                        <Button
                            className="rounded-lg px-4 py-2"
                            onClick={closeModal}
                        >
                            {t("cancel")}
                        </Button>
                    </div>
                </div>
            </Modal>

            <Menu id={CONTEXT_MENU_ID}>
                <Item onClick={() => removeTorrent(removeTorrentId!)}>
                    {t("torrent_remove")}
                </Item>
            </Menu>
        </div>
    );
}

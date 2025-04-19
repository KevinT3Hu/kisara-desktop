import { getTorrentStats } from "@/commands/commands";
import type { TorrentStat } from "@/commands/types";
import TorrentItem from "@/components/TorrentItem";
import { Collapse } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ChevronDown, ChevronRight } from "lucide-react";
import { useEffect, useMemo, useState } from "react";

export default function Local() {
    const [torrents, setTorrents] = useState<TorrentStat[]>([]);

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

    return (
        <div className="flex flex-col w-full h-full">
            <div
                className="flex flex-row w-full items-center hover:cursor-pointer"
                onClick={toggleCompleted}
            >
                <div>{completedOpen ? <ChevronDown /> : <ChevronRight />}</div>
                <p className="text-xl select-none">Completed</p>
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
                <p className="text-xl select-none">Downloading</p>
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
                            />
                        );
                    })}
                </div>
            </Collapse>
        </div>
    );
}

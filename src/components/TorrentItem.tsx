import type { ManagedTorrentInfo } from "@/commands/types";
import { ActionIcon, Progress } from "@mantine/core";
import { ArrowDown, ArrowUp, TvMinimalPlay } from "lucide-react";
import { useCallback, useMemo } from "react";
import { useNavigate } from "react-router";

export default function TorrentItem({
    epDisplay,
    torrent,
    torrentId,
}: {
    epDisplay: string;
    torrent: ManagedTorrentInfo;
    torrentId: string;
}) {
    const navigate = useNavigate();

    const progress = useMemo(() => {
        if (torrent.stats.finished) {
            return 100;
        }
        return Math.round(
            (torrent.stats.progress_bytes / torrent.stats.total_bytes) * 100
        );
    }, [
        torrent.stats.progress_bytes,
        torrent.stats.total_bytes,
        torrent.stats.finished,
    ]);

    const bytesToHumanReadable = useCallback((bytes: number): string => {
        const units = ["B", "KB", "MB", "GB", "TB"];
        let index = 0;
        let adjustedBytes = bytes;
        while (adjustedBytes >= 1024 && index < units.length - 1) {
            adjustedBytes /= 1024;
            index++;
        }
        return `${adjustedBytes.toFixed(2)} ${units[index]}`;
    }, []);

    const uploaded = useMemo(() => {
        if (torrent.stats.uploaded_bytes === 0) {
            return "0 B";
        }
        return bytesToHumanReadable(torrent.stats.uploaded_bytes);
    }, [torrent.stats.uploaded_bytes, bytesToHumanReadable]);

    function play() {
        navigate(`/play/${torrentId}`);
    }

    return (
        <div className="flex flex-row justify-between rounded-lg shadow-sm p-2 m-1 hover:bg-gray-100 transition-all duration-200 select-none">
            <div className="flex flex-col ">
                <div className="flex flex-row flex-wrap gap-1 justify-start items-center">
                    <p className="text-base">{torrent.name}</p>
                    <p className="text-sm rounded-full bg-cyan-200 px-1 py-0.5">
                        {epDisplay}
                    </p>
                </div>
                {!torrent.stats.finished && (
                    <Progress
                        className="w-full my-1"
                        size="xs"
                        value={progress}
                    />
                )}
                <div className="flex flex-row gap-2 items-center justify-start">
                    {!torrent.stats.finished && torrent.stats.live && (
                        <div className="flex flex-row items-center">
                            <ArrowDown size={16} />
                            <p>
                                {
                                    torrent.stats.live?.download_speed
                                        .human_readable
                                }
                            </p>
                        </div>
                    )}
                    {torrent.stats.live && (
                        <div className="flex flex-row items-center">
                            <ArrowUp size={16} />
                            <p>
                                {
                                    torrent.stats.live?.upload_speed
                                        .human_readable
                                }
                            </p>
                        </div>
                    )}
                    <div className="flex flex-row items-center">
                        <p>Uploaded</p>
                        <p>{uploaded}</p>
                    </div>
                    {torrent.stats.error && (
                        <p className="text-red-500">{torrent.stats.error}</p>
                    )}
                </div>
            </div>
            <div className="flex flex-row items-center">
                <ActionIcon variant="subtle" onClick={play}>
                    <TvMinimalPlay />
                </ActionIcon>
            </div>
        </div>
    );
}

import { torrentIsPresent } from "@/commands/commands";
import type { Episode } from "@/commands/types";
import { ActionIcon } from "@mantine/core";
import { Magnet, TvMinimalPlay } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router";

export default function EpisodeItem({
    ep,
    onClickMagnet,
    isLastWatched = false,
}: {
    ep: Episode;
    onClickMagnet: () => void;
    isLastWatched?: boolean;
}) {
    const navigate = useNavigate();

    const [torrentId, setTorrentId] = useState<string | null>(null);

    const torrentPresent = useMemo(() => {
        if (torrentId === null) {
            return false;
        }
        return torrentId.length > 0;
    }, [torrentId]);

    useEffect(() => {
        torrentIsPresent(ep.id).then((v) => {
            setTorrentId(v);
        });
    }, [ep.id]);

    function onClickPlayOrMagnet() {
        if (torrentPresent) {
            navigate(`/play/${torrentId}`);
        } else {
            onClickMagnet();
        }
    }

    return (
        <div
            key={ep.id}
            className="flex flex-row items-center justify-between py-2 hover:bg-gray-100 px-4 rounded-md hover:cursor-pointer transition-all duration-200"
        >
            <div className="flex flex-row items-center gap-2 justify-start">
                <div className="flex items-center justify-center w-10 h-10 ">
                    <p className="text-lg text-gray-700 select-none">
                        {ep.ep ?? ep.sort}
                    </p>
                </div>
                <div className="flex flex-col items-start">
                    {ep.name_cn.length > 0 ? (
                        <>
                            <p className="text-sm text-gray-500 line-clamp-2">
                                {ep.name_cn}
                            </p>
                            <p className="text-sm text-gray-500 line-clamp-2">
                                {ep.name}
                            </p>
                        </>
                    ) : (
                        <p className="text-sm text-gray-500 line-clamp-2">
                            {ep.name}
                        </p>
                    )}
                </div>
                <div className="h-full flex flex-col justify-start items-start">
                    <p>{ep.air_date}</p>
                </div>
                {isLastWatched && (
                    <div className="h-full flex flex-col justify-start items-start">
                        <p className="text-xs text-gray-500 px-1 py-0.5 rounded-full border-blue-300 border-2">
                            Last watched
                        </p>
                    </div>
                )}
            </div>
            <div className="flex flex-row gap-2 justify-end">
                <ActionIcon
                    variant="outline"
                    size="lg"
                    onClick={onClickPlayOrMagnet}
                >
                    {torrentPresent ? <TvMinimalPlay /> : <Magnet />}
                </ActionIcon>
            </div>
        </div>
    );
}

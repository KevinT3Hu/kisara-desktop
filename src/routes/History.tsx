import { getHistory } from "@/commands/commands";
import type { Anime, Episode } from "@/commands/types";
import HistoryItem from "@/components/HistoryItem";
import { useEffect, useState } from "react";

export default function History() {
    const [history, setHistory] = useState<[Anime, Episode][]>([]);

    useEffect(() => {
        getHistory().then(setHistory).catch(console.error);
    }, []);

    return (
        <div className="flex flex-col size-full">
            {history.map(([anime, ep]) => (
                <HistoryItem key={ep.id} ep={ep} anime={anime} />
            ))}
        </div>
    );
}

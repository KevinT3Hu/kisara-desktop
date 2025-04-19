import { addAnime } from "@/commands/commands";
import type { AnimeSearchResultItem } from "@/commands/types";
import { Button } from "@mantine/core";
import { useState } from "react";

export default function SearchAnimeItem({
    item,
}: {
    item: AnimeSearchResultItem & { added: boolean };
}) {
    const [addedState, setAddedState] = useState(item.added);

    function addToList() {
        addAnime(item)
            .then(() => {
                setAddedState(true);
            })
            .catch((e) => {
                console.error(e);
            });
    }

    return (
        <div className="flex flex-row items-start justify-start gap-2 p-4 hover:bg-gray-100 transition-all duration-200 ease-in-out rounded-md">
            <img
                src={item.images.common}
                alt={item.name}
                className="w-[100px] h-auto"
            />
            <div className="flex flex-col justify-start items-start gap-2">
                <p className="text-lg text-gray-700">{item.name}</p>
                <p className="text-sm text-gray-500">{item.name_cn}</p>
                <p className="text-sm text-gray-500 line-clamp-3 overflow-ellipsis">
                    {item.summary}
                </p>
                {addedState ? (
                    <p className="text-sm text-green-500">已添加到列表</p>
                ) : (
                    <Button onClick={addToList}>添加到列表</Button>
                )}
            </div>
        </div>
    );
}

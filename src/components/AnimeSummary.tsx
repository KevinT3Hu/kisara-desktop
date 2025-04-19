import type { AnimeSearchResultItem } from "@/commands/types";

export default function AnimeSummary({
    anime,
}: {
    anime: AnimeSearchResultItem;
}) {
    return (
        <div className="flex flex-row items-start gap-4">
            <img
                src={anime.images.common}
                alt={anime.name}
                className="w-[120px] h-[170px] object-cover"
            />
            <div className="flex flex-col justify-start items-start ml-2">
                {anime.name_cn.length > 0 ? (
                    <>
                        <p className="text-xl text-gray-700">{anime.name_cn}</p>
                        <p className="text-sm text-gray-500 line-clamp-2">
                            {anime.name}
                        </p>
                    </>
                ) : (
                    <p className="text-xl text-gray-700">{anime.name}</p>
                )}
                <p className="text-gray-700">{anime.date}</p>
                <div>{anime.meta_tags.join(" / ")}</div>
            </div>
        </div>
    );
}

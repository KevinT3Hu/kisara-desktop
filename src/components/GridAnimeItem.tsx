import type { Anime } from "@/commands/types";
import { useNavigate } from "react-router";

export default function GridAnimeItem({ anime }: { anime: Anime }) {
    const navigate = useNavigate();

    function navigateAnime(animeId: number) {
        navigate(`/addedAnime/${animeId}`);
    }

    return (
        <div key={anime.id} className="w-[145px] flex flex-col">
            <img
                src={anime.image}
                alt={anime.name}
                className="w-[140px] h-[198px] object-cover hover:cursor-pointer"
                onClick={() => navigateAnime(anime.id)}
            />
            <p className="text-lg text-gray-700 line-clamp-1">{anime.name}</p>
            <p className="text-sm text-gray-500 line-clamp-1">
                {anime.name_cn}
            </p>
        </div>
    );
}

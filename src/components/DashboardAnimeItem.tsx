import type { Anime } from "@/commands/types";
import { useNavigate } from "react-router";

export default function DashboardAnimeItem({ anime }: { anime: Anime }) {
    const navigate = useNavigate();
    function navigateAnime() {
        navigate(`/addedAnime/${anime.id}`);
    }

    return (
        <div className="w-[145px] flex flex-col">
            <img
                src={anime.image}
                alt={anime.name}
                className="w-[140px] h-[198px] object-cover hover:cursor-pointer"
                onClick={navigateAnime}
            />
            <p className="text-lg text-gray-700 line-clamp-2">{anime.name}</p>
            <p className="text-sm text-gray-500 line-clamp-2">
                {anime.name_cn}
            </p>
        </div>
    );
}

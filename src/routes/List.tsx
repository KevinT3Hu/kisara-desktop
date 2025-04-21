import { listAnimes } from "@/commands/commands";
import type { Anime } from "@/commands/types";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router";

export default function List() {
    const [data, setData] = useState<Anime[]>([]);

    const navigate = useNavigate();

    useEffect(() => {
        listAnimes().then((v) => {
            setData(v);
        });
    }, []);

    function navigateAnime(animeId: number) {
        navigate(`/addedAnime/${animeId}`);
    }

    return (
        <div className="flex flex-col justify-start items-start gap-2">
            <div className="w-full h-full flex-wrap gap-2">
                {data.map((anime) => (
                    <div key={anime.id} className="w-[145px] flex flex-col">
                        <img
                            src={anime.image}
                            alt={anime.name}
                            className="w-[140px] h-[198px] object-cover hover:cursor-pointer"
                            onClick={() => navigateAnime(anime.id)}
                        />
                        <p className="text-lg text-gray-700 line-clamp-2">
                            {anime.name}
                        </p>
                        <p className="text-sm text-gray-500 line-clamp-2">
                            {anime.name_cn}
                        </p>
                    </div>
                ))}
            </div>
        </div>
    );
}

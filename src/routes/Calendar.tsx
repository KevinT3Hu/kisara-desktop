import { getAirCalendar } from "@/commands/commands";
import type { Anime, Episode } from "@/commands/types";
import CalendarGridItem from "@/components/CalendarGridItem";
import dayjs from "dayjs";
import { useEffect, useState } from "react";

export default function Calendar() {
    const [data, setData] = useState<[Anime, Episode][][]>([]);
    const [dates, setDates] = useState<[string, string][]>([]);

    useEffect(() => {
        getAirCalendar().then((v) => {
            console.log(v);
            setData(v);
        });

        const today = dayjs();
        // format [weekday, MM/DD]
        const week = Array.from({ length: 7 }, (_, i) => {
            const date = today.add(i, "day");
            return [date.format("ddd"), date.format("MM/DD")] as [
                string,
                string
            ];
        });
        setDates(week);
    }, []);

    return (
        <div className="grid grid-cols-7 divide-x divide-gray-200 h-full">
            {dates.map(([day, date], i) => (
                <div
                    key={day}
                    className="flex flex-col divide-y divide-gray-200 items-center h-full"
                >
                    <div>
                        <div className="text-sm text-gray-500">{day}</div>
                        <div className="text-lg font-bold">{date}</div>
                    </div>
                    {data[i]?.map(([anime, episode]) => (
                        <CalendarGridItem
                            key={anime.id}
                            anime={anime}
                            episode={episode}
                        />
                    ))}
                </div>
            ))}
        </div>
    );
}

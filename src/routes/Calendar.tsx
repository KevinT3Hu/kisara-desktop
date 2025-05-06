import { getAirCalendar } from "@/commands/commands";
import type { Anime, Episode } from "@/commands/types";
import CalendarGridItem from "@/components/CalendarGridItem";
import { Tooltip } from "@mantine/core";
import dayjs from "dayjs";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

export default function Calendar() {
    const { t } = useTranslation();
    const [data, setData] = useState<[Anime, Episode][][]>([]);
    const [dates, setDates] = useState<[string, string][]>([]);

    useEffect(() => {
        getAirCalendar().then((v) => {
            console.log(v);
            setData(v);
        });
    }, []);

    useEffect(() => {
        const today = dayjs();
        // format [weekday, MM/DD]
        const week = Array.from({ length: 7 }, (_, i) => {
            const date = today.add(i, "day");
            // get dayOfWeek starting from Sunday
            const dayOfWeek = date.day();
            const weekday = t(`weekdays_short.${dayOfWeek}`);
            return [weekday, date.format("MM/DD")] as [string, string];
        });
        setDates(week);
    }, [t]);

    return (
        <div className="h-full w-full overflow-x-auto">
            <div className="grid grid-flow-col auto-cols-[250px] divide-x divide-gray-200 h-full select-none">
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
                            <Tooltip label={anime.name_cn} key={anime.id}>
                                <CalendarGridItem
                                    key={anime.id}
                                    anime={anime}
                                    episode={episode}
                                />
                            </Tooltip>
                        ))}
                    </div>
                ))}
            </div>
        </div>
    );
}

import {
    ArrowDownToDot,
    Cast,
    Gauge,
    HistoryIcon,
    ListVideo,
    Search,
} from "lucide-react";
import { useCurrentTitle, useDownloadingNum } from "@/states";
import { NavLink } from "react-router";
import cn from "classnames";

const sidebarItems = [
    {
        title: "Dashboard",
        icon: <Gauge />,
        url: "/",
    },
    {
        title: "Current On",
        icon: <Cast />,
        url: "/current-on",
    },
    {
        title: "History",
        icon: <HistoryIcon />,
        url: "/history",
    },
    {
        title: "List",
        icon: <ListVideo />,
        url: "/list",
    },
    {
        title: "Local",
        icon: <ArrowDownToDot />,
        url: "/local",
    },
    {
        title: "Search",
        icon: <Search />,
        url: "/search",
    },
] as const;

export default function KisaraSidebar() {
    const setCurrentTitle = useCurrentTitle((state) => state.updateTitle);
    const num = useDownloadingNum((state) => state.num);

    return (
        <div>
            <div className="flex flex-col w-full h-full p-2">
                {sidebarItems.map((item) => (
                    <NavLink
                        key={item.title}
                        to={item.url}
                        className={({ isActive }) =>
                            cn(
                                "flex flex-row items-center gap-2 p-2 rounded-md hover:bg-gray-200 transition-colors",
                                {
                                    "bg-gray-200": isActive,
                                }
                            )
                        }
                        onClick={() => setCurrentTitle(item.title)}
                    >
                        {item.icon}
                        <p>{item.title}</p>
                        {item.title === "Local" && num > 0 && (
                            <span className="ml-auto text-sm font-bold text-red-500">
                                {num}
                            </span>
                        )}
                    </NavLink>
                ))}
            </div>
        </div>
    );
}

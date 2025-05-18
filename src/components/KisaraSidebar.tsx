import {
    ArrowDownToDot,
    BoltIcon,
    CalendarClock,
    Gauge,
    HistoryIcon,
    ListVideo,
    Search,
} from "lucide-react";
import { useCurrentTitle, useDownloadingNum } from "@/states";
import { NavLink, useNavigate } from "react-router";
import cn from "classnames";
import { ActionIcon } from "@mantine/core";
import { useTranslation } from "react-i18next";

const sidebarItems = [
    {
        titleKey: "dashboard_title",
        icon: <Gauge />,
        url: "/",
    },
    // {
    //     titleKey: "current_title",
    //     icon: <Cast />,
    //     url: "/current-on",
    // },
    {
        titleKey: "calendar_title",
        icon: <CalendarClock />,
        url: "/calendar",
    },
    {
        titleKey: "history_title",
        icon: <HistoryIcon />,
        url: "/history",
    },
    {
        titleKey: "list_title",
        icon: <ListVideo />,
        url: "/list",
    },
    {
        titleKey: "local_title",
        icon: <ArrowDownToDot />,
        url: "/local",
    },
    {
        titleKey: "search_title",
        icon: <Search />,
        url: "/search",
    },
] as const;

export default function KisaraSidebar() {
    const setCurrentTitle = useCurrentTitle((state) => state.updateTitle);
    const num = useDownloadingNum((state) => state.num);
    const navigate = useNavigate();

    const { t } = useTranslation();

    return (
        <div className="size-full">
            <div className="flex flex-col justify-between w-full h-full p-2">
                <div className="flex flex-col gap-0.5 w-full">
                    {sidebarItems.map((item) => (
                        <NavLink
                            viewTransition
                            key={item.titleKey}
                            to={item.url}
                            className={({ isActive }) =>
                                cn(
                                    "flex flex-row items-center gap-2 p-2 rounded-md hover:bg-gray-200 transition-colors",
                                    {
                                        "bg-gray-200": isActive,
                                    }
                                )
                            }
                            onClick={() => setCurrentTitle(t(item.titleKey))}
                        >
                            {item.icon}
                            <p>{t(item.titleKey)}</p>
                            {item.titleKey === "local_title" && num > 0 && (
                                <span className="ml-auto text-sm font-bold text-red-500">
                                    {num}
                                </span>
                            )}
                        </NavLink>
                    ))}
                </div>
                <div className="flex flex-row items-center justify-start w-full">
                    <ActionIcon
                        size={40}
                        variant="outline"
                        onClick={() => {
                            navigate("/settings");
                        }}
                    >
                        <BoltIcon size={32} />
                    </ActionIcon>
                </div>
            </div>
        </div>
    );
}

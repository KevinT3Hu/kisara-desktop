import { Expand, Minimize, Minus, XIcon } from "lucide-react";
import { useEffect, useState } from "react";
import { ActionIcon } from "@mantine/core";
import {
    closeWindow,
    getWindowIsMaximized,
    maximizeWindow,
    minimizeWindow,
    unmaximizeWindow,
} from "@/commands/commands";

export default function CloseAppBar() {
    const [maximized, setMaximized] = useState(false);

    useEffect(() => {
        getWindowIsMaximized()
            .then((isMaximized) => {
                setMaximized(isMaximized);
            })
            .catch((error) => {
                console.error("Error getting window state:", error);
            });
    }, []);

    function closeApp() {
        closeWindow();
    }

    function minimizeApp() {
        minimizeWindow();
    }

    function toggleMaximize() {
        if (maximized) {
            unmaximizeWindow();
        } else {
            maximizeWindow();
        }
        setMaximized(!maximized);
    }

    return (
        <div className="flex flex-row gap-2">
            <ActionIcon variant="subtle" onClick={minimizeApp}>
                <Minus />
            </ActionIcon>
            <ActionIcon variant="subtle" onClick={toggleMaximize}>
                {maximized ? <Minimize /> : <Expand />}
            </ActionIcon>
            <ActionIcon variant="subtle" onClick={closeApp}>
                <XIcon />
            </ActionIcon>
        </div>
    );
}

import CloseAppBar from "@/components/CloseAppBar";
import KisaraSidebar from "@/components/KisaraSidebar";
import { useCurrentTitle, useDownloadingNum } from "@/states";
import { AppShell, Burger, ScrollArea } from "@mantine/core";
import { Outlet } from "react-router";
import { useDisclosure } from "@mantine/hooks";
import { useEffect } from "react";
import { getDownloadingTorrentsNum } from "@/commands/commands";

export default function Home() {
    const title = useCurrentTitle((state) => state.title);
    const { setNum } = useDownloadingNum((state) => state);

    const [open, { toggle }] = useDisclosure(false);

    useEffect(() => {
        getDownloadingTorrentsNum().then((v) => {
            setNum(v);
        });
        const interval = setInterval(() => {
            getDownloadingTorrentsNum().then((v) => {
                setNum(v);
            });
        }, 1000);
        return () => {
            clearInterval(interval);
        };
    }, [setNum]);

    return (
        <AppShell
            padding="md"
            header={{ height: 60 }}
            navbar={{
                width: 200,
                breakpoint: "sm",
                collapsed: { mobile: !open },
            }}
        >
            <AppShell.Header>
                <div
                    className="flex flex-row items-center justify-between w-full h-full px-2 select-none"
                    data-tauri-drag-region
                >
                    <div className="flex flex-row items-center gap-2">
                        <Burger
                            opened={open}
                            onClick={toggle}
                            hiddenFrom="sm"
                        />
                        <p className="text-xl font-bold">Kisara</p>
                    </div>
                    <p className="text-sm font-semibold">{title}</p>
                    <CloseAppBar />
                </div>
            </AppShell.Header>
            <AppShell.Navbar>
                <KisaraSidebar />
            </AppShell.Navbar>
            <AppShell.Main>
                <ScrollArea h="calc(100vh - 80px)">
                    <Outlet />
                </ScrollArea>
            </AppShell.Main>
        </AppShell>
    );
}

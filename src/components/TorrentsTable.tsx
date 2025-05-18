import { addTorrent, openUrl } from "@/commands/commands";
import type { TorrentInfo } from "@/commands/types";
import { ActionIcon, Table } from "@mantine/core";
import { ArrowDownToLine, PanelTop } from "lucide-react";
import { useTranslation } from "react-i18next";

export default function TorrentsTable({
    epId,
    source,
    torrents,
}: {
    epId: number;
    source: string;
    torrents: TorrentInfo[];
}) {
    const { t } = useTranslation();

    function openUrlHandler(url: string) {
        openUrl(url);
    }

    function addTorrentF(magnet: string) {
        addTorrent(magnet, epId).catch((e) => {
            console.error(e);
        });
    }

    return (
        <div className="flex flex-col items-start justify-start w-full px-2">
            <h2 className="text-xl">{source}</h2>
            <Table>
                <Table.Thead>
                    <Table.Tr>
                        <Table.Th>{t("torrent_name")}</Table.Th>
                        <Table.Th>{t("torrent_size")}</Table.Th>
                        <Table.Th>{t("torrent_date")}</Table.Th>
                        <Table.Th>{t("torrent_seeders")}</Table.Th>
                        <Table.Th>{t("torrent_leechers")}</Table.Th>
                        <Table.Th>{t("torrent_uploader")}</Table.Th>
                        <Table.Th>{t("torrent_actions")}</Table.Th>
                    </Table.Tr>
                </Table.Thead>
                <Table.Tbody>
                    {torrents.map((torrent) => (
                        <Table.Tr key={torrent.magnet}>
                            <Table.Td>{torrent.name}</Table.Td>
                            <Table.Td>{torrent.size}</Table.Td>
                            <Table.Td>{torrent.date}</Table.Td>
                            <Table.Td>{torrent.seeders}</Table.Td>
                            <Table.Td>{torrent.leechers}</Table.Td>
                            <Table.Td>{torrent.uploader}</Table.Td>
                            <Table.Td>
                                <div className="flex flex-row gap-2">
                                    <ActionIcon.Group>
                                        <ActionIcon
                                            variant="outline"
                                            onClick={() =>
                                                addTorrentF(torrent.magnet)
                                            }
                                        >
                                            <ArrowDownToLine />
                                        </ActionIcon>
                                        {torrent.url !== null && (
                                            <ActionIcon
                                                variant="outline"
                                                onClick={() =>
                                                    openUrlHandler(torrent.url!)
                                                }
                                            >
                                                <PanelTop />
                                            </ActionIcon>
                                        )}
                                    </ActionIcon.Group>
                                </div>
                            </Table.Td>
                        </Table.Tr>
                    ))}
                </Table.Tbody>
            </Table>
        </div>
    );
}

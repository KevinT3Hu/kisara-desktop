import { getAnimeById, setAnimeKeywords } from "@/commands/commands";
import type { Anime, AnimeSearchResultItem } from "@/commands/types";
import { ActionIcon, Button, Modal, Input, Divider } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { PlusIcon, SquarePen, X } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

export default function AnimeSummary({
    anime,
}: {
    anime: AnimeSearchResultItem;
}) {
    const [
        editKeywordsOpen,
        { open: openEditKeywords, close: closeEditKeywords },
    ] = useDisclosure(false);

    const [animeInfo, setAnimeInfo] = useState<Anime | undefined>(undefined);

    // 新增本地关键字数组
    const [localKeywords, setLocalKeywords] = useState<string[]>([]);
    // 新增：控制是否显示新增关键字输入框
    const [addingKeyword, setAddingKeyword] = useState(false);
    const [newKeywordValue, setNewKeywordValue] = useState("");

    useEffect(() => {
        getAnimeById(anime.id).then((v) => {
            setAnimeInfo(v);
            setLocalKeywords(v?.keywords ?? []);
        });
    }, [anime.id]);

    const { t } = useTranslation();

    // 删除关键字
    const handleDeleteKeyword = (keyword: string) => {
        setLocalKeywords((prev) => prev.filter((k) => k !== keyword));
    };

    // 新增关键字输入框行为
    const handleAddKeyword = () => {
        setAddingKeyword(true);
        setNewKeywordValue("");
    };

    // 输入框确认添加
    const confirmAddKeyword = () => {
        const value = newKeywordValue.trim();
        if (value && !localKeywords.includes(value)) {
            setLocalKeywords((prev) => [...prev, value]);
        }
        setAddingKeyword(false);
        setNewKeywordValue("");
    };

    // 确认按钮点击事件（可根据需要扩展）
    const handleConfirm = () => {
        setAnimeKeywords(anime.id, localKeywords)
            .catch((error) => {
                console.error("Error setting keywords:", error);
            })
            .finally(() => {
                closeEditKeywords();
            });
    };

    return (
        <>
            <div className="flex flex-row items-start gap-4">
                <img
                    src={anime.images.common}
                    alt={anime.name}
                    className="w-[120px] h-[170px] object-cover"
                />
                <div className="flex flex-col justify-start items-start ml-2">
                    <div className="flex flex-row items-center gap-2">
                        {anime.name_cn.length > 0 ? (
                            <div className="flex flex-col">
                                <p className="text-xl text-gray-700">
                                    {anime.name_cn}
                                </p>
                                <p className="text-sm text-gray-500 line-clamp-2">
                                    {anime.name}
                                </p>
                            </div>
                        ) : (
                            <p className="text-xl text-gray-700">
                                {anime.name}
                            </p>
                        )}
                        <ActionIcon variant="subtle" onClick={openEditKeywords}>
                            <SquarePen />
                        </ActionIcon>
                    </div>
                    <p className="text-gray-700">{anime.date}</p>
                    <div>{anime.meta_tags.join(" / ")}</div>
                </div>
            </div>

            <Modal
                opened={editKeywordsOpen}
                onClose={closeEditKeywords}
                title={t("edit_keywords")}
            >
                <div className="flex flex-col gap-0.5">
                    <ul>
                        <li className="px-2 py-1 hover:bg-gray-200 rounded-md">
                            {anime.name}
                        </li>
                        <Divider my={2} />
                        <li className="px-2 py-1 hover:bg-gray-200 rounded-md">
                            {anime.name_cn}
                        </li>
                        {animeInfo?.aliases.length ? (
                            <>
                                <Divider my={2} />
                                {animeInfo.aliases.map((alias, idx) => (
                                    <div key={alias}>
                                        <li className="px-2 py-1 hover:bg-gray-200 rounded-md">
                                            {alias}
                                        </li>
                                        {idx !==
                                            animeInfo.aliases.length - 1 && (
                                            <Divider my={2} />
                                        )}
                                    </div>
                                ))}
                            </>
                        ) : null}
                        {localKeywords.length > 0 && <Divider my={2} />}
                        {/* 关键字列表，带删除按钮 */}
                        {localKeywords.map((keyword, idx) => (
                            <div key={keyword}>
                                <li className="px-2 py-1 hover:bg-gray-200 rounded-md flex flex-row items-center justify-between">
                                    <span>{keyword}</span>
                                    <ActionIcon
                                        size="sm"
                                        variant="light"
                                        color="red"
                                        onClick={() =>
                                            handleDeleteKeyword(keyword)
                                        }
                                        aria-label={t("delete")}
                                    >
                                        <X size={16} />
                                    </ActionIcon>
                                </li>
                                {idx !== localKeywords.length - 1 && (
                                    <Divider my={2} />
                                )}
                            </div>
                        ))}
                        {/* 新增关键字输入框 */}
                        {addingKeyword ? (
                            <>
                                {localKeywords.length > 0 && <Divider my={2} />}
                                <li className="px-2 py-1 rounded-md flex flex-row items-center gap-2 bg-gray-100">
                                    <Input
                                        autoFocus
                                        className="flex-1"
                                        value={newKeywordValue}
                                        onChange={(e) =>
                                            setNewKeywordValue(e.target.value)
                                        }
                                        onBlur={confirmAddKeyword}
                                        onKeyDown={(e) => {
                                            if (e.key === "Enter") {
                                                confirmAddKeyword();
                                            } else if (e.key === "Escape") {
                                                setAddingKeyword(false);
                                                setNewKeywordValue("");
                                            }
                                        }}
                                        placeholder={t("input_new_keyword")}
                                        size="sm"
                                    />
                                </li>
                            </>
                        ) : (
                            <>
                                <Divider my={2} />
                                <li
                                    className="px-2 py-1 hover:bg-gray-200 rounded-md cursor-pointer"
                                    onClick={handleAddKeyword}
                                >
                                    <div className="flex flex-row items-center justify-between">
                                        <p className="text-gray-500">
                                            {t("new_keyword")}
                                        </p>
                                        <PlusIcon className="text-gray-500" />
                                    </div>
                                </li>
                            </>
                        )}
                    </ul>
                    {/* 确认按钮 */}
                    <Button
                        className="mt-2 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
                        onClick={handleConfirm}
                    >
                        {t("confirm")}
                    </Button>
                </div>
            </Modal>
        </>
    );
}

import { SearchIcon } from "lucide-react";
import { useState } from "react";
import { ActionIcon, Autocomplete, Pagination, Select } from "@mantine/core";
import SearchAnimeItem from "@/components/SearchAnimeItem";
import type { AnimeSearchResultItem } from "@/commands/types";
import {
    animesInList,
    searchAnimes,
    searchSuggestions,
} from "@/commands/commands";
import { useTranslation } from "react-i18next";

const sortTypes = [
    {
        value: "Match",
        labelKey: "sort_match",
    },
    {
        value: "Heat",
        labelKey: "sort_heat",
    },
    {
        value: "Rank",
        labelKey: "sort_rank",
    },
    {
        value: "Score",
        labelKey: "sort_score",
    },
] as const;

type SortType = (typeof sortTypes)[number]["value"];

export default function Search() {
    const { t } = useTranslation();

    const [sortType, setSortType] = useState<SortType>("Match");

    const [suggestions, setSuggestions] = useState<string[]>([]);
    const [query, setQuery] = useState<string>("");

    const [data, setData] = useState<
        (AnimeSearchResultItem & { added: boolean })[]
    >([]);
    const [totalPage, setTotalPage] = useState<number>(1);
    const [page, setPage] = useState<number>(1);
    let timeout: NodeJS.Timeout | null = null;

    function getSuggestions(query: string) {
        searchSuggestions(query).then((v) => {
            setSuggestions(v);
        });
    }

    function onKeywordChange(value: string) {
        const v = value.trim();
        setQuery(v);
        if (timeout) {
            clearTimeout(timeout);
        }
        if (v.length > 0) {
            // get suggestions if idle for 1500ms
            console.log("suggestions", v);

            timeout = setTimeout(() => getSuggestions(v), 1500);
        } else {
            setSuggestions([]);
        }
    }

    function search(page: number) {
        searchAnimes(query, sortType, page).then((v) => {
            animesInList(v.data.map((item) => item.id))
                .then((ret) => {
                    const data = v.data.map((item, index) => {
                        return {
                            ...item,
                            added: ret[index],
                        };
                    });
                    console.log("data", data);
                    setData(data);
                })
                .catch((e) => {
                    console.error(e);
                });
            const totalPage = Math.ceil(v.total / v.limit);
            setTotalPage(totalPage);
            const currentPage = Math.floor(v.offset / v.limit) + 1;
            setPage(currentPage);
        });
    }

    return (
        <div className="flex flex-col p-2 h-full">
            <div className="flex flex-row w-full items-center gap-2">
                <Autocomplete
                    leftSection={<SearchIcon />}
                    onChange={onKeywordChange}
                    value={query}
                    data={suggestions}
                    placeholder="Search..."
                    radius="md"
                    size="md"
                />
                <Select
                    data={sortTypes.map((type) => ({
                        value: type.value,
                        label: t(type.labelKey),
                    }))}
                    value={sortType}
                    onChange={(value) => setSortType(value as SortType)}
                />
                <ActionIcon
                    onClick={() => search(1)}
                    variant="outline"
                    size="lg"
                >
                    <SearchIcon />
                </ActionIcon>
            </div>
            <div className="flex flex-col mt-2 w-full">
                {data.map((item) => (
                    <SearchAnimeItem item={item} key={item.id} />
                ))}
                {totalPage > 1 && (
                    <Pagination
                        value={page}
                        total={totalPage}
                        onChange={(s) => {
                            search(s);
                        }}
                    />
                )}
            </div>
        </div>
    );
}

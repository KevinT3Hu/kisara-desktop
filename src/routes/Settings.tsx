import { getConfig } from "@/commands/commands";
import type { Config } from "@/commands/types";
import { useCurrentTitle } from "@/states";
import { Select } from "@mantine/core";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

const languages = [
    { value: "zh", label: "简体中文" },
    { value: "en", label: "English" },
    { value: "ja", label: "日本語" },
] as const;

export default function Settings() {
    const { t, i18n } = useTranslation();

    const setTitle = useCurrentTitle((state) => state.updateTitle);

    const [config, setConfig] = useState<Config | undefined>(undefined);

    useEffect(() => {
        setTitle(t("settings_title"));
        getConfig().then((v) => {
            setConfig(v);
        });
    }, [setTitle, t]);

    function changeLanguage(lang: string | null) {
        if (lang) {
            i18n.changeLanguage(lang, (e, t) => {
                if (e) {
                    console.error(e);
                }
                console.log("i18n", t);
            });
        }
    }

    return (
        <div>
            <Select
                value={config?.locale}
                onChange={changeLanguage}
                data={languages}
            />
        </div>
    );
}

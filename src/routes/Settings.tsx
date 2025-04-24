import {
    getConfig,
    selectDownloadPath,
    setBangumiProxy,
    setLogLevel,
    setTorrentsProxy,
} from "@/commands/commands";
import type { Config, LogLevelFilter } from "@/commands/types";
import { useCurrentTitle } from "@/states";
import { Button, Input, Select, Switch, TableOfContents } from "@mantine/core";
import { useEffect, useMemo, useState } from "react";
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
    const [bgmProxyTmp, setBgmProxyTmp] = useState<string | undefined>(
        undefined
    );
    const [trsProxyTmp, setTrsProxyTmp] = useState<string | undefined>(
        undefined
    );

    const logLevels = useMemo(() => {
        return ["error", "warn", "info", "debug", "trace"].map((v, i) => {
            return {
                value: v,
                label: t(`log_level.${i}`),
            };
        });
    }, [t]);

    useEffect(() => {
        setTitle(t("settings_title"));
        getConfig().then((v) => {
            setConfig(v);
        });
    }, [setTitle, t]);

    useEffect(() => {
        setBgmProxyTmp(config?.network_config.bgm_proxy);
        setTrsProxyTmp(config?.network_config.torrents_proxy);
    }, [config]);

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

    function setEnableBgmProxy(enabled: boolean) {
        setBangumiProxy(enabled).then((c) => {
            setConfig(c);
        });
    }

    function setBgmProxy() {
        setBangumiProxy(true, bgmProxyTmp).then((c) => {
            setConfig(c);
        });
    }

    function setEnableTorrentsProxy(enabled: boolean) {
        setTorrentsProxy(enabled).then((c) => {
            setConfig(c);
        });
    }

    function setTrsProxy() {
        setTorrentsProxy(true, trsProxyTmp).then((c) => {
            setConfig(c);
        });
    }

    function chooseDownloadDirectory() {
        selectDownloadPath().then((c) => {
            setConfig(c);
        });
    }

    function setLLevel(level: LogLevelFilter) {
        setLogLevel(level).then((c) => {
            setConfig(c);
        });
    }

    return (
        <div className="flex flex-row">
            <div className="flex flex-col justify-start items-start gap-2 grow">
                <div className="flex flex-col justify-start items-start gap-1">
                    <h2 className="text-2xl font-bold mb-2">
                        {t("settings_ui")}
                    </h2>
                    <div className="flex flex-wrap justify-start items-start">
                        <div className="flex flex-row items-center gap-2">
                            <span>{t("settings_language")}</span>
                            <Select
                                value={config?.locale}
                                onChange={changeLanguage}
                                data={languages}
                            />
                        </div>
                    </div>
                </div>
                <div className="flex flex-col justify-start items-start gap-1">
                    <h2 className="text-2xl font-bold mb-2">
                        {t("settings_network")}
                    </h2>
                    <div className="flex flex-col justify-start items-start gap-2">
                        <div className="flex flex-row items-center gap-2">
                            <span>{t("settings_network_proxy_bgm")}</span>
                            <Switch
                                checked={
                                    config?.network_config.bgm_proxy_enabled
                                }
                                onChange={(e) =>
                                    setEnableBgmProxy(e.currentTarget.checked)
                                }
                            />
                            <Input
                                placeholder={t(
                                    "settings_network_proxy_bgm_placeholder"
                                )}
                                value={config?.network_config.bgm_proxy}
                                onChange={(e) =>
                                    setBgmProxyTmp(e.currentTarget.value)
                                }
                                onBlur={setBgmProxy}
                                onKeyDown={(e) => {
                                    if (e.key === "Enter") {
                                        setBgmProxy();
                                    }
                                }}
                                disabled={
                                    !config?.network_config.bgm_proxy_enabled
                                }
                            />
                        </div>
                        <div className="flex flex-row items-center gap-2">
                            <span>{t("settings_network_proxy_torrents")}</span>
                            <Switch
                                checked={
                                    config?.network_config
                                        .torrents_proxy_enabled
                                }
                                onChange={(e) =>
                                    setEnableTorrentsProxy(
                                        e.currentTarget.checked
                                    )
                                }
                            />
                            <Input
                                placeholder={t(
                                    "settings_network_proxy_bgm_placeholder"
                                )}
                                value={config?.network_config.torrents_proxy}
                                onChange={(e) =>
                                    setTrsProxyTmp(e.currentTarget.value)
                                }
                                onBlur={setTrsProxy}
                                onKeyDown={(e) => {
                                    if (e.key === "Enter") {
                                        setTrsProxy();
                                    }
                                }}
                                disabled={
                                    !config?.network_config
                                        .torrents_proxy_enabled
                                }
                            />
                        </div>
                    </div>
                </div>
                <div className="flex flex-col justify-start items-start gap-1">
                    <h2 className="text-2xl font-bold mb-2">
                        {t("settings_download")}
                    </h2>
                    <div className="flex flex-col justify-start items-start gap-2">
                        <div className="flex flex-row items-center gap-2">
                            <span>{t("settings_download_path")}</span>
                            <Button
                                onClick={chooseDownloadDirectory}
                                variant="outline"
                            >
                                {config?.download_config.download_path}
                            </Button>
                        </div>
                    </div>
                </div>
                <div className="flex flex-col justify-start items-start gap-1">
                    <h2 className="text-2xl font-bold mb-2">
                        {t("settings_debug")}
                    </h2>
                    <div className="flex flex-col justify-start items-start gap-2">
                        <div className="flex flex-row items-center gap-2">
                            <span>{t("settings_debug_log")}</span>
                            <Select
                                value={config?.debug_config.log_level}
                                onChange={(e) => setLLevel(e as LogLevelFilter)}
                                data={logLevels}
                            />
                        </div>
                    </div>
                </div>
            </div>
            <div className="w-[200px]">
                <TableOfContents
                    scrollSpyOptions={{
                        selector: "h2",
                    }}
                    getControlProps={({ data }) => ({
                        onClick: () => data.getNode().scrollIntoView(),
                        children: data.value,
                    })}
                />
            </div>
        </div>
    );
}

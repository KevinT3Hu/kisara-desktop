import {
    fullscreenWindow,
    parseTorrentPlayInfo,
    setProgress as sP,
    unfullscreenWindow,
} from "@/commands/commands";
import { useCurrentTitle } from "@/states";
import { ActionIcon, Select, Slider } from "@mantine/core";
import {
    Fullscreen,
    Pause,
    PlayIcon,
    RotateCcw,
    RotateCw,
    Volume2,
    VolumeX,
} from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { useParams } from "react-router";
import cn from "classnames";
import { useTranslation } from "react-i18next";
import { convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export default function Play() {
    const { t } = useTranslation();
    const videoRef = useRef<HTMLVideoElement | null>(null);
    const trackRef = useRef<HTMLTrackElement | null>(null);
    const videoContainerRef = useRef<HTMLDivElement | null>(null);
    const params = useParams();
    const setTitle = useCurrentTitle((state) => state.updateTitle);
    const title = useCurrentTitle((state) => state.title);
    const [videoSrc, setVideoSrc] = useState<string | null>(null);
    const [trackList, setTrackList] = useState<string[]>([]);
    const [track, setTrack] = useState<string | null>(null);
    const [epId, setEpId] = useState<number | null>(null);
    const [playing, setPlaying] = useState(false);
    const [progress, setProgress] = useState(0);
    const [isFullscreen, setIsFullscreen] = useState(false);
    const [shouldShowFullscreenControls, setShouldShowFullscreenControls] =
        useState(false);
    const [mouseMoveTimeout, setMouseMoveTimeout] =
        useState<NodeJS.Timeout | null>(null);
    const [isOverlayHovered, setIsOverlayHovered] = useState(false);
    const [volume, setVolume] = useState(1); // 0~1
    const [isMuted, setIsMuted] = useState(false);

    const tracksData = useMemo(
        () =>
            trackList.map((track) => {
                // split the track name from the file path
                let name = track.split(/[/\\]/).pop() || track;
                // remove the file extension
                name = name.replace(/\.[^/.]+$/, "");
                return {
                    value: track,
                    label: name,
                };
            }),
        [trackList]
    );

    // set track to the first one if not set
    useEffect(() => {
        if (track === null && trackList.length > 0) {
            setTrack(trackList[0]);
        }
    }, [track, trackList]);

    // 加载剧集播放信息，设置视频源、字幕、标题，并在canplay时跳转到上次进度
    useEffect(() => {
        if (params.torrentId === undefined) return;
        parseTorrentPlayInfo(params.torrentId).then((info) => {
            const src = convertFileSrc(info.video);
            setEpId(info.ep.id);
            setVideoSrc(src);
            setTrackList(info.subtitles);
            const displayTitle = `${info.anime.name_cn} ${t("episode_num", {
                num: info.ep.ep ?? info.ep.sort,
            })} ${info.ep.name_cn}`;
            setTitle(displayTitle);
            videoRef.current?.addEventListener(
                "canplay",
                () => {
                    // seek to 5 secs before the last watch time
                    seekTo(Math.max(0, info.ep.progress - 5));
                },
                { once: true }
            );
        });
    }, [params.torrentId, setTitle, t]);

    // 处理播放状态、进度、全屏、快捷键、鼠标移动等事件监听和清理
    useEffect(() => {
        const playListener = () => {
            setPlaying(true);
        };
        const pauseListener = () => {
            setPlaying(false);
        };
        const timeUpdateListener = () => {
            if (videoRef.current) {
                setProgress(videoRef.current.currentTime);
                if (epId) {
                    sP(epId, videoRef.current.currentTime);
                }
            }
        };
        const fullscreenChangeListener = () => {
            if (document.fullscreenElement) {
                fullscreenWindow();
                setIsFullscreen(true);
            } else {
                unfullscreenWindow();
                setIsFullscreen(false);
            }
        };
        const mouseMoveListener = (_: MouseEvent) => {
            // 如果鼠标当前在overlay上，不做任何处理
            if (isOverlayHovered) return;

            setShouldShowFullscreenControls(true);

            if (mouseMoveTimeout) {
                clearTimeout(mouseMoveTimeout);
            }

            setMouseMoveTimeout(
                setTimeout(() => {
                    if (!isOverlayHovered) {
                        setShouldShowFullscreenControls(false);
                    }
                }, 5000)
            );
        };
        const shortcutListener = (e: KeyboardEvent) => {
            if (videoRef.current) {
                switch (e.key) {
                    case " ":
                        togglePlay();
                        break;
                    case "f":
                        fullscreen();
                        break;
                    case "ArrowLeft":
                        seekRelative(-10);
                        break;
                    case "ArrowRight":
                        seekRelative(10);
                        break;
                }
            }
        };

        videoRef.current?.addEventListener("play", playListener);
        videoRef.current?.addEventListener("playing", playListener);
        videoRef.current?.addEventListener("pause", pauseListener);
        videoRef.current?.addEventListener("ended", pauseListener);
        videoRef.current?.addEventListener("timeupdate", timeUpdateListener);
        videoContainerRef.current?.addEventListener(
            "fullscreenchange",
            fullscreenChangeListener
        );
        videoContainerRef.current?.addEventListener(
            "mousemove",
            mouseMoveListener
        );
        document.addEventListener("keydown", shortcutListener);

        return () => {
            videoRef.current?.removeEventListener("play", playListener);
            videoRef.current?.removeEventListener("playing", playListener);
            videoRef.current?.removeEventListener("pause", pauseListener);
            videoRef.current?.removeEventListener("ended", pauseListener);
            videoRef.current?.removeEventListener(
                "timeupdate",
                timeUpdateListener
            );
            videoContainerRef.current?.removeEventListener(
                "fullscreenchange",
                fullscreenChangeListener
            );
            videoContainerRef.current?.removeEventListener(
                "mousemove",
                mouseMoveListener
            );
            document.removeEventListener("keydown", shortcutListener);
        };
    }, [isOverlayHovered, mouseMoveTimeout, epId]);

    // 监听窗口关闭事件，自动暂停视频
    useEffect(() => {
        let unlisten = () => {};
        listen("tauri://close-requested", () => {
            if (videoRef.current) {
                videoRef.current.pause();
            }
        }).then((unsub) => {
            unlisten = unsub;
        });
        return () => {
            unlisten();
        };
    }, []);

    // 同步音量和静音状态到 video 元素
    useEffect(() => {
        if (videoRef.current) {
            videoRef.current.volume = volume;
            videoRef.current.muted = isMuted;
        }
    }, [volume, isMuted]);

    function seekTo(time: number) {
        setProgress(time);
        if (videoRef.current) {
            videoRef.current.currentTime = time;
        }
    }

    const [progressReadable, durationReadable] = useMemo(() => {
        if (videoRef.current) {
            const duration = videoRef.current.duration;
            const progressReadable = new Date(progress * 1000)
                .toISOString()
                .substr(11, 8);
            const durationReadable = new Date(duration * 1000)
                .toISOString()
                .substr(11, 8);
            return [progressReadable, durationReadable];
        }
        return ["00:00:00", "00:00:00"];
    }, [progress]);

    function seekRelative(time: number) {
        if (videoRef.current) {
            videoRef.current.currentTime = Math.max(
                0,
                Math.min(videoRef.current.duration, progress + time)
            );
        }
    }

    function togglePlay() {
        if (videoRef.current) {
            if (videoRef.current.paused) {
                videoRef.current.play();
            } else {
                videoRef.current.pause();
            }
        }
    }

    function fullscreen() {
        if (videoContainerRef.current) {
            if (videoContainerRef.current.requestFullscreen) {
                videoContainerRef.current.requestFullscreen();
            }
        }
    }

    function exitFullscreen() {
        if (videoContainerRef.current) {
            if (document.exitFullscreen) {
                document.exitFullscreen();
            }
        }
    }

    function toggleFullscreen() {
        if (isFullscreen) {
            exitFullscreen();
        } else {
            fullscreen();
        }
    }

    const handleOverlayMouseEnter = () => {
        setIsOverlayHovered(true);
        setShouldShowFullscreenControls(true);
        if (mouseMoveTimeout) {
            clearTimeout(mouseMoveTimeout);
        }
    };

    const handleOverlayMouseLeave = () => {
        setIsOverlayHovered(false);
        setMouseMoveTimeout(
            setTimeout(() => {
                setShouldShowFullscreenControls(false);
            }, 5000)
        );
    };

    return (
        <div className="flex flex-col items-center justify-center size-full">
            <div
                className="w-full h-full flex items-center justify-center"
                ref={videoContainerRef}
            >
                <video
                    ref={videoRef}
                    src={videoSrc || undefined}
                    crossOrigin="anonymous"
                    autoPlay
                    className={cn({
                        "max-h-[70%] max-w-[90%]": !isFullscreen,
                        "w-full h-full object-contain": isFullscreen,
                    })}
                    onClick={togglePlay}
                    onDoubleClick={toggleFullscreen}
                >
                    <track
                        ref={trackRef}
                        kind="captions"
                        srcLang="en"
                        default
                        src={convertFileSrc(track || "")}
                    />
                </video>
                {isFullscreen && (
                    <>
                        <div
                            className={cn(
                                "absolute top-0 left-0 w-full flex flex-row px-[20px] py-[10px] items-center justify-start z-10 transition-transform duration-300 ease-in-out",
                                {
                                    "translate-y-0":
                                        shouldShowFullscreenControls ||
                                        isOverlayHovered,
                                    "-translate-y-full":
                                        !shouldShowFullscreenControls &&
                                        !isOverlayHovered,
                                }
                            )}
                            onMouseEnter={handleOverlayMouseEnter}
                            onMouseLeave={handleOverlayMouseLeave}
                        >
                            <p className="text-white text-2xl">{title}</p>
                        </div>
                        <div
                            className={cn(
                                "absolute bottom-0 left-0 w-full flex flex-col px-[20px] pb-2 z-10 transition-transform duration-300 ease-in-out",
                                {
                                    "translate-y-0":
                                        shouldShowFullscreenControls ||
                                        isOverlayHovered,
                                    "translate-y-full":
                                        !shouldShowFullscreenControls &&
                                        !isOverlayHovered,
                                }
                            )}
                            onMouseEnter={handleOverlayMouseEnter}
                            onMouseLeave={handleOverlayMouseLeave}
                        >
                            <Slider
                                value={progress}
                                size="sm"
                                min={0}
                                step={0.01}
                                max={videoRef.current?.duration}
                                label={progressReadable}
                                onChange={seekTo}
                                className="w-full grow my-2"
                            />
                            <div className="flex flex-row items-center justify-between">
                                <div className="flex flex-row items-center gap-x-2">
                                    <ActionIcon
                                        size={40}
                                        variant="subtle"
                                        onClick={() => {
                                            playing
                                                ? videoRef.current?.pause()
                                                : videoRef.current?.play();
                                        }}
                                    >
                                        {playing ? (
                                            <Pause color="white" size={32} />
                                        ) : (
                                            <PlayIcon color="white" size={32} />
                                        )}
                                    </ActionIcon>
                                    <ActionIcon
                                        size={32}
                                        variant="subtle"
                                        onClick={() => setIsMuted((m) => !m)}
                                    >
                                        {isMuted || volume === 0 ? (
                                            <VolumeX color="white" size={24} />
                                        ) : (
                                            <Volume2 color="white" size={24} />
                                        )}
                                    </ActionIcon>
                                    <Slider
                                        value={isMuted ? 0 : volume}
                                        min={0}
                                        max={1}
                                        step={0.01}
                                        onChange={(v) => {
                                            setVolume(v);
                                            if (v === 0) setIsMuted(true);
                                            else setIsMuted(false);
                                        }}
                                        className="w-[80px] mx-2"
                                        styles={{
                                            track: { backgroundColor: "#fff" },
                                            thumb: { borderColor: "#fff" },
                                        }}
                                    />
                                    <p className="text-white">{`${progressReadable} / ${durationReadable}`}</p>
                                </div>
                                <ActionIcon
                                    size={40}
                                    variant="subtle"
                                    onClick={exitFullscreen}
                                >
                                    <Fullscreen color="white" size={32} />
                                </ActionIcon>
                            </div>
                        </div>
                    </>
                )}
            </div>

            <div className="flex flex-row items-center justify-center w-[90%] gap-2 select-none">
                <p className="text-gray-500">{progressReadable}</p>
                <Slider
                    value={progress}
                    min={0}
                    step={0.01}
                    max={videoRef.current?.duration}
                    label={progressReadable}
                    onChange={seekTo}
                    className="w-full grow my-2"
                />
                <p className="text-gray-500">{durationReadable}</p>
            </div>

            <div className="flex flex-row items-center justify-between w-[90%] gap-2 my-2">
                <div className="flex-1">
                    {trackList.length > 0 && (
                        <Select
                            className="w-[200px]"
                            data={tracksData}
                            value={track}
                            onChange={setTrack}
                        />
                    )}
                </div>
                <div className="flex flex-row items-center justify-center">
                    <ActionIcon
                        size={40}
                        variant="subtle"
                        onClick={() => {
                            seekRelative(-10);
                        }}
                    >
                        <RotateCcw size={32} />
                    </ActionIcon>
                    <ActionIcon
                        size={40}
                        variant="subtle"
                        onClick={() => {
                            playing
                                ? videoRef.current?.pause()
                                : videoRef.current?.play();
                        }}
                    >
                        {playing ? <Pause size={32} /> : <PlayIcon size={32} />}
                    </ActionIcon>
                    <ActionIcon
                        size={40}
                        variant="subtle"
                        onClick={() => {
                            seekRelative(10);
                        }}
                    >
                        <RotateCw size={32} />
                    </ActionIcon>
                    <ActionIcon size={40} variant="subtle" onClick={fullscreen}>
                        <Fullscreen size={32} />
                    </ActionIcon>
                </div>
                <div className="flex-1 flex flex-row items-center justify-end">
                    <ActionIcon
                        size={32}
                        variant="subtle"
                        onClick={() => setIsMuted((m) => !m)}
                    >
                        {isMuted || volume === 0 ? (
                            <VolumeX size={24} />
                        ) : (
                            <Volume2 size={24} />
                        )}
                    </ActionIcon>
                    <Slider
                        value={isMuted ? 0 : volume}
                        min={0}
                        max={1}
                        step={0.01}
                        onChange={(v) => {
                            setVolume(v);
                            if (v === 0) setIsMuted(true);
                            else setIsMuted(false);
                        }}
                        className="w-[80px] mx-2"
                    />
                </div>
            </div>
        </div>
    );
}

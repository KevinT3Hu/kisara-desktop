import {
    fullscreenWindow,
    parseTorrentPlayInfo,
    setProgress as sP,
    unfullscreenWindow,
} from "@/commands/commands";
import { useCurrentTitle } from "@/states";
import { ActionIcon, Slider } from "@mantine/core";
import { Fullscreen, Pause, PlayIcon, RotateCcw, RotateCw } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { useParams } from "react-router";

export default function Play() {
    const videoRef = useRef<HTMLVideoElement | null>(null);
    const params = useParams();
    const setTitle = useCurrentTitle((state) => state.updateTitle);
    const [videoSrc, setVideoSrc] = useState<string | null>(null);
    const [track, _] = useState<string | null>(null);
    const [epId, setEpId] = useState<number | null>(null);
    const [playing, setPlaying] = useState(false);
    const [progress, setProgress] = useState(0);

    useEffect(() => {
        if (params.torrentId === undefined) return;
        parseTorrentPlayInfo(params.torrentId).then((info) => {
            const src = `http://localhost:8080/${info.video}`;
            setEpId(info.ep.id);
            setVideoSrc(src);
            const displayTitle = `${info.anime.name_cn} - E${
                info.ep.ep ?? info.ep.sort
            } ${info.ep.name_cn}`;
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
    }, [params.torrentId, setTitle]);

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
            } else {
                unfullscreenWindow();
            }
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
        videoRef.current?.addEventListener(
            "fullscreenchange",
            fullscreenChangeListener
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
            videoRef.current?.removeEventListener(
                "fullscreenchange",
                fullscreenChangeListener
            );
            document.removeEventListener("keydown", shortcutListener);
        };
    });

    function seekTo(time: number) {
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
        if (videoRef.current) {
            if (videoRef.current.requestFullscreen) {
                videoRef.current.requestFullscreen();
            }
        }
    }

    return (
        <div className="flex flex-col items-center justify-center size-full">
            <video
                ref={videoRef}
                src={videoSrc || undefined}
                autoPlay
                className="max-h-[70%] max-w-[90%]"
                onClick={togglePlay}
                onDoubleClick={fullscreen}
            >
                <track kind="captions" srcLang="en" src={track || undefined} />
            </video>
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
        </div>
    );
}

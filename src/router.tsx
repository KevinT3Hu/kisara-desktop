import { createBrowserRouter } from "react-router";
import Home from "./routes/Home";
import { lazy } from "react";
import Dashboard from "./routes/Dashboard";

const Search = lazy(() => import("./routes/Search"));
const AddedAnime = lazy(() => import("./routes/AddedAnime"));
const Local = lazy(() => import("./routes/Local"));
const Play = lazy(() => import("./routes/Play"));
const History = lazy(() => import("./routes/History"));
const List = lazy(() => import("./routes/List"));
const Settings = lazy(() => import("./routes/Settings"));
const Calendar = lazy(() => import("./routes/Calendar"));
const CurrentOn = lazy(() => import("./routes/CurrentOn"));

const router = createBrowserRouter([
    {
        Component: Home,
        children: [
            {
                index: true,
                Component: Dashboard,
            },
            {
                path: "/current-on",
                Component: CurrentOn,
            },
            {
                path: "/calendar",
                Component: Calendar,
            },
            {
                path: "/history",
                Component: History,
            },
            {
                path: "/list",
                Component: List,
            },
            {
                path: "/local",
                Component: Local,
            },
            {
                path: "/search",
                Component: Search,
            },
            {
                path: "/addedAnime/:animeId",
                Component: AddedAnime,
            },
            {
                path: "/play/:torrentId",
                Component: Play,
            },
            {
                path: "/settings",
                Component: Settings,
            },
        ],
    },
]);

export default router;

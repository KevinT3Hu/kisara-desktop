import { createBrowserRouter } from "react-router";
import Home from "./routes/Home";
import Dashboard from "./routes/Dashboard";
import Search from "./routes/Search";
import CurrentOn from "./routes/CurrentOn";
import AddedAnime from "./routes/AddedAnime";
import Local from "./routes/Local";
import Play from "./routes/Play";
import History from "./routes/History";
import List from "./routes/List";
import Settings from "./routes/Settings";
import Calendar from "./routes/Calendar";

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

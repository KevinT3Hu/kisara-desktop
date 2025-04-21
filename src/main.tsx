import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router";
import router from "./router";
import "@mantine/core/styles.css";
import "mantine-contextmenu/styles.css";
import "./App.css";
import { MantineProvider } from "@mantine/core";
import "./i18n";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <MantineProvider>
            <RouterProvider router={router} />
        </MantineProvider>
    </React.StrictMode>
);

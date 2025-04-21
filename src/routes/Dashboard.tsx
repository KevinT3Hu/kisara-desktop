import { getDashboardSummary } from "@/commands/commands";
import type { DashboardSummary } from "@/commands/types";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

export default function Dashboard() {
    const { t } = useTranslation();
    const [dashboardSummary, setDashboardSummary] = useState<
        DashboardSummary | undefined
    >(undefined);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        getDashboardSummary()
            .then((summary) => {
                setDashboardSummary(summary);
                setLoading(false);
            })
            .catch((err) => {
                console.error(err);
                setLoading(false);
            });
    }, []);

    if (loading) {
        return <div>{t("loading")}</div>;
    }
    if (!dashboardSummary) {
        return <div>{t("dashboard_error")}</div>;
    }

    return <div>{t("dashboard_title")}</div>;
}

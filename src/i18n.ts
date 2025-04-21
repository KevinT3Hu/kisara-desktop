import i18n, { type ModuleType } from "i18next";
import { initReactI18next } from "react-i18next";
import { changeLocale, getConfig } from "@/commands/commands";
import zhResources from "./locales/zh.json";
import enResources from "./locales/en.json";
import jaResources from "./locales/ja.json";

const languageDetector = {
	type: "languageDetector" as ModuleType,
	async: true, // If this is set to true, your detect function receives a callback function that you should call with your language, useful to retrieve your language stored in AsyncStorage for example
	detect: async () => {
		const locale = await getConfig()
			.then((c) => c.locale)
			.catch(() => "zh");
		console.log("locale", locale);
		return locale;
	},
	cacheUserLanguage: async (locale: string) => {
		await changeLocale(locale).catch(() => {
			console.error("Failed to change locale");
		});
	},
};

i18n.use(initReactI18next)
	.use(languageDetector)
	.init({
		fallbackLng: "zh",
		resources: {
			zh: {
				translation: zhResources,
			},
			en: {
				translation: enResources,
			},
			ja: {
				translation: jaResources,
			},
		},
		interpolation: {
			escapeValue: false, // React already does escaping
		},
	});

export default i18n;

import { create } from "zustand";

type TitleState = {
	title: string;
	updateTitle: (newTitle: string) => void;
};

const useCurrentTitle = create<TitleState>((set) => ({
	title: "Dashboard",
	updateTitle: (newTitle: string) => {
		console.log("Setting title to:", newTitle);
		set({ title: newTitle });
	},
}));

type DownloadingNumState = {
	num: number;
	increase: () => void;
	decrease: () => void;
	setNum: (newNum: number) => void;
};

const useDownloadingNum = create<DownloadingNumState>((set) => ({
	num: 0,
	increase: () => set((state) => ({ num: state.num + 1 })),
	decrease: () => set((state) => ({ num: state.num - 1 })),
	setNum: (newNum: number) => set({ num: newNum }),
}));

export { useCurrentTitle, useDownloadingNum };

import { writable } from "svelte/store";
import { getSetting, setSetting, getAllSettings } from "../db/settingsDb";

export interface GeneralSettings {
	autoSave: boolean;
	showGrid: boolean;
	defaultChartType: "candlesticks" | "bar" | "line";
}

export interface TradeSettings {
	bidDistance: number;
	askDistance: number;
	spread: number;
	slippage: number;
	lotSize: number;
	maxOrders: number;
	stopLoss: number;
	takeProfit: number;
}

export interface AppearanceSettings {
	theme: "dark" | "light" | "auto";
	fontSize: "small" | "medium" | "large";
	accentColor: string;
}

export interface AppSettings {
	general: GeneralSettings;
	trade: TradeSettings;
	appearance: AppearanceSettings;
}

const defaultSettings: AppSettings = {
	general: {
		autoSave: true,
		showGrid: true,
		defaultChartType: "candlesticks",
	},
	trade: {
		bidDistance: 5,
		askDistance: 5,
		spread: 2,
		slippage: 1,
		lotSize: 0.01,
		maxOrders: 10,
		stopLoss: 50,
		takeProfit: 100,
	},
	appearance: {
		theme: "dark",
		fontSize: "medium",
		accentColor: "#3b82f6",
	},
};

function createSettingsStore() {
	const { subscribe, set, update } = writable<AppSettings>(defaultSettings);

	let isInitialized = false;

	return {
		subscribe,

		async init() {
			if (isInitialized) return;

			try {
				const savedSettings = await getAllSettings();

				const loadedSettings: AppSettings = {
					general: {
						...defaultSettings.general,
						...(savedSettings.general || {}),
					},
					trade: {
						...defaultSettings.trade,
						...(savedSettings.trade || {}),
					},
					appearance: {
						...defaultSettings.appearance,
						...(savedSettings.appearance || {}),
					},
				};

				set(loadedSettings);
				isInitialized = true;
				console.log("Settings loaded from database");
			} catch (error) {
				console.error("Failed to load settings:", error);
				set(defaultSettings);
			}
		},

		async updateGeneral<K extends keyof GeneralSettings>(
			key: K,
			value: GeneralSettings[K]
		) {
			update((state) => {
				const newState = {
					...state,
					general: { ...state.general, [key]: value },
				};
				setSetting("general", newState.general);
				return newState;
			});
		},

		async updateTrade<K extends keyof TradeSettings>(
			key: K,
			value: TradeSettings[K]
		) {
			update((state) => {
				const newState = {
					...state,
					trade: { ...state.trade, [key]: value },
				};
				setSetting("trade", newState.trade);
				return newState;
			});
		},

		async updateAppearance<K extends keyof AppearanceSettings>(
			key: K,
			value: AppearanceSettings[K]
		) {
			update((state) => {
				const newState = {
					...state,
					appearance: { ...state.appearance, [key]: value },
				};
				setSetting("appearance", newState.appearance);
				return newState;
			});
		},

		async reset() {
			set(defaultSettings);
			await setSetting("general", defaultSettings.general);
			await setSetting("trade", defaultSettings.trade);
			await setSetting("appearance", defaultSettings.appearance);
		},
	};
}

export const settingsStore = createSettingsStore();

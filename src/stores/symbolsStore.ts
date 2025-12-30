import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export interface TimeframeInfo {
	name: string;
	display_name: string;
	candle_count: number;
	file_path: string;
}

export interface SymbolData {
	symbol: string;
	timeframes: TimeframeInfo[];
	total_candles: number;
	uploaded_at: string;
}

function createSymbolsStore() {
	const { subscribe, set, update } = writable<SymbolData[]>([]);

	return {
		subscribe,
		loadSymbols: async () => {
			try {
				const symbols = await invoke<SymbolData[]>("get_available_symbols");
				set(symbols);
				return symbols;
			} catch (error) {
				console.error("Failed to load symbols:", error);
				return [];
			}
		},
		refresh: async () => {
			const symbols = await invoke<SymbolData[]>("get_available_symbols");
			set(symbols);
		},
	};
}

export const symbolsStore = createSymbolsStore();

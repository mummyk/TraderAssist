// src/stores/chartStore.ts
import { writable } from "svelte/store";

interface ChartState {
	selectedSymbol: string | null;
	selectedTimeframe: string | null;
	availableTimeframes: string[];
}

function createChartStore() {
	const { subscribe, set, update } = writable<ChartState>({
		selectedSymbol: null,
		selectedTimeframe: null,
		availableTimeframes: [],
	});

	return {
		subscribe,
		setSymbol: (symbol: string, timeframes: string[]) => {
			update((state) => ({
				...state,
				selectedSymbol: symbol,
				availableTimeframes: timeframes,
				selectedTimeframe: timeframes[0] || null,
			}));
		},
		setTimeframe: (timeframe: string) => {
			update((state) => ({
				...state,
				selectedTimeframe: timeframe,
			}));
		},
		reset: () => {
			set({
				selectedSymbol: null,
				selectedTimeframe: null,
				availableTimeframes: [],
			});
		},
	};
}

export const chartStore = createChartStore();

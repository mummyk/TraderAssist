import { writable } from "svelte/store";
import shortcutsConfig from "../shortcuts.json";

export interface Shortcut {
	id: string;
	name: string;
	keys: string[];
	description: string;
	action: string;
}

export interface ShortcutsState {
	shortcuts: Shortcut[];
	enabled: boolean;
}

function createShortcutsStore() {
	const { subscribe, set, update } = writable<ShortcutsState>({
		shortcuts: shortcutsConfig.shortcuts,
		enabled: true,
	});

	return {
		subscribe,
		loadShortcuts: () => {
			set({
				shortcuts: shortcutsConfig.shortcuts,
				enabled: true,
			});
		},
		updateShortcut: (id: string, keys: string[]) => {
			update((state) => ({
				...state,
				shortcuts: state.shortcuts.map((shortcut) =>
					shortcut.id === id ? { ...shortcut, keys } : shortcut
				),
			}));
		},
		toggleEnabled: () => {
			update((state) => ({
				...state,
				enabled: !state.enabled,
			}));
		},
		getShortcutByAction: (action: string): Shortcut | undefined => {
			let result: Shortcut | undefined;
			subscribe((state) => {
				result = state.shortcuts.find((s) => s.action === action);
			})();
			return result;
		},
	};
}

export const shortcutsStore = createShortcutsStore();

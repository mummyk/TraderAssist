import { shortcutsStore, type Shortcut } from "../stores/shortcutsStore";

type ActionHandler = () => void;

export class KeyboardHandler {
	private actionHandlers: Map<string, ActionHandler> = new Map();
	private shortcuts: Shortcut[] = [];
	private enabled = true;

	constructor() {
		shortcutsStore.subscribe((state) => {
			this.shortcuts = state.shortcuts;
			this.enabled = state.enabled;
		});

		this.setupListener();
	}

	private setupListener() {
		document.addEventListener("keydown", (e: KeyboardEvent) => {
			if (!this.enabled) return;

			// Don't trigger shortcuts when typing in inputs
			const target = e.target as HTMLElement;
			if (
				target.tagName === "INPUT" ||
				target.tagName === "TEXTAREA" ||
				target.isContentEditable
			) {
				return;
			}

			const key = this.normalizeKey(e);
			const matchedShortcut = this.shortcuts.find((shortcut) =>
				shortcut.keys.some((k) => this.matchesKey(k, key))
			);

			if (matchedShortcut) {
				e.preventDefault();
				const handler = this.actionHandlers.get(matchedShortcut.action);
				if (handler) {
					handler();
				}
			}
		});
	}

	private normalizeKey(e: KeyboardEvent): string {
		const parts: string[] = [];

		// Use cmd on Mac, ctrl on others
		const isMac = navigator.platform.toUpperCase().indexOf("MAC") >= 0;

		if (e.ctrlKey && !isMac) parts.push("ctrl");
		if (e.metaKey && isMac) parts.push("cmd");
		if (e.altKey) parts.push("alt");
		if (e.shiftKey) parts.push("shift");

		// Normalize key names
		let key = e.key.toLowerCase();
		if (key === " ") key = "space";
		if (key === "escape") key = "esc";

		parts.push(key);

		return parts.join("+");
	}

	private matchesKey(shortcutKey: string, pressedKey: string): boolean {
		// Normalize both keys for comparison
		const normalizeForComparison = (k: string) => {
			// Handle both cmd and ctrl as equivalent modifiers
			return k.replace(/^(cmd|ctrl)\+/, "mod+");
		};

		const normalizedShortcut = normalizeForComparison(shortcutKey);
		const normalizedPressed = normalizeForComparison(pressedKey);

		return normalizedShortcut === normalizedPressed;
	}

	registerAction(action: string, handler: ActionHandler) {
		this.actionHandlers.set(action, handler);
	}

	unregisterAction(action: string) {
		this.actionHandlers.delete(action);
	}

	getShortcutKeys(action: string): string[] {
		const shortcut = this.shortcuts.find((s) => s.action === action);
		return shortcut?.keys || [];
	}

	destroy() {
		this.actionHandlers.clear();
	}
}

export const keyboardHandler = new KeyboardHandler();

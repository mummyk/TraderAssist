<script lang="ts">
	import { onMount } from "svelte";
	import { settingsStore } from "../../../stores/settingsStore";
	import GeneralSettings from "../settings/GeneralSettings.svelte";
	import TradeSettings from "../settings/TradeSettings.svelte";
	import AppearanceSettings from "../settings/AppearanceSettings.svelte";
	import ShortcutsSettings from "../settings/ShortcutsSettings.svelte";

	type TabType = "general" | "trade" | "shortcuts" | "appearance";

	let activeTab = $state<TabType>("general");

	onMount(() => {
		// Initialize settings from database
		settingsStore.init();
	});

	function handleTabChange(tab: TabType) {
		activeTab = tab;
	}
</script>

<div class="settings-modal">
	<div class="settings-sidebar">
		<button
			class="sidebar-btn"
			class:active={activeTab === "general"}
			onclick={() => handleTabChange("general")}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="12" cy="12" r="3" />
				<path
					d="M12 1v6m0 6v6m-9-9h6m6 0h6m-2.636-6.364l-4.243 4.243m0 4.242l-4.243 4.243m12.728 0l-4.243-4.243m0-4.242l-4.243-4.243"
				/>
			</svg>
			General
		</button>

		<button
			class="sidebar-btn"
			class:active={activeTab === "trade"}
			onclick={() => handleTabChange("trade")}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<line x1="12" y1="1" x2="12" y2="23" />
				<path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
			</svg>
			Trade
		</button>

		<button
			class="sidebar-btn"
			class:active={activeTab === "shortcuts"}
			onclick={() => handleTabChange("shortcuts")}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<rect x="2" y="4" width="20" height="16" rx="2" />
				<path d="M6 8h.01M10 8h.01M14 8h.01M18 8h.01" />
				<path d="M8 12h8M7 16h10" />
			</svg>
			Shortcuts
		</button>

		<button
			class="sidebar-btn"
			class:active={activeTab === "appearance"}
			onclick={() => handleTabChange("appearance")}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z" />
			</svg>
			Appearance
		</button>
	</div>

	<div class="settings-content">
		{#if activeTab === "general"}
			<GeneralSettings />
		{:else if activeTab === "trade"}
			<TradeSettings />
		{:else if activeTab === "shortcuts"}
			<ShortcutsSettings />
		{:else if activeTab === "appearance"}
			<AppearanceSettings />
		{/if}
	</div>
</div>

<style>
	.settings-modal {
		display: flex;
		min-width: 700px;
		min-height: 500px;
	}

	.settings-sidebar {
		width: 200px;
		border-right: 1px solid var(--border-color);
		padding: 8px;
		display: flex;
		flex-direction: column;
		gap: 4px;
		background-color: var(--bg-secondary);
	}

	.sidebar-btn {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 12px 16px;
		background: transparent;
		border: none;
		border-radius: 6px;
		color: var(--text-secondary);
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.sidebar-btn:hover {
		background-color: var(--border-color);
		color: var(--text-primary);
	}

	.sidebar-btn.active {
		background-color: var(--accent);
		color: white;
	}

	.sidebar-btn svg {
		flex-shrink: 0;
	}

	.settings-content {
		flex: 1;
		padding: 24px;
		overflow-y: auto;
	}

	.settings-content::-webkit-scrollbar {
		width: 8px;
	}

	.settings-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.settings-content::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 4px;
	}
</style>

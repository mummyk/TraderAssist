<script lang="ts">
	import ToolbarButton from "./ToolbarButton.svelte";
	import PlaybackControls from "./PlaybackControls.svelte";
	import { modalStore } from "../../../stores/modalStore";
	import { symbolsStore, type SymbolData } from "../../../stores/symbolsStore";
	import { chartStore } from "../../../stores/chartStore";
	import IndicatorModal from "$lib/components/modals/IndicatorModal.svelte";
	import ShapeModal from "$lib/components/modals/ShapeModal.svelte";
	import NewChartModal from "$lib/components/modals/NewChartModal.svelte";
	import SettingsModal from "$lib/components/modals/SettingsModal.svelte";
	import { onMount, onDestroy } from "svelte";
	import { keyboardHandler } from "../../../utils/keyboardHandler";

	interface ToolbarProps {
		chartType?: string;
		isPlaying?: boolean;
		isPaused?: boolean;
		playbackSpeed?: number;
	}

	let {
		chartType = $bindable("candlesticks"),
		isPlaying = $bindable(false),
		isPaused = $bindable(false),
		playbackSpeed = $bindable(1),
	}: ToolbarProps = $props();

	let availableSymbols = $state<SymbolData[]>([]);
	let selectedSymbol = $state<string | null>(null);
	let selectedTimeframe = $state<string | null>(null);
	let availableTimeframes = $state<string[]>([]);

	onMount(() => {
		symbolsStore.loadSymbols();

		// Register keyboard shortcuts
		keyboardHandler.registerAction("openNewChart", openNewChartModal);
		keyboardHandler.registerAction("openSettings", openSettingsModal);
		keyboardHandler.registerAction("openIndicator", openIndicatorModal);
		keyboardHandler.registerAction("openShape", openShapesModal);
		keyboardHandler.registerAction("saveChart", handleSave);
		keyboardHandler.registerAction("openChart", handleOpen);
	});

	onDestroy(() => {
		// Clean up keyboard shortcuts
		keyboardHandler.unregisterAction("openNewChart");
		keyboardHandler.unregisterAction("openSettings");
		keyboardHandler.unregisterAction("openIndicator");
		keyboardHandler.unregisterAction("openShape");
		keyboardHandler.unregisterAction("saveChart");
		keyboardHandler.unregisterAction("openChart");
	});

	// Subscribe to symbols store
	symbolsStore.subscribe((symbols) => {
		availableSymbols = symbols;
	});

	// Subscribe to chart store
	chartStore.subscribe((state) => {
		selectedSymbol = state.selectedSymbol;
		selectedTimeframe = state.selectedTimeframe;
		availableTimeframes = state.availableTimeframes;
	});

	function openNewChartModal() {
		modalStore.open("New Chart", NewChartModal, {});
	}

	function openSettingsModal() {
		modalStore.open("Settings", SettingsModal, {});
	}

	function openIndicatorModal() {
		modalStore.open("Add Indicator", IndicatorModal, {
			onSelect: (indicator: string) => {
				console.log("Indicator selected:", indicator);
				modalStore.close();
			},
		});
	}

	function openShapesModal() {
		modalStore.open("Add Shape", ShapeModal, {
			onSelect: (shape: string) => {
				console.log("Shape selected:", shape);
				modalStore.close();
			},
		});
	}

	function handleOpen() {
		console.log("Open chart");
		// TODO: Implement open functionality
	}

	function handleSave() {
		console.log("Save chart");
		// TODO: Implement save functionality
	}

	function handleSymbolChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		const symbol = target.value;

		const symbolData = availableSymbols.find((s) => s.symbol === symbol);
		if (symbolData) {
			const timeframes = symbolData.timeframes.map((tf) => tf.name);
			chartStore.setSymbol(symbol, timeframes);
		}
	}

	function handleTimeframeChange(event: Event) {
		const target = event.target as HTMLSelectElement;
		chartStore.setTimeframe(target.value);
	}
</script>

<div class="toolbar">
	<div class="toolbar-section">
		<ToolbarButton title="New Chart (Ctrl/Cmd + N)" onclick={openNewChartModal}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
				<polyline points="14 2 14 8 20 8" />
				<line x1="12" y1="18" x2="12" y2="12" />
				<line x1="9" y1="15" x2="15" y2="15" />
			</svg>
		</ToolbarButton>

		<ToolbarButton title="Open (Ctrl/Cmd + O)" onclick={handleOpen}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path
					d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"
				/>
			</svg>
		</ToolbarButton>

		<ToolbarButton title="Save (Ctrl/Cmd + S)" onclick={handleSave}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path
					d="M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z"
				/>
				<polyline points="17 21 17 13 7 13 7 21" />
				<polyline points="7 3 7 8 15 8" />
			</svg>
		</ToolbarButton>

		<div class="toolbar-separator"></div>

		<ToolbarButton
			title="Bar Chart"
			active={chartType === "bar"}
			onclick={() => (chartType = "bar")}
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
				<line x1="12" y1="20" x2="12" y2="10" />
				<line x1="18" y1="20" x2="18" y2="4" />
				<line x1="6" y1="20" x2="6" y2="16" />
			</svg>
		</ToolbarButton>

		<ToolbarButton
			title="Candlesticks"
			active={chartType === "candlesticks"}
			onclick={() => (chartType = "candlesticks")}
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
				<line x1="9" y1="2" x2="9" y2="22" />
				<line x1="15" y1="2" x2="15" y2="22" />
				<rect x="7" y="6" width="4" height="8" />
				<rect x="13" y="10" width="4" height="6" />
			</svg>
		</ToolbarButton>

		<ToolbarButton
			title="Line Chart"
			active={chartType === "line"}
			onclick={() => (chartType = "line")}
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
				<polyline points="22 12 18 12 15 21 9 3 6 12 2 12" />
			</svg>
		</ToolbarButton>
	</div>

	<div class="toolbar-separator"></div>

	<PlaybackControls bind:isPlaying bind:isPaused bind:playbackSpeed />

	<div class="toolbar-separator"></div>

	<div class="toolbar-section">
		<button
			class="toolbar-btn-with-label"
			title="Add Indicator (Ctrl/Cmd + I)"
			onclick={openIndicatorModal}
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
				<line x1="12" y1="5" x2="12" y2="19" />
				<line x1="5" y1="12" x2="19" y2="12" />
				<circle cx="12" cy="12" r="10" />
			</svg>
			<span>Indicator</span>
		</button>

		<button
			class="toolbar-btn-with-label"
			title="Add Shape (Ctrl/Cmd + Shift + S)"
			onclick={openShapesModal}
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
				<path
					d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
				/>
			</svg>
			<span>Shape</span>
		</button>
	</div>

	<div class="toolbar-separator"></div>

	<div class="toolbar-section">
		{#if availableSymbols.length > 0}
			<select
				class="toolbar-select"
				value={selectedSymbol || ""}
				onchange={handleSymbolChange}
				disabled={availableSymbols.length === 0}
			>
				{#if !selectedSymbol}
					<option value="" disabled>Select Symbol</option>
				{/if}
				{#each availableSymbols as symbol}
					<option value={symbol.symbol}>{symbol.symbol}</option>
				{/each}
			</select>
		{:else}
			<select class="toolbar-select" disabled>
				<option>No Symbols</option>
			</select>
		{/if}

		{#if availableTimeframes.length > 0}
			<select
				class="toolbar-select"
				value={selectedTimeframe || ""}
				onchange={handleTimeframeChange}
				disabled={!selectedSymbol}
			>
				{#if !selectedTimeframe}
					<option value="" disabled>Select TF</option>
				{/if}
				{#each availableTimeframes as tf}
					<option value={tf}>{tf}</option>
				{/each}
			</select>
		{:else}
			<select class="toolbar-select" disabled>
				<option>No TF</option>
			</select>
		{/if}
	</div>

	<div class="toolbar-section">
		<ToolbarButton title="Zoom In (Ctrl/Cmd + =)">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="11" cy="11" r="8" />
				<path d="m21 21-4.35-4.35" />
				<line x1="11" y1="8" x2="11" y2="14" />
				<line x1="8" y1="11" x2="14" y2="11" />
			</svg>
		</ToolbarButton>

		<ToolbarButton title="Zoom Out (Ctrl/Cmd + -)">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<circle cx="11" cy="11" r="8" />
				<path d="m21 21-4.35-4.35" />
				<line x1="8" y1="11" x2="14" y2="11" />
			</svg>
		</ToolbarButton>
	</div>

	<div class="toolbar-separator"></div>

	<div class="toolbar-section">
		<ToolbarButton title="Settings (Ctrl/Cmd + ,)" onclick={openSettingsModal}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path
					d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
				/>
				<circle cx="12" cy="12" r="3" />
			</svg>
		</ToolbarButton>
	</div>
</div>

<style>
	.toolbar {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 4px 8px;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
		height: 36px;
		flex-shrink: 0;
	}

	.toolbar-section {
		display: flex;
		align-items: center;
		gap: 2px;
	}

	.toolbar-separator {
		width: 1px;
		height: 20px;
		background-color: var(--border-color);
		margin: 0 4px;
	}

	.toolbar-select {
		height: 26px;
		padding: 0 8px;
		background-color: var(--surface-color);
		border: 1px solid var(--border-color);
		border-radius: 3px;
		color: var(--text-primary);
		font-size: 12px;
		cursor: pointer;
		min-width: 80px;
	}

	.toolbar-select:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.toolbar-btn-with-label {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 4px 12px 4px 8px;
		height: 28px;
		background: transparent;
		border: 1px solid var(--border-color);
		border-radius: 4px;
		color: var(--text-primary);
		cursor: pointer;
		transition: all 0.15s;
		font-size: 12px;
		font-weight: 500;
		white-space: nowrap;
	}

	.toolbar-btn-with-label:hover {
		background-color: var(--border-color);
		border-color: var(--accent);
	}

	.toolbar-btn-with-label svg {
		flex-shrink: 0;
	}

	.toolbar-btn-with-label span {
		line-height: 1;
	}
</style>

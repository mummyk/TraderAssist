<script lang="ts">
	import { onMount } from "svelte";
	import { invoke } from "@tauri-apps/api/core";
	import { symbolsStore, type SymbolData } from "../../../stores/symbolsStore";
	import { chartStore } from "../../../stores/chartStore";
	import { ask } from "@tauri-apps/plugin-dialog";

	interface MarketItem {
		symbol: string;
		bid: string;
		ask: string;
		timeframes: string[];
	}

	let activeTab = $state<"market" | "navigator">("market");
	let symbols = $state<SymbolData[]>([]);
	let marketData = $state<MarketItem[]>([]);
	let contextMenu = $state<{
		visible: boolean;
		x: number;
		y: number;
		symbol: string;
	}>({ visible: false, x: 0, y: 0, symbol: "" });
	let renameModal = $state<{
		visible: boolean;
		symbol: string;
		newName: string;
	}>({ visible: false, symbol: "", newName: "" });

	onMount(() => {
		loadSymbols();
		document.addEventListener("click", closeContextMenu);
		return () => {
			document.removeEventListener("click", closeContextMenu);
		};
	});

	symbolsStore.subscribe((data) => {
		symbols = data;
		updateMarketData(data);

		// Auto-select first symbol if none is selected
		if (data.length > 0 && !$chartStore.selectedSymbol) {
			const firstSymbol = data[0];
			selectSymbol(
				firstSymbol.symbol,
				firstSymbol.timeframes.map((tf) => tf.name)
			);
		}
	});

	// Subscribe to chart store to highlight selected symbol
	let selectedSymbol = $state<string | null>(null);
	chartStore.subscribe((state) => {
		selectedSymbol = state.selectedSymbol;
	});

	async function loadSymbols() {
		await symbolsStore.loadSymbols();
	}

	function updateMarketData(symbolData: SymbolData[]) {
		marketData = symbolData.map((symbol) => ({
			symbol: symbol.symbol,
			bid: generateMockPrice(),
			ask: generateMockPrice(true),
			timeframes: symbol.timeframes.map((tf) => tf.name),
		}));
	}

	function generateMockPrice(isAsk: boolean = false): string {
		const base = (1 + Math.random()).toFixed(4);
		return isAsk ? (parseFloat(base) + 0.0002).toFixed(4) : base;
	}

	function selectSymbol(symbol: string, timeframes?: string[]) {
		console.log("Selected symbol:", symbol);

		// Find the symbol data
		const symbolData = symbols.find((s) => s.symbol === symbol);
		if (symbolData) {
			const tfNames = symbolData.timeframes.map((tf) => tf.name);
			chartStore.setSymbol(symbol, tfNames);
		} else if (timeframes) {
			chartStore.setSymbol(symbol, timeframes);
		}
	}

	function handleKeyDown(event: KeyboardEvent, symbol: string) {
		if (event.key === "Enter" || event.key === " ") {
			event.preventDefault();
			selectSymbol(symbol);
		}
	}

	function handleContextMenu(event: MouseEvent, symbol: string) {
		event.preventDefault();
		contextMenu = {
			visible: true,
			x: event.clientX,
			y: event.clientY,
			symbol,
		};
	}

	function closeContextMenu() {
		contextMenu = { ...contextMenu, visible: false };
	}

	function openRenameModal() {
		renameModal = {
			visible: true,
			symbol: contextMenu.symbol,
			newName: contextMenu.symbol,
		};
		closeContextMenu();
	}

	async function handleDelete() {
		const symbolToDelete = contextMenu.symbol;
		// closeContextMenu();

		// Create a Yes/No dialog
		const answer = await ask(
			`Are you sure you want to delete "${symbolToDelete}"?`,
			{
				title: "Delete Symbol",
				kind: "warning",
			}
		);

		console.log(answer);

		if (!answer) {
			return;
		}

		try {
			await invoke("delete_symbol", { symbol: symbolToDelete });
			await symbolsStore.refresh();

			// If deleted symbol was selected, reset selection
			if (selectedSymbol === symbolToDelete) {
				chartStore.reset();
			}
		} catch (error) {
			alert(`Failed to delete symbol: ${error}`);
		}
	}

	async function handleRename() {
		if (!renameModal.newName || renameModal.newName === renameModal.symbol) {
			renameModal = { visible: false, symbol: "", newName: "" };
			return;
		}

		try {
			await invoke("rename_symbol", {
				oldSymbol: renameModal.symbol,
				newSymbol: renameModal.newName,
			});

			await symbolsStore.refresh();

			// Update selection if renamed symbol was selected
			if (selectedSymbol === renameModal.symbol) {
				const newSymbolData = symbols.find(
					(s) => s.symbol === renameModal.newName.toUpperCase()
				);
				if (newSymbolData) {
					selectSymbol(
						newSymbolData.symbol,
						newSymbolData.timeframes.map((tf) => tf.name)
					);
				}
			}

			renameModal = { visible: false, symbol: "", newName: "" };
		} catch (error) {
			alert(`Failed to rename symbol: ${error}`);
		}
	}

	function cancelRename() {
		renameModal = { visible: false, symbol: "", newName: "" };
	}
</script>

<aside class="market-watch">
	<div class="panel-tabs">
		<button
			class="tab-btn"
			class:active={activeTab === "market"}
			onclick={() => (activeTab = "market")}
		>
			Market Watch
		</button>
		<button
			class="tab-btn"
			class:active={activeTab === "navigator"}
			onclick={() => (activeTab = "navigator")}
		>
			Navigator
		</button>
	</div>

	<div class="panel-content">
		{#if activeTab === "market"}
			{#if marketData.length === 0}
				<div class="empty-state">
					<p>No symbols uploaded yet</p>
					<p class="hint">Upload a symbol folder to get started</p>
				</div>
			{:else}
				{#each marketData as item}
					<div
						class="market-item"
						class:selected={item.symbol === selectedSymbol}
						role="button"
						tabindex="0"
						onclick={() => selectSymbol(item.symbol, item.timeframes)}
						onkeydown={(e) => handleKeyDown(e, item.symbol)}
						oncontextmenu={(e) => handleContextMenu(e, item.symbol)}
					>
						<div class="symbol">{item.symbol}</div>
						<div class="price bid">{item.bid}</div>
						<div class="price ask">{item.ask}</div>
						<div class="timeframes-count">{item.timeframes.length} TFs</div>
					</div>
				{/each}
			{/if}
		{:else}
			<div class="navigator-content">
				<div class="nav-section">
					<div class="nav-header">Uploaded Symbols ({symbols.length})</div>
					{#each symbols as symbol}
						<div
							class="nav-item"
							class:selected={symbol.symbol === selectedSymbol}
							role="button"
							tabindex="0"
							onclick={() => selectSymbol(symbol.symbol)}
							onkeydown={(e) => handleKeyDown(e, symbol.symbol)}
							oncontextmenu={(e) => handleContextMenu(e, symbol.symbol)}
						>
							{symbol.symbol}
							<span class="badge">{symbol.timeframes.length}</span>
						</div>
					{/each}
				</div>
			</div>
		{/if}
	</div>
</aside>

{#if contextMenu.visible}
	<div
		class="context-menu"
		style="top: {contextMenu.y}px; left: {contextMenu.x}px;"
	>
		<button class="context-menu-item" onclick={openRenameModal}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M17 3a2.85 2.83 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5Z" />
			</svg>
			Rename
		</button>
		<button class="context-menu-item delete" onclick={handleDelete}>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="14"
				height="14"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M3 6h18" />
				<path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" />
				<path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" />
			</svg>
			Delete
		</button>
	</div>
{/if}

{#if renameModal.visible}
	<div class="modal-overlay">
		<div class="modal-content">
			<h3>Rename Symbol</h3>
			<p class="rename-hint">Renaming "{renameModal.symbol}"</p>
			<input
				type="text"
				class="rename-input"
				bind:value={renameModal.newName}
				placeholder="Enter new symbol name"
				onkeydown={(e) => {
					if (e.key === "Enter") handleRename();
					if (e.key === "Escape") cancelRename();
				}}
			/>
			<div class="modal-actions">
				<button class="btn-secondary" onclick={cancelRename}>Cancel</button>
				<button class="btn-primary" onclick={handleRename}>Rename</button>
			</div>
		</div>
	</div>
{/if}

<style>
	.market-watch {
		width: 220px;
		background-color: var(--bg-secondary);
		border-right: 1px solid var(--border-color);
		display: flex;
		flex-direction: column;
		flex-shrink: 0;
	}

	.panel-tabs {
		display: flex;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
	}

	.tab-btn {
		flex: 1;
		padding: 8px 12px;
		background: transparent;
		border: none;
		color: var(--text-secondary);
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.tab-btn.active {
		color: var(--text-primary);
		background-color: var(--surface-color);
		border-bottom: 2px solid var(--accent);
	}

	.panel-content {
		flex: 1;
		overflow-y: auto;
		padding: 4px;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		padding: 24px;
		text-align: center;
	}

	.empty-state p {
		margin: 0;
		font-size: 12px;
		color: var(--text-secondary);
	}

	.empty-state .hint {
		margin-top: 8px;
		font-size: 11px;
		color: var(--text-tertiary);
	}

	.market-item {
		display: grid;
		grid-template-columns: 1fr auto auto;
		gap: 8px;
		padding: 6px 8px;
		font-size: 11px;
		border-bottom: 1px solid var(--border-color);
		cursor: pointer;
		transition: background-color 0.15s;
		outline: none;
	}

	.market-item:hover,
	.market-item:focus {
		background-color: var(--border-color);
	}

	.market-item.selected {
		background-color: var(--accent);
		color: white;
	}

	.market-item.selected .symbol,
	.market-item.selected .price {
		color: white;
	}

	.symbol {
		font-weight: 600;
		color: var(--text-primary);
	}

	.price {
		font-family: "Consolas", "Monaco", monospace;
		font-size: 11px;
	}

	.price.bid {
		color: #ef4444;
	}

	.price.ask {
		color: #22c55e;
	}

	.timeframes-count {
		grid-column: 1 / -1;
		font-size: 10px;
		color: var(--text-tertiary);
	}

	.navigator-content {
		padding: 4px;
	}

	.nav-section {
		margin-bottom: 12px;
	}

	.nav-header {
		font-size: 11px;
		font-weight: 600;
		color: var(--text-primary);
		padding: 6px 8px;
		background-color: var(--border-color);
		border-radius: 3px;
		margin-bottom: 4px;
	}

	.nav-item {
		font-size: 11px;
		color: var(--text-secondary);
		padding: 6px 8px 6px 20px;
		cursor: pointer;
		border-radius: 3px;
		transition: background-color 0.15s;
		display: flex;
		justify-content: space-between;
		align-items: center;
		outline: none;
	}

	.nav-item:hover,
	.nav-item:focus {
		background-color: var(--border-color);
		color: var(--text-primary);
	}

	.nav-item.selected {
		background-color: var(--accent);
		color: white;
	}

	.badge {
		background-color: var(--accent);
		color: white;
		padding: 2px 6px;
		border-radius: 10px;
		font-size: 10px;
		font-weight: 600;
	}

	.nav-item.selected .badge {
		background-color: white;
		color: var(--accent);
	}

	.context-menu {
		position: fixed;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
		padding: 4px;
		z-index: 1000;
		min-width: 140px;
	}

	.context-menu-item {
		display: flex;
		align-items: center;
		gap: 8px;
		width: 100%;
		padding: 8px 12px;
		background: transparent;
		border: none;
		color: var(--text-primary);
		font-size: 12px;
		cursor: pointer;
		border-radius: 4px;
		transition: background-color 0.15s;
		text-align: left;
	}

	.context-menu-item:hover {
		background-color: var(--border-color);
	}

	.context-menu-item.delete {
		color: #ef4444;
	}

	.context-menu-item.delete:hover {
		background-color: rgba(239, 68, 68, 0.1);
	}

	.modal-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(0, 0, 0, 0.5);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 2000;
	}

	.modal-content {
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 8px;
		padding: 24px;
		min-width: 320px;
		max-width: 400px;
	}

	.modal-content h3 {
		margin: 0 0 8px 0;
		font-size: 16px;
		color: var(--text-primary);
	}

	.rename-hint {
		font-size: 12px;
		color: var(--text-secondary);
		margin: 0 0 16px 0;
	}

	.rename-input {
		width: 100%;
		padding: 8px 12px;
		background-color: var(--surface-color);
		border: 1px solid var(--border-color);
		border-radius: 4px;
		color: var(--text-primary);
		font-size: 13px;
		margin-bottom: 16px;
	}

	.rename-input:focus {
		outline: none;
		border-color: var(--accent);
	}

	.modal-actions {
		display: flex;
		gap: 8px;
		justify-content: flex-end;
	}

	.btn-secondary,
	.btn-primary {
		padding: 8px 16px;
		border: none;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.btn-secondary {
		background-color: var(--border-color);
		color: var(--text-primary);
	}

	.btn-secondary:hover {
		background-color: var(--surface-color);
	}

	.btn-primary {
		background-color: var(--accent);
		color: white;
	}

	.btn-primary:hover {
		opacity: 0.9;
	}

	.panel-content::-webkit-scrollbar {
		width: 8px;
	}

	.panel-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.panel-content::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 4px;
	}
</style>

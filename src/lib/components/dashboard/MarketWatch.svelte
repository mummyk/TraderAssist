<script lang="ts">
	import { onMount } from "svelte";
	import { symbolsStore, type SymbolData } from "../../../stores/symbolsStore";
	import { chartStore } from "../../../stores/chartStore";

	interface MarketItem {
		symbol: string;
		bid: string;
		ask: string;
		timeframes: string[];
	}

	let activeTab = $state<"market" | "navigator">("market");
	let symbols = $state<SymbolData[]>([]);
	let marketData = $state<MarketItem[]>([]);

	onMount(async () => {
		await loadSymbols();
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

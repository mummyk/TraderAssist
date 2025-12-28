<script lang="ts">
	interface MarketItem {
		symbol: string;
		bid: string;
		ask: string;
	}

	let activeTab: "market" | "navigator" = "market";

	const marketData: MarketItem[] = [
		{ symbol: "EURUSD", bid: "1.0845", ask: "1.0847" },
		{ symbol: "GBPUSD", bid: "1.2634", ask: "1.2636" },
		{ symbol: "USDJPY", bid: "149.82", ask: "149.85" },
		{ symbol: "AUDUSD", bid: "0.6521", ask: "0.6523" },
		{ symbol: "USDCAD", bid: "1.3654", ask: "1.3656" },
		{ symbol: "NZDUSD", bid: "0.5987", ask: "0.5989" },
	];

	function selectSymbol(symbol: string) {
		console.log("Selected symbol:", symbol);
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
			{#each marketData as item}
				<div class="market-item" onclick={() => selectSymbol(item.symbol)}>
					<div class="symbol">{item.symbol}</div>
					<div class="price bid">{item.bid}</div>
					<div class="price ask">{item.ask}</div>
				</div>
			{/each}
		{:else}
			<div class="navigator-content">
				<div class="nav-section">
					<div class="nav-header">Accounts</div>
					<div class="nav-item">Demo Account</div>
				</div>
				<div class="nav-section">
					<div class="nav-header">Indicators</div>
					<div class="nav-item">Moving Average</div>
					<div class="nav-item">RSI</div>
					<div class="nav-item">MACD</div>
				</div>
				<div class="nav-section">
					<div class="nav-header">Expert Advisors</div>
					<div class="nav-item">My EA</div>
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

	.market-item {
		display: grid;
		grid-template-columns: 1fr auto auto;
		gap: 8px;
		padding: 6px 8px;
		font-size: 11px;
		border-bottom: 1px solid var(--border-color);
		cursor: pointer;
		transition: background-color 0.15s;
	}

	.market-item:hover {
		background-color: var(--border-color);
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
	}

	.nav-item:hover {
		background-color: var(--border-color);
		color: var(--text-primary);
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

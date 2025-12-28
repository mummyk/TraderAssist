<script lang="ts">
	interface Position {
		order: string;
		time: string;
		symbol: string;
		type: "Buy" | "Sell";
		volume: string;
		price: string;
		sl: string;
		tp: string;
		profit: string;
	}

	let activeTab: "terminal" | "tester" | "alerts" = "terminal";

	const positions: Position[] = [
		// Empty for now - will be populated with actual data
	];
</script>

<div class="terminal">
	<div class="panel-tabs">
		<button
			class="tab-btn"
			class:active={activeTab === "terminal"}
			onclick={() => (activeTab = "terminal")}
		>
			Terminal
		</button>
		<button
			class="tab-btn"
			class:active={activeTab === "tester"}
			onclick={() => (activeTab = "tester")}
		>
			Strategy Tester
		</button>
		<button
			class="tab-btn"
			class:active={activeTab === "alerts"}
			onclick={() => (activeTab = "alerts")}
		>
			Alerts
		</button>
	</div>

	<div class="terminal-content">
		{#if activeTab === "terminal"}
			<table class="terminal-table">
				<thead>
					<tr>
						<th>Order</th>
						<th>Time</th>
						<th>Symbol</th>
						<th>Type</th>
						<th>Volume</th>
						<th>Price</th>
						<th>S/L</th>
						<th>T/P</th>
						<th>Profit</th>
					</tr>
				</thead>
				<tbody>
					{#if positions.length === 0}
						<tr>
							<td colspan="9" class="no-data">No open positions</td>
						</tr>
					{:else}
						{#each positions as position}
							<tr>
								<td>{position.order}</td>
								<td>{position.time}</td>
								<td>{position.symbol}</td>
								<td
									class:buy={position.type === "Buy"}
									class:sell={position.type === "Sell"}
								>
									{position.type}
								</td>
								<td>{position.volume}</td>
								<td>{position.price}</td>
								<td>{position.sl}</td>
								<td>{position.tp}</td>
								<td class="profit">{position.profit}</td>
							</tr>
						{/each}
					{/if}
				</tbody>
			</table>
		{:else if activeTab === "tester"}
			<div class="tab-placeholder">
				<p>Strategy Tester</p>
				<span>Configure and run backtests here</span>
			</div>
		{:else}
			<div class="tab-placeholder">
				<p>Alerts</p>
				<span>No active alerts</span>
			</div>
		{/if}
	</div>
</div>

<style>
	.terminal {
		display: flex;
		flex-direction: column;
		background-color: var(--bg-secondary);
		height: 100%;
		overflow: hidden;
	}

	.panel-tabs {
		display: flex;
		background-color: var(--bg-secondary);
		border-bottom: 1px solid var(--border-color);
		flex-shrink: 0;
	}

	.tab-btn {
		padding: 8px 16px;
		background: transparent;
		border: none;
		color: var(--text-secondary);
		font-size: 11px;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tab-btn.active {
		color: var(--text-primary);
		background-color: var(--surface-color);
		border-bottom: 2px solid var(--accent);
	}

	.terminal-content {
		flex: 1;
		overflow: auto;
	}

	.terminal-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 11px;
	}

	.terminal-table thead {
		background-color: var(--bg-secondary);
		position: sticky;
		top: 0;
		z-index: 1;
	}

	.terminal-table th {
		padding: 8px 12px;
		text-align: left;
		font-weight: 600;
		color: var(--text-primary);
		border-bottom: 1px solid var(--border-color);
	}

	.terminal-table td {
		padding: 6px 12px;
		color: var(--text-secondary);
		border-bottom: 1px solid var(--border-color);
		font-family: "Consolas", "Monaco", monospace;
	}

	.buy {
		color: #22c55e !important;
		font-weight: 600;
	}

	.sell {
		color: #ef4444 !important;
		font-weight: 600;
	}

	.profit {
		font-weight: 600;
	}

	.no-data {
		text-align: center;
		padding: 24px !important;
		color: var(--text-secondary);
		font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto",
			"Helvetica Neue", Arial, sans-serif !important;
	}

	.tab-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: var(--text-secondary);
		gap: 8px;
	}

	.tab-placeholder p {
		font-size: 14px;
		font-weight: 600;
	}

	.tab-placeholder span {
		font-size: 12px;
	}

	.terminal-content::-webkit-scrollbar {
		width: 8px;
		height: 8px;
	}

	.terminal-content::-webkit-scrollbar-track {
		background: transparent;
	}

	.terminal-content::-webkit-scrollbar-thumb {
		background: var(--border-color);
		border-radius: 4px;
	}
</style>

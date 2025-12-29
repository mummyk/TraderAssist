<script lang="ts">
	export let onSelect: ((indicator: string) => void) | undefined = undefined;

	const indicators = [
		{ name: "Moving Average", category: "Trend" },
		{ name: "RSI", category: "Momentum" },
		{ name: "MACD", category: "Momentum" },
		{ name: "Bollinger Bands", category: "Volatility" },
		{ name: "Stochastic", category: "Momentum" },
	];

	function handleSelect(indicator: string) {
		if (onSelect) {
			onSelect(indicator);
		}
		console.log("Selected indicator:", indicator);
	}
</script>

<div class="indicator-modal">
	<div class="search-box">
		<input
			type="text"
			placeholder="Search indicators..."
			class="search-input"
		/>
	</div>

	<div class="indicator-list">
		{#each indicators as indicator}
			<button
				class="indicator-item"
				onclick={() => handleSelect(indicator.name)}
			>
				<div class="indicator-info">
					<span class="indicator-name">{indicator.name}</span>
					<span class="indicator-category">{indicator.category}</span>
				</div>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
				>
					<polyline points="9 18 15 12 9 6" />
				</svg>
			</button>
		{/each}
	</div>
</div>

<style>
	.indicator-modal {
		min-width: 400px;
	}

	.search-box {
		margin-bottom: 16px;
	}

	.search-input {
		width: 100%;
		padding: 8px 12px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 4px;
		color: var(--text-primary);
		font-size: 13px;
		outline: none;
	}

	.search-input:focus {
		border-color: var(--accent);
	}

	.indicator-list {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.indicator-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 12px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.indicator-item:hover {
		background-color: var(--border-color);
		border-color: var(--accent);
	}

	.indicator-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.indicator-name {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-primary);
	}

	.indicator-category {
		font-size: 11px;
		color: var(--text-secondary);
	}

	.indicator-item svg {
		color: var(--text-secondary);
	}

	.indicator-item:hover svg {
		color: var(--accent);
	}
</style>

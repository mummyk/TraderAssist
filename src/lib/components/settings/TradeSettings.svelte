<script lang="ts">
	import { settingsStore } from "../../../stores/settingsStore";
	import type { TradeSettings } from "../../../stores/settingsStore";

	let settings = $state<TradeSettings>({
		bidDistance: 5,
		askDistance: 5,
		spread: 2,
		slippage: 1,
		lotSize: 0.01,
		maxOrders: 10,
		stopLoss: 50,
		takeProfit: 100,
	});

	settingsStore.subscribe((state) => {
		settings = state.trade;
	});

	function handleNumberChange(key: keyof TradeSettings, value: string) {
		const numValue = parseFloat(value);
		if (!isNaN(numValue)) {
			settingsStore.updateTrade(key, numValue);
		}
	}
</script>

<div class="settings-section">
	<h3 class="section-title">Trade Settings</h3>
	<p class="section-description">
		Configure trading parameters and risk management
	</p>

	<div class="settings-grid">
		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="bid-distance">Bid Distance</label>
				<p class="setting-desc">Distance from current price (pips)</p>
			</div>
			<input
				type="number"
				id="bid-distance"
				class="setting-input"
				value={settings.bidDistance}
				min="0"
				step="0.1"
				oninput={(e) =>
					handleNumberChange(
						"bidDistance",
						(e.target as HTMLInputElement).value
					)}
			/>
		</div>

		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="ask-distance">Ask Distance</label>
				<p class="setting-desc">Distance from current price (pips)</p>
			</div>
			<input
				type="number"
				id="ask-distance"
				class="setting-input"
				value={settings.askDistance}
				min="0"
				step="0.1"
				oninput={(e) =>
					handleNumberChange(
						"askDistance",
						(e.target as HTMLInputElement).value
					)}
			/>
		</div>

		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="spread">Spread</label>
				<p class="setting-desc">Maximum acceptable spread (pips)</p>
			</div>
			<input
				type="number"
				id="spread"
				class="setting-input"
				value={settings.spread}
				min="0"
				step="0.1"
				oninput={(e) =>
					handleNumberChange("spread", (e.target as HTMLInputElement).value)}
			/>
		</div>

		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="slippage">Slippage</label>
				<p class="setting-desc">Maximum acceptable slippage (pips)</p>
			</div>
			<input
				type="number"
				id="slippage"
				class="setting-input"
				value={settings.slippage}
				min="0"
				step="0.1"
				oninput={(e) =>
					handleNumberChange("slippage", (e.target as HTMLInputElement).value)}
			/>
		</div>

		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="lot-size">Lot Size</label>
				<p class="setting-desc">Default trading lot size</p>
			</div>
			<input
				type="number"
				id="lot-size"
				class="setting-input"
				value={settings.lotSize}
				min="0.01"
				step="0.01"
				oninput={(e) =>
					handleNumberChange("lotSize", (e.target as HTMLInputElement).value)}
			/>
		</div>

		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="max-orders">Max Orders</label>
				<p class="setting-desc">Maximum concurrent orders</p>
			</div>
			<input
				type="number"
				id="max-orders"
				class="setting-input"
				value={settings.maxOrders}
				min="1"
				step="1"
				oninput={(e) =>
					handleNumberChange("maxOrders", (e.target as HTMLInputElement).value)}
			/>
		</div>

		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="stop-loss">Stop Loss</label>
				<p class="setting-desc">Default stop loss (pips)</p>
			</div>
			<input
				type="number"
				id="stop-loss"
				class="setting-input"
				value={settings.stopLoss}
				min="0"
				step="1"
				oninput={(e) =>
					handleNumberChange("stopLoss", (e.target as HTMLInputElement).value)}
			/>
		</div>

		<div class="setting-item">
			<div class="setting-info">
				<label class="setting-label" for="take-profit">Take Profit</label>
				<p class="setting-desc">Default take profit (pips)</p>
			</div>
			<input
				type="number"
				id="take-profit"
				class="setting-input"
				value={settings.takeProfit}
				min="0"
				step="1"
				oninput={(e) =>
					handleNumberChange(
						"takeProfit",
						(e.target as HTMLInputElement).value
					)}
			/>
		</div>
	</div>
</div>

<style>
	.settings-section {
		max-width: 600px;
	}

	.section-title {
		font-size: 18px;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 8px 0;
	}

	.section-description {
		font-size: 13px;
		color: var(--text-secondary);
		margin: 0 0 24px 0;
	}

	.settings-grid {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.setting-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 0;
		border-bottom: 1px solid var(--border-color);
	}

	.setting-item:last-child {
		border-bottom: none;
	}

	.setting-info {
		flex: 1;
	}

	.setting-label {
		display: block;
		font-size: 14px;
		font-weight: 500;
		color: var(--text-primary);
		margin-bottom: 4px;
	}

	.setting-desc {
		font-size: 12px;
		color: var(--text-secondary);
		margin: 0;
	}

	.setting-input {
		width: 120px;
		padding: 8px 12px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		color: var(--text-primary);
		font-size: 13px;
		text-align: right;
	}

	.setting-input:focus {
		outline: none;
		border-color: var(--accent);
	}

	/* Remove spinner for number inputs */
	.setting-input::-webkit-outer-spin-button,
	.setting-input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}

	.setting-input[type="number"] {
		-moz-appearance: textfield;
	}
</style>

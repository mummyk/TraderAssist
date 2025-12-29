<script lang="ts">
	interface YFinanceData {
		symbol: string;
		startDate: string;
		endDate: string;
		interval: string;
	}

	export let onFetch: ((data: YFinanceData) => void) | undefined = undefined;

	let symbol = $state("");
	let startDate = $state("");
	let endDate = $state("");
	let interval = $state("1d");

	function handleSubmit(event: Event) {
		event.preventDefault();

		if (!symbol || !startDate || !endDate) {
			alert("Please fill in all required fields");
			return;
		}

		const data: YFinanceData = {
			symbol,
			startDate,
			endDate,
			interval,
		};

		if (onFetch) {
			onFetch(data);
		}

		console.log("Fetching yFinance data:", data);
	}
</script>

<form class="yfinance-form" onsubmit={handleSubmit}>
	<div class="form-group">
		<label for="symbol">Symbol *</label>
		<input
			type="text"
			id="symbol"
			bind:value={symbol}
			placeholder="e.g., AAPL, MSFT, TSLA"
			class="input"
			required
		/>
	</div>

	<div class="form-row">
		<div class="form-group">
			<label for="start-date">Start Date *</label>
			<input
				type="date"
				id="start-date"
				bind:value={startDate}
				class="input"
				required
			/>
		</div>
		<div class="form-group">
			<label for="end-date">End Date *</label>
			<input
				type="date"
				id="end-date"
				bind:value={endDate}
				class="input"
				required
			/>
		</div>
	</div>

	<div class="form-group">
		<label for="interval">Interval</label>
		<select id="interval" bind:value={interval} class="input">
			<option value="1d">1 Day</option>
			<option value="1h">1 Hour</option>
			<option value="15m">15 Minutes</option>
			<option value="5m">5 Minutes</option>
			<option value="1m">1 Minute</option>
		</select>
	</div>

	<button type="submit" class="submit-btn">Fetch Data</button>
</form>

<style>
	.yfinance-form {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.form-row {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 16px;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.form-group label {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-primary);
	}

	.input {
		padding: 10px 12px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		color: var(--text-primary);
		font-size: 13px;
		outline: none;
		transition: all 0.15s;
	}

	.input:focus {
		border-color: var(--accent);
		background-color: var(--bg-primary);
	}

	.input::placeholder {
		color: var(--text-secondary);
	}

	.submit-btn {
		padding: 12px 24px;
		background-color: var(--accent);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
		margin-top: 8px;
	}

	.submit-btn:hover {
		background-color: var(--primary-hover);
		transform: translateY(-1px);
	}

	.submit-btn:active {
		transform: translateY(0);
	}
</style>

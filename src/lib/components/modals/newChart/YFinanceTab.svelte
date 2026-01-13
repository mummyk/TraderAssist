<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { modalStore } from "../../../../stores/modalStore";
	import { symbolsStore } from "../../../../stores/symbolsStore";

	interface YFinanceResult {
		success: boolean;
		message: string;
		symbol: string;
		timeframes_downloaded: string[];
		total_candles: number;
	}

	let symbol = $state("");
	let saveAs = $state("");
	let startDate = $state("");
	let endDate = $state("");
	let selectedTimeframes = $state<string[]>(["D1"]);
	let isDownloading = $state(false);
	let downloadStatus = $state("");
	let error = $state<string | null>(null);

	const availableTimeframes = [
		{ value: "M1", label: "1 Minute" },
		{ value: "M2", label: "2 Minutes" },
		{ value: "M5", label: "5 Minutes" },
		{ value: "M15", label: "15 Minutes" },
		{ value: "M30", label: "30 Minutes" },
		{ value: "H1", label: "1 Hour" },
		{ value: "H4", label: "4 Hours" },
		{ value: "D1", label: "1 Day" },
		{ value: "W1", label: "1 Week" },
		{ value: "MN1", label: "1 Month" },
	];

	function toggleTimeframe(tf: string) {
		const index = selectedTimeframes.indexOf(tf);
		if (index === -1) {
			selectedTimeframes = [...selectedTimeframes, tf];
		} else {
			selectedTimeframes = selectedTimeframes.filter((t) => t !== tf);
		}
	}

	async function handleSubmit(event: Event) {
		event.preventDefault();
		error = null;

		if (!symbol.trim()) {
			error = "Please enter a symbol (e.g., AAPL, MSFT)";
			return;
		}

		if (!saveAs.trim()) {
			error = "Please enter a name to save the symbol as";
			return;
		}

		if (!startDate || !endDate) {
			error = "Please select both start and end dates";
			return;
		}

		if (selectedTimeframes.length === 0) {
			error = "Please select at least one timeframe";
			return;
		}

		isDownloading = true;
		downloadStatus = "Connecting to yFinance...";

		try {
			const result = await invoke<YFinanceResult>(
				"fetch_yfinance_data_command",
				{
					request: {
						symbol: symbol.trim().toUpperCase(),
						save_as: saveAs.trim(),
						start_date: startDate,
						end_date: endDate,
						timeframes: selectedTimeframes,
					},
				}
			);

			console.log("Download result:", result);
			downloadStatus = result.message;

			// Refresh symbols list
			await symbolsStore.refresh();

			// Close modal after 2 seconds on success
			setTimeout(() => {
				modalStore.close();
			}, 2000);
		} catch (err) {
			console.error("Error downloading data:", err);
			error = String(err);
			downloadStatus = "";
		} finally {
			isDownloading = false;
		}
	}
</script>

<form class="yfinance-form" onsubmit={handleSubmit}>
	<div class="form-group">
		<label for="symbol">yFinance Symbol *</label>
		<input
			type="text"
			id="symbol"
			bind:value={symbol}
			placeholder="e.g., AAPL, MSFT, TSLA, ^GSPC"
			class="input"
			required
			disabled={isDownloading}
		/>
		<span class="hint">Enter the ticker symbol from Yahoo Finance</span>
	</div>

	<div class="form-group">
		<label for="save-as">Save As *</label>
		<input
			type="text"
			id="save-as"
			bind:value={saveAs}
			placeholder="e.g., APPLE, MICROSOFT"
			class="input"
			required
			disabled={isDownloading}
		/>
		<span class="hint">Choose a name to save this symbol in your database</span>
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
				disabled={isDownloading}
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
				disabled={isDownloading}
			/>
		</div>
	</div>

	<div class="form-group">
		<label>Select Timeframes *</label>
		<div class="timeframes-grid">
			{#each availableTimeframes as tf}
				<button
					type="button"
					class="tf-button"
					class:selected={selectedTimeframes.includes(tf.value)}
					onclick={() => toggleTimeframe(tf.value)}
					disabled={isDownloading}
				>
					{tf.label}
				</button>
			{/each}
		</div>
		<span class="hint">
			{selectedTimeframes.length} timeframe{selectedTimeframes.length !== 1
				? "s"
				: ""} selected
		</span>
	</div>

	{#if error}
		<div class="status-message error">
			❌ {error}
		</div>
	{/if}

	{#if downloadStatus}
		<div class="status-message success">
			✓ {downloadStatus}
		</div>
	{/if}

	<button type="submit" class="submit-btn" disabled={isDownloading}>
		{isDownloading ? "⏳ Downloading..." : "📥 Download Data"}
	</button>
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

	.hint {
		font-size: 11px;
		color: var(--text-secondary);
		font-style: italic;
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

	.input:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.input::placeholder {
		color: var(--text-secondary);
	}

	.timeframes-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(110px, 1fr));
		gap: 8px;
	}

	.tf-button {
		padding: 10px 12px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		color: var(--text-primary);
		font-size: 12px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
		text-align: center;
	}

	.tf-button:hover:not(:disabled) {
		border-color: var(--accent);
		background-color: var(--bg-primary);
	}

	.tf-button.selected {
		background-color: var(--accent);
		color: white;
		border-color: var(--accent);
	}

	.tf-button:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.status-message {
		padding: 12px 16px;
		border-radius: 6px;
		font-size: 13px;
		animation: slideIn 0.3s ease-out;
	}

	.status-message.success {
		border-left: 3px solid #22c55e;
		background-color: rgba(34, 197, 94, 0.1);
		color: var(--text-primary);
	}

	.status-message.error {
		border-left: 3px solid #ef4444;
		background-color: rgba(239, 68, 68, 0.1);
		color: #ef4444;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateY(-10px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
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

	.submit-btn:hover:not(:disabled) {
		background-color: var(--primary-hover);
		transform: translateY(-1px);
	}

	.submit-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
		transform: none;
	}

	.submit-btn:active:not(:disabled) {
		transform: translateY(0);
	}
</style>

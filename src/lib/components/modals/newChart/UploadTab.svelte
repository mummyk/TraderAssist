<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { open } from "@tauri-apps/plugin-dialog";
	import { modalStore } from "../../../../stores/modalStore";
	import { symbolsStore } from "../../../../stores/symbolsStore";

	interface ProcessResult {
		success: boolean;
		message: string;
		symbol: string | null;
		timeframes_processed: string[];
		total_candles: number;
	}

	let isProcessing = $state(false);
	let selectedFolder = $state<string | null>(null);
	let processingStatus = $state<string>("");
	let error = $state<string | null>(null);

	async function handleSelectFolder() {
		error = null;
		try {
			// Explicitly set directory to true and multiple to false
			const selected = await open({
				directory: true,
				multiple: false,
				title: "Select Symbol Folder (e.g., EURUSD containing CSV files)",
				defaultPath: undefined,
			});

			console.log("Selected:", selected); // Debug log

			if (selected) {
				// Handle both string and array responses
				const folderPath =
					typeof selected === "string" ? selected : selected[0];

				if (folderPath) {
					selectedFolder = folderPath;
					await processFolder(folderPath);
				}
			}
		} catch (err) {
			console.error("Error selecting folder:", err);
			error = "Failed to select folder: " + String(err);
		}
	}

	async function processFolder(folderPath: string) {
		isProcessing = true;
		error = null;
		processingStatus = "Processing folder...";

		try {
			const result = await invoke<ProcessResult>("process_chart_folder", {
				folderPath,
			});

			console.log("Processing result:", result);
			processingStatus = result.message;

			// Refresh symbols list
			await symbolsStore.refresh();

			// Close modal after 2 seconds on success
			setTimeout(() => {
				modalStore.close();
			}, 2000);
		} catch (err) {
			console.error("Error processing folder:", err);
			error = String(err);
			processingStatus = "";
		} finally {
			isProcessing = false;
		}
	}
</script>

<div class="upload-tab">
	<div class="upload-area" role="button" tabindex="0">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="48"
			height="48"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="1.5"
		>
			<path
				d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
			/>
		</svg>
		<h4>Upload Symbol Data</h4>
		<p>Select a folder containing timeframe CSV files</p>

		<button
			class="upload-btn"
			onclick={handleSelectFolder}
			disabled={isProcessing}
		>
			{isProcessing ? "Processing..." : "📁 Select Folder"}
		</button>

		{#if selectedFolder}
			<div class="selected-folder">
				<span class="folder-icon">📁</span>
				<span class="folder-path">{selectedFolder}</span>
			</div>
		{/if}

		{#if error}
			<div class="status-message error">
				❌ {error}
			</div>
		{/if}

		{#if processingStatus}
			<div class="status-message success">
				✓ {processingStatus}
			</div>
		{/if}
	</div>

	<div class="folder-structure-info">
		<h5>Required Folder Structure:</h5>
		<div class="structure-example">
			<code>
				📁 EURUSD/ <span class="comment">← Select this folder</span><br />
				&nbsp;&nbsp;├── M1.csv<br />
				&nbsp;&nbsp;├── M5.csv<br />
				&nbsp;&nbsp;├── M15.csv<br />
				&nbsp;&nbsp;├── M30.csv<br />
				&nbsp;&nbsp;├── H1.csv<br />
				&nbsp;&nbsp;├── H4.csv<br />
				&nbsp;&nbsp;├── D1.csv<br />
				&nbsp;&nbsp;├── W1.csv<br />
				&nbsp;&nbsp;└── MN1.csv
			</code>
		</div>
		<div class="info-notes">
			<p class="note">
				<strong>Note:</strong> Select the parent folder (e.g., EURUSD) that contains
				the CSV files
			</p>
			<p class="csv-format">
				<strong>CSV Format:</strong> timestamp,open,high,low,close,volume
			</p>
			<p class="note">
				<strong>Timeframes:</strong> M = minutes, H = hours, D = daily, W = weekly,
				MN = monthly
			</p>
		</div>
	</div>
</div>

<style>
	.upload-tab {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.upload-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 48px 24px;
		border: 2px dashed var(--border-color);
		border-radius: 8px;
		text-align: center;
		transition: all 0.2s;
	}

	.upload-area svg {
		color: var(--text-secondary);
		margin-bottom: 16px;
	}

	.upload-area h4 {
		font-size: 15px;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 8px 0;
	}

	.upload-area p {
		font-size: 13px;
		color: var(--text-secondary);
		margin: 0 0 20px 0;
	}

	.upload-btn {
		padding: 10px 24px;
		background-color: var(--accent);
		color: white;
		border: none;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.upload-btn:hover:not(:disabled) {
		background-color: var(--primary-hover);
		transform: translateY(-1px);
	}

	.upload-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.selected-folder {
		display: flex;
		align-items: center;
		gap: 8px;
		margin-top: 16px;
		padding: 8px 16px;
		background-color: var(--bg-secondary);
		border-radius: 6px;
		font-size: 12px;
		color: var(--text-primary);
		max-width: 100%;
		overflow: hidden;
	}

	.folder-icon {
		font-size: 16px;
		flex-shrink: 0;
	}

	.folder-path {
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.status-message {
		margin-top: 12px;
		padding: 8px 16px;
		border-radius: 4px;
		font-size: 12px;
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

	.folder-structure-info {
		background-color: var(--bg-secondary);
		padding: 16px;
		border-radius: 8px;
		border: 1px solid var(--border-color);
	}

	.folder-structure-info h5 {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 12px 0;
	}

	.structure-example {
		background-color: var(--bg-primary);
		padding: 12px;
		border-radius: 6px;
		border: 1px solid var(--border-color);
		margin-bottom: 12px;
	}

	.structure-example code {
		font-family: "Consolas", "Monaco", monospace;
		font-size: 12px;
		color: var(--text-primary);
		line-height: 1.8;
		display: block;
	}

	.comment {
		color: var(--accent);
		font-style: italic;
	}

	.info-notes {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.note,
	.csv-format {
		font-size: 11px;
		color: var(--text-secondary);
		margin: 0;
		padding: 6px 8px;
		background-color: var(--bg-primary);
		border-radius: 4px;
		border-left: 2px solid var(--accent);
	}

	.note strong,
	.csv-format strong {
		color: var(--text-primary);
	}
</style>

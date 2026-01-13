<script lang="ts">
	import { invoke } from "@tauri-apps/api/core";
	import { modalStore } from "../../../../stores/modalStore";
	import { symbolsStore } from "../../../../stores/symbolsStore";

	interface GitHubResult {
		success: boolean;
		message: string;
		symbols_processed: string[];
		total_timeframes: number;
	}

	let repoUrl = $state("");
	let branch = $state("main");
	let structureType = $state<"single" | "multi">("single");
	let symbolName = $state("");
	let downloadAllSymbols = $state(false);
	let isDownloading = $state(false);
	let downloadStatus = $state("");
	let error = $state<string | null>(null);

	function handleSubmit(event: Event) {
		event.preventDefault();
		error = null;

		if (!repoUrl.trim()) {
			error = "Please enter a GitHub repository URL";
			return;
		}

		// Validate structure-specific fields
		if (structureType === "single" && !symbolName.trim()) {
			error = "Please enter a symbol name for single-symbol repository";
			return;
		}

		if (
			structureType === "multi" &&
			!downloadAllSymbols &&
			!symbolName.trim()
		) {
			error = "Please enter a symbol name or select 'Download all symbols'";
			return;
		}

		downloadData();
	}

	async function downloadData() {
		isDownloading = true;
		downloadStatus = "Connecting to GitHub...";

		try {
			const result = await invoke<GitHubResult>("fetch_github_data_command", {
				request: {
					repo_url: repoUrl.trim(),
					branch: branch.trim() || "main",
					structure_type: structureType,
					symbol_name:
						structureType === "single" || !downloadAllSymbols
							? symbolName.trim() || null
							: null,
				},
			});

			console.log("Download result:", result);
			downloadStatus = result.message;

			// Refresh symbols list
			await symbolsStore.refresh();

			// Close modal after 2 seconds on success
			setTimeout(() => {
				modalStore.close();
			}, 2000);
		} catch (err) {
			console.error("Error downloading from GitHub:", err);
			error = String(err);
			downloadStatus = "";
		} finally {
			isDownloading = false;
		}
	}
</script>

<form class="github-form" onsubmit={handleSubmit}>
	<div class="info-box">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="20"
			height="20"
			viewBox="0 0 24 24"
			fill="currentColor"
		>
			<path
				d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
			/>
		</svg>
		<div>
			<strong>GitHub Repository Structure</strong>
			<p>Choose based on how your data is organized in the repository</p>
		</div>
	</div>

	<div class="form-group">
		<label>Repository Structure *</label>
		<div class="structure-options">
			<button
				type="button"
				class="structure-btn"
				class:selected={structureType === "single"}
				onclick={() => (structureType = "single")}
				disabled={isDownloading}
			>
				<div class="structure-icon">📄</div>
				<div class="structure-info">
					<strong>Single Symbol</strong>
					<span>Files in root (M1.csv, M5.csv, etc.)</span>
				</div>
			</button>
			<button
				type="button"
				class="structure-btn"
				class:selected={structureType === "multi"}
				onclick={() => (structureType = "multi")}
				disabled={isDownloading}
			>
				<div class="structure-icon">📁</div>
				<div class="structure-info">
					<strong>Multiple Symbols</strong>
					<span>Folders for each symbol (EURUSD/, GBPUSD/)</span>
				</div>
			</button>
		</div>
	</div>

	<div class="form-group">
		<label for="repo-url">Repository URL *</label>
		<input
			type="text"
			id="repo-url"
			bind:value={repoUrl}
			placeholder="https://github.com/username/repo"
			class="input"
			required
			disabled={isDownloading}
		/>
		<span class="hint">Example: https://github.com/username/trading-data</span>
	</div>

	<div class="form-row">
		<div class="form-group">
			<label for="branch">Branch</label>
			<input
				type="text"
				id="branch"
				bind:value={branch}
				placeholder="main"
				class="input"
				disabled={isDownloading}
			/>
		</div>
		<div class="form-group">
			<label for="symbol-name">
				{structureType === "single" ? "Save As *" : "Symbol Name"}
			</label>
			<input
				type="text"
				id="symbol-name"
				bind:value={symbolName}
				placeholder={structureType === "single"
					? "e.g., EURUSD"
					: "e.g., EURUSD (optional)"}
				class="input"
				required={structureType === "single"}
				disabled={isDownloading ||
					(structureType === "multi" && downloadAllSymbols)}
			/>
		</div>
	</div>

	{#if structureType === "multi"}
		<div class="form-group">
			<label class="checkbox-label">
				<input
					type="checkbox"
					bind:checked={downloadAllSymbols}
					disabled={isDownloading}
				/>
				<span>Download all symbols from repository</span>
			</label>
			<span class="hint">
				{downloadAllSymbols
					? "All symbol folders will be downloaded"
					: "Only the specified symbol will be downloaded"}
			</span>
		</div>
	{/if}

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
		{isDownloading ? "⏳ Downloading..." : "📥 Download from GitHub"}
	</button>

	<div class="examples">
		<h5>Repository Structure Examples:</h5>
		<div class="example-box">
			<strong>Single Symbol:</strong>
			<code>
				repo/<br />
				├── M1.csv<br />
				├── M5.csv<br />
				├── H1.csv<br />
				└── D1.csv
			</code>
		</div>
		<div class="example-box">
			<strong>Multiple Symbols:</strong>
			<code>
				repo/<br />
				├── EURUSD/<br />
				│ ├── M1.csv<br />
				│ └── D1.csv<br />
				└── GBPUSD/<br />
				&nbsp;&nbsp;&nbsp;&nbsp;├── M1.csv<br />
				&nbsp;&nbsp;&nbsp;&nbsp;└── D1.csv
			</code>
		</div>
	</div>
</form>

<style>
	.github-form {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.info-box {
		display: flex;
		gap: 12px;
		padding: 12px 16px;
		background-color: rgba(59, 130, 246, 0.1);
		border-left: 3px solid #3b82f6;
		border-radius: 6px;
		font-size: 12px;
	}

	.info-box svg {
		flex-shrink: 0;
		margin-top: 2px;
		color: #3b82f6;
	}

	.info-box strong {
		display: block;
		color: var(--text-primary);
		margin-bottom: 4px;
	}

	.info-box p {
		color: var(--text-secondary);
		margin: 0;
	}

	.structure-options {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 12px;
	}

	.structure-btn {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 16px;
		background-color: var(--bg-secondary);
		border: 2px solid var(--border-color);
		border-radius: 8px;
		cursor: pointer;
		transition: all 0.15s;
		text-align: left;
	}

	.structure-btn:hover:not(:disabled) {
		border-color: var(--accent);
		background-color: var(--bg-primary);
	}

	.structure-btn.selected {
		border-color: var(--accent);
		background-color: rgba(59, 130, 246, 0.1);
	}

	.structure-btn:disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	.structure-icon {
		font-size: 24px;
		flex-shrink: 0;
	}

	.structure-info {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.structure-info strong {
		font-size: 13px;
		color: var(--text-primary);
	}

	.structure-info span {
		font-size: 11px;
		color: var(--text-secondary);
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

	.checkbox-label {
		display: flex;
		align-items: center;
		gap: 8px;
		cursor: pointer;
		font-size: 13px;
		color: var(--text-primary);
	}

	.checkbox-label input[type="checkbox"] {
		width: 16px;
		height: 16px;
		cursor: pointer;
	}

	.checkbox-label input[type="checkbox"]:disabled {
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

	.examples {
		margin-top: 8px;
		padding: 16px;
		background-color: var(--bg-secondary);
		border-radius: 8px;
		border: 1px solid var(--border-color);
	}

	.examples h5 {
		font-size: 13px;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0 0 12px 0;
	}

	.example-box {
		background-color: var(--bg-primary);
		padding: 12px;
		border-radius: 6px;
		margin-bottom: 12px;
		border-left: 2px solid var(--accent);
	}

	.example-box:last-child {
		margin-bottom: 0;
	}

	.example-box strong {
		display: block;
		font-size: 12px;
		color: var(--text-primary);
		margin-bottom: 8px;
	}

	.example-box code {
		font-family: "Consolas", "Monaco", monospace;
		font-size: 11px;
		color: var(--text-secondary);
		line-height: 1.6;
		display: block;
	}
</style>

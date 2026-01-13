<script lang="ts">
	import { modalStore } from "../../../stores/modalStore";
	import UploadTab from "../modals/newChart/UploadTab.svelte";
	import YFinanceTab from "../modals/newChart/YFinanceTab.svelte";
	import GitHubTab from "../modals/newChart/GitHubTab.svelte";

	type TabType = "upload" | "yfinance" | "github";

	let activeTab = $state<TabType>("upload");

	function handleTabChange(tab: TabType) {
		activeTab = tab;
	}
</script>

<div class="new-chart-modal">
	<div class="tabs">
		<button
			class="tab"
			class:active={activeTab === "upload"}
			onclick={() => handleTabChange("upload")}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
				<polyline points="17 8 12 3 7 8" />
				<line x1="12" y1="3" x2="12" y2="15" />
			</svg>
			Upload
		</button>
		<button
			class="tab"
			class:active={activeTab === "yfinance"}
			onclick={() => handleTabChange("yfinance")}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<line x1="12" y1="1" x2="12" y2="23" />
				<path d="M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" />
			</svg>
			yFinance
		</button>
		<button
			class="tab"
			class:active={activeTab === "github"}
			onclick={() => handleTabChange("github")}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
			>
				<path
					d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"
				/>
				<path d="M9 18c-4.51 2-5-2-7-2" />
			</svg>
			GitHub
		</button>
	</div>

	<div class="tab-content">
		{#if activeTab === "upload"}
			<UploadTab />
		{:else if activeTab === "yfinance"}
			<YFinanceTab />
		{:else if activeTab === "github"}
			<div class="tab-panel">
				<form class="github-form">
					<div class="form-group">
						<label for="repo-url">Repository URL</label>
						<input
							type="text"
							id="repo-url"
							placeholder="https://github.com/username/repo"
							class="input"
						/>
					</div>

					<div class="form-group">
						<label for="file-path">File Path</label>
						<input
							type="text"
							id="file-path"
							placeholder="data/prices.csv"
							class="input"
						/>
					</div>

					<div class="form-group">
						<label for="branch">Branch (optional)</label>
						<input type="text" id="branch" placeholder="main" class="input" />
					</div>

					<button type="submit" class="submit-btn">Load from GitHub</button>
				</form>
			</div>
		{/if}
	</div>
</div>

<style>
	.new-chart-modal {
		min-width: 550px;
	}

	.tabs {
		display: flex;
		gap: 4px;
		border-bottom: 1px solid var(--border-color);
		margin-bottom: 20px;
	}

	.tab {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 12px 20px;
		background: transparent;
		border: none;
		border-bottom: 2px solid transparent;
		color: var(--text-secondary);
		font-size: 13px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s;
	}

	.tab:hover {
		color: var(--text-primary);
		background-color: var(--bg-secondary);
	}

	.tab.active {
		color: var(--accent);
		border-bottom-color: var(--accent);
	}

	.tab svg {
		flex-shrink: 0;
	}

	.tab-content {
		min-height: 300px;
	}

	.tab-panel {
		animation: fadeIn 0.2s ease-in;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: translateY(-5px);
		}
		to {
			opacity: 1;
			transform: translateY(0);
		}
	}

	/* Form Styles */
	.yfinance-form,
	.github-form {
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

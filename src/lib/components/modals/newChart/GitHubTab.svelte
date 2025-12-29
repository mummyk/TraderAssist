<script lang="ts">
	interface GitHubData {
		repoUrl: string;
		filePath: string;
		branch: string;
	}

	let { onLoad }: { onLoad?: (data: GitHubData) => void } = $props();

	let repoUrl = $state("");
	let filePath = $state("");
	let branch = $state("main");

	function handleSubmit(event: Event) {
		event.preventDefault();

		if (!repoUrl || !filePath) {
			alert("Please fill in all required fields");
			return;
		}

		const data: GitHubData = {
			repoUrl,
			filePath,
			branch: branch || "main",
		};

		if (onLoad) {
			onLoad(data);
		}

		console.log("Loading from GitHub:", data);
	}
</script>

<form class="github-form" onsubmit={handleSubmit}>
	<div class="form-group">
		<label for="repo-url">Repository URL *</label>
		<input
			type="text"
			id="repo-url"
			bind:value={repoUrl}
			placeholder="https://github.com/username/repo"
			class="input"
			required
		/>
	</div>

	<div class="form-group">
		<label for="file-path">File Path *</label>
		<input
			type="text"
			id="file-path"
			bind:value={filePath}
			placeholder="data/prices.csv"
			class="input"
			required
		/>
	</div>

	<div class="form-group">
		<label for="branch">Branch</label>
		<input
			type="text"
			id="branch"
			bind:value={branch}
			placeholder="main"
			class="input"
		/>
	</div>

	<button type="submit" class="submit-btn">Load from GitHub</button>
</form>

<style>
	.github-form {
		display: flex;
		flex-direction: column;
		gap: 20px;
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

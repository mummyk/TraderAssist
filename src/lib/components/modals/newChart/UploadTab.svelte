<script lang="ts">
	export let onFileSelect: ((file: File) => void) | undefined = undefined;

	function handleFileChange(event: Event) {
		const input = event.target as HTMLInputElement;
		if (input.files && input.files[0]) {
			const file = input.files[0];
			if (onFileSelect) {
				onFileSelect(file);
			}
			console.log("File selected:", file.name);
		}
	}

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		if (event.dataTransfer?.files && event.dataTransfer.files[0]) {
			const file = event.dataTransfer.files[0];
			if (onFileSelect) {
				onFileSelect(file);
			}
			console.log("File dropped:", file.name);
		}
	}

	function handleDragOver(event: DragEvent) {
		event.preventDefault();
	}
</script>

<div
	class="upload-area"
	ondrop={handleDrop}
	ondragover={handleDragOver}
	role="button"
	tabindex="0"
>
	<svg
		xmlns="http://www.w3.org/2000/svg"
		width="48"
		height="48"
		viewBox="0 0 24 24"
		fill="none"
		stroke="currentColor"
		stroke-width="1.5"
	>
		<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
		<polyline points="17 8 12 3 7 8" />
		<line x1="12" y1="3" x2="12" y2="15" />
	</svg>
	<h4>Upload CSV or Excel File</h4>
	<p>Drag and drop your file here or click to browse</p>
	<input
		type="file"
		accept=".csv,.xlsx,.xls"
		id="file-upload"
		hidden
		onchange={handleFileChange}
	/>
	<label for="file-upload" class="upload-btn">Choose File</label>
	<div class="file-info">
		<span>Supported formats: CSV, XLSX, XLS</span>
	</div>
</div>

<style>
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
		cursor: pointer;
	}

	.upload-area:hover {
		border-color: var(--accent);
		background-color: var(--bg-secondary);
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
		display: inline-block;
		padding: 10px 24px;
		background-color: var(--accent);
		color: white;
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s;
	}

	.upload-btn:hover {
		background-color: var(--primary-hover);
		transform: translateY(-1px);
	}

	.file-info {
		margin-top: 16px;
		font-size: 11px;
		color: var(--text-secondary);
	}
</style>

<script lang="ts">
	import { settingsStore } from "../../../stores/settingsStore";
	import type { AppearanceSettings } from "../../../stores/settingsStore";

	let settings = $state<AppearanceSettings>({
		theme: "dark",
		fontSize: "medium",
		accentColor: "#3b82f6",
	});

	settingsStore.subscribe((state) => {
		settings = state.appearance;
	});

	function handleSelectChange(key: keyof AppearanceSettings, value: string) {
		settingsStore.updateAppearance(key, value as any);
	}

	function handleColorChange(value: string) {
		settingsStore.updateAppearance("accentColor", value);
	}
</script>

<div class="settings-section">
	<h3 class="section-title">Appearance</h3>
	<p class="section-description">
		Customize the look and feel of the application
	</p>

	<div class="setting-item">
		<div class="setting-info">
			<label class="setting-label" for="theme-select">Theme</label>
			<p class="setting-desc">Choose your preferred theme</p>
		</div>
		<select
			class="setting-select"
			id="theme-select"
			value={settings.theme}
			onchange={(e) =>
				handleSelectChange("theme", (e.target as HTMLSelectElement).value)}
		>
			<option value="dark">Dark</option>
			<option value="light">Light</option>
			<option value="auto">Auto</option>
		</select>
	</div>

	<div class="setting-item">
		<div class="setting-info">
			<label class="setting-label" for="font-size-select">Font Size</label>
			<p class="setting-desc">Adjust interface font size</p>
		</div>
		<select
			class="setting-select"
			id="font-size-select"
			value={settings.fontSize}
			onchange={(e) =>
				handleSelectChange("fontSize", (e.target as HTMLSelectElement).value)}
		>
			<option value="small">Small</option>
			<option value="medium">Medium</option>
			<option value="large">Large</option>
		</select>
	</div>

	<div class="setting-item">
		<div class="setting-info">
			<label class="setting-label" for="accent-color">Accent Color</label>
			<p class="setting-desc">Choose your preferred accent color</p>
		</div>
		<div class="color-picker-wrapper">
			<input
				type="color"
				id="accent-color"
				class="color-picker"
				value={settings.accentColor}
				oninput={(e) => handleColorChange((e.target as HTMLInputElement).value)}
			/>
			<span class="color-value">{settings.accentColor}</span>
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

	.setting-select {
		padding: 8px 12px;
		background-color: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 6px;
		color: var(--text-primary);
		font-size: 13px;
		cursor: pointer;
		min-width: 150px;
	}

	.setting-select:focus {
		outline: none;
		border-color: var(--accent);
	}

	.color-picker-wrapper {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.color-picker {
		width: 50px;
		height: 32px;
		border: 1px solid var(--border-color);
		border-radius: 6px;
		cursor: pointer;
		background: none;
		padding: 0;
	}

	.color-picker::-webkit-color-swatch-wrapper {
		padding: 2px;
	}

	.color-picker::-webkit-color-swatch {
		border: none;
		border-radius: 4px;
	}

	.color-value {
		font-size: 12px;
		font-family: monospace;
		color: var(--text-secondary);
		min-width: 80px;
	}
</style>

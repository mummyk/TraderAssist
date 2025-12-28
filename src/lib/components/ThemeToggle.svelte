<script lang="ts">
	import { onMount } from "svelte";

	let isDark = $state(false);

	onMount(() => {
		// Check system preference or localStorage
		const savedTheme = localStorage.getItem("theme");
		if (
			savedTheme === "dark" ||
			(!savedTheme && window.matchMedia("(prefers-color-scheme: dark)").matches)
		) {
			isDark = true;
		}
		updateTheme();
	});

	function toggle() {
		isDark = !isDark;
		updateTheme();
	}

	function updateTheme() {
		document.documentElement.setAttribute(
			"data-theme",
			isDark ? "dark" : "light"
		);
		localStorage.setItem("theme", isDark ? "dark" : "light");
	}
</script>

<button class="theme-btn" onclick={toggle} aria-label="Toggle theme">
	{#if isDark}
		<!-- Sun Icon -->
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="24"
			height="24"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			><circle cx="12" cy="12" r="4" /><path d="M12 2v2" /><path
				d="M12 20v2"
			/><path d="m4.93 4.93 1.41 1.41" /><path
				d="m17.66 17.66 1.41 1.41"
			/><path d="M2 12h2" /><path d="M20 12h2" /><path
				d="m6.34 17.66-1.41 1.41"
			/><path d="m19.07 4.93-1.41 1.41" /></svg
		>
	{:else}
		<!-- Moon Icon -->
		<svg
			xmlns="http://www.w3.org/2000/svg"
			width="24"
			height="24"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" /></svg
		>
	{/if}
</button>

<style>
	.theme-btn {
		background: var(--surface-color);
		border: 1px solid var(--border-color);
		color: var(--text-primary);
		width: 40px;
		height: 40px;
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
		transition: all 0.2s;
	}
	.theme-btn:hover {
		transform: scale(1.1);
	}
</style>

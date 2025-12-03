<script lang="ts">
	import { page } from '$app/stores';
</script>

<svelte:head>
	<title>Insufficient Permissions</title>
</svelte:head>

<div class="container">
	<div class="error-card">
		<div class="error-icon">
			<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
				<circle cx="12" cy="12" r="10" />
				<line x1="15" y1="9" x2="9" y2="15" />
				<line x1="9" y1="9" x2="15" y2="15" />
			</svg>
		</div>

		<h1 class="text-white">Insufficient Permissions</h1>
		<p class="text-white">You don't have the required permissions to access this page.</p>

		<div class="actions">
			<a href="/" class="btn btn-primary">Go Home</a>
			<a href="/admin" class="btn btn-secondary">Admin Dashboard</a>
			<button on:click={() => history.back()} class="btn btn-outline">Go Back</button>
		</div>
	</div>
</div>

<style>
	.container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem;
		background: #1e1e1e;
	}

	.error-card {
		background: 11ee1e;
		padding: 3rem;
		border-radius: 1rem;
		-webkit-box-shadow: 0px 0px 80px 7px rgba(7, 7, 7, 1);
		-moz-box-shadow: 0px 0px 80px 7px rgba(7, 7, 7, 1);
		box-shadow: 0px 0px 80px 7px rgba(7, 7, 7, 1);
		text-align: center;
		max-width: 500px;
		width: 100%;
	}

	.error-icon {
		color: #ef4444;
		margin-bottom: 1.5rem;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	h1 {
		color: #fff;
		font-size: 2rem;
		font-weight: 700;
		margin-bottom: 1rem;
	}

	p {
		color: #6b7280;
		font-size: 1.125rem;
		margin-bottom: 2rem;
		line-height: 1.6;
	}

	.actions {
		display: flex;
		gap: 1rem;
		justify-content: center;
		flex-wrap: wrap;
	}

	.btn {
		padding: 0.75rem 1.5rem;
		border-radius: 0.5rem;
		text-decoration: none;
		font-weight: 600;
		transition: all 0.2s;
		border: none;
		cursor: pointer;
		font-size: 0.875rem;
	}

	.btn-primary {
		background: #3b82f6;
		color: white;
	}

	.btn-primary:hover {
		background: #2563eb;
	}

	.btn-secondary {
		background: #6b7280;
		color: white;
	}

	.btn-secondary:hover {
		background: #4b5563;
	}

	.btn-outline {
		background: transparent;
		color: #6b7280;
		border: 2px solid #d1d5db;
	}

	.btn-outline:hover {
		background: #f9fafb;
		border-color: #9ca3af;
	}

	@media (max-width: 640px) {
		.container {
			padding: 1rem;
		}

		.error-card {
			padding: 2rem;
		}

		.actions {
			flex-direction: column;
		}

		.btn {
			width: 100%;
		}
	}
</style>

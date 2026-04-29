<svelte:head>
	<title>Admin Dashboard</title>
</svelte:head>

<div class="wrap">
	<div class="card">
		<h1>Admin Dashboard</h1>
		<p>Coming soon.</p>
		<a class="link" href="/home">Back to Home</a>
	</div>
</div>

<style>
	.wrap {
		display: grid;
		place-items: center;
		min-height: 100vh;
		width: 100%;
		padding: 1rem;
		box-sizing: border-box;
	}

	.card {
		width: min(720px, 92vw);
		border-radius: 16px;
		padding: 2rem;
		background: rgba(38, 42, 51, 0.85);
		border: 1px solid rgba(255, 255, 255, 0.06);
		box-shadow: 0 20px 60px rgba(0, 0, 0, 0.45);
		backdrop-filter: blur(8px);
	}

	h1 {
		margin: 0 0 0.5rem 0;
		font-size: 2rem;
		font-weight: 700;
		color: #f7fafc;
	}

	p {
		margin: 0 0 1rem 0;
		color: #a0aec0;
		line-height: 1.6;
	}

	.link {
		display: inline-block;
		color: #68d391;
		text-decoration: none;
		font-weight: 600;
	}

	.link:hover {
		text-decoration: underline;
	}
</style>

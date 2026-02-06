<script>
	import { isAuthenticated, user } from '$lib/stores/auth';
</script>

<p>This is a test of another page.</p>
<br /> <br />

{#if $isAuthenticated && $user}
	<div>
		<span>User name: {$user.name}</span><br />
		<span>User email: {$user.email}</span><br />
	</div>
{:else}
	<div class="action-card">
		Go to <a href="/">home</a>.
	</div>
{/if}

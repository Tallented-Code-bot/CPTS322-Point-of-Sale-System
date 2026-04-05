<script lang="ts">
import { goto } from '$app/navigation';

async function checkout(){
    goto('./cashier');
}
</script>

<button on:click={checkout} class="button logout"> Checkout </button>


<style>
	.button {
		font-size: 1.2rem;
		letter-spacing: 0.12em;
		font-weight: 700;
		padding: 1rem 1.5rem;
		border-radius: 7px;
		border: none;
		cursor: pointer;
		transition: opacity 0.15s, transform 0.1s;
        background-color: rgb(145, 0, 0);
    }

	.button:focus {
		box-shadow: 0 0 0 4px rgba(99, 179, 237, 0.5);
	}
</style>

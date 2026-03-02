<script lang="ts">
import { goto } from '$app/navigation';

async function checkout(){
    goto('./cashier');
}
</script>

<button on:click={checkout} class="button logout"> Checkout </button>


<style>
	.button {
		padding: 1.1rem 2.8rem;
		font-size: 1.2rem;
		font-weight: 600;
		border-radius: 10px;
		border: none;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.25, 0.8, 0.25, 1);
		color: 0 8px 20px rgba(255, 0, 212, 0.4);
		text-transform: uppercase;
		letter-spacing: 0.08em;
		outline: none;
        background-color: rgb(145, 0, 0);
    }

	.button:focus {
		box-shadow: 0 0 0 4px rgba(99, 179, 237, 0.5);
	}
</style>

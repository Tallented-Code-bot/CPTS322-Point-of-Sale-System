<script lang="ts">
    import {isLoading, isLoggedIn, user} from '$lib/stores/auth';
    import LogoutButton from '$lib/components/LogoutButton.svelte';
	  import { goto } from '$app/navigation';
    import CheckoutButton from '$lib/components/CheckoutButton.svelte';
    import {
		addToCart,
		cartItems,
		subtotal,
		tax,
		total,
		setQty,
		removeItem,
		clearCart,
		buildCheckoutPayload
	} from '$lib/stores/cart';
    import { fetchProductByUPC, checkout } from '$lib/api/pos';


    

  let width = window.innerWidth;
  let height = window.innerHeight;

  console.log($isLoggedIn + " from /home")

  let upc = '';
	let error = '';
	let paid = 0;
	let receiptMsg = '';
	let isSubmitting = false;
	let isScanning = false;

  function getErrMsg(e: unknown) {
		if (e instanceof Error) return e.message;
		if (typeof e === 'string') return e;
		try {
			return JSON.stringify(e);
		} catch {
			return 'Unknown error.';
		}
	}

  async function scanAdd() {
		error = '';
		receiptMsg = '';

		const trimmed = upc.trim();
		if (!trimmed) return;

		isScanning = true;

		try {
			// Debug: confirm the handler fired
			console.log('[cashier] scanAdd UPC:', trimmed);

			const product = await fetchProductByUPC(trimmed);
			console.log(product);

			// Guard: ensure product looks valid for your cart store
			if (!product || !product.upc || !product.name || typeof product.price !== 'number') {
				console.error('[cashier] Invalid product returned:', product);
				throw new Error('Product data from API is missing fields (upc/name/price).');
			}

			addToCart(product, 1);
			upc = '';
		} catch (e) {
			console.error('[cashier] scanAdd failed:', e);
			error = getErrMsg(e) || 'Could not find product.';
		} finally {
			isScanning = false;
		}
	}

 
</script>




<div class="testclass">
    {#if $isLoggedIn}
        
        <div class="listwindow">

           <div class="scanRow">
            <input
              placeholder="Scan or type UPC and press Enter"
              bind:value={upc}
              autocomplete="off"
              autocapitalize="off"
              spellcheck="false"
            />
            <button on:click={scanAdd} disabled={isScanning}>
              {isScanning ? 'Adding...' : 'Add'}
            </button>
          </div>

        </div>
        <div class="topbar">
          <LogoutButton />
          <div class="deets">
            {#if $user}
            <span>User name: {$user.name}</span><br />
            <span>User email: {$user.email}</span>
            {/if}
          </div>
        </div>
        <div class="bottombar">
          <!-- <div class="scanRow">
            <input
              placeholder="Scan or type UPC and press Enter"
              
              autocomplete="off"
              autocapitalize="off"
              spellcheck="false"
            /> -->
          <!-- <button type="submit" disabled={isScanning}>
            {isScanning ? 'Adding...' : 'Add'}
          </button> 
        </div> -->
          <CheckoutButton />
        </div>
    {:else}
        <div>
            Please sign in first.
            <LogoutButton />
        </div>
    {/if}
    
</div>

<style>
    .testclass{
        display: flex;
        height: 100vh;
        justify-content: center; /* centers horizontally */
        align-items: center; /* centers vertically */
        padding-top: 5vh;
        padding-bottom: 10vh;
    }
    .listwindow {
        height:80vh;
        width: 100%;
        overflow: auto;
    }
    .topbar{
      position:fixed;
      display:flex;
      z-index: 9999;
      top: 0;       
      right: 0;
      height:10vh;
      width:100%;
      vertical-align: top;
      align-items:center;
      justify-content:flex-end;
      padding-right: 1rem;
      pointer-events: auto
    }
    .bottombar{
      position:fixed;
      display:flex;
      z-index: 9999;
      bottom: 0;       
      right: 0;
      height:10vh;
      width:100%;
      margin-top: auto;
      vertical-align: bottom;
      align-items:center;
      justify-content:flex-end;
      padding-right: 1rem;
    }
    .deets{
      position:fixed;
      display:flex;
      flex-direction:column;
      top: 0;       
      left: 0;
      height:10vh;
      width:50%; /*Change as needed */
      vertical-align: top;
      justify-content:flex-start;
      padding-right: 1rem;
    }
    
    .scanRow {
		display: flex;
		gap: 0.75rem;
		margin: 1rem 0 1.25rem;
	}
</style>
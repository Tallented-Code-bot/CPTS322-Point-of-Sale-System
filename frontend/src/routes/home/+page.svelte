<script lang="ts">
    import {isLoading, isLoggedIn, user} from '$lib/stores/auth';
    import LogoutButton from '$lib/components/LogoutButton.svelte';
	  import { goto } from '$app/navigation';
    import CheckoutButton from '$lib/components/CheckoutButton.svelte';
    // import { BottomNav, BottomNavItem } from "flowbite-svelte";


    //this is just a random item generator from flowbyte svelte
    function getRandomLorem(minWords: number, maxWords: number) {
    const lorem = "Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua".split(" ");
    const wordCount = Math.floor(Math.random() * (maxWords - minWords + 1)) + minWords;
    let result = [];
    for (let i = 0; i < wordCount; i++) {
      const word = lorem[Math.floor(Math.random() * lorem.length)];
      result.push(word);
    }
    return result.join(" ");


  }

  const items = Array.from({ length: 5000 }, (_, i) => `Item ${i + 1}: ${getRandomLorem(10, 70)}`);

  let width = window.innerWidth;
  let height = window.innerHeight;

  console.log('items length', items.length);

  console.log($isLoggedIn + " from /home")

 
</script>


<div class="testclass">
    {#if $isLoggedIn}
        <div class="listwindow">
            {#each items as item, index}
                <div class="border-b p-2 text-gray-900 dark:text-white dark:hover:bg-gray-800">
                {index} / {items.length} - {item}
                </div>
            {/each}
            
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
      z-index: 9999;
      top: 0;       
      right: 0;
      height:10vh;
      width:100%;
      vertical-align: top;
      justify-content:flex-start;
      padding-right: 1rem;
    }
</style>
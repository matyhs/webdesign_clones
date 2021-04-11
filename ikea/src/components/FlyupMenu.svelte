<script>
    import { createEventDispatcher } from 'svelte';
    import { slide } from 'svelte/transition';

    let expanded = false;
    const dispatch = createEventDispatcher();

    function expand() {
        expanded = !expanded;
        dispatch('expanded', {
            expanded
        });
    }

</script>

<div class="tw-relative">
    {#if expanded}
    <ul class="tw-absolute tw-bottom-24 tw-bg-white tw-w-full tw-overflow-y-auto tw-h-96 tw-rounded-t-5xl" transition:slide>
        <li class="tw-px-4 tw-py-6 hover:tw-bg-gray-200">Option 1</li> 
        <li class="tw-px-4 tw-py-6 hover:tw-bg-gray-200">Option 2</li> 
        <li class="tw-px-4 tw-py-6 hover:tw-bg-gray-200">Option 3</li>
        <li class="tw-px-4 tw-py-6 hover:tw-bg-gray-200">Option 4</li>
        <li class="tw-px-4 tw-py-6 hover:tw-bg-gray-200">Option 5</li>
    </ul> 
    {/if}
    <div class="tw-flex tw-w-full tw-bg-yellow-400 tw-justify-center {expanded? 'tw-rounded-b-5xl' : 'tw-rounded-full'}">
        {#if !expanded}
            <button class="tw-font-bold tw-px-8">Go to</button>
        {/if}
        <div class="tw-flex tw-flex-1 tw-px-4 tw-py-8 tw-w-full tw-bg-yellow-300 tw-justify-between tw-items-center {expanded? 'tw-rounded-b-5xl' : 'tw-rounded-r-full'}" on:click={expand}>
            <span>Select a site</span>
            <!-- Please refer: https://github.com/shubhamjain/svg-loader -->
            <svg class="tw-transition-transform tw-duration-500 tw-ease-in-out {expanded? 'tw-transform tw--rotate-180' : ''}" data-src="https://s2.svgbox.net/hero-outline.svg?ic=chevron-up" width="32" height="32" color="currentColor"></svg>
        </div>
    </div>
</div>
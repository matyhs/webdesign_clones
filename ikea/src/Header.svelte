<script>
    import {cubicOut} from 'eases-jsnext';

    export let title;
    export let expanded = false;

    function handleOnMenuClicked()
    {
        expanded = !expanded;
    }

    function slide(node, { 
        delay = 0, 
        duration = 400, 
        easing = cubicOut 
    }) {
        const style = getComputedStyle(node);
        const opacity = +getComputedStyle(node).opacity;
        const width = parseFloat(style.width);
        const paddingTop = parseFloat(style.paddingTop);
        const paddingBottom = parseFloat(style.paddingBottom);
        const marginTop = parseFloat(style.marginTop);
        const marginBottom = parseFloat(style.marginBottom);
        const borderTopWidth = parseFloat(style.borderTopWidth);
        const borderBottomWidth = parseFloat(style.borderBottomWidth);

        return {
            delay,
            duration,
            easing,
            css: t =>
                `overflow: hidden;` +
                `opacity: ${t*opacity};` +
                `width: ${t * width}px;` +
                `padding-top: ${t * paddingTop}px;` +
                `padding-bottom: ${t * paddingBottom}px;` +
                `margin-top: ${t * marginTop}px;` +
                `margin-bottom: ${t * marginBottom}px;` +
                `border-top-width: ${t * borderTopWidth}px;` +
                `border-bottom-width: ${t * borderBottomWidth}px;`
	    };
    }

</script>

<header class="tw-flex tw-items-center tw-py-6 tw-sticky tw-top-0 tw-bg-white tw-z-10">
    <div class="tw-flex-col tw-cursor-pointer tw-z-20 tw-px-8" on:click={handleOnMenuClicked}>
        <div class="tw-border-2 tw-h-1 tw-w-7 tw-transition-transform {expanded? 'tw-transform tw-rotate-45 tw-translate-y-2 tw-border-white' : 'tw-border-black'}"></div>
        <div class="tw-border-2 tw-h-1 tw-w-7 tw-my-1 tw-transition-transform {expanded? 'tw-transform tw-scale-0 tw-border-white' : 'tw-border-black'}"></div>
        <div class="tw-border-2 tw-h-1 tw-w-7 tw-transition-transform {expanded? 'tw-transform tw--rotate-45 tw--translate-y-2 tw-border-white' : 'tw-border-black'}"></div>
    </div>
    <div class="tw-text-yellow-300 tw-font-extrabold tw-text-3xl tw-uppercase tw-z-20">{title}</div>
    {#if expanded}
    <div class="tw-absolute tw-top-0 tw-w-full tw-h-screen tw-bg-black tw-bg-opacity-60">
        <div class="tw-flex tw-items-center tw-justify-center tw-w-1/3 tw-h-full tw-bg-blue-800" transition:slide>
            <div class="tw-flex tw-flex-col tw-text-white tw-text-2xl tw-space-y-3">
                <span class="tw-cursor-pointer">Shop</span>
                <span class="tw-cursor-pointer">About MQDupe</span>
                <span class="tw-cursor-pointer">Museum</span>
                <span class="tw-cursor-pointer">International Sales</span>
            </div>
        </div>
    </div>
    {/if}
</header>

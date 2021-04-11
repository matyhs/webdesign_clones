<script>
    import { createEventDispatcher } from 'svelte';

    let hovering;
    const dispatcher = createEventDispatcher();

    function enter() {
        hovering = true;
        dispatcher('enter');
    }

    function leave() {
        hovering = false;
        dispatcher('exit');
    }

</script>

<div on:mouseenter="{() => hovering = true}" on:mouseleave="{() => hovering = false}">
    <slot {hovering}></slot>
</div>
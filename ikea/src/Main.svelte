<script>
    import FlexImage from './components/FlexImage.svelte';
    import FlexImageContainer from './components/FlexImageContainer.svelte';
    import FlyupMenu from './components/FlyupMenu.svelte';

    export let title;
    let expanded = false;

    function handleOnFlyupMenuExpanded(event)
    {
        expanded = event.detail.expanded
    }
</script>

<div class="tw-px-8">
    <section class="tw-flex tw-h-screen80 tw-space-x-5">
        <div class="tw-relative tw-w-2/3">
            <img class="tw-w-full tw-h-full" src="http://cdn.home-designing.com/wp-content/uploads/2019/09/ring-pendant-light.jpg" alt=""/>
            <div class="tw-absolute tw-top-0 tw-w-full tw-h-full">
                <div class="tw-relative tw-text-center tw-mt-3 tw-h-1/5">
                    <span class="tw-block tw-text-9xl tw-text-yellow-300 tw-font-black tw-transition-transform {expanded? 'tw-transform tw-scale-50' : ''}">{title}</span>
                </div>
                <div class="tw-relative tw-flex tw-flex-col tw-items-center tw-top-1/3 tw-h-1/3">
                    <div class="tw-w-1/2">
                        <span class="tw-text-white tw-font-bold tw-text-4xl">Go shopping on your local site</span>
                    </div>
                    <div class="tw-w-1/2 tw-text-white tw-text-4xl tw-p-5 tw-animate-bounce">
                        <svg data-src="https://s2.svgbox.net/hero-outline.svg?ic=arrow-down" width="32" height="32" color="currentColor"></svg>
                    </div>
                    <div class="tw-w-1/2 tw-h-1/3">
                        <!-- <select class="tw-w-full tw-h-full tw-rounded-full">
                            <option>1</option>
                            <option>2</option>
                            <option>3</option>
                            <option>4</option>
                        </select> -->
                        <FlyupMenu on:expanded={handleOnFlyupMenuExpanded}/>
                    </div>
                </div>
            </div>
        </div>
        <div class="tw-w-1/3">
            <FlexImageContainer>
                <!-- <a href="https://www.freepik.com/photos/vintage">Vintage photo created by kjpargeter - www.freepik.com</a> -->
                <FlexImage src="img/sample.png"/>
                <!-- <a href="https://www.freepik.com/photos/house">House photo created by vanitjan - www.freepik.com</a> -->
                <FlexImage src="img/sample1.png"/>
                <!-- <a href="https://www.freepik.com/photos/background">Background photo created by valeria_aksakova - www.freepik.com</a> -->
                <FlexImage src="img/sample2.png"/>
                <!-- <a href="https://www.freepik.com/photos/light">Light photo created by wirestock - www.freepik.com</a> -->
                <FlexImage src="img/sample3.png"/>
            </FlexImageContainer>
        </div>
    </section>
    <section class="tw-flex tw-h-screen80 tw-space-x-5 tw-pt-5">
        <div class="tw-relative tw-w-1/3">
            <img class="tw-w-full tw-h-full" src="https://i.pinimg.com/736x/54/bf/d6/54bfd600f97198bdad7058eca3618e16.jpg" alt=""/>
            <div class="tw-absolute tw-flex tw-flex-col tw-top-0 tw-h-full tw-w-full">
                <div class="tw-flex tw-w-full tw-justify-end">
                    <button class="tw-rounded-full tw-bg-white tw-p-1 tw-m-5 tw-text-black">
                        <svg data-src="https://s2.svgbox.net/hero-outline.svg?ic=dots-horizontal" width="32" height="32" color="currentColor"></svg>
                    </button>
                </div>
                <div class="tw-flex tw-flex-col tw-h-full tw-w-full tw-justify-center tw-items-center tw-space-y-8">
                    <span class="tw-text-white tw-text-3xl tw-font-semibold">No store in your location?</span>
                    <button class="tw-flex tw-bg-white tw-place-items-center tw-px-10 tw-py-3 tw-rounded-full tw-text-black">
                        <svg class="tw-text-xl tw-mx-2" data-src="https://s2.svgbox.net/hero-outline.svg?ic=arrow-right" width="32" height="32" color="currentColor"></svg>
                        <span class="tw-text-xl tw-font-bold">International sales</span>
                    </button>
                </div>
            </div>
        </div>
        <div class="tw-relative tw-w-2/3">
            <img class="tw-w-full tw-h-full" src="https://www.italianbark.com/wp-content/uploads/2020/10/minimalist-tile-ideas-casalgrandepadana-r-evolution_amb_terra-tortora.jpg" alt=""/>
            <div class="tw-absolute tw-top-0 tw-flex tw-items-end tw-justify-end tw-w-full tw-h-full">
                <div class="tw-flex tw-items-center tw-w-1/3 tw-mb-8 tw-transform tw-duration-1000 tw-ease-in-out hover:tw-translate-x-8 tw-cursor-pointer mq-gradient-text tw-bg-gradient-to-r tw-from-white tw-to-yellow-500 tw-text-red-300">
                    <span class="tw-w-2/3 tw-font-semibold tw-text-4xl">Explore something special</span>
                    <svg class="tw-text-2xl" data-src="https://s2.svgbox.net/hero-outline.svg?ic=arrow-right" width="32" height="32" color="currentColor"></svg>
                </div>
            </div>
        </div>
    </section>
</div>

<style>
    .mq-gradient-text {
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
    }
</style>
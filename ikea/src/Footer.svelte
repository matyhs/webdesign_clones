<script>
    import Backdrop from './components/Backdrop.svelte';
    import Clock from './components/Clock.svelte';
</script>

<section class="tw-flex tw-flex-col tw-h-screen80">
    <div class="tw-flex tw-flex-col tw-space-y-8 tw-h-1/2 tw-text-center tw-justify-center">
        <p>No store in your location? <span class="tw-font-bold tw-underline tw-cursor-pointer">See our international sales</span></p>
        <p class="tw-text-7xl tw-font-bold">Shop till you drop</p>
        <div class="tw-text-xl tw-flex tw-justify-center">
            <span class="tw-cursor-pointer tw-px-2 hover:tw-underline">About</span>
            <span class="tw-cursor-pointer tw-border-l tw-border-black tw-px-2 hover:tw-underline">Life at home</span>
            <span class="tw-cursor-pointer tw-border-l tw-border-black tw-px-2 hover:tw-underline">Sustainability</span>
            <span class="tw-cursor-pointer tw-border-l tw-border-black tw-px-2 hover:tw-underline">Work with us</span>
            <span class="tw-cursor-pointer tw-border-l tw-border-black tw-px-2 hover:tw-underline">Newsroom</span>
        </div>
    </div>
    <div class="tw-h-1/2">
        <svg width="100%" height="100%">
            <g>
                <!-- <a href='https://www.freepik.com/vectors/background'>Background vector created by upklyak - www.freepik.com</a> -->
                <Backdrop href="img/sample5.png"/>
            </g>
            <svg viewBox="-5 8 20 100">
                <Clock />
            </svg>
        </svg>
    </div>
</section>
<section class="tw-bg-gradient-to-b tw-from-red-500 tw-to-yellow-800 tw-flex tw-justify-between tw-items-center tw-py-8 tw-px-4">
    <div class="tw-text-white">
      © MQDupe 2021
    </div>
    <div class="tw-text-white tw-cursor-pointer">
        Cookie Information  
    </div>
  </section>
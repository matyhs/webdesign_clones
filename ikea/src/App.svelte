<script>
	import Imports from './Imports.svelte';
	import Header from './Header.svelte';
	import Main from './Main.svelte';
	import Footer from './Footer.svelte';
	import Sidebar from './Sidebar.svelte';

	let title = "MQDupe";
	let menuExpanded;
</script>

<Imports />

<Header bind:expanded={menuExpanded} {title}/>
<Main {title}/>
<Footer />

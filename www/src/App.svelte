<script>
	export let bindings;
	import { Content } from "carbon-components-svelte";
</script>

<Content>
	<h1>
		{bindings.add(5, 7)}
		{bindings.mul(5, 7)}
	</h1></Content
>

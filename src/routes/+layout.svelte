<script lang="ts">
	import '../app.css';

	import Sun from 'lucide-svelte/icons/sun';
	import Moon from 'lucide-svelte/icons/moon';

	import { ModeWatcher } from 'mode-watcher';
	import { toggleMode } from 'mode-watcher';
	import { Button } from '$lib/components/ui/button/index.js';
</script>

<div class="h-full flex flex-col font-serif">
	<ModeWatcher />
	<div class="flex h-16 place-content-between items-center border-b-2">
		<div class="ml-7">
			<p class="whitespace-nowrap text-3xl font-semibold">CANDBC</p>
		</div>
		<div class="mr-5">
			<Button on:click={toggleMode} variant="outline" size="icon">
				<Sun
					class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
				/>
				<Moon
					class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
				/>
			</Button>
		</div>
	</div>
	<div class="flex-1 flex-col flex">
		<slot />
	</div>
</div>

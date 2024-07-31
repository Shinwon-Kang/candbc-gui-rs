<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { ScrollArea } from '$lib/components/ui/scroll-area/index.js';
</script>

<div class="space-y-5 text-xl font-semibold dark:text-white">
	<div>
		<p>DBC file:</p>
		<div class="mt-2">
			<Badge variant="secondary" class="text-base">CANDBC_FILE_.dbc</Badge>
		</div>
	</div>
	<div>
		<p>Version:</p>
		<div class="mt-2">
			<Badge variant="secondary" class="mt-1 text-base">0.1.0</Badge>
		</div>
	</div>
	<div>
		<p>Symbols:</p>
		<div class="mt-2 space-x-1">
			<Badge variant="secondary" class="text-base">CM_</Badge>
			<Badge variant="secondary" class="text-base">BA_</Badge>
			<Badge variant="secondary" class="text-base">VAL_</Badge>
			<Badge variant="secondary" class="text-base">CAT_</Badge>
		</div>
	</div>
	<div>
		<p>Nodes:</p>
		<div class="mt-2 space-x-1">
			<Badge variant="secondary" class="text-base">K16_BECM</Badge>
			<Badge variant="secondary" class="text-base">K114B_HPCM</Badge>
			<Badge variant="secondary" class="text-base">T18_BatteryCharger</Badge>
		</div>
	</div>
	<div>
		<p>Messages:</p>
		<div class="mt-2 space-x-1">
			<Badge variant="secondary" class="text-base">WebData_1840 (3)</Badge>
			<Badge variant="secondary" class="text-base">Battery_Module_1 (10)</Badge>
			<Badge variant="secondary" class="text-base">Battery_Module_2 (15)</Badge>
		</div>
	</div>
	<div>
		<p>Comments:</p>

		<ul
			class="mt-2 w-full text-sm font-medium text-gray-900 border border-gray-200 rounded-lg dark:border-gray-600"
		>
			<li class="w-full px-4 border-b border-gray-200 rounded-t-lg dark:border-gray-600">
				<div class="py-3 space-x-1">
					<Badge variant="secondary" class="text-base"
						>Imported file _honda_common.dbc starts here</Badge
					>
				</div>
			</li>
			<li class="w-full px-4 border-b border-gray-200 rounded-t-lg dark:border-gray-600">
				<div class="py-3 space-x-1">
					<Badge variant="secondary" class="text-base">BO_</Badge>
					<Badge variant="secondary" class="text-base">1840</Badge>
					<Badge variant="secondary" class="text-base">Some Message comment</Badge>
				</div>
			</li>
			<li class="w-full px-4 border-b border-gray-200 rounded-t-lg dark:border-gray-600">
				<div class="py-3 space-x-1">
					<Badge variant="secondary" class="text-base">SG_</Badge>
					<Badge variant="secondary" class="text-base">1840</Badge>
					<Badge variant="secondary" class="text-base">Signal_4</Badge>
					<Badge variant="secondary" class="text-base"
						>asaklfjlsdfjlsdfglsHH?=(%)/&KKDKFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv</Badge
					>
				</div>
			</li>
			<li class="w-full px-4 border-b border-gray-200 rounded-t-lg dark:border-gray-600">
				<div class="py-3 space-x-1">
					<Badge variant="secondary" class="text-base">SG_</Badge>
					<Badge variant="secondary" class="text-base">5</Badge>
					<Badge variant="secondary" class="text-base">TestSigLittleUnsigned1</Badge>
					<Badge variant="secondary" class="text-base"
						>asaklfjlsdfjlsdfgls=0943503450KFSDKFKDFKSDFKSDFNKCnvsdcvsvxkcv</Badge
					>
				</div>
			</li>
			<li class="w-full px-4 border-b border-gray-200 rounded-t-lg dark:border-gray-600">
				<div class="py-3 space-x-1">
					<Badge variant="secondary" class="text-base">BU_</Badge>
					<Badge variant="secondary" class="text-base">K17_EBCM</Badge>
					<Badge variant="secondary" class="text-base">Electronic Brake Control Module</Badge>
				</div>
			</li>
		</ul>
	</div>
</div>

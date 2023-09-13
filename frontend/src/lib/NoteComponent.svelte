<script lang="ts">
	import { ListBox, ListBoxItem, RadioGroup, RadioItem } from '@skeletonlabs/skeleton';
	import { data } from '../store';

	let currently_all_forced_visible: boolean = false;
	let selected_preset: Preset | null = null;
	let current_output: string = '';

	$: {
		if (selected_preset) {
			let output = '';
			selected_preset.fields.forEach((field) => {
				output += field.current_inputs.join(' ');
				output += ';';
			});
			current_output = output.slice(0, -1);
		}
	}
</script>

<h2 class="h2 mt-12">Create a card</h2>
{#if $data.presets.length}
	<RadioGroup>
		{#each $data.presets as preset}
			<RadioItem bind:group={selected_preset} name="type" value={preset}>{preset.name}</RadioItem>
		{/each}
	</RadioGroup>

	{#if selected_preset}
		<div class="card p-4 variant-ghost-warning">
			delete preset {selected_preset.name}
			<button
				type="button"
				class="btn-icon variant-filled-primary"
				style="font-weight: bold;"
				on:click={() => {
					// @ts-ignore
					$data.presets = $data.presets.filter((p) => p !== selected_preset);
					selected_preset = null;
					localStorage.setItem('presets', JSON.stringify($data.presets));
				}}
			>
				<i class="fa-solid fa-remove" />
			</button>
		</div>
		<div class="card p-4 variant-ghost-secondary">
			force each field visible
			<button
				type="button"
				class={`btn-icon variant-filled${currently_all_forced_visible ? '-warning' : ''}`}
				style="font-weight: bold;"
				on:click={() => {
					currently_all_forced_visible = !currently_all_forced_visible;
				}}
			>
				{#if currently_all_forced_visible}
					<i class="fa-solid fa-eye" />
				{:else}
					<i class="fa-solid fa-eye-slash" />
				{/if}
			</button>
		</div>
		<div style="display: grid; grid-template-columns: 140px 1fr 5ch 5ch;">
			{#each selected_preset.fields as field}
				{#if field.currently_visible || currently_all_forced_visible}
					<div style="display: flex; justify-content: center; align-items: center;">
						{field.name}
					</div>
					{#if field.type === 'text'}
						<input type="text" bind:value={field.current_inputs[0]} />
					{:else if field.type === 'selectOne'}
						<RadioGroup>
							{#each field.options as option}
								<RadioItem bind:group={field.current_inputs[0]} name="type" value={option}
									>{option || '(empty)'}</RadioItem
								>
							{/each}
						</RadioGroup>
					{:else if field.type === 'selectMany'}
						<ListBox multiple>
							<div class="card" style="display: flex; flex-direction: row;">
								{#each field.options as option}
									<ListBoxItem bind:group={field.current_inputs} name="type" value={option}
										>{option || '(empty)'}</ListBoxItem
									>
								{/each}
							</div>
						</ListBox>
					{/if}
					<button
						style="width: 4.5ch;"
						class="btn btn-large variant-filled"
						on:click={() => {
							field.current_inputs = JSON.parse(JSON.stringify(field.default));
						}}
					>
						<abbr title={`reset to '${field.default}'`}><i class="fa-solid fa-rotate-left" /></abbr>
					</button>
					<button
						style="width: 4.5ch;"
						class="btn btn-large {field.currently_frozen ? 'variant-filled' : 'variant-ghost'}"
						on:click={() => {
							field.currently_frozen = !field.currently_frozen;
						}}
					>
						<abbr title={`reset to '${field.default}'`}>
							{#if field.currently_frozen}
								<i class="fa-solid fa-lock" />
							{:else}
								<i class="fa-solid fa-lock-open" />
							{/if}
						</abbr>
					</button>

					<!-- <button
                    style="width: 4.5ch;"
                    on:click={() => {
                        field.currently_visible = !field.currently_visible;
                    }}
                    class="btn btn-large {field.currently_visible
                        ? `variant-filled${field.visible_by_default ? '' : '-primary'}`
                        : `variant-ghost${field.visible_by_default ? '-primary' : ''}`}"
                >
                    <div>
                        {#if field.currently_visible}
                            <i class="fa-solid fa-eye" />
                        {:else}
                            <i class="fa-solid fa-eye-slash" />
                        {/if}
                    </div>
                </button> -->
					<!-- </div> -->
				{/if}
			{/each}
		</div>
		<button
			style="margin-top: 12px;"
			class="btn btn-large variant-filled-success"
			on:click={() => {
				$data.string_for_export +=
					current_output +
					`
`;
				if (selected_preset?.fields?.length) {
					for (let i = 0; i < selected_preset?.fields?.length || 0; i++) {
						if (!selected_preset?.fields[i].currently_frozen) {
							selected_preset.fields[i].current_inputs = JSON.parse(
								JSON.stringify(selected_preset?.fields[i].default)
							);
						}
					}
				}
				localStorage.setItem('string_for_export', $data.string_for_export);
			}}>add card</button
		>
		<div>
			current result: <pre>{current_output}</pre>
		</div>
	{/if}
{/if}

<style lang="postcss">
	input {
		color: black;
	}
</style>
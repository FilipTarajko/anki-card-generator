<script lang="ts">
	import { ListBox, ListBoxItem, RadioGroup, RadioItem } from '@skeletonlabs/skeleton';
	import { data } from '../store';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import axios from 'axios';

	const toastStore = getToastStore();

	function showErrorToast(message: string) {
		toastStore.trigger({
			message,
			timeout: 5000,
			background: 'variant-filled-primary',
			autohide: true,
			hideDismiss: false
		});
	}

	function showWarningToast(message: string) {
		toastStore.trigger({
			message,
			timeout: 5000,
			background: 'variant-filled-warning',
			autohide: true,
			hideDismiss: false
		});
	}

	function showSuccessToast(message: string) {
		toastStore.trigger({
			message,
			timeout: 5000,
			background: 'variant-filled-success',
			autohide: true,
			hideDismiss: false
		});
	}

	let preset_name = 'new preset';
	let selected_hue: string = '';
	let new_field_name = '';
	let new_field_type: 'text' | 'selectOne' | 'selectMany' | null = null;
	let selected_preset: Preset | null = null;
	let default_fields = [
		{
			id: 0,
			name: 'deck',
			type: 'bound',
			options: [],
			default: [],
			visible_by_default: true,
			current_inputs: [],
			bound_to: 5,
			bindings: [
				['nl', 'Dutch'],
				['en', 'English']
			]
		},
		{
			id: 1,
			name: 'notetype',
			type: 'selectOne',
			options: ['2X22', '3X22'],
			default: ['2X22'],
			visible_by_default: true,
			current_inputs: []
		},
		{
			id: 2,
			name: 'tags',
			type: 'selectMany',
			options: ['AnkiCC', 'test', 'test::AnkiCC', 'test::foo::baz::baz'],
			default: ['AnkiCC', 'test'],
			visible_by_default: true,
			current_inputs: []
		},
		{
			id: 3,
			name: 'front',
			type: 'text',
			options: [],
			default: [],
			visible_by_default: true,
			current_inputs: []
		},
		{
			id: 4,
			name: 'back',
			type: 'text',
			options: [],
			default: [],
			visible_by_default: true,
			current_inputs: []
		},
		{
			id: 5,
			name: 'theme',
			type: 'selectOne',
			options: ['nl', 'en', 'de', 'pl'],
			default: [],
			visible_by_default: true,
			current_inputs: []
		}
	];
	let fields: Field[] = JSON.parse(JSON.stringify(default_fields));

	function create_field() {
		fields = [
			...fields,
			{
				id: fields.length,
				name: new_field_name,
				type: new_field_type ?? 'text',
				options: [],
				default: [],
				visible_by_default: true,
				current_inputs: []
			}
		];
		new_field_name = '';
		new_field_type = null;
	}

	function validate_and_prepare_fields() {
		for (let i = 0; i < fields.length; i++) {
			if (
				fields[i].options.length < 2 &&
				(fields[i].type == 'selectOne' || fields[i].type == 'selectMany')
			) {
				showErrorToast('Select fields must have at least 2 options!');
				return false;
			}
			fields[i].current_inputs = [];
		}
		return true;
	}

	function save_preset_as_new() {
		if (!preset_name) {
			showErrorToast("Please enter preset's name");
		} else if (
			// @ts-ignore
			$data.presets.find((e) => e.name.toLowerCase().trim() === preset_name.toLowerCase().trim())
		) {
			showErrorToast('Preset with this name already exists!');
		} else if (!validate_and_prepare_fields()) {
			return;
		} else {
			$data.presets = [
				...$data.presets,
				{
					name: preset_name,
					fields: JSON.parse(JSON.stringify(fields)),
					last_edited: new Date().getTime(),
					status: 'unsynced',
					hue: selected_hue
				}
			];
			localStorage.setItem('presets', JSON.stringify($data.presets));
			showSuccessToast(`Preset "${preset_name}" saved!`);
		}
	}

	function update_preset() {
		if (!selected_preset) {
			console.error('selected_preset is null');
		} else if (!preset_name) {
			showErrorToast("Please enter preset's name");
		} else if (
			// @ts-ignore
			$data.presets.find(
				(e: Preset) =>
					e.name.toLowerCase().trim() === preset_name.toLowerCase().trim() && e !== selected_preset
			)
		) {
			showErrorToast('Preset with this name already exists!');
		} else if (!validate_and_prepare_fields()) {
			return;
		} else {
			let old_preset_name = selected_preset.name;
			selected_preset.fields = fields;
			selected_preset.name = preset_name;
			selected_preset.last_edited = new Date().getTime();
			selected_preset.hue = selected_hue;

			if (selected_preset.status == 'synced') {
				selected_preset.status = 'to_update';
			}
			$data.presets = $data.presets;
			localStorage.setItem('presets', JSON.stringify($data.presets));
			if (old_preset_name == preset_name) {
				showSuccessToast(`Preset "${preset_name}" updated!`);
			} else {
				showSuccessToast(`Preset "${old_preset_name}" updated as "${preset_name}"!`);
			}
		}
	}

	function sync_presets() {
		axios
			.post(
				$data.backend_url + '/sync_presets',
				JSON.stringify([
					$data.presets.filter((e: Preset) => e.status == 'unsynced'),
					$data.presets.filter((e: Preset) => e.status == 'to_update'),
					$data.ids_of_presets_to_remove || []
				]),
				{
					headers: {
						Authorization: `Bearer ${$data.jwt}`,
						'Content-Type': 'application/json'
					}
				}
			)
			.then((response) => {
				console.log(response);
				if (response.status === 200) {
					console.log(response.data[0]);
					let sync_report = response.data[0];
					if (sync_report.ignored_presets.length > 0) {
						showWarningToast(
							`Some presets were later changed on different device!<br/>${sync_report.ignored_presets.join(
								'<br/>'
							)}`
						);
					}
					if (sync_report.unfound_presets.length > 0) {
						showWarningToast(
							`You had changes to already-deleted presets!<br/>${sync_report.unfound_presets.join(
								'<br/>'
							)}`
						);
					}
					if (sync_report.ignored_presets.length == 0 && sync_report.unfound_presets.length == 0) {
						showSuccessToast('Presets synced!');
					}
					$data.presets = response.data[1];
					localStorage.setItem('presets', JSON.stringify($data.presets));
					$data.ids_of_presets_to_remove = [];
					localStorage.setItem('ids_of_presets_to_remove', JSON.stringify([]));
					fields = JSON.parse(JSON.stringify(default_fields));
					selected_preset = null;
				}
			})
			.catch((error) => {
				showErrorToast('Presets sync failed!');
			});
	}

	function add_missing_bindings(field: Field) {
		if (!field.bindings || !fields) {
			return;
		}
		let binding_field = null;
		for (let i = 0; i < fields?.length; i++) {
			if (field.bound_to == fields[i].id) {
				binding_field = fields[i];
				break;
			}
		}
		if (!binding_field) {
			return;
		}
		let to_check: string[] = [];
		console.log(binding_field.type);
		switch (binding_field.type) {
			case 'text':
				to_check = [binding_field.default[0]];
				break;
			case 'selectOne': // fall-through for both selectOne and selectMany
			case 'selectMany':
				to_check = binding_field.options;
				console.log(binding_field.options);
				break;
			case 'bound':
				// @ts-ignore
				to_check = binding_field?.bindings.map((e) => e[1]);
				break;
			default:
				to_check = [];
				break;
		}
		loop_outer: for (let i = 0; i < to_check.length; i++) {
			for (let j = 0; j < field.bindings.length; j++) {
				if (field.bindings[j][0] == to_check[i]) {
					console.log(to_check[i]);
					continue loop_outer;
				}
			}
			field.bindings.push([to_check[i], '']);
		}
		fields = fields;
	}
</script>

<h2 class="h2 mt-12">Create a card preset</h2>
<div class="card p-2 ml-6 mr-6">
	{#each $data.presets as preset}
		<button
			style={`color: hsl(${preset.hue} ${
				selected_preset?.name == preset.name
					? '100% 20%); background-color: hsl(' + preset.hue + ' 100% 87%);'
					: '70% 50%);'
			}`}
			class={`btn ${
				selected_preset?.name == preset.name ? 'variant-filled' : 'variant-ghost'
			} m-0.5`}
			on:click={() => {
				preset_name = preset.name;
				fields = JSON.parse(JSON.stringify(preset.fields));
				selected_preset = preset;
				selected_hue = preset.hue;
			}}
		>
			{preset.name}
		</button>
	{/each}
	{#if $data.presets.length}
		<br />
	{/if}
	<button
		class={`btn ${!selected_preset ? 'variant-filled' : 'variant-ghost'} m-0.5`}
		on:click={() => {
			preset_name = 'new preset';
			fields = JSON.parse(JSON.stringify(default_fields));
			selected_preset = null;
		}}
	>
		<b><i>new preset</i></b>
	</button>
</div>
<div class="card p-4">
	<button
		class={`btn-icon ${
			selected_preset && $data.presets.indexOf(selected_preset) > 0
				? 'variant-filled'
				: 'variant-ghost'
		} m-0.5`}
		on:click={() => {
			if (selected_preset && $data.presets.indexOf(selected_preset) > 0) {
				// @ts-ignore
				$data.presets = $data.presets.filter((p) => p !== selected_preset);
				$data.presets.unshift(selected_preset);
				localStorage.setItem('presets', JSON.stringify($data.presets));
			}
		}}
		><i class="fa-solid fa-angles-left" />
	</button>
	<button
		class={`btn-icon ${
			selected_preset && $data.presets.indexOf(selected_preset) > 0
				? 'variant-filled'
				: 'variant-ghost'
		} m-0.5`}
		on:click={() => {
			if (selected_preset && $data.presets.indexOf(selected_preset) > 0) {
				// @ts-ignore
				let index = $data.presets.indexOf(selected_preset);
				let temp = $data.presets[index - 1];
				$data.presets[index - 1] = selected_preset;
				$data.presets[index] = temp;
				localStorage.setItem('presets', JSON.stringify($data.presets));
			}
		}}
		><i class="fa-solid fa-chevron-left" />
	</button>
	<button
		type="button"
		class={`btn-icon ${selected_preset ? 'variant-filled-primary' : 'variant-ghost-primary'} m-0.5`}
		style="font-weight: bold;"
		on:click={() => {
			if (selected_preset?.status !== 'unsynced' && selected_preset?._id) {
				$data.ids_of_presets_to_remove.push(selected_preset?._id);
				localStorage.setItem(
					'ids_of_presets_to_remove',
					JSON.stringify($data.ids_of_presets_to_remove)
				);
			}
			// @ts-ignore
			$data.presets = $data.presets.filter((p) => p !== selected_preset);
			selected_preset = null;
			localStorage.setItem('presets', JSON.stringify($data.presets));
		}}
	>
		<i class="fa-solid fa-remove" />
	</button>
	<button
		class={`btn-icon ${
			selected_preset && $data.presets.indexOf(selected_preset) < $data.presets.length - 1
				? 'variant-filled'
				: 'variant-ghost'
		} m-0.5`}
		on:click={() => {
			if (selected_preset && $data.presets.indexOf(selected_preset) < $data.presets.length - 1) {
				// @ts-ignore
				let index = $data.presets.indexOf(selected_preset);
				let temp = $data.presets[index + 1];
				$data.presets[index + 1] = selected_preset;
				$data.presets[index] = temp;
				localStorage.setItem('presets', JSON.stringify($data.presets));
			}
		}}
		><i class="fa-solid fa-chevron-right" />
	</button>
	<button
		class={`btn-icon ${
			selected_preset && $data.presets.indexOf(selected_preset) < $data.presets.length - 1
				? 'variant-filled'
				: 'variant-ghost'
		} m-0.5`}
		on:click={() => {
			if (selected_preset && $data.presets.indexOf(selected_preset) < $data.presets.length - 1) {
				// @ts-ignore
				$data.presets = $data.presets.filter((p) => p !== selected_preset);
				$data.presets.push(selected_preset);
				localStorage.setItem('presets', JSON.stringify($data.presets));
			}
		}}
		><i class="fa-solid fa-angles-right" />
	</button>
</div>
<button class="btn variant-filled-success" on:click={sync_presets}>sync presets</button>
<div>
	<div class="mb-4">
		{#if selected_preset}
			based on: {selected_preset.name}
		{:else}
			creating preset from scratch
		{/if}
	</div>
	<span style="font-weight: bold"> preset name: </span>
	<input type="text" bind:value={preset_name} />

	<div
		style={`display: grid; grid-template-columns: 4.004rem repeat(16, 1fr); color: hsl(${selected_hue} 50% 50%)`}
		class="mt-2"
	>
		color:
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<!-- svelte-ignore a11y-no-static-element-interactions -->
		<div
			on:click={() => {
				selected_hue = '';
			}}
			style={`background-color: white; width: 100%; height: 1.5444rem;`}
		/>
		{#each Array.from(new Array(15), (_, i) => i * 24) as hue}
			<!-- svelte-ignore a11y-click-events-have-key-events -->
			<!-- svelte-ignore a11y-no-static-element-interactions -->
			<div
				on:click={() => {
					selected_hue = hue.toString();
				}}
				style={`background-color: hsl(${hue} 50% 50%); width: 100%; height: 1.5444rem;`}
			/>
		{/each}
	</div>
	{#each fields as field, i_field}
		<div
			class="card p-4"
			style="display: grid; grid-template-columns: 10.582rem 22.88rem 2.574rem 2.574rem 1fr 2.574rem; gap: 0.5148rem; margin-top: 0.572rem;"
		>
			<input type="text" style="grid-column: 1; margin-right: 0.286rem;" bind:value={field.name} />
			<RadioGroup>
				<RadioItem
					bind:group={field.type}
					on:click={() => {
						field.default = [field.default[0] ?? ''];
					}}
					name="type"
					value="text">text</RadioItem
				>
				<RadioItem
					bind:group={field.type}
					on:click={() => {
						if (!field.options.includes(field.default[0])) {
							field.options = [...field.options, field.default[0] ?? ''];
						}
						field.default = [field.default[0] ?? ''];
					}}
					name="type"
					value="selectOne">single</RadioItem
				>
				<RadioItem
					bind:group={field.type}
					on:click={() => {
						console.log(field.options);
						field.options = field.options.filter((e) => e);
						if (field.default[0]?.length > 0 && !field.options.includes(field.default[0])) {
							field.options = [...field.options, field.default[0]];
						}
						field.default = field.default.filter((e) => e);
					}}
					name="type"
					value="selectMany">multiple</RadioItem
				>
				<RadioItem
					bind:group={field.type}
					on:click={() => {
						field.bindings = [
							['nl', 'Dutch'],
							['en', 'English']
						];
						// console.log(field.options);
						// field.options = field.options.filter((e) => e);
						// if (field.default[0]?.length > 0 && !field.options.includes(field.default[0])) {
						// 	field.options = [...field.options, field.default[0]];
						// }
						// field.default = field.default.filter((e) => e);
					}}
					name="type"
					value="bound">bound</RadioItem
				>
			</RadioGroup>
			<button
				on:click={() => {
					field.expanded_in_editor = !field.expanded_in_editor;
				}}
				class="btn btn-sm {field.expanded_in_editor ? 'variant-filled' : 'variant-ghost'}"
			>
				{#if field.expanded_in_editor}
					<i class="fa-solid fa-chevron-up" />
				{:else}
					<i class="fa-solid fa-chevron-down" />
				{/if}
			</button>
			<button
				on:click={() => {
					field.visible_by_default = !field.visible_by_default;
				}}
				class="btn btn-sm {field.visible_by_default ? 'variant-filled' : 'variant-ghost'}"
			>
				{#if field.visible_by_default}
					<i class="fa-solid fa-eye" />
				{:else}
					<i class="fa-solid fa-eye-slash" />
				{/if}
			</button>
			<div style="display: flex; direction: row;">
				<button
					style="border-top-right-radius: 0; border-bottom-right-radius: 0; margin-bottom: 0.286rem;"
					class="btn btn-sm {i_field == 0 ? 'variant-ghost-secondary' : 'variant-filled-secondary'}"
					disabled={i_field == 0}
					on:click={() => {
						let temp = fields[i_field - 1];
						fields[i_field - 1] = field;
						fields[i_field] = temp;
					}}><i class="fa-solid fa-arrow-up" /></button
				>
				<button
					style="border-top-left-radius: 0; border-bottom-left-radius: 0; margin-top: 0.286rem;"
					class="btn btn-sm {i_field == fields.length - 1
						? 'variant-ghost-secondary'
						: 'variant-filled-secondary'}"
					disabled={i_field == fields.length - 1}
					on:click={() => {
						let temp = fields[i_field + 1];
						fields[i_field + 1] = field;
						fields[i_field] = temp;
					}}><i class="fa-solid fa-arrow-down" /></button
				>
			</div>
			<button
				style="font-weight: bold;"
				class="btn btn-sm variant-filled-primary"
				on:click={() => {
					fields = fields.filter((e) => e.id != field.id);
				}}
			>
				<i class="fa-solid fa-remove" /></button
			>
			{#if field.expanded_in_editor}
				{#if field.type === 'text'}
					<div style="grid-column: 2;">
						default: <input
							class="mt-2"
							style="width: calc(100% - 9ch);"
							type="text"
							bind:value={field.default[0]}
						/>
					</div>
				{:else if field.type === 'selectOne'}
					<div style="grid-column-start: 2; grid-column-end: 4">
						<!-- <div>
							options: {field.options.join(', ')}
						</div>
						<div style="margin-bottom: 0.572rem;">
							default: {field.default[0]}
						</div> -->
						<ListBox>
							<div
								class="mt-2"
								style="display: grid; grid-template-columns: 1fr 2.574rem; row-gap: 0.286rem; column-gap: 0.5148rem;"
							>
								{#each field.options as option, i_option}
									<ListBoxItem bind:group={field.default[0]} name="option" value={option}
										>{option || '(empty)'}</ListBoxItem
									>
									<button
										type="button"
										class="btn btn-sm variant-filled-warning"
										style="font-weight: bold;"
										on:click={() => {
											field.options.splice(i_option, 1);
											field.options = field.options;
											if (field.default[0] == option) {
												field.default = [];
											}
										}}
									>
										<i class="fa-solid fa-remove" />
									</button>
								{/each}
								<input type="text" bind:value={field.current_inputs[0]} />
								<button
									type="button"
									class="btn btn-sm variant-filled-success"
									on:click={() => {
										if (field.options.includes(field.current_inputs[0])) {
											showErrorToast('Value already exists!');
											return;
										}
										field.options.push(field.current_inputs[0] || '');
										field.current_inputs = [''];
									}}
								>
									<i class="fa-solid fa-plus" />
								</button>
							</div>
						</ListBox>
					</div>
				{:else if field.type === 'selectMany'}
					<div style="grid-column-start: 2; grid-column-end: 4">
						<!-- <div>
							options: {field.options.join(', ')}
						</div>
						<div style="margin-bottom: 0.572rem;">
							default: {field.default.join(', ')}
						</div> -->
						<ListBox multiple>
							<div
								class="mt-2"
								style="display: grid; grid-template-columns: 1fr 2.574rem; row-gap: 0.286rem; column-gap: 0.5148rem;"
							>
								{#each field.options as option, i_option}
									<ListBoxItem bind:group={field.default} name="option" value={option}
										>{option || '(empty)'}</ListBoxItem
									>
									<button
										type="button"
										class="btn btn-sm variant-filled-warning"
										style="font-weight: bold;"
										on:click={() => {
											field.options.splice(i_option, 1);
											field.options = field.options;
											if (field.default.includes(option)) {
												field.default = field.default.filter((e) => e != option);
											}
										}}
									>
										<i class="fa-solid fa-remove" />
									</button>
								{/each}
								<input type="text" bind:value={field.current_inputs[0]} />
								<button
									type="button"
									class="btn btn-sm variant-filled-success"
									on:click={() => {
										if (!field.current_inputs[0]?.length) {
											showErrorToast("Value of 'select many' must not be empty!");
											return;
										}
										if (field.options.includes(field.current_inputs[0])) {
											showErrorToast('Value already exists!');
											return;
										}
										field.options.push(field.current_inputs[0] || '');
										field.current_inputs = [''];
									}}
								>
									<i class="fa-solid fa-plus" />
								</button>
							</div>
						</ListBox>
					</div>
				{:else if field.type === 'bound'}
					<div style="grid-column: 2;">
						default: <input
							class="mt-2"
							style="width: calc(100% - 3.85rem);"
							type="text"
							bind:value={field.default[0]}
						/>
					</div>
					<div style="grid-column: 2;">
						bound to:
						<div class="card p-2 pr-2">
							{#each fields.filter((e) => e != field) as field_to_bind}
								<button
									class={`btn ${
										field_to_bind.id == field?.bound_to ? 'variant-filled' : 'variant-ghost'
									} m-0.5`}
									on:click={() => {
										field.bound_to = field_to_bind.id;
									}}
								>
									{field_to_bind.name}
								</button>
							{/each}
						</div>
					</div>
					<div style="grid-column-start: 2; grid-column-end: 4;">
						<div
							class="mt-2"
							style="display: grid; grid-template-columns: 1fr 3ch 1fr 2.574rem; row-gap: 0.286rem; column-gap: 0.5148rem;"
						>
							{#each field.bindings || [] as binding, i_binding}
								<div>
									<input style="width:100%" type="text" bind:value={binding[0]} />
								</div>
								<div>
									{'->'}
								</div>
								<div>
									<input style="width:100%" type="text" bind:value={binding[1]} />
								</div>
								<button
									type="button"
									class="btn btn-sm variant-filled-warning"
									style="font-weight: bold;"
									on:click={() => {
										field?.bindings?.splice(i_binding, 1);
										field.bindings = field.bindings;
									}}
								>
									<i class="fa-solid fa-remove" />
								</button>
							{/each}
							<button
								style="grid-column: 1; height: 2.574rem;"
								type="button"
								class="btn btn-sm variant-filled-success"
								on:click={() => {
									add_missing_bindings(field);
								}}
							>
								add missing
							</button>
							<button
								style="grid-column: 4; height: 2.574rem;"
								type="button"
								class="btn btn-sm variant-filled-success"
								on:click={() => {
									field.bindings = [...(field.bindings || []), ['', '']];
								}}
							>
								<i class="fa-solid fa-plus" />
							</button>
						</div>
					</div>
				{/if}
			{/if}
		</div>
	{/each}

	<div
		class="card p-4"
		style="display: grid; grid-template-columns: 10.582rem 22.88rem 2.574rem 2.574rem 4.004rem 2.574rem; gap: 0.572rem; margin-top: 0.858rem;"
	>
		<input
			type="text"
			style="grid-column: 1; margin-right: 0.286rem;"
			bind:value={new_field_name}
		/>
		<RadioGroup>
			<RadioItem bind:group={new_field_type} name="type" value="text">text</RadioItem>
			<RadioItem bind:group={new_field_type} name="type" value="selectOne">single</RadioItem>
			<RadioItem bind:group={new_field_type} name="type" value="selectMany">multiple</RadioItem>
			<RadioItem bind:group={new_field_type} name="type" value="bound">bound</RadioItem>
		</RadioGroup>
		<button
			style="grid-column: 3/7;"
			class="btn btn-sm variant-filled-success"
			on:click={() => {
				create_field();
			}}>add field</button
		>
	</div>
	{#if selected_preset}
		<button
			style="margin-top: 0.858rem;"
			class="btn btn-large variant-filled-warning"
			on:click={update_preset}>update {selected_preset.name}</button
		>
		<button
			style="margin-top: 0.858rem;"
			class="btn btn-large variant-filled-success"
			on:click={save_preset_as_new}>save as new</button
		>
	{:else}
		<button
			style="margin-top: 0.858rem;"
			class="btn btn-large variant-filled-success"
			on:click={save_preset_as_new}>save preset</button
		>
	{/if}
</div>

<style lang="postcss">
	input {
		color: black;
	}
</style>

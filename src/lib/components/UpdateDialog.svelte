<script lang="ts">
    import { onMount } from "svelte";
    import * as Dialog from "$lib/components/ui/dialog";
    import Button from "$lib/components/ui/button/button.svelte";
    import { Download, CircleAlert } from "@lucide/svelte";
    import {
        onUpdateStateChange,
        downloadAndInstall,
        dismissUpdate,
        type UpdateState,
    } from "$lib/updater";

    let open = $state(false);
    let updateState: UpdateState = $state({
        checking: false,
        available: false,
        downloading: false,
        progress: 0,
        update: null,
        error: null,
    });

    onMount(() => {
        onUpdateStateChange((state) => {
            updateState = state;
            if (state.available && state.update) {
                open = true;
            }
        });
    });

    async function handleInstall() {
        if (!updateState.update) return;

        try {
            await downloadAndInstall(updateState.update);
        } catch (error) {
            console.error("Update installation failed:", error);
        }
    }

    function handleDismiss() {
        dismissUpdate();
        open = false;
    }
</script>

<Dialog.Root bind:open>
    <Dialog.Portal>
        <Dialog.Overlay />
        <Dialog.Content class="sm:max-w-[500px]">
            <Dialog.Header>
                <Dialog.Title>
                    {#if updateState.error}
                        Update Error
                    {:else if updateState.downloading}
                        Installing Update
                    {:else}
                        Update Available
                    {/if}
                </Dialog.Title>
                <Dialog.Description>
                    {#if updateState.error}
                        An error occurred while updating the application.
                    {:else if updateState.downloading}
                        Downloading and installing version {updateState.update
                            ?.version}. Please wait...
                    {:else if updateState.update}
                        Version {updateState.update.version} is now available. You
                        are currently using version {updateState.update
                            .currentVersion}.
                    {/if}
                </Dialog.Description>
            </Dialog.Header>

            <div class="py-4">
                {#if updateState.error}
                    <div
                        class="flex items-start gap-3 rounded-lg border border-destructive/50 bg-destructive/10 p-4"
                    >
                        <CircleAlert class="size-5 text-destructive mt-0.5" />
                        <div class="flex-1">
                            <p class="text-sm font-medium text-destructive">
                                Failed to update
                            </p>
                            <p class="text-sm text-muted-foreground mt-1">
                                {updateState.error}
                            </p>
                        </div>
                    </div>
                {:else if updateState.downloading}
                    <div class="space-y-3">
                        <div
                            class="w-full bg-secondary rounded-full h-2.5 overflow-hidden"
                        >
                            <div
                                class="bg-primary h-2.5 transition-all duration-300 ease-in-out"
                                style="width: {updateState.progress}%"
                            ></div>
                        </div>
                        <p class="text-sm text-center text-muted-foreground">
                            {updateState.progress}% complete
                        </p>
                    </div>
                {:else if updateState.update}
                    <div class="space-y-3">
                        {#if updateState.update.date}
                            <div class="text-sm">
                                <span class="font-medium">Release Date:</span>
                                <span class="text-muted-foreground ml-2">
                                    {new Date(
                                        updateState.update.date,
                                    ).toLocaleDateString()}
                                </span>
                            </div>
                        {/if}

                        {#if updateState.update.body}
                            <div class="space-y-2">
                                <p class="text-sm font-medium">
                                    Release Notes:
                                </p>
                                <div
                                    class="rounded-lg border bg-muted/50 p-3 text-sm text-muted-foreground max-h-[200px] overflow-y-auto"
                                >
                                    <pre
                                        class="whitespace-pre-wrap font-sans">{updateState
                                            .update.body}</pre>
                                </div>
                            </div>
                        {/if}
                    </div>
                {/if}
            </div>

            <Dialog.Footer>
                {#if updateState.error}
                    <Button onclick={handleDismiss}>Close</Button>
                {:else if updateState.downloading}
                    <Button disabled>
                        <Download class="animate-bounce" />
                        Installing...
                    </Button>
                {:else}
                    <Button variant="outline" onclick={handleDismiss}
                        >Later</Button
                    >
                    <Button onclick={handleInstall}>
                        <Download />
                        Update Now
                    </Button>
                {/if}
            </Dialog.Footer>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>

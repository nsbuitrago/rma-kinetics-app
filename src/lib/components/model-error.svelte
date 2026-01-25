<script lang="ts">
    import * as AlertDialog from "$lib/components/ui/alert-dialog/index.js";

    interface Props {
        open: boolean;
        errorMessage: string;
    }

    let { open = $bindable(), errorMessage }: Props = $props();

    function handleDismiss() {
        open = false;
    }
</script>

<AlertDialog.Root bind:open>
    <AlertDialog.Content>
        <AlertDialog.Header>
            <AlertDialog.Title>Simulation Error</AlertDialog.Title>
            <AlertDialog.Description>
                {errorMessage}
            </AlertDialog.Description>
        </AlertDialog.Header>
        <AlertDialog.Footer>
            <AlertDialog.Action onclick={handleDismiss}>Dismiss</AlertDialog.Action>
        </AlertDialog.Footer>
    </AlertDialog.Content>
</AlertDialog.Root>

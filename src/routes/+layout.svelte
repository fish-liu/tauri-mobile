<script lang="ts">
    import "../app.css";
    import { chatType } from "$lib/store";
    import Chat from '$lib/Chat.svelte'
    import MyChatType from '$lib/ChatType.svelte';
            
</script>

<div>
    {#if $chatType == ''  }
    <MyChatType/>
    {/if}

    {#if $chatType != ''}
    <Chat />
    {/if}
</div>
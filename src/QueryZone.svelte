<div class="editor-area">
    <textarea name="" id="" cols="30" rows="10" style="resize: none;" bind:value={ $textContent }></textarea>
    <!-- <button on:click="{select}">botao</button> -->
</div>

<script context="module">
    import { writable, readable } from 'svelte/store';
    export const textContent = writable('select * from person');
</script>

<script lang="ts">
    import { EditorState } from "@codemirror/state"
    import { EditorView, keymap } from "@codemirror/view"
    import { defaultKeymap } from "@codemirror/commands"
    import { onMount } from 'svelte';

/*
    let startState = EditorState.create({
        doc: "Hello World",
        extensions: [keymap.of(defaultKeymap)]
    })

    var editor: EditorView;
    onMount(() => {
        editor = new EditorView({
            state: startState,
            parent: document.getElementById("codeeditor")
        })
    })
    let target = document.querySelector("#codeeditor")
*/
</script>

<style>
</style>

<div class="editor-area">
    <!-- <textarea name="editor" id="" cols="30" rows="10"></textarea> -->
    <div id="codeeditor">
    </div>
    <textarea name="" id="" cols="30" rows="10" style="resize: none;"></textarea>
    <button on:click="{select}">botao</button>
</div>

<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
    import {EditorState} from "@codemirror/state"
    import {EditorView, keymap} from "@codemirror/view"
    import {defaultKeymap} from "@codemirror/commands"
    import { onMount } from 'svelte';

    let startState = EditorState.create({
        doc: "Hello World",
        extensions: [keymap.of(defaultKeymap)]
    })

    var editor: EditorView;
    onMount(() => {
        editor = new EditorView({
            state: startState,
            parent: document.getElementById("codeeditor")//document.querySelector(".editor-area")
        })
    })
    let target = document.querySelector("#codeeditor")

    function select() {
        console.log(editor)
        let query = document.querySelector('textarea').value;
		invoke('select', {query: query}).then(function (res: string) {
			console.log(res)
		})
	}
</script>
<style>
    .editor-area {
        width: 100%;
        height: 100%;
        justify-content: center;
        align-items: center;
    }
</style>

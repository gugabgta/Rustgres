<div class="editor-area">
    <div id="input"></div>
</div>

<script context="module">
    import { writable, readable } from 'svelte/store';
    export const textContent = writable('select * from person');
</script>

<script lang="ts">
    import { EditorState, Compartment} from "@codemirror/state"
    import { EditorView, keymap, gutter, lineNumbers, highlightSpecialChars } from "@codemirror/view"
    import { defaultKeymap } from "@codemirror/commands"
    import { onMount } from 'svelte';
    import { defaultHighlightStyle, syntaxHighlighting } from "@codemirror/language"
    import { PostgreSQL, sql } from "@codemirror/lang-sql"
    import { javascript } from "@codemirror/lang-javascript"
    import { basicSetup } from "codemirror"

    EditorView.lineWrapping
    let config_lang = sql({dialect: PostgreSQL})

    let myTheme = EditorView.theme({
        "&": {
            color: "white",
            backgroundColor: "#034",
            height: "400px",
            width: "200%",
        },
        ".cm-content": {
            caretColor: "#0e9",
            minHeight: "200px",
        },
        "&.cm-focused .cm-cursor": {
            borderLeftColor: "#0e9"
        },
        "&.cm-focused .cm-selectionBackground, ::selection": {
            backgroundColor: "#f80"
        },
        ".cm-gutters": {
            backgroundColor: "#045",
            color: "#ddd",
            border: "none",
            minHeight: "200px",
        },
        ".cm-scroller": {
            overflow: "auto",
        },
        ".cm-editor": {
            // width: "900px",
        }
    }, {dark: true})

    onMount(() =>
    {
        let startState = EditorState.create({
            doc: "SELECT * FROM person WHERE name = 'gustavo'",
            extensions: [
                keymap.of(defaultKeymap),
                lineNumbers(),
                gutter({class: "cm-mygutter"}),
                myTheme,
                EditorView.updateListener.of(function(e) {
                    $textContent = e.state.doc.toString();
                    // console.log($textContent)
                }),
                highlightSpecialChars(),
                syntaxHighlighting(defaultHighlightStyle, {fallback: true}),
                config_lang,
            ],
        })

        let view = new EditorView({
            state: startState,
            parent: document.querySelector('#input'),
        })
    })
</script>

<style>
    #input {
        height: 40%;
        max-height: 50%;
        min-width: 600px;
        max-width: 80%;
        overflow-x: auto;
        overflow-y: hidden;
    }
</style>

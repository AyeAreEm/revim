<script>
    import { invoke } from '@tauri-apps/api';
    import { save } from '@tauri-apps/api/dialog';
    import { open } from '@tauri-apps/api/dialog';
    import { readTextFile } from '@tauri-apps/api/fs';
    import hljs from 'highlight.js/lib/common';
    import cpp from 'highlight.js/lib/languages/cpp';
    import javascript from 'highlight.js/lib/languages/javascript';
    import xml from 'highlight.js/lib/languages/xml';
    import css from 'highlight.js/lib/languages/css';

    hljs.registerLanguage('cpp', cpp);
    hljs.registerLanguage('javascript', javascript);
    hljs.registerLanguage('css', css);
    hljs.registerLanguage('xml', xml);

    export let value;
    export let code;
    export let lang;
    export let mode;
    export let commandRef = null;

    let command = "";
    let fileName = "untitled";


    const getLanguage = (extension) => {
        switch (extension) {
            case "css":
            case "xml":
            case "cpp":
                return extension;
            
            case "js":
                return "javascript";

            case "html":
                return "xml"
            
            default:
                break;
        }
    }


    const saveFile = async () => {
        try {
            if (fileName == "untitled") {
                const savePath = await save();
                fileName = savePath;
                if (!savePath) return;
                await invoke("save_file", {path: savePath, contents: value.innerText})
            } else {
                await invoke("save_file", {path: fileName, contents: value.innerText})
            }
            value.focus();
        } catch (err) {
            console.log(err)
        }
    }

    const readFileContents = async () => {
        try {
            const selectedPath = await open({
                multiple: false,
                title: "Open File"
            });
            if (!selectedPath) return;
            // @ts-ignore
            let extension = selectedPath.substring(selectedPath.indexOf('.') + 1);
            lang = getLanguage(extension);
            // @ts-ignore
            value.innerText = await readTextFile(selectedPath);
            code.innerHTML = hljs.highlight(value.innerText, {language: lang}).value
            value.focus();

            // @ts-ignore
            fileName = selectedPath;
        } catch (err) {
            console.log(err)
        }
    }

    const handleCommand = async (e) => {
        if (e.keyCode === 13 && command === ":s") {
            saveFile()
            command = "";
            mode = "I"
        } else if (e.keyCode === 13 && command === ":sf") {
            readFileContents()
            command = "";
            mode = "I"
        } else if (e.keyCode === 13 && command.includes("/")) {
            let word = command.substring(1);
            console.log(word);
        } else if (e.keyCode === 13 && command === ":help") {
            await invoke("open_docs")
        }else if (e.keyCode === 13 && command === "") {
            value.focus();
            mode = "I";
        }
    }
</script>

<div class="commandCentre">
    <div class="infos">
      <p class="mode" style={mode == "I" ? "background-color: #3b8fa3;" : "background-color: #588d62;"}>{mode}</p>
      <p>{fileName}</p>
    </div>
    <input bind:value={command} bind:this={commandRef} type="text" on:keydown={handleCommand} />
</div>
<script>
  import { invoke } from '@tauri-apps/api';
  import { save } from '@tauri-apps/api/dialog';
  import { open } from '@tauri-apps/api/dialog';
  import { readTextFile } from '@tauri-apps/api/fs';
  import { onMount } from "svelte";

  let value;
  let command;
  let fileName = "untitled";

  onMount(() => {
    value.focus()
  })

  let mode = "I"; // N, I, C

  const saveFile = async() => {
    try {
      const savePath = await save();
      fileName = savePath;
      if (!savePath) return;
      await invoke("save_file", {path: savePath, contents: value.textContent})
    } catch (err) {
      console.log(err)
    }
  }

  const readFileContents = async() => {
    try {
      const selectedPath = await open({
        multiple: false,
        title: "Open File"
      });
      if (!selectedPath) return;
      // @ts-ignore
      value.textContent = await readTextFile(selectedPath);
      // @ts-ignore
      fileName = selectedPath;
    } catch (err) {
      console.log(err)
    }
  }

  const handleCommand = (e) => {
    if (e.keyCode === 13 && command.value === ":s") {
      saveFile()
      command.value = "";
      value.focus();
      mode = "N"
    } else if (e.keyCode === 13 && command.value === ":sf") {
      readFileContents()
      command.value = "";
      value.focus();
      mode = "N"
    }
  }

  const handleKeysPrev = (e) => {
    e.preventDefault();

    if (e.keyCode === 13 && mode == "I") { // not needed, cant be I
      console.log(value.textContent)
    } else if (e.keyCode === 13 && mode == "N") { // remove check for N, it must be N 
      console.log(value.textContent)
    } else if (e.keyCode === 27 && mode != "N") { // not needed, it must be in N
      console.log("N")
      mode = "N";
      value.focus()
    } else if (e.keyCode === 73 && mode != "I") { // remove check for I 
      console.log("I")
      mode = "I";
      value.focus()
    } else if (e.key === ":" && (mode == "N" || mode == "C")) { // remove including and after &&
      mode = "C";
      command.value = ":"
      command.focus();
    }
  }

  const handleKeys = (e) => {
    if (e.keyCode === 13 && mode == "I") {
      console.log(value.textContent)
    } else if (e.keyCode === 13 && mode == "N") {
      console.log(value.textContent)
    } else if (e.keyCode === 27 && mode != "N") {
      console.log("N")
      mode = "N";
      value.focus()
    } else if (e.keyCode === 73 && mode != "I") {
      console.log("I")
      mode = "I";
      value.focus()
    } else if (e.key === ":" && (mode == "N" || mode == "C")) {
      mode = "C";
      command.value = ":"
      command.focus();
    }
  }

</script>

<div class="editor">
  <div bind:this={value} class="editText" spellcheck="false" contenteditable="true" on:keydown={mode == "N" ? handleKeysPrev : handleKeys}></div>

  <div class="commandCentre">
    <div class="infos">
      <p class="mode" style={mode == "I" ? "background-color: #3b8fa3;" : "background-color: #588d62;"}>{mode}</p>
      <p >{fileName}</p>
    </div>
    <input bind:this={command} type="text" on:keydown={handleCommand}/>
  </div>
</div>

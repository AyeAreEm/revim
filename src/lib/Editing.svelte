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

  let mode = "I"; // I, C

  const saveFile = async() => {
    try {
      const savePath = await save();
      fileName = savePath;
      if (!savePath) return;
      await invoke("save_file", {path: savePath, contents: value.innerText})
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
      value.innerText = await readTextFile(selectedPath);
      // @ts-ignore
      fileName = selectedPath;
    } catch (err) {
      console.log(err)
    }
  }

  const setCaret = (direction) => {
    let sel = window.getSelection();
    let offset = sel.focusOffset;
    let focus = sel.focusNode;
    let range = document.createRange();

    switch (direction) {
      case "left":
        range.selectNode(focus);
        range.setStart(focus, offset - 1);
        range.collapse(true);
        sel.removeAllRanges();
        sel.addRange(range);
        break;

      case "right":
        range.selectNode(focus);
        range.setStart(focus, offset + 1);
        range.collapse(true);
        sel.removeAllRanges();
        sel.addRange(range);
        break;

      // MAYBE: try to figure out the line numbers so that you can get a 
      case "up":
        range = sel.getRangeAt(0);
        let rect = range.getClientRects()[0];
        range = document.createRange();

        // fix issue with top - Uncaught TypeError: Cannot read properties of undefined (reading 'top') NOTE: it works when there is a keystroke in between
        let top = rect.top > 0 ? (rect.top / 20) - 1 : 0;
        console.log(top)
        range.selectNode(focus);
        range.setStart(value.childNodes[top], 0);
        range.collapse(true);
        sel.removeAllRanges();
        sel.addRange(range);

        console.log("up")
        break;

      case "down":
        console.log("down")
        break;
    
      default:
        break;
    }

}

  const goToTop = () => {
    let sel = window.getSelection();
    let focus = sel.focusNode;
    console.log(focus)

    let range = document.createRange();
    range.selectNode(focus);

    range.setStart(value.childNodes[0], 0);
    
    range.collapse(true);
    sel.removeAllRanges();
    sel.addRange(range);
  }

  const goToBottom = () => {
    let sel = window.getSelection();
    let focus = sel.focusNode;
    console.log(focus)

    let range = document.createRange();
    range.selectNode(focus);

    range.setStart(value.lastChild, 0);
    
    range.collapse(true);
    sel.removeAllRanges();
    sel.addRange(range);
  }

  const handleCommand = (e) => {
    if (e.keyCode === 13 && command.value === ":s") {
      saveFile()
      command.value = "";
      value.focus();
      mode = "I"
    } else if (e.keyCode === 13 && command.value === ":sf") {
      readFileContents()
      command.value = "";
      value.focus();
      mode = "I"
    } else if (e.keyCode === 13 && command.value === "") {
      value.focus();
      mode = "I";
    }
  }

  // const handleKeysPrev = (e) => {
  //   if (e.keyCode === 73) { 
  //     console.log("I")
  //     mode = "I";
  //     value.focus()
  //   } else if (e.key === ":") {
  //     mode = "C";
  //     command.value = ":"
  //     command.focus();
  //   } else if (e.key === "s" && e.ctrlKey) {
  //     console.log("go right")
  //   }
  // }

  const handleKeys = (e) => {
    if (e.keyCode === 13) {
      // '~([\'"])[^\'"]*\1(*SKIP)(*F)|<[^>]*>~'
      console.log(value.innerText)
    } else if (e.key === ":" && e.ctrlKey) {
      mode = "C";
      command.value = ":"
      command.focus();
    } else if (e.key === "j" && e.ctrlKey) {
      setCaret("left");
    } else if (e.key === ";" && e.ctrlKey) {
      setCaret("right");
    } else if (e.key === "k" && e.ctrlKey) {
      setCaret("up");
    } else if (e.key === "l" && e.ctrlKey) {
      setCaret("down");
    } else if (e.key === "t" && e.ctrlKey) {
      goToTop();
    } else if (e.key === "T" && e.ctrlKey) {
      goToBottom();
    }
  }

</script>

<div class="editor">
  <div bind:this={value} class="editText" spellcheck="false" contenteditable="true" on:keydown={handleKeys}></div>

  <div class="commandCentre">
    <div class="infos">
      <p class="mode" style={mode == "I" ? "background-color: #3b8fa3;" : "background-color: #588d62;"}>{mode}</p>
      <p>{fileName}</p>
    </div>
    <input bind:this={command} type="text" on:keydown={handleCommand}/>
  </div>
</div>

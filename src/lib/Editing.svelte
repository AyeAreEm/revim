<script>
  import { invoke } from '@tauri-apps/api';
  import { save } from '@tauri-apps/api/dialog';
  import { open } from '@tauri-apps/api/dialog';
  import { readTextFile } from '@tauri-apps/api/fs';
  import { onMount } from "svelte";
  import hljs from 'highlight.js/lib/common';
  import cpp from 'highlight.js/lib/languages/cpp';
  import javascript from 'highlight.js/lib/languages/javascript';
  import xml from 'highlight.js/lib/languages/xml';
  import css from 'highlight.js/lib/languages/css';
  import NumberLines from './NumberLines.svelte';

  hljs.registerLanguage('cpp', cpp);
  hljs.registerLanguage('javascript', javascript);
  hljs.registerLanguage('css', css);
  hljs.registerLanguage('xml', xml);

  let value;
  let command;
  let code;
  let lang = ""
  let fileName = "untitled";
  let mode = "I"; // N, I, C
  let allowed = ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "Enter", "Backspace", "Meta", "Control", "v"]
  let syncEvents = ["scroll", "paste"]

  onMount(() => {
    value.focus();
    code.innerHTML = value.innerText;
  })

  syncEvents.forEach(e => {
    window.addEventListener(e, () => {
      setTimeout(() => {
        code.scrollTop = value.scrollTop;
      }, 5)
    })
  })

  // work on this, but in the number lines svelte file
  const numLines = () => {
    let divs = value.querySelectorAll("div:not(.ignore)");

    for (let i = 0; i < divs.length; i++) {
      const element = divs[i];
      // console.log(element);
    }
  }

  const getLanguage = (extension) => {
    switch (extension) {
      case "cpp":
        return extension;
    
      case "js":
        return "javascript";
    
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
      mode = "I"
    } else if (e.keyCode === 13 && command.value === ":sf") {
      readFileContents()
      command.value = "";
      mode = "I"
    } else if (e.keyCode === 13 && command.value.includes("/")) {
      let word = command.value.substring(1);
      console.log(word);
    } else if (e.keyCode === 13 && command.value === "") {
      value.focus();
      mode = "I";
    }
  }

  const handleKeysPrev = async (e) => {

    // maybe include the number system from vim so do an action
    // multiple times - maybe make an array
    // have a check for each event to see if its a number
    // if so, add to the array
    // if isn't, iterate an action by getting all the numbers
    // in the array and add them together (left to right)
    // have a check if the array is empty and to just iterate once

    if (!allowed.includes(e.key)) {
      e.preventDefault();

      if (e.key === "j" || e.key === "J") await invoke("move_direction", {direction: "left"});
      else if (e.key === ";" || e.key === ":") await invoke("move_direction", {direction: "right"});
      else if (e.key === "k" || e.key === "K") await invoke("move_direction", {direction: "up"});
      else if (e.key === "l" || e.key === "L") await invoke("move_direction", {direction: "down"});
      else if (e.key === "t") await invoke("move_direction", {direction: "top"});
      else if (e.key === "b") await invoke("move_direction", {direction: "bottom"});
      else if (e.key === "o") await invoke("new_line");
      // else if (e.key === "O") await invoke("new_line", {direction: "up"});
      else if (e.key === "p") await invoke("paste");
      else if (e.key === "x") await invoke("backspace");
      else if (e.key === "'") {
        mode = "C"
        command.focus();
      } else if (e.key === "i") {
        mode = "I"
        value.focus();
      } 
    }
  }

  const handleKeys = async (e) => {

    setTimeout(() => {
      // formatCode(value.innerText);
      code.scrollTop = value.scrollTop

      if (lang) {
        code.innerHTML = hljs.highlight(value.innerText, {language: lang}).value
        console.log(code.innerHTML, "passed")
      } else {
        code.innerHTML = hljs.highlightAuto(value.innerText).value
        console.log(code.innerHTML, "failed")
      }
      // code = value.innerText
    }, 5);

    if (e.keyCode === 9) {
      // e.preventDefault();
      
      // let doc = value.ownerDocument.defaultView;
      // let sel = doc.getSelection();
      // let range = sel.getRangeAt(0);

      // // fix this
      // let tabNode = document.createTextNode("&#009");
      // range.insertNode(tabNode);

      // range.setStartAfter(tabNode);
      // range.setEndAfter(tabNode); 
      // sel.removeAllRanges();
      // sel.addRange(range);

      document.execCommand('insertHTML', false, '&#009');
      e.preventDefault();
      
    } else if (e.key === "Enter") {
      numLines()
    } else if (e.key === "Escape") {
      e.preventDefault();
      mode = "N"
    }
  }
</script>

<div class="editor">
  <!-- <div class="numLines ignore">1</div> -->

  <div bind:this={value} style="background: transparent; color: transparent; caret-color: white;" class="editText" spellcheck="false" contenteditable="true" on:keydown={mode == "N" ? handleKeysPrev : handleKeys}></div>
  <div bind:this={code} class="editText"></div>

  <!-- make this as a seperate component -->
  <div class="commandCentre">
    <div class="infos">
      <p class="mode" style={mode == "I" ? "background-color: #3b8fa3;" : "background-color: #588d62;"}>{mode}</p>
      <p>{fileName}</p>
    </div>
    <input bind:this={command} type="text" on:keydown={handleCommand}/>
  </div>
</div>

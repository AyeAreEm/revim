<script>
  import { invoke } from '@tauri-apps/api';
  import { onMount } from "svelte";
  import hljs from 'highlight.js/lib/common';
  import cpp from 'highlight.js/lib/languages/cpp';
  import javascript from 'highlight.js/lib/languages/javascript';
  import xml from 'highlight.js/lib/languages/xml';
  import css from 'highlight.js/lib/languages/css';
  import NumberLines from './NumberLines.svelte';
  import CommandCentre from './CommandCentre.svelte';

  hljs.registerLanguage('cpp', cpp);
  hljs.registerLanguage('javascript', javascript);
  hljs.registerLanguage('css', css);
  hljs.registerLanguage('xml', xml);

  let value;
  let code;
  let commandRef;
  let lang = ""
  let fileName = "untitled";
  let mode = "I"; // N, I, C
  let allowed = ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "Enter", "Backspace", "Meta", "Control", "v", "c"]
  let syncEvents = ["scroll", "paste"]

  onMount(() => {
    value.focus();
    code.innerHTML = value.innerText;
  })

  syncEvents.forEach(e => {
    window.addEventListener(e, () => {
      setTimeout(() => {
        code.scrollTop = value.scrollTop;
        if (lang && e == "paste") {
          code.innerHTML = hljs.highlight(value.innerText, {language: lang}).value
        } else if (!lang && e == "paste"){
          code.innerHTML = hljs.highlightAuto(value.innerText).value
        }
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

  const handleKeysPrev = async (e) => {
    setTimeout(() => {
      code.scrollTop = value.scrollTop

      if (lang) {
        code.innerHTML = hljs.highlight(value.innerText, {language: lang}).value
      } else {
        code.innerHTML = hljs.highlightAuto(value.innerText).value
      }
    }, 1);

    // maybe include the number system from vim to do an action
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
      else if (e.key === "o") await invoke("new_line", {direction: "down"});
      else if (e.key === "w") await invoke("new_line", {direction: "up"});
      else if (e.key === "p") await invoke("copy_paste", {edit: "v"});
      else if (e.key === "y") await invoke("copy_paste", {edit: "c"});
      else if (e.key === "x") await invoke("backspace");
      else if (e.key === "'") {
        mode = "C"
        commandRef.focus();
      } else if (e.key === "i") {
        mode = "I"
        value.focus();
      } 
    }
  }

  const handleKeys = async (e) => {

    setTimeout(() => {
      code.scrollTop = value.scrollTop

      if (lang) {
        code.innerHTML = hljs.highlight(value.innerText, {language: lang}).value
      } else {
        code.innerHTML = hljs.highlightAuto(value.innerText).value
      }
    }, 1);

    if (e.keyCode === 9) {
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
  <div bind:this={value} style="background: transparent; color: transparent; caret-color: white;" class="editText" spellcheck="false" contenteditable="true" on:keydown={mode == "N" ? handleKeysPrev : handleKeys}></div>
  <div bind:this={code} class="editText"></div>

  <CommandCentre value={value} code={code} lang={lang} mode={mode} bind:commandRef/>
</div>

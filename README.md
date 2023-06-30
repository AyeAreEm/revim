# Revim
Personal code editor inspired by vim with changes to suit my preferences.
I am trying my best not to rely on third party libraries and as of this moment, I'm only using tauri with svelte.

## Tech Stack
## Frontend
    Svelte (Javascript)

## Backend ?
    Tauri (Rust)

## Resources and references
As mentioned before, I am trying my hardest to not rely on third part libraries but of course, it would just be ludicrous and just become painful. But as to not feel "guilty", I am referencing all of the sources I use.

    [Enigo](https://docs.rs/enigo/latest/enigo/) (Rust)
    [Highlight.js](https://highlightjs.org/) (Javascript)

### Use of Enigo
This crate allows for native keyboard and mouse events - I need this because when in `normal` mode, no characters should be able to be entered but I still need access to the functions those keys have. Such as arrow keys for navigation, the character V to allow for pasting, etc.

```rust
let mut enigo = Enigo::new();
enigo.key_click(Key::Return); // this presses the key Return on Mac / Linux or Enter on Windows
```

### Use of Highlight.js
All good code editors need syntax highlighting. It would be super hard to write code without knowing if what you are referencing in code actually exists but MAINLY it's just incredibly boring to look at plain white text

Highlight.js provides a tokenisation system that returns back the code you gave, now with html classes with the token name as the class. This lets you create your own styling or copy over some css from already made themes. Highlight.js does provide all these themes but there was some styling that doesn't work with my editor

```js
// both of these return back highlighted / tokenised code but one lets you specific set the programming language
// while the other automatically gets the language - this can be finicky sometimes but it eventually works after
// enough code is written
hljs.highlight(value.innerText, {language: lang}).value 
hljs.highlightAuto(value.innerText).value
```
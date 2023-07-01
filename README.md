# Revim
Personal code editor inspired by vim with changes to suit my preferences.
I am trying my best not to rely on third party libraries and as of this moment, I'm only using tauri with svelte.

## Tech Stack
## Frontend
    Svelte (Javascript)

## Backend ?
    Tauri (Rust)

## Features that need to be added
    Line numbers
    AI to help with code - iframe with (phind)[https://www.phind.com/search?q=how%20to%20make%20a%20linked%20list%20in%20javascript&source=searchbox] / poe? 
    Autocompletion - at least () and "", preferably code completion as well

## Terminal
This is super bloody annoying. Rust's standard library for commands and such - `std::process::Command` works fine in must situations except for the particular one I need, changing directories. After it's open, it can change directories like any other cmd or terminal but when creating a new instance of it, the cd command just doesn't work. I believe it's a known issue but I haven't found any solutions so for now, it will default to this directories path.

```rust
Command::new("cmd")
        .spawn()
        .expect("failed");
```

## Resources and references
As mentioned before, I am trying my hardest to not rely on third party libraries but of course, it would just be ludicrous and just become painful. But as to not feel "guilty", I am referencing all of the sources I use.

[Enigo](https://docs.rs/enigo/latest/enigo/) (Rust)
[Highlight.js](https://highlightjs.org/) (Javascript)
[Open](https://docs.rs/open/latest/open/) (Rust)

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
hljs.highlight(value.innerText, {language: lang}).value;
hljs.highlightAuto(value.innerText).value;
```

### Use of Open
This one I don't really count as a big rely on since I could accomplish the same thing in a different way using Tauri's Window Builder but I will mention it because it's clean and works as expected

```rust
open::that("https://github.com/AyeAreEm/revim");
```

## Help
The keybinds, the commands and the lot of it.

### Keybinds
|key|function|window|macos|
|-|--------|---|---|
|j|left arrow|✔|✔|
|k|up arrow|✔|✔|
|l|down arrow|✔|✔|
|;|right arrow|✔|✔|
|o|enter line below |~~~|✔|
|w|enter line above|~~~|✔|
|y|copy|✔|✔|
|p|paste|✔|✔|
|t|top of page|~~~|✔|
|b|bottom of page|~~~|✔|
|x|backspace|✔|✔|
|i|insert mode|✔|✔|
|'|command mode|✔|✔|

### Commands
|command|function|window|macos|
|-|--------|---|---|
|:s|save file|✔|✔|
|:sf|search file|✔|✔|
|:help|goto help page|✔|✔|
|:term|new terminal|✔|✔|
|:phind|new phind window|✔|✔|

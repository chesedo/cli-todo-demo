---
marp: true
author: Pieter Engelbrecht
theme: uncover
paginate: skip
---


<style>
:root {
    --rust-orange: #FF4500;
    --sa-red: #EE2733;
    --sa-blue: #002395;
    --sa-green: #007A4D;
    --sa-yellow: #FFB612;
    --color-fg-default: #333333;
    font-size: 2.5em;
}

h1 {
    color: var(--rust-orange);
    font-size: 1.5em;
}

h2 {
    font-size: 1.2em;
    color: #666666;
}

/* Add some visual interest to lists */
ul li {
    margin: 0.5em 0;
}

/* Style code blocks */
code {
    background: #1E1E1E;
    color: #FFFFFF;
    padding: 0.2em 0.4em;
    border-radius: 4px;
}

/* Style command prompts differently */
code.language-bash {
    border-left: 3px solid var(--rust-orange);
}
</style>

# Crafting CLI Tools with Rust & Serde
## Interactive Workshop

---

<!-- paginate: true -->

# Workshop Goals

- Learn Serde serialization basics 🔄
- Master Rust error handling patterns ⚠️
- Build a practical CLI tool 🛠️

📱 Follow along: [github.com/chesedo/cli-todo-demo](https://github.com/chesedo/cli-todo-demo)
![bg right:30% contain](qr.svg)

---
# Workshop Flow

1. **CLI Commands** `clap derive`
2. **Task Management** `colored`
3. **Data Persistence** `serde + serde_json`
4. **Error Handling** `thiserror + anyhow`

---

# Prep

```bash
$ cargo new cli-todo
$ cd cli-todo
```

---

# Handle CLI commands

```bash
$ cargo add clap --features derive
```

Initial commands:

- Add task
- List tasks
- Mark task as completed

---

# Task manager

Some coloring would be nice...

```bash
$ cargo add colored
```

---

# Store using serde

```bash
$ cargo add serde --features derive
$ cargo add serde_json
```

---

# Ways to improve errors

```bash
$ cargo add thiserror anyhow
```

---

# Let's add a new field

... now what do we need to update?

---
# Challenges
- Add due dates
- Allowing editing tasks
- Remove / don't show completed tasks
- Fix incorrect values with a custom deserializer
- Use a TUI
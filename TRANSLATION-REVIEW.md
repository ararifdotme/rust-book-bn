# Translation review notes

Chapters 7 onwards were translated in one unattended run (requested
2026-11-19: "next all chapter translate koro ... auto mode"). Unlike chapters
0–6, the terms for these chapters were **not approved in advance**. This file
lists, per chapter, what to look at:

- **Auto-chosen terms**: added to the TRANSLATION.md glossary following the
  same conventions as chapters 0–6. Change any you disagree with in the
  glossary and in the chapter text.
- **Decisions**: chapter-specific choices that go beyond the existing rules.
- **Unsure sentences**: English on the left, the Bengali used on the right.

Every chapter is a separate commit (`bn: translate chapter N - ...`), so a
chapter can be reviewed or reverted on its own.

## Chapter 7: Packages, Crates, and Modules

**Auto-chosen terms:** module system, path, item, crate root, module tree,
submodule, parent / child / sibling, ancestor, absolute path / relative path,
encapsulate, workspace, glob operator, re-export, alias, nested path, symbolic
link, build script, static method. Everyday: ফাইল সিস্টেম, শর্টকাট, কোডবেস.

**Decisions:**
- The restaurant terms "front of house" / "back of house" stay in English with
  a Bengali gloss, because the code uses `front_of_house` / `back_of_house`.
- "Modules Cheat Sheet" became "module-এর এক নজরের সারাংশ".
- "Inline" (module code inside `{}`) became "সেখানেই (inline)".
- Listing 7-9's comments are translated; its commented-out code line
  `// meal.seasonal_fruit = …` is unchanged. The `// Absolute path` /
  `// Relative path` comments in listings 7-3/7-5/7-7 are left as they are
  (they are just the term names).

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| be able to work with scopes like a pro! | একজন দক্ষ প্রোগ্রামারের মতো scope নিয়ে কাজ করতে পারা উচিত! |
| A crate is the smallest amount of code that the Rust compiler considers at a time. | এক সময়ে Rust compiler যতটুকু কোড বিবেচনা করে, তার ক্ষুদ্রতম একক |
| The nested context in which code is written | যে স্তরবিন্যস্ত প্রেক্ষাপটে কোড লেখা হয় |
| your public API is your contract with users of your crate | আপনার public API হলো আপনার crate-এর ব্যবহারকারীদের সঙ্গে আপনার চুক্তি |
| not only are you the author, but you’re also a client! | আপনি শুধু লেখক নন, একজন ব্যবহারকারীও! |
| hosts seat customers, servers take orders and payment, and bartenders make drinks | host খদ্দেরদের বসান, পরিবেশনকারীরা অর্ডার আর টাকা নেন, আর bartender পানীয় বানান |
| `mod` is not an “include” operation | `mod` এমন কোনো “include” operation নয় |

## Chapter 8: Common Collections

**Auto-chosen terms:** map, hashing function, hasher, iterate, wrapper,
associated type, coerce, grapheme cluster, constant time, profile
(performance), byte slice, buffer overflow, null-terminated, predicate.
Everyday: জোড়া লাগানো (concatenate), কৌণিক বন্ধনী, সিরিলিক / দেবনাগরী.

**Decisions:**
- *profile* (performance) gets a different gloss from chapter 1's *profile*
  (build settings): "কোডের কোন অংশে কত সময় লাগে তা মাপা".
- In Ownership Inventory #2 the options that read like compiler messages
  ("`v.remove(i)` cannot borrow `v` as mutable", "cannot borrow `v` as mutable
  twice …") stay in English, following the chapter 6 rule.
- The greeting strings in Listing 8-14 and the Hindi example "नमस्ते" are
  unchanged (code / example data).
- Listing comments translated: 8-10, 8-12, 8-18, 8-22.

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| strings being a more complicated data structure than many programmers give them credit for | অনেক প্রোগ্রামার যতটা ভাবেন তার চেয়ে string বেশি জটিল একটি data structure হওয়া |
| The error tells the story | error-টিই পুরো গল্প বলে |
| the closest thing to what we would call _letters_ | আমরা যাকে _বর্ণ_ বলি, তার সবচেয়ে কাছাকাছি |
| They’re diacritics that don’t make sense on their own. | এগুলো এমন চিহ্ন (diacritic), যা একা অর্থবহ নয়। |
| many more goodies are hiding in the functions | function-গুলোতে আরও অনেক দারুণ জিনিস লুকিয়ে আছে |
| plays more nicely with the borrow checker | borrow checker-এর সঙ্গেও বেশি ভালোভাবে মানিয়ে চলে |
| Rust developers need job security (quiz joke) | Rust ডেভেলপারদের চাকরির নিরাপত্তা দরকার |
| several granularities of sequenced data (quiz) | ক্রমিক ডেটার কয়েকটি আলাদা স্তর |

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

## Chapter 9: Error Handling

**Auto-chosen terms:** recoverable / unrecoverable error, unwind, abort, call
stack, backtrace, environment variable, buffer overread, debug symbol, file
handle, propagate, chain (method calls), parser, rate limit, system call.
Everyday: চুক্তি (contract), প্রশ্নবোধক চিহ্নের operator.

**Decisions:**
- The section title "To `panic!` or Not to `panic!`" became "`panic!` করব
  নাকি করব না" (the second `panic!` is dropped as natural Bengali).
- Link labels to chapter 18 ("Encoding States and Behavior as Types", "Using
  Trait Objects to Abstract over Shared Behavior") stay in English until that
  chapter is translated.
- *prototype* stays in Latin.

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| Errors are a fact of life in software | সফটওয়্যারে error থাকবেই |
| That’s a lot of `match`! The `match` expression is very useful but also very much a primitive. | অনেক `match`! `match` expression খুবই কাজের, কিন্তু বেশ আদিম একটি গঠনও। |
| this is just a different, more ergonomic way to write it | এটি শুধু লেখার একটি আলাদা, বেশি ব্যবহার-স্বাচ্ছন্দ্যপূর্ণ উপায় |
| a _bad state_ is when some assumption, guarantee, contract, or invariant has been broken | _খারাপ অবস্থা_ মানে যখন কোনো ধারণা, নিশ্চয়তা, চুক্তি বা invariant ভেঙে গেছে |
| a contract violation always indicates a caller-side bug | চুক্তি লঙ্ঘন সবসময় caller-এর দিকের একটি বাগ বোঝায় |
| will make your code more reliable in the face of inevitable problems | অনিবার্য সমস্যার মুখে আপনার কোড আরও নির্ভরযোগ্য হবে |

## Chapter 10: Generic Types, Traits, and Lifetimes

**Auto-chosen terms:** concrete type, monomorphization, trait bound, default
implementation, override, coherence, orphan rule (অনাথ নিয়ম), blanket
implementation (ঢালাও implementation), `impl Trait`, `where` clause, syntax
sugar, dangling reference, lifetime annotation, lifetime elision rules,
input / output lifetime, `'static` lifetime, UpperCamelCase, inheritance,
dynamically typed, deallocate. Everyday: বিমূর্ত রূপ (abstraction), নিজস্ব
(local, as in "local to our crate").

**Decisions:**
- The link label to chapter 18 ("Using Trait Objects to Abstract over Shared
  Behavior") stays in English; labels to chapters 4 and 5 use their Bengali
  headings.
- Comments in listings 10-17 and 10-18 are lifetime diagrams (`'a`, `'b`
  bars), so they stay as they are. The comments in listing 10-25 are outside
  the displayed `here` anchor, so they were left in English.
- Inventory quiz options that read like compiler messages ("cannot borrow
  `elems` as mutable for `sort`") stay in English, as in earlier inventories.
- "does not live long enough" is quoted in English with a Bengali gloss,
  because it is the literal compiler message.

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| abstract stand-ins for concrete types or other properties | concrete (নির্দিষ্ট) type বা অন্য বৈশিষ্ট্যের জায়গায় বসানো বিমূর্ত বিকল্প |
| so named because the parent type is not present | নামটি এসেছে কারণ এখানে parent type উপস্থিত থাকে না |
| The subject of the reference doesn’t live as long as the reference. | reference যাকে নির্দেশ করছে, সেটি reference-এর মতো বেশি সময় বাঁচে না। |
| the compiler might only be able to point to a use of our code many steps away from the cause of the problem | compiler হয়তো শুধু আমাদের কোডের এমন কোনো ব্যবহার দেখাতে পারত যা সমস্যার আসল কারণ থেকে অনেক ধাপ দূরে |
| Let’s pretend we’re the compiler. | চলুন নিজেদের compiler ভেবে নিই। |
| 3 really is the correct answer! If you disagree with the answer, please don't submit a bug report! | 3-ই আসলে সঠিক উত্তর! উত্তরের সঙ্গে একমত না হলে দয়া করে bug report জমা দেবেন না! |

## Chapter 11: Writing Automated Tests

**Auto-chosen terms:** test, assert / assertion, test runner, test harness,
unit test, integration test, test suite, correctness, filtering, benchmark
test, documentation test, race condition, thread-safe, parallel,
configuration (`cfg`). Everyday: template (কাঠামো), উপেক্ষিত (ignored).

**Decisions:**
- "pass" / "fail" stay in Latin as verbs ("test pass করে", "fail করে"), since
  the output shows `ok`, `FAILED`, `passed`, `failed`.
- The Dijkstra quote is translated into Bengali; the essay title stays in
  English.
- Link labels to chapter 14 ("Documentation Comments as Tests") and Appendix C
  ("Derivable Traits") stay in English until those parts are translated.
- Listing comments translated: no-listing-11 (`expensive_test`), and the
  `setup` comment in no-listing-12 and no-listing-13. None of them appear in
  any output.txt.
- Quiz: "Which of the following is NOT …" puts **নয়** in bold, to keep the
  emphasis the capitals gave.

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| program testing can be a very effective way to show the presence of bugs, but it is hopelessly inadequate for showing their absence | প্রোগ্রাম testing বাগের উপস্থিতি দেখানোর খুব কার্যকর উপায় হতে পারে, কিন্তু বাগের অনুপস্থিতি দেখানোর জন্য এটি একেবারেই অপর্যাপ্ত |
| we have a silly function | একটি সাদামাটা function |
| Regardless of which testing ideology you adhere to | আপনি testing-এর যে মতাদর্শই মানুন না কেন |
| to more closely imitate the way end users will be using your crate | শেষ ব্যবহারকারীরা আপনার crate যেভাবে ব্যবহার করবে তা আরও কাছ থেকে অনুকরণ করতে |

## Chapter 12: An I/O Project: Building a Command Line Program

**Auto-chosen terms:** test-driven development (TDD), test coverage,
case-insensitive / case-sensitive, exit status, separation of concerns
(দায়িত্ব আলাদা করা), modular / modularity, redirect (অন্যত্র পাঠানো). Everyday:
ছোট হাতের / বড় হাতের অক্ষর (lowercase / uppercase).

**Decisions:**
- `> **Note:** there are no quizzes…` keeps the English bold `**Note:**`
  prefix, the same way `> Note:` is kept.
- The `grep` acronym explanation (**g**lobally search a **r**egular
  **e**xpression and **p**rint) stays in English, because the bold letters
  spell the name.
- The poem title “I’m Nobody! Who are you?” stays in English (the poem file is
  English).
- Link labels to chapters 3, 7, 9, 10 and 11 use their Bengali headings; the
  chapter 13 and 18 links are bare chapter numbers ("অধ্যায় 13").
- Listing comments `// do something with line` (listings 12-17, 12-18) were
  translated; they do not appear in any output.txt.

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| We’ll test-drive the implementation of the functionality | কার্যক্ষমতাটি আমরা test-এর নির্দেশনায় implement করব |
| giving up a little performance to gain simplicity is a worthwhile trade-off | সরলতার বিনিময়ে সামান্য performance ছেড়ে দেওয়া লাভজনক আপস |
| than to try to hyperoptimize code on your first pass | প্রথম চেষ্টাতেই কোডকে অতিমাত্রায় optimize করার চেষ্টার চেয়ে |
| Whew! … we’ve set ourselves up for success in the future. | উফ! … আমরা ভবিষ্যতের সাফল্যের জন্য ভিত তৈরি করে ফেলেছি। |
| Our program is not currently well behaved | আমাদের প্রোগ্রাম এখন ঠিকমতো আচরণ করছে না |

## Chapter 13: Functional Language Features: Iterators and Closures

**Auto-chosen terms:** capture, environment (of a closure), `Fn` / `FnMut` /
`FnOnce` traits, `move` closure, lazy, consume, consuming adapter, iterator
adapter, loop unrolling, top-level function. Everyday: আগেভাগে বলে দিই
(spoiler alert).

**Decisions:**
- The aligned `add_one_v1 … v4` block and the benchmark output block are kept
  byte for byte.
- The Stroustrup quote is translated into Bengali; the talk title “Foundations
  of C++” and the book title _The Adventures of Sherlock Holmes_ stay in
  English.
- Quiz: the True/false question's options became "সত্য" / "মিথ্যা" (the
  MultipleChoice answer is prose).
- No listing comments needed translating in this chapter.

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| in an additive fashion | যোগ হওয়ার ধাঁচে |
| This is a contrived, convoluted way (that doesn’t work) | এটি একটি কৃত্রিম, জটিল (এবং কাজ করে না এমন) চেষ্টা |
| Instead of fiddling with the various bits of looping | loop-এর নানা খুঁটিনাটি নিয়ে নাড়াচাড়া না করে |
| sometimes called the "toilet closure" (quiz, inside code: left in English) | — |
| What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better. | যা আপনি ব্যবহার করেন না, তার জন্য আপনাকে মূল্য দিতে হয় না। আর আরও বলা যায়: যা আপনি ব্যবহার করেন, তা আপনি হাতে লিখে এর চেয়ে ভালো করতে পারতেন না। |

## Chapter 14: More About Cargo and Crates.io

**Auto-chosen terms:** release profile, optimization (`opt-level`), yank,
binary target, path dependency, API token, license identifier, subcommand.
Everyday: dual license (দুটি license একসঙ্গে), first-come, first-served (আগে
এলে আগে পাবেন ভিত্তিতে).

**Decisions:**
- "Figure 14-N" stays in English in the captions, like "Listing N-M"; the
  caption text and the `alt` text are translated.
- The doc comments (`///`, `//!`) in listings 14-1 and 14-2 stay in English,
  because Figures 14-1 and 14-2 are screenshots of exactly that text.
- The shortcut link `[Semantic Versioning]` keeps its English label, because
  in Markdown the label is also the reference key.
- "Section headings" used inside docs (Examples, Panics, Errors, Safety) stay
  in English, since they are literal Markdown headings rustdoc users write.
- Quiz: the Yes/No options became "হ্যাঁ" / "না".
- From this chapter on, long files were translated paragraph by paragraph
  with a script that leaves every code block byte for byte untouched.

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| Creating a useful public API structure is more an art than a science | একটি কাজের public API গঠন তৈরি করা বিজ্ঞানের চেয়ে বেশি শিল্প |
| a faster, Rustier tool for searching files | ফাইলে খোঁজার জন্য আরও দ্রুত, আরও Rust-সুলভ একটি tool |
| than one big blob of code | একটি বড় কোডের স্তূপের বদলে |
| If that happens, you must reset those secrets immediately. | এমন হলে সঙ্গে সঙ্গে সেই গোপন তথ্যগুলো (যেমন password বা token) বদলে ফেলতে হবে। (added the example in parentheses) |

## Chapter 15: Smart Pointers

**Auto-chosen terms:** box, recursive type, cons list, indirection, interior
mutability, reference cycle, memory leak, strong / weak reference, test double
/ mock object, destructor, double free, static analysis, deep copy,
single-threaded, wrapper type, stack overflow, tree. Everyday: resource
(সম্পদ).

**Decisions:**
- The long `alt` texts of Figures 15-1 to 15-4 are translated in full.
- Listing 15-26: the two prose comments ("Uncomment the next line…") are
  translated; the commented-out `println!` line stays as code.
- "Halting Problem" stays in English (a proper name).

**Unsure sentences:**

| English | Bengali |
| --- | --- |
| there would be an uproar from the remaining TV watchers! | বাকি দর্শকদের মধ্যে হইচই পড়ে যেত! |
| Think of it in the sense of a stunt double in filmmaking | চলচ্চিত্রের stunt double-এর কথা ভাবুন |
| Static analysis, like the Rust compiler, is inherently conservative. | Rust compiler-এর মতো static analysis স্বভাবতই রক্ষণশীল। |
| This technique is pretty neat! | এই কৌশলটি বেশ চমৎকার! |
| If this chapter has piqued your interest | এই অধ্যায় যদি আপনার আগ্রহ জাগিয়ে থাকে |

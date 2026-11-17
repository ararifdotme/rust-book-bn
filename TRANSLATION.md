# Bengali Translation Guide

This file records how the Bengali (bn) translation of the Brown University
interactive edition of *The Rust Programming Language* is done: the rules, the
glossary, and progress. The glossary grows one chapter at a time.

## Rules

### Language and style

- Write natural, modern চলিত Bengali, the way a good Bangladeshi teacher would
  explain it. Translate meaning, not word order; rephrase any sentence whose
  literal rendering sounds stiff.
- Keep Rust technical terms in English, in Latin script (ownership, borrowing,
  reference, lifetime, trait, struct, enum, closure, crate, module, macro,
  stack, heap, compiler, and similar), because readers will meet these exact
  words in error messages and documentation. On first use in each chapter add
  a short Bengali gloss in parentheses, e.g. "ownership (মালিকানা)". Bengali
  suffixes attach with a hyphen: "variable-টি", "function-এর".
- Translate ordinary words fully; do not leave English where a common Bengali
  word exists ("উদাহরণ", not "example").
- Use the form recorded in the Glossary below. Add each new term to the
  Glossary in the chapter where it first appears.

### What must never change

- Never change file names, link targets, anchors, heading IDs, HTML tags and
  attributes, mdBook directives, Aquascope blocks, code, identifiers, compiler
  output, or command lines. Changing these breaks links, diagrams, or makes
  examples disagree with real compiler output.
- Under `listings/`, only comments may be translated. String literals and
  printed output stay as they are.
- In quiz files translate only the prose (question, options, explanations).
  Keys, answer values, and code stay untouched. (Details below.)
- Keep the Markdown structure identical: same headings, paragraphs, lists and
  code blocks in the same order, so merges from upstream stay manageable.
- Keep chapter, listing and figure numbers ("Listing 4-1") in their original
  form so cross-references match.
- Never touch: `first-edition`, `second-edition`, `2018-edition`, `nostarch`,
  `redirects`.

### Markdown and mdBook details (from reading chapter 1 and the build tools)

**Heading IDs: pin every translated heading.** mdBook derives a heading's ID
from its text. A Bengali heading gets a new, garbled ID (vowel signs are
dropped: "সমস্যা সমাধান" becomes `#সমসযা-সমাধান`), which breaks every link to
`#troubleshooting` etc. When translating a heading, append the ID the English
heading produced, in `{#…}`:

```markdown
### সমস্যা সমাধান {#troubleshooting}
```

The English ID is the heading text lowercased, punctuation removed, spaces
turned into `-` (check `book/<chapter>.html` built from the untranslated file
if unsure). Tested with this repo's build: the pinned ID is kept.

**Directives, never edit:**

- `{{#quiz ../quizzes/<file>.toml}}`: embeds a quiz.
- `{{#include ../listings/...}}`, `{{#rustdoc_include ...}}`: pull code or
  output from `listings/`.
- `<!-- ignore -->` after a link (sometimes split across lines as
  `<!--` / `ignore -->`): tells the link checker to skip it. Keep it touching
  the link exactly as in the original.
- `<!-- Old headings. Do not remove or links may break. -->` followed by
  `<a id="..."></a>`: keep both lines exactly.
- Reference-style link definitions at the bottom of a file
  (`[install]: https://...`): keep. In the text, translate the link label
  (`[the community page][community]` → `[কমিউনিটি পেজ][community]`) but keep
  the `[community]` key.

**Notes:** `> Note: ...` blockquotes are turned into styled note boxes by the
`trpl-note` preprocessor only when the text starts with exactly `Note: `.
Keep the English `Note: ` prefix and translate the rest of the sentence; a
translated prefix ("দ্রষ্টব্য:") silently renders as a plain quote. Blockquotes
that start with a heading (`> ### Command Line Notation`) are also notes; for
those, translate the heading (with a pinned ID) and the body.

**Listings:**

```markdown
<Listing number="1-1" file-name="main.rs" caption="A program that prints `Hello, world!`">
```

- `number` and `file-name`: never change.
- `caption`: this is reader-visible prose and **is** translated. Code spans
  inside it stay as code. The tool adds the English "Listing 1-1:" prefix
  itself.
- `<span class="filename">Filename: src/main.rs</span>`: keep as is (it matches
  the "Filename:" text the Listing tool generates).

**Image alt text:** `alt="..."` on `<img>` is translated, because screen
readers read it aloud. `src`, `class` and every other attribute stay as they
are. (`caption` on `<Listing>` and `alt` are the only attributes translated.)

**Code blocks:** ```` ```rust ````, ```` ```console ````, ```` ```text ````,
```` ```toml ````, ```` ```cmd ````, ```` ```powershell ```` etc. are never
translated, including comments inside `console`/`cmd` blocks (those are part
of a command line). Comments inside ```` ```rust ```` blocks in chapter text
may be translated, the same as under `listings/`.

**Aquascope blocks** (```` ```aquascope,interpreter ````,
```` ```aquascope,permissions,stepper ```` and other option combinations) are
programs that Aquascope compiles and analyses to draw diagrams. Do not edit
anything inside them, including comments, the `#`-prefixed hidden lines
(`#fn main() {`), and annotations such as `` `[]` `` or `` `{}` ``. Also keep
the inline permission markup in surrounding prose, e.g.
`@Perm[gained]{read}`, and the icon tags `<i class="fa fa-arrow-right"></i>`.

### Quiz files (`quizzes/*.toml`)

Each question is a `[[questions]]` table. Some files use a `[multipart]`
table first, whose values (`q = """..."""`) are shared context prose.

| Field | Translate? |
| --- | --- |
| `id` | No (UUID; mdbook-quiz uses it to store answers) |
| `type` (`MultipleChoice`, `ShortAnswer`, `Tracing`) | No |
| `prompt.prompt` | Yes: prose. Keep code blocks and `` `code` `` spans inside it unchanged |
| `prompt.distractors` | Yes: these are the wrong options shown to the reader. Code inside stays code; options that are pure code (`"R"`, ``"`parse_flag_v2`"``) stay as is |
| `answer.answer` in **MultipleChoice** | Yes, as prose: it is the text of the correct option shown to the reader. Same code rule as distractors. If it is a list (`["No permissions"]`), translate each string the same way |
| `answer.answer` in **ShortAnswer** | **No.** It is the answer key the reader's typed answer is compared against (`rustup`, `main`, `foobar::engine::run`) |
| `answer.alternatives` (ShortAnswer) | No: other accepted answer keys |
| `prompt.program` (Tracing) | No: Rust code |
| `answer.doesCompile`, `answer.lineNumber` | No: answer key |
| `answer.stdout` | No: program output |
| `prompt.answerIndex`, `prompt.sortAnswers` | No: control fields |
| `context` | Yes: the explanation shown after answering. Keep code and links unchanged; translate link labels only |

Keep TOML quoting intact (`"..."` for one line, `"""..."""` for multi-line).
The quiz validator (`validate = true` in `book.toml`) accepts Bengali prose;
tested with this repo's build.

## Glossary

| English term | Form we use | Bengali gloss | First chapter it appears in |
| --- | --- | --- | --- |
| API | API | প্রোগ্রামিং ইন্টারফেস | 0 |
| async / await | async / await | অ্যাসিঙ্ক্রোনাস প্রোগ্রামিংয়ের সিনট্যাক্স | 0 |
| build tool | build tool | কোড বিল্ড ও সংকলনের টুল | 0 |
| closure | closure | পারিপার্শ্বিক মান ধরে রাখতে পারে এমন নামহীন function | 0 |
| collection | collection | একাধিক মান রাখার কাঠামো | 0 |
| compile | compile করা | সংকলন করা | 0 |
| compile time | compile time | সংকলনের সময় | 0 |
| compiler | compiler | সংকলক | 0 |
| concurrency | concurrency | একসঙ্গে একাধিক কাজ সামলানো | 0 |
| control flow | control flow | কোড চলার ক্রম নিয়ন্ত্রণ | 0 |
| data structure | data structure | ডেটা সাজানোর কাঠামো | 0 |
| dependency | dependency | নির্ভরতা (যে লাইব্রেরির ওপর কোড নির্ভর করে) | 0 |
| derivable trait | derivable trait | স্বয়ংক্রিয়ভাবে যুক্ত করা যায় এমন trait | 0 |
| edition | edition | Rust-এর সংস্করণ-ধাপ | 0 |
| enum | enum | কয়েকটি সম্ভাব্য রূপের একটি নিতে পারে এমন টাইপ | 0 |
| error handling | error handling | ত্রুটি সামলানো | 0 |
| expression | expression | মান তৈরি করে এমন কোড-অংশ | 0 |
| function | function | ফাংশন | 0 |
| functional programming | functional programming | ফাংশনভিত্তিক প্রোগ্রামিং | 0 |
| future | future | ভবিষ্যতে পাওয়া যাবে এমন মান | 0 |
| generics | generics | একাধিক টাইপের জন্য একই কোড | 0 |
| hash map | hash map | key-value জোড়ার তালিকা | 0 |
| high-level | high-level | উচ্চ-স্তরের | 0 |
| idiom | idiom | Rust-এর প্রচলিত লেখার ধরন | 0 |
| iterator | iterator | একটির পর একটি উপাদান দেয় এমন বস্তু | 0 |
| keyword | keyword | ভাষার সংরক্ষিত শব্দ | 0 |
| library | library | লাইব্রেরি | 0 |
| lifetime | lifetime | reference কতক্ষণ বৈধ থাকে | 0 |
| low-level | low-level | নিম্ন-স্তরের | 0 |
| macro | macro | কোড তৈরি করে এমন কোড | 0 |
| memory safety | memory safety | মেমোরি-নিরাপত্তা | 0 |
| method | method | টাইপের সঙ্গে যুক্ত function | 0 |
| module | module | কোডের অংশ ভাগ করার একক | 0 |
| multithreaded | multithreaded | একাধিক thread-যুক্ত | 0 |
| nightly Rust | nightly Rust | প্রতি রাতে তৈরি পরীক্ষামূলক সংস্করণ | 0 |
| object-oriented programming | object-oriented programming | অবজেক্টভিত্তিক প্রোগ্রামিং | 0 |
| operator | operator | অপারেটর-চিহ্ন | 0 |
| ownership | ownership | মালিকানা | 0 |
| package manager | package manager | প্যাকেজ ব্যবস্থাপক | 0 |
| panic | panic | প্রোগ্রামের হঠাৎ থেমে যাওয়া | 0 |
| pattern | pattern | মানের গঠনের নকশা | 0 |
| pattern matching | pattern matching | নকশার সঙ্গে মেলানো | 0 |
| privacy | privacy | গোপনীয়তা (কে কী দেখতে পারবে) | 0 |
| refactoring | refactoring | আচরণ না বদলে কোড পুনর্গঠন | 0 |
| runtime | runtime | প্রোগ্রাম চলার সময় | 0 |
| Rustacean | Rustacean | Rust প্রোগ্রামার | 0 |
| smart pointer | smart pointer | অতিরিক্ত ক্ষমতাসম্পন্ন pointer | 0 |
| standard library | standard library | আদর্শ লাইব্রেরি | 0 |
| stream | stream | ধারাবাহিক মানের প্রবাহ | 0 |
| string | string | লেখা (টেক্সট) | 0 |
| struct | struct | নিজস্ব গঠনের ডেটা টাইপ | 0 |
| syntax | syntax | ভাষার লিখনরীতি | 0 |
| systems programming | systems programming | সিস্টেম-স্তরের প্রোগ্রামিং | 0 |
| task | task | কাজ | 0 |
| testing | testing | পরীক্ষা ("test লেখা") | 0 |
| thread | thread | প্রোগ্রামের ভেতরের আলাদা কাজের ধারা | 0 |
| trait | trait | টাইপের সাধারণ আচরণের সংজ্ঞা | 0 |
| type | type | ডেটার ধরন | 0 |
| unsafe Rust | unsafe Rust | নিরাপত্তা-যাচাই শিথিল করা Rust | 0 |
| vector | vector | পরিবর্তনশীল দৈর্ঘ্যের তালিকা | 0 |
| zero-cost abstraction | zero-cost abstraction | বাড়তি খরচ ছাড়া উচ্চ-স্তরের সুবিধা | 0 |

### Everyday computing words

These are written in Bengali script because a common Bengali form exists.
Names and acronyms (IDE, DevOps, Firefox, GitHub) stay unchanged.

| English | Form we use | First chapter it appears in |
| --- | --- | --- |
| bug | বাগ | 0 |
| command line | কমান্ড লাইন | 0 |
| cookie | কুকি | 0 |
| ecosystem | ইকোসিস্টেম | 0 |
| highlight | হাইলাইট | 0 |
| open source | ওপেন সোর্স | 0 |
| quiz | কুইজ | 0 |

## Progress

Translating from upstream
[cognitive-engineering-lab/rust-book](https://github.com/cognitive-engineering-lab/rust-book)
commit `88250e0392cef0622f318e35469108d68694c1f7`
("Merge pull request #394 from cognitive-engineering-lab/dev").

Each line covers the chapter's `.md` files and the quizzes and listings they
use.

- [x] Chapter 0: Front matter (experiment-intro, title-page, foreword, ch00-00-introduction)
- [ ] Chapter 1: Getting Started (ch01-*)
- [ ] Chapter 2: Programming a Guessing Game (ch02-*)
- [ ] Chapter 3: Common Programming Concepts (ch03-*)
- [ ] Chapter 4: Understanding Ownership (ch04-*)
- [ ] Chapter 5: Using Structs to Structure Related Data (ch05-*)
- [ ] Chapter 6: Enums and Pattern Matching (ch06-*)
- [ ] Chapter 7: Packages, Crates, and Modules (ch07-*)
- [ ] Chapter 8: Common Collections (ch08-*)
- [ ] Chapter 9: Error Handling (ch09-*)
- [ ] Chapter 10: Generic Types, Traits, and Lifetimes (ch10-*)
- [ ] Chapter 11: Writing Automated Tests (ch11-*)
- [ ] Chapter 12: An I/O Project: Building a Command Line Program (ch12-*)
- [ ] Chapter 13: Functional Language Features: Iterators and Closures (ch13-*)
- [ ] Chapter 14: More about Cargo and Crates.io (ch14-*)
- [ ] Chapter 15: Smart Pointers (ch15-*)
- [ ] Chapter 16: Fearless Concurrency (ch16-*)
- [ ] Chapter 17: Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams (ch17-*)
- [ ] Chapter 18: Object Oriented Programming Features (ch18-*)
- [ ] Chapter 19: Patterns and Matching (ch19-*)
- [ ] Chapter 20: Advanced Features (ch20-*)
- [ ] Chapter 21: Final Project: Building a Multithreaded Web Server (ch21-*)
- [ ] End of Experiment (end-of-experiment)
- [ ] Appendix (appendix-00 to appendix-07)

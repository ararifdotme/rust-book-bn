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

**Comments quoted in compiler output stay in English.** If a code comment's
line also appears in an error message shown in the book (a `text` block in the
chapter, or the listing's `output.txt`), leave that comment untranslated, e.g.
`// oh no! x isn't defined!` (ch04-01) and `// error!`
(`no-listing-19-slice-error`). Otherwise the example and the output disagree.

**Quiz options that quote compiler messages stay in English** (chapter 6
onwards), e.g. "cannot return reference to local variable `default`". They
imitate what rustc prints; the question text around them is translated.

**"Ownership Inventory #N"** (chapters 6, 8, 10, 18) keeps its English series
name in the heading; the rest of the page is translated.

**Quotations from people** are translated inside the same blockquote; titles of
talks or papers stay in English.

**Named principles** (chapter 4): give a Bengali name with the English in
parentheses at first mention, e.g. "pointer নিরাপত্তার নীতি (Pointer Safety
Principle)", then the Bengali name alone.

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
| ahead-of-time compiled | ahead-of-time compiled | চালানোর আগেই সংকলিত | 1 |
| argument | argument | function-এ পাঠানো আসল মান | 1 |
| benchmark | benchmark করা | চলার গতি মাপা | 1 |
| build | build করা (noun: build) | কোড compile করে চালানোর উপযোগী ফাইল বানানো | 1 |
| build system | build system | কোড build করার ব্যবস্থা | 1 |
| crate | crate | Rust-এর কোডের একক (library বা প্রোগ্রাম) | 1 |
| debug build | debug build | দ্রুত তৈরি, ডিবাগের উপযোগী build | 1 |
| declare | declare করা | ঘোষণা করা | 1 |
| dynamic language | dynamic language | Ruby বা Python-এর মতো সরাসরি চালানো ভাষা | 1 |
| executable / binary | executable / binary | চালানো যায় এমন ফাইল | 1 |
| flag | flag | কমান্ডের সঙ্গে দেওয়া অপশন | 1 |
| function body | function body | function-এর মূল অংশ | 1 |
| key (in TOML) | key | key-value জোড়ার নাম অংশ | 1 |
| linker | linker | compile হওয়া অংশগুলো জুড়ে একটি ফাইল বানানোর প্রোগ্রাম | 1 |
| optimization | optimization (verb: optimize করা) | কোড দ্রুততর করা | 1 |
| package | package | প্যাকেজ (কোডের বিতরণযোগ্য একক) | 1 |
| parameter | parameter | function-এ পাঠানো মানের জন্য রাখা নাম | 1 |
| profile | profile | build-এর সেটিংসের সমষ্টি | 1 |
| prompt (command line) | prompt | কমান্ড লেখার জায়গা নির্দেশকারী চিহ্ন | 1 |
| release build | release build | চূড়ান্ত, optimize করা build | 1 |
| stable (version/release) | stable সংস্করণ | স্থিতিশীল সংস্করণ | 1 |
| variable | variable | মান রাখার নাম | 1 |
| version control system | version control system | কোডের পরিবর্তনের ইতিহাস রাখার ব্যবস্থা | 1 |
| annotation | annotation (verb: annotate করা) | type স্পষ্ট করে লিখে দেওয়া | 2 |
| arm (of `match`) | arm | `match`-এর একেকটি শাখা | 2 |
| artifact | artifact | build-এর ফলে তৈরি ফাইল | 2 |
| associated function | associated function | কোনো type-এর সঙ্গে যুক্ত function | 2 |
| binary crate | binary crate | চালানো যায় এমন crate | 2 |
| bind | bind করা | কোনো নামের সঙ্গে মান জুড়ে দেওয়া | 2 |
| carriage return | carriage return | লাইনের শুরুতে ফেরার অক্ষর | 2 |
| catch-all | catch-all | বাকি সব কিছুর সঙ্গে মিলে যায় এমন | 2 |
| enumeration | enumeration | enum-এর পূর্ণ নাম | 2 |
| error | error | ত্রুটি | 2 |
| evaluate | evaluate করা | মান নির্ণয় করা | 2 |
| format string | format string | placeholder-সহ string | 2 |
| handle | handle | কোনো resource ব্যবহারের মাধ্যম | 2 |
| immutable | immutable | অপরিবর্তনীয় | 2 |
| implement | implement করা | বাস্তবায়ন করা | 2 |
| infinite loop | infinite loop | কখনো না থামা loop | 2 |
| instance | instance | কোনো type-এর একটি নির্দিষ্ট মান | 2 |
| integer | integer | পূর্ণসংখ্যা | 2 |
| iteration | iteration | loop-এর একেকবার চলা | 2 |
| library crate | library crate | অন্য প্রোগ্রামে ব্যবহারের জন্য লেখা crate | 2 |
| loop | loop | বারবার চলতে থাকা কোড-অংশ | 2 |
| mutability | mutability | পরিবর্তনযোগ্যতা | 2 |
| mutable | mutable | পরিবর্তনযোগ্য | 2 |
| newline | newline | নতুন লাইনের অক্ষর | 2 |
| patch release | patch release | ছোটখাটো সংশোধনী সংস্করণ | 2 |
| placeholder | placeholder | মান বসানোর জন্য রাখা জায়গা | 2 |
| prelude | prelude | প্রতিটি প্রোগ্রামে আপনা-আপনি আসা জিনিসের সেট | 2 |
| random number generator | random number generator | র‍্যান্ডম সংখ্যা তৈরির ব্যবস্থা | 2 |
| range | range | একটি সীমার ভেতরের মানগুলো | 2 |
| reference | reference | ডেটা কপি না করে তার দিকে নির্দেশ করার উপায় | 2 |
| registry | registry | crate-এর তথ্যভান্ডার | 2 |
| regression | regression | আগে কাজ করা কিছু নতুন সংস্করণে ভেঙে যাওয়া | 2 |
| reproducible build | reproducible build | প্রতিবার হুবহু একই ফল দেয় এমন build | 2 |
| return | return করা | ফেরত দেওয়া | 2 |
| scope | scope ("scope-এ আনা") | কোডের যে অংশে একটি নাম ব্যবহার করা যায় | 2 |
| seed | seed করা | র‍্যান্ডম সংখ্যার শুরুর মান দেওয়া | 2 |
| Semantic Versioning (SemVer) | Semantic Versioning (SemVer) | সংস্করণ নম্বর লেখার একটি মানদণ্ড | 2 |
| shadowing | shadowing (verb: shadow করা) | একই নামে নতুন variable দিয়ে আগেরটিকে আড়াল করা | 2 |
| standard input | standard input | কিবোর্ড থেকে আসা ইনপুটের ধারা | 2 |
| statement | statement | কাজ করে কিন্তু মান তৈরি করে না এমন কোড-নির্দেশ | 2 |
| static type system | static type system | compile-এর সময়েই type নির্দিষ্ট হয় এমন ব্যবস্থা | 2 |
| type inference | type inference (verb: infer করা) | type অনুমান করে নেওয়া | 2 |
| unsigned | unsigned | চিহ্নহীন (ঋণাত্মক হতে পারে না) | 2 |
| variant | variant | enum-এর সম্ভাব্য একটি রূপ | 2 |
| version specifier | version specifier | সংস্করণ নির্ধারক | 2 |
| allocate | allocate করা | মেমোরিতে জায়গা বরাদ্দ করা | 3 |
| architecture | architecture | প্রসেসরের গঠন (32-bit বা 64-bit) | 3 |
| array | array | একই type-এর নির্দিষ্টসংখ্যক মানের সারি | 3 |
| assign | assign করা | মান বসানো | 3 |
| block | block | `{}` দিয়ে ঘেরা কোড-অংশ | 3 |
| Boolean | Boolean | সত্য বা মিথ্যা মান | 3 |
| call | call করা | (function বা macro) চালানোর নির্দেশ দেওয়া | 3 |
| caller | caller | যে কোড function-টি call করে | 3 |
| character type | character type | একটি অক্ষর রাখার type (`char`) | 3 |
| compound type | compound type | একাধিক মান মিলিয়ে তৈরি type | 3 |
| conditional | conditional | শর্তের ওপর নির্ভর করে চলা কোড | 3 |
| constant | constant | ধ্রুবক (কখনো বদলায় না এমন মান) | 3 |
| constant expression | constant expression | compile-এর সময়েই মান নির্ণয় করা যায় এমন expression | 3 |
| data type | data type | ডেটার ধরন | 3 |
| debug mode | debug mode | ডিবাগের জন্য compile | 3 |
| destructuring | destructuring (verb: destructure করা) | একটি মানকে ভেঙে আলাদা আলাদা অংশে নেওয়া | 3 |
| documentation comment | documentation comment | ডকুমেন্টেশন তৈরির জন্য লেখা কমেন্ট | 3 |
| expression-based language | expression-based language | expression-কেন্দ্রিক ভাষা | 3 |
| floating-point number | floating-point সংখ্যা | দশমিক বিন্দুযুক্ত সংখ্যা | 3 |
| global scope | global scope | পুরো প্রোগ্রামজুড়ে দৃশ্যমান scope | 3 |
| hardcoded value | hardcoded মান | কোডে সরাসরি লিখে দেওয়া মান | 3 |
| index | index | ক্রমিক অবস্থান নম্বর (0 থেকে শুরু) | 3 |
| initialize | initialize করা | প্রথম মান দেওয়া | 3 |
| integer overflow | integer overflow | type-এর ধারণক্ষমতার বাইরে মান চলে যাওয়া | 3 |
| literal | literal | কোডে সরাসরি লেখা মান | 3 |
| loop label | loop label | loop-কে চেনানোর নাম-চিহ্ন | 3 |
| machine code | machine code | প্রসেসর সরাসরি চালাতে পারে এমন কোড | 3 |
| primitive type | primitive type | ভাষায় সরাসরি থাকা মৌলিক type | 3 |
| release mode | release mode | চূড়ান্ত, optimize করা compile | 3 |
| return type | return type | function যে type-এর মান ফেরত দেয় | 3 |
| saturate | saturate করা | সীমায় পৌঁছে আটকে থাকা | 3 |
| scalar type | scalar type | একটিমাত্র মান প্রকাশ করে এমন type | 3 |
| signature | signature | function-এর নাম, parameter ও return type-এর ঘোষণা | 3 |
| signed | signed | চিহ্নযুক্ত | 3 |
| snake case | snake case | ছোট হাতের অক্ষর আর আন্ডারস্কোর দিয়ে নাম লেখার রীতি | 3 |
| stack / heap | stack / heap | ক্রমানুসারে মান রাখার মেমোরি-অংশ / প্রয়োজনমতো জায়গা বরাদ্দের মেমোরি-অংশ | 3 |
| string literal | string literal | উদ্ধৃতিচিহ্নের ভেতরে সরাসরি লেখা string | 3 |
| tuple | tuple | বিভিন্ন type-এর নির্দিষ্টসংখ্যক মানের একটি দল | 3 |
| two’s complement | two’s complement | ঋণাত্মক সংখ্যা বাইনারিতে রাখার একটি পদ্ধতি | 3 |
| type suffix | type suffix | সংখ্যার শেষে জুড়ে দেওয়া type-এর নাম | 3 |
| Unicode scalar value | Unicode scalar value | Unicode-এর একটি একক অক্ষর-মান | 3 |
| unit / unit type | unit / unit type | খালি মান ও তার type, `()` | 3 |
| wrapping | wrapping | সীমা পেরোলে আবার শুরুর দিকে ঘুরে আসা | 3 |
| aliasing / alias | aliasing / alias | একই ডেটা একাধিক variable দিয়ে ব্যবহার করা | 4 |
| assembly code | assembly code | প্রসেসরের নির্দেশের মানুষের পড়ার উপযোগী রূপ | 4 |
| borrow | borrow করা | ধার নেওয়া (reference তৈরি করা) | 4 |
| borrow checker | borrow checker | borrow-এর নিয়ম যাচাই করা compiler-এর অংশ | 4 |
| box (`Box`) | box | heap-এর ডেটার মালিক এমন pointer | 4 |
| clone | clone করা | ডেটার হুবহু নতুন কপি তৈরি করা | 4 |
| copyable (`Copy`) | copyable | move ছাড়াই কপি করা যায় এমন | 4 |
| data race | data race | একাধিক thread একই ডেটা একসঙ্গে বদলালে হওয়া অনিশ্চিত আচরণ | 4 |
| deallocate / free / drop | deallocate করা (free, drop) | বরাদ্দ করা মেমোরি ফেরত দেওয়া | 4 |
| dereference | dereference করা | pointer ধরে তার ডেটায় পৌঁছানো | 4 |
| deref coercion | deref coercion | স্বয়ংক্রিয় dereference-রূপান্তর | 4 |
| double-free | double-free | একই মেমোরি দুবার free করা | 4 |
| endpoint | endpoint | API-র একটি নির্দিষ্ট ঠিকানা | 4 |
| exception | exception | চলার সময় ঘটা error-এর সংকেত | 4 |
| fat pointer | fat pointer | metadata-সহ pointer | 4 |
| field | field | struct বা tuple-এর একেকটি অংশ | 4 |
| frame / stack frame | frame | একটি function-এর variable-গুলোর মেমোরি-অংশ | 4 |
| garbage collection | garbage collection (garbage collector) | অব্যবহৃত মেমোরি আপনা-আপনি মুক্ত করার ব্যবস্থা | 4 |
| immutable reference | immutable reference (shared reference) | শুধু পড়ার reference | 4 |
| interpreter | interpreter | কোড লাইন ধরে ধরে চালানোর প্রোগ্রাম | 4 |
| lifetime parameter | lifetime parameter | reference-এর lifetime বোঝানোর parameter | 4 |
| local variable | local variable | শুধু একটি function-এর ভেতরে থাকা variable | 4 |
| memory corruption | memory corruption | মেমোরির ডেটা অবৈধভাবে নষ্ট হওয়া | 4 |
| memory management | memory management | মেমোরি বরাদ্দ ও ফেরতের ব্যবস্থাপনা | 4 |
| move | move করা | ownership এক variable থেকে আরেকটিতে সরানো | 4 |
| mutable reference | mutable reference (unique reference) | বদলানোর অনুমতিসহ একক reference | 4 |
| mutate / mutation | mutate করা / mutation | ডেটা বদলানো | 4 |
| non-owning pointer | non-owning pointer | ডেটার মালিক নয় এমন pointer | 4 |
| owner / own | owner / own করা | মালিক / মালিক হওয়া | 4 |
| permission | permission | অনুমতি; নামগুলো ইংরেজিতে: Read (পড়া), Write (লেখা/বদলানো), Own (মালিকানা), Flow (প্রবাহ) | 4 |
| place | place | যা assignment-এর বাঁ পাশে বসতে পারে (variable, `*a`, `a[0]` ইত্যাদি) | 4 |
| pointer / pointee | pointer / pointee | মেমোরির একটি অবস্থান নির্দেশকারী মান / pointer যে ডেটাকে নির্দেশ করে | 4 |
| raw pointer | raw pointer | borrow checker যাচাই করে না এমন pointer | 4 |
| reference counting | reference counting | কতগুলো reference আছে গুনে মেমোরি ফেরতের ব্যবস্থা | 4 |
| register | register | প্রসেসরের ভেতরের ছোট মেমোরি-ঘর | 4 |
| segmentation fault | segmentation fault | অবৈধ মেমোরিতে ঢোকার ফলে হওয়া error | 4 |
| shallow copy / deep copy | shallow copy / deep copy | শুধু pointer কপি / পুরো ডেটা কপি | 4 |
| slice | slice | collection-এর একটানা একটি অংশের reference | 4 |
| sound / incomplete (analysis) | sound / incomplete | কখনো ভুল গ্রহণ করে না / কিছু সঠিক কোডও বাতিল করে | 4 |
| string slice | string slice | string-এর একটি অংশের reference (`&str`) | 4 |
| syntactic sugar | syntactic sugar | একই কাজের সহজতর লেখার রূপ | 4 |
| type alias | type alias | কোনো type-এর বিকল্প নাম | 4 |
| undefined behavior | undefined behavior | অনির্ধারিত আচরণ (যার ফল ভাষা নির্দিষ্ট করে না) | 4 |
| `unsafe` block | `unsafe` block | নিরাপত্তা-যাচাই শিথিল করা কোড-অংশ | 4 |
| use-after-free | use-after-free | free করা মেমোরি আবার ব্যবহার করা | 4 |
| attribute | attribute | কোডের সঙ্গে জুড়ে দেওয়া বাড়তি নির্দেশ (`#[...]`) | 5 |
| constructor | constructor | নতুন instance তৈরি করে এমন function | 5 |
| derive | derive করা | trait-এর implementation আপনা-আপনি তৈরি করা | 5 |
| desugar | desugar করা | syntactic sugar খুলে মূল রূপে লেখা | 5 |
| dot notation | dot notation | `.` দিয়ে field বা method-এ পৌঁছানোর লিখনরীতি | 5 |
| field init shorthand | field init shorthand | variable আর field-এর নাম এক হলে সংক্ষিপ্ত লেখা | 5 |
| getter | getter | শুধু একটি field-এর মান ফেরত দেয় এমন method | 5 |
| `impl` block | `impl` block | একটি type-এর method ও associated function সংজ্ঞায়িত করার block | 5 |
| instantiate | instantiate করা | একটি instance তৈরি করা | 5 |
| lifetime specifier | lifetime specifier | lifetime নির্দেশক | 5 |
| namespace | namespace | নামগুলোর আলাদা একটি পরিসর | 5 |
| object | object | বস্তু (object-oriented ভাষায় ডেটা ও আচরণের একক) | 5 |
| outer attribute | outer attribute | যে item-এর ঠিক আগে বসে, তার ওপর প্রযোজ্য attribute | 5 |
| owned type | owned type | নিজের ডেটার মালিক এমন type (যেমন `String`) | 5 |
| public / private | public / private | বাইরে থেকে ব্যবহার করা যায় / যায় না | 5 |
| reborrow | reborrow করা | একটি reference থেকে আবার borrow করা | 5 |
| receiver (method receiver) | receiver | যে মানের ওপর method call করা হয় | 5 |
| standard output / standard error | standard output / standard error | সাধারণ আউটপুটের ধারা / error-বার্তার আউটপুটের ধারা | 5 |
| struct update syntax | struct update syntax | অন্য instance থেকে বাকি field-এর মান নেওয়ার syntax | 5 |
| trait object | trait object | trait-এর মাধ্যমে বিভিন্ন type ব্যবহার করার মান | 5 |
| tuple struct | tuple struct | field-এর নাম ছাড়া tuple-এর মতো struct | 5 |
| type checking | type checking | type ঠিক আছে কি না যাচাই | 5 |
| unit-like struct | unit-like struct | কোনো field নেই এমন struct | 5 |
| anti-pattern | anti-pattern | এড়িয়ে চলা উচিত এমন প্রচলিত ধরন | 6 |
| binding mode | binding mode | pattern-এ মান move না reference হিসেবে bind হবে, তার নিয়ম | 6 |
| boilerplate | boilerplate | বারবার লিখতে হয় এমন গৎবাঁধা কোড | 6 |
| clone-on-write | clone-on-write | দরকার হলে তবেই clone করার ব্যবস্থা (`Cow`) | 6 |
| dangling pointer | dangling pointer | অবৈধ মেমোরিকে নির্দেশ করা pointer | 6 |
| exhaustive | exhaustive | সব সম্ভাবনা সামলানো | 6 |
| generic type parameter | generic type parameter | যেকোনো type বসানো যায় এমন জায়গা-রাখা নাম (`T`) | 6 |
| happy path | happy path | সব ঠিকঠাক থাকলে কোড যে পথে চলে | 6 |
| invariant | invariant | সবসময় সত্য থাকতে হবে এমন শর্ত | 6 |
| null | null | "কোনো মান নেই" বোঝানো বিশেষ মান | 6 |
| substring | substring | একটি string-এর ভেতরের অংশ | 4 |
| type safety | type safety | type-এর নিয়ম মানার নিশ্চয়তা | 6 |
| type system | type system | ভাষার type-সংক্রান্ত নিয়মের ব্যবস্থা | 2 |
| unwrap | unwrap করা | `Option`/`Result`-এর ভেতরের মান বের করা | 6 |
| wildcard | wildcard | যেকোনো মানের সঙ্গে মেলে এমন pattern | 6 |
| module system | module system | package, crate, module আর path নিয়ে কোড সংগঠনের ব্যবস্থা | 7 |
| path | path | কোনো item-এর নাম লেখার পথ (যেমন `a::b::c`) | 7 |
| item | item | function, struct, enum, module, constant-এর মতো সংজ্ঞায়িত জিনিস | 7 |
| crate root | crate root | crate-এর মূল ফাইল, যেখান থেকে compiler শুরু করে | 7 |
| module tree | module tree | module-গুলোর গাছের মতো কাঠামো | 7 |
| submodule | submodule | module-এর ভেতরের module | 7 |
| parent / child / sibling module | parent / child / sibling | ওপরের module / ভেতরের module / একই স্তরের module | 7 |
| ancestor module | ancestor | ওপরের যেকোনো স্তরের module | 7 |
| absolute path / relative path | absolute path / relative path | crate root থেকে শুরু হওয়া পূর্ণ পথ / বর্তমান module থেকে শুরু হওয়া পথ | 7 |
| encapsulate | encapsulate করা | ভেতরের খুঁটিনাটি লুকিয়ে রাখা | 7 |
| workspace | workspace | একসঙ্গে বেড়ে ওঠা package-গুলোর সমষ্টি | 7 |
| glob operator | glob operator | `*` দিয়ে সব public item একসঙ্গে আনার চিহ্ন | 7 |
| re-export | re-export করা | আনা নামকে আবার বাইরে প্রকাশ করা | 7 |
| alias | alias | বিকল্প নাম | 7 |
| nested path | nested path | একই শুরুর অংশের পথগুলো `{}` দিয়ে একত্র করা | 7 |
| symbolic link | symbolic link | ফাইল বা ডিরেক্টরির শর্টকাট-লিংক | 7 |
| build script | build script | build-এর আগে চলা Rust স্ক্রিপ্ট (`build.rs`) | 7 |
| static method | static method | instance ছাড়াই call করা যায় এমন associated function | 7 |
| map | map | key থেকে মানে যাওয়ার data structure | 8 |
| hashing function / hash function | hashing function | key থেকে একটি সংখ্যা (hash) হিসাব করার function | 8 |
| hasher | hasher | hash function তৈরি করে এমন type | 8 |
| iterate | iterate করা | একে একে প্রতিটি উপাদানে যাওয়া | 8 |
| wrapper | wrapper | অন্য একটি type-কে মুড়ে রাখা type | 8 |
| associated type | associated type | trait-এর সঙ্গে যুক্ত type | 8 |
| coerce | coerce করা | এক type-কে স্বয়ংক্রিয়ভাবে আরেক type-এ রূপান্তর করা | 8 |
| grapheme cluster | grapheme cluster | মানুষের চোখে একটি অক্ষর, যা একাধিক Unicode মান মিলে গঠিত | 8 |
| constant time | constant time | ইনপুটের আকার যা-ই হোক, একই সময় | 8 |
| profile (performance) | profile করা | কোডের কোন অংশে কত সময় লাগে তা মাপা | 8 |
| byte slice | byte slice | byte-এর slice (`&[u8]`) | 8 |
| buffer overflow | buffer overflow | বাফারের সীমার বাইরে লেখা | 8 |
| null-terminated | null-terminated | শেষে একটি null byte দিয়ে শেষ হওয়া | 8 |
| predicate | predicate | true/false ফেরত দেয় এমন শর্ত-function | 8 |
| recoverable / unrecoverable error | recoverable / unrecoverable error | সামলে ওঠা যায় এমন / সামলে ওঠা যায় না এমন error | 9 |
| unwind | unwind করা | stack ধরে ফিরে গিয়ে ডেটা পরিষ্কার করা | 9 |
| abort | abort করা | পরিষ্কার না করেই প্রোগ্রাম সঙ্গে সঙ্গে বন্ধ করা | 9 |
| call stack | call stack | call হওয়া function-গুলোর ক্রম | 9 |
| backtrace | backtrace | এই পর্যন্ত call হওয়া সব function-এর তালিকা | 9 |
| environment variable | environment variable | অপারেটিং সিস্টেমে রাখা নামযুক্ত সেটিং-মান | 9 |
| buffer overread | buffer overread | বাফারের সীমার বাইরে পড়া | 9 |
| debug symbols | debug symbol | ডিবাগের জন্য প্রোগ্রামে রাখা বাড়তি তথ্য | 9 |
| file handle | file handle | খোলা ফাইল ব্যবহারের মাধ্যম | 9 |
| propagate | propagate করা | error-কে call করা কোডের দিকে ফেরত পাঠানো | 9 |
| chain (method calls) | chain করা | একটির পর একটি method call জুড়ে লেখা | 9 |
| parser | parser | লেখা বিশ্লেষণ করে গঠন বের করে এমন প্রোগ্রাম | 9 |
| rate limit | rate limit | নির্দিষ্ট সময়ে অনুরোধের সর্বোচ্চ সীমা | 9 |
| system call | system call | অপারেটিং সিস্টেমের কাছে সরাসরি অনুরোধ | 9 |
| concrete type | concrete type | নির্দিষ্ট type (generic নয়) | 10 |
| monomorphization | monomorphization | generic কোডকে নির্দিষ্ট type-এর কোডে রূপান্তর | 10 |
| trait bound | trait bound | generic type-এর ওপর trait-এর শর্ত | 10 |
| default implementation | default implementation | trait-এ আগে থেকে দেওয়া method-এর body | 10 |
| override | override করা | নিজের সংস্করণ দিয়ে প্রতিস্থাপন করা | 10 |
| coherence | coherence | সামঞ্জস্য (একই type-এ একই trait-এর একাধিক implementation না থাকা) | 10 |
| orphan rule | orphan rule | অনাথ নিয়ম: বাইরের type-এ বাইরের trait implement করা যায় না | 10 |
| blanket implementation | blanket implementation | trait bound পূরণ করা সব type-এর জন্য ঢালাও implementation | 10 |
| `impl Trait` | `impl Trait` | "এই trait implement করে এমন কোনো type" বোঝানোর syntax | 10 |
| `where` clause | `where` clause | signature-এর পরে trait bound লেখার জায়গা | 10 |
| syntax sugar | syntax sugar | লেখা সহজ করার সংক্ষিপ্ত রূপ | 10 |
| dangling reference | dangling reference | অবৈধ ডেটার দিকে নির্দেশ করা reference | 10 |
| lifetime annotation | lifetime annotation | reference-এ lifetime স্পষ্ট করে লেখা (`'a`) | 10 |
| lifetime elision rules | lifetime elision rules | lifetime বাদ দিয়ে লেখার নিয়ম | 10 |
| input / output lifetime | input / output lifetime | parameter / return মানের lifetime | 10 |
| `'static` lifetime | `'static` lifetime | পুরো প্রোগ্রাম জুড়ে বেঁচে থাকতে পারে এমন lifetime | 10 |
| UpperCamelCase | UpperCamelCase | প্রতিটি শব্দের প্রথম অক্ষর বড় হাতের, মাঝে ফাঁক নেই | 10 |
| inheritance | inheritance | উত্তরাধিকার | 10 |
| dynamically typed | dynamically typed | চলার সময় type যাচাই হয় এমন | 10 |
| deallocate | deallocate করা | মেমোরি মুক্ত করা | 10 |
| test | test | কোড ঠিকমতো কাজ করছে কি না যাচাই করার function | 11 |
| assert / assertion | assert করা / assertion | শর্তটি সত্য কি না জোর দিয়ে যাচাই করা | 11 |
| test runner | test runner | test চালানোর প্রোগ্রাম | 11 |
| test harness | test harness | test চালানোর কাঠামো | 11 |
| unit test | unit test | একটি module-কে আলাদা করে test করা | 11 |
| integration test | integration test | library-র বাইরে থেকে public API দিয়ে test করা | 11 |
| test suite | test suite | সব test-এর সংগ্রহ | 11 |
| correctness | correctness | সঠিকতা | 11 |
| filtering (tests) | filtering | নাম মিলিয়ে কিছু test ছেঁকে নেওয়া | 11 |
| benchmark test | benchmark test | performance মাপার test | 11 |
| documentation test (doc test) | documentation test | documentation-এর কোড উদাহরণ চালিয়ে করা test | 11 |
| race condition | race condition | একসঙ্গে চলা কাজের মধ্যে প্রতিযোগিতাজনিত সমস্যা | 11 |
| thread-safe | thread-safe | একাধিক thread থেকে একসঙ্গে ব্যবহার করা নিরাপদ | 11 |
| parallel | parallel-ভাবে | একসঙ্গে (সমান্তরালে) | 11 |
| configuration (`cfg`) | configuration | কোন অবস্থায় কোড যুক্ত হবে তার বিকল্প | 11 |
| test-driven development (TDD) | test-driven development | আগে test লিখে তারপর কোড লেখার পদ্ধতি | 12 |
| test coverage | test coverage | কোডের কতটা অংশ test-এর আওতায় | 12 |
| case-insensitive / case-sensitive | case-insensitive / case-sensitive | বড়-ছোট হাতের অক্ষরের পার্থক্য উপেক্ষা করে / মেনে | 12 |
| exit status (code) | exit status | প্রোগ্রাম শেষে ফেরত দেওয়া সংখ্যা (0 মানে সফল) | 12 |
| separation of concerns | দায়িত্ব আলাদা করা (separation of concerns) | প্রতিটি অংশকে একটি কাজের দায়িত্ব দেওয়া | 12 |
| modular / modularity | modular / modularity | আলাদা আলাদা অংশে ভাগ করা | 12 |
| redirect (output) | অন্যত্র পাঠানো (redirect) | output স্ক্রিনের বদলে ফাইলে পাঠানো | 12 |
| capture (a value) | capture করা | closure-এর বাইরের মান ধরে রাখা | 13 |
| environment (of a closure) | environment | closure যেখানে সংজ্ঞায়িত, সেখানকার পারিপার্শ্বিক মানগুলো | 13 |
| `Fn` / `FnMut` / `FnOnce` traits | `Fn` traits | closure কীভাবে call করা যায় তা বোঝানো তিনটি trait | 13 |
| `move` closure | `move` closure | capture করা মানের ownership নিয়ে নেয় এমন closure | 13 |
| lazy (iterator) | lazy | consume না করা পর্যন্ত কোনো কাজ করে না | 13 |
| consume (an iterator) | consume করা | iterator খরচ করে ফেলা | 13 |
| consuming adapter | consuming adapter | iterator খরচ করে এমন method (যেমন `sum`) | 13 |
| iterator adapter | iterator adapter | এক iterator থেকে আরেক iterator তৈরি করে এমন method (যেমন `map`) | 13 |
| loop unrolling | loop unrolling | loop খুলে পুনরাবৃত্তিগুলো সরাসরি লিখে দেওয়ার optimization | 13 |
| top-level function | top-level function | কোনো কিছুর ভেতরে নয়, সরাসরি module-এ সংজ্ঞায়িত function | 13 |
| release profile | release profile | compile-এর বিকল্পগুলোর পূর্বনির্ধারিত সেট (`dev`, `release`) | 14 |
| optimization (`opt-level`) | optimization | কোড দ্রুত চালাতে compiler-এর করা উন্নয়ন | 14 |
| yank | yank করা | প্রকাশিত version নতুন project-এর জন্য তুলে নেওয়া (মুছে না ফেলে) | 14 |
| binary target | binary target | crate থেকে তৈরি হওয়া চালানো যায় এমন প্রোগ্রাম | 14 |
| path dependency | path dependency | স্থানীয় path দিয়ে উল্লেখ করা dependency | 14 |
| API token | API token | account-এর পরিচয় প্রমাণের গোপন চাবি | 14 |
| license identifier | license identifier | license শনাক্তকারী মান (যেমন `MIT`) | 14 |
| subcommand | subcommand | `cargo`-র অধীনের command (যেমন `cargo build`) | 14 |
| box (`Box<T>`) | box | heap-এ মান রাখার সবচেয়ে সরল smart pointer | 15 |
| recursive type | recursive type | নিজের ভেতরে নিজেকেই ধারণ করে এমন type | 15 |
| cons list | cons list | Lisp থেকে আসা, জোড়ার ভেতরে জোড়া দিয়ে তৈরি তালিকা | 15 |
| indirection | indirection | মান সরাসরি না রেখে pointer দিয়ে পরোক্ষভাবে রাখা | 15 |
| interior mutability | interior mutability | immutable reference থাকা অবস্থাতেও ভেতরের মান পরিবর্তন করার pattern | 15 |
| reference cycle | reference cycle | একে অপরকে নির্দেশ করা reference-এর চক্র | 15 |
| memory leak | memory leak | মেমোরি ফুটো হয়ে যাওয়া: কখনো পরিষ্কার না হওয়া মেমোরি | 15 |
| strong / weak reference | strong / weak reference | ownership ভাগ করে / করে না এমন reference (`Rc<T>` / `Weak<T>`) | 15 |
| test double / mock object | test double / mock object | test-এর সময় আসল type-এর বদলি / যা ঘটছে তা লিখে রাখে এমন বদলি | 15 |
| destructor | destructor | instance পরিষ্কার করে এমন function | 15 |
| double free | double free | একই মেমোরি দুবার মুক্ত করার error | 15 |
| static analysis | static analysis | কোড না চালিয়ে বিশ্লেষণ | 15 |
| deep copy | deep copy | পুরো ডেটার সম্পূর্ণ কপি | 15 |
| single-threaded | single-threaded | একটিমাত্র thread-যুক্ত | 15 |
| wrapper type | wrapper type | অন্য মানকে মুড়ে রাখে এমন type | 15 |
| stack overflow | stack overflow | stack-এর জায়গা ফুরিয়ে যাওয়া | 15 |
| tree (data structure) | tree | গাছের মতো parent-child কাঠামো | 15 |
| fearless concurrency | fearless concurrency | নির্ভয় concurrency: compile time-এই concurrency বাগ ধরা | 16 |
| concurrent / parallel programming | concurrent / parallel programming | অংশগুলো স্বাধীনভাবে চলে / একই সময়ে চলে এমন প্রোগ্রামিং | 16 |
| process | process | operating system-এ চলমান একটি প্রোগ্রাম | 16 |
| deadlock | deadlock | অচলাবস্থা: দুটি thread একে অপরের জন্য চিরকাল অপেক্ষা করে | 16 |
| spawn (a thread) | spawn করা / চালু করা | নতুন thread চালু করা | 16 |
| join handle (`JoinHandle<T>`) | join handle | thread শেষ হওয়ার জন্য অপেক্ষা করতে দেয় এমন মান | 16 |
| block (a thread) | block করা | thread-কে কাজ করা বা বেরোনো থেকে আটকে রাখা | 16 |
| message passing | message passing | বার্তা আদান-প্রদান করে thread-এর যোগাযোগ | 16 |
| channel (transmitter / receiver) | channel | এক thread থেকে আরেক thread-এ ডেটা পাঠানোর পথ (প্রেরক / গ্রাহক) | 16 |
| shared-state concurrency | shared-state concurrency | একাধিক thread একই ডেটায় প্রবেশ করে এমন concurrency | 16 |
| mutex (mutual exclusion) | mutex | পারস্পরিক বর্জন: একবারে একটি thread-কে ডেটায় ঢুকতে দেয় | 16 |
| lock | lock | mutex-এর ডেটায় একচেটিয়া প্রবেশাধিকারের ব্যবস্থা | 16 |
| concurrency primitive | concurrency primitive | concurrency-র মৌলিক গঠন (যেমন mutex, atomic) | 16 |
| atomic | atomic | অবিভাজ্যভাবে সম্পন্ন, thread-এর মধ্যে নিরাপদ | 16 |
| marker trait | marker trait | method ছাড়া শুধু একটি বৈশিষ্ট্য চিহ্নিত করে এমন trait (`Send`, `Sync`) | 16 |
| asynchronous programming (async) | asynchronous programming | সম্ভাব্য থামার জায়গা আর পরে পাওয়া ফলের ভাষায় কোড লেখা | 17 |
| await (a future) | await করা | future প্রস্তুত হওয়ার জন্য অপেক্ষা করা | 17 |
| polling | polling | future-এর মান পাওয়া যাচ্ছে কি না যাচাই করা | 17 |
| await point | await point | যেখানে নিয়ন্ত্রণ runtime-এর হাতে ফেরে | 17 |
| executor | executor | runtime-এর যে অংশ async কোড চালায় | 17 |
| state machine | state machine | অবস্থা পরিবর্তনের যন্ত্র | 17 |
| cooperative multitasking | cooperative multitasking | প্রতিটি কাজ নিজেই ঠিক করে কখন নিয়ন্ত্রণ ছাড়বে | 17 |
| work stealing | work stealing | ব্যস্ততা অনুযায়ী task-কে thread থেকে thread-এ সরানো | 17 |
| pin (`Pin`) / `Unpin` | pin করা / `Unpin` | মান মেমোরিতে সরতে না দেওয়া / সরানো নিরাপদ চিহ্ন | 17 |
| self-referential type | self-referential type | নিজেকেই reference করে এমন type | 17 |
| CPU-bound / I/O-bound | CPU-bound / I/O-bound | হিসাবের গতি / input-output-এর গতির ওপর নির্ভরশীল | 17 |
| serial | serial | ক্রমানুসারী, একটির পর একটি | 17 |
| blocking / non-blocking | blocking / non-blocking | অগ্রগতি আটকে রাখে / রাখে না এমন | 17 |
| object | object | ডেটা আর সেই ডেটার ওপর কাজ করা method একসঙ্গে | 18 |
| encapsulation | encapsulation | implementation-এর খুঁটিনাটি লুকিয়ে রাখা (আবদ্ধকরণ) | 18 |
| polymorphism | polymorphism | বহুরূপতা: একাধিক type-এর ডেটা নিয়ে কাজ করা কোড | 18 |
| subclass / superclass | subclass / superclass | উত্তরাধিকারী class / parent class | 18 |
| design pattern | design pattern | প্রচলিত সমস্যার পুনর্ব্যবহারযোগ্য নকশা-সমাধান | 18 |
| state pattern / state object | state pattern / state object | অবস্থা অনুযায়ী আচরণ বদলানোর pattern / একেকটি অবস্থা বোঝানো object | 18 |
| duck typing | duck typing | type নয়, কী কী method আছে তা দিয়ে মান বিচার করা | 18 |
| static / dynamic dispatch | static / dynamic dispatch | compile time-এ / runtime-এ কোন method call হবে তা ঠিক হওয়া | 18 |
| dyn compatibility | dyn compatibility | কোন trait trait object হিসেবে ব্যবহার করা যায় তার নিয়ম | 18 |
| inline (a function) | inline করা | call-এর জায়গায় function-এর কোড সরাসরি বসানো | 18 |
| delegate | delegate করা | কাজ অন্য method-এর হাতে তুলে দেওয়া | 18 |
| refutable / irrefutable pattern | refutable / irrefutable | মেলাতে ব্যর্থ হতে পারে / পারে না এমন pattern | 19 |
| match guard | match guard | match arm-এর pattern-এর পরে বাড়তি `if` শর্ত | 19 |
| `@` binding | `@` binding | pattern যাচাইয়ের পাশাপাশি মানটি variable-এ রেখে দেওয়া | 19 |
| nested | nested | একটির ভেতরে আরেকটি | 19 |
| precedence | অগ্রাধিকার (precedence) | কোন অংশ আগে প্রযোজ্য হবে | 19 |
| unsafe superpowers | unsafe superpowers | unsafe Rust-এর পাঁচটি বাড়তি ক্ষমতা | 20 |
| Foreign Function Interface (FFI) | FFI | অন্য ভাষার function call করার ব্যবস্থা | 20 |
| application binary interface (ABI) | ABI | assembly স্তরে function call করার নিয়ম | 20 |
| name mangling | mangling | compiler-এর function-এর নাম বদলে দেওয়া | 20 |
| static variable | static variable | Rust-এর global variable, মেমোরিতে নির্দিষ্ট ঠিকানা | 20 |
| union | union | একবারে একটি field ব্যবহৃত হয় এমন struct-সদৃশ type | 20 |
| Miri | Miri | undefined behavior খোঁজার Rust-এর দাপ্তরিক tool | 20 |
| lint | lint | সম্ভাব্য সমস্যার সতর্কতা | 20 |
| operator overloading | operator overloading | নিজের type-এর জন্য `+`-এর মতো operator-এর আচরণ সংজ্ঞায়িত করা | 20 |
| default type parameter | default type parameter | generic type parameter-এর পূর্বনির্ধারিত type | 20 |
| fully qualified syntax | fully qualified syntax | পূর্ণ পরিচয়সহ syntax (`<Type as Trait>::f`) | 20 |
| supertrait | supertrait | যে trait-এর ওপর আরেকটি trait নির্ভর করে | 20 |
| newtype pattern | newtype pattern | একটি type-কে এক field-এর tuple struct-এ মুড়ে নতুন type বানানো | 20 |
| never type (`!`) | never type | কখনো return করে না এমন function-এর type | 20 |
| diverging function | diverging function | কখনো return করে না এমন function | 20 |
| dynamically sized type (DST) | dynamically sized type | শুধু runtime-এ আকার জানা যায় এমন type | 20 |
| function pointer (`fn`) | function pointer | function-কে মান হিসেবে পাঠানোর type | 20 |
| opaque type | opaque type | অস্বচ্ছ type: যার ভেতরটা দেখা বা লেখা যায় না | 20 |
| declarative / procedural macro | declarative / procedural macro | pattern মিলিয়ে কোড বসায় / কোড নিয়ে কোড তৈরি করে এমন macro | 20 |
| metaprogramming | metaprogramming | কোড নিয়ে কোড লেখা | 20 |
| token / `TokenStream` | token | কোডের ক্ষুদ্রতম একক / token-এর ধারা | 20 |
| reflection | reflection | চলার সময় নিজের গঠন পরীক্ষার ক্ষমতা | 20 |

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
| cache | ক্যাশ | 1 |
| configuration (file) | কনফিগারেশন (ফাইল) | 1 |
| curly brackets / semicolon / parentheses | কার্লি ব্র্যাকেট / সেমিকোলন / বন্ধনী | 1 |
| directory | ডিরেক্টরি | 1 |
| documentation | ডকুমেন্টেশন | 1 |
| file extension | এক্সটেনশন | 1 |
| plugin | প্লাগইন | 1 |
| repository | রিপোজিটরি | 0 |
| script | স্ক্রিপ্ট | 1 |
| shell | শেল | 1 |
| source file / source code | সোর্স ফাইল / সোর্স কোড | 0 |
| terminal | টার্মিনাল | 1 |
| text editor | টেক্সট এডিটর | 1 |
| character (letter) | অক্ষর | 2 |
| comment (in code) | কমেন্ট | 2 |
| crash | ক্র্যাশ করা | 2 |
| indent | ইনডেন্ট | 2 |
| input / output | ইনপুট / আউটপুট | 2 |
| keyboard shortcut / sidebar | কিবোর্ড শর্টকাট / সাইডবার | 2 |
| memory | মেমোরি | 2 |
| random | র‍্যান্ডম | 2 |
| tutorial | টিউটোরিয়াল | 2 |
| warning | সতর্কবার্তা | 1 |
| whitespace | ফাঁকা অক্ষর | 2 |
| condition | শর্ত | 3 |
| decimal / hex / octal / binary / byte | দশমিক / হেক্সাডেসিমাল / অক্টাল / বাইনারি / বাইট | 3 |
| define / definition | সংজ্ঞায়িত করা / সংজ্ঞা | 1 |
| element (of an array/tuple) | উপাদান | 3 |
| emoji | ইমোজি | 3 |
| precision | নির্ভুলতা | 3 |
| semantics | অর্থ ও আচরণ | 3 |
| buffer | বাফার | 4 |
| capacity | ধারণক্ষমতা | 4 |
| in-place | নিজ জায়গাতেই | 4 |
| metadata | মেটাডেটা | 4 |
| safe / unsafe (adjectives) | নিরাপদ / অনিরাপদ | 4 |
| security vulnerability | নিরাপত্তা-দুর্বলতা | 4 |
| client code | ব্যবহারকারী কোড | 5 |
| console | কনসোল | 5 |
| debug / debugging / debugger | ডিবাগ করা / ডিবাগিং / ডিবাগার | 1 |
| end user | শেষ ব্যবহারকারী | 5 |
| format / formatting | ফরম্যাট / ফরম্যাটিং | 1 |
| pixel | পিক্সেল | 5 |
| browser | ব্রাউজার | 6 |
| cent / quarter / penny (US coins) | সেন্ট / quarter (25 সেন্টের মুদ্রা) / penny (1 সেন্টের মুদ্রা) | 6 |
| cursor / scrollbar | কার্সর / স্ক্রলবার | 6 |
| enumerate | একে একে তালিকাভুক্ত করা | 6 |
| hover (the mouse) | মাউস রাখা | 6 |
| filesystem | ফাইল সিস্টেম | 7 |
| shortcut | শর্টকাট | 7 |
| codebase | কোডবেস | 7 |
| front of house / back of house (restaurant) | front of house (সামনের অংশ) / back of house (পেছনের অংশ) | 7 |
| concatenate | জোড়া লাগানো | 8 |
| angle brackets | কৌণিক বন্ধনী | 8 |
| Cyrillic / Devanagari | সিরিলিক / দেবনাগরী | 8 |
| median / mode | median / mode (with an explanation in the sentence) | 8 |
| prototype | prototype (Latin; "প্রোটোটাইপ" also fine) | 9 |
| contract (of a function) | চুক্তি (contract) | 9 |
| question mark operator | প্রশ্নবোধক চিহ্নের operator | 9 |
| abstraction | বিমূর্ত রূপ (abstraction) | 10 |
| local (to a crate) | নিজস্ব (local) | 10 |
| template | template (কাঠামো) | 11 |
| ignored | উপেক্ষিত (ignored) | 11 |
| lowercase / uppercase | ছোট হাতের / বড় হাতের অক্ষর | 12 |
| spoiler alert | আগেভাগে বলে দিই | 13 |
| dual license | দুটি license একসঙ্গে (dual license) | 14 |
| first-come, first-served | আগে এলে আগে পাবেন ভিত্তিতে | 14 |
| resource | resource (সম্পদ, যেমন ফাইল বা network connection) | 15 |
| paradigm | paradigm (ধারা) | 18 |
| design trade-offs | নকশার সুবিধা-অসুবিধা | 18 |

## Progress

Translating from upstream
[cognitive-engineering-lab/rust-book](https://github.com/cognitive-engineering-lab/rust-book)
commit `88250e0392cef0622f318e35469108d68694c1f7`
("Merge pull request #394 from cognitive-engineering-lab/dev").

Each line covers the chapter's `.md` files and the quizzes and listings they
use.

- [x] Chapter 0: Front matter (experiment-intro, title-page, foreword, ch00-00-introduction)
- [x] Chapter 1: Getting Started (ch01-*)
- [x] Chapter 2: Programming a Guessing Game (ch02-*)
- [x] Chapter 3: Common Programming Concepts (ch03-*)
- [x] Chapter 4: Understanding Ownership (ch04-*)
- [x] Chapter 5: Using Structs to Structure Related Data (ch05-*)
- [x] Chapter 6: Enums and Pattern Matching (ch06-*)
- [x] Chapter 7: Packages, Crates, and Modules (ch07-*)
- [x] Chapter 8: Common Collections (ch08-*)
- [x] Chapter 9: Error Handling (ch09-*)
- [x] Chapter 10: Generic Types, Traits, and Lifetimes (ch10-*)
- [x] Chapter 11: Writing Automated Tests (ch11-*)
- [x] Chapter 12: An I/O Project: Building a Command Line Program (ch12-*)
- [x] Chapter 13: Functional Language Features: Iterators and Closures (ch13-*)
- [x] Chapter 14: More about Cargo and Crates.io (ch14-*)
- [x] Chapter 15: Smart Pointers (ch15-*)
- [x] Chapter 16: Fearless Concurrency (ch16-*)
- [x] Chapter 17: Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams (ch17-*)
- [x] Chapter 18: Object Oriented Programming Features (ch18-*)
- [x] Chapter 19: Patterns and Matching (ch19-*)
- [x] Chapter 20: Advanced Features (ch20-*)
- [ ] Chapter 21: Final Project: Building a Multithreaded Web Server (ch21-*)
- [ ] End of Experiment (end-of-experiment)
- [ ] Appendix (appendix-00 to appendix-07)

## ইনস্টলেশন {#installation}

প্রথম ধাপ হলো Rust ইনস্টল করা। আমরা Rust ডাউনলোড করব `rustup` দিয়ে; এটি
Rust-এর বিভিন্ন সংস্করণ আর সংশ্লিষ্ট টুলগুলো সামলানোর একটি কমান্ড লাইন টুল।
ডাউনলোডের জন্য ইন্টারনেট সংযোগ লাগবে।

> Note: কোনো কারণে `rustup` ব্যবহার করতে না চাইলে আরও বিকল্পের জন্য
> [Rust ইনস্টলের অন্যান্য পদ্ধতির পাতাটি][otherinstall] দেখুন।

নিচের ধাপগুলো Rust compiler (সংকলক)-এর সর্বশেষ stable সংস্করণ (স্থিতিশীল
সংস্করণ) ইনস্টল করে। Rust স্থিতিশীলতার নিশ্চয়তা দেয়, তাই বইয়ের যেসব উদাহরণ
এখন compile (সংকলন) হয়, সেগুলো Rust-এর নতুন সংস্করণেও compile হতে থাকবে।
তবে সংস্করণভেদে আউটপুট সামান্য আলাদা হতে পারে, কারণ Rust প্রায়ই তার error
message আর সতর্কবার্তা উন্নত করে। অর্থাৎ, এই ধাপগুলো দিয়ে Rust-এর যে নতুন
stable সংস্করণই ইনস্টল করুন, এই বইয়ের বিষয়বস্তুর সঙ্গে সেটি ঠিকঠাক কাজ করবে।

> ### কমান্ড লাইনের লেখার নিয়ম {#command-line-notation}
>
> এই অধ্যায়ে এবং পুরো বইজুড়ে আমরা টার্মিনালে ব্যবহৃত কিছু কমান্ড দেখাব।
> টার্মিনালে যে লাইনগুলো আপনাকে লিখতে হবে, সেগুলো সব `$` দিয়ে শুরু। `$`
> চিহ্নটি আপনাকে টাইপ করতে হবে না; এটি কমান্ড লাইনের prompt (কমান্ড লেখার
> জায়গা নির্দেশকারী চিহ্ন), যা প্রতিটি কমান্ডের শুরু বোঝাতে দেখানো হয়। যেসব
> লাইন `$` দিয়ে শুরু হয় না, সেগুলো সাধারণত আগের কমান্ডের আউটপুট দেখায়। এছাড়া
> শুধু PowerShell-এর উদাহরণগুলোতে `$`-এর বদলে `>` ব্যবহার করা হবে।

### Linux বা macOS-এ `rustup` ইনস্টল করা {#installing-rustup-on-linux-or-macos}

Linux বা macOS ব্যবহার করলে একটি টার্মিনাল খুলে নিচের কমান্ডটি লিখুন:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

কমান্ডটি একটি স্ক্রিপ্ট ডাউনলোড করে `rustup` টুলের ইনস্টলেশন শুরু করে, আর
এই টুলটি Rust-এর সর্বশেষ stable সংস্করণ ইনস্টল করে। আপনার কাছে পাসওয়ার্ড চাওয়া
হতে পারে। ইনস্টলেশন সফল হলে নিচের লাইনটি দেখা যাবে:

```text
Rust is installed now. Great!
```

আপনার একটি _linker_-ও লাগবে। এটি এমন একটি প্রোগ্রাম, যা দিয়ে Rust তার compile
করা আউটপুটগুলো জুড়ে একটি ফাইল বানায়।
খুব সম্ভবত আপনার কম্পিউটারে এটি আগে থেকেই আছে। linker-সংক্রান্ত error পেলে একটি
C compiler ইনস্টল করুন; সাধারণত এর সঙ্গে একটি linker-ও থাকে। C compiler আরেকটি
কারণেও কাজের: Rust-এর কিছু বহুল ব্যবহৃত package (প্যাকেজ, কোডের বিতরণযোগ্য একক)
C কোডের ওপর নির্ভর করে, তাই সেগুলোর জন্য C compiler লাগে।

macOS-এ নিচের কমান্ড চালিয়ে একটি C compiler পেতে পারেন:

```console
$ xcode-select --install
```

Linux ব্যবহারকারীদের সাধারণত নিজেদের ডিস্ট্রিবিউশনের ডকুমেন্টেশন অনুযায়ী GCC বা
Clang ইনস্টল করা উচিত। যেমন, Ubuntu ব্যবহার করলে `build-essential` package-টি
ইনস্টল করতে পারেন।

### Windows-এ `rustup` ইনস্টল করা {#installing-rustup-on-windows}

Windows-এ [https://www.rust-lang.org/tools/install][install]<!-- ignore
--> পাতায় গিয়ে Rust ইনস্টলের নির্দেশনা অনুসরণ করুন। ইনস্টলেশনের একপর্যায়ে
আপনাকে Visual Studio ইনস্টল করতে বলা হবে। এটি একটি linker এবং প্রোগ্রাম compile
করার জন্য দরকারি native library (লাইব্রেরি) সরবরাহ করে। এই ধাপে আরও সাহায্য
লাগলে দেখুন
[https://rust-lang.github.io/rustup/installation/windows-msvc.html][msvc]<!--
ignore -->।

বইয়ের বাকি অংশে এমন কমান্ড ব্যবহার করা হয়েছে, যা _cmd.exe_ ও PowerShell
দুটোতেই কাজ করে। কোথাও নির্দিষ্ট পার্থক্য থাকলে কোনটি ব্যবহার করবেন, তা আমরা
বলে দেব।

### সমস্যা সমাধান {#troubleshooting}

Rust ঠিকমতো ইনস্টল হয়েছে কি না দেখতে একটি শেল খুলে এই লাইনটি লিখুন:

```console
$ rustc --version
```

প্রকাশিত সর্বশেষ stable সংস্করণের সংস্করণ নম্বর, commit hash আর commit-এর তারিখ
নিচের আকারে দেখতে পাবেন:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

এই তথ্য দেখতে পেলে আপনি সফলভাবে Rust ইনস্টল করেছেন! যদি না দেখেন, তাহলে
নিচের উপায়ে দেখে নিন Rust আপনার `PATH`
সিস্টেম variable (মান রাখার নাম)-এ আছে কি না।

Windows CMD-তে লিখুন:

```console
> echo %PATH%
```

PowerShell-এ লিখুন:

```powershell
> echo $env:Path
```

Linux ও macOS-এ লিখুন:

```console
$ echo $PATH
```

সব ঠিক থাকার পরও Rust কাজ না করলে সাহায্য পাওয়ার অনেক জায়গা আছে। অন্য
Rustacean-দের (নিজেদের আমরা মজা করে এই নামে ডাকি) সঙ্গে কীভাবে যোগাযোগ করবেন,
তা জানতে দেখুন [কমিউনিটি পেজ][community]।

### আপডেট ও আনইনস্টল {#updating-and-uninstalling}

`rustup` দিয়ে Rust ইনস্টল করা থাকলে নতুন প্রকাশিত সংস্করণে আপডেট করা সহজ।
আপনার শেল থেকে নিচের আপডেট স্ক্রিপ্টটি চালান:

```console
$ rustup update
```

Rust আর `rustup` আনইনস্টল করতে শেল থেকে নিচের আনইনস্টল স্ক্রিপ্টটি চালান:

```console
$ rustup self uninstall
```

<!-- Old headings. Do not remove or links may break. -->
<a id="local-documentation"></a>

### লোকাল ডকুমেন্টেশন পড়া {#reading-the-local-documentation}

Rust ইনস্টলের সঙ্গে ডকুমেন্টেশনের একটি লোকাল কপিও আসে, যাতে আপনি অফলাইনে
পড়তে পারেন। ব্রাউজারে লোকাল ডকুমেন্টেশন খুলতে `rustup doc` চালান।

standard library (আদর্শ লাইব্রেরি)-র দেওয়া কোনো type (ডেটার ধরন) বা function
(ফাংশন) কী করে বা কীভাবে ব্যবহার করতে হয়, তা নিয়ে নিশ্চিত না হলে যেকোনো সময়
application programming interface (API) ডকুমেন্টেশন দেখে নিন!

<!-- Old headings. Do not remove or links may break. -->
<a id="text-editors-and-integrated-development-environments"></a>

### টেক্সট এডিটর ও IDE ব্যবহার {#using-text-editors-and-ides}

Rust কোড লিখতে আপনি কোন টুল ব্যবহার করবেন, তা নিয়ে এই বই কিছু ধরে নেয় না।
প্রায় যেকোনো টেক্সট এডিটরেই কাজ চলবে! তবে অনেক টেক্সট এডিটর আর integrated
development environment (IDE)-এ Rust-এর জন্য বিল্ট-ইন সাপোর্ট আছে। Rust
ওয়েবসাইটের [টুলস
পেজে][tools] অনেক এডিটর ও IDE-র মোটামুটি হালনাগাদ একটি তালিকা সবসময় পাবেন।

### এই বই নিয়ে অফলাইনে কাজ করা {#working-offline-with-this-book}

কয়েকটি উদাহরণে আমরা standard library-র বাইরের Rust package ব্যবহার করব। সেই
উদাহরণগুলো করতে হলে হয় ইন্টারনেট সংযোগ লাগবে, নয়তো সেই dependency (নির্ভরতা,
অর্থাৎ যে লাইব্রেরির ওপর কোড নির্ভর করে)-গুলো আগে থেকেই ডাউনলোড করে রাখতে
হবে। আগে থেকে ডাউনলোড করতে নিচের কমান্ডগুলো চালাতে পারেন। (`cargo` কী, আর
এই কমান্ডগুলোর প্রতিটি কী করে, তা আমরা পরে বিস্তারিত বলব।)

```console
$ cargo new get-dependencies
$ cd get-dependencies
$ cargo add rand@0.8.5 trpl@0.2.0
```

এতে এই package-গুলো ডাউনলোড হয়ে ক্যাশে জমা থাকবে, ফলে পরে আর ডাউনলোড করতে
হবে না। কমান্ডটি একবার চালানোর পর `get-dependencies` ফোল্ডারটি রাখার দরকার নেই।
এই কমান্ড চালিয়ে থাকলে বইয়ের বাকি অংশে সব `cargo` কমান্ডের সঙ্গে `--offline`
flag (কমান্ডের সঙ্গে দেওয়া অপশন) ব্যবহার করতে পারেন; তখন নেটওয়ার্ক ব্যবহারের
চেষ্টা না করে ক্যাশে রাখা সংস্করণগুলোই ব্যবহার হবে।

{{#quiz ../quizzes/ch01-01-installation.toml}}

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools

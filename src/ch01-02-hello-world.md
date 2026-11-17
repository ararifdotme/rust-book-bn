## Hello, World! {#hello-world}

Rust ইনস্টল তো হলো, এবার আপনার প্রথম Rust প্রোগ্রাম লেখার পালা। নতুন কোনো
ভাষা শেখার সময় স্ক্রিনে `Hello, world!` লেখাটি প্রিন্ট করে এমন একটি ছোট প্রোগ্রাম
লেখার রেওয়াজ আছে, তাই আমরাও এখানে তা-ই করব!

> Note: এই বই ধরে নেয় যে কমান্ড লাইনের সঙ্গে আপনার প্রাথমিক পরিচয় আছে। আপনি
> কীভাবে কোড লিখবেন, কোন টুল ব্যবহার করবেন বা কোড কোথায় রাখবেন, তা নিয়ে
> Rust-এর নির্দিষ্ট কোনো দাবি নেই; তাই কমান্ড লাইনের বদলে IDE ব্যবহার করতে
> চাইলে নির্দ্বিধায় নিজের পছন্দের IDE ব্যবহার করুন। এখন অনেক IDE-তেই কমবেশি
> Rust সাপোর্ট আছে; বিস্তারিত জানতে IDE-র ডকুমেন্টেশন দেখুন। Rust টিম
> `rust-analyzer`-এর মাধ্যমে চমৎকার IDE সাপোর্ট দেওয়ার দিকে মনোযোগ দিচ্ছে। আরও
> জানতে [পরিশিষ্ট D][devtools]<!-- ignore --> দেখুন।

<!-- Old headings. Do not remove or links may break. -->
<a id="creating-a-project-directory"></a>

### প্রজেক্ট ডিরেক্টরি তৈরি {#project-directory-setup}

শুরুতে আপনার Rust কোড রাখার জন্য একটি ডিরেক্টরি বানাবেন। আপনার কোড কোথায়
থাকে, Rust-এর কাছে তাতে কিছু যায় আসে না; তবে এই বইয়ের অনুশীলনী আর
প্রজেক্টগুলোর জন্য আমাদের পরামর্শ: আপনার হোম ডিরেক্টরিতে একটি _projects_
ডিরেক্টরি বানিয়ে সব প্রজেক্ট সেখানে রাখুন।

একটি টার্মিনাল খুলে নিচের কমান্ডগুলো লিখুন। এগুলো একটি _projects_ ডিরেক্টরি
বানাবে, আর _projects_-এর ভেতরে “Hello, world!” প্রজেক্টের জন্য আরেকটি ডিরেক্টরি
বানাবে।

Linux, macOS এবং Windows-এর PowerShell-এ এটি লিখুন:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windows CMD-তে এটি লিখুন:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-and-running-a-rust-program"></a>

### Rust প্রোগ্রামের প্রাথমিক কথা {#rust-program-basics}

এরপর একটি নতুন সোর্স ফাইল বানিয়ে তার নাম দিন _main.rs_। Rust ফাইলের নাম
সবসময় _.rs_ এক্সটেনশনে শেষ হয়। ফাইলের নামে একাধিক শব্দ থাকলে শব্দগুলোর মাঝে
আন্ডারস্কোর দেওয়াই রীতি। যেমন, _helloworld.rs_ না লিখে
_hello_world.rs_ লিখুন।

এবার সদ্য বানানো _main.rs_ ফাইলটি খুলে Listing 1-1-এর কোডটি লিখুন।

<Listing number="1-1" file-name="main.rs" caption="`Hello, world!` প্রিন্ট করে এমন একটি প্রোগ্রাম">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

ফাইলটি সেভ করে _~/projects/hello_world_ ডিরেক্টরিতে খোলা টার্মিনাল উইন্ডোতে
ফিরে যান। Linux বা macOS-এ ফাইলটি compile করে চালাতে নিচের কমান্ডগুলো
লিখুন:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

Windows-এ `./main`-এর বদলে `.\main` কমান্ডটি লিখুন:

```powershell
> rustc main.rs
> .\main
Hello, world!
```

আপনার অপারেটিং সিস্টেম যেটাই হোক, টার্মিনালে `Hello, world!` string (লেখা)-টি
প্রিন্ট হওয়ার কথা। এই আউটপুট না দেখলে সাহায্য পাওয়ার উপায় জানতে ইনস্টলেশন
অংশের [“সমস্যা সমাধান”][troubleshooting]<!-- ignore --> অংশে ফিরে যান।

`Hello, world!` প্রিন্ট হয়ে থাকলে অভিনন্দন! আপনি আনুষ্ঠানিকভাবে একটি Rust
প্রোগ্রাম লিখে ফেলেছেন। তার মানে আপনি এখন একজন Rust প্রোগ্রামার—স্বাগতম!

<!-- Old headings. Do not remove or links may break. -->

<a id="anatomy-of-a-rust-program"></a>

### একটি Rust প্রোগ্রামের গঠন {#the-anatomy-of-a-rust-program}

চলুন, এই “Hello, world!” প্রোগ্রামটি খুঁটিয়ে দেখি। ধাঁধার প্রথম টুকরোটি এই:

```rust
fn main() {

}
```

এই লাইনগুলো `main` নামের একটি function সংজ্ঞায়িত করে। `main` function
বিশেষ: প্রতিটি executable (চালানো যায় এমন ফাইল) Rust প্রোগ্রামে এটিই সবসময়
সবার আগে চলে। এখানে প্রথম লাইনটি `main` নামের এমন একটি function declare
(ঘোষণা) করে, যার কোনো parameter (function-এ পাঠানো মানের জন্য রাখা নাম) নেই
এবং যা কোনো মান ফেরত দেয় না। parameter থাকলে সেগুলো বন্ধনী (`()`)-র ভেতরে বসত।

function body (function-এর মূল অংশ) `{}` দিয়ে ঘেরা থাকে। Rust-এ প্রতিটি
function body-র চারপাশে কার্লি ব্র্যাকেট দেওয়া বাধ্যতামূলক। শুরুর কার্লি
ব্র্যাকেটটি function ঘোষণার লাইনেই, মাঝে একটি ফাঁকা জায়গা রেখে বসানো ভালো রীতি।

> Note: সব Rust প্রজেক্টে একই আদর্শ স্টাইল মেনে চলতে চাইলে `rustfmt` নামের
> স্বয়ংক্রিয় ফরম্যাটার টুল দিয়ে কোডকে একটি নির্দিষ্ট স্টাইলে সাজিয়ে নিতে পারেন
> (`rustfmt` নিয়ে আরও আছে
> [পরিশিষ্ট D][devtools]<!-- ignore -->-তে)। `rustc`-এর মতো এই টুলটিও Rust
> টিম Rust-এর সাধারণ ইনস্টলেশনের সঙ্গেই দিয়েছে, তাই এটি সম্ভবত আপনার
> কম্পিউটারে আগে থেকেই ইনস্টল করা আছে!

`main` function-এর body-তে আছে নিচের কোড:

```rust
println!("Hello, world!");
```

এই লাইনটিই এই ছোট প্রোগ্রামের সব কাজ করে: এটি স্ক্রিনে লেখা প্রিন্ট করে। এখানে
তিনটি গুরুত্বপূর্ণ বিষয় লক্ষ করার মতো।

প্রথমত, `println!` একটি Rust macro (কোড তৈরি করে এমন কোড) call করে। এটি যদি
macro-র বদলে কোনো function call করত, তাহলে লেখা হতো `println` (`!` ছাড়া)।
Rust macro দিয়ে এমন কোড লেখা যায়, যা নিজেই কোড তৈরি করে Rust-এর syntax (ভাষার
লিখনরীতি) প্রসারিত করে; [অধ্যায় 20][ch20-macros]<!-- ignore -->-এ এগুলো নিয়ে
বিস্তারিত আলোচনা করব। আপাতত এটুকু জানলেই চলবে যে `!` ব্যবহার করা মানে আপনি
সাধারণ function নয়, macro call করছেন, আর macro সবসময় function-এর নিয়ম মেনে
চলে না।

দ্বিতীয়ত, আপনি `"Hello, world!"` string দেখছেন। আমরা এই string-টি `println!`-কে
argument (function-এ পাঠানো আসল মান) হিসেবে দিই, আর string-টি স্ক্রিনে প্রিন্ট
হয়।

তৃতীয়ত, লাইনটি আমরা সেমিকোলন (`;`) দিয়ে শেষ করি, যা বোঝায় যে এই expression
(মান তৈরি করে এমন কোড-অংশ) শেষ, আর পরেরটি শুরু হতে প্রস্তুত। Rust কোডের
বেশিরভাগ লাইন সেমিকোলন দিয়ে শেষ হয়।

<!-- Old headings. Do not remove or links may break. -->
<a id="compiling-and-running-are-separate-steps"></a>

### প্রোগ্রাম compile করা ও চালানো {#compilation-and-execution}

আপনি এইমাত্র নতুন বানানো একটি প্রোগ্রাম চালিয়েছেন, তাই চলুন প্রক্রিয়াটির প্রতিটি
ধাপ দেখি।

Rust প্রোগ্রাম চালানোর আগে Rust compiler দিয়ে সেটি compile করতে হয়:
`rustc` কমান্ড লিখে তার সঙ্গে আপনার সোর্স ফাইলের নাম দিন, এভাবে:

```console
$ rustc main.rs
```

আপনার C বা C++ জানা থাকলে খেয়াল করবেন, এটি `gcc` বা `clang`-এর মতোই।
সফলভাবে compile হওয়ার পর Rust একটি binary executable তৈরি করে।

Linux, macOS এবং Windows-এর PowerShell-এ শেলে `ls` কমান্ড লিখে executable
ফাইলটি দেখতে পারেন:

```console
$ ls
main  main.rs
```

Linux ও macOS-এ দুটি ফাইল দেখবেন। Windows-এর PowerShell-এ দেখবেন সেই একই তিনটি
ফাইল, যা CMD-তে দেখতেন। Windows-এর CMD-তে আপনি লিখবেন:

```cmd
> dir /B %= the /B option says to only show the file names =%
main.exe
main.pdb
main.rs
```

এখানে দেখা যাচ্ছে _.rs_ এক্সটেনশনের সোর্স কোড ফাইল, executable ফাইল (Windows-এ
_main.exe_, অন্য সব প্ল্যাটফর্মে _main_), আর Windows ব্যবহার করলে _.pdb_
এক্সটেনশনের একটি ফাইল, যাতে ডিবাগিংয়ের তথ্য থাকে। এখান থেকে _main_ বা
_main.exe_ ফাইলটি চালান, এভাবে:

```console
$ ./main # or .\main on Windows
```

আপনার _main.rs_ যদি “Hello, world!” প্রোগ্রাম হয়, তাহলে এই লাইনটি টার্মিনালে
`Hello,
world!` প্রিন্ট করবে।

আপনি যদি Ruby, Python বা JavaScript-এর মতো কোনো dynamic language (সরাসরি চালানো
ভাষা)-য় বেশি অভ্যস্ত হন, তাহলে প্রোগ্রাম compile করা আর চালানোকে আলাদা ধাপ
হিসেবে দেখতে হয়তো অভ্যস্ত নন। Rust একটি _ahead-of-time compiled_ (চালানোর আগেই
সংকলিত) ভাষা, অর্থাৎ আপনি একটি প্রোগ্রাম compile করে executable ফাইলটি অন্য
কাউকে দিতে পারেন, আর তার কম্পিউটারে Rust ইনস্টল করা না থাকলেও সে সেটি চালাতে
পারবে। কিন্তু কাউকে _.rb_, _.py_ বা _.js_ ফাইল দিলে তার কম্পিউটারে যথাক্রমে
Ruby, Python বা JavaScript চালানোর সফটওয়্যার ইনস্টল করা থাকতে হবে। তবে ওই
ভাষাগুলোতে প্রোগ্রাম compile করে চালাতে একটিমাত্র কমান্ডই লাগে। ভাষার নকশায়
সবকিছুতেই কিছু পেতে হলে কিছু ছাড়তে হয়।

সাধারণ প্রোগ্রামের জন্য শুধু `rustc` দিয়ে compile করাই যথেষ্ট, কিন্তু প্রজেক্ট
বড় হতে থাকলে আপনি সব অপশন গুছিয়ে সামলাতে চাইবেন, আর কোড অন্যদের সঙ্গে ভাগ
করে নেওয়াও সহজ করতে চাইবেন। এরপর আমরা Cargo টুলের সঙ্গে পরিচয় করিয়ে দেব, যা
আপনাকে বাস্তব কাজের উপযোগী Rust প্রোগ্রাম লিখতে সাহায্য করবে।

{{#quiz ../quizzes/ch01-02-hello-world.toml}}


[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
[ch20-macros]: ch20-05-macros.html

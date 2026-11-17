## Hello, Cargo! {#hello-cargo}

Cargo হলো Rust-এর build system ও package manager। বেশিরভাগ Rustacean নিজেদের
Rust প্রজেক্ট সামলাতে এই টুল ব্যবহার করেন, কারণ Cargo আপনার হয়ে অনেক কাজ করে
দেয়, যেমন আপনার কোড build (কোড compile করে চালানোর উপযোগী ফাইল বানানো) করা,
আপনার কোড যেসব library-র ওপর নির্ভর করে সেগুলো ডাউনলোড করা, এবং সেই
library-গুলো build করা। (আপনার কোডের দরকারি এই library-গুলোকে আমরা
_dependency_ বলি।)

এখন পর্যন্ত আমরা যে ধরনের প্রোগ্রাম লিখেছি, সেরকম সবচেয়ে সরল Rust প্রোগ্রামের
কোনো dependency থাকে না। “Hello, world!” প্রজেক্টটি Cargo দিয়ে build করলে সেটি
Cargo-র শুধু সেই অংশটুকু ব্যবহার করত, যা আপনার কোড build করে। আরও জটিল Rust
প্রোগ্রাম লেখার সময় আপনি dependency যোগ করবেন, আর প্রজেক্টটি Cargo দিয়ে শুরু
করলে dependency যোগ করা অনেক সহজ হবে।

যেহেতু বেশিরভাগ Rust প্রজেক্টই Cargo ব্যবহার করে, বইয়ের বাকি অংশ ধরে নেয় যে
আপনিও Cargo ব্যবহার করছেন।
[“ইনস্টলেশন”][installation]<!-- ignore --> অংশে আলোচিত অফিসিয়াল installer
ব্যবহার করে থাকলে Cargo Rust-এর সঙ্গেই ইনস্টল হয়ে গেছে। অন্য কোনো উপায়ে Rust
ইনস্টল করে থাকলে টার্মিনালে নিচের কমান্ডটি লিখে দেখুন Cargo ইনস্টল আছে কি না:

```console
$ cargo --version
```

কোনো সংস্করণ নম্বর দেখলে বুঝবেন, Cargo আছে! আর `command
not found`-এর মতো কোনো error দেখলে, Cargo আলাদাভাবে কীভাবে ইনস্টল করবেন তা
জানতে আপনার ইনস্টলেশন পদ্ধতির ডকুমেন্টেশন দেখুন।

### Cargo দিয়ে প্রজেক্ট তৈরি {#creating-a-project-with-cargo}

চলুন Cargo দিয়ে একটি নতুন প্রজেক্ট বানাই এবং দেখি সেটি আমাদের আগের
“Hello, world!” প্রজেক্ট থেকে কোথায় আলাদা। আপনার _projects_ ডিরেক্টরিতে (অথবা
কোড রাখার জন্য যে জায়গা ঠিক করেছেন, সেখানে) ফিরে যান। তারপর যেকোনো অপারেটিং
সিস্টেমে নিচের কমান্ডগুলো চালান:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

প্রথম কমান্ডটি _hello_cargo_ নামে একটি নতুন ডিরেক্টরি ও প্রজেক্ট তৈরি করে।
আমরা প্রজেক্টের নাম দিয়েছি _hello_cargo_, আর Cargo একই নামের একটি ডিরেক্টরিতে
এর ফাইলগুলো তৈরি করে।

_hello_cargo_ ডিরেক্টরিতে গিয়ে ফাইলগুলোর তালিকা দেখুন। দেখবেন, Cargo আমাদের
জন্য দুটি ফাইল আর একটি ডিরেক্টরি তৈরি করেছে: একটি _Cargo.toml_ ফাইল, আর একটি
_src_ ডিরেক্টরি, যার ভেতরে আছে একটি _main.rs_ ফাইল।

এটি একটি _.gitignore_ ফাইলসহ একটি নতুন Git রিপোজিটরিও তৈরি করেছে। আগে থেকে
থাকা কোনো Git রিপোজিটরির ভেতরে `cargo new` চালালে Git-এর ফাইলগুলো তৈরি হবে না;
`cargo new --vcs=git` ব্যবহার করে এই আচরণ বদলাতে পারেন।

> Note: Git একটি বহুল ব্যবহৃত version control system (কোডের পরিবর্তনের ইতিহাস
> রাখার ব্যবস্থা)। `--vcs` flag ব্যবহার করে `cargo new`-কে অন্য কোনো version
> control system ব্যবহার করাতে পারেন, কিংবা কোনো version control system ছাড়াই
> চালাতে পারেন। কোন কোন অপশন আছে দেখতে `cargo new --help` চালান।

_Cargo.toml_ ফাইলটি আপনার পছন্দের টেক্সট এডিটরে খুলুন। এটি Listing 1-2-এর
কোডের মতো দেখাবে।

<Listing number="1-2" file-name="Cargo.toml" caption="`cargo new`-এর তৈরি *Cargo.toml*-এর বিষয়বস্তু">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

এই ফাইলটি [_TOML_][toml]<!-- ignore --> (_Tom’s Obvious, Minimal
Language_) ফরম্যাটে লেখা, যা Cargo-র কনফিগারেশন ফরম্যাট।

প্রথম লাইন `[package]` একটি অংশের শিরোনাম, যা বোঝায় যে পরের লাইনগুলো একটি
package কনফিগার করছে। এই ফাইলে আরও তথ্য যোগ করার সঙ্গে সঙ্গে আমরা অন্য অংশও
যোগ করব।

পরের তিনটি লাইন আপনার প্রোগ্রাম compile করার জন্য Cargo-র দরকারি কনফিগারেশন
তথ্য সেট করে: নাম, সংস্করণ, আর Rust-এর কোন edition (Rust-এর সংস্করণ-ধাপ)
ব্যবহার হবে। `edition` key (key-value জোড়ার নাম অংশ) নিয়ে আমরা
[পরিশিষ্ট E][appendix-e]<!-- ignore -->-তে কথা বলব।

শেষ লাইন `[dependencies]` হলো এমন একটি অংশের শুরু, যেখানে আপনার প্রজেক্টের
dependency-গুলোর তালিকা লিখবেন। Rust-এ কোডের package-কে বলা হয় _crate_
(Rust-এর কোডের একক)। এই প্রজেক্টের জন্য আর কোনো crate লাগবে না, তবে অধ্যায় 2-এর
প্রথম প্রজেক্টে লাগবে; তখন এই dependencies অংশটি ব্যবহার করব।

এবার _src/main.rs_ খুলে একবার দেখুন:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo আপনার জন্য একটি “Hello, world!” প্রোগ্রাম তৈরি করে দিয়েছে, ঠিক যেমনটি
আমরা Listing 1-1-এ লিখেছিলাম! এ পর্যন্ত আমাদের প্রজেক্ট আর Cargo-র তৈরি
প্রজেক্টের পার্থক্য এটুকুই: Cargo কোডটি _src_ ডিরেক্টরিতে রেখেছে, আর মূল
ডিরেক্টরিতে আমাদের একটি _Cargo.toml_ কনফিগারেশন ফাইল আছে।

Cargo আশা করে আপনার সোর্স ফাইলগুলো _src_ ডিরেক্টরির ভেতরে থাকবে। প্রজেক্টের
মূল ডিরেক্টরি শুধু README ফাইল, লাইসেন্সের তথ্য, কনফিগারেশন ফাইল এবং আপনার
কোডের সঙ্গে সম্পর্কহীন অন্যান্য জিনিসের জন্য। Cargo ব্যবহার করলে প্রজেক্ট গুছিয়ে
রাখা সহজ হয়। প্রতিটি জিনিসের একটা নির্দিষ্ট জায়গা আছে, আর প্রতিটি জিনিস তার
জায়গাতেই থাকে।

আপনি যদি এমন কোনো প্রজেক্ট শুরু করে থাকেন, যা Cargo ব্যবহার করে না (যেমনটি
আমরা “Hello, world!” প্রজেক্টে করেছিলাম), তাহলে সেটিকে Cargo-ভিত্তিক প্রজেক্টে
রূপান্তর করতে পারেন। প্রজেক্টের কোড _src_ ডিরেক্টরিতে সরিয়ে একটি উপযুক্ত
_Cargo.toml_ ফাইল তৈরি করুন। _Cargo.toml_ ফাইল পাওয়ার একটি সহজ উপায় হলো
`cargo init` চালানো, যা আপনার জন্য ফাইলটি স্বয়ংক্রিয়ভাবে তৈরি করে দেবে।

### Cargo প্রজেক্ট build করা ও চালানো {#building-and-running-a-cargo-project}

এবার দেখা যাক, Cargo দিয়ে “Hello, world!” প্রোগ্রামটি build করে চালালে কী
পার্থক্য হয়! আপনার _hello_cargo_ ডিরেক্টরি থেকে নিচের কমান্ড লিখে প্রজেক্টটি
build করুন:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

এই কমান্ডটি executable ফাইলটি আপনার বর্তমান ডিরেক্টরিতে না বানিয়ে
_target/debug/hello_cargo_-এ (Windows-এ _target\debug\hello_cargo.exe_-এ) বানায়।
ডিফল্ট build হলো একটি debug build (দ্রুত তৈরি, ডিবাগের উপযোগী build), তাই Cargo
binary-টি _debug_ নামের একটি ডিরেক্টরিতে রাখে। এই কমান্ড দিয়ে executable-টি
চালাতে পারেন:

```console
$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
Hello, world!
```

সব ঠিকঠাক থাকলে টার্মিনালে `Hello, world!` প্রিন্ট হওয়ার কথা। প্রথমবার `cargo
build` চালালে Cargo মূল ডিরেক্টরিতে একটি নতুন ফাইলও তৈরি করে: _Cargo.lock_। এই
ফাইলটি আপনার প্রজেক্টের dependency-গুলোর সঠিক সংস্করণের হিসাব রাখে। এই
প্রজেক্টের কোনো dependency নেই, তাই ফাইলটিতে তেমন কিছু নেই। এই ফাইল আপনাকে কখনো
নিজ হাতে বদলাতে হবে না; Cargo নিজেই এর বিষয়বস্তু সামলায়।

আমরা এইমাত্র `cargo build` দিয়ে একটি প্রজেক্ট build করলাম এবং
`./target/debug/hello_cargo` দিয়ে চালালাম। তবে `cargo run` ব্যবহার করে একটিমাত্র
কমান্ডেই কোড compile করে তৈরি হওয়া executable চালানো যায়:

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

`cargo
build` চালিয়ে তারপর binary-র পুরো পাথ মনে রেখে চালানোর চেয়ে `cargo run` ব্যবহার
করা বেশি সুবিধাজনক, তাই বেশিরভাগ ডেভেলপার `cargo
run` ব্যবহার করেন।

লক্ষ করুন, এবার আমরা এমন কোনো আউটপুট দেখিনি, যা বোঝায় যে Cargo
`hello_cargo` compile করছে। Cargo বুঝতে পেরেছে যে ফাইলগুলো বদলায়নি, তাই সে
আবার build না করে শুধু binary-টি চালিয়েছে। আপনি সোর্স কোড বদলালে Cargo চালানোর
আগে প্রজেক্টটি আবার build করত, আর আপনি এই আউটপুট দেখতেন:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo `cargo check` নামে একটি কমান্ডও দেয়। এই কমান্ড দ্রুত যাচাই করে দেখে যে
আপনার কোড compile হয় কি না, কিন্তু কোনো executable তৈরি করে না:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

executable কেন চাইবেন না? প্রায়ই `cargo check`, `cargo build`-এর চেয়ে অনেক
দ্রুত, কারণ এটি executable তৈরির ধাপটি বাদ দেয়। কোড লেখার সময় বারবার নিজের কাজ
যাচাই করতে থাকলে `cargo check` ব্যবহারে প্রজেক্ট এখনো compile হচ্ছে কি না, তা আরও
দ্রুত জানতে পারবেন! তাই অনেক Rustacean প্রোগ্রাম লেখার সময় মাঝে মাঝে `cargo
check` চালিয়ে দেখে নেন সেটি compile হচ্ছে কি না। তারপর executable ব্যবহারের জন্য
প্রস্তুত হলে `cargo build` চালান।

Cargo সম্পর্কে এ পর্যন্ত যা শিখলাম, চলুন একবার ঝালিয়ে নিই:

- `cargo new` দিয়ে প্রজেক্ট তৈরি করা যায়।
- `cargo build` দিয়ে প্রজেক্ট build করা যায়।
- `cargo run` দিয়ে এক ধাপেই প্রজেক্ট build করে চালানো যায়।
- binary তৈরি না করে শুধু error আছে কি না যাচাই করতে
  `cargo check` দিয়ে প্রজেক্ট build করা যায়।
- build-এর ফলাফল আমাদের কোডের ডিরেক্টরিতে না রেখে Cargo সেটি _target/debug_
  ডিরেক্টরিতে রাখে।

Cargo ব্যবহারের আরেকটি সুবিধা হলো, আপনি যে অপারেটিং সিস্টেমেই কাজ করুন না কেন,
কমান্ডগুলো একই। তাই এখন থেকে আমরা Linux ও macOS বনাম Windows-এর জন্য আলাদা
নির্দেশনা আর দেব না।

### প্রকাশের জন্য build করা {#building-for-release}

আপনার প্রজেক্ট শেষমেশ প্রকাশের জন্য প্রস্তুত হলে `cargo build
--release` দিয়ে optimization (কোড দ্রুততর করা)-সহ compile করতে পারেন। এই কমান্ড
_target/debug_-এর বদলে _target/release_-এ একটি executable তৈরি করবে।
optimization আপনার Rust কোডকে দ্রুত চালায়, কিন্তু এটি চালু করলে প্রোগ্রাম compile
হতে বেশি সময় লাগে। এ কারণেই দুটি আলাদা profile (build-এর সেটিংসের সমষ্টি)
আছে: একটি ডেভেলপমেন্টের জন্য, যখন আপনি দ্রুত ও ঘনঘন আবার build করতে চান; আর
অন্যটি সেই চূড়ান্ত প্রোগ্রাম build করার জন্য, যা আপনি ব্যবহারকারীকে দেবেন, যা
বারবার build করা হবে না এবং যা যত দ্রুত সম্ভব চলবে। আপনার কোড চলতে কত সময় নেয়
তা benchmark (চলার গতি মাপা) করতে চাইলে অবশ্যই `cargo build --release` চালাবেন
এবং _target/release_-এর executable দিয়ে benchmark করবেন।

<!-- Old headings. Do not remove or links may break. -->
<a id="cargo-as-convention"></a>

### Cargo-র রীতিনীতির সুবিধা নেওয়া {#leveraging-cargos-conventions}

সাধারণ প্রজেক্টে শুধু `rustc` ব্যবহারের তুলনায় Cargo খুব বেশি বাড়তি সুবিধা দেয়
না, কিন্তু প্রোগ্রাম যত জটিল হবে, Cargo তত নিজের মূল্য প্রমাণ করবে। প্রোগ্রাম
একাধিক ফাইলে ছড়িয়ে পড়লে বা কোনো dependency লাগলে build-এর সমন্বয়ের দায়িত্ব
Cargo-কে দেওয়াই অনেক সহজ।

`hello_cargo` প্রজেক্টটি সাধারণ হলেও এখন এটি সেই আসল টুলগুলোর অনেকগুলোই
ব্যবহার করছে, যা আপনি Rust নিয়ে পুরো ক্যারিয়ারজুড়ে ব্যবহার করবেন। সত্যি বলতে,
আগে থেকে থাকা যেকোনো প্রজেক্টে কাজ করতে নিচের কমান্ডগুলো দিয়ে Git-এর মাধ্যমে
কোড নামিয়ে আনতে, সেই প্রজেক্টের ডিরেক্টরিতে যেতে এবং build করতে পারেন:

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Cargo সম্পর্কে আরও জানতে [এর ডকুমেন্টেশন][cargo] দেখুন।

{{#quiz ../quizzes/ch01-03-hello-cargo.toml}}


## সারসংক্ষেপ {#summary}

Rust শেখার যাত্রায় আপনার শুরুটা দারুণ হয়েছে! এই অধ্যায়ে আপনি শিখেছেন কীভাবে:

- `rustup` ব্যবহার করে Rust-এর সর্বশেষ stable সংস্করণ ইনস্টল করতে হয়।
- Rust-এর নতুন কোনো সংস্করণে আপডেট করতে হয়।
- কম্পিউটারে ইনস্টল করা ডকুমেন্টেশন খুলতে হয়।
- সরাসরি `rustc` ব্যবহার করে একটি “Hello, world!” প্রোগ্রাম লিখে চালাতে হয়।
- Cargo-র রীতি মেনে একটি নতুন প্রজেক্ট তৈরি করে চালাতে হয়।

Rust কোড পড়া ও লেখায় অভ্যস্ত হতে এবার আরেকটু বড় কোনো প্রোগ্রাম বানানোর উপযুক্ত
সময়। তাই অধ্যায় 2-এ আমরা একটি সংখ্যা আন্দাজ করার খেলা বানাব। আর যদি আগে জানতে
চান প্রোগ্রামিংয়ের সাধারণ ধারণাগুলো Rust-এ কীভাবে কাজ করে, তাহলে অধ্যায় 3 পড়ে
তারপর অধ্যায় 2-এ ফিরে আসুন।

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/

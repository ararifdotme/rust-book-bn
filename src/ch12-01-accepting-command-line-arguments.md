## command line argument গ্রহণ করা {#accepting-command-line-arguments}

বরাবরের মতো `cargo new` দিয়ে একটি নতুন project তৈরি করি। আপনার system-এ হয়তো আগে থেকেই থাকা `grep` tool থেকে আলাদা করতে
আমাদের project-এর নাম দেব `minigrep`:

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

প্রথম কাজ হলো `minigrep`-কে তার দুটি command line argument গ্রহণ করানো: file path আর যে string খুঁজতে হবে সেটি। অর্থাৎ আমরা
চাই প্রোগ্রামটি এভাবে চালাতে পারি: `cargo run`, তারপর দুটি hyphen, যা বোঝায় পরের argument-গুলো `cargo`-র জন্য নয় বরং আমাদের
প্রোগ্রামের জন্য, তারপর খোঁজার string, আর তারপর যে ফাইলে খুঁজতে হবে তার path, এভাবে:

```console
$ cargo run -- searchstring example-filename.txt
```

এই মুহূর্তে `cargo new`-এর তৈরি প্রোগ্রাম আমাদের দেওয়া argument প্রক্রিয়া করতে পারে না। [crates.io](https://crates.io/)-তে
থাকা কিছু library command line argument গ্রহণ করে এমন প্রোগ্রাম লিখতে সাহায্য করতে পারে, কিন্তু আপনি যেহেতু ধারণাটি কেবল
শিখছেন, তাই চলুন এই ক্ষমতাটি নিজেরাই implement করি।

### argument-এর মান পড়া {#reading-the-argument-values}

`minigrep`-কে পাঠানো command line argument-এর মান পড়তে আমাদের Rust standard library-র দেওয়া `std::env::args` function দরকার।
এই function `minigrep`-কে পাঠানো command line argument-গুলোর একটি iterator (একটির পর একটি উপাদান দেয় এমন বস্তু) return করে।
iterator নিয়ে পুরোপুরি আলোচনা করব [অধ্যায় 13][ch13]<!-- ignore
-->-এ। আপাতত iterator সম্পর্কে শুধু দুটি বিষয় জানলেই চলবে: iterator একের পর এক মানের একটি ধারা তৈরি করে, আর একটি iterator-এর
ওপর `collect` method call করে একে একটি collection-এ, যেমন vector-এ, রূপান্তর করা যায়, যাতে iterator-এর তৈরি সব উপাদান থাকে।

Listing 12-1-এর কোড আপনার `minigrep` প্রোগ্রামকে তাকে পাঠানো যেকোনো command line argument পড়তে এবং মানগুলো একটি vector-এ জড়ো
করতে দেয়।

<Listing number="12-1" file-name="src/main.rs" caption="command line argument-গুলো একটি vector-এ জড়ো করে print করা">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

প্রথমে একটি `use` statement দিয়ে `std::env` module-কে scope-এ আনি, যাতে এর `args` function ব্যবহার করতে পারি। খেয়াল করুন,
`std::env::args` function দুই স্তরের module-এর ভেতরে আছে। [অধ্যায়
7][ch7-idiomatic-use]<!-- ignore -->-এ যেমন আলোচনা করেছি, কাঙ্ক্ষিত function একাধিক module-এর ভেতরে থাকলে আমরা function-এর
বদলে parent module-কে scope-এ আনার পথ বেছে নিয়েছি। এতে `std::env`-এর অন্য function-গুলোও সহজে ব্যবহার করা যায়। এটা
`use std::env::args` যোগ করে শুধু `args` দিয়ে function call করার চেয়ে কম অস্পষ্টও, কারণ `args`-কে সহজেই বর্তমান module-এ
সংজ্ঞায়িত কোনো function বলে ভুল হতে পারে।

> ### `args` function ও অবৈধ Unicode {#the-args-function-and-invalid-unicode}
>
> খেয়াল করুন, কোনো argument-এ অবৈধ Unicode থাকলে `std::env::args` panic করবে। আপনার প্রোগ্রামকে যদি অবৈধ Unicode-যুক্ত
> argument গ্রহণ করতে হয়, তাহলে এর বদলে `std::env::args_os` ব্যবহার করুন। ওই function এমন একটি iterator return করে যা
> `String` মানের বদলে `OsString` মান তৈরি করে। সরলতার জন্য আমরা এখানে `std::env::args` ব্যবহার করেছি, কারণ `OsString` মান
> platform অনুযায়ী আলাদা হয় এবং `String` মানের চেয়ে এদের নিয়ে কাজ করা জটিল।

`main`-এর প্রথম লাইনে আমরা `env::args` call করি, আর সঙ্গে সঙ্গে `collect` ব্যবহার করে iterator-টিকে এমন একটি vector-এ রূপান্তর
করি যাতে iterator-এর তৈরি সব মান থাকে। `collect` function দিয়ে অনেক ধরনের collection তৈরি করা যায়, তাই আমরা স্পষ্টভাবে
`args`-এর type annotate করে বলে দিই যে আমরা string-এর একটি vector চাই। Rust-এ type annotate করার দরকার খুব কমই হয়, কিন্তু
`collect` এমন একটি function যাকে প্রায়ই annotate করতে হয়, কারণ আপনি কোন ধরনের collection চান তা Rust অনুমান করতে পারে না।

শেষে debug macro দিয়ে vector-টি print করি। চলুন প্রথমে কোনো argument ছাড়া আর তারপর দুটি argument দিয়ে কোডটি চালিয়ে দেখি:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

খেয়াল করুন, vector-এর প্রথম মান হলো `"target/debug/minigrep"`, যা আমাদের binary-র নাম। এটি C-তে argument তালিকার আচরণের সঙ্গে
মেলে, যেখানে প্রোগ্রাম চলার সময় তাকে যে নামে চালানো হয়েছে সেই নাম ব্যবহার করতে পারে। প্রোগ্রামের নাম হাতে থাকলে প্রায়ই
সুবিধা হয়, যেমন message-এ নামটি print করতে চাইলে, বা প্রোগ্রাম চালাতে কোন command line alias ব্যবহার করা হয়েছে তার ওপর
নির্ভর করে প্রোগ্রামের আচরণ বদলাতে চাইলে। তবে এই অধ্যায়ের উদ্দেশ্যে আমরা এটি উপেক্ষা করব এবং শুধু প্রয়োজনীয় দুটি argument
সংরক্ষণ করব।

### argument-এর মান variable-এ সংরক্ষণ করা {#saving-the-argument-values-in-variables}

প্রোগ্রামটি এখন command line argument হিসেবে দেওয়া মানগুলো পেতে পারে। এবার দুটি argument-এর মান variable-এ সংরক্ষণ করতে হবে,
যাতে প্রোগ্রামের বাকি অংশ জুড়ে মানগুলো ব্যবহার করা যায়। Listing 12-2-এ আমরা সেটাই করি।

<Listing number="12-2" file-name="src/main.rs" caption="query argument আর file path argument রাখার জন্য variable তৈরি করা">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

vector print করার সময় যেমন দেখেছি, প্রোগ্রামের নাম vector-এর প্রথম মান `args[0]`-এর জায়গা নেয়, তাই আমরা argument শুরু করছি
index 1 থেকে। `minigrep` প্রথম যে argument নেয় তা হলো খোঁজার string, তাই প্রথম argument-এর একটি reference রাখি `query`
variable-এ। দ্বিতীয় argument হবে file path, তাই দ্বিতীয় argument-এর একটি reference রাখি `file_path` variable-এ।

কোড আমাদের উদ্দেশ্য অনুযায়ী কাজ করছে তা প্রমাণ করতে আমরা সাময়িকভাবে এই variable-গুলোর মান print করি। চলুন `test` আর
`sample.txt` argument দিয়ে প্রোগ্রামটি আবার চালাই:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

চমৎকার, প্রোগ্রামটি কাজ করছে! প্রয়োজনীয় argument-গুলোর মান সঠিক variable-এ সংরক্ষিত হচ্ছে। পরে আমরা কিছু error handling
যোগ করব, যাতে কিছু সম্ভাব্য ভুল পরিস্থিতি, যেমন ব্যবহারকারী কোনো argument না দিলে, সামলানো যায়; আপাতত সেই পরিস্থিতি উপেক্ষা
করে বরং ফাইল পড়ার ক্ষমতা যোগ করার কাজ করব।

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths

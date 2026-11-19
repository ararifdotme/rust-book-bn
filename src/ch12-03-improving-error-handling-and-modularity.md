## modularity ও error handling উন্নত করতে refactoring {#refactoring-to-improve-modularity-and-error-handling}

আমাদের প্রোগ্রাম উন্নত করতে আমরা চারটি সমস্যা ঠিক করব, যেগুলো প্রোগ্রামের গঠন আর সম্ভাব্য error সামলানোর ধরনের সঙ্গে জড়িত।
প্রথমত, আমাদের `main` function এখন দুটি কাজ করে: argument parse করে (বিশ্লেষণ করে) এবং ফাইল পড়ে। প্রোগ্রাম বড় হতে থাকলে
`main` function-এর সামলানো আলাদা আলাদা কাজের সংখ্যা বাড়বে। একটি function-এর দায়িত্ব যত বাড়ে, তা নিয়ে যুক্তি দিয়ে ভাবা তত
কঠিন হয়, test করা কঠিন হয়, আর এর কোনো অংশ না ভেঙে বদলানোও কঠিন হয়। কার্যক্ষমতা এমনভাবে আলাদা করা সবচেয়ে ভালো, যাতে প্রতিটি
function একটি কাজের দায়িত্বে থাকে।

এই সমস্যাটি দ্বিতীয় সমস্যার সঙ্গেও জড়িত: `query` আর `file_path` আমাদের প্রোগ্রামের configuration variable হলেও `contents`-এর
মতো variable ব্যবহার হয় প্রোগ্রামের logic চালাতে। `main` যত লম্বা হবে, তত বেশি variable scope-এ আনতে হবে; scope-এ যত বেশি
variable থাকবে, প্রতিটির উদ্দেশ্য মনে রাখা তত কঠিন হবে। configuration variable-গুলোকে একটি কাঠামোয় জড়ো করে তাদের উদ্দেশ্য
স্পষ্ট করাই সবচেয়ে ভালো।

তৃতীয় সমস্যা হলো, ফাইল পড়া ব্যর্থ হলে error message print করতে আমরা `expect` ব্যবহার করেছি, কিন্তু error message শুধু print
করে `Should have been
able to read the file`। ফাইল পড়া নানা কারণে ব্যর্থ হতে পারে: যেমন, ফাইলটি না-ও থাকতে পারে, বা সেটি খোলার অনুমতি আমাদের না-ও
থাকতে পারে। এই মুহূর্তে পরিস্থিতি যা-ই হোক, আমরা সবকিছুর জন্য একই error message print করতাম, যা ব্যবহারকারীকে কোনো তথ্যই দিত
না!

চতুর্থত, আমরা error সামলাতে `expect` ব্যবহার করি, আর ব্যবহারকারী যথেষ্ট argument না দিয়ে আমাদের প্রোগ্রাম চালালে Rust থেকে একটি
`index out of bounds` error পাবে, যা সমস্যাটি পরিষ্কারভাবে ব্যাখ্যা করে না। সব error-handling কোড এক জায়গায় থাকলে সবচেয়ে ভালো
হতো, যাতে ভবিষ্যতে যারা কোড রক্ষণাবেক্ষণ করবে, error-handling logic বদলানোর দরকার হলে তাদের শুধু এক জায়গায় কোড দেখতে হয়। সব
error-handling কোড এক জায়গায় থাকলে এটাও নিশ্চিত হবে যে আমরা এমন message print করছি যা শেষ ব্যবহারকারীদের কাছে অর্থবহ।

চলুন আমাদের project refactor করে এই চারটি সমস্যার সমাধান করি।

<!-- Old headings. Do not remove or links may break. -->

<a id="separation-of-concerns-for-binary-projects"></a>

### binary project-এ দায়িত্ব আলাদা করা {#separating-concerns-in-binary-projects}

`main` function-কে একাধিক কাজের দায়িত্ব দেওয়ার এই সাংগঠনিক সমস্যা অনেক binary project-এই দেখা যায়। ফলে অনেক Rust প্রোগ্রামার
`main` function বড় হতে শুরু করলে একটি binary প্রোগ্রামের আলাদা আলাদা দায়িত্ব ভাগ করে ফেলা কাজের মনে করেন। এই প্রক্রিয়ার ধাপগুলো:

- আপনার প্রোগ্রামকে একটি _main.rs_ ফাইল আর একটি _lib.rs_ ফাইলে ভাগ করুন, এবং প্রোগ্রামের logic _lib.rs_-এ সরিয়ে নিন।
- যতক্ষণ command line parse করার logic ছোট, ততক্ষণ এটি `main` function-এই থাকতে পারে।
- command line parse করার logic জটিল হতে শুরু করলে একে `main` function থেকে বের করে অন্য function বা type-এ নিয়ে যান।

এই প্রক্রিয়ার পর `main` function-এ যে দায়িত্বগুলো থাকে, তা এগুলোর মধ্যে সীমিত হওয়া উচিত:

- argument-এর মান দিয়ে command line parse করার logic call করা
- অন্য যেকোনো configuration প্রস্তুত করা
- _lib.rs_-এর একটি `run` function call করা
- `run` error return করলে সেই error সামলানো

এই pattern-এর মূল কথা দায়িত্ব আলাদা করা: _main.rs_ প্রোগ্রাম চালানোর দায়িত্ব নেয়, আর _lib.rs_ হাতের কাজটির সব logic সামলায়।
`main` function সরাসরি test করা যায় না বলে এই গঠন আপনার প্রোগ্রামের সব logic `main` function-এর বাইরে সরিয়ে সেগুলো test করার
সুযোগ দেয়। `main` function-এ যে কোড থেকে যায় তা এত ছোট হবে যে পড়েই এর সঠিকতা যাচাই করা যাবে। চলুন এই প্রক্রিয়া মেনে আমাদের
প্রোগ্রাম নতুন করে সাজাই।

#### argument parser বের করে আনা {#extracting-the-argument-parser}

argument parse করার কার্যক্ষমতা আমরা একটি function-এ বের করে আনব, যা `main` call করবে। Listing 12-5-এ `main` function-এর নতুন
শুরুটা দেখানো হয়েছে, যা `parse_config` নামের একটি নতুন function call করে; এটি আমরা _src/main.rs_-এ সংজ্ঞায়িত করব।

<Listing number="12-5" file-name="src/main.rs" caption="`main` থেকে একটি `parse_config` function বের করে আনা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

</Listing>

আমরা এখনো command line argument-গুলো একটি vector-এ জড়ো করছি, কিন্তু `main` function-এর ভেতরে index 1-এর argument-এর মান `query`
variable-এ আর index 2-এর argument-এর মান `file_path` variable-এ বসানোর বদলে পুরো vector-টি `parse_config` function-এ পাঠাই। তখন
`parse_config` function-এ থাকে সেই logic, যা ঠিক করে কোন argument কোন variable-এ যাবে, এবং মানগুলো `main`-এ ফেরত পাঠায়। আমরা
এখনো `main`-এ `query` আর `file_path` variable তৈরি করি, কিন্তু command line argument আর variable-গুলো কীভাবে মিলবে তা ঠিক করার
দায়িত্ব আর `main`-এর নেই।

আমাদের ছোট প্রোগ্রামের জন্য এই পুনর্বিন্যাস বাড়াবাড়ি মনে হতে পারে, কিন্তু আমরা ছোট ছোট ধাপে ধীরে ধীরে refactor করছি। এই
পরিবর্তনের পর আবার প্রোগ্রাম চালিয়ে যাচাই করুন যে argument parsing এখনো কাজ করছে। ঘন ঘন অগ্রগতি যাচাই করা ভালো, এতে সমস্যা
দেখা দিলে তার কারণ চিহ্নিত করা সহজ হয়।

#### configuration-এর মানগুলো একসঙ্গে রাখা {#grouping-configuration-values}

`parse_config` function আরও উন্নত করতে আমরা আরেকটি ছোট পদক্ষেপ নিতে পারি। এই মুহূর্তে আমরা একটি tuple return করছি, কিন্তু
তারপরই সঙ্গে সঙ্গে সেই tuple আবার আলাদা অংশে ভেঙে ফেলছি। এটি একটি লক্ষণ যে হয়তো আমাদের কাছে এখনো সঠিক abstraction নেই।

উন্নতির সুযোগ আছে এমন আরেকটি ইঙ্গিত হলো `parse_config`-এর `config` অংশটি, যা বোঝায় যে আমাদের return করা দুটি মান পরস্পর
সম্পর্কিত এবং দুটিই একটি configuration মানের অংশ। দুটি মানকে একটি tuple-এ রাখা ছাড়া ডেটার গঠনে আমরা এখন এই অর্থ প্রকাশ করছি
না; এর বদলে আমরা দুটি মানকে একটি struct-এ রাখব এবং struct-এর প্রতিটি field-কে একটি অর্থবহ নাম দেব। এতে ভবিষ্যতে এই কোড যারা
রক্ষণাবেক্ষণ করবে তাদের পক্ষে বোঝা সহজ হবে যে ভিন্ন ভিন্ন মানগুলো একে অপরের সঙ্গে কীভাবে সম্পর্কিত এবং তাদের উদ্দেশ্য কী।

Listing 12-6-এ `parse_config` function-এর উন্নতিগুলো দেখানো হয়েছে।

<Listing number="12-6" file-name="src/main.rs" caption="`parse_config`-কে refactor করে একটি `Config` struct-এর instance return করানো">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

</Listing>

আমরা `Config` নামের একটি struct যোগ করেছি, যার `query` আর `file_path` নামের field আছে। `parse_config`-এর signature এখন বোঝায় যে এটি
একটি `Config` মান return করে। `parse_config`-এর body-তে আগে আমরা এমন string slice return করতাম যা `args`-এর `String` মানকে
নির্দেশ করত, এখন `Config`-কে এমনভাবে সংজ্ঞায়িত করেছি যাতে এতে owned `String` মান থাকে। `main`-এর `args` variable argument-এর
মানগুলোর owner, এবং এটি শুধু `parse_config` function-কে সেগুলো borrow করতে দিচ্ছে, অর্থাৎ `Config` যদি `args`-এর মানগুলোর ownership
নিতে চাইত, তাহলে আমরা Rust-এর borrowing-এর নিয়ম ভঙ্গ করতাম।

`String` ডেটা সামলানোর কয়েকটি উপায় আছে; সবচেয়ে সহজ, যদিও কিছুটা অদক্ষ, পথ হলো মানগুলোর ওপর `clone` method call করা। এতে
`Config` instance-এর মালিকানার জন্য ডেটার একটি পূর্ণ কপি তৈরি হবে, যাতে string ডেটার একটি reference রাখার চেয়ে বেশি সময় ও মেমোরি
লাগে। তবে ডেটা clone করলে আমাদের কোডও খুব সোজাসাপ্টা হয়, কারণ reference-গুলোর lifetime সামলাতে হয় না; এই পরিস্থিতিতে সরলতার
বিনিময়ে সামান্য performance ছেড়ে দেওয়া লাভজনক আপস।

> ### `clone` ব্যবহারের সুবিধা-অসুবিধা {#the-trade-offs-of-using-clone}
>
> runtime-এর খরচের কারণে অনেক Rustacean (Rust প্রোগ্রামার)-এর মধ্যে ownership-এর সমস্যা ঠিক করতে `clone` ব্যবহার এড়িয়ে চলার
> প্রবণতা আছে।
> [অধ্যায় 13][ch13]<!-- ignore -->-এ আপনি শিখবেন এই ধরনের পরিস্থিতিতে কীভাবে আরও দক্ষ method ব্যবহার করতে হয়। তবে আপাতত
> এগিয়ে যাওয়ার জন্য কয়েকটি string কপি করা ঠিক আছে, কারণ এই কপিগুলো আপনি মাত্র একবার করবেন এবং আপনার file path ও query string
> খুবই ছোট। প্রথম চেষ্টাতেই কোডকে অতিমাত্রায় optimize করার চেষ্টার চেয়ে একটু অদক্ষ কিন্তু কাজ করে এমন প্রোগ্রাম থাকা ভালো।
> Rust-এ অভিজ্ঞতা বাড়লে সবচেয়ে দক্ষ সমাধান দিয়ে শুরু করা সহজ হবে, কিন্তু আপাতত `clone` call করা পুরোপুরি গ্রহণযোগ্য।

আমরা `main` হালনাগাদ করেছি, যাতে এটি `parse_config`-এর return করা `Config` instance-কে `config` নামের একটি variable-এ রাখে, আর
আগে আলাদা `query` ও `file_path` variable ব্যবহার করা কোডটিও হালনাগাদ করেছি, যাতে এখন এর বদলে `Config` struct-এর field ব্যবহার
করে।

এখন আমাদের কোড আরও পরিষ্কারভাবে প্রকাশ করে যে `query` আর `file_path` সম্পর্কিত এবং তাদের উদ্দেশ্য প্রোগ্রাম কীভাবে কাজ করবে তা
নির্ধারণ করা। এই মানগুলো ব্যবহার করা যেকোনো কোড জানে যে এগুলো `config` instance-এ, তাদের উদ্দেশ্য অনুযায়ী নাম দেওয়া field-এ
খুঁজতে হবে।

#### `Config`-এর জন্য একটি constructor তৈরি করা {#creating-a-constructor-for-config}

এ পর্যন্ত আমরা command line argument parse করার দায়িত্বে থাকা logic `main` থেকে বের করে `parse_config` function-এ রেখেছি। এতে
আমরা দেখতে পেয়েছি যে `query` আর `file_path` মান দুটি সম্পর্কিত, এবং সেই সম্পর্ক আমাদের কোডে প্রকাশ পাওয়া উচিত। তারপর `query`
আর `file_path`-এর সম্পর্কিত উদ্দেশ্যের একটি নাম দিতে এবং `parse_config` function থেকে মানগুলোর নাম struct-এর field-এর নাম হিসেবে
return করতে আমরা একটি `Config` struct যোগ করেছি।

তাহলে, `parse_config` function-এর উদ্দেশ্য যেহেতু এখন একটি `Config` instance তৈরি করা, তাই আমরা `parse_config`-কে একটি সাধারণ
function থেকে বদলে `Config` struct-এর সঙ্গে যুক্ত `new` নামের একটি function বানাতে পারি। এই পরিবর্তনে কোড আরও idiomatic (Rust-এর
প্রচলিত লেখার ধরন অনুযায়ী) হবে। standard library-র type, যেমন `String`-এর instance আমরা `String::new` call করে তৈরি করতে পারি।
একইভাবে, `parse_config`-কে `Config`-এর সঙ্গে যুক্ত একটি `new` function-এ বদলে দিলে `Config::new` call করে `Config`-এর instance তৈরি
করতে পারব। Listing 12-7-এ প্রয়োজনীয় পরিবর্তনগুলো দেখানো হয়েছে।

<Listing number="12-7" file-name="src/main.rs" caption="`parse_config`-কে `Config::new`-এ বদলানো">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

</Listing>

`main`-এ যেখানে `parse_config` call করছিলাম, সেখানে হালনাগাদ করে এর বদলে `Config::new` call করেছি। `parse_config`-এর নাম বদলে
`new` করেছি এবং একে একটি `impl` block-এর ভেতরে সরিয়ে নিয়েছি, যা `new` function-কে `Config`-এর সঙ্গে যুক্ত করে। কোডটি কাজ করছে
কি না নিশ্চিত হতে আবার compile করে দেখুন।

### error handling ঠিক করা {#fixing-the-error-handling}

এবার আমরা error handling ঠিক করার কাজ করব। মনে করুন, `args` vector-এ তিনটির কম item থাকলে index 1 বা index 2-এর মান পাওয়ার চেষ্টা
করলে প্রোগ্রাম panic করবে। কোনো argument ছাড়া প্রোগ্রামটি চালিয়ে দেখুন; দেখতে এমন হবে:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

`index out of bounds: the len is 1 but the index is 1` লাইনটি প্রোগ্রামারদের জন্য একটি error message। এটি আমাদের শেষ ব্যবহারকারীদের
বুঝতে সাহায্য করবে না যে তাদের এর বদলে কী করা উচিত। চলুন এখনই এটি ঠিক করি।

#### error message উন্নত করা {#improving-the-error-message}

Listing 12-8-এ আমরা `new` function-এ একটি যাচাই যোগ করি, যা index 1 ও index 2-এর মান নেওয়ার আগে নিশ্চিত করবে যে slice-টি যথেষ্ট
লম্বা। slice যথেষ্ট লম্বা না হলে প্রোগ্রাম panic করে এবং আরও ভালো একটি error message দেখায়।

<Listing number="12-8" file-name="src/main.rs" caption="argument-এর সংখ্যা যাচাই যোগ করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

</Listing>

এই কোড [Listing
9-13-এ লেখা `Guess::new` function-এর][ch9-custom-types]<!-- ignore --> মতো, যেখানে `value` argument বৈধ মানের সীমার বাইরে থাকলে আমরা
`panic!` call করেছিলাম। এখানে মানের সীমা যাচাই না করে আমরা যাচাই করছি যে `args`-এর দৈর্ঘ্য অন্তত `3`, আর function-এর বাকি অংশ এই
ধারণার ওপর কাজ করতে পারে যে শর্তটি পূরণ হয়েছে। `args`-এ তিনটির কম item থাকলে এই শর্ত `true` হবে, আর আমরা প্রোগ্রাম সঙ্গে সঙ্গে
শেষ করতে `panic!` macro call করি।

`new`-এ এই কয়েক লাইন বাড়তি কোড যোগ করে চলুন আবার কোনো argument ছাড়া প্রোগ্রামটি চালিয়ে দেখি error এখন দেখতে কেমন:

```console
{{#include ../listings/ch12-an-io-project/listing-12-08/output.txt}}
```

এই output আগের চেয়ে ভালো: এখন আমাদের একটি যুক্তিসঙ্গত error message আছে। তবে এতে এমন বাড়তি তথ্যও আছে যা আমরা ব্যবহারকারীদের
দিতে চাই না। হয়তো Listing 9-13-এ ব্যবহার করা কৌশলটি এখানে ব্যবহারের জন্য সেরা নয়: [অধ্যায় 9-এ যেমন আলোচনা
করা হয়েছে][ch9-error-guidelines]<!-- ignore -->, `panic!` call করা ব্যবহারজনিত সমস্যার চেয়ে প্রোগ্রামিংয়ের সমস্যার জন্য বেশি উপযুক্ত।
এর বদলে আমরা অধ্যায় 9-এ শেখা অন্য কৌশলটি ব্যবহার করব—[একটি
`Result` return করা][ch9-result]<!-- ignore -->, যা সাফল্য অথবা error বোঝায়।

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-a-result-from-new-instead-of-calling-panic"></a>

#### `panic!` call না করে একটি `Result` return করা {#returning-a-result-instead-of-calling-panic}

এর বদলে আমরা একটি `Result` মান return করতে পারি, যাতে সফল হলে একটি `Config` instance থাকবে আর error-এর ক্ষেত্রে সমস্যার বর্ণনা
থাকবে। function-এর নামও `new` থেকে বদলে `build` করব, কারণ অনেক প্রোগ্রামার আশা করেন যে `new` function কখনো ব্যর্থ হবে না।
`Config::build` যখন `main`-এর সঙ্গে যোগাযোগ করছে, তখন কোনো সমস্যা হয়েছে তা জানাতে আমরা `Result` type ব্যবহার করতে পারি। তারপর
`main` বদলে একটি `Err` variant-কে ব্যবহারকারীদের জন্য আরও কাজের একটি error-এ রূপান্তর করতে পারি, `panic!` call করলে যে `thread
'main'` আর `RUST_BACKTRACE` সংক্রান্ত আশপাশের লেখা আসে তা ছাড়াই।

Listing 12-9-এ দেখানো হয়েছে, যে function-কে আমরা এখন `Config::build` বলছি তার return মানে, এবং `Result` return করতে function-এর
body-তে কী কী পরিবর্তন দরকার। খেয়াল করুন, `main`-ও হালনাগাদ না করা পর্যন্ত এটি compile হবে না; সেটি আমরা পরের listing-এ করব।

<Listing number="12-9" file-name="src/main.rs" caption="`Config::build` থেকে একটি `Result` return করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

</Listing>

আমাদের `build` function একটি `Result` return করে, যাতে সফল হলে একটি `Config` instance আর error হলে একটি string literal থাকে। আমাদের
error-এর মানগুলো সবসময় `'static` lifetime-এর string literal হবে।

function-এর body-তে আমরা দুটি পরিবর্তন করেছি: ব্যবহারকারী যথেষ্ট argument না দিলে `panic!` call না করে এখন একটি `Err` মান return
করি, আর `Config` return মানটিকে একটি `Ok`-এর ভেতরে মুড়ে দিয়েছি। এই পরিবর্তনগুলোর ফলে function-টি তার নতুন type signature-এর
সঙ্গে মিলে যায়।

`Config::build` থেকে একটি `Err` মান return করলে `main` function `build` function-এর return করা `Result` মান সামলাতে পারে এবং error-এর
ক্ষেত্রে process আরও পরিচ্ছন্নভাবে বন্ধ করতে পারে।

<!-- Old headings. Do not remove or links may break. -->

<a id="calling-confignew-and-handling-errors"></a>

#### `Config::build` call করা এবং error সামলানো {#calling-configbuild-and-handling-errors}

error-এর ক্ষেত্রটি সামলাতে এবং ব্যবহারকারী-বান্ধব একটি message print করতে আমাদের `main` হালনাগাদ করতে হবে, যাতে এটি
`Config::build`-এর return করা `Result` সামলায়, যেমনটি Listing 12-10-এ দেখানো হয়েছে। শূন্য নয় এমন error code দিয়ে command line
tool থেকে বেরিয়ে যাওয়ার দায়িত্বও আমরা `panic!`-এর কাছ থেকে সরিয়ে নিজেরাই implement করব। শূন্য নয় এমন exit status হলো একটি
প্রচলিত রীতি, যা আমাদের প্রোগ্রাম call করা process-কে জানায় যে প্রোগ্রামটি error অবস্থায় বন্ধ হয়েছে।

<Listing number="12-10" file-name="src/main.rs" caption="`Config` তৈরি ব্যর্থ হলে একটি error code দিয়ে বেরিয়ে যাওয়া">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

</Listing>

এই listing-এ আমরা এমন একটি method ব্যবহার করেছি যা এখনো বিস্তারিত আলোচনা করিনি: `unwrap_or_else`, যা standard library
`Result<T, E>`-এর ওপর সংজ্ঞায়িত করেছে। `unwrap_or_else` ব্যবহার করে আমরা `panic!` ছাড়া নিজস্ব কিছু error handling সংজ্ঞায়িত করতে
পারি। `Result` যদি একটি `Ok` মান হয়, তাহলে এই method-এর আচরণ `unwrap`-এর মতো: `Ok` যে মানটিকে মুড়ে রেখেছে, তা return করে। কিন্তু
মানটি যদি একটি `Err` মান হয়, তাহলে এই method closure (পারিপার্শ্বিক মান ধরে রাখতে পারে এমন নামহীন function)-এর কোড call করে,
যা আমাদের সংজ্ঞায়িত একটি নামহীন function এবং যা আমরা `unwrap_or_else`-এ argument হিসেবে পাঠাই। closure নিয়ে আরও বিস্তারিত
আলোচনা করব [অধ্যায় 13][ch13]<!-- ignore -->-এ। আপাতত শুধু জানলেই চলবে যে `unwrap_or_else` `Err`-এর ভেতরের মানটি, যা এই ক্ষেত্রে
Listing 12-9-এ যোগ করা static string `"not enough arguments"`, আমাদের closure-এ খাড়া দাগ দুটির মাঝে থাকা `err` argument-এ পাঠাবে।
closure-এর কোড চলার সময় তখন `err` মানটি ব্যবহার করতে পারে।

standard library থেকে `process`-কে scope-এ আনতে আমরা একটি নতুন `use` লাইন যোগ করেছি। error-এর ক্ষেত্রে যে closure-এর কোড চলবে তা
মাত্র দুই লাইনের: আমরা `err` মানটি print করি, তারপর `process::exit` call করি। `process::exit` function সঙ্গে সঙ্গে প্রোগ্রাম থামিয়ে
দেবে এবং যে সংখ্যা পাঠানো হয়েছে তা exit status code হিসেবে return করবে। এটি Listing 12-8-এ ব্যবহার করা `panic!`-ভিত্তিক handling-এর
মতো, তবে এখন আর সব বাড়তি output পাই না। চলুন চালিয়ে দেখি:

```console
{{#include ../listings/ch12-an-io-project/listing-12-10/output.txt}}
```

চমৎকার! এই output আমাদের ব্যবহারকারীদের জন্য অনেক বেশি বন্ধুত্বপূর্ণ।

<!-- Old headings. Do not remove or links may break. -->

<a id="extracting-logic-from-the-main-function"></a>

### `main` থেকে logic বের করে আনা {#extracting-logic-from-main}

configuration parse করার refactoring শেষ, এবার প্রোগ্রামের logic-এর দিকে নজর দিই। [“binary project-এ দায়িত্ব
আলাদা করা”](#separation-of-concerns-for-binary-projects)<!-- ignore --> অংশে যেমন বলেছি, আমরা `run` নামের একটি function বের করে আনব,
যাতে `main` function-এর সেই সব logic থাকবে যা configuration প্রস্তুত করা বা error সামলানোর সঙ্গে জড়িত নয়। কাজ শেষ হলে `main`
function সংক্ষিপ্ত হবে এবং দেখেই যাচাই করা সহজ হবে, আর বাকি সব logic-এর জন্য আমরা test লিখতে পারব।

Listing 12-11-এ একটি `run` function বের করে আনার ছোট, ধাপে ধাপে উন্নতিটি দেখানো হয়েছে।

<Listing number="12-11" file-name="src/main.rs" caption="প্রোগ্রামের বাকি logic-যুক্ত একটি `run` function বের করে আনা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

</Listing>

`run` function-এ এখন ফাইল পড়া থেকে শুরু করে `main`-এর বাকি সব logic আছে। `run` function argument হিসেবে `Config` instance নেয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="returning-errors-from-the-run-function"></a>

#### `run` থেকে error return করা {#returning-errors-from-run}

প্রোগ্রামের বাকি logic `run` function-এ আলাদা হয়ে যাওয়ায় আমরা error handling উন্নত করতে পারি, যেমনটি Listing 12-9-এ
`Config::build`-এর ক্ষেত্রে করেছিলাম। `expect` call করে প্রোগ্রামকে panic করতে না দিয়ে কোনো সমস্যা হলে `run` function একটি
`Result<T, E>` return করবে। এতে error সামলানোর logic আরও একত্র করে ব্যবহারকারী-বান্ধব উপায়ে `main`-এ রাখা যাবে। Listing 12-12-এ
`run`-এর signature আর body-তে প্রয়োজনীয় পরিবর্তনগুলো দেখানো হয়েছে।

<Listing number="12-12" file-name="src/main.rs" caption="`run` function বদলে `Result` return করানো">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

</Listing>

এখানে আমরা তিনটি গুরুত্বপূর্ণ পরিবর্তন করেছি। প্রথমত, `run` function-এর return type বদলে `Result<(), Box<dyn Error>>` করেছি। এই
function আগে unit type `()` return করত, আর `Ok`-এর ক্ষেত্রে return মান হিসেবে সেটিই রেখেছি।

error type-এর জন্য আমরা trait object `Box<dyn Error>` ব্যবহার করেছি (এবং শুরুতে একটি `use` statement দিয়ে `std::error::Error`-কে
scope-এ এনেছি)। trait object নিয়ে আলোচনা করব [অধ্যায় 18][ch18]<!-- ignore -->-এ। আপাতত শুধু জেনে রাখুন, `Box<dyn Error>` মানে
function-টি এমন একটি type return করবে যা `Error` trait implement করে, কিন্তু return মানটি ঠিক কোন type-এর হবে তা আমাদের উল্লেখ
করতে হবে না। এতে বিভিন্ন error-এর ক্ষেত্রে বিভিন্ন type-এর error মান return করার নমনীয়তা পাই। `dyn` keyword হলো _dynamic_
(চলার সময় নির্ধারিত)-এর সংক্ষেপ।

দ্বিতীয়ত, আমরা `expect` call সরিয়ে `?` operator ব্যবহার করেছি, যেমনটি [অধ্যায় 9][ch9-question-mark]<!-- ignore -->-এ আলোচনা
করেছিলাম। error হলে `panic!` না করে `?` বর্তমান function থেকে error মানটি return করবে, যাতে caller সেটি সামলাতে পারে।

তৃতীয়ত, `run` function এখন সফল হলে একটি `Ok` মান return করে। signature-এ আমরা `run` function-এর সাফল্যের type `()` হিসেবে ঘোষণা
করেছি, অর্থাৎ unit type-এর মানটিকে `Ok` মানের ভেতরে মুড়ে দিতে হবে। এই `Ok(())` syntax প্রথমে একটু অদ্ভুত লাগতে পারে। কিন্তু
এভাবে `()` ব্যবহার করাই প্রচলিত উপায় এটা বোঝাতে যে আমরা `run` call করছি শুধু এর side effect (পার্শ্বপ্রতিক্রিয়া)-এর জন্য; এটি
আমাদের দরকারি কোনো মান return করে না।

এই কোড চালালে এটি compile হবে, তবে একটি warning দেখাবে:

```console
{{#include ../listings/ch12-an-io-project/listing-12-12/output.txt}}
```

Rust আমাদের বলছে যে আমাদের কোড `Result` মানটি উপেক্ষা করেছে, আর `Result` মান হয়তো বোঝাতে পারে যে একটি error ঘটেছে। কিন্তু error
হয়েছে কি না তা আমরা যাচাই করছি না, আর compiler মনে করিয়ে দিচ্ছে যে আমরা সম্ভবত এখানে কিছু error-handling কোড রাখতে চেয়েছিলাম!
চলুন এখনই সমস্যাটি ঠিক করি।

#### `main`-এ `run`-এর return করা error সামলানো {#handling-errors-returned-from-run-in-main}

Listing 12-10-এ `Config::build`-এর সঙ্গে যে কৌশল ব্যবহার করেছিলাম তার মতোই একটি কৌশলে আমরা error যাচাই করে সামলাব, তবে সামান্য
পার্থক্যসহ:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

`run` একটি `Err` মান return করে কি না যাচাই করতে এবং করলে `process::exit(1)` call করতে আমরা `unwrap_or_else`-এর বদলে `if let`
ব্যবহার করি। `Config::build` যেভাবে `Config` instance return করে, `run` function সেভাবে এমন কোনো মান return করে না যা আমরা `unwrap`
করতে চাই। `run` যেহেতু সফল হলে `()` return করে, তাই আমাদের শুধু error শনাক্ত করা নিয়ে মাথাব্যথা, ফলে unwrap করা মান return করতে
`unwrap_or_else` দরকার নেই, যা শুধু `()`-ই হতো।

দুই ক্ষেত্রেই `if let` আর `unwrap_or_else` function-এর body একই: আমরা error print করি এবং বেরিয়ে যাই।

### কোডকে একটি library crate-এ ভাগ করা {#splitting-code-into-a-library-crate}

আমাদের `minigrep` project এ পর্যন্ত বেশ ভালো দেখাচ্ছে! এবার আমরা _src/main.rs_ ফাইল ভাগ করে কিছু কোড _src/lib.rs_ ফাইলে রাখব।
এতে কোড test করা যাবে, আর _src/main.rs_ ফাইলের দায়িত্বও কমবে।

লেখা খোঁজার দায়িত্বে থাকা কোড _src/main.rs_-এর বদলে _src/lib.rs_-এ সংজ্ঞায়িত করি, যাতে আমরা (বা আমাদের `minigrep` library ব্যবহার
করা অন্য যে কেউ) শুধু আমাদের `minigrep` binary নয়, আরও বিভিন্ন প্রেক্ষাপট থেকে খোঁজার function call করতে পারি।

প্রথমে _src/lib.rs_-এ `search` function-এর signature সংজ্ঞায়িত করি, যেমনটি Listing 12-13-এ দেখানো হয়েছে, যার body `unimplemented!`
macro call করে। implementation লেখার সময় signature-টি আরও বিস্তারিত ব্যাখ্যা করব।

<Listing number="12-13" file-name="src/lib.rs" caption="*src/lib.rs*-এ `search` function সংজ্ঞায়িত করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs}}
```

</Listing>

`search`-কে আমাদের library crate-এর public API-র অংশ হিসেবে চিহ্নিত করতে function-এর সংজ্ঞায় `pub` keyword ব্যবহার করেছি। এখন
আমাদের একটি library crate আছে, যা binary crate থেকে ব্যবহার করা যায় এবং test করা যায়!

এবার _src/lib.rs_-এ সংজ্ঞায়িত কোডকে _src/main.rs_-এর binary crate-এর scope-এ এনে call করতে হবে, যেমনটি Listing 12-14-এ দেখানো
হয়েছে।

<Listing number="12-14" file-name="src/main.rs" caption="*src/main.rs*-এ `minigrep` library crate-এর `search` function ব্যবহার করা">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

</Listing>

library crate থেকে `search` function-কে binary crate-এর scope-এ আনতে আমরা একটি `use minigrep::search` লাইন যোগ করি। তারপর `run`
function-এ ফাইলের বিষয়বস্তু print না করে `search` function call করি এবং argument হিসেবে `config.query` মান আর `contents` পাঠাই।
তারপর `run` একটি `for` loop দিয়ে `search` থেকে return হওয়া প্রতিটি লাইন print করবে, যা query-র সঙ্গে মিলেছে। `main` function-এ
query আর file path দেখানো `println!` call-গুলো সরিয়ে দেওয়ারও এটি ভালো সময়, যাতে আমাদের প্রোগ্রাম শুধু খোঁজার ফল print করে (যদি
কোনো error না ঘটে)।

খেয়াল করুন, কিছু print হওয়ার আগেই search function সব ফল একটি vector-এ জড়ো করবে এবং সেটি return করবে। বড় ফাইলে খোঁজার সময় এই
implementation ফল দেখাতে ধীর হতে পারে, কারণ ফল পাওয়ার সঙ্গে সঙ্গে print হয় না; iterator ব্যবহার করে এটি ঠিক করার একটি সম্ভাব্য
উপায় নিয়ে অধ্যায় 13-এ আলোচনা করব।

উফ! অনেক কাজ হলো, তবে আমরা ভবিষ্যতের সাফল্যের জন্য ভিত তৈরি করে ফেলেছি। এখন error সামলানো অনেক সহজ, আর কোড আরও modular
(আলাদা আলাদা অংশে ভাগ করা) হয়েছে। এখন থেকে আমাদের প্রায় সব কাজ হবে _src/lib.rs_-এ।

চলুন এই নতুন পাওয়া modularity-র সুবিধা নিয়ে এমন কিছু করি যা পুরোনো কোডে কঠিন হতো কিন্তু নতুন কোডে সহজ: আমরা কিছু test লিখব!

[ch13]: ch13-00-functional-features.html
[ch9-custom-types]: ch09-03-to-panic-or-not-to-panic.html#creating-custom-types-for-validation
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
[ch18]: ch18-00-oop.html
[ch9-question-mark]: ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

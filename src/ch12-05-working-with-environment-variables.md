## environment variable নিয়ে কাজ করা {#working-with-environment-variables}

আমরা `minigrep` binary-তে একটি বাড়তি বৈশিষ্ট্য যোগ করে একে উন্নত করব: case-insensitive (বড়-ছোট হাতের অক্ষরের পার্থক্য উপেক্ষা
করে) খোঁজার একটি বিকল্প, যা ব্যবহারকারী একটি environment variable (প্রোগ্রামের বাইরে থেকে সেট করা পরিবেশের মান) দিয়ে চালু
করতে পারবে। এই বৈশিষ্ট্যটিকে আমরা একটি command line বিকল্প বানিয়ে ব্যবহারকারীদের প্রতিবার এটি লিখতে বাধ্য করতে পারতাম, কিন্তু
এর বদলে environment variable বানালে ব্যবহারকারীরা একবার environment variable সেট করে সেই terminal session-এর সব খোঁজাই
case-insensitive করতে পারবে।

<!-- Old headings. Do not remove or links may break. -->
<a id="writing-a-failing-test-for-the-case-insensitive-search-function"></a>

### case-insensitive খোঁজার জন্য fail করে এমন একটি test লেখা {#writing-a-failing-test-for-case-insensitive-search}

প্রথমে আমরা `minigrep` library-তে একটি নতুন `search_case_insensitive` function যোগ করব, যা environment variable-এ কোনো মান থাকলে call
হবে। আমরা TDD প্রক্রিয়া মেনে চলতে থাকব, তাই প্রথম ধাপ আবারও fail করে এমন একটি test লেখা। নতুন `search_case_insensitive`
function-এর জন্য একটি নতুন test যোগ করব, আর দুটি test-এর পার্থক্য স্পষ্ট করতে পুরোনো test-এর নাম `one_result` থেকে বদলে
`case_sensitive` করব, যেমনটি Listing 12-20-এ দেখানো হয়েছে।

<Listing number="12-20" file-name="src/lib.rs" caption="যে case-insensitive function যোগ করতে যাচ্ছি, তার জন্য fail করে এমন একটি নতুন test যোগ করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

</Listing>

খেয়াল করুন, আমরা পুরোনো test-এর `contents`-ও বদলেছি। বড় হাতের _D_ দিয়ে `"Duct tape."` লেখাসহ একটি নতুন লাইন যোগ করেছি, যা
case-sensitive (বড়-ছোট হাতের অক্ষরের পার্থক্য মেনে) খোঁজার সময় `"duct"` query-র সঙ্গে মেলা উচিত নয়। পুরোনো test এভাবে বদলালে
নিশ্চিত হয় যে আমরা ইতিমধ্যে implement করা case-sensitive খোঁজার কার্যক্ষমতা ভুল করে ভেঙে ফেলছি না। এই test এখন pass করার কথা, এবং
case-insensitive খোঁজা নিয়ে কাজ করার সময়ও pass করতে থাকার কথা।

case-_insensitive_ খোঁজার নতুন test query হিসেবে `"rUsT"` ব্যবহার করে। যে `search_case_insensitive` function যোগ করতে যাচ্ছি, তাতে
`"rUsT"` query-র মেলা উচিত বড় হাতের _R_-যুক্ত `"Rust:"` লাইনের সঙ্গে, আর `"Trust me."` লাইনের সঙ্গেও, যদিও দুটিরই অক্ষরের হাত
query থেকে আলাদা। এটি আমাদের fail করা test, আর এটি compile-ই হবে না, কারণ আমরা এখনো `search_case_insensitive` function সংজ্ঞায়িত
করিনি। চাইলে Listing 12-16-এ `search` function-এর জন্য যেমন করেছিলাম তেমনভাবে সবসময় একটি খালি vector return করে এমন একটি কাঠামোগত
implementation যোগ করে দেখতে পারেন, test compile হয়ে fail করছে।

### `search_case_insensitive` function implement করা {#implementing-the-search_case_insensitive-function}

Listing 12-21-এ দেখানো `search_case_insensitive` function প্রায় `search` function-এর মতোই হবে। একমাত্র পার্থক্য হলো আমরা `query`
আর প্রতিটি `line`-কে ছোট হাতের অক্ষরে রূপান্তর করব, যাতে input argument-গুলোর অক্ষর যে হাতেরই হোক, লাইনে query আছে কি না যাচাই
করার সময় দুটির অক্ষর একই হাতের থাকে।

<Listing number="12-21" file-name="src/lib.rs" caption="`search_case_insensitive` function সংজ্ঞায়িত করা, যা তুলনার আগে query আর লাইনকে ছোট হাতের অক্ষরে রূপান্তর করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

</Listing>

প্রথমে আমরা `query` string-কে ছোট হাতের অক্ষরে রূপান্তর করে একই নামের একটি নতুন variable-এ রাখি, ফলে মূল `query` shadow হয়ে
যায়। query-র ওপর `to_lowercase` call করা দরকার, যাতে ব্যবহারকারীর query `"rust"`, `"RUST"`, `"Rust"` বা `"rUsT"`, যা-ই হোক, আমরা
query-কে `"rust"` হিসেবে গণ্য করি এবং অক্ষরের হাত উপেক্ষা করি। `to_lowercase` মৌলিক Unicode সামলাবে, তবে এটি 100 শতাংশ নির্ভুল হবে
না। বাস্তব কোনো application লিখলে আমরা এখানে আরও একটু কাজ করতাম, কিন্তু এই অংশটি environment variable নিয়ে, Unicode নিয়ে নয়, তাই
এখানে এটুকুতেই থামব।

খেয়াল করুন, `query` এখন string slice-এর বদলে একটি `String`, কারণ `to_lowercase` call করলে বিদ্যমান ডেটাকে নির্দেশ না করে নতুন ডেটা
তৈরি হয়। উদাহরণ হিসেবে ধরুন query হলো `"rUsT"`: ওই string slice-এ আমাদের ব্যবহারের জন্য ছোট হাতের `u` বা `t` নেই, তাই `"rust"`-যুক্ত
একটি নতুন `String` allocate করতে হবে। এখন `contains` method-এ argument হিসেবে `query` পাঠানোর সময় একটি ampersand যোগ করতে হবে,
কারণ `contains`-এর signature একটি string slice নেওয়ার মতো করে সংজ্ঞায়িত।

এরপর প্রতিটি `line`-এর ওপর `to_lowercase` call যোগ করে সব অক্ষর ছোট হাতের করি। `line` আর `query` দুটিকেই ছোট হাতের অক্ষরে রূপান্তর
করায় query-র অক্ষর যে হাতেরই হোক, আমরা মিল খুঁজে পাব।

চলুন দেখি এই implementation test-গুলো pass করে কি না:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

চমৎকার! সেগুলো pass করেছে। এবার `run` function থেকে নতুন `search_case_insensitive` function call করি। প্রথমে case-sensitive আর
case-insensitive খোঁজার মধ্যে বদল করার জন্য `Config` struct-এ একটি configuration বিকল্প যোগ করব। এই field যোগ করলে compiler error
আসবে, কারণ আমরা এখনো কোথাও এই field-এর প্রাথমিক মান দিইনি:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:here}}
```

আমরা `ignore_case` field যোগ করেছি, যা একটি Boolean রাখে। এরপর `run` function-কে `ignore_case` field-এর মান যাচাই করে সেটি দিয়ে ঠিক
করতে হবে `search` function call করবে নাকি `search_case_insensitive` function, যেমনটি Listing 12-22-এ দেখানো হয়েছে। এটি এখনো compile
হবে না।

<Listing number="12-22" file-name="src/main.rs" caption="`config.ignore_case`-এর মানের ওপর ভিত্তি করে `search` অথবা `search_case_insensitive` call করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/main.rs:there}}
```

</Listing>

শেষে আমাদের environment variable যাচাই করতে হবে। environment variable নিয়ে কাজ করার function-গুলো standard library-র `env` module-এ
আছে, যা ইতিমধ্যে _src/main.rs_-এর শুরুতে scope-এ আনা আছে। `IGNORE_CASE` নামের একটি environment variable-এর জন্য কোনো মান সেট করা হয়েছে
কি না যাচাই করতে আমরা `env` module-এর `var` function ব্যবহার করব, যেমনটি Listing 12-23-এ দেখানো হয়েছে।

<Listing number="12-23" file-name="src/main.rs" caption="`IGNORE_CASE` নামের environment variable-এ কোনো মান আছে কি না যাচাই করা">

```rust,ignore,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/main.rs:here}}
```

</Listing>

এখানে আমরা `ignore_case` নামে একটি নতুন variable তৈরি করি। এর মান বসাতে `env::var` function call করে তাকে `IGNORE_CASE` environment
variable-এর নাম পাঠাই। `env::var` function একটি `Result` return করে, যা environment variable-এ কোনো মান সেট করা থাকলে সেই মানসহ
সফল `Ok` variant হবে। environment variable সেট করা না থাকলে এটি `Err` variant return করবে।

environment variable সেট করা আছে কি না, অর্থাৎ প্রোগ্রামের case-insensitive খোঁজা করা উচিত কি না, তা যাচাই করতে আমরা `Result`-এর
ওপর `is_ok` method ব্যবহার করছি। `IGNORE_CASE` environment variable-এ কিছুই সেট করা না থাকলে `is_ok` `false` return করবে এবং প্রোগ্রাম
case-sensitive খোঁজা করবে। environment variable-এর _মান_ নিয়ে আমাদের মাথাব্যথা নেই, শুধু এটি সেট করা আছে নাকি নেই তা নিয়ে, তাই
`unwrap`, `expect` বা `Result`-এর অন্য যেসব method দেখেছি সেগুলো ব্যবহার না করে `is_ok` যাচাই করছি।

`ignore_case` variable-এর মানটি আমরা `Config` instance-এ পাঠাই, যাতে `run` function সেই মান পড়ে ঠিক করতে পারে
`search_case_insensitive` call করবে নাকি `search`, যেমনটি Listing 12-22-এ implement করেছি।

চলুন চালিয়ে দেখি! প্রথমে environment variable সেট না করে `to` query দিয়ে প্রোগ্রাম চালাব, যা পুরো ছোট হাতের _to_ শব্দযুক্ত যেকোনো
লাইনের সঙ্গে মেলা উচিত:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

মনে হচ্ছে এটি এখনো কাজ করছে! এবার `IGNORE_CASE`-কে `1` সেট করে একই `to` query দিয়ে প্রোগ্রাম চালাই:

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

PowerShell ব্যবহার করলে environment variable সেট করা আর প্রোগ্রাম চালানো আলাদা command হিসেবে করতে হবে:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

এতে আপনার shell session-এর বাকি সময় জুড়ে `IGNORE_CASE` থেকে যাবে। `Remove-Item` cmdlet দিয়ে এটি সরানো যায়:

```console
PS> Remove-Item Env:IGNORE_CASE
```

আমাদের এমন লাইন পাওয়ার কথা যাতে _to_ আছে, এবং যাতে বড় হাতের অক্ষরও থাকতে পারে:

<!-- manual-regeneration
cd listings/ch12-an-io-project/listing-12-23
IGNORE_CASE=1 cargo run -- to poem.txt
can't extract because of the environment variable
-->

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

চমৎকার, আমরা _To_-যুক্ত লাইনগুলোও পেয়েছি! আমাদের `minigrep` প্রোগ্রাম এখন একটি environment variable-এর নিয়ন্ত্রণে case-insensitive
খোঁজা করতে পারে। এখন আপনি জানেন command line argument বা environment variable, যেকোনোটি দিয়ে সেট করা বিকল্প কীভাবে সামলাতে হয়।

কিছু প্রোগ্রাম একই configuration-এর জন্য argument _এবং_ environment variable দুটিই অনুমোদন করে। সেসব ক্ষেত্রে প্রোগ্রাম ঠিক করে
কোনটি অগ্রাধিকার পাবে। নিজে নিজে আরেকটি অনুশীলন হিসেবে command line argument বা environment variable, যেকোনোটি দিয়ে case sensitivity
নিয়ন্ত্রণ করার চেষ্টা করুন। যদি প্রোগ্রাম এমনভাবে চালানো হয় যে একটি case sensitive আর অন্যটি ignore case সেট করা, তাহলে command line
argument অগ্রাধিকার পাবে নাকি environment variable, তা ঠিক করুন।

`std::env` module-এ environment variable সামলানোর জন্য আরও অনেক কাজের বৈশিষ্ট্য আছে: কী কী পাওয়া যায় তা দেখতে এর documentation দেখুন।

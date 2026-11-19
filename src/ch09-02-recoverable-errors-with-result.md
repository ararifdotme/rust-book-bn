## `Result` দিয়ে recoverable error {#recoverable-errors-with-result}

বেশিরভাগ error এতটা গুরুতর নয় যে প্রোগ্রাম পুরোপুরি থামিয়ে দিতে হবে। কখনো কখনো কোনো function ব্যর্থ হয় এমন কারণে,
যা আপনি সহজেই বুঝতে পারেন এবং যার জবাব দিতে পারেন। যেমন, আপনি একটি ফাইল খোলার চেষ্টা করলে আর ফাইলটির অস্তিত্ব না
থাকায় সেই operation ব্যর্থ হলে, process বন্ধ করে দেওয়ার বদলে হয়তো ফাইলটি তৈরি করতে চাইবেন।

অধ্যায় 2-এর [“`Result` দিয়ে সম্ভাব্য ব্যর্থতা সামলানো”][handle_failure]<!--
ignore --> থেকে মনে করুন, `Result` enum সংজ্ঞায়িত হয়েছে দুটি variant নিয়ে, `Ok` আর `Err`, এভাবে:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`T` আর `E` হলো generic type parameter (যেকোনো type বসানো যায় এমন জায়গা-রাখা নাম): generics নিয়ে আমরা অধ্যায় 10-এ আরও
বিস্তারিত আলোচনা করব। এখন আপনার যা জানা দরকার তা হলো, `T` প্রকাশ করে সফলতার ক্ষেত্রে `Ok` variant-এর ভেতরে return হওয়া
মানের type, আর `E` প্রকাশ করে ব্যর্থতার ক্ষেত্রে `Err` variant-এর ভেতরে return হওয়া error-এর type। `Result`-এর এই generic
type parameter-গুলো থাকায় আমরা `Result` type আর এর ওপর সংজ্ঞায়িত function-গুলো এমন অনেক আলাদা পরিস্থিতিতে ব্যবহার করতে
পারি, যেখানে আমাদের return করতে চাওয়া সফলতার মান আর error-এর মান আলাদা হতে পারে।

চলুন এমন একটি function call করি, যা ব্যর্থ হতে পারে বলে একটি `Result` মান return করে। Listing 9-3-এ আমরা একটি ফাইল খোলার
চেষ্টা করি।

<Listing number="9-3" file-name="src/main.rs" caption="একটি ফাইল খোলা">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

</Listing>

`File::open`-এর return type হলো একটি `Result<T, E>`। `File::open`-এর implementation generic parameter `T`-এর জায়গায় বসিয়েছে
সফলতার মানের type, `std::fs::File`, যা একটি file handle (খোলা ফাইল ব্যবহারের মাধ্যম)। error-এর মানে ব্যবহৃত `E`-এর type হলো
`std::io::Error`। এই return type-এর মানে, `File::open` call সফল হয়ে এমন একটি file handle return করতে পারে, যা থেকে আমরা
পড়তে বা যাতে লিখতে পারি। function call ব্যর্থও হতে পারে: যেমন, ফাইলটির অস্তিত্ব না-ও থাকতে পারে, বা ফাইলটিতে প্রবেশের অনুমতি
আমাদের না-ও থাকতে পারে। `File::open` function-এর এমন একটি উপায় দরকার, যা দিয়ে এটি জানাতে পারে সফল হয়েছে নাকি ব্যর্থ, আর
একই সঙ্গে হয় file handle নয়তো error-এর তথ্য দিতে পারে। `Result` enum ঠিক এই তথ্যটিই বহন করে।

`File::open` সফল হলে `greeting_file_result` variable-এর মান হবে `Ok`-এর একটি instance, যাতে একটি file handle থাকে। ব্যর্থ হলে
`greeting_file_result`-এর মান হবে `Err`-এর একটি instance, যাতে কী ধরনের error ঘটেছে তার আরও তথ্য থাকে।

`File::open` কী মান return করে, তার ওপর নির্ভর করে আলাদা কাজ করতে Listing 9-3-এর কোডে আমাদের কিছু যোগ করতে হবে। Listing
9-4-এ একটি মৌলিক টুল ব্যবহার করে `Result` সামলানোর একটি উপায় দেখানো হয়েছে, আর সেটি হলো অধ্যায় 6-এ আলোচিত `match`
expression।

<Listing number="9-4" file-name="src/main.rs" caption="return হতে পারে এমন `Result` variant-গুলো সামলাতে একটি `match` expression ব্যবহার করা">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

</Listing>

লক্ষ করুন, `Option` enum-এর মতোই `Result` enum আর এর variant-গুলো prelude দিয়ে scope-এ আনা হয়েছে, তাই `match` arm-গুলোতে
`Ok` আর `Err` variant-এর আগে `Result::` লিখতে হয় না।

ফলাফল `Ok` হলে এই কোড `Ok` variant থেকে ভেতরের `file` মানটি return করবে, আর তারপর আমরা সেই file handle মানটি
`greeting_file` variable-এ assign করি। `match`-এর পরে আমরা পড়া বা লেখার জন্য file handle-টি ব্যবহার করতে পারি।

`match`-এর অন্য arm-টি সেই ক্ষেত্রটি সামলায়, যেখানে আমরা `File::open` থেকে একটি `Err` মান পাই। এই উদাহরণে আমরা `panic!`
macro call করা বেছে নিয়েছি। আমাদের বর্তমান ডিরেক্টরিতে _hello.txt_ নামের কোনো ফাইল না থাকলে আর এই কোড চালালে আমরা `panic!`
macro থেকে নিচের আউটপুট দেখব:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

বরাবরের মতোই, এই আউটপুট আমাদের ঠিক বলে দেয় কী ভুল হয়েছে।

### আলাদা error-এর ওপর match করা {#matching-on-different-errors}

`File::open` যে কারণেই ব্যর্থ হোক, Listing 9-4-এর কোড `panic!` করবে। তবে আমরা ব্যর্থতার আলাদা কারণের জন্য আলাদা কাজ করতে
চাই। ফাইলটির অস্তিত্ব না থাকায় `File::open` ব্যর্থ হলে আমরা ফাইলটি তৈরি করে নতুন ফাইলের handle return করতে চাই। অন্য কোনো
কারণে `File::open` ব্যর্থ হলে—যেমন, ফাইলটি খোলার অনুমতি আমাদের না থাকলে—আমরা তবু চাই কোড Listing 9-4-এর মতোই `panic!`
করুক। এর জন্য আমরা একটি ভেতরের `match` expression যোগ করি, যা Listing 9-5-এ দেখানো হয়েছে।

<Listing number="9-5" file-name="src/main.rs" caption="আলাদা ধরনের error আলাদা উপায়ে সামলানো">

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

</Listing>

`Err` variant-এর ভেতরে `File::open` যে মান return করে, তার type হলো `io::Error`, যা standard library-র দেওয়া একটি struct।
এই struct-এর একটি method আছে, `kind`, যা call করে আমরা একটি `io::ErrorKind` মান পেতে পারি। `io::ErrorKind` enum standard
library-র দেওয়া, আর এর variant-গুলো একটি `io` operation থেকে ঘটতে পারে এমন আলাদা ধরনের error প্রকাশ করে। আমরা যে variant
ব্যবহার করতে চাই, তা হলো `ErrorKind::NotFound`, যা বোঝায় আমরা যে ফাইলটি খোলার চেষ্টা করছি, তার এখনো অস্তিত্ব নেই। তাই আমরা
`greeting_file_result`-এর ওপর match করি, কিন্তু সেই সঙ্গে `error.kind()`-এর ওপর একটি ভেতরের match-ও রাখি।

ভেতরের match-এ আমরা যে শর্ত যাচাই করতে চাই, তা হলো `error.kind()`-এর return করা মান `ErrorKind` enum-এর `NotFound` variant
কি না। তা হলে আমরা `File::create` দিয়ে ফাইলটি তৈরি করার চেষ্টা করি। তবে `File::create`-ও ব্যর্থ হতে পারে বলে ভেতরের
`match` expression-এ আমাদের দ্বিতীয় একটি arm দরকার। ফাইল তৈরি করা না গেলে একটি আলাদা error message প্রিন্ট হয়। বাইরের
`match`-এর দ্বিতীয় arm একই থাকে, তাই অনুপস্থিত ফাইলের error ছাড়া অন্য যেকোনো error-এ প্রোগ্রাম panic করে।

> #### `Result<T, E>`-এর সঙ্গে `match` ব্যবহারের বিকল্প {#alternatives-to-using-match-with-resultt-e}
>
> অনেক `match`! `match` expression খুবই কাজের, কিন্তু বেশ আদিম একটি গঠনও। অধ্যায় 13-এ আপনি closure (পারিপার্শ্বিক মান
> ধরে রাখতে পারে এমন নামহীন function) সম্পর্কে শিখবেন, যা `Result<T, E>`-এর ওপর সংজ্ঞায়িত অনেক method-এর সঙ্গে ব্যবহার হয়।
> আপনার কোডে `Result<T, E>` মান সামলানোর সময় এই method-গুলো `match` ব্যবহারের চেয়ে বেশি সংক্ষিপ্ত হতে পারে।
>
> যেমন, Listing 9-5-এ দেখানো একই যুক্তি লেখার আরেকটি উপায় এখানে দেওয়া হলো, এবার closure আর `unwrap_or_else` method
> ব্যবহার করে:
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Problem creating the file: {error:?}");
>             })
>         } else {
>             panic!("Problem opening the file: {error:?}");
>         }
>     });
> }
> ```
>
> এই কোডের আচরণ Listing 9-5-এর মতো হলেও এতে কোনো `match` expression নেই, আর পড়তেও বেশি পরিচ্ছন্ন। অধ্যায় 13 পড়ার পর এই
> উদাহরণে ফিরে আসুন, আর standard library-র ডকুমেন্টেশনে `unwrap_or_else` method-টি দেখে নিন। error নিয়ে কাজ করার সময় এ
> ধরনের আরও অনেক method বিশাল, একটির ভেতরে আরেকটি `match` expression পরিষ্কার করে দিতে পারে।

{{#quiz ../quizzes/ch09-02-recoverable-errors-sec1.toml}}

<!-- Old headings. Do not remove or links may break. -->

<a id="shortcuts-for-panic-on-error-unwrap-and-expect"></a>

#### error হলে panic করার শর্টকাট {#shortcuts-for-panic-on-error}

`match` ব্যবহার যথেষ্ট ভালো কাজ করে, কিন্তু একটু লম্বা-চওড়া হতে পারে, আর সবসময় উদ্দেশ্যটা ভালোভাবে প্রকাশ করে না।
`Result<T, E>` type-এর ওপর নানা ধরনের, আরও নির্দিষ্ট কাজ করার জন্য অনেক সহায়ক method সংজ্ঞায়িত আছে। `unwrap` method হলো
একটি শর্টকাট method, যা implement করা হয়েছে ঠিক Listing 9-4-এ আমাদের লেখা `match` expression-এর মতো। `Result` মান `Ok`
variant হলে `unwrap` `Ok`-এর ভেতরের মানটি return করবে। `Result` `Err` variant হলে `unwrap` আমাদের হয়ে `panic!` macro call
করবে। এখানে `unwrap`-এর কাজের একটি উদাহরণ:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

</Listing>

_hello.txt_ ফাইল ছাড়া এই কোড চালালে আমরা `unwrap` method-এর করা `panic!` call থেকে একটি error message দেখব:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-04-unwrap
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

একইভাবে, `expect` method আমাদের `panic!`-এর error message-ও বেছে নিতে দেয়। `unwrap`-এর বদলে `expect` ব্যবহার করে ভালো error
message দিলে আপনার উদ্দেশ্য প্রকাশ পায়, আর panic-এর উৎস খুঁজে বের করা সহজ হয়। `expect`-এর syntax দেখতে এরকম:

<Listing file-name="src/main.rs">

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

</Listing>

`unwrap`-এর মতো করেই আমরা `expect` ব্যবহার করি: file handle return করতে অথবা `panic!` macro call করতে। `panic!` call-এ
`expect` যে error message ব্যবহার করে, তা হবে `expect`-কে আমাদের দেওয়া parameter, `unwrap`-এর ব্যবহৃত ডিফল্ট `panic!` বার্তা
নয়। এটি দেখতে এরকম:

<!-- manual-regeneration
cd listings/ch09-error-handling/no-listing-05-expect
cargo run
copy and paste relevant text
-->

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

production-মানের কোডে বেশিরভাগ Rustacean `unwrap`-এর বদলে `expect` বেছে নেন, আর operation-টি কেন সবসময় সফল হবে বলে আশা
করা হচ্ছে, তার আরও প্রেক্ষাপট দেন। এভাবে আপনার ধারণা কখনো ভুল প্রমাণিত হলে ডিবাগ করার জন্য আপনার হাতে আরও তথ্য থাকবে।

### error propagate করা {#propagating-errors}

কোনো function-এর implementation যখন এমন কিছু call করে, যা ব্যর্থ হতে পারে, তখন function-এর ভেতরেই error সামলানোর বদলে আপনি
error-টি call করা কোডের কাছে return করতে পারেন, যাতে সে ঠিক করতে পারে কী করবে। একে বলা হয় error _propagating_ (error-কে call
করা কোডের দিকে ফেরত পাঠানো), আর এতে call করা কোডের হাতে বেশি নিয়ন্ত্রণ যায়, যেখানে error কীভাবে সামলানো উচিত তা নির্ধারণের
জন্য আপনার কোডের প্রেক্ষাপটে যা আছে, তার চেয়ে বেশি তথ্য বা যুক্তি থাকতে পারে।

যেমন, Listing 9-6-এ একটি function দেখানো হয়েছে, যা একটি ফাইল থেকে একটি username পড়ে। ফাইলটির অস্তিত্ব না থাকলে বা পড়া না
গেলে এই function সেই error-গুলো function-টি call করা কোডের কাছে return করবে।

<Listing number="9-6" file-name="src/main.rs" caption="`match` ব্যবহার করে call করা কোডের কাছে error return করে এমন একটি function">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-06/src/main.rs:here}}
```

</Listing>

এই function অনেক ছোট করে লেখা যায়, তবে error handling ঘুরে দেখার জন্য আমরা এর অনেকটা নিজ হাতে করে শুরু করব; শেষে ছোট উপায়টি
দেখাব। প্রথমে function-টির return type দেখা যাক: `Result<String, io::Error>`। এর মানে function-টি `Result<T, E>` type-এর একটি মান
return করছে, যেখানে generic parameter `T`-এর জায়গায় বসেছে নির্দিষ্ট type `String`, আর generic type `E`-এর জায়গায় বসেছে নির্দিষ্ট
type `io::Error`।

এই function কোনো সমস্যা ছাড়াই সফল হলে এটি call করা কোড একটি `Ok` মান পাবে, যাতে একটি `String` থাকে—এই function ফাইল থেকে যে
`username` পড়েছে। এই function কোনো সমস্যার মুখোমুখি হলে call করা কোড একটি `Err` মান পাবে, যাতে `io::Error`-এর একটি instance
থাকে, যাতে সমস্যাগুলো কী ছিল তার আরও তথ্য থাকে। এই function-এর return type হিসেবে আমরা `io::Error` বেছে নিয়েছি, কারণ এই
function-এর body-তে আমরা ব্যর্থ হতে পারে এমন যে দুটি operation call করছি, দুটি থেকেই return হওয়া error মানের type কাকতালীয়ভাবে
এটিই: `File::open` function আর `read_to_string` method।

function-এর body শুরু হয় `File::open` function call করে। তারপর আমরা Listing 9-4-এর `match`-এর মতো একটি `match` দিয়ে `Result`
মান সামলাই। `File::open` সফল হলে pattern variable `file`-এর file handle mutable variable `username_file`-এর মান হয়ে যায় এবং
function চলতে থাকে। `Err`-এর ক্ষেত্রে `panic!` call করার বদলে আমরা `return` keyword ব্যবহার করে আগেভাগেই function থেকে
পুরোপুরি বেরিয়ে যাই, আর `File::open`-এর error মান, যা এখন pattern variable `e`-তে আছে, এই function-এর error মান হিসেবে call করা
কোডের কাছে ফেরত পাঠাই।

তাহলে `username_file`-এ একটি file handle থাকলে function-টি `username` variable-এ একটি নতুন `String` তৈরি করে, এবং ফাইলের
বিষয়বস্তু `username`-এ পড়তে `username_file`-এর file handle-এর ওপর `read_to_string` method call করে। `File::open` সফল হলেও
`read_to_string` method ব্যর্থ হতে পারে বলে এটিও একটি `Result` return করে। তাই সেই `Result` সামলাতে আমাদের আরেকটি `match`
দরকার: `read_to_string` সফল হলে আমাদের function সফল হয়েছে, আর আমরা ফাইল থেকে পাওয়া username, যা এখন `username`-এ আছে,
একটি `Ok`-এ মুড়ে return করি। `read_to_string` ব্যর্থ হলে আমরা error মানটি সেভাবেই return করি, যেভাবে `File::open`-এর return
মান সামলানো `match`-এ error মান return করেছিলাম। তবে এখানে আমাদের স্পষ্টভাবে `return` বলতে হয় না, কারণ এটি function-এর শেষ
expression।

এই কোড call করা কোড তখন হয় একটি username থাকা `Ok` মান, নয়তো একটি `io::Error` থাকা `Err` মান সামলাবে। সেই মানগুলো নিয়ে কী
করবে, তা ঠিক করা call করা কোডের ব্যাপার। call করা কোড একটি `Err` মান পেলে, যেমন, এটি `panic!` call করে প্রোগ্রাম ক্র্যাশ
করাতে পারে, একটি ডিফল্ট username ব্যবহার করতে পারে, বা ফাইল ছাড়া অন্য কোথাও থেকে username খুঁজতে পারে। call করা কোড আসলে কী
করার চেষ্টা করছে, সে সম্পর্কে আমাদের কাছে যথেষ্ট তথ্য নেই, তাই আমরা সফলতা বা error-এর সব তথ্য ওপরের দিকে propagate করি, যাতে
সে উপযুক্তভাবে তা সামলাতে পারে।

error propagate করার এই ধরন Rust-এ এতই প্রচলিত যে এটি সহজ করতে Rust প্রশ্নবোধক চিহ্নের operator `?` দেয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="a-shortcut-for-propagating-errors-the--operator"></a>

#### `?` operator-এর শর্টকাট {#the--operator-shortcut}

Listing 9-7-এ `read_username_from_file`-এর এমন একটি implementation দেখানো হয়েছে, যার কার্যকারিতা Listing 9-6-এর মতোই, কিন্তু
এই implementation `?` operator ব্যবহার করে।

<Listing number="9-7" file-name="src/main.rs" caption="`?` operator ব্যবহার করে call করা কোডের কাছে error return করে এমন একটি function">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

</Listing>

একটি `Result` মানের পরে বসানো `?` এমনভাবে সংজ্ঞায়িত যে এটি প্রায় সেই `match` expression-গুলোর মতোই কাজ করে, যেগুলো আমরা
Listing 9-6-এ `Result` মান সামলাতে সংজ্ঞায়িত করেছিলাম। `Result`-এর মান একটি `Ok` হলে `Ok`-এর ভেতরের মানটি এই expression থেকে
return হবে, আর প্রোগ্রাম চলতে থাকবে। মানটি একটি `Err` হলে `Err`-টি পুরো function থেকে return হবে, যেন আমরা `return` keyword
ব্যবহার করেছি, যাতে error মানটি call করা কোডের কাছে propagate হয়।

Listing 9-6-এর `match` expression যা করে আর `?` operator যা করে, তার মধ্যে একটি পার্থক্য আছে: যেসব error মানের ওপর `?`
operator call করা হয়, সেগুলো `from` function-এর ভেতর দিয়ে যায়, যা standard library-র `From` trait-এ সংজ্ঞায়িত এবং যা এক
type-এর মানকে আরেক type-এ রূপান্তর করতে ব্যবহার হয়। `?` operator যখন `from` function call করে, তখন পাওয়া error type-টি
বর্তমান function-এর return type-এ সংজ্ঞায়িত error type-এ রূপান্তরিত হয়। এটি কাজে আসে যখন একটি function ব্যর্থ হতে পারে এমন
সব উপায় প্রকাশ করতে একটি error type return করে, এমনকি এর অংশগুলো অনেক আলাদা কারণে ব্যর্থ হতে পারলেও।

যেমন, আমরা Listing 9-7-এর `read_username_from_file` function বদলে আমাদের নিজের সংজ্ঞায়িত `OurError` নামের একটি নিজস্ব error type
return করাতে পারি। আমরা যদি একটি `io::Error` থেকে `OurError`-এর একটি instance তৈরি করতে `impl From<io::Error> for OurError`-ও
সংজ্ঞায়িত করি, তাহলে `read_username_from_file`-এর body-র `?` operator call-গুলো `from` call করে error type রূপান্তর করবে,
function-এ আর কোনো কোড যোগ না করেই।

Listing 9-7-এর প্রেক্ষাপটে, `File::open` call-এর শেষের `?` একটি `Ok`-এর ভেতরের মান `username_file` variable-এ return করবে।
কোনো error ঘটলে `?` operator আগেভাগেই পুরো function থেকে বেরিয়ে যাবে এবং যেকোনো `Err` মান call করা কোডকে দেবে।
`read_to_string` call-এর শেষের `?`-এর ক্ষেত্রেও একই কথা প্রযোজ্য।

`?` operator অনেক boilerplate (বারবার লিখতে হয় এমন গৎবাঁধা কোড) দূর করে এই function-এর implementation সহজ করে। `?`-এর ঠিক
পরেই method call chain (একটির পর একটি method call জুড়ে লেখা) করে আমরা এই কোড আরও ছোট করতে পারি, Listing 9-8-এ যেমন দেখানো
হয়েছে।

<Listing number="9-8" file-name="src/main.rs" caption="`?` operator-এর পরে method call chain করা">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-08/src/main.rs:here}}
```

</Listing>

`username`-এ নতুন `String` তৈরি করাটা আমরা function-এর শুরুতে সরিয়ে এনেছি; ওই অংশ বদলায়নি। `username_file` variable তৈরি করার
বদলে আমরা `read_to_string` call-টি সরাসরি `File::open("hello.txt")?`-এর ফলাফলের সঙ্গে chain করেছি। `read_to_string` call-এর শেষে
এখনো একটি `?` আছে, আর `File::open` আর `read_to_string` দুটোই error return না করে সফল হলে আমরা এখনো `username` থাকা একটি `Ok`
মান return করি। কার্যকারিতা আবারও Listing 9-6 আর Listing 9-7-এর মতোই; এটি শুধু লেখার একটি আলাদা, বেশি ব্যবহার-স্বাচ্ছন্দ্যপূর্ণ
উপায়।

Listing 9-9-এ `fs::read_to_string` ব্যবহার করে এটি আরও ছোট করার একটি উপায় দেখানো হয়েছে।

<Listing number="9-9" file-name="src/main.rs" caption="ফাইল খুলে তারপর পড়ার বদলে `fs::read_to_string` ব্যবহার করা">

<!-- Deliberately not using rustdoc_include here; the `main` function in the
file panics. We do want to include it for reader experimentation purposes, but
don't want to include it for rustdoc testing purposes. -->

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

</Listing>

একটি ফাইল একটি string-এ পড়া বেশ প্রচলিত একটি operation, তাই standard library সুবিধাজনক `fs::read_to_string` function দেয়, যা
ফাইল খোলে, একটি নতুন `String` তৈরি করে, ফাইলের বিষয়বস্তু পড়ে, সেই বিষয়বস্তু ওই `String`-এ রাখে, আর সেটি return করে। অবশ্যই,
`fs::read_to_string` ব্যবহার করলে সব error handling ব্যাখ্যা করার সুযোগ মেলে না, তাই আমরা প্রথমে লম্বা উপায়ে করেছি।

<!-- Old headings. Do not remove or links may break. -->

<a id="where-the--operator-can-be-used"></a>

#### `?` operator কোথায় ব্যবহার করবেন {#where-to-use-the--operator}

`?` operator শুধু এমন function-এ ব্যবহার করা যায়, যার return type `?` যে মানের ওপর ব্যবহার করা হচ্ছে, তার সঙ্গে সামঞ্জস্যপূর্ণ।
এর কারণ হলো, `?` operator এমনভাবে সংজ্ঞায়িত যে এটি function থেকে আগেভাগে একটি মান return করে, ঠিক যেভাবে Listing 9-6-এ আমাদের
সংজ্ঞায়িত `match` expression করেছিল। Listing 9-6-এ `match` একটি `Result` মান ব্যবহার করছিল, আর আগেভাগে return করা arm-টি একটি
`Err(e)` মান return করেছিল। এই `return`-এর সঙ্গে সামঞ্জস্যপূর্ণ হতে function-এর return type-কে একটি `Result` হতে হবে।

Listing 9-10-এ চলুন দেখি, এমন একটি `main` function-এ `?` operator ব্যবহার করলে কী error পাব, যার return type `?` যে মানের ওপর
ব্যবহার করা হচ্ছে, তার type-এর সঙ্গে সামঞ্জস্যপূর্ণ নয়।

<Listing number="9-10" file-name="src/main.rs" caption="`()` return করে এমন `main` function-এ `?` ব্যবহারের চেষ্টা compile হবে না।">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

</Listing>

এই কোড একটি ফাইল খোলে, যা ব্যর্থ হতে পারে। `?` operator `File::open`-এর return করা `Result` মানের পরে আছে, কিন্তু এই `main`
function-এর return type হলো `()`, `Result` নয়। এই কোড compile করলে আমরা নিচের error message পাই:

```console
{{#include ../listings/ch09-error-handling/listing-09-10/output.txt}}
```

এই error দেখিয়ে দেয় যে `?` operator ব্যবহারের অনুমতি আছে শুধু এমন function-এ, যা `Result`, `Option` বা `FromResidual` implement
করে এমন অন্য কোনো type return করে।

error-টি ঠিক করতে আপনার দুটি বিকল্প আছে। একটি বিকল্প হলো, কোনো বাধা না থাকলে, আপনার function-এর return type বদলে এমন করা, যা
`?` operator যে মানের ওপর ব্যবহার করছেন, তার সঙ্গে সামঞ্জস্যপূর্ণ। অন্য বিকল্পটি হলো একটি `match` বা `Result<T, E>`-এর কোনো একটি
method ব্যবহার করে `Result<T, E>`-কে উপযুক্ত যেকোনো উপায়ে সামলানো।

error message এটাও উল্লেখ করেছে যে `?` `Option<T>` মানের সঙ্গেও ব্যবহার করা যায়। `Result`-এ `?` ব্যবহারের মতোই, `Option`-এ `?`
শুধু এমন function-এ ব্যবহার করা যায়, যা একটি `Option` return করে। একটি `Option<T>`-এর ওপর call করলে `?` operator-এর আচরণ একটি
`Result<T, E>`-এর ওপর call করার আচরণের মতোই: মানটি `None` হলে ওই জায়গায় function থেকে আগেভাগে `None` return হবে। মানটি `Some`
হলে `Some`-এর ভেতরের মানটিই expression-এর ফলাফল-মান, আর function চলতে থাকে। Listing 9-11-এ এমন একটি function-এর উদাহরণ আছে,
যা দেওয়া লেখার প্রথম লাইনের শেষ অক্ষরটি খুঁজে বের করে।

<Listing number="9-11" caption="একটি `Option<T>` মানের ওপর `?` operator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-11/src/main.rs:here}}
```

</Listing>

এই function `Option<char>` return করে, কারণ সেখানে একটি অক্ষর থাকা সম্ভব, আবার না থাকাও সম্ভব। এই কোড `text` string slice
argument নিয়ে তার ওপর `lines` method call করে, যা string-এর লাইনগুলোর ওপর একটি iterator return করে। এই function প্রথম লাইনটি
পরীক্ষা করতে চায় বলে iterator থেকে প্রথম মানটি পেতে iterator-এর ওপর `next` call করে। `text` ফাঁকা string হলে `next`-এর এই call
`None` return করবে, আর সেই ক্ষেত্রে আমরা `?` ব্যবহার করে থেমে যাই এবং `last_char_of_first_line` থেকে `None` return করি। `text`
ফাঁকা string না হলে `next` একটি `Some` মান return করবে, যাতে `text`-এর প্রথম লাইনের একটি string slice থাকবে।

`?` string slice-টি বের করে আনে, আর আমরা সেই string slice-এর ওপর `chars` call করে এর অক্ষরগুলোর একটি iterator পেতে পারি। আমরা এই
প্রথম লাইনের শেষ অক্ষরে আগ্রহী, তাই iterator-এর শেষ আইটেমটি return করতে `last` call করি। এটি একটি `Option`, কারণ প্রথম লাইনটি
ফাঁকা string হওয়া সম্ভব; যেমন, `text` যদি একটি ফাঁকা লাইন দিয়ে শুরু হয়, কিন্তু অন্য লাইনে অক্ষর থাকে, যেমন `"\nhi"`। তবে প্রথম
লাইনে একটি শেষ অক্ষর থাকলে তা `Some` variant-এ return হবে। মাঝখানের `?` operator এই যুক্তি প্রকাশের একটি সংক্ষিপ্ত উপায় দেয়, যা
দিয়ে function-টি এক লাইনে implement করা যায়। `Option`-এ `?` operator ব্যবহার করতে না পারলে আমাদের আরও method call বা একটি
`match` expression দিয়ে এই যুক্তি implement করতে হতো।

লক্ষ করুন, `Result` return করে এমন function-এ আপনি একটি `Result`-এর ওপর `?` operator ব্যবহার করতে পারেন, আর `Option` return
করে এমন function-এ একটি `Option`-এর ওপর `?` operator ব্যবহার করতে পারেন, কিন্তু দুটো মিশিয়ে ব্যবহার করতে পারেন না। `?`
operator আপনা-আপনি একটি `Result`-কে `Option`-এ বা উল্টোটা রূপান্তর করবে না; সেসব ক্ষেত্রে রূপান্তরটি স্পষ্টভাবে করতে `Result`-এর
`ok` method বা `Option`-এর `ok_or` method-এর মতো method ব্যবহার করতে পারেন।

এ পর্যন্ত আমাদের ব্যবহৃত সব `main` function `()` return করেছে। `main` function বিশেষ, কারণ এটি একটি executable প্রোগ্রামের শুরু আর
শেষের জায়গা, আর প্রোগ্রাম প্রত্যাশামতো আচরণ করার জন্য এর return type কী হতে পারে, তা নিয়ে কিছু সীমাবদ্ধতা আছে।

সৌভাগ্যক্রমে, `main` একটি `Result<(), E>`-ও return করতে পারে। Listing 9-12-এ Listing 9-10-এর কোড আছে, তবে আমরা `main`-এর return
type বদলে `Result<(), Box<dyn Error>>` করেছি এবং শেষে একটি return মান `Ok(())` যোগ করেছি। এই কোড এখন compile হবে।

<Listing number="9-12" file-name="src/main.rs" caption="`main`-কে `Result<(), E>` return করার মতো করে বদলালে `Result` মানের ওপর `?` operator ব্যবহার করা যায়।">

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

</Listing>

`Box<dyn Error>` type হলো একটি trait object (trait-এর মাধ্যমে বিভিন্ন type ব্যবহার করার মান), যা নিয়ে আমরা অধ্যায় 18-এর [“Using
Trait Objects to Abstract over Shared Behavior”][trait-objects]<!-- ignore -->
অংশে কথা বলব। আপাতত `Box<dyn Error>`-কে আপনি “যেকোনো ধরনের error” হিসেবে পড়তে পারেন। error type `Box<dyn Error>`-সহ একটি `main`
function-এ একটি `Result` মানের ওপর `?` ব্যবহারের অনুমতি আছে, কারণ এতে যেকোনো `Err` মান আগেভাগে return করা যায়। এই `main`
function-এর body কেবল `std::io::Error` type-এর error return করলেও `Box<dyn Error>` নির্দিষ্ট করায় এই signature সঠিক থাকবে, এমনকি
`main`-এর body-তে অন্য error return করে এমন আরও কোড যোগ করা হলেও।

একটি `main` function যখন একটি `Result<(), E>` return করে, তখন `main` `Ok(())` return করলে executable `0` মান দিয়ে বন্ধ হবে, আর
`main` একটি `Err` মান return করলে শূন্য নয় এমন একটি মান দিয়ে বন্ধ হবে। C-তে লেখা executable বন্ধ হওয়ার সময় integer return করে:
সফলভাবে বন্ধ হওয়া প্রোগ্রাম integer `0` return করে, আর error হওয়া প্রোগ্রাম `0` ছাড়া অন্য কোনো integer return করে। এই রীতির সঙ্গে
সামঞ্জস্য রাখতে Rust-ও executable থেকে integer return করে।

`main` function এমন যেকোনো type return করতে পারে, যা [`std::process::Termination` trait][termination]<!-- ignore --> implement
করে, যাতে `report` নামের একটি function আছে, যা একটি `ExitCode` return করে। নিজের type-এর জন্য `Termination` trait implement করা
নিয়ে আরও তথ্যের জন্য standard library-র ডকুমেন্টেশন দেখুন।

`panic!` call করা বা `Result` return করার খুঁটিনাটি নিয়ে আলোচনা হলো, এবার চলুন সেই বিষয়ে ফিরে যাই যে কোন ক্ষেত্রে কোনটি ব্যবহার
করা উপযুক্ত, তা কীভাবে ঠিক করবেন।

{{#quiz ../quizzes/ch09-02-recoverable-errors-sec2.toml}}

[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-result
[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[termination]: https://doc.rust-lang.org/std/process/trait.Termination.html

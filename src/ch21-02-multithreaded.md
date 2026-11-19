<!-- Old headings. Do not remove or links may break. -->

<a id="turning-our-single-threaded-server-into-a-multithreaded-server"></a>
<a id="from-single-threaded-to-multithreaded-server"></a>

## single-threaded থেকে multithreaded server {#from-a-single-threaded-to-a-multithreaded-server}

এই মুহূর্তে server প্রতিটি request পালা করে প্রক্রিয়া করবে, অর্থাৎ প্রথম connection-এর প্রক্রিয়া শেষ না হওয়া পর্যন্ত দ্বিতীয় connection প্রক্রিয়া করবে না। server যত বেশি request পাবে, এই ক্রমানুসারী
চলা তত কম কার্যকর হবে। server যদি এমন একটি request পায় যা প্রক্রিয়া করতে অনেক সময় লাগে, তাহলে পরের request-গুলোকে সেই লম্বা request শেষ না হওয়া পর্যন্ত অপেক্ষা করতে হবে, যদিও নতুন request-গুলো দ্রুত
প্রক্রিয়া করা যেত। আমাদের এটি ঠিক করতে হবে, তবে প্রথমে সমস্যাটি ঘটতে দেখব।

<!-- Old headings. Do not remove or links may break. -->

<a id="simulating-a-slow-request-in-the-current-server-implementation"></a>

### একটি ধীর request-এর অনুকরণ {#simulating-a-slow-request}

আমরা দেখব ধীরে প্রক্রিয়া হওয়া একটি request আমাদের বর্তমান server implementation-এ করা অন্য request-গুলোকে কীভাবে প্রভাবিত করতে পারে। Listing 21-10 _/sleep_-এর একটি request সামলানোর ব্যবস্থা implement করে,
একটি অনুকৃত ধীর response দিয়ে, যা server-কে সাড়া দেওয়ার আগে পাঁচ সেকেন্ড sleep করাবে।

<Listing number="21-10" file-name="src/main.rs" caption="পাঁচ সেকেন্ড sleep করে একটি ধীর request-এর অনুকরণ">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-10/src/main.rs:here}}
```

</Listing>

এখন তিনটি ক্ষেত্র থাকায় আমরা `if` থেকে `match`-এ বদলেছি। string literal মানের সঙ্গে pattern-match করতে আমাদের স্পষ্টভাবে `request_line`-এর একটি slice-এর ওপর match করতে হয়; সমতার method-এর মতো `match`
স্বয়ংক্রিয়ভাবে reference ও dereference করে না।

প্রথম arm Listing 21-9-এর `if` block-এর মতোই। দ্বিতীয় arm _/sleep_-এর একটি request-এর সঙ্গে মেলে। সেই request পেলে server সফলতার HTML page দেখানোর আগে পাঁচ সেকেন্ড sleep করবে। তৃতীয় arm Listing
21-9-এর `else` block-এর মতোই।

দেখতেই পাচ্ছেন আমাদের server কতটা আদিম: আসল library-গুলো একাধিক request চেনার কাজ অনেক কম শব্দে সামলাত!

`cargo run` দিয়ে server চালু করুন। তারপর দুটি browser window খুলুন: একটি _http://127.0.0.1:7878_-এর জন্য, আরেকটি _http://127.0.0.1:7878/sleep_-এর জন্য। আগের মতো কয়েকবার _/_ URI দিলে দেখবেন
এটি দ্রুত সাড়া দেয়। কিন্তু _/sleep_ দিয়ে তারপর _/_ load করলে দেখবেন _/_ load হওয়ার আগে `sleep`-এর পুরো পাঁচ সেকেন্ড sleep শেষ হওয়া পর্যন্ত অপেক্ষা করে।

একটি ধীর request-এর পেছনে request জমে যাওয়া এড়াতে কয়েকটি কৌশল ব্যবহার করা যায়, যার মধ্যে আছে অধ্যায় 17-এর মতো async ব্যবহার; আমরা যেটি implement করব তা হলো একটি thread pool।

### thread pool দিয়ে throughput বাড়ানো {#improving-throughput-with-a-thread-pool}

একটি _thread pool_ হলো চালু করা thread-এর একটি দল, যারা একটি কাজ সামলানোর জন্য প্রস্তুত হয়ে অপেক্ষা করছে। প্রোগ্রাম একটি নতুন কাজ পেলে pool-এর একটি thread-কে কাজটি দেয়, আর সেই thread কাজটি
প্রক্রিয়া করবে। প্রথম thread প্রক্রিয়া করার সময় যে অন্য কাজ আসে, তা সামলাতে pool-এর বাকি thread-গুলো হাতে থাকে। প্রথম thread তার কাজ প্রক্রিয়া শেষ করলে অলস thread-এর pool-এ ফিরে যায়, নতুন কাজ
সামলাতে প্রস্তুত। thread pool আপনাকে concurrent-ভাবে connection প্রক্রিয়া করতে দেয়, ফলে আপনার server-এর throughput বাড়ে।

DoS আক্রমণ (অসংখ্য request পাঠিয়ে সেবা অচল করে দেওয়া) থেকে নিজেদের রক্ষা করতে আমরা pool-এর thread-এর সংখ্যা অল্পে সীমিত রাখব; প্রতিটি request আসার সঙ্গে সঙ্গে আমাদের প্রোগ্রাম যদি তার জন্য
একটি নতুন thread তৈরি করত, তাহলে কেউ আমাদের server-এ 1 কোটি request পাঠিয়ে server-এর সব resource শেষ করে দিয়ে request প্রক্রিয়া পুরোপুরি থামিয়ে দিয়ে তাণ্ডব ঘটাতে পারত।

তাই সীমাহীন thread চালু না করে আমরা pool-এ নির্দিষ্ট সংখ্যক thread অপেক্ষায় রাখব। যে request আসে সেগুলো প্রক্রিয়ার জন্য pool-এ পাঠানো হয়। pool আসা request-এর একটি queue (সারি) রাখবে। pool-এর প্রতিটি
thread এই queue থেকে একটি request তুলে নেবে, request-টি সামলাবে, তারপর queue-এর কাছে আরেকটি request চাইবে। এই নকশায় আমরা একসঙ্গে
_`N`_ পর্যন্ত request প্রক্রিয়া করতে পারি, যেখানে _`N`_ হলো thread-এর সংখ্যা। প্রতিটি thread যদি দীর্ঘ সময় ধরে চলা একটি request-এর সাড়া দিতে থাকে, তাহলে পরের request-গুলো তবুও queue-তে জমতে পারে,
তবে সেই অবস্থায় পৌঁছানোর আগে কতগুলো দীর্ঘ request সামলাতে পারি তার সংখ্যা আমরা বাড়িয়েছি।

এই কৌশল একটি web server-এর throughput বাড়ানোর অনেক উপায়ের একটি মাত্র। আরও যে বিকল্পগুলো খুঁজে দেখতে পারেন তা হলো fork/join model, single-threaded async I/O model আর multithreaded async I/O model। এই
বিষয়ে আগ্রহী হলে অন্য সমাধানগুলো সম্পর্কে আরও পড়ে সেগুলো implement করার চেষ্টা করতে পারেন; Rust-এর মতো একটি নিম্ন-স্তরের ভাষায় এই সব বিকল্পই সম্ভব।

thread pool implement করা শুরুর আগে চলুন কথা বলি pool ব্যবহার দেখতে কেমন হওয়া উচিত। কোড নকশা করার সময় প্রথমে client interface লিখলে তা আপনার নকশাকে পথ দেখাতে পারে। কোডের API এমনভাবে লিখুন, যেভাবে
আপনি একে call করতে চান; তারপর কার্যক্ষমতা implement করে public API নকশা না করে, বরং সেই কাঠামোর ভেতরে কার্যক্ষমতা implement করুন।

অধ্যায় 12-এর project-এ যেভাবে test-driven development ব্যবহার করেছিলাম, তেমনি এখানে compiler-driven development (compiler-এর error দেখে দেখে কোড এগিয়ে নেওয়া) ব্যবহার করব। আমরা কাঙ্ক্ষিত function
call করা কোড লিখব, তারপর কোড কাজ করাতে এরপর কী বদলাতে হবে তা ঠিক করতে compiler-এর error দেখব। তবে তার আগে শুরুর বিন্দু হিসেবে যে কৌশল ব্যবহার করব না সেটি দেখব।

<!-- Old headings. Do not remove or links may break. -->

<a id="code-structure-if-we-could-spawn-a-thread-for-each-request"></a>

#### প্রতিটি request-এর জন্য একটি thread চালু করা {#spawning-a-thread-for-each-request}

প্রথমে দেখা যাক প্রতিটি connection-এর জন্য একটি নতুন thread তৈরি করলে আমাদের কোড দেখতে কেমন হতো। আগেই যেমন বলা হয়েছে, সম্ভাব্য সীমাহীন সংখ্যক thread চালু করার সমস্যার কারণে এটি আমাদের চূড়ান্ত
পরিকল্পনা নয়, তবে প্রথমে একটি কাজ করা multithreaded server পাওয়ার জন্য এটি একটি শুরুর বিন্দু। তারপর উন্নতি হিসেবে thread pool যোগ করব, আর দুটি সমাধানের তুলনা করা সহজ হবে।

Listing 21-11-এ `for` loop-এর ভেতরে প্রতিটি stream সামলাতে একটি নতুন thread চালু করার জন্য `main`-এ যে পরিবর্তন করতে হবে তা দেখানো হয়েছে।

<Listing number="21-11" file-name="src/main.rs" caption="প্রতিটি stream-এর জন্য একটি নতুন thread চালু করা">

```rust,no_run
{{#rustdoc_include ../listings/ch21-web-server/listing-21-11/src/main.rs:here}}
```

</Listing>

অধ্যায় 16-এ যেমন শিখেছেন, `thread::spawn` একটি নতুন thread তৈরি করে তারপর নতুন thread-এ closure-এর কোড চালাবে। এই কোড চালিয়ে browser-এ _/sleep_ load করে, তারপর আরও দুটি browser tab-এ _/_ load করলে
সত্যিই দেখবেন _/_-এর request-গুলোকে _/sleep_ শেষ হওয়ার জন্য অপেক্ষা করতে হয় না। তবে আগেই যেমন বলেছি, এটি শেষ পর্যন্ত system-কে অতিরিক্ত চাপে ফেলবে, কারণ আপনি কোনো সীমা ছাড়াই নতুন thread তৈরি
করতে থাকবেন।

অধ্যায় 17 থেকে হয়তো মনে আছে, এটি ঠিক সেই ধরনের পরিস্থিতি যেখানে async আর await সত্যিই উজ্জ্বল! thread pool বানানোর সময় এটি মনে রাখুন, আর ভাবুন async দিয়ে কোন জিনিসগুলো ভিন্ন বা একই রকম দেখাত।

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-a-similar-interface-for-a-finite-number-of-threads"></a>

#### সীমিত সংখ্যক thread তৈরি করা {#creating-a-finite-number-of-threads}

আমরা চাই আমাদের thread pool একই রকম, পরিচিত উপায়ে কাজ করুক, যাতে thread থেকে thread pool-এ যেতে আমাদের API ব্যবহার করা কোডে বড় পরিবর্তন না লাগে। Listing 21-12-এ `thread::spawn`-এর বদলে আমরা যে
`ThreadPool` struct ব্যবহার করতে চাই, তার কাল্পনিক interface দেখানো হয়েছে।

<Listing number="21-12" file-name="src/main.rs" caption="আমাদের আদর্শ `ThreadPool` interface">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-12/src/main.rs:here}}
```

</Listing>

configure করা যায় এমন সংখ্যক thread-সহ, এই ক্ষেত্রে চারটি, একটি নতুন thread pool তৈরি করতে আমরা `ThreadPool::new` ব্যবহার করি। তারপর `for` loop-এ `pool.execute`-এর interface `thread::spawn`-এর মতো,
কারণ এটি এমন একটি closure নেয়, যা pool-এর প্রতিটি stream-এর জন্য চালানো উচিত। আমাদের `pool.execute` এমনভাবে implement করতে হবে যাতে এটি closure নিয়ে pool-এর একটি thread-কে চালাতে দেয়। এই কোড এখনো
compile হবে না, তবে চেষ্টা করব, যাতে compiler আমাদের দেখিয়ে দেয় কীভাবে ঠিক করতে হবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="building-the-threadpool-struct-using-compiler-driven-development"></a>

#### compiler-driven development দিয়ে `ThreadPool` তৈরি করা {#building-threadpool-using-compiler-driven-development}

_src/main.rs_-এ Listing 21-12-এর পরিবর্তনগুলো করুন, তারপর চলুন `cargo check`-এর compiler error দিয়ে আমাদের development এগিয়ে নিই। প্রথম যে error পাই তা এই:

```console
{{#include ../listings/ch21-web-server/listing-21-12/output.txt}}
```

চমৎকার! এই error বলছে আমাদের একটি `ThreadPool` type বা module দরকার, তাই এখনই একটি বানাব। আমাদের `ThreadPool` implementation আমাদের web server কোন ধরনের কাজ করছে তা থেকে স্বাধীন হবে। তাই চলুন
আমাদের `ThreadPool` implementation রাখতে `hello` crate-কে binary crate থেকে library crate-এ বদলে নিই। library crate-এ বদলানোর পর thread pool ব্যবহার করে যেকোনো কাজের জন্য, শুধু web request সামলানোর জন্য
নয়, আলাদা thread pool library ব্যবহার করতে পারব।

নিচেরটি থাকা একটি _src/lib.rs_ ফাইল তৈরি করুন, যা আপাতত আমাদের পক্ষে সম্ভব একটি `ThreadPool` struct-এর সবচেয়ে সরল সংজ্ঞা:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

</Listing>


তারপর _src/main.rs_-এর শুরুতে নিচের কোড যোগ করে _main.rs_ ফাইল সম্পাদনা করুন, যাতে library crate থেকে `ThreadPool` scope-এ আসে:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/src/main.rs:here}}
```

</Listing>

এই কোড এখনো কাজ করবে না, তবে পরের যে error-এর সমাধান করতে হবে তা পেতে চলুন আবার যাচাই করি:

```console
{{#include ../listings/ch21-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

এই error বোঝায় যে এরপর আমাদের `ThreadPool`-এর জন্য `new` নামের একটি associated function তৈরি করতে হবে। আমরা এটাও জানি যে `new`-এর এমন একটি parameter থাকতে হবে যা argument হিসেবে `4` গ্রহণ করতে পারে,
এবং এটি একটি `ThreadPool` instance return করবে। চলুন এই বৈশিষ্ট্যগুলোসহ সবচেয়ে সরল `new` function implement করি:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

</Listing>

`size` parameter-এর type হিসেবে আমরা `usize` বেছে নিয়েছি, কারণ জানি ঋণাত্মক সংখ্যক thread অর্থহীন। আমরা এটাও জানি যে এই `4`-কে thread-এর একটি collection-এর উপাদানের সংখ্যা হিসেবে ব্যবহার করব, আর
অধ্যায় 3-এর [“integer type”][integer-types]<!--
ignore --> অংশে যেমন আলোচনা করা হয়েছে, `usize` type ঠিক এই কাজের জন্যই।

চলুন আবার কোড যাচাই করি:

```console
{{#include ../listings/ch21-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

এখন error হচ্ছে, কারণ `ThreadPool`-এ কোনো `execute` method নেই। [“সীমিত সংখ্যক thread তৈরি
করা”](#creating-a-finite-number-of-threads)<!-- ignore --> অংশ থেকে মনে করুন, আমরা ঠিক করেছিলাম আমাদের thread pool-এর interface `thread::spawn`-এর মতো হওয়া উচিত। তা ছাড়া আমরা `execute` function এমনভাবে
implement করব যাতে এটি তাকে দেওয়া closure নিয়ে pool-এর একটি অলস thread-কে চালাতে দেয়।

`ThreadPool`-এ `execute` method এমনভাবে সংজ্ঞায়িত করব যাতে এটি parameter হিসেবে একটি closure নেয়। অধ্যায় 13-এর [“capture করা মান closure থেকে বের করে
নেওয়া”][moving-out-of-closures]<!-- ignore --> থেকে মনে করুন, তিনটি ভিন্ন trait দিয়ে closure-কে parameter হিসেবে নেওয়া যায়: `Fn`, `FnMut` আর `FnOnce`। এখানে কোন ধরনের closure ব্যবহার করব তা ঠিক করতে
হবে। আমরা জানি শেষ পর্যন্ত standard library-র `thread::spawn` implementation-এর মতো কিছু একটা করব, তাই দেখতে পারি `thread::spawn`-এর signature তার parameter-এর ওপর কোন bound রাখে। documentation আমাদের এটি
দেখায়:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

এখানে আমাদের মাথাব্যথা `F` type parameter নিয়ে; `T` type parameter return মানের সঙ্গে সম্পর্কিত, আর তা নিয়ে আমাদের মাথাব্যথা নেই। দেখতে পাচ্ছি `spawn` `F`-এর trait bound হিসেবে `FnOnce` ব্যবহার করে। সম্ভবত আমরাও
এটিই চাই, কারণ শেষ পর্যন্ত `execute`-এ পাওয়া argument আমরা `spawn`-এ পাঠাব। আরও নিশ্চিত হতে পারি যে `FnOnce`-ই আমাদের কাঙ্ক্ষিত trait, কারণ একটি request চালানোর thread সেই request-এর closure মাত্র একবার
চালাবে, যা `FnOnce`-এর `Once`-এর সঙ্গে মেলে।

`F` type parameter-এর trait bound `Send` আর lifetime bound `'static`-ও আছে, যা আমাদের পরিস্থিতিতে কাজের: closure এক thread থেকে আরেক thread-এ হস্তান্তর করতে আমাদের `Send` দরকার, আর thread চালাতে কত সময়
লাগবে তা জানি না বলে `'static` দরকার। চলুন `ThreadPool`-এ এমন একটি `execute` method তৈরি করি, যা এই bound-গুলোসহ `F` type-এর একটি generic parameter নেবে:

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

</Listing>

আমরা তবুও `FnOnce`-এর পরে `()` ব্যবহার করি, কারণ এই `FnOnce` এমন একটি closure বোঝায় যা কোনো parameter নেয় না এবং unit type `()` return করে। function-এর সংজ্ঞার মতোই signature থেকে return type বাদ দেওয়া
যায়, কিন্তু কোনো parameter না থাকলেও বন্ধনী লাগে।

আবারও, এটি `execute` method-এর সবচেয়ে সরল implementation: এটি কিছুই করে না, আমরা শুধু কোড compile করানোর চেষ্টা করছি। চলুন আবার যাচাই করি:

```console
{{#include ../listings/ch21-web-server/no-listing-03-define-execute/output.txt}}
```

এটি compile হয়! তবে খেয়াল করুন, `cargo run` চালিয়ে browser-এ একটি request করলে browser-এ সেই error-গুলোই দেখবেন যা অধ্যায়ের শুরুতে দেখেছিলাম। আমাদের library এখনো আসলে `execute`-এ পাঠানো closure call
করছে না!

> Note: Haskell আর Rust-এর মতো কড়া compiler-যুক্ত ভাষা নিয়ে একটি কথা হয়তো শুনবেন: “কোড compile হলে কাজ করে।” কিন্তু এই কথা সর্বত্র সত্য নয়। আমাদের project compile হয়, কিন্তু একেবারেই কিছু করে
> না! একটি আসল, পূর্ণাঙ্গ project বানালে এটি হতো unit test লেখা শুরু করার ভালো সময়, যাতে যাচাই করা যায় কোড compile হয় _এবং_ আমাদের কাঙ্ক্ষিত আচরণ করে।

ভেবে দেখুন: closure-এর বদলে আমরা যদি একটি future চালাতে যেতাম, তাহলে এখানে কী আলাদা হতো?

#### `new`-এ thread-এর সংখ্যা যাচাই করা {#validating-the-number-of-threads-in-new}

`new` আর `execute`-এর parameter দিয়ে আমরা কিছুই করছি না। চলুন কাঙ্ক্ষিত আচরণসহ এই function-গুলোর body implement করি। শুরুতে `new` নিয়ে ভাবা যাক। আগে আমরা `size` parameter-এর জন্য একটি unsigned (চিহ্নহীন)
type বেছে নিয়েছিলাম, কারণ ঋণাত্মক সংখ্যক thread-যুক্ত pool অর্থহীন। তবে শূন্য thread-যুক্ত pool-ও অর্থহীন, অথচ শূন্য একটি পুরোপুরি বৈধ `usize`। একটি `ThreadPool` instance return করার আগে `size` শূন্যের
চেয়ে বড় কি না তা যাচাইয়ের কোড যোগ করব, আর শূন্য পেলে `assert!` macro ব্যবহার করে প্রোগ্রামকে panic করাব, যেমনটি Listing 21-13-এ দেখানো হয়েছে।

<Listing number="21-13" file-name="src/lib.rs" caption="`size` শূন্য হলে panic করার জন্য `ThreadPool::new` implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-13/src/lib.rs:here}}
```

</Listing>

doc comment দিয়ে আমাদের `ThreadPool`-এর জন্য কিছু documentation-ও যোগ করেছি। খেয়াল করুন, অধ্যায় 14-এ যেমন আলোচনা করা হয়েছে, আমাদের function কোন পরিস্থিতিতে panic করতে পারে তা উল্লেখ করা একটি অংশ যোগ করে আমরা
ভালো documentation-এর চর্চা মেনেছি। `cargo doc --open` চালিয়ে `ThreadPool` struct-এ click করে দেখুন `new`-এর তৈরি হওয়া docs দেখতে কেমন!

এখানে যেমন `assert!` macro যোগ করেছি, তার বদলে `new`-কে `build`-এ বদলে একটি `Result` return করাতে পারতাম, যেমনটি Listing 12-9-এ I/O project-এ `Config::build`-এর ক্ষেত্রে করেছিলাম। কিন্তু এই ক্ষেত্রে আমরা
ঠিক করেছি যে কোনো thread ছাড়া thread pool তৈরির চেষ্টা একটি unrecoverable error হওয়া উচিত। উচ্চাভিলাষী মনে হলে `new` function-এর সঙ্গে তুলনা করতে এই signature-সহ `build` নামের একটি function লেখার চেষ্টা
করুন:

```rust,ignore
pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
```

#### thread রাখার জায়গা তৈরি করা {#creating-space-to-store-the-threads}

pool-এ রাখার জন্য thread-এর সংখ্যা বৈধ কি না তা জানার উপায় এখন আমাদের আছে, তাই struct return করার আগে সেই thread-গুলো তৈরি করে `ThreadPool` struct-এ রাখতে পারি। কিন্তু একটি thread কীভাবে “রাখি”? চলুন
`thread::spawn` signature আরেকবার দেখি:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

`spawn` function একটি `JoinHandle<T>` return করে, যেখানে `T` হলো closure যে type return করে। চলুন আমরাও `JoinHandle` ব্যবহার করে দেখি কী ঘটে। আমাদের ক্ষেত্রে thread pool-এ পাঠানো closure-গুলো connection
সামলাবে এবং কিছু return করবে না, তাই `T` হবে unit type `()`।

Listing 21-14-এর কোড compile হবে, তবে এটি এখনো কোনো thread তৈরি করে না। আমরা `ThreadPool`-এর সংজ্ঞা বদলে `thread::JoinHandle<()>` instance-এর একটি vector রাখিয়েছি, `size` ধারণক্ষমতাসহ vector-টির
প্রাথমিক মান দিয়েছি, thread তৈরির কিছু কোড চালাবে এমন একটি `for` loop প্রস্তুত করেছি, আর সেগুলো রাখা একটি `ThreadPool` instance return করেছি।

<Listing number="21-14" file-name="src/lib.rs" caption="thread রাখার জন্য `ThreadPool`-এর একটি vector তৈরি করা">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-14/src/lib.rs:here}}
```

</Listing>

library crate-এ আমরা `std::thread` scope-এ এনেছি, কারণ `ThreadPool`-এর vector-এর item-গুলোর type হিসেবে `thread::JoinHandle` ব্যবহার করছি।

একটি বৈধ আকার পেলে আমাদের `ThreadPool` একটি নতুন vector তৈরি করে, যা `size` সংখ্যক item রাখতে পারে। `with_capacity` function `Vec::new`-এর মতোই কাজ করে, তবে একটি গুরুত্বপূর্ণ পার্থক্যসহ: এটি vector-এ আগাম
জায়গা allocate করে। আমরা যেহেতু জানি vector-এ `size` সংখ্যক উপাদান রাখতে হবে, তাই শুরুতেই এই allocation করা `Vec::new` ব্যবহারের চেয়ে একটু বেশি দক্ষ, যা উপাদান ঢোকানোর সময় নিজের আকার বদলায়।

আবার `cargo check` চালালে এটি সফল হওয়ার কথা।

<!-- Old headings. Do not remove or links may break. -->
<a id ="a-worker-struct-responsible-for-sending-code-from-the-threadpool-to-a-thread"></a>

#### `ThreadPool` থেকে একটি thread-এ কোড পাঠানো {#sending-code-from-the-threadpool-to-a-thread}

Listing 21-14-এর `for` loop-এ thread তৈরি নিয়ে আমরা একটি comment রেখেছিলাম। এখানে দেখব আসলে কীভাবে thread তৈরি করি। standard library thread তৈরির উপায় হিসেবে `thread::spawn` দেয়, আর
`thread::spawn` আশা করে thread তৈরি হওয়ার সঙ্গে সঙ্গে thread-এর চালানোর মতো কিছু কোড পাবে। কিন্তু আমাদের ক্ষেত্রে আমরা thread তৈরি করে সেগুলোকে পরে পাঠানো কোডের জন্য _অপেক্ষা_ করাতে চাই। standard
library-র thread implementation-এ এটা করার কোনো উপায় নেই; আমাদের এটি হাতে implement করতে হবে।

`ThreadPool` আর thread-গুলোর মাঝে একটি নতুন data structure এনে আমরা এই আচরণ implement করব, যা এই নতুন আচরণ সামলাবে। এই data structure-কে আমরা বলব _Worker_ (কর্মী), যা pooling implementation-এ একটি
প্রচলিত পরিভাষা। `Worker` চালাতে হবে এমন কোড তুলে নিয়ে নিজের thread-এ সেই কোড চালায়।

একটি রেস্তোরাঁর রান্নাঘরে কাজ করা মানুষের কথা ভাবুন: কর্মীরা গ্রাহকদের কাছ থেকে order আসা পর্যন্ত অপেক্ষা করে, তারপর সেই order নিয়ে তা পূরণের দায়িত্ব পালন করে।

thread pool-এ `JoinHandle<()>` instance-এর একটি vector না রেখে আমরা `Worker` struct-এর instance রাখব। প্রতিটি `Worker` একটি করে `JoinHandle<()>` instance রাখবে। তারপর `Worker`-এ এমন একটি method implement
করব, যা চালানোর জন্য কোডের একটি closure নিয়ে চলমান thread-এ চালানোর জন্য পাঠাবে। প্রতিটি `Worker`-কে একটি `id`-ও দেব, যাতে log বা debug করার সময় pool-এর বিভিন্ন `Worker` instance আলাদা করে চেনা যায়।

একটি `ThreadPool` তৈরি করলে এই নতুন প্রক্রিয়া ঘটবে। এভাবে `Worker` প্রস্তুত করার পর আমরা thread-এ closure পাঠানোর কোড implement করব:

1. একটি `id` আর একটি `JoinHandle<()>` রাখা একটি `Worker` struct সংজ্ঞায়িত করা।
2. `ThreadPool` বদলে `Worker` instance-এর একটি vector রাখানো।
3. এমন একটি `Worker::new` function সংজ্ঞায়িত করা, যা একটি `id` সংখ্যা নিয়ে এমন একটি `Worker` instance return করে, যাতে `id` আর একটি খালি closure দিয়ে চালু করা একটি thread থাকে।
4. `ThreadPool::new`-এ `for` loop-এর counter দিয়ে একটি `id` তৈরি করে সেই `id`-সহ একটি নতুন `Worker` তৈরি করা, এবং `Worker`-টি vector-এ রাখা।

চ্যালেঞ্জের জন্য প্রস্তুত থাকলে Listing 21-15-এর কোড দেখার আগে নিজে এই পরিবর্তনগুলো implement করার চেষ্টা করুন।

প্রস্তুত? আগের পরিবর্তনগুলো করার একটি উপায়সহ Listing 21-15 এখানে দেওয়া হলো।

<Listing number="21-15" file-name="src/lib.rs" caption="সরাসরি thread না রেখে `Worker` instance রাখার জন্য `ThreadPool` পরিবর্তন করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-15/src/lib.rs:here}}
```

</Listing>

`ThreadPool`-এর field-এর নাম `threads` থেকে বদলে `workers` করেছি, কারণ এটি এখন `JoinHandle<()>` instance-এর বদলে `Worker` instance রাখছে। `for` loop-এর counter আমরা `Worker::new`-এর argument হিসেবে
ব্যবহার করি, আর প্রতিটি নতুন `Worker` `workers` নামের vector-এ রাখি।

বাইরের কোডের (যেমন _src/main.rs_-এর আমাদের server) `ThreadPool`-এর ভেতরে `Worker` struct ব্যবহারের implementation-এর খুঁটিনাটি জানার দরকার নেই, তাই আমরা `Worker` struct আর এর `new` function private রাখি।
`Worker::new` function আমাদের দেওয়া `id` ব্যবহার করে এবং একটি খালি closure দিয়ে নতুন thread চালু করে তৈরি হওয়া একটি `JoinHandle<()>` instance রাখে।

> Note: যথেষ্ট system resource না থাকায় operating system একটি thread তৈরি করতে না পারলে `thread::spawn` panic করবে। তাতে আমাদের পুরো server panic করবে, যদিও কিছু thread তৈরি সফল হতে পারত। সরলতার
> জন্য এই আচরণ ঠিক আছে, তবে production-এর thread pool implementation-এ হয়তো এর বদলে
> [`std::thread::Builder`][builder]<!-- ignore --> আর এর
> [`spawn`][builder-spawn]<!-- ignore --> method ব্যবহার করতে চাইবেন, যা `Result` return করে।

এই কোড compile হবে এবং `ThreadPool::new`-এর argument হিসেবে উল্লেখ করা সংখ্যক `Worker` instance রাখবে। কিন্তু `execute`-এ পাওয়া closure আমরা _এখনো_ প্রক্রিয়া করছি না। এরপর দেখা যাক কীভাবে তা করতে হয়।

#### channel দিয়ে thread-এ request পাঠানো {#sending-requests-to-threads-via-channels}

এরপর আমরা যে সমস্যার মোকাবিলা করব তা হলো, `thread::spawn`-এ দেওয়া closure-গুলো একেবারেই কিছু করে না। এই মুহূর্তে আমরা যে closure চালাতে চাই তা `execute` method-এ পাই। কিন্তু `ThreadPool` তৈরির সময়
প্রতিটি `Worker` তৈরি করতে গিয়ে `thread::spawn`-কে চালানোর জন্য একটি closure দিতে হবে।

আমরা চাই এইমাত্র তৈরি করা `Worker` struct-গুলো `ThreadPool`-এ রাখা একটি queue থেকে চালানোর কোড নিয়ে নিজের thread-এ চালানোর জন্য পাঠাক।

অধ্যায় 16-এ শেখা channel—দুটি thread-এর মধ্যে যোগাযোগের একটি সরল উপায়—এই কাজের জন্য একেবারে উপযুক্ত হবে। কাজের (job-এর) queue হিসেবে আমরা একটি channel ব্যবহার করব, আর `execute` `ThreadPool` থেকে
`Worker` instance-গুলোতে একটি job পাঠাবে, যা job-টি নিজের thread-এ পাঠাবে। পরিকল্পনা এই:

1. `ThreadPool` একটি channel তৈরি করে sender ধরে রাখবে।
2. প্রতিটি `Worker` receiver ধরে রাখবে।
3. আমরা একটি নতুন `Job` struct তৈরি করব, যা channel দিয়ে পাঠাতে চাওয়া closure-গুলো রাখবে।
4. `execute` method যে job চালাতে চায় তা sender দিয়ে পাঠাবে।
5. নিজের thread-এ `Worker` তার receiver-এর ওপর দিয়ে loop করবে এবং পাওয়া যেকোনো job-এর closure চালাবে।

চলুন `ThreadPool::new`-এ একটি channel তৈরি করে `ThreadPool` instance-এ sender ধরে রাখা দিয়ে শুরু করি, যেমনটি Listing 21-16-এ দেখানো হয়েছে। `Job` struct আপাতত কিছু রাখে না, তবে channel দিয়ে আমরা যে item
পাঠাচ্ছি এটি হবে তার type।

<Listing number="21-16" file-name="src/lib.rs" caption="`Job` instance পাঠানো একটি channel-এর sender রাখতে `ThreadPool` পরিবর্তন করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-16/src/lib.rs:here}}
```

</Listing>

`ThreadPool::new`-এ আমরা নতুন channel তৈরি করে pool-কে sender ধরে রাখতে দিই। এটি সফলভাবে compile হবে।

thread pool channel তৈরি করার সময় চলুন প্রতিটি `Worker`-এ channel-এর একটি receiver পাঠানোর চেষ্টা করি। আমরা জানি `Worker` instance-গুলো যে thread চালু করে, সেখানে receiver ব্যবহার করতে চাই, তাই closure-এ
`receiver` parameter-কে নির্দেশ করব। Listing 21-17-এর কোড এখনো পুরোপুরি compile হবে না।

<Listing number="21-17" file-name="src/lib.rs" caption="প্রতিটি `Worker`-এ receiver পাঠানো">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-17/src/lib.rs:here}}
```

</Listing>

আমরা কয়েকটি ছোট ও সোজাসাপ্টা পরিবর্তন করেছি: `Worker::new`-এ receiver পাঠাই, তারপর closure-এর ভেতরে এটি ব্যবহার করি।

এই কোড যাচাই করার চেষ্টা করলে এই error পাই:

```console
{{#include ../listings/ch21-web-server/listing-21-17/output.txt}}
```

কোডটি একাধিক `Worker` instance-এ `receiver` পাঠানোর চেষ্টা করছে। এটি কাজ করবে না, যেমনটি অধ্যায় 16 থেকে মনে থাকবে: Rust-এর দেওয়া channel implementation হলো একাধিক _producer_, একক _consumer_। এর মানে
এই কোড ঠিক করতে channel-এর গ্রহণের প্রান্তটি clone করা যায় না। একাধিক consumer-এর কাছে একটি message একাধিকবার পাঠাতেও চাই না; আমরা চাই একাধিক `Worker` instance-সহ message-এর একটি তালিকা, যাতে প্রতিটি
message একবার প্রক্রিয়া হয়।

তা ছাড়া channel-এর queue থেকে একটি job তুলে নিতে `receiver` পরিবর্তন করতে হয়, তাই thread-গুলোর `receiver` ভাগ করার ও পরিবর্তন করার একটি নিরাপদ উপায় দরকার; নইলে race condition হতে পারে (অধ্যায় 16-এ
যেমন আলোচনা করা হয়েছে)।

অধ্যায় 16-এ আলোচিত thread-safe smart pointer মনে করুন: একাধিক thread-এর মধ্যে ownership ভাগ করতে এবং thread-গুলোকে মান পরিবর্তন করতে দিতে আমাদের `Arc<Mutex<T>>` ব্যবহার করতে হবে। `Arc` type একাধিক
`Worker` instance-কে receiver-এর মালিক হতে দেবে, আর `Mutex` নিশ্চিত করবে যে একবারে শুধু একটি `Worker` receiver থেকে একটি job পায়। Listing 21-18-এ প্রয়োজনীয় পরিবর্তনগুলো দেখানো হয়েছে।

<Listing number="21-18" file-name="src/lib.rs" caption="`Arc` আর `Mutex` ব্যবহার করে `Worker` instance-গুলোর মধ্যে receiver ভাগ করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-18/src/lib.rs:here}}
```

</Listing>

`ThreadPool::new`-এ আমরা receiver-কে একটি `Arc` আর একটি `Mutex`-এ রাখি। প্রতিটি নতুন `Worker`-এর জন্য reference count বাড়াতে `Arc` clone করি, যাতে `Worker` instance-গুলো receiver-এর ownership ভাগ করে
নিতে পারে।

এই পরিবর্তনগুলোর ফলে কোড compile হয়! আমরা লক্ষ্যের কাছাকাছি পৌঁছে যাচ্ছি!

#### `execute` method implement করা {#implementing-the-execute-method}

চলুন অবশেষে `ThreadPool`-এ `execute` method implement করি। `Job`-কেও একটি struct থেকে বদলে এমন একটি trait object-এর type alias করব, যা `execute`-এর পাওয়া closure-এর type রাখে। অধ্যায় 20-এর [“type synonym
ও type alias”][type-aliases]<!-- ignore --> অংশে যেমন আলোচনা করা হয়েছে, type alias ব্যবহারের সুবিধার জন্য লম্বা type ছোট করতে দেয়। Listing 21-19 দেখুন।

<Listing number="21-19" file-name="src/lib.rs" caption="প্রতিটি closure রাখা একটি `Box`-এর জন্য একটি `Job` type alias তৈরি করে তারপর channel দিয়ে job পাঠানো">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-19/src/lib.rs:here}}
```

</Listing>

`execute`-এ পাওয়া closure ব্যবহার করে একটি নতুন `Job` instance তৈরি করার পর আমরা সেই job channel-এর পাঠানোর প্রান্ত দিয়ে পাঠাই। পাঠানো ব্যর্থ হওয়ার ক্ষেত্রের জন্য `send`-এর ওপর `unwrap` call করছি। এটা
ঘটতে পারে যদি, যেমন, আমরা আমাদের সব thread চলা বন্ধ করে দিই, অর্থাৎ গ্রহণের প্রান্ত নতুন message গ্রহণ বন্ধ করে দিয়েছে। এই মুহূর্তে আমরা thread-গুলোর চলা থামাতে পারি না: pool যতক্ষণ আছে, আমাদের thread-গুলো
ততক্ষণ চলতে থাকে। `unwrap` ব্যবহারের কারণ হলো আমরা জানি ব্যর্থতার ক্ষেত্রটি ঘটবে না, কিন্তু compiler তা জানে না।

তবে কাজ এখনো পুরোপুরি শেষ হয়নি! `Worker`-এ `thread::spawn`-এ পাঠানো আমাদের closure এখনো শুধু channel-এর গ্রহণের প্রান্তকে _নির্দেশ_ করে। বরং আমাদের দরকার closure চিরকাল loop করুক, channel-এর গ্রহণের
প্রান্তের কাছে একটি job চাক এবং পেলে job-টি চালাক। চলুন `Worker::new`-এ Listing 21-20-এ দেখানো পরিবর্তনটি করি।

<Listing number="21-20" file-name="src/lib.rs" caption="`Worker` instance-এর thread-এ job গ্রহণ করে চালানো">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-20/src/lib.rs:here}}
```

</Listing>

এখানে আমরা প্রথমে mutex পেতে `receiver`-এর ওপর `lock` call করি, তারপর যেকোনো error-এ panic করতে `unwrap` call করি। mutex _poisoned_ (দূষিত) অবস্থায় থাকলে lock নেওয়া ব্যর্থ হতে পারে, যা ঘটতে পারে যদি অন্য
কোনো thread lock ছেড়ে না দিয়ে ধরে রাখা অবস্থায় panic করে। এই পরিস্থিতিতে এই thread-কে panic করাতে `unwrap` call করাই সঠিক পদক্ষেপ। চাইলে এই `unwrap`-কে আপনার কাছে অর্থবহ একটি error message-সহ `expect`-এ
বদলে দিতে পারেন।

mutex-এর lock পেলে channel থেকে একটি `Job` গ্রহণ করতে `recv` call করি। শেষ একটি `unwrap` এখানকার যেকোনো error-ও পার হয়ে যায়, যা ঘটতে পারে যদি sender ধরে রাখা thread বন্ধ হয়ে যায়, ঠিক যেমন receiver বন্ধ
হলে `send` method `Err` return করে।

`recv`-এর call block করে, তাই এখনো কোনো job না থাকলে বর্তমান thread একটি job পাওয়া পর্যন্ত অপেক্ষা করবে। `Mutex<T>` নিশ্চিত করে যে একবারে শুধু একটি `Worker` thread একটি job চাওয়ার চেষ্টা করছে।

আমাদের thread pool এখন কাজ করার অবস্থায় আছে! একবার `cargo run` চালিয়ে কয়েকটি request করুন:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field `workers` is never read
 --> src/lib.rs:7:5
  |
6 | pub struct ThreadPool {
  |            ---------- field in this struct
7 |     workers: Vec<Worker>,
  |     ^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: fields `id` and `thread` are never read
  --> src/lib.rs:48:5
   |
47 | struct Worker {
   |        ------ fields in this struct
48 |     id: usize,
   |     ^^
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^

warning: `hello` (lib) generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.91s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```

সফল! এখন আমাদের কাছে এমন একটি thread pool আছে, যা asynchronous-ভাবে connection চালায়। চারটির বেশি thread কখনো তৈরি হয় না, তাই server অনেক request পেলেও আমাদের system অতিরিক্ত চাপে পড়বে না। _/sleep_-এ
একটি request করলে server অন্য request-গুলো আরেকটি thread দিয়ে চালিয়ে সামলাতে পারবে।

> Note: একাধিক browser window-এ একসঙ্গে _/sleep_ খুললে সেগুলো হয়তো পাঁচ সেকেন্ড পরপর একটি একটি করে load হবে। কিছু web browser caching-এর কারণে একই request-এর একাধিক instance ক্রমানুসারে চালায়। এই সীমাবদ্ধতা
> আমাদের web server-এর কারণে নয়।

এটি একটু থেমে ভাবার ভালো সময়, করণীয় কাজের জন্য closure-এর বদলে future ব্যবহার করলে Listing 21-18, 21-19
আর 21-20-এর কোড কীভাবে আলাদা হতো। কোন type বদলাত? method signature কীভাবে আলাদা হতো, আদৌ হতো কি? কোডের কোন অংশগুলো একই থাকত?

অধ্যায় 17 আর অধ্যায় 19-এ `while let` loop সম্পর্কে শেখার পর হয়তো ভাবছেন, `Worker` thread-এর কোড কেন Listing 21-21-এর মতো লিখিনি।

<Listing number="21-21" file-name="src/lib.rs" caption="`while let` ব্যবহার করে `Worker::new`-এর একটি বিকল্প implementation">

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-21/src/lib.rs:here}}
```

</Listing>

এই কোড compile হয়ে চলে, কিন্তু কাঙ্ক্ষিত threading আচরণ দেয় না: একটি ধীর request তবুও অন্য request-গুলোকে প্রক্রিয়ার জন্য অপেক্ষা করাবে। কারণটি একটু সূক্ষ্ম: `Mutex` struct-এর কোনো public `unlock` method নেই,
কারণ lock-এর ownership নির্ভর করে `lock` method-এর return করা `LockResult<MutexGuard<T>>`-এর ভেতরের `MutexGuard<T>`-এর lifetime-এর ওপর। compile time-এ borrow checker তখন এই নিয়ম প্রয়োগ করতে পারে যে
lock না ধরে রাখলে একটি `Mutex`-এর পাহারায় থাকা resource-এ প্রবেশ করা যায় না। তবে `MutexGuard<T>`-এর lifetime নিয়ে সতর্ক না থাকলে এই implementation-এ lock উদ্দেশ্যের চেয়ে বেশি সময় ধরে থাকতে পারে।

Listing 21-20-এর যে কোড `let job =
receiver.lock().unwrap().recv().unwrap();` ব্যবহার করে তা কাজ করে, কারণ `let`-এর ক্ষেত্রে সমান চিহ্নের ডান পাশের expression-এ ব্যবহৃত যেকোনো সাময়িক মান `let` statement শেষ হওয়ার সঙ্গে সঙ্গে drop হয়ে যায়।
কিন্তু `while
let` (এবং `if let` ও `match`) সংশ্লিষ্ট block শেষ না হওয়া পর্যন্ত সাময়িক মান drop করে না। Listing 21-21-এ `job()` call চলার পুরো সময় lock ধরা থাকে, অর্থাৎ অন্য `Worker` instance-গুলো job পেতে পারে না।

[type-aliases]: ch20-03-advanced-types.html#type-synonyms-and-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[moving-out-of-closures]: ch13-01-closures.html#moving-captured-values-out-of-closures
[builder]: https://doc.rust-lang.org/std/thread/struct.Builder.html
[builder-spawn]: https://doc.rust-lang.org/std/thread/struct.Builder.html#method.spawn

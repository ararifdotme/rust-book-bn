## future ও async syntax {#futures-and-the-async-syntax}

Rust-এ asynchronous programming-এর মূল উপাদান হলো _futures_ (future) আর Rust-এর
`async` ও `await` keyword।

_future_ হলো এমন একটি মান, যা এখন প্রস্তুত না-ও থাকতে পারে, কিন্তু ভবিষ্যতে কোনো এক সময় প্রস্তুত হবে। (একই ধারণা অনেক ভাষায় আছে, কখনো কখনো অন্য নামে, যেমন _task_ বা _promise_।)
Rust একটি মৌলিক উপাদান হিসেবে `Future` trait দেয়, যাতে বিভিন্ন async operation ভিন্ন ভিন্ন data structure দিয়ে, কিন্তু একটি সাধারণ interface-সহ implement করা যায়। Rust-এ future
হলো এমন type, যা `Future` trait implement করে। প্রতিটি future নিজের অগ্রগতির তথ্য আর “প্রস্তুত” বলতে কী বোঝায় তা নিজের কাছে রাখে।

block আর function-এ `async` keyword ব্যবহার করে বলে দেওয়া যায় যে এগুলো থামানো ও আবার চালু করা যায়। একটি async block বা async function-এর ভেতরে `await` keyword ব্যবহার করে
একটি _future-এর জন্য await করা_ (অর্থাৎ এটি প্রস্তুত হওয়ার জন্য অপেক্ষা করা) যায়। async block বা function-এর ভেতরে যেখানেই একটি future-এর জন্য await করেন, সেটিই সেই block বা
function-এর থামা ও আবার চালু হওয়ার একটি সম্ভাব্য জায়গা। একটি future-এর মান ইতিমধ্যে পাওয়া যাচ্ছে কি না তা যাচাই করার প্রক্রিয়াকে বলা হয় _polling_।

C# আর JavaScript-এর মতো আরও কিছু ভাষাও async programming-এর জন্য `async` আর `await` keyword ব্যবহার করে। সেই ভাষাগুলোর সঙ্গে পরিচিত হলে আপনি হয়তো খেয়াল করবেন যে Rust
syntax-টি যেভাবে সামলায় তাতে কিছু উল্লেখযোগ্য পার্থক্য আছে। দেখব, এর পেছনে ভালো কারণ আছে!

async Rust লেখার সময় বেশিরভাগ সময় আমরা `async` আর `await` keyword ব্যবহার করি। Rust এগুলোকে `Future` trait ব্যবহার করা সমতুল্য কোডে compile করে, অনেকটা যেভাবে `for` loop-কে
`Iterator` trait ব্যবহার করা সমতুল্য কোডে compile করে। তবে Rust যেহেতু `Future` trait দেয়, তাই দরকার হলে নিজের data type-এর জন্যও এটি implement করতে পারেন। এই অধ্যায় জুড়ে
আমরা যে অনেক function দেখব, সেগুলো এমন type return করে যাদের `Future`-এর নিজস্ব implementation আছে। অধ্যায়ের শেষে আমরা trait-টির সংজ্ঞায় ফিরে আসব এবং এটি কীভাবে কাজ করে তা
আরও গভীরভাবে দেখব, তবে এগিয়ে যাওয়ার জন্য এটুকু খুঁটিনাটিই যথেষ্ট।

এসব হয়তো একটু বিমূর্ত লাগছে, তাই চলুন আমাদের প্রথম async প্রোগ্রাম লিখি: একটি ছোট web scraper (web page থেকে তথ্য সংগ্রহের প্রোগ্রাম)। আমরা command line থেকে দুটি URL দেব, দুটিই
concurrent-ভাবে নিয়ে আসব, আর যেটি আগে শেষ হয় তার ফল return করব। এই উদাহরণে বেশ কিছু নতুন syntax থাকবে, তবে চিন্তা করবেন না—এগোতে এগোতে যা যা জানা দরকার সব ব্যাখ্যা করব।

## আমাদের প্রথম async প্রোগ্রাম {#our-first-async-program}

ecosystem-এর বিভিন্ন অংশ নিয়ে নাড়াচাড়া না করে এই অধ্যায়ের মনোযোগ async শেখার দিকে রাখতে আমরা `trpl` crate তৈরি করেছি (`trpl` হলো “The Rust
Programming Language”-এর সংক্ষেপ)। এটি আপনার দরকারি সব type, trait আর function re-export করে, মূলত [`futures`][futures-crate]<!-- ignore --> আর
[`tokio`][tokio]<!-- ignore --> crate থেকে। `futures` crate হলো async কোড নিয়ে Rust-এর পরীক্ষা-নিরীক্ষার একটি দাপ্তরিক আবাস, আর আসলে `Future` trait প্রথমে এখানেই নকশা করা
হয়েছিল। Tokio আজ Rust-এ সবচেয়ে বেশি ব্যবহৃত async runtime, বিশেষ করে web application-এর জন্য। আরও অনেক চমৎকার runtime আছে, আর সেগুলো হয়তো আপনার উদ্দেশ্যের জন্য বেশি
উপযুক্ত। আমরা `trpl`-এর ভেতরে `tokio` crate ব্যবহার করি, কারণ এটি ভালোভাবে পরীক্ষিত এবং ব্যাপকভাবে ব্যবহৃত।

কিছু ক্ষেত্রে `trpl` মূল API-গুলোর নাম বদলে দেয় বা মুড়ে দেয়, যাতে আপনার মনোযোগ এই অধ্যায়ের প্রাসঙ্গিক খুঁটিনাটিতে থাকে। crate-টি কী করে তা বুঝতে চাইলে [এর source code][crate-source]
দেখার পরামর্শ দিই। সেখানে দেখতে পাবেন প্রতিটি re-export কোন crate থেকে এসেছে, আর crate-টি কী করে তা ব্যাখ্যা করে আমরা বিস্তারিত comment রেখেছি।

`hello-async` নামে একটি নতুন binary project তৈরি করুন এবং dependency হিসেবে `trpl` crate যোগ করুন:

```console
$ cargo new hello-async
$ cd hello-async
$ cargo add trpl
```

এখন `trpl`-এর দেওয়া বিভিন্ন অংশ ব্যবহার করে আমাদের প্রথম async প্রোগ্রাম লিখতে পারি। আমরা একটি ছোট command line tool বানাব, যা দুটি web page নিয়ে আসে, প্রতিটি থেকে `<title>`
element বের করে, এবং যে page পুরো প্রক্রিয়াটি আগে শেষ করে তার title print করে।

### page_title function সংজ্ঞায়িত করা {#defining-the-page_title-function}

চলুন এমন একটি function লেখা দিয়ে শুরু করি, যা parameter হিসেবে একটি page-এর URL নেয়, সেখানে একটি request পাঠায়, এবং `<title>` element-এর লেখা return করে (Listing
17-1 দেখুন)।

<Listing number="17-1" file-name="src/main.rs" caption="একটি HTML page থেকে title element পেতে একটি async function সংজ্ঞায়িত করা">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-01/src/main.rs:all}}
```

</Listing>

প্রথমে আমরা `page_title` নামের একটি function সংজ্ঞায়িত করে একে `async` keyword দিয়ে চিহ্নিত করি। তারপর পাঠানো URL নিয়ে আসতে `trpl::get` function ব্যবহার করি এবং response-এর
জন্য await করতে `await` keyword যোগ করি। `response`-এর লেখা পেতে এর `text` method call করি এবং আবার `await` keyword দিয়ে এর জন্য await করি। এই দুটি ধাপই asynchronous। `get`
function-এর ক্ষেত্রে server-এর response-এর প্রথম অংশ ফেরত পাঠানোর জন্য আমাদের অপেক্ষা করতে হয়, যাতে HTTP header, cookie ইত্যাদি থাকে এবং যা response body থেকে আলাদাভাবে
আসতে পারে। বিশেষ করে body খুব বড় হলে সবটা আসতে কিছু সময় লাগতে পারে। যেহেতু আমাদের _পুরো_ response আসার জন্য অপেক্ষা করতে হয়, তাই `text` method-ও async।

এই দুটি future-এর জন্যই আমাদের স্পষ্টভাবে await করতে হয়, কারণ Rust-এ future _lazy_: `await` keyword দিয়ে না বলা পর্যন্ত এরা কিছুই করে না। (আসলে কোনো future ব্যবহার না করলে Rust
একটি compiler warning দেখাবে।) এটি হয়তো অধ্যায় 13-এর [“iterator দিয়ে একের পর এক item প্রক্রিয়া
করা”][iterators-lazy]<!-- ignore --> অংশে iterator নিয়ে আলোচনার কথা মনে করিয়ে দেবে। iterator-এর `next` method call না করলে—সরাসরি, অথবা `for` loop বা `map`-এর মতো
method ব্যবহার করে, যেগুলো ভেতরে ভেতরে `next` ব্যবহার করে—iterator কিছুই করে না। একইভাবে, স্পষ্টভাবে না বললে future কিছুই করে না। এই lazy স্বভাবের কারণে Rust async কোড সত্যিই
দরকার না হওয়া পর্যন্ত চালানো এড়াতে পারে।

> Note: অধ্যায় 16-এর [“`spawn` দিয়ে নতুন thread তৈরি করা”][thread-spawn]<!-- ignore -->
> অংশে `thread::spawn` ব্যবহারের সময় যে আচরণ দেখেছিলাম, এটি তা থেকে আলাদা, যেখানে অন্য thread-এ পাঠানো closure সঙ্গে সঙ্গে চলতে শুরু করেছিল। অন্য অনেক ভাষা async-কে যেভাবে
> দেখে, তা থেকেও এটি আলাদা। কিন্তু Rust-এর performance-এর নিশ্চয়তা দিতে পারার জন্য এটি গুরুত্বপূর্ণ, ঠিক যেমন iterator-এর ক্ষেত্রে।

`response_text` পেয়ে গেলে `Html::parse` ব্যবহার করে একে `Html` type-এর একটি instance-এ parse করতে পারি। একটি কাঁচা string-এর বদলে এখন আমাদের কাছে এমন একটি data type আছে, যা
দিয়ে HTML-কে আরও সমৃদ্ধ data structure হিসেবে ব্যবহার করা যায়। বিশেষ করে, একটি নির্দিষ্ট CSS selector-এর প্রথম instance খুঁজতে `select_first` method ব্যবহার করতে পারি। string
`"title"` পাঠালে document-এর প্রথম `<title>` element পাব, যদি থাকে। কোনো মিলে যাওয়া element না-ও থাকতে পারে বলে `select_first` একটি `Option<ElementRef>` return করে। শেষে আমরা
`Option::map` method ব্যবহার করি, যা `Option`-এ item থাকলে তা নিয়ে কাজ করতে দেয়, আর না থাকলে কিছুই করে না। (এখানে একটি `match` expression-ও ব্যবহার করা যেত, তবে `map` বেশি
idiomatic।) `map`-এ দেওয়া function-এর body-তে আমরা `title`-এর ওপর `inner_html` call করে এর বিষয়বস্তু পাই, যা একটি `String`। সব শেষে আমাদের কাছে থাকে একটি `Option<String>`।

খেয়াল করুন, Rust-এর `await` keyword যে expression-এর জন্য await করছেন তার আগে নয়, _পরে_ বসে। অর্থাৎ এটি একটি _postfix_ (পরে বসা) keyword। অন্য ভাষায় `async` ব্যবহার করে থাকলে এটি
আপনার অভ্যাসের থেকে আলাদা হতে পারে, কিন্তু Rust-এ এতে method-এর শিকল নিয়ে কাজ করা অনেক সহজ হয়। ফলে আমরা `page_title`-এর body বদলে `trpl::get` আর `text` function call-গুলো
মাঝখানে `await` রেখে একসঙ্গে জুড়ে দিতে পারি, যেমনটি Listing 17-2-এ দেখানো হয়েছে।

<Listing number="17-2" file-name="src/main.rs" caption="`await` keyword দিয়ে শিকল তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-02/src/main.rs:chaining}}
```

</Listing>

এর মাধ্যমে আমরা সফলভাবে আমাদের প্রথম async function লিখে ফেলেছি! একে call করতে `main`-এ কিছু কোড যোগ করার আগে চলুন আরেকটু আলোচনা করি আমরা কী লিখেছি এবং এর মানে কী।

Rust যখন `async` keyword দিয়ে চিহ্নিত একটি _block_ দেখে, তখন একে `Future` trait implement করা একটি অনন্য, নামহীন data type-এ compile করে। Rust যখন `async` দিয়ে চিহ্নিত একটি
_function_ দেখে, তখন একে এমন একটি non-async function-এ compile করে, যার body একটি async block। একটি async function-এর return type হলো সেই async block-এর জন্য compiler-এর তৈরি
নামহীন data type-এর type।

তাই `async fn` লেখা এমন একটি function লেখার সমতুল্য, যা return type-এর একটি _future_ return করে। compiler-এর কাছে Listing 17-1-এর `async fn page_title`-এর মতো একটি function-এর
সংজ্ঞা মোটামুটি এভাবে সংজ্ঞায়িত একটি non-async function-এর সমতুল্য:

```rust
# extern crate trpl; // required for mdbook test
use std::future::Future;
use trpl::Html;

fn page_title(url: &str) -> impl Future<Output = Option<String>> {
    async move {
        let text = trpl::get(url).await.text().await;
        Html::parse(&text)
            .select_first("title")
            .map(|title| title.inner_html())
    }
}
```

রূপান্তরিত সংস্করণের প্রতিটি অংশ একে একে দেখা যাক:

- এটি অধ্যায় 10-এর [“trait-কে parameter হিসেবে ব্যবহার করা”][impl-trait]<!-- ignore --> অংশে আলোচিত `impl Trait` syntax ব্যবহার করে।
- return করা মান `Output` associated type-সহ `Future` trait implement করে। খেয়াল করুন, `Output` type হলো `Option<String>`, যা `page_title`-এর `async fn` সংস্করণের মূল return
  type-এর সমান।
- মূল function-এর body-তে call করা সব কোড একটি `async move` block-এ মোড়ানো। মনে রাখবেন, block হলো expression। এই পুরো block-টিই function থেকে return হওয়া expression।
- এইমাত্র যেমন বর্ণনা করা হলো, এই async block `Option<String>` type-এর একটি মান তৈরি করে। সেই মান return type-এর `Output` type-এর সঙ্গে মেলে। আপনার দেখা অন্য block-গুলোর মতোই।
- নতুন function body একটি `async move` block, কারণ এটি `url` parameter ব্যবহার করে। (`async` বনাম `async move` নিয়ে অধ্যায়ের পরের দিকে আরও অনেক কথা বলব।)

এবার `main`-এ `page_title` call করতে পারি।

<!-- Old headings. Do not remove or links may break. -->

<a id ="determining-a-single-pages-title"></a>

### runtime দিয়ে একটি async function চালানো {#executing-an-async-function-with-a-runtime}

শুরুতে আমরা একটিমাত্র page-এর title নেব, যেমনটি Listing 17-3-এ দেখানো হয়েছে। দুর্ভাগ্যবশত, এই কোড এখনো compile হয় না।

<Listing number="17-3" file-name="src/main.rs" caption="ব্যবহারকারীর দেওয়া argument দিয়ে `main` থেকে `page_title` function call করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-03/src/main.rs:main}}
```

</Listing>

অধ্যায় 12-এর [“command line argument গ্রহণ করা”][cli-args]<!-- ignore --> অংশে command line argument নিতে যে pattern ব্যবহার করেছিলাম, এখানেও তা-ই অনুসরণ করি। তারপর URL
argument `page_title`-এ পাঠিয়ে ফলের জন্য await করি। future-এর তৈরি মান যেহেতু একটি `Option<String>`, তাই page-টিতে `<title>` ছিল কি না তা বিবেচনা করে ভিন্ন ভিন্ন message print
করতে একটি `match` expression ব্যবহার করি।

`await` keyword শুধু async function বা block-এর ভেতরেই ব্যবহার করা যায়, আর Rust আমাদের বিশেষ `main` function-কে `async` হিসেবে চিহ্নিত করতে দেবে না।

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-03
cargo build
copy just the compiler error
-->

```text
error[E0752]: `main` function is not allowed to be `async`
 --> src/main.rs:6:1
  |
6 | async fn main() {
  | ^^^^^^^^^^^^^^^ `main` function is not allowed to be `async`
```

`main`-কে `async` হিসেবে চিহ্নিত করা যায় না কারণ async কোডের একটি _runtime_ দরকার: এমন একটি Rust crate, যা asynchronous কোড চালানোর খুঁটিনাটি সামলায়। একটি প্রোগ্রামের `main`
function একটি runtime _চালু করতে_ পারে, কিন্তু এটি _নিজে_ কোনো runtime নয়। (কেন এমন, তা একটু পরেই আরও দেখব।) async কোড চালানো প্রতিটি Rust প্রোগ্রামে অন্তত একটি জায়গা থাকে, যেখানে
এটি future চালানোর জন্য একটি runtime প্রস্তুত করে।

async সমর্থন করে এমন বেশিরভাগ ভাষা সঙ্গে একটি runtime দেয়, কিন্তু Rust দেয় না। বরং অনেক ভিন্ন ভিন্ন async runtime পাওয়া যায়, যার প্রতিটি তার লক্ষ্য করা ব্যবহারের জন্য উপযুক্ত
ভিন্ন ভিন্ন আপস করে। যেমন, অনেক CPU core আর প্রচুর RAM-যুক্ত একটি উচ্চ-throughput web server-এর চাহিদা একটিমাত্র core, অল্প RAM আর heap allocation-এর ক্ষমতা নেই এমন একটি
microcontroller-এর চাহিদা থেকে একেবারে আলাদা। এই runtime-গুলো দেয় এমন crate প্রায়ই ফাইল বা network I/O-র মতো সাধারণ কার্যক্ষমতার async সংস্করণও দেয়।

এখানে, এবং এই অধ্যায়ের বাকি অংশ জুড়ে, আমরা `trpl` crate-এর `block_on` function ব্যবহার করব, যা argument হিসেবে একটি future নেয় এবং future-টি শেষ পর্যন্ত চলা না পর্যন্ত বর্তমান
thread block করে রাখে। পর্দার আড়ালে `block_on` call করলে `tokio` crate ব্যবহার করে একটি runtime প্রস্তুত হয়, যা পাঠানো future চালাতে ব্যবহৃত হয় (`trpl` crate-এর `block_on`-এর
আচরণ অন্য runtime crate-গুলোর `block_on` function-এর মতোই)। future শেষ হলে `block_on` future-এর তৈরি মানটি return করে।

`page_title`-এর return করা future সরাসরি `block_on`-এ পাঠাতে পারতাম, আর এটি শেষ হলে Listing 17-3-এ যেমন চেষ্টা করেছিলাম তেমনভাবে ফলাফলের `Option<String>`-এর ওপর match করতে
পারতাম। তবে এই অধ্যায়ের বেশিরভাগ উদাহরণে (আর বাস্তবের বেশিরভাগ async কোডে) আমরা শুধু একটি async function call-এর চেয়ে বেশি কিছু করব, তাই এর বদলে আমরা একটি `async` block
পাঠাব এবং `page_title` call-এর ফলের জন্য স্পষ্টভাবে await করব, যেমনটি Listing 17-4-এ আছে।

<Listing number="17-4" caption="`trpl::block_on` দিয়ে একটি async block-এর জন্য await করা" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook test does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-04/src/main.rs:run}}
```

</Listing>

এই কোড চালালে শুরুতে যে আচরণ আশা করেছিলাম তা পাই:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-04
cargo build # skip all the build noise
cargo run -- "https://www.rust-lang.org"
# copy the output here
-->

```console
$ cargo run -- "https://www.rust-lang.org"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/async_await 'https://www.rust-lang.org'`
The title for https://www.rust-lang.org was
            Rust Programming Language
```

যাক—অবশেষে আমাদের কাছে কাজ করে এমন async কোড আছে! তবে দুটি site-এর মধ্যে প্রতিযোগিতা করানোর কোড যোগ করার আগে চলুন সংক্ষেপে আবার future কীভাবে কাজ করে সেদিকে মনোযোগ দিই।

প্রতিটি _await point_—অর্থাৎ কোডে যেখানে যেখানে `await` keyword ব্যবহার হয়েছে—এমন একটি জায়গা বোঝায় যেখানে নিয়ন্ত্রণ runtime-এর হাতে ফিরিয়ে দেওয়া হয়। এটি কাজ করাতে Rust-কে async
block-এ জড়িত অবস্থার হিসাব রাখতে হয়, যাতে runtime অন্য কোনো কাজ শুরু করতে পারে এবং প্রথমটি আবার এগিয়ে নেওয়ার চেষ্টা করতে প্রস্তুত হলে ফিরে আসতে পারে। এটি একটি অদৃশ্য state
machine (অবস্থা পরিবর্তনের যন্ত্র), যেন প্রতিটি await point-এ বর্তমান অবস্থা সংরক্ষণের জন্য আপনি এমন একটি enum লিখেছেন:

```rust
{{#rustdoc_include ../listings/ch17-async-await/no-listing-state-machine/src/lib.rs:enum}}
```

তবে প্রতিটি অবস্থার মধ্যে বদলের কোড হাতে লেখা একঘেয়ে এবং ভুলপ্রবণ হতো, বিশেষ করে যখন পরে কোডে আরও কার্যক্ষমতা আর আরও অবস্থা যোগ করতে হয়। সৌভাগ্যক্রমে Rust compiler async কোডের
জন্য state machine-এর data structure স্বয়ংক্রিয়ভাবে তৈরি ও সামলায়। data structure নিয়ে borrowing আর ownership-এর সাধারণ নিয়মগুলো এখনো প্রযোজ্য, আর আনন্দের কথা, compiler আমাদের
হয়ে সেগুলো যাচাইও করে এবং কাজের error message দেয়। অধ্যায়ের পরের দিকে এর কয়েকটি নিয়ে কাজ করব।

শেষ পর্যন্ত কোনো কিছুকে এই state machine চালাতে হয়, আর সেই কিছু হলো একটি runtime। (এ কারণেই runtime নিয়ে খোঁজ নেওয়ার সময় হয়তো _executors_ (executor)-এর উল্লেখ পাবেন: executor
হলো runtime-এর সেই অংশ, যা async কোড চালানোর দায়িত্বে থাকে।)

এখন বুঝতে পারছেন, Listing 17-3-এ compiler কেন আমাদের `main`-কে নিজেই একটি async function বানাতে দেয়নি। `main` যদি একটি async function হতো, তাহলে `main`-এর return করা future-এর
state machine অন্য কাউকে সামলাতে হতো, কিন্তু `main`-ই তো প্রোগ্রামের শুরুর জায়গা! বরং আমরা `main`-এ `trpl::block_on` function call করে একটি runtime প্রস্তুত করেছি এবং `async`
block-এর return করা future শেষ না হওয়া পর্যন্ত চালিয়েছি।

> Note: কিছু runtime এমন macro দেয়, যাতে আপনি একটি async `main` function লিখতে _পারেন_। সেই macro-গুলো `async fn main() { ... }`-কে নতুন করে লিখে একটি সাধারণ `fn
> main` বানায়, যা Listing 17-4-এ আমরা হাতে যা করেছি তা-ই করে: এমন একটি function call করে, যা `trpl::block_on`-এর মতো করে একটি future শেষ পর্যন্ত চালায়।

এবার এই অংশগুলো একসঙ্গে জুড়ে দেখি কীভাবে concurrent কোড লেখা যায়।

<!-- Old headings. Do not remove or links may break. -->

<a id="racing-our-two-urls-against-each-other"></a>

### দুটি URL-এর মধ্যে concurrent-ভাবে প্রতিযোগিতা করানো {#racing-two-urls-against-each-other-concurrently}

Listing 17-5-এ আমরা command line থেকে পাঠানো দুটি ভিন্ন URL দিয়ে `page_title` call করি এবং যে future আগে শেষ হয় সেটি বেছে নিয়ে দুটির মধ্যে প্রতিযোগিতা করাই।

<Listing number="17-5" caption="কোনটি আগে return করে দেখতে দুটি URL-এর জন্য `page_title` call করা" file-name="src/main.rs">

<!-- should_panic,noplayground because mdbook does not pass args -->

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch17-async-await/listing-17-05/src/main.rs:all}}
```

</Listing>

প্রথমে ব্যবহারকারীর দেওয়া প্রতিটি URL-এর জন্য `page_title` call করি। ফলাফলের future-গুলো `title_fut_1` আর `title_fut_2` হিসেবে রাখি। মনে রাখবেন, এগুলো এখনো কিছুই করে না, কারণ
future lazy এবং আমরা এখনো এগুলোর জন্য await করিনি। তারপর future-গুলো `trpl::select`-এ পাঠাই, যা একটি মান return করে, যা বোঝায় তাকে পাঠানো future-গুলোর মধ্যে কোনটি আগে শেষ হয়।

> Note: ভেতরে ভেতরে `trpl::select` তৈরি `futures` crate-এ সংজ্ঞায়িত আরও সাধারণ একটি `select` function-এর ওপর। `futures` crate-এর `select` function এমন অনেক কিছু করতে পারে যা
> `trpl::select` function পারে না, তবে এর কিছু বাড়তি জটিলতাও আছে, যা আপাতত এড়িয়ে যেতে পারি।

যেকোনো future-ই বৈধভাবে “জিততে” পারে, তাই একটি `Result` return করা অর্থবহ নয়। বরং `trpl::select` এমন একটি type return করে যা আমরা আগে দেখিনি,
`trpl::Either`। `Either` type কিছুটা `Result`-এর মতো, কারণ এর দুটি অবস্থা আছে। তবে `Result`-এর মতো নয়, `Either`-এ সাফল্য বা ব্যর্থতার কোনো ধারণা নেই। বরং এটি “একটি অথবা
অন্যটি” বোঝাতে `Left` আর `Right` ব্যবহার করে:

```rust
enum Either<A, B> {
    Left(A),
    Right(B),
}
```

প্রথম argument জিতলে `select` function সেই future-এর output-সহ `Left` return করে, আর _দ্বিতীয়টি_ জিতলে দ্বিতীয় future argument-এর output-সহ `Right` return করে। function call
করার সময় argument-গুলো যে ক্রমে থাকে তার সঙ্গে এটি মেলে: প্রথম argument দ্বিতীয় argument-এর বাম দিকে।

আমরা `page_title`-ও হালনাগাদ করি, যাতে এটি পাঠানো একই URL return করে। এতে যে page আগে return করে তাতে যদি আমাদের বের করার মতো কোনো `<title>` না থাকে, তবুও একটি অর্থবহ message
print করতে পারি। সেই তথ্য হাতে নিয়ে শেষে আমরা `println!` output হালনাগাদ করি, যাতে এটি বোঝায় কোন URL আগে শেষ হয়েছে এবং সেই URL-এর web page-এর `<title>` কী, যদি থাকে।

আপনি এখন একটি ছোট, কাজ করা web scraper বানিয়ে ফেলেছেন! কয়েকটি URL বেছে নিয়ে command line tool-টি চালান। হয়তো দেখবেন কিছু site সবসময় অন্যগুলোর চেয়ে দ্রুত, আবার অন্য ক্ষেত্রে
দ্রুততর site প্রতিবার চালানোয় বদলে যায়। আরও গুরুত্বপূর্ণ হলো, আপনি future নিয়ে কাজ করার মূল বিষয়গুলো শিখেছেন, তাই এখন async দিয়ে আর কী করা যায় তা আরও গভীরভাবে দেখতে পারি।

{{#quiz ../quizzes/async-01-futures-and-syntax.toml}}

[impl-trait]: ch10-02-traits.html#traits-as-parameters
[iterators-lazy]: ch13-02-iterators.html
[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[cli-args]: ch12-01-accepting-command-line-arguments.html

<!-- TODO: map source link version to version of Rust? -->

[crate-source]: https://github.com/rust-lang/book/tree/main/packages/trpl
[futures-crate]: https://crates.io/crates/futures
[tokio]: https://tokio.rs

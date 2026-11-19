<!-- Old headings. Do not remove or links may break. -->

<a id="traits-defining-shared-behavior"></a>

## trait দিয়ে সাধারণ আচরণ সংজ্ঞায়িত করা {#defining-shared-behavior-with-traits}

একটি _trait_ (টাইপের সাধারণ আচরণের সংজ্ঞা) সংজ্ঞায়িত করে কোনো নির্দিষ্ট type-এর কী কী কার্যক্ষমতা আছে, যা সে অন্য
type-এর সঙ্গে ভাগ করে নিতে পারে। trait দিয়ে আমরা বিমূর্তভাবে সাধারণ আচরণ সংজ্ঞায়িত করতে পারি। আর _trait bound_
(generic type-এর ওপর trait-এর শর্ত) দিয়ে বলে দিতে পারি যে একটি generic type এমন যেকোনো type হতে পারে যার নির্দিষ্ট
কিছু আচরণ আছে।

> Note: অন্যান্য ভাষায় প্রায়ই _interfaces_ (ইন্টারফেস) নামে পরিচিত একটি বৈশিষ্ট্যের সঙ্গে trait-এর মিল আছে, যদিও কিছু
> পার্থক্যও আছে।

### একটি trait সংজ্ঞায়িত করা {#defining-a-trait}

একটি type-এর আচরণ বলতে বোঝায় সেই type-এর ওপর আমরা কোন কোন method call করতে পারি। যদি একাধিক type-এর ওপর একই
method call করা যায়, তাহলে সেই type-গুলো একই আচরণ ভাগ করে নেয়। trait-এর সংজ্ঞা হলো method signature-গুলোকে একসঙ্গে
জড়ো করে কোনো উদ্দেশ্য পূরণের জন্য প্রয়োজনীয় আচরণের একটি সেট সংজ্ঞায়িত করার উপায়।

যেমন ধরুন, আমাদের কয়েকটি struct আছে যারা বিভিন্ন ধরনের ও পরিমাণের লেখা রাখে: একটি `NewsArticle` struct, যা কোনো
নির্দিষ্ট জায়গা থেকে পাঠানো একটি সংবাদ রাখে, আর একটি `SocialPost`, যাতে সর্বোচ্চ 280টি অক্ষর থাকতে পারে, সঙ্গে থাকে
মেটাডেটা, যা বলে এটি নতুন post, repost, নাকি অন্য কোনো post-এর উত্তর।

আমরা `aggregator` নামে একটি media aggregator (বিভিন্ন উৎসের লেখা এক জায়গায় জড়ো করে এমন) library crate বানাতে চাই, যা
`NewsArticle` বা `SocialPost` instance-এ রাখা ডেটার সারসংক্ষেপ দেখাতে পারে। এজন্য প্রতিটি type থেকে একটি সারসংক্ষেপ
দরকার, আর সেটি আমরা চাইব একটি instance-এর ওপর `summarize` method call করে। Listing 10-12-এ একটি public `Summary` trait-এর
সংজ্ঞা দেখানো হয়েছে, যা এই আচরণ প্রকাশ করে।

<Listing number="10-12" file-name="src/lib.rs" caption="একটি `Summary` trait, যা একটি `summarize` method-এর দেওয়া আচরণ নিয়ে গঠিত">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

</Listing>

এখানে আমরা `trait` keyword আর তারপর trait-এর নাম, এই ক্ষেত্রে `Summary`, লিখে একটি trait ঘোষণা করি। trait-টিকে `pub`
হিসেবেও ঘোষণা করি, যাতে এই crate-এর ওপর নির্ভরশীল অন্য crate-ও এই trait ব্যবহার করতে পারে, যা কয়েকটি উদাহরণে দেখব।
curly bracket-এর ভেতরে আমরা সেই method signature-গুলো ঘোষণা করি যা এই trait implement করা type-গুলোর আচরণ বর্ণনা করে,
এই ক্ষেত্রে `fn summarize(&self) -> String`।

method signature-এর পরে curly bracket-এর ভেতরে implementation না দিয়ে আমরা একটি semicolon দিই। এই trait implement করা
প্রতিটি type-কে method-এর body-র জন্য নিজস্ব আচরণ দিতে হবে। compiler নিশ্চিত করবে যে `Summary` trait আছে এমন যেকোনো
type-এ ঠিক এই signature-সহ `summarize` method সংজ্ঞায়িত থাকবে।

একটি trait-এর body-তে একাধিক method থাকতে পারে: method signature-গুলো প্রতি লাইনে একটি করে লেখা হয়, আর প্রতিটি লাইন
শেষ হয় semicolon দিয়ে।

### একটি type-এর ওপর trait implement করা {#implementing-a-trait-on-a-type}

`Summary` trait-এর method-গুলোর কাঙ্ক্ষিত signature সংজ্ঞায়িত করার পর এখন আমরা আমাদের media aggregator-এর type-গুলোর ওপর
এটি implement করতে পারি। Listing 10-13-এ `NewsArticle` struct-এর ওপর `Summary` trait-এর একটি implementation দেখানো
হয়েছে, যা শিরোনাম, লেখক আর জায়গা ব্যবহার করে `summarize`-এর return মান তৈরি করে। `SocialPost` struct-এর জন্য আমরা
`summarize`-কে সংজ্ঞায়িত করি username আর তার পরে post-এর পুরো লেখা হিসেবে, ধরে নিয়ে যে post-এর বিষয়বস্তু আগে থেকেই
280 অক্ষরে সীমিত।

<Listing number="10-13" file-name="src/lib.rs" caption="`NewsArticle` ও `SocialPost` type-এর ওপর `Summary` trait implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

</Listing>

একটি type-এর ওপর trait implement করা সাধারণ method implement করার মতোই। পার্থক্য হলো `impl`-এর পরে আমরা যে trait
implement করতে চাই তার নাম লিখি, তারপর `for` keyword, তারপর যে type-এর জন্য trait-টি implement করতে চাই তার নাম।
`impl` block-এর ভেতরে trait-এর সংজ্ঞায় থাকা method signature-গুলো লিখি। প্রতিটি signature-এর পরে semicolon না দিয়ে
curly bracket দিই, আর method-এর body-তে সেই নির্দিষ্ট আচরণ লিখি যা আমরা চাই trait-এর method-গুলো ওই নির্দিষ্ট type-এর
জন্য করুক।

library-টি `NewsArticle` ও `SocialPost`-এর ওপর `Summary` trait implement করার পর crate-এর ব্যবহারকারীরা `NewsArticle` ও
`SocialPost`-এর instance-এর ওপর trait-এর method call করতে পারে, ঠিক যেভাবে আমরা সাধারণ method call করি। একমাত্র
পার্থক্য হলো ব্যবহারকারীকে type-গুলোর পাশাপাশি trait-টিকেও scope-এ আনতে হবে। একটি binary crate কীভাবে আমাদের
`aggregator` library crate ব্যবহার করতে পারে, তার একটি উদাহরণ:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

এই কোড print করে `1 new post: horse_ebooks: of course, as you probably already
know, people`।

`aggregator` crate-এর ওপর নির্ভরশীল অন্য crate-গুলোও নিজেদের type-এর ওপর `Summary` implement করার জন্য `Summary`
trait-কে scope-এ আনতে পারে। একটি সীমাবদ্ধতা মনে রাখার মতো: একটি type-এর ওপর trait implement করা যায় কেবল তখনই, যখন
trait অথবা type, কিংবা দুটোই, আমাদের crate-এর নিজস্ব (local)। যেমন, `aggregator` crate-এর কার্যক্ষমতার অংশ হিসেবে
`SocialPost`-এর মতো নিজস্ব type-এর ওপর `Display`-এর মতো standard library-র trait implement করতে পারি, কারণ `SocialPost`
type-টি আমাদের `aggregator` crate-এর নিজস্ব। আবার আমাদের `aggregator` crate-এ `Vec<T>`-এর ওপর `Summary`-ও implement করতে
পারি, কারণ `Summary` trait-টি আমাদের `aggregator` crate-এর নিজস্ব।

কিন্তু বাইরের type-এর ওপর বাইরের trait implement করা যায় না। যেমন, আমাদের `aggregator` crate-এর ভেতরে `Vec<T>`-এর ওপর
`Display` trait implement করা যায় না, কারণ `Display` আর `Vec<T>` দুটিই standard library-তে সংজ্ঞায়িত, আমাদের
`aggregator` crate-এর নিজস্ব নয়। এই সীমাবদ্ধতা _coherence_ (সামঞ্জস্য) নামের একটি বৈশিষ্ট্যের অংশ, আরও নির্দিষ্ট করে
বললে _orphan rule_ (অনাথ নিয়ম)-এর অংশ; নামটি এসেছে কারণ এখানে parent type উপস্থিত থাকে না। এই নিয়ম নিশ্চিত করে যে
অন্যের কোড আপনার কোড ভাঙতে পারবে না, আর আপনার কোডও অন্যেরটা ভাঙতে পারবে না। এই নিয়ম না থাকলে দুটি crate একই type-এর
জন্য একই trait implement করতে পারত, আর Rust জানত না কোন implementation ব্যবহার করবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="default-implementations"></a>

### default implementation ব্যবহার করা {#using-default-implementations}

কখনো কখনো প্রতিটি type-এ সব method-এর implementation বাধ্যতামূলক না করে trait-এর কিছু বা সব method-এর জন্য একটি default
(পূর্বনির্ধারিত) আচরণ রাখা কাজে লাগে। তখন কোনো নির্দিষ্ট type-এর ওপর trait implement করার সময় প্রতিটি method-এর default
আচরণ রেখে দিতে বা override (নিজের সংস্করণ দিয়ে প্রতিস্থাপন) করতে পারি।

Listing 10-14-এ আমরা Listing 10-12-এর মতো শুধু method signature সংজ্ঞায়িত না করে `Summary` trait-এর `summarize` method-এর
জন্য একটি default string দিয়েছি।

<Listing number="10-14" file-name="src/lib.rs" caption="`summarize` method-এর একটি default implementation-সহ `Summary` trait সংজ্ঞায়িত করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

</Listing>

`NewsArticle`-এর instance-এর সারসংক্ষেপ করতে default implementation ব্যবহার করতে চাইলে আমরা `impl Summary for NewsArticle {}`
দিয়ে একটি খালি `impl` block দিই।

যদিও এখন আর `NewsArticle`-এর ওপর সরাসরি `summarize` method সংজ্ঞায়িত করছি না, তবু আমরা একটি default implementation
দিয়েছি এবং বলে দিয়েছি যে `NewsArticle` `Summary` trait implement করে। ফলে `NewsArticle`-এর একটি instance-এর ওপর এখনো
`summarize` method call করতে পারি, এভাবে:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs:here}}
```

এই কোড print করে `New article available! (Read more...)`।

default implementation তৈরি করার জন্য Listing 10-13-এ `SocialPost`-এর ওপর `Summary`-এর implementation-এ কিছুই বদলাতে হয়
না। কারণ default implementation override করার syntax আর default implementation নেই এমন trait method implement করার syntax
একই।

default implementation একই trait-এর অন্য method call করতে পারে, এমনকি সেই method-গুলোর default implementation না থাকলেও।
এভাবে একটি trait অনেক দরকারি কার্যক্ষমতা দিতে পারে, আর implement-কারীদের শুধু এর ছোট একটি অংশ নির্দিষ্ট করে দিতে হয়।
যেমন, আমরা `Summary` trait-এ একটি `summarize_author` method রাখতে পারি যার implementation বাধ্যতামূলক, তারপর একটি
`summarize` method সংজ্ঞায়িত করতে পারি যার default implementation `summarize_author` method-টি call করে:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

`Summary`-এর এই সংস্করণ ব্যবহার করতে কোনো type-এর ওপর trait implement করার সময় আমাদের শুধু `summarize_author` সংজ্ঞায়িত
করতে হবে:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

`summarize_author` সংজ্ঞায়িত করার পর `SocialPost` struct-এর instance-এর ওপর `summarize` call করা যায়, আর `summarize`-এর
default implementation আমাদের দেওয়া `summarize_author`-এর সংজ্ঞাটি call করবে। আমরা `summarize_author` implement করেছি বলে
`Summary` trait আর কোনো কোড না লিখিয়েই আমাদের `summarize` method-এর আচরণ দিয়ে দিয়েছে। দেখতে এমন:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/main.rs:here}}
```

এই কোড print করে `1 new post: (Read more from @horse_ebooks...)`।

খেয়াল করুন, একই method-এর override করা implementation থেকে default implementation call করা সম্ভব নয়।

{{#quiz ../quizzes/ch10-02-traits-sec1.toml}}

<!-- Old headings. Do not remove or links may break. -->

<a id="traits-as-parameters"></a>

### trait-কে parameter হিসেবে ব্যবহার করা {#using-traits-as-parameters}

trait কীভাবে সংজ্ঞায়িত ও implement করতে হয় তা যেহেতু জানেন, এবার দেখা যাক trait ব্যবহার করে কীভাবে এমন function
সংজ্ঞায়িত করা যায় যা অনেক রকম type গ্রহণ করে। Listing 10-13-এ `NewsArticle` ও `SocialPost` type-এর ওপর implement করা
`Summary` trait ব্যবহার করে আমরা একটি `notify` function সংজ্ঞায়িত করব, যা তার `item` parameter-এর ওপর `summarize` method
call করে; এই parameter-টি এমন কোনো type-এর, যা `Summary` trait implement করে। এজন্য আমরা `impl Trait` syntax ব্যবহার করি, এভাবে:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

`item` parameter-এর জন্য কোনো concrete type-এর বদলে আমরা `impl` keyword আর trait-এর নাম লিখি। এই parameter উল্লিখিত trait
implement করে এমন যেকোনো type গ্রহণ করে। `notify`-এর body-তে `item`-এর ওপর `Summary` trait থেকে আসা যেকোনো method, যেমন
`summarize`, call করতে পারি। `notify` call করে `NewsArticle` বা `SocialPost`-এর যেকোনো instance পাঠাতে পারি। অন্য কোনো
type, যেমন `String` বা `i32`, দিয়ে function-টি call করা কোড compile হবে না, কারণ ওই type-গুলো `Summary` implement করে না।

<!-- Old headings. Do not remove or links may break. -->

<a id="fixing-the-largest-function-with-trait-bounds"></a>

#### trait bound syntax {#trait-bound-syntax}

`impl Trait` syntax সহজ ক্ষেত্রে কাজ করে, তবে এটি আসলে _trait bound_ নামে পরিচিত একটি দীর্ঘ রূপের syntax sugar (লেখা
সহজ করার সংক্ষিপ্ত রূপ); দীর্ঘ রূপটি দেখতে এমন:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

এই দীর্ঘ রূপটি আগের অংশের উদাহরণের সমতুল্য, তবে বেশি শব্দবহুল। আমরা trait bound বসাই generic type parameter-এর ঘোষণার
সঙ্গে, একটি colon-এর পরে এবং angle bracket-এর ভেতরে।

`impl Trait` syntax সুবিধাজনক এবং সহজ ক্ষেত্রে কোড আরও সংক্ষিপ্ত করে, অন্যদিকে পূর্ণ trait bound syntax অন্য ক্ষেত্রে আরও
জটিলতা প্রকাশ করতে পারে। যেমন, আমাদের এমন দুটি parameter থাকতে পারে যারা `Summary` implement করে। `impl Trait` syntax দিয়ে
তা লিখলে দেখতে এমন:

```rust,ignore
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```

আমরা যদি চাই এই function `item1` ও `item2`-কে ভিন্ন type-এর হতে দিক (যতক্ষণ দুটি type-ই `Summary` implement করে), তাহলে
`impl Trait` ব্যবহার করাই উপযুক্ত। কিন্তু যদি দুটি parameter-কে একই type-এর হতে বাধ্য করতে চাই, তাহলে trait bound ব্যবহার
করতে হবে, এভাবে:

```rust,ignore
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```

`item1` ও `item2` parameter-এর type হিসেবে দেওয়া generic type `T` function-টিকে এমনভাবে সীমাবদ্ধ করে যে `item1` ও
`item2`-এর argument হিসেবে পাঠানো মানের concrete type একই হতে হবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="specifying-multiple-trait-bounds-with-the--syntax"></a>

#### `+` syntax দিয়ে একাধিক trait bound {#multiple-trait-bounds-with-the--syntax}

একাধিক trait bound-ও উল্লেখ করা যায়। ধরুন আমরা চাই `notify` `item`-এর ওপর `summarize`-এর পাশাপাশি display formatting-ও
ব্যবহার করুক: তাহলে `notify`-এর সংজ্ঞায় বলে দিই যে `item`-কে `Display` আর `Summary` দুটিই implement করতে হবে। এটা করা যায়
`+` syntax দিয়ে:

```rust,ignore
pub fn notify(item: &(impl Summary + Display)) {
```

generic type-এর trait bound-এর সঙ্গেও `+` syntax বৈধ:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

দুটি trait bound উল্লেখ করায় `notify`-এর body `summarize` call করতে পারে এবং `item`-কে format করতে `{}` ব্যবহার করতে
পারে।

#### `where` clause দিয়ে আরও পরিষ্কার trait bound {#clearer-trait-bounds-with-where-clauses}

খুব বেশি trait bound ব্যবহারের অসুবিধাও আছে। প্রতিটি generic-এর নিজস্ব trait bound থাকে, তাই একাধিক generic type
parameter-যুক্ত function-এর নাম আর parameter তালিকার মাঝখানে অনেক trait bound-এর তথ্য জমে যেতে পারে, ফলে function
signature পড়া কঠিন হয়ে যায়। এ কারণে Rust-এ function signature-এর পরে একটি `where` clause-এর ভেতরে trait bound উল্লেখ
করার বিকল্প syntax আছে। তাই এভাবে লেখার বদলে:

```rust,ignore
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

আমরা একটি `where` clause ব্যবহার করতে পারি, এভাবে:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

এই function-এর signature কম জট পাকানো: function-এর নাম, parameter তালিকা আর return type কাছাকাছি থাকে, অনেক trait bound
ছাড়া একটি function-এর মতো।

### trait implement করে এমন type return করা {#returning-types-that-implement-traits}

return-এর জায়গাতেও `impl Trait` syntax ব্যবহার করে এমন কোনো type-এর মান return করা যায় যা একটি trait implement করে, যেমন
এখানে দেখানো হয়েছে:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

return type হিসেবে `impl Summary` ব্যবহার করে আমরা বলে দিই যে `returns_summarizable` function এমন কোনো type return করে যা
`Summary` trait implement করে, concrete type-টির নাম না বলেই। এই ক্ষেত্রে `returns_summarizable` একটি `SocialPost` return
করে, কিন্তু এই function call করা কোডের তা জানার দরকার নেই।

শুধু implement করা trait দিয়ে return type উল্লেখ করার সুবিধা বিশেষভাবে কাজে লাগে closure ও iterator-এর ক্ষেত্রে, যা
আমরা অধ্যায় 13-এ আলোচনা করব। closure ও iterator এমন type তৈরি করে যা শুধু compiler জানে, অথবা যার নাম লেখা খুবই দীর্ঘ।
`impl Trait` syntax দিয়ে খুব দীর্ঘ type লিখতে না হয়ে সংক্ষেপে বলে দেওয়া যায় যে একটি function `Iterator` trait implement
করে এমন কোনো type return করে।

তবে `impl Trait` ব্যবহার করা যায় কেবল তখনই, যখন আপনি একটিমাত্র type return করছেন। যেমন, return type `impl Summary` দিয়ে
`NewsArticle` অথবা `SocialPost` return করা এই কোড কাজ করবে না:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-06-impl-trait-returns-one-type/src/lib.rs:here}}
```

compiler-এ `impl Trait` syntax যেভাবে implement করা হয়েছে তার সীমাবদ্ধতার কারণে `NewsArticle` অথবা `SocialPost` return
করা অনুমোদিত নয়। এমন আচরণের function কীভাবে লিখতে হয়, তা আমরা অধ্যায় 18-এর [“Using Trait
Objects to Abstract over Shared Behavior”][trait-objects]<!-- ignore --> অংশে দেখব।

### শর্তসাপেক্ষে method implement করতে trait bound ব্যবহার {#using-trait-bounds-to-conditionally-implement-methods}

generic type parameter ব্যবহার করা একটি `impl` block-এ trait bound ব্যবহার করে আমরা উল্লিখিত trait implement করে এমন
type-এর জন্য শর্তসাপেক্ষে method implement করতে পারি। যেমন, Listing 10-15-এর `Pair<T>` type সবসময় `new` function implement
করে, যা `Pair<T>`-এর একটি নতুন instance return করে (অধ্যায় 5-এর [“method
syntax”][methods]<!-- ignore --> অংশ থেকে মনে করুন, `Self` হলো `impl` block-এর type-এর একটি type alias (বিকল্প নাম), যা এই
ক্ষেত্রে `Pair<T>`)। কিন্তু পরের `impl` block-এ `Pair<T>` `cmp_display` method implement করে কেবল তখনই, যখন এর ভেতরের type
`T` তুলনা সম্ভব করে এমন `PartialOrd` trait _এবং_ print সম্ভব করে এমন `Display` trait implement করে।

<Listing number="10-15" file-name="src/lib.rs" caption="trait bound-এর ওপর নির্ভর করে একটি generic type-এর ওপর শর্তসাপেক্ষে method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

</Listing>

অন্য একটি trait implement করে এমন যেকোনো type-এর জন্যও শর্তসাপেক্ষে একটি trait implement করা যায়। trait bound পূরণ করে
এমন যেকোনো type-এর ওপর একটি trait-এর implementation-কে বলা হয় _blanket implementations_ (ঢালাও implementation), আর Rust
standard library-তে এগুলো ব্যাপকভাবে ব্যবহৃত হয়। যেমন, standard library `Display` trait implement করে এমন যেকোনো type-এর
ওপর `ToString` trait implement করে। standard library-র `impl` block-টি দেখতে মোটামুটি এই কোডের মতো:

```rust,ignore
impl<T: Display> ToString for T {
    // --snip--
}
```

standard library-তে এই blanket implementation থাকায় `Display` trait implement করে এমন যেকোনো type-এর ওপর `ToString` trait-এর
সংজ্ঞায়িত `to_string` method call করতে পারি। যেমন, integer `Display` implement করে বলে এভাবে integer-কে তার সংশ্লিষ্ট
`String` মানে রূপান্তর করা যায়:

```rust
let s = 3.to_string();
```

blanket implementation-গুলো trait-এর documentation-এ “Implementors” অংশে দেখা যায়।

trait ও trait bound দিয়ে আমরা এমন কোড লিখতে পারি যা generic type parameter ব্যবহার করে পুনরাবৃত্তি কমায়, আবার compiler-কে
জানিয়েও দেয় যে আমরা চাই generic type-টির নির্দিষ্ট আচরণ থাকুক। compiler তখন trait bound-এর তথ্য ব্যবহার করে যাচাই করতে
পারে যে আমাদের কোডের সঙ্গে ব্যবহৃত সব concrete type সঠিক আচরণ দেয়। dynamically typed (চলার সময় type যাচাই হয় এমন)
ভাষায় কোনো type-এর ওপর এমন method call করলে, যা সেই type-এ সংজ্ঞায়িত নেই, আমরা runtime-এ error পেতাম। কিন্তু Rust এই
error-গুলোকে compile time-এ নিয়ে আসে, ফলে আমাদের কোড চলতে পারার আগেই সমস্যাগুলো ঠিক করতে বাধ্য হই। তা ছাড়া runtime-এ
আচরণ যাচাই করার কোড লিখতে হয় না, কারণ compile time-এই যাচাই হয়ে গেছে। এতে generics-এর নমনীয়তা না ছেড়েই performance
বাড়ে।

{{#quiz ../quizzes/ch10-02-traits-sec2.toml}}

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[methods]: ch05-03-method-syntax.html#method-syntax

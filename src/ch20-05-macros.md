## macro {#macros}

এই বই জুড়ে আমরা `println!`-এর মতো macro ব্যবহার করেছি, কিন্তু macro আসলে কী এবং কীভাবে কাজ করে তা পুরোপুরি দেখিনি। _macro_ পরিভাষাটি Rust-এর একগুচ্ছ বৈশিষ্ট্য বোঝায়—`macro_rules!` দিয়ে declarative
macro আর তিন ধরনের procedural macro:

- নিজস্ব `#[derive]` macro, যা struct আর enum-এ ব্যবহৃত `derive` attribute দিয়ে যোগ হওয়া কোড নির্দিষ্ট করে
- attribute-সদৃশ macro, যা যেকোনো item-এ ব্যবহারযোগ্য নিজস্ব attribute সংজ্ঞায়িত করে
- function-সদৃশ macro, যা দেখতে function call-এর মতো, কিন্তু argument হিসেবে দেওয়া token-গুলোর ওপর কাজ করে

আমরা এগুলোর প্রতিটি নিয়ে একে একে কথা বলব, তবে প্রথমে দেখা যাক function থাকা সত্ত্বেও macro-র দরকার কেন।

### macro আর function-এর পার্থক্য {#the-difference-between-macros-and-functions}

মূলত macro হলো এমন কোড লেখার উপায়, যা অন্য কোড লেখে; একে বলা হয় _metaprogramming_ (কোড নিয়ে কোড লেখা)। Appendix C-তে আমরা `derive` attribute নিয়ে আলোচনা করেছি, যা আপনার জন্য বিভিন্ন trait-এর
implementation তৈরি করে। বই জুড়ে আমরা `println!` আর `vec!` macro-ও ব্যবহার করেছি। এই সব macro _expand_ হয়ে (বিস্তৃত হয়ে) আপনার হাতে লেখা কোডের চেয়ে বেশি কোড তৈরি করে।

আপনাকে যতটা কোড লিখতে ও রক্ষণাবেক্ষণ করতে হয় তা কমাতে metaprogramming কাজে লাগে, যা function-এরও একটি ভূমিকা। তবে macro-র কিছু বাড়তি ক্ষমতা আছে, যা function-এর নেই।

একটি function signature-কে ঘোষণা করতে হয় function-এর কতগুলো এবং কোন type-এর parameter আছে। অন্যদিকে macro পরিবর্তনশীল সংখ্যক parameter নিতে পারে: আমরা একটি argument দিয়ে `println!("hello")`, বা দুটি
argument দিয়ে `println!("hello {}", name)` call করতে পারি। তা ছাড়া compiler কোডের অর্থ ব্যাখ্যা করার আগেই macro expand হয়, তাই একটি macro, যেমন, একটি নির্দিষ্ট type-এর ওপর একটি trait implement করতে পারে।
একটি function তা পারে না, কারণ এটি runtime-এ call হয়, আর trait implement করতে হয় compile time-এ।

function-এর বদলে macro implement করার অসুবিধা হলো, macro-র সংজ্ঞা function-এর সংজ্ঞার চেয়ে জটিল, কারণ আপনি এমন Rust কোড লিখছেন যা Rust কোড লেখে। এই পরোক্ষতার কারণে macro-র সংজ্ঞা সাধারণত function-এর
সংজ্ঞার চেয়ে পড়া, বোঝা আর রক্ষণাবেক্ষণ করা কঠিন।

macro আর function-এর আরেকটি গুরুত্বপূর্ণ পার্থক্য হলো, একটি ফাইলে macro call করার _আগে_ এটি সংজ্ঞায়িত করতে বা scope-এ আনতে হয়, অন্যদিকে function যেকোনো জায়গায় সংজ্ঞায়িত করে যেকোনো জায়গায় call করা যায়।

<!-- Old headings. Do not remove or links may break. -->

<a id="declarative-macros-with-macro_rules-for-general-metaprogramming"></a>

### সাধারণ metaprogramming-এর জন্য declarative macro {#declarative-macros-for-general-metaprogramming}

Rust-এ macro-র সবচেয়ে বেশি ব্যবহৃত রূপ হলো _declarative macro_। এগুলোকে কখনো কখনো “macros by example,” “`macro_rules!` macros,” বা শুধু “macros” বলা হয়। মূলত declarative macro আপনাকে Rust-এর `match`
expression-এর মতো কিছু লিখতে দেয়। অধ্যায় 6-এ যেমন আলোচনা করা হয়েছে, `match` expression হলো এমন control গঠন, যা একটি expression নিয়ে expression-এর ফলাফলের মানকে pattern-এর সঙ্গে তুলনা করে, তারপর মিলে
যাওয়া pattern-এর সঙ্গে যুক্ত কোড চালায়। macro-ও একটি মানকে নির্দিষ্ট কোডের সঙ্গে যুক্ত pattern-এর সঙ্গে তুলনা করে: এই ক্ষেত্রে মানটি হলো macro-তে পাঠানো Rust-এর হুবহু source code; pattern-গুলো সেই source
code-এর গঠনের সঙ্গে তুলনা করা হয়; আর মিলে গেলে প্রতিটি pattern-এর সঙ্গে যুক্ত কোড macro-তে পাঠানো কোডের জায়গা নেয়। এই সবকিছু ঘটে compilation-এর সময়।

macro সংজ্ঞায়িত করতে `macro_rules!` গঠন ব্যবহার করেন। চলুন `vec!` macro কীভাবে সংজ্ঞায়িত তা দেখে `macro_rules!` কীভাবে ব্যবহার করতে হয় তা দেখি। অধ্যায় 8-এ আমরা দেখেছি নির্দিষ্ট মানসহ একটি নতুন
vector তৈরি করতে কীভাবে `vec!` macro ব্যবহার করা যায়। যেমন, নিচের macro তিনটি integer-যুক্ত একটি নতুন vector তৈরি করে:

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

`vec!` macro দিয়ে দুটি integer-এর vector বা পাঁচটি string slice-এর vector-ও বানাতে পারতাম। একই কাজ function দিয়ে করতে পারতাম না, কারণ মানের সংখ্যা বা type আগে থেকে জানতাম না।

Listing 20-35-এ `vec!` macro-র একটু সরলীকৃত একটি সংজ্ঞা দেখানো হয়েছে।

<Listing number="20-35" file-name="src/lib.rs" caption="`vec!` macro-র সংজ্ঞার একটি সরলীকৃত রূপ">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-35/src/lib.rs}}
```

</Listing>

> Note: standard library-তে `vec!` macro-র আসল সংজ্ঞায় শুরুতেই সঠিক পরিমাণ মেমোরি আগাম allocate করার কোড আছে। সেই কোড একটি optimization, যা উদাহরণ সরল রাখতে আমরা এখানে রাখিনি।

`#[macro_export]` annotation বোঝায় যে macro যে crate-এ সংজ্ঞায়িত সেই crate scope-এ আনা হলেই এই macro-টি পাওয়া যাবে। এই annotation ছাড়া macro-টি scope-এ আনা যায় না।

তারপর আমরা macro-র সংজ্ঞা শুরু করি `macro_rules!` আর যে macro সংজ্ঞায়িত করছি তার নাম দিয়ে, বিস্ময়চিহ্ন _ছাড়া_। নামের পরে, এই ক্ষেত্রে `vec`, macro-র সংজ্ঞার body বোঝাতে curly bracket আসে।

`vec!`-এর body-র গঠন একটি `match` expression-এর গঠনের মতো। এখানে আমাদের একটি arm আছে, যার pattern `( $( $x:expr ),* )`, তারপর `=>` আর এই pattern-এর সঙ্গে যুক্ত কোডের block। pattern মিললে সংশ্লিষ্ট
কোডের block তৈরি হবে। এই macro-তে এটিই যেহেতু একমাত্র pattern, তাই মেলার একটিই বৈধ উপায় আছে; অন্য যেকোনো pattern-এ error হবে। আরও জটিল macro-তে একাধিক arm থাকবে।

macro-র সংজ্ঞায় বৈধ pattern syntax অধ্যায় 19-এ আলোচিত pattern syntax থেকে আলাদা, কারণ macro pattern মানের সঙ্গে নয়, Rust কোডের গঠনের সঙ্গে মেলানো হয়। চলুন Listing 20-29-এর pattern-এর অংশগুলোর মানে কী
তা একে একে দেখি; macro pattern-এর পূর্ণ syntax-এর জন্য [Rust
Reference][ref] দেখুন।

প্রথমে পুরো pattern ঘিরে রাখতে আমরা এক জোড়া বন্ধনী ব্যবহার করি। macro system-এ এমন একটি variable ঘোষণা করতে ডলার চিহ্ন (`$`) ব্যবহার করি, যাতে pattern-এর সঙ্গে মেলা Rust কোড থাকবে। ডলার চিহ্ন স্পষ্ট
করে দেয় যে এটি একটি সাধারণ Rust variable নয়, বরং একটি macro variable। এরপর আসে এক জোড়া বন্ধনী, যা বন্ধনীর ভেতরের pattern-এর সঙ্গে মেলা মানগুলো প্রতিস্থাপনের কোডে ব্যবহারের জন্য capture করে। `$()`-এর
ভেতরে আছে `$x:expr`, যা যেকোনো Rust expression-এর সঙ্গে মেলে এবং expression-টিকে `$x` নাম দেয়।

`$()`-এর পরের কমা বোঝায় যে `$()`-এর কোডের সঙ্গে মেলা কোডের প্রতিটি instance-এর মাঝে একটি আক্ষরিক কমা বিভাজক থাকতে হবে। `*` বলে দেয় যে pattern-টি `*`-এর আগে যা আছে তার শূন্য বা তার বেশি সংখ্যকের সঙ্গে মেলে।

`vec![1, 2, 3];` দিয়ে এই macro call করলে `$x` pattern তিনটি expression `1`, `2` আর `3`-এর সঙ্গে তিনবার মেলে।

এবার এই arm-এর সঙ্গে যুক্ত কোডের body-র pattern দেখা যাক: pattern কতবার মেলে তার ওপর নির্ভর করে pattern-এ `$()`-এর সঙ্গে মেলা প্রতিটি অংশের জন্য `$()*`-এর ভেতরের `temp_vec.push()` শূন্য বা তার বেশিবার
তৈরি হয়। `$x`-এর জায়গায় মিলে যাওয়া প্রতিটি expression বসে। `vec![1, 2, 3];` দিয়ে এই macro call করলে এই macro call-এর জায়গায় তৈরি হওয়া কোড হবে এই:

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

আমরা এমন একটি macro সংজ্ঞায়িত করেছি, যা যেকোনো type-এর যেকোনো সংখ্যক argument নিতে পারে এবং উল্লিখিত উপাদানগুলো রাখা একটি vector তৈরির কোড তৈরি করতে পারে।

macro কীভাবে লিখতে হয় সে সম্পর্কে আরও জানতে online documentation বা অন্য উৎস দেখুন, যেমন Daniel Keep-এর শুরু করা এবং Lukas Wirth-এর এগিয়ে নেওয়া [“The Little Book of Rust Macros”][tlborm]।

### attribute থেকে কোড তৈরির জন্য procedural macro {#procedural-macros-for-generating-code-from-attributes}

macro-র দ্বিতীয় রূপ হলো procedural macro, যা অনেকটা function-এর মতো আচরণ করে (এবং এক ধরনের procedure)। declarative macro যেমন pattern-এর সঙ্গে মিলিয়ে কোডকে অন্য কোড দিয়ে প্রতিস্থাপন করে, তেমন না করে
_Procedural macros_ (procedural macro) input হিসেবে কিছু কোড নেয়, সেই কোডের ওপর কাজ করে, এবং output হিসেবে কিছু কোড তৈরি করে। তিন ধরনের procedural macro হলো নিজস্ব `derive`, attribute-সদৃশ আর
function-সদৃশ, এবং সবগুলোই একই রকমভাবে কাজ করে।

procedural macro তৈরির সময় সংজ্ঞাগুলোকে একটি বিশেষ crate type-এর নিজস্ব crate-এ থাকতে হয়। এটি জটিল কারিগরি কারণে, যা আমরা ভবিষ্যতে দূর করার আশা রাখি। Listing 20-36-এ আমরা দেখাই কীভাবে একটি
procedural macro সংজ্ঞায়িত করতে হয়, যেখানে `some_attribute` হলো নির্দিষ্ট কোনো ধরনের macro ব্যবহারের একটি placeholder।

<Listing number="20-36" file-name="src/lib.rs" caption="একটি procedural macro সংজ্ঞায়িত করার উদাহরণ">

```rust,ignore
use proc_macro::TokenStream;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

</Listing>

একটি procedural macro সংজ্ঞায়িত করা function input হিসেবে একটি `TokenStream` নেয় এবং output হিসেবে একটি `TokenStream` তৈরি করে। `TokenStream` type সংজ্ঞায়িত করে Rust-এর সঙ্গে আসা `proc_macro` crate,
আর এটি token-এর একটি ধারা বোঝায়। এটিই macro-র মূল: macro যে source code-এর ওপর কাজ করছে তা input `TokenStream` গঠন করে, আর macro যে কোড তৈরি করে তা output `TokenStream`। function-টির সঙ্গে একটি
attribute-ও যুক্ত থাকে, যা বলে দেয় আমরা কোন ধরনের procedural macro তৈরি করছি। একই crate-এ একাধিক ধরনের procedural macro থাকতে পারে।

চলুন বিভিন্ন ধরনের procedural macro দেখি। আমরা শুরু করব একটি নিজস্ব `derive` macro দিয়ে, তারপর ব্যাখ্যা করব কোন ছোট ছোট অমিল অন্য রূপগুলোকে আলাদা করে।

<!-- Old headings. Do not remove or links may break. -->

<a id="how-to-write-a-custom-derive-macro"></a>

### নিজস্ব `derive` macro {#custom-derive-macros}

চলুন `hello_macro` নামে একটি crate তৈরি করি, যা `hello_macro` নামের একটি associated function-যুক্ত `HelloMacro` নামের একটি trait সংজ্ঞায়িত করে। ব্যবহারকারীদের দিয়ে তাদের প্রতিটি type-এর জন্য `HelloMacro`
trait implement করানোর বদলে আমরা একটি procedural macro দেব, যাতে ব্যবহারকারীরা তাদের type-কে `#[derive(HelloMacro)]` দিয়ে annotate করে `hello_macro` function-এর একটি default implementation পায়। default
implementation print করবে `Hello, Macro! My name is
TypeName!`, যেখানে `TypeName` হলো সেই type-এর নাম, যার ওপর এই trait সংজ্ঞায়িত হয়েছে। অন্যভাবে বললে, আমরা এমন একটি crate লিখব, যা ব্যবহার করে আরেকজন প্রোগ্রামার Listing 20-37-এর মতো কোড লিখতে পারবে।

<Listing number="20-37" file-name="src/main.rs" caption="আমাদের procedural macro ব্যবহার করে আমাদের crate-এর একজন ব্যবহারকারী যে কোড লিখতে পারবে">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-37/src/main.rs}}
```

</Listing>

কাজ শেষ হলে এই কোড print করবে `Hello, Macro! My name is Pancakes!`। প্রথম ধাপ হলো একটি নতুন library crate বানানো, এভাবে:

```console
$ cargo new hello_macro --lib
```

এরপর Listing 20-38-এ আমরা `HelloMacro` trait আর এর associated function সংজ্ঞায়িত করব।

<Listing file-name="src/lib.rs" number="20-38" caption="একটি সরল trait, যা আমরা `derive` macro-র সঙ্গে ব্যবহার করব">

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-38/hello_macro/src/lib.rs}}
```

</Listing>

আমাদের একটি trait আর এর function আছে। এই পর্যায়ে আমাদের crate-এর ব্যবহারকারী কাঙ্ক্ষিত কার্যক্ষমতা পেতে trait-টি implement করতে পারত, যেমনটি Listing 20-39-এ।

<Listing number="20-39" file-name="src/main.rs" caption="ব্যবহারকারীরা হাতে `HelloMacro` trait-এর implementation লিখলে যেমন দেখাত">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-39/pancakes/src/main.rs}}
```

</Listing>

কিন্তু `hello_macro`-এর সঙ্গে যে প্রতিটি type ব্যবহার করতে চাইত, তার জন্য তাদের implementation block লিখতে হতো; আমরা তাদের এই কাজ থেকে রেহাই দিতে চাই।

তা ছাড়া আমরা এখনো `hello_macro` function-কে এমন default implementation দিতে পারি না, যা trait যে type-এর ওপর implement করা সেই type-এর নাম print করবে: Rust-এ reflection (চলার সময় নিজের গঠন
পরীক্ষার ক্ষমতা) নেই, তাই runtime-এ type-এর নাম খুঁজে বের করতে পারে না। compile time-এ কোড তৈরি করতে আমাদের একটি macro দরকার।

পরের ধাপ হলো procedural macro সংজ্ঞায়িত করা। এই লেখার সময় procedural macro-কে নিজস্ব crate-এ থাকতে হয়। একসময় এই সীমাবদ্ধতা হয়তো তুলে নেওয়া হবে। crate আর macro crate সাজানোর রীতি হলো: `foo`
নামের একটি crate-এর জন্য নিজস্ব `derive` procedural macro crate-এর নাম হয় `foo_derive`। চলুন আমাদের `hello_macro` project-এর ভেতরে `hello_macro_derive` নামে একটি নতুন crate শুরু করি:

```console
$ cargo new hello_macro_derive --lib
```

আমাদের দুটি crate ঘনিষ্ঠভাবে সম্পর্কিত, তাই আমরা `hello_macro` crate-এর directory-র ভেতরে procedural macro crate তৈরি করি। `hello_macro`-এ trait-এর সংজ্ঞা বদলালে `hello_macro_derive`-এ procedural
macro-র implementation-ও বদলাতে হবে। দুটি crate আলাদাভাবে প্রকাশ করতে হবে, আর এই crate ব্যবহার করা প্রোগ্রামারদের দুটিকেই dependency হিসেবে যোগ করে দুটিকেই scope-এ আনতে হবে। এর বদলে `hello_macro`
crate-কে দিয়ে `hello_macro_derive`-কে dependency হিসেবে ব্যবহার করিয়ে procedural macro-র কোড re-export করাতে পারতাম। তবে আমরা project যেভাবে সাজিয়েছি, তাতে প্রোগ্রামাররা `derive` কার্যক্ষমতা না চাইলেও
`hello_macro` ব্যবহার করতে পারে।

`hello_macro_derive` crate-কে একটি procedural macro crate হিসেবে ঘোষণা করতে হবে। একটু পরেই দেখবেন, `syn` আর `quote` crate-এর কার্যক্ষমতাও লাগবে, তাই এগুলোকে dependency হিসেবে যোগ করতে হবে।
`hello_macro_derive`-এর _Cargo.toml_ ফাইলে নিচেরটি যোগ করুন:

<Listing file-name="hello_macro_derive/Cargo.toml">

```toml
{{#include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

</Listing>

procedural macro সংজ্ঞায়িত করা শুরু করতে `hello_macro_derive` crate-এর _src/lib.rs_ ফাইলে Listing 20-40-এর কোড রাখুন। খেয়াল করুন, `impl_hello_macro` function-এর সংজ্ঞা যোগ না করা পর্যন্ত এই কোড compile
হবে না।

<Listing number="20-40" file-name="hello_macro_derive/src/lib.rs" caption="Rust কোড প্রক্রিয়া করতে বেশিরভাগ procedural macro crate-এর যে কোড লাগবে">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-40/hello_macro/hello_macro_derive/src/lib.rs}}
```

</Listing>

খেয়াল করুন, আমরা কোডকে `hello_macro_derive` function, যা `TokenStream` parse করার দায়িত্বে, আর `impl_hello_macro` function, যা syntax tree রূপান্তরের দায়িত্বে, এই দুই ভাগে ভাগ করেছি: এতে procedural
macro লেখা আরও সুবিধাজনক হয়। বাইরের function-এর কোড (এই ক্ষেত্রে `hello_macro_derive`) আপনার দেখা বা তৈরি প্রায় প্রতিটি procedural macro crate-এ একই থাকবে। ভেতরের function-এর body-তে (এই ক্ষেত্রে
`impl_hello_macro`) আপনি যে কোড লিখবেন, তা আপনার procedural macro-র উদ্দেশ্য অনুযায়ী আলাদা হবে।

আমরা তিনটি নতুন crate এনেছি: `proc_macro`, [`syn`][syn]<!-- ignore -->,
আর [`quote`][quote]<!-- ignore -->। `proc_macro` crate Rust-এর সঙ্গে আসে, তাই _Cargo.toml_-এর dependency-তে এটি যোগ করতে হয়নি। `proc_macro` crate হলো compiler-এর API, যা আমাদের কোড থেকে Rust কোড পড়তে
ও পরিবর্তন করতে দেয়।

`syn` crate একটি string থেকে Rust কোড parse করে এমন একটি data structure-এ পরিণত করে, যার ওপর আমরা operation চালাতে পারি। `quote` crate `syn`-এর data structure-কে আবার Rust কোডে পরিণত করে। এই crate-গুলো
আমাদের সামলাতে চাওয়া যেকোনো ধরনের Rust কোড parse করা অনেক সহজ করে দেয়: Rust কোডের জন্য একটি পূর্ণ parser লেখা সহজ কাজ নয়।

আমাদের library-র একজন ব্যবহারকারী একটি type-এ `#[derive(HelloMacro)]` উল্লেখ করলে `hello_macro_derive` function call হবে। এটা সম্ভব, কারণ আমরা এখানে `hello_macro_derive` function-কে `proc_macro_derive`
দিয়ে annotate করে `HelloMacro` নামটি উল্লেখ করেছি, যা আমাদের trait-এর নামের সঙ্গে মেলে; বেশিরভাগ procedural macro এই রীতি মেনে চলে।

`hello_macro_derive` function প্রথমে `input`-কে একটি `TokenStream` থেকে এমন একটি data structure-এ রূপান্তর করে, যা তারপর ব্যাখ্যা করে operation চালাতে পারি। এখানেই কাজে আসে `syn`। `syn`-এর `parse`
function একটি `TokenStream` নিয়ে parse করা Rust কোড বোঝানো একটি `DeriveInput` struct return করে। Listing 20-41-এ `struct Pancakes;` string parse করে পাওয়া `DeriveInput` struct-এর প্রাসঙ্গিক অংশগুলো
দেখানো হয়েছে।

<Listing number="20-41" caption="Listing 20-37-এর macro-র attribute-যুক্ত কোড parse করলে যে `DeriveInput` instance পাই">

```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

</Listing>

এই struct-এর field-গুলো দেখায় যে আমাদের parse করা Rust কোড একটি unit struct, যার `ident` (_identifier_, অর্থাৎ নাম) হলো `Pancakes`। সব রকম Rust কোড বর্ণনার জন্য এই struct-এ আরও field আছে; আরও
তথ্যের জন্য [`DeriveInput`-এর `syn`
documentation][syn-docs] দেখুন।

শিগগিরই আমরা `impl_hello_macro` function সংজ্ঞায়িত করব, যেখানে আমাদের যোগ করতে চাওয়া নতুন Rust কোড তৈরি করব। তবে তার আগে খেয়াল করুন, আমাদের `derive` macro-র output-ও একটি `TokenStream`। return
হওয়া `TokenStream` আমাদের crate-এর ব্যবহারকারীদের লেখা কোডে যুক্ত হয়, তাই তারা তাদের crate compile করলে পরিবর্তিত `TokenStream`-এ আমাদের দেওয়া বাড়তি কার্যক্ষমতা পাবে।

হয়তো খেয়াল করেছেন, এখানে `syn::parse` function-এর call ব্যর্থ হলে `hello_macro_derive` function-কে panic করাতে আমরা `unwrap` call করছি। error-এ আমাদের procedural macro-র panic করা দরকার, কারণ procedural
macro API মেনে চলতে `proc_macro_derive` function-কে `Result`-এর বদলে `TokenStream` return করতে হয়। `unwrap` ব্যবহার করে আমরা এই উদাহরণ সরল রেখেছি; production কোডে `panic!` বা `expect` ব্যবহার করে কী ভুল
হয়েছে সে সম্পর্কে আরও নির্দিষ্ট error message দেওয়া উচিত।

annotate করা Rust কোডকে একটি `TokenStream` থেকে একটি `DeriveInput` instance-এ রূপান্তরের কোড আমাদের আছে, এবার annotate করা type-এর ওপর `HelloMacro` trait implement করার কোড তৈরি করি, যেমনটি Listing
20-42-এ দেখানো হয়েছে।

<Listing number="20-42" file-name="hello_macro_derive/src/lib.rs" caption="parse করা Rust কোড ব্যবহার করে `HelloMacro` trait implement করা">

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-42/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

</Listing>

`ast.ident` ব্যবহার করে আমরা annotate করা type-এর নাম (identifier) রাখা একটি `Ident` struct instance পাই। Listing 20-41-এর struct দেখায় যে Listing 20-37-এর কোডের ওপর `impl_hello_macro` function চালালে
আমরা যে `ident` পাব, তার `ident` field-এর মান হবে `"Pancakes"`। তাই Listing 20-42-এর `name` variable-এ একটি `Ident` struct instance থাকবে, যা print করলে হবে string `"Pancakes"`, অর্থাৎ Listing
20-37-এর struct-এর নাম।

`quote!` macro দিয়ে আমরা যে Rust কোড return করতে চাই তা সংজ্ঞায়িত করা যায়। `quote!` macro চলার সরাসরি ফল থেকে compiler ভিন্ন কিছু আশা করে, তাই একে একটি `TokenStream`-এ রূপান্তর করতে হয়। এটা করি `into`
method call করে, যা এই মধ্যবর্তী রূপটি খরচ করে দরকারি `TokenStream` type-এর একটি মান return করে।

`quote!` macro কিছু দারুণ templating ব্যবস্থাও দেয়: আমরা `#name` লিখতে পারি, আর `quote!` একে `name` variable-এর মান দিয়ে প্রতিস্থাপন করবে। সাধারণ macro যেভাবে কাজ করে, তেমনভাবে কিছু পুনরাবৃত্তিও করা যায়।
পূর্ণাঙ্গ পরিচিতির জন্য [`quote` crate-এর docs][quote-docs] দেখুন।

আমরা চাই আমাদের procedural macro ব্যবহারকারীর annotate করা type-এর জন্য আমাদের `HelloMacro` trait-এর একটি implementation তৈরি করুক, যা `#name` ব্যবহার করে পেতে পারি। trait-এর implementation-এ একটি
function আছে, `hello_macro`, যার body-তে আমাদের দিতে চাওয়া কার্যক্ষমতা আছে: `Hello, Macro! My name is` আর তারপর annotate করা type-এর নাম print করা।

এখানে ব্যবহৃত `stringify!` macro Rust-এর ভেতরেই আছে। এটি `1 + 2`-এর মতো একটি Rust expression নিয়ে compile time-এ expression-টিকে `"1 + 2"`-এর মতো একটি string literal-এ পরিণত করে। এটি `format!` বা
`println!` থেকে আলাদা, যেগুলো expression-টি মূল্যায়ন করে তারপর ফলকে একটি `String`-এ পরিণত করে। `#name` input হয়তো হুবহু print করার মতো একটি expression হতে পারে, তাই আমরা `stringify!` ব্যবহার করি।
`stringify!` ব্যবহার করলে compile time-এ `#name`-কে একটি string literal-এ রূপান্তর করায় একটি allocation-ও বাঁচে।

এই পর্যায়ে `hello_macro` আর `hello_macro_derive` দুটিতেই `cargo build` সফলভাবে শেষ হওয়ার কথা। procedural macro কাজ করতে দেখতে চলুন এই crate-গুলোকে Listing
20-37-এর কোডের সঙ্গে জুড়ে দিই! `cargo new pancakes` দিয়ে আপনার _projects_ directory-তে একটি নতুন binary project তৈরি করুন। `pancakes` crate-এর _Cargo.toml_-এ `hello_macro` আর `hello_macro_derive`-কে
dependency হিসেবে যোগ করতে হবে। আপনি যদি `hello_macro` আর `hello_macro_derive`-এর আপনার সংস্করণ [crates.io](https://crates.io/)<!-- ignore -->-তে প্রকাশ করেন, তাহলে এগুলো সাধারণ dependency হবে;
না করলে এগুলোকে এভাবে `path` dependency হিসেবে উল্লেখ করতে পারেন:

```toml
{{#include ../listings/ch20-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:6:8}}
```

Listing 20-37-এর কোড _src/main.rs_-এ রেখে `cargo run` চালান: এটি print করার কথা `Hello, Macro! My name is Pancakes!`। procedural macro থেকে আসা `HelloMacro` trait-এর implementation যুক্ত হয়েছে,
`pancakes` crate-কে তা implement করতে হয়নি; `#[derive(HelloMacro)]` trait-এর implementation যোগ করে দিয়েছে।

এরপর দেখা যাক অন্য ধরনের procedural macro নিজস্ব
`derive` macro থেকে কীভাবে আলাদা।

### attribute-সদৃশ macro {#attribute-like-macros}

attribute-সদৃশ macro নিজস্ব `derive` macro-র মতো, তবে `derive` attribute-এর জন্য কোড তৈরি না করে এরা নতুন attribute তৈরি করতে দেয়। এরা আরও নমনীয়ও: `derive` শুধু struct আর enum-এর জন্য কাজ করে; attribute
অন্য item-এও প্রয়োগ করা যায়, যেমন function-এ।
attribute-সদৃশ macro ব্যবহারের একটি উদাহরণ এই। ধরুন একটি web application framework ব্যবহারের সময় `route` নামের একটি attribute আছে, যা function annotate করে:

```rust,ignore
#[route(GET, "/")]
fn index() {
```

এই `#[route]` attribute framework-টি একটি procedural macro হিসেবে সংজ্ঞায়িত করত। macro-র সংজ্ঞা function-এর signature দেখতে এমন হতো:

```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

এখানে `TokenStream` type-এর দুটি parameter আছে। প্রথমটি attribute-এর বিষয়বস্তুর জন্য: `GET, "/"` অংশটি। দ্বিতীয়টি হলো attribute যে item-এর সঙ্গে যুক্ত তার body: এই ক্ষেত্রে `fn index() {}` আর
function-এর body-র বাকি অংশ।

এ ছাড়া attribute-সদৃশ macro নিজস্ব `derive` macro-র মতোই কাজ করে: আপনি `proc-macro` crate type-সহ একটি crate তৈরি করে এমন একটি function implement করেন, যা আপনার কাঙ্ক্ষিত কোড তৈরি করে!

### function-সদৃশ macro {#function-like-macros}

function-সদৃশ macro এমন macro সংজ্ঞায়িত করে, যা দেখতে function call-এর মতো। `macro_rules!` macro-র মতোই এরা function-এর চেয়ে বেশি নমনীয়; যেমন, এরা অজানা সংখ্যক argument নিতে পারে। তবে `macro_rules!` macro
শুধু আগের [“সাধারণ metaprogramming-এর জন্য declarative macro”][decl]<!-- ignore --> অংশে আলোচিত match-সদৃশ syntax দিয়েই সংজ্ঞায়িত করা যায়। function-সদৃশ macro একটি `TokenStream` parameter নেয়, আর অন্য
দুই ধরনের procedural macro-র মতোই এদের সংজ্ঞা Rust কোড দিয়ে সেই `TokenStream` পরিবর্তন করে। function-সদৃশ macro-র একটি উদাহরণ হলো একটি `sql!` macro, যা এভাবে call করা হতে পারে:

```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

এই macro এর ভেতরের SQL statement parse করে যাচাই করত যে এটি syntax-এর দিক থেকে সঠিক, যা একটি `macro_rules!` macro যা করতে পারে তার চেয়ে অনেক জটিল প্রক্রিয়া। `sql!` macro এভাবে সংজ্ঞায়িত হতো:

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

এই সংজ্ঞা নিজস্ব `derive` macro-র signature-এর মতো: আমরা বন্ধনীর ভেতরের token-গুলো পাই এবং যে কোড তৈরি করতে চেয়েছিলাম তা return করি।

{{#quiz ../quizzes/ch19-06-macros.toml}}

## সারসংক্ষেপ {#summary}

উফ! এখন আপনার হাতিয়ারের বাক্সে Rust-এর এমন কিছু বৈশিষ্ট্য আছে, যা সম্ভবত প্রায়ই ব্যবহার করবেন না, তবে আপনি জানবেন খুব নির্দিষ্ট পরিস্থিতিতে এগুলো পাওয়া যায়। আমরা কয়েকটি জটিল বিষয়ের সঙ্গে পরিচয়
করিয়েছি, যাতে error message-এর পরামর্শে বা অন্যের কোডে এগুলোর মুখোমুখি হলে এই ধারণা আর syntax চিনতে পারেন। সমাধানের পথ খুঁজতে এই অধ্যায়টি রেফারেন্স হিসেবে ব্যবহার করুন।

এরপর আমরা বই জুড়ে আলোচিত সবকিছু কাজে লাগিয়ে আরও একটি project করব!

[ref]: https://doc.rust-lang.org/reference/macros-by-example.html
[tlborm]: https://veykril.github.io/tlborm/
[syn]: https://crates.io/crates/syn
[quote]: https://crates.io/crates/quote
[syn-docs]: https://docs.rs/syn/2.0/syn/struct.DeriveInput.html
[quote-docs]: https://docs.rs/quote
[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming

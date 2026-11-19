# নকশার সুবিধা-অসুবিধা {#design-trade-offs}

এই অংশটি Rust-এ **নকশার সুবিধা-অসুবিধা (design trade-offs)** নিয়ে। একজন দক্ষ Rust প্রকৌশলী হতে শুধু Rust কীভাবে কাজ করে তা জানাই যথেষ্ট নয়। একটি নির্দিষ্ট কাজের জন্য Rust-এর অনেক হাতিয়ারের মধ্যে কোনগুলো উপযুক্ত তা আপনাকে ঠিক করতে হবে। এই অংশে Rust-এ নকশার সুবিধা-অসুবিধা সম্পর্কে আপনার বোঝাপড়া নিয়ে আমরা একের পর এক কুইজ দেব।  প্রতিটি কুইজের পরে প্রতিটি প্রশ্নের পেছনে আমাদের যুক্তি বিস্তারিত ব্যাখ্যা করব।

একটি প্রশ্ন দেখতে কেমন হবে তার একটি উদাহরণ এখানে দেওয়া হলো। এটি শুরু হবে একাধিক সম্ভাব্য নকশাসহ একটি software case study-র বর্ণনা দিয়ে:

> **প্রেক্ষাপট:** আপনি একটি global configuration-যুক্ত application নকশা করছেন, যেমন যাতে command-line flag থাকে।
>
> **কার্যক্ষমতা:** application-টিকে পুরো application জুড়ে এই configuration-এর immutable reference পাঠাতে হবে।
>
> **নকশা:** কার্যক্ষমতাটি implement করার জন্য নিচে কয়েকটি প্রস্তাবিত নকশা দেওয়া হলো।
>
> ```rust,ignore
> use std::rc::Rc;
> use std::sync::Arc;
>
> struct Config { 
>     flags: Flags,
>     // .. more fields ..
> }
> 
> // Option 1: use a reference
> struct ConfigRef<'a>(&'a Config);
> 
> // Option 2: use a reference-counted pointer
> struct ConfigRef(Rc<Config>);
> 
> // Option 3: use an atomic reference-counted pointer
> struct ConfigRef(Arc<Config>);
> ```

শুধু প্রেক্ষাপট আর মূল কার্যক্ষমতা দেওয়া থাকলে তিনটি নকশাই সম্ভাব্য প্রার্থী। 
কোনগুলো সবচেয়ে অর্থবহ তা ঠিক করতে system-এর লক্ষ্য সম্পর্কে আরও তথ্য দরকার।
তাই আমরা একটি নতুন চাহিদা দিই:

> নিচের চাহিদা পূরণ করে এমন প্রতিটি নকশার বিকল্প বেছে নিন:
>
> **চাহিদা:** configuration-এর reference একাধিক thread-এর মধ্যে ভাগ করা যেতে হবে।
>
> **উত্তর:**
>
> <input type="checkbox" checked disabled> Option 1 <br>
> <input type="checkbox" disabled> Option 2 <br>
> <input type="checkbox" checked disabled> Option 3 <br>

আনুষ্ঠানিক ভাষায় এর মানে `ConfigRef` [`Send`] আর [`Sync`] implement করে। 
`Config: Send + Sync` ধরে নিলে `&Config` আর `Arc<Config>` দুটিই এই চাহিদা পূরণ করে,
কিন্তু [`Rc`] করে না (কারণ non-atomic reference-counted pointer thread-safe নয়)। তাই Option 2 চাহিদা পূরণ করে না, আর Option 3 করে।

আমরা হয়তো এই সিদ্ধান্তেও পৌঁছাতে চাইব যে Option 1 চাহিদা পূরণ করে না, কারণ [`thread::spawn`]-এর মতো function চায় একটি thread-এ সরানো সব ডেটায় শুধু `'static` lifetime-এর reference থাকুক। তবে দুটি কারণে এটি Option 1-কে বাদ দেয় না:
1.  `Config` একটি global static variable হিসেবে রাখা যেতে পারে (যেমন [`OnceLock`] ব্যবহার করে), ফলে `&'static Config` reference তৈরি করা যায়।
2. সব concurrency ব্যবস্থায় `'static` lifetime লাগে না, যেমন [`thread::scope`]। 

তাই বর্ণিত চাহিদা শুধু non-[`Send`] type-গুলোকে বাদ দেয়, আর আমরা Option 1 ও 3-কে সঠিক উত্তর হিসেবে গণ্য করি।

[`thread::spawn`]: https://doc.rust-lang.org/std/thread/fn.spawn.html
[`Send`]: https://doc.rust-lang.org/std/marker/trait.Send.html
[`Sync`]: https://doc.rust-lang.org/std/marker/trait.Sync.html
[`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[`OnceLock`]: https://doc.rust-lang.org/std/sync/struct.OnceLock.html
[`thread::scope`]: https://doc.rust-lang.org/std/thread/fn.scope.html

<hr>

এবার নিচের প্রশ্নগুলো দিয়ে আপনি চেষ্টা করুন! প্রতিটি অংশে একটিমাত্র পরিস্থিতির ওপর কেন্দ্রীভূত একটি কুইজ আছে। কুইজটি শেষ করুন, এবং প্রতিটি কুইজের পরে উত্তরের ব্যাখ্যা অবশ্যই পড়ুন।
 <!-- These questions are both experimental and opinionated &mdash; please leave us feedback via the bug button 🐞 if you disagree with our answers. -->

প্রতিটি কুইজের সঙ্গে আমরা সেই জনপ্রিয় Rust crate-গুলোর link-ও দিয়েছি, যেগুলো কুইজটির অনুপ্রেরণা ছিল।

## reference {#references}

*অনুপ্রেরণা:* [Bevy assets], [Petgraph node indices], [Cargo units]

{{#quiz ../quizzes/ch17-05-design-challenge-references.toml}}


[Bevy assets]: https://docs.rs/bevy/0.11.2/bevy/asset/struct.Assets.html
[Petgraph node indices]: https://docs.rs/petgraph/0.6.4/petgraph/graph/struct.NodeIndex.html
[Cargo units]: https://docs.rs/cargo/0.73.1/cargo/core/compiler/struct.Unit.html

## trait tree {#trait-trees}

*অনুপ্রেরণা:* [Yew components], [Druid widgets]

{{#quiz ../quizzes/ch17-05-design-challenge-trait-trees.toml}}

[Yew components]: https://docs.rs/yew/0.20.0/yew/html/trait.Component.html
[Druid widgets]: https://docs.rs/druid/0.8.3/druid/trait.Widget.html

## dispatch {#dispatch}

*অনুপ্রেরণা:* [Bevy systems], [Diesel queries], [Axum handlers]

{{#quiz ../quizzes/ch17-05-design-challenge-dispatch.toml}}

[Bevy systems]: https://docs.rs/bevy_ecs/0.11.2/bevy_ecs/system/trait.IntoSystem.html
[Diesel queries]: https://docs.diesel.rs/2.1.x/diesel/query_dsl/trait.BelongingToDsl.html
[Axum handlers]: https://docs.rs/axum/0.6.20/axum/handler/trait.Handler.html

## মধ্যবর্তী রূপ {#intermediates}

*অনুপ্রেরণা:* [Serde] আর [miniserde]

{{#quiz ../quizzes/ch17-05-design-challenge-intermediates.toml}}

[Serde]: https://docs.rs/serde/1.0.188/serde/trait.Serialize.html
[miniserde]: https://docs.rs/miniserde/0.1.34/miniserde/trait.Serialize.html
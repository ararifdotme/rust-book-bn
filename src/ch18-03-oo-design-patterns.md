## একটি object-oriented design pattern implement করা {#implementing-an-object-oriented-design-pattern}

_state pattern_ হলো একটি object-oriented design pattern। এই pattern-এর মূল কথা হলো, একটি মান ভেতরে ভেতরে কী কী অবস্থায় থাকতে পারে তার একটি সেট আমরা সংজ্ঞায়িত করি। অবস্থাগুলো বোঝানো হয়
কিছু _state objects_ (state object) দিয়ে, আর মানটির আচরণ তার অবস্থা অনুযায়ী বদলায়। আমরা একটি blog post struct-এর উদাহরণ নিয়ে কাজ করব, যার একটি field-এ এর অবস্থা থাকবে, যা হবে “draft,”
“review,” বা “published” সেটের একটি state object।

state object-গুলো কার্যক্ষমতা ভাগ করে নেয়: অবশ্যই Rust-এ আমরা object আর inheritance-এর বদলে struct আর trait ব্যবহার করি। প্রতিটি state object নিজের আচরণের এবং কখন অন্য অবস্থায় বদলাবে তা
নিয়ন্ত্রণের দায়িত্বে থাকে। যে মান একটি state object রাখে, সে অবস্থাগুলোর ভিন্ন ভিন্ন আচরণ বা কখন এক অবস্থা থেকে আরেক অবস্থায় যেতে হবে, সে সম্পর্কে কিছুই জানে না।

state pattern ব্যবহারের সুবিধা হলো, প্রোগ্রামের ব্যবসায়িক চাহিদা বদলালে অবস্থা রাখা মানের কোড বা সেই মান ব্যবহার করা কোড বদলাতে হবে না। নিয়ম বদলাতে, বা হয়তো আরও state object যোগ করতে, শুধু
একটি state object-এর ভেতরের কোড হালনাগাদ করতে হবে।

প্রথমে আমরা আরও প্রথাগত object-oriented উপায়ে state pattern implement করব। তারপর Rust-এ একটু বেশি স্বাভাবিক একটি পদ্ধতি ব্যবহার করব। চলুন state pattern ব্যবহার করে ধাপে ধাপে একটি blog post-এর কাজের
ধারা implement করি।

চূড়ান্ত কার্যক্ষমতা দেখতে এমন হবে:

1. একটি blog post শুরু হয় একটি খালি draft হিসেবে।
1. draft শেষ হলে post-টির review-এর অনুরোধ করা হয়।
1. post অনুমোদিত হলে এটি প্রকাশিত হয়।
1. শুধু প্রকাশিত blog post-ই print করার জন্য বিষয়বস্তু return করে, যাতে অননুমোদিত post ভুল করে প্রকাশিত হয়ে যেতে না পারে।

একটি post-এ চেষ্টা করা অন্য যেকোনো পরিবর্তনের কোনো প্রভাব থাকা উচিত নয়। যেমন, review-এর অনুরোধ করার আগেই একটি draft blog post অনুমোদন করার চেষ্টা করলে post-টি অপ্রকাশিত draft হিসেবেই থাকা
উচিত।

<!-- Old headings. Do not remove or links may break. -->

<a id="a-traditional-object-oriented-attempt"></a>

### প্রথাগত object-oriented ধাঁচের চেষ্টা {#attempting-traditional-object-oriented-style}

একই সমস্যা সমাধানে কোড সাজানোর অসংখ্য উপায় আছে, প্রতিটির ভিন্ন ভিন্ন সুবিধা-অসুবিধা। এই অংশের implementation বেশি প্রথাগত object-oriented ধাঁচের, যা Rust-এ লেখা সম্ভব, কিন্তু Rust-এর কিছু শক্তির
সুবিধা নেয় না। পরে আমরা একটি ভিন্ন সমাধান দেখাব, যা এখনো object-oriented design pattern ব্যবহার করে, তবে এমনভাবে সাজানো যা object-oriented অভিজ্ঞতাসম্পন্ন প্রোগ্রামারদের কাছে কম পরিচিত মনে হতে পারে।
অন্য ভাষার কোড থেকে ভিন্নভাবে Rust কোড নকশা করার সুবিধা-অসুবিধা অনুভব করতে আমরা দুটি সমাধান তুলনা করব।

Listing 18-11-এ এই কাজের ধারা কোডের আকারে দেখানো হয়েছে: এটি `blog` নামের একটি library crate-এ আমরা যে API implement করব, তার ব্যবহারের একটি উদাহরণ। এটি এখনো compile হবে না, কারণ আমরা `blog`
crate implement করিনি।

<Listing number="18-11" file-name="src/main.rs" caption="আমাদের `blog` crate-এর কাছ থেকে কাঙ্ক্ষিত আচরণ দেখানো কোড">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:all}}
```

</Listing>

আমরা চাই ব্যবহারকারী `Post::new` দিয়ে একটি নতুন draft blog post তৈরি করতে পারুক। আমরা চাই blog post-এ লেখা যোগ করা যাক। অনুমোদনের আগেই সঙ্গে সঙ্গে post-এর বিষয়বস্তু পেতে চাইলে কোনো লেখা পাওয়া
উচিত নয়, কারণ post-টি এখনো draft। দেখানোর উদ্দেশ্যে আমরা কোডে `assert_eq!` যোগ করেছি। এর জন্য একটি চমৎকার unit test হতো এটা assert করা যে একটি draft blog post `content` method থেকে একটি খালি
string return করে, তবে এই উদাহরণের জন্য আমরা test লিখব না।

এরপর আমরা post-এর review-এর অনুরোধ সম্ভব করতে চাই, আর review-এর অপেক্ষার সময় `content` যেন একটি খালি string return করে তা চাই। post অনুমোদন পেলে এটি প্রকাশিত হওয়া উচিত, অর্থাৎ `content` call
করলে post-এর লেখা return হবে।

খেয়াল করুন, crate-এর যে একমাত্র type-এর সঙ্গে আমরা কাজ করছি তা হলো `Post` type। এই type state pattern ব্যবহার করবে এবং এমন একটি মান রাখবে, যা একটি post-এর সম্ভাব্য বিভিন্ন অবস্থা—draft, review
বা published—বোঝানো তিনটি state object-এর একটি। এক অবস্থা থেকে আরেক অবস্থায় বদল `Post` type-এর ভেতরেই সামলানো হবে। আমাদের library-র ব্যবহারকারীরা `Post` instance-এর ওপর যে method call করে,
তার সাড়া হিসেবে অবস্থা বদলায়, কিন্তু অবস্থার বদল তাদের সরাসরি সামলাতে হয় না। তা ছাড়া ব্যবহারকারীরা অবস্থা নিয়ে ভুল করতে পারে না, যেমন review-এর আগেই একটি post প্রকাশ করা।

<!-- Old headings. Do not remove or links may break. -->

<a id="defining-post-and-creating-a-new-instance-in-the-draft-state"></a>

#### `Post` সংজ্ঞায়িত করা ও নতুন instance তৈরি করা {#defining-post-and-creating-a-new-instance}

চলুন library-র implementation শুরু করি! আমরা জানি কিছু বিষয়বস্তু রাখা একটি public `Post` struct দরকার, তাই struct-এর সংজ্ঞা আর `Post`-এর instance তৈরি করার জন্য একটি সংশ্লিষ্ট public `new`
function দিয়ে শুরু করব, যেমনটি Listing 18-12-এ দেখানো হয়েছে। আমরা একটি private `State` trait-ও বানাব, যা সংজ্ঞায়িত করবে একটি `Post`-এর সব state object-এর কী আচরণ থাকতে হবে।

তারপর state object রাখতে `Post` `state` নামের একটি private field-এ একটি `Option<T>`-এর ভেতরে `Box<dyn State>`-এর একটি trait object রাখবে। `Option<T>` কেন দরকার তা একটু পরেই দেখবেন।

<Listing number="18-12" file-name="src/lib.rs" caption="একটি `Post` struct আর নতুন `Post` instance তৈরি করা একটি `new` function, একটি `State` trait, আর একটি `Draft` struct-এর সংজ্ঞা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-12/src/lib.rs}}
```

</Listing>

`State` trait বিভিন্ন post অবস্থার ভাগ করা আচরণ সংজ্ঞায়িত করে। state object-গুলো হলো `Draft`, `PendingReview` আর `Published`, আর সবগুলোই `State` trait implement করবে। আপাতত trait-টির কোনো
method নেই, আর আমরা শুধু `Draft` অবস্থা সংজ্ঞায়িত করে শুরু করব, কারণ আমরা চাই একটি post এই অবস্থা দিয়েই শুরু হোক।

একটি নতুন `Post` তৈরি করলে আমরা এর `state` field-এ একটি `Box` রাখা `Some` মান বসাই। এই `Box` `Draft` struct-এর একটি নতুন instance-কে নির্দেশ করে। এতে নিশ্চিত হয় যে যখনই `Post`-এর নতুন
instance তৈরি করি, এটি draft হিসেবে শুরু হবে। `Post`-এর `state` field যেহেতু private, তাই অন্য কোনো অবস্থায় একটি `Post` তৈরির কোনো উপায় নেই! `Post::new` function-এ আমরা `content` field-এ একটি
নতুন, খালি `String` বসাই।

#### post-এর বিষয়বস্তুর লেখা সংরক্ষণ করা {#storing-the-text-of-the-post-content}

Listing 18-11-এ দেখেছি আমরা `add_text` নামের একটি method call করে তাকে একটি `&str` পাঠাতে চাই, যা তারপর blog post-এর লেখার বিষয়বস্তু হিসেবে যোগ হয়। `content` field-কে `pub` হিসেবে প্রকাশ না
করে আমরা এটি একটি method হিসেবে implement করি, যাতে পরে এমন একটি method implement করতে পারি যা নিয়ন্ত্রণ করবে `content` field-এর ডেটা কীভাবে পড়া হবে। `add_text` method বেশ সোজাসাপ্টা, তাই
Listing 18-13-এর implementation `impl
Post` block-এ যোগ করি।

<Listing number="18-13" file-name="src/lib.rs" caption="একটি post-এর `content`-এ লেখা যোগ করতে `add_text` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-13/src/lib.rs:here}}
```

</Listing>

`add_text` method `self`-এর একটি mutable reference নেয়, কারণ যে `Post` instance-এর ওপর `add_text` call করছি তা আমরা বদলাচ্ছি। তারপর `content`-এর `String`-এর ওপর `push_str` call করি এবং সংরক্ষিত
`content`-এ যোগ করতে `text` argument পাঠাই। এই আচরণ post কোন অবস্থায় আছে তার ওপর নির্ভর করে না, তাই এটি state pattern-এর অংশ নয়। `add_text` method `state` field-এর সঙ্গে মোটেই কাজ করে না,
তবে এটি আমরা যে আচরণ সমর্থন করতে চাই তার অংশ।

<!-- Old headings. Do not remove or links may break. -->

<a id="ensuring-the-content-of-a-draft-post-is-empty"></a>

#### draft post-এর বিষয়বস্তু খালি থাকা নিশ্চিত করা {#ensuring-that-the-content-of-a-draft-post-is-empty}

`add_text` call করে post-এ কিছু বিষয়বস্তু যোগ করার পরও আমরা চাই `content` method একটি খালি string slice return করুক, কারণ post-টি এখনো draft অবস্থায় আছে, যেমনটি Listing 18-11-এর প্রথম
`assert_eq!` দেখায়। আপাতত এই চাহিদা পূরণ করে এমন সবচেয়ে সরল জিনিস দিয়ে `content` method implement করি: সবসময় একটি খালি string slice return করা। পরে post-এর অবস্থা বদলে প্রকাশ করার ক্ষমতা
implement করার পর এটি বদলাব। এ পর্যন্ত post শুধু draft অবস্থায় থাকতে পারে, তাই post-এর বিষয়বস্তু সবসময় খালি থাকা উচিত। Listing 18-14-এ এই অস্থায়ী implementation দেখানো হয়েছে।

<Listing number="18-14" file-name="src/lib.rs" caption="`Post`-এর ওপর `content` method-এর একটি অস্থায়ী implementation যোগ করা, যা সবসময় একটি খালি string slice return করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-14/src/lib.rs:here}}
```

</Listing>

এই `content` method যোগ করায় Listing 18-11-এর প্রথম
`assert_eq!` পর্যন্ত সবকিছু উদ্দেশ্য অনুযায়ী কাজ করে।

<!-- Old headings. Do not remove or links may break. -->

<a id="requesting-a-review-of-the-post-changes-its-state"></a>
<a id="requesting-a-review-changes-the-posts-state"></a>

#### review-এর অনুরোধ, যা post-এর অবস্থা বদলায় {#requesting-a-review-which-changes-the-posts-state}

এরপর একটি post-এর review-এর অনুরোধ করার কার্যক্ষমতা যোগ করতে হবে, যা এর অবস্থা `Draft` থেকে `PendingReview`-এ বদলে দেবে। Listing 18-15-এ এই কোড দেখানো হয়েছে।

<Listing number="18-15" file-name="src/lib.rs" caption="`Post` আর `State` trait-এর ওপর `request_review` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-15/src/lib.rs:here}}
```

</Listing>

আমরা `Post`-কে `request_review` নামের একটি public method দিই, যা `self`-এর একটি mutable reference নেবে। তারপর `Post`-এর বর্তমান অবস্থার ওপর একটি ভেতরের `request_review` method call করি, আর এই
দ্বিতীয় `request_review` method বর্তমান অবস্থা খরচ করে একটি নতুন অবস্থা return করে।

আমরা `State` trait-এ `request_review` method যোগ করি; trait-টি implement করা সব type-কে এখন `request_review` method implement করতে হবে। খেয়াল করুন, method-এর প্রথম parameter হিসেবে `self`,
`&self` বা `&mut self`-এর বদলে আমাদের আছে `self: Box<Self>`। এই syntax-এর মানে হলো method-টি কেবল তখনই বৈধ, যখন type-টি রাখা একটি `Box`-এর ওপর call করা হয়। এই syntax `Box<Self>`-এর ownership নেয়,
পুরোনো অবস্থাকে অবৈধ করে দেয়, যাতে `Post`-এর অবস্থার মান একটি নতুন অবস্থায় রূপান্তরিত হতে পারে।

পুরোনো অবস্থা খরচ করতে `request_review` method-কে অবস্থার মানের ownership নিতে হয়। এখানেই কাজে আসে `Post`-এর `state` field-এর `Option`: আমরা `take` method call করে `state` field থেকে `Some` মানটি বের
করে নিই এবং তার জায়গায় একটি `None` রেখে দিই, কারণ Rust struct-এ খালি field রাখতে দেয় না। এতে `state` মানটি borrow না করে `Post` থেকে বাইরে সরিয়ে নিতে পারি। তারপর post-এর `state` মান এই
operation-এর ফলে বসাব।

`state` মানের ownership পেতে আমাদের `self.state = self.state.request_review();`-এর মতো কোড দিয়ে সরাসরি না বসিয়ে সাময়িকভাবে `state`-কে `None` করতে হয়। এতে নিশ্চিত হয় যে পুরোনো `state` মানকে নতুন
অবস্থায় রূপান্তর করার পর `Post` আর সেটি ব্যবহার করতে পারবে না।

`Draft`-এর ওপর `request_review` method একটি নতুন `PendingReview` struct-এর একটি নতুন, box করা instance return করে, যা বোঝায় post review-এর অপেক্ষায় আছে এমন অবস্থা। `PendingReview` struct-ও
`request_review` method implement করে, কিন্তু কোনো রূপান্তর করে না। বরং নিজেকেই return করে, কারণ ইতিমধ্যে `PendingReview` অবস্থায় থাকা একটি post-এর জন্য review-এর অনুরোধ করলে এটি `PendingReview`
অবস্থাতেই থাকা উচিত।

এখন state pattern-এর সুবিধাগুলো দেখা শুরু করতে পারি: `Post`-এর `state` মান যা-ই হোক, এর ওপর `request_review` method একই। প্রতিটি অবস্থা নিজের নিয়মের দায়িত্বে থাকে।

`Post`-এর ওপর `content` method যেমন আছে তেমনই রেখে দেব, যা একটি খালি string slice return করে। এখন `Draft` অবস্থার পাশাপাশি `PendingReview` অবস্থাতেও একটি `Post` থাকতে পারে, কিন্তু আমরা
`PendingReview` অবস্থাতেও একই আচরণ চাই। Listing 18-11 এখন দ্বিতীয় `assert_eq!` call পর্যন্ত কাজ করে!

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-the-approve-method-that-changes-the-behavior-of-content"></a>
<a id="adding-approve-to-change-the-behavior-of-content"></a>

#### `content`-এর আচরণ বদলাতে `approve` যোগ করা {#adding-approve-to-change-contents-behavior}

`approve` method `request_review` method-এর মতোই হবে: এটি `state`-কে সেই মানে বসাবে, যা বর্তমান অবস্থা বলে সেই অবস্থা অনুমোদিত হলে থাকা উচিত, যেমনটি Listing 18-16-এ দেখানো হয়েছে।

<Listing number="18-16" file-name="src/lib.rs" caption="`Post` আর `State` trait-এর ওপর `approve` method implement করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-16/src/lib.rs:here}}
```

</Listing>

আমরা `State` trait-এ `approve` method যোগ করি এবং `State` implement করা একটি নতুন struct যোগ করি, `Published` অবস্থা।

`PendingReview`-এর ওপর `request_review` যেভাবে কাজ করে তার মতোই, একটি `Draft`-এর ওপর `approve` method call করলে কোনো প্রভাব পড়বে না, কারণ `approve` `self` return করবে। `PendingReview`-এর ওপর
`approve` call করলে এটি `Published` struct-এর একটি নতুন, box করা instance return করে। `Published` struct `State` trait implement করে, আর `request_review` method আর `approve` method দুটির ক্ষেত্রেই
নিজেকে return করে, কারণ সেসব ক্ষেত্রে post-টি `Published` অবস্থাতেই থাকা উচিত।

এবার `Post`-এর ওপর `content` method হালনাগাদ করতে হবে। আমরা চাই `content` থেকে return হওয়া মান `Post`-এর বর্তমান অবস্থার ওপর নির্ভর করুক, তাই `Post`-কে দিয়ে এর `state`-এর ওপর সংজ্ঞায়িত একটি
`content` method-এর হাতে কাজটি তুলে দেব (delegate করব), যেমনটি Listing 18-17-এ দেখানো হয়েছে।

<Listing number="18-17" file-name="src/lib.rs" caption="`Post`-এর ওপর `content` method হালনাগাদ করে `State`-এর ওপর একটি `content` method-এর হাতে কাজ তুলে দেওয়া">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-oop/listing-18-17/src/lib.rs:here}}
```

</Listing>

লক্ষ্য যেহেতু এই সব নিয়ম `State` implement করা struct-গুলোর ভেতরে রাখা, তাই আমরা `state`-এর মানের ওপর একটি `content` method call করি এবং argument হিসেবে post instance (অর্থাৎ `self`) পাঠাই।
তারপর `state` মানের ওপর `content` method ব্যবহার করে যে মান return হয় তা return করি।

`Option`-এর ওপর আমরা `as_ref` method call করি, কারণ আমরা `Option`-এর ভেতরের মানের ownership নয়, বরং একটি reference চাই। `state` যেহেতু একটি `Option<Box<dyn State>>`, তাই `as_ref` call করলে একটি
`Option<&Box<dyn
State>>` return হয়। `as_ref` call না করলে error পেতাম, কারণ function parameter-এর borrow করা `&self` থেকে `state` বাইরে সরানো যায় না।

তারপর `unwrap` method call করি, যা আমরা জানি কখনো panic করবে না, কারণ আমরা জানি `Post`-এর method-গুলো নিশ্চিত করে যে সেই method-গুলো শেষ হলে `state`-এ সবসময় একটি `Some` মান থাকবে। এটি অধ্যায় 9-এর
[“যখন আপনার কাছে compiler-এর চেয়ে বেশি তথ্য
আছে”][more-info-than-rustc]<!-- ignore --> অংশে আলোচিত ক্ষেত্রগুলোর একটি, যখন আমরা জানি যে একটি `None` মান কখনো সম্ভব নয়, যদিও compiler তা বুঝতে পারে না।

এই পর্যায়ে `&Box<dyn State>`-এর ওপর `content` call করলে `&` আর `Box`-এর ওপর deref coercion কার্যকর হবে, ফলে `content` method শেষ পর্যন্ত `State` trait implement করা type-এর ওপর call হবে। এর
মানে `State` trait-এর সংজ্ঞায় `content` যোগ করতে হবে, আর সেখানেই রাখব কোন অবস্থায় কোন বিষয়বস্তু return করতে হবে তার logic, যেমনটি Listing 18-18-এ দেখানো হয়েছে।

<Listing number="18-18" file-name="src/lib.rs" caption="`State` trait-এ `content` method যোগ করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-18/src/lib.rs:here}}
```

</Listing>

আমরা `content` method-এর জন্য একটি default implementation যোগ করি, যা একটি খালি string slice return করে। এর মানে `Draft` আর `PendingReview` struct-এ `content` implement করতে হবে না। `Published`
struct `content` method override করবে এবং `post.content`-এর মান return করবে। সুবিধাজনক হলেও `State`-এর ওপর `content` method দিয়ে `Post`-এর বিষয়বস্তু নির্ধারণ করা `State`-এর দায়িত্ব আর
`Post`-এর দায়িত্বের মধ্যের সীমারেখা ঝাপসা করে দিচ্ছে।

খেয়াল করুন, অধ্যায় 10-এ যেমন আলোচনা করেছি, এই method-এ lifetime annotation দরকার। আমরা argument হিসেবে একটি `post`-এর reference নিচ্ছি এবং সেই `post`-এর একটি অংশের reference return করছি, তাই
return করা reference-এর lifetime `post` argument-এর lifetime-এর সঙ্গে সম্পর্কিত।

আর কাজ শেষ—Listing 18-11-এর সবটাই এখন কাজ করে! আমরা blog post-এর কাজের ধারার নিয়মগুলো দিয়ে state pattern implement করেছি। নিয়ম সংক্রান্ত logic `Post` জুড়ে ছড়িয়ে না থেকে state object-গুলোর
ভেতরে থাকে।

> ### কেন enum নয়? {#why-not-an-enum}
>
> হয়তো ভাবছিলেন, post-এর সম্ভাব্য ভিন্ন ভিন্ন অবস্থাকে variant হিসেবে রেখে আমরা কেন একটি enum ব্যবহার করিনি। এটি অবশ্যই একটি সম্ভাব্য সমাধান; চেষ্টা করে দেখুন এবং শেষ ফল তুলনা করে দেখুন কোনটি
> আপনার পছন্দ! enum ব্যবহারের একটি অসুবিধা হলো, যেখানেই enum-এর মান যাচাই করা হয়, সেখানেই প্রতিটি সম্ভাব্য variant সামলাতে একটি `match` expression বা অনুরূপ কিছু লাগবে। এতে এই trait object
> সমাধানের চেয়ে বেশি পুনরাবৃত্তি হতে পারে।

<!-- Old headings. Do not remove or links may break. -->

<a id="trade-offs-of-the-state-pattern"></a>

#### state pattern-এর মূল্যায়ন {#evaluating-the-state-pattern}

আমরা দেখিয়েছি যে প্রতিটি অবস্থায় একটি post-এর যে ভিন্ন ভিন্ন আচরণ থাকা উচিত তা encapsulate করতে Rust object-oriented state pattern implement করতে সক্ষম। `Post`-এর method-গুলো বিভিন্ন আচরণ সম্পর্কে
কিছুই জানে না। কোড যেভাবে সাজিয়েছি, তাতে একটি প্রকাশিত post কীভাবে কীভাবে আচরণ করতে পারে তা জানতে শুধু এক জায়গায় দেখতে হয়: `Published` struct-এর ওপর `State` trait-এর implementation।

state pattern ব্যবহার করে না এমন একটি বিকল্প implementation তৈরি করলে আমরা হয়তো `Post`-এর method-গুলোতে বা এমনকি `main` কোডে `match` expression ব্যবহার করতাম, যা post-এর অবস্থা যাচাই করে সেসব
জায়গায় আচরণ বদলায়। তাতে একটি post প্রকাশিত অবস্থায় থাকার সব প্রভাব বুঝতে কয়েক জায়গায় দেখতে হতো।

state pattern-এ `Post`-এর method-গুলো আর যেসব জায়গায় `Post` ব্যবহার করি সেগুলোতে `match` expression লাগে না, আর একটি নতুন অবস্থা যোগ করতে শুধু একটি নতুন struct যোগ করে এক জায়গায় সেই একটি
struct-এর ওপর trait method-গুলো implement করতে হতো।

state pattern ব্যবহার করা implementation-এ আরও কার্যক্ষমতা যোগ করা সহজ। state pattern ব্যবহার করা কোড রক্ষণাবেক্ষণ কতটা সরল তা দেখতে এই পরামর্শগুলোর কয়েকটি চেষ্টা করুন:

- একটি `reject` method যোগ করুন, যা post-এর অবস্থা `PendingReview` থেকে আবার `Draft`-এ বদলায়।
- অবস্থা `Published`-এ বদলানোর আগে `approve`-এর দুটি call বাধ্যতামূলক করুন।
- ব্যবহারকারীদের শুধু তখনই লেখার বিষয়বস্তু যোগ করতে দিন, যখন post `Draft` অবস্থায় থাকে।
  ইঙ্গিত: বিষয়বস্তুর কী বদলাতে পারে তার দায়িত্ব state object-কে দিন, কিন্তু `Post` পরিবর্তনের দায়িত্ব নয়।

state pattern-এর একটি অসুবিধা হলো, যেহেতু অবস্থাগুলোই অবস্থা বদল implement করে, তাই কিছু অবস্থা একে অপরের সঙ্গে জড়িয়ে যায়। `PendingReview` আর `Published`-এর মাঝে `Scheduled`-এর মতো আরেকটি অবস্থা
যোগ করলে `PendingReview`-এর কোড বদলে বরং `Scheduled`-এ যাওয়ার ব্যবস্থা করতে হতো। নতুন অবস্থা যোগ করায় `PendingReview` না বদলাতে হলে কাজ কম হতো, কিন্তু তার মানে হতো অন্য একটি design pattern-এ
চলে যাওয়া।

আরেকটি অসুবিধা হলো, আমরা কিছু logic নকল করেছি। কিছু পুনরাবৃত্তি দূর করতে আমরা হয়তো `State` trait-এ `request_review` আর `approve` method-এর এমন default implementation বানানোর চেষ্টা করতাম, যা
`self` return করে। কিন্তু এটি কাজ করত না: `State`-কে trait object হিসেবে ব্যবহারের সময় trait জানে না concrete `self` ঠিক কী হবে, তাই compile time-এ return type জানা যায় না। (এটি আগে উল্লেখ করা dyn
compatibility-র নিয়মগুলোর একটি।)

অন্য পুনরাবৃত্তির মধ্যে আছে `Post`-এর ওপর `request_review` আর `approve` method-এর একই রকম implementation। দুটি method-ই `Post`-এর `state` field-এর সঙ্গে `Option::take` ব্যবহার করে, আর `state` যদি `Some`
হয়, তাহলে মোড়ানো মানের একই method-এর implementation-এর হাতে কাজ তুলে দেয় এবং `state` field-এর নতুন মান হিসেবে ফলটি বসায়। `Post`-এ এই pattern মেনে চলা অনেক method থাকলে পুনরাবৃত্তি দূর করতে আমরা
একটি macro সংজ্ঞায়িত করার কথা ভাবতে পারতাম (অধ্যায় 20-এর [“Macros”][macros]<!-- ignore --> অংশ দেখুন)।

object-oriented ভাষার জন্য যেভাবে সংজ্ঞায়িত, ঠিক সেভাবে state pattern implement করে আমরা Rust-এর শক্তির পুরো সুবিধা নিচ্ছি না। চলুন `blog` crate-এ এমন কিছু পরিবর্তন দেখি, যা অবৈধ অবস্থা আর
অবস্থা বদলকে compile-time error-এ পরিণত করতে পারে।

### অবস্থা আর আচরণকে type হিসেবে প্রকাশ করা {#encoding-states-and-behavior-as-types}

state pattern নতুন করে ভেবে কীভাবে ভিন্ন কিছু সুবিধা-অসুবিধা পাওয়া যায় তা দেখাব। অবস্থা আর অবস্থা বদলকে পুরোপুরি encapsulate করে বাইরের কোডকে এগুলো সম্পর্কে অজ্ঞ রাখার বদলে আমরা অবস্থাগুলোকে
ভিন্ন ভিন্ন type-এ প্রকাশ করব। ফলে যেখানে শুধু প্রকাশিত post অনুমোদিত, সেখানে draft post ব্যবহারের চেষ্টা Rust-এর type-checking ব্যবস্থা compiler error দিয়ে আটকে দেবে।

Listing 18-11-এর `main`-এর প্রথম অংশটি বিবেচনা করি:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-11/src/main.rs:here}}
```

</Listing>

আমরা এখনো `Post::new` দিয়ে draft অবস্থায় নতুন post তৈরি আর post-এর বিষয়বস্তুতে লেখা যোগ করা সম্ভব রাখি। কিন্তু draft post-এর ওপর একটি খালি string return করা `content` method রাখার বদলে
এমন ব্যবস্থা করব যাতে draft post-এ `content` method-ই না থাকে। এতে একটি draft post-এর বিষয়বস্তু পাওয়ার চেষ্টা করলে compiler error পাব, যা বলবে method-টি নেই। ফলে ভুল করে production-এ draft
post-এর বিষয়বস্তু দেখিয়ে ফেলা আমাদের পক্ষে অসম্ভব হবে, কারণ সেই কোড compile-ই হবে না।
Listing 18-19-এ একটি `Post` struct আর একটি `DraftPost` struct-এর সংজ্ঞা এবং প্রতিটির method দেখানো হয়েছে।

<Listing number="18-19" file-name="src/lib.rs" caption="`content` method-যুক্ত একটি `Post` আর `content` method ছাড়া একটি `DraftPost`">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-19/src/lib.rs}}
```

</Listing>

`Post` আর `DraftPost` দুটি struct-এরই একটি private `content` field আছে, যা blog post-এর লেখা রাখে। struct-গুলোতে আর `state` field নেই, কারণ আমরা অবস্থার প্রকাশ struct-গুলোর type-এ সরিয়ে নিচ্ছি।
`Post` struct একটি প্রকাশিত post বোঝাবে, আর এর একটি `content` method আছে, যা `content` return করে।

আমাদের এখনো একটি `Post::new` function আছে, তবে এটি `Post`-এর instance return না করে `DraftPost`-এর instance return করে। `content` যেহেতু private এবং `Post` return করে এমন কোনো function নেই, তাই
এই মুহূর্তে `Post`-এর instance তৈরি করা সম্ভব নয়।

`DraftPost` struct-এর একটি `add_text` method আছে, তাই আগের মতোই `content`-এ লেখা যোগ করা যায়, কিন্তু খেয়াল করুন `DraftPost`-এ কোনো `content` method সংজ্ঞায়িত নেই! তাই এখন প্রোগ্রাম নিশ্চিত
করে যে সব post draft post হিসেবে শুরু হয়, আর draft post-এর বিষয়বস্তু দেখানোর জন্য পাওয়া যায় না। এই বাধ্যবাধকতাগুলো এড়ানোর যেকোনো চেষ্টায় compiler error হবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="implementing-transitions-as-transformations-into-different-types"></a>

তাহলে প্রকাশিত post কীভাবে পাব? আমরা এই নিয়ম প্রয়োগ করতে চাই যে প্রকাশের আগে একটি draft post-এর review ও অনুমোদন হতে হবে। review-এর অপেক্ষায় থাকা অবস্থার একটি post-ও কোনো বিষয়বস্তু দেখানো উচিত
নয়। চলুন আরেকটি struct, `PendingReviewPost`, যোগ করে, `DraftPost`-এর ওপর `PendingReviewPost` return করা `request_review` method সংজ্ঞায়িত করে, আর `PendingReviewPost`-এর ওপর `Post` return করা
`approve` method সংজ্ঞায়িত করে এই বাধ্যবাধকতাগুলো implement করি, যেমনটি Listing 18-20-এ দেখানো হয়েছে।

<Listing number="18-20" file-name="src/lib.rs" caption="`DraftPost`-এর ওপর `request_review` call করে তৈরি হওয়া একটি `PendingReviewPost`, আর একটি `approve` method, যা `PendingReviewPost`-কে একটি প্রকাশিত `Post`-এ রূপান্তর করে">

```rust,noplayground
{{#rustdoc_include ../listings/ch18-oop/listing-18-20/src/lib.rs:here}}
```

</Listing>

`request_review` আর `approve` method `self`-এর ownership নেয়, ফলে `DraftPost` আর `PendingReviewPost` instance খরচ করে এদের যথাক্রমে একটি `PendingReviewPost` আর একটি প্রকাশিত `Post`-এ রূপান্তর
করে। এতে `request_review` call করার পর কোনো `DraftPost` instance আর পড়ে থাকবে না, ইত্যাদি। `PendingReviewPost` struct-এ কোনো `content` method সংজ্ঞায়িত নেই, তাই `DraftPost`-এর মতোই এর
বিষয়বস্তু পড়ার চেষ্টা করলে compiler error হয়। `content` method সংজ্ঞায়িত থাকা একটি প্রকাশিত `Post` instance পাওয়ার একমাত্র উপায় হলো একটি `PendingReviewPost`-এর ওপর `approve` method call করা, আর
একটি `PendingReviewPost` পাওয়ার একমাত্র উপায় হলো একটি `DraftPost`-এর ওপর `request_review` method call করা, তাই আমরা এখন blog post-এর কাজের ধারা type system-এ প্রকাশ করে ফেলেছি।

তবে `main`-এও কিছু ছোট পরিবর্তন করতে হবে। `request_review` আর `approve` method যে struct-এর ওপর call করা হয় তা পরিবর্তন না করে নতুন instance return করে, তাই return হওয়া instance-গুলো রাখতে আরও
`let post =` shadowing assignment যোগ করতে হবে। draft আর review-অপেক্ষমাণ post-এর বিষয়বস্তু খালি string হওয়ার assertion-ও রাখা যাবে না, দরকারও নেই: ওই অবস্থার post-এর বিষয়বস্তু ব্যবহারের চেষ্টা
করা কোড আর compile করা যায় না। `main`-এর হালনাগাদ করা কোড Listing 18-21-এ দেখানো হয়েছে।

<Listing number="18-21" file-name="src/main.rs" caption="blog post-এর কাজের ধারার নতুন implementation ব্যবহার করতে `main`-এ পরিবর্তন">

```rust,ignore
{{#rustdoc_include ../listings/ch18-oop/listing-18-21/src/main.rs}}
```

</Listing>

`post` আবার বসাতে `main`-এ যে পরিবর্তন করতে হয়েছে, তার মানে এই implementation আর ঠিক object-oriented state pattern মেনে চলে না: অবস্থাগুলোর মধ্যের রূপান্তর আর পুরোপুরি `Post`-এর implementation-এর
ভেতরে encapsulate করা নেই। তবে আমাদের লাভ হলো, type system আর compile time-এর type checking-এর কারণে অবৈধ অবস্থা এখন অসম্ভব! এতে নিশ্চিত হয় যে কিছু বাগ, যেমন একটি অপ্রকাশিত post-এর বিষয়বস্তু
দেখানো, production-এ পৌঁছানোর আগেই ধরা পড়বে।

Listing 18-21-এর পরের অবস্থার `blog` crate-এ এই অংশের শুরুতে প্রস্তাবিত কাজগুলো চেষ্টা করে দেখুন, কোডের এই সংস্করণের নকশা সম্পর্কে আপনার কী মনে হয়। খেয়াল করুন, এই নকশায় কিছু কাজ হয়তো আগে
থেকেই সম্পন্ন।

আমরা দেখেছি যে Rust object-oriented design pattern implement করতে সক্ষম হলেও Rust-এ অন্য pattern-ও আছে, যেমন অবস্থাকে type system-এ প্রকাশ করা। এই pattern-গুলোর সুবিধা-অসুবিধা ভিন্ন। আপনি
object-oriented pattern-এর সঙ্গে খুব পরিচিত হলেও Rust-এর বৈশিষ্ট্যের সুবিধা নিতে সমস্যাটি নতুন করে ভাবলে লাভ হতে পারে, যেমন compile time-এ কিছু বাগ ঠেকানো। ownership-এর মতো কিছু বৈশিষ্ট্যের
কারণে, যা object-oriented ভাষায় নেই, Rust-এ object-oriented pattern সবসময় সেরা সমাধান হবে না।

## সারসংক্ষেপ {#summary}

এই অধ্যায় পড়ার পর আপনি Rust-কে object-oriented ভাষা মনে করুন বা না করুন, আপনি এখন জানেন যে Rust-এ কিছু object-oriented বৈশিষ্ট্য পেতে trait object ব্যবহার করা যায়। dynamic dispatch সামান্য runtime
performance-এর বিনিময়ে আপনার কোডকে কিছু নমনীয়তা দিতে পারে। এই নমনীয়তা কাজে লাগিয়ে এমন object-oriented pattern implement করতে পারেন, যা আপনার কোডের রক্ষণাবেক্ষণ সহজ করতে পারে। Rust-এর আরও কিছু
বৈশিষ্ট্য আছে, যেমন ownership, যা object-oriented ভাষায় নেই। Rust-এর শক্তির সুবিধা নেওয়ার জন্য object-oriented pattern সবসময় সেরা উপায় হবে না, তবে এটি একটি বিকল্প হিসেবে আছে।

এরপর আমরা pattern দেখব, যা Rust-এর আরেকটি বৈশিষ্ট্য এবং অনেক নমনীয়তা দেয়। বই জুড়ে সংক্ষেপে এগুলো দেখেছি, তবে এদের পুরো ক্ষমতা এখনো দেখিনি। চলুন শুরু করি!

{{#quiz ../quizzes/ch17-03-oo-design-patterns.toml}}

[more-info-than-rustc]: ch09-03-to-panic-or-not-to-panic.html#cases-in-which-you-have-more-information-than-the-compiler
[macros]: ch20-05-macros.html#macros

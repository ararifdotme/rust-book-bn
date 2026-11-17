<!-- Old headings. Do not remove or links may break. -->

<a id="the-match-control-flow-operator"></a>

## `match` control flow গঠন {#the-match-control-flow-construct}

Rust-এ `match` নামের অত্যন্ত শক্তিশালী একটি control flow (কোড চলার ক্রম নিয়ন্ত্রণ) গঠন আছে,
যা দিয়ে একটি মানকে কয়েকটি pattern (মানের গঠনের নকশা)-এর সঙ্গে একে একে তুলনা করা যায়, আর
কোন pattern মিলল তার ভিত্তিতে কোড চালানো যায়। pattern গঠিত হতে পারে literal (কোডে সরাসরি লেখা
মান) মান, variable-এর নাম, wildcard (যেকোনো মানের সঙ্গে মেলে এমন pattern) এবং আরও অনেক কিছু
দিয়ে; [অধ্যায়
19][ch19-00-patterns]<!-- ignore -->-এ সব ধরনের pattern আর সেগুলো কী করে, তা আলোচনা করা
হয়েছে। `match`-এর শক্তি আসে pattern-গুলোর প্রকাশক্ষমতা থেকে, আর এই বাস্তবতা থেকে যে
compiler নিশ্চিত করে সব সম্ভাব্য ক্ষেত্র সামলানো হয়েছে।

একটি `match` expression-কে মুদ্রা বাছাইয়ের একটি যন্ত্রের মতো ভাবুন: মুদ্রাগুলো একটি পথ বেয়ে
গড়িয়ে নামে, যার গায়ে নানা মাপের ছিদ্র আছে, আর প্রতিটি মুদ্রা পথে পড়া প্রথম যে ছিদ্রে এঁটে
যায়, সেটি দিয়েই নিচে পড়ে যায়। একইভাবে, মানগুলো একটি `match`-এর প্রতিটি pattern পেরিয়ে
যায়, আর প্রথম যে pattern-এ মানটি “এঁটে যায়”, মানটি সেই pattern-এর সঙ্গে যুক্ত কোড block-এ
গিয়ে পড়ে, যা চলার সময় ব্যবহার হয়।

মুদ্রার কথাই যখন উঠল, `match` ব্যবহারের উদাহরণ হিসেবে মুদ্রাই নেওয়া যাক! আমরা এমন একটি
function লিখতে পারি, যা একটি অজানা মার্কিন মুদ্রা নেয় এবং গোনার যন্ত্রের মতো করে ঠিক করে
সেটি কোন মুদ্রা, আর সেন্টে তার মান return করে, Listing 6-3-এ যেমন দেখানো হয়েছে।

<Listing number="6-3" caption="একটি enum আর একটি `match` expression, যার pattern হলো enum-টির variant-গুলো">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

</Listing>

চলুন `value_in_cents` function-এর `match`-টি ভেঙে দেখি। প্রথমে আমরা `match` keyword লিখি,
তারপর একটি expression, যা এই ক্ষেত্রে `coin` মানটি। এটি দেখতে `if`-এর সঙ্গে ব্যবহৃত একটি
শর্তমূলক expression-এর খুব কাছাকাছি, কিন্তু একটি বড় পার্থক্য আছে: `if`-এর ক্ষেত্রে শর্তকে
একটি Boolean মানে evaluate হতে হয়, কিন্তু এখানে তা যেকোনো type হতে পারে। এই উদাহরণে `coin`-এর
type হলো প্রথম লাইনে সংজ্ঞায়িত `Coin` enum।

এরপর আসে `match`-এর arm (`match`-এর একেকটি শাখা)-গুলো। একটি arm-এর দুটি অংশ: একটি pattern
আর কিছু কোড। এখানে প্রথম arm-এর pattern হলো `Coin::Penny` মানটি, তারপর `=>` operator, যা
pattern আর চালানোর কোডকে আলাদা করে। এই ক্ষেত্রে কোডটি শুধু `1` মান। প্রতিটি arm পরেরটি থেকে
একটি কমা দিয়ে আলাদা।

`match` expression চলার সময় এটি ফলাফলের মানটিকে ক্রমানুসারে প্রতিটি arm-এর pattern-এর সঙ্গে
তুলনা করে। কোনো pattern মানটির সঙ্গে মিললে সেই pattern-এর সঙ্গে যুক্ত কোড চলে। pattern না
মিললে চলা পরের arm-এ চলে যায়, অনেকটা মুদ্রা বাছাইয়ের যন্ত্রের মতো। যত arm দরকার, তত রাখা যায়:
Listing 6-3-এ আমাদের `match`-এ চারটি arm আছে।

প্রতিটি arm-এর সঙ্গে যুক্ত কোড একটি expression, আর মিলে যাওয়া arm-এর expression-এর ফলাফলের
মানই পুরো `match` expression-এর return মান হয়।

match arm-এর কোড ছোট হলে আমরা সাধারণত কার্লি ব্র্যাকেট ব্যবহার করি না, যেমন Listing 6-3-এ,
যেখানে প্রতিটি arm শুধু একটি মান return করে। একটি match arm-এ একাধিক লাইনের কোড চালাতে চাইলে
কার্লি ব্র্যাকেট ব্যবহার করতেই হবে, আর তখন arm-এর পরের কমাটি ঐচ্ছিক। যেমন, নিচের কোডটি
যতবার method-টি একটি `Coin::Penny` দিয়ে call হয়, ততবার “Lucky penny!” প্রিন্ট করে, কিন্তু
তবু block-এর শেষ মান `1` return করে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### যে pattern মানের সঙ্গে bind হয় {#patterns-that-bind-to-values}

match arm-এর আরেকটি কাজের বৈশিষ্ট্য হলো, pattern-এর সঙ্গে মিলে যাওয়া মানের অংশগুলোর সঙ্গে এরা
bind (কোনো নামের সঙ্গে মান জুড়ে দেওয়া) হতে পারে। এভাবেই আমরা enum variant থেকে মান বের করে
আনতে পারি।

উদাহরণ হিসেবে, চলুন আমাদের একটি enum variant-কে বদলে এর ভেতরে ডেটা রাখার মতো করি। 1999
থেকে 2008 সাল পর্যন্ত যুক্তরাষ্ট্র quarter (25 সেন্টের মুদ্রা) তৈরি করেছিল, যার এক পাশে 50টি
অঙ্গরাজ্যের প্রতিটির জন্য আলাদা নকশা ছিল। অন্য কোনো মুদ্রায় অঙ্গরাজ্যের নকশা ছিল না, তাই শুধু
quarter-এরই এই বাড়তি মান আছে। `Quarter` variant-টি বদলে এর ভেতরে একটি `UsState` মান রাখার মতো
করে আমরা আমাদের `enum`-এ এই তথ্য যোগ করতে পারি, যা আমরা Listing 6-4-এ করেছি।

<Listing number="6-4" caption="একটি `Coin` enum, যার `Quarter` variant একটি `UsState` মানও রাখে">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

</Listing>

কল্পনা করুন, একজন বন্ধু 50টি অঙ্গরাজ্যের সব quarter সংগ্রহের চেষ্টা করছে। খুচরো পয়সাগুলো
মুদ্রার ধরন অনুযায়ী বাছাই করার সময় আমরা প্রতিটি quarter-এর সঙ্গে যুক্ত অঙ্গরাজ্যের নামও বলে
দেব, যাতে সেটি যদি এমন কোনোটি হয়, যা বন্ধুর কাছে নেই, তাহলে সে সেটি তার সংগ্রহে যোগ করতে পারে।

এই কোডের match expression-এ আমরা `Coin::Quarter` variant-এর মানের সঙ্গে মেলে এমন pattern-এ
`state` নামের একটি variable যোগ করি। একটি `Coin::Quarter` মিললে `state` variable সেই quarter-এর
অঙ্গরাজ্যের মানের সঙ্গে bind হবে। তারপর ওই arm-এর কোডে আমরা `state` ব্যবহার করতে পারি, এভাবে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

আমরা যদি `value_in_cents(Coin::Quarter(UsState::Alaska))` call করতাম, তাহলে `coin` হতো
`Coin::Quarter(UsState::Alaska)`। সেই মানটিকে প্রতিটি match arm-এর সঙ্গে তুলনা করলে
`Coin::Quarter(state)`-এ পৌঁছানোর আগে কোনোটিই মেলে না। সেখানে পৌঁছালে `state`-এর binding হবে
`UsState::Alaska` মানটি। তারপর আমরা সেই binding `println!` expression-এ ব্যবহার করতে পারি,
এভাবে `Quarter`-এর জন্য `Coin` enum variant থেকে ভেতরের অঙ্গরাজ্যের মানটি বের করে আনি।

<!-- Old headings. Do not remove or links may break. -->

<a id="matching-with-optiont"></a>

### `Option<T>`-এর `match` pattern {#the-optiont-match-pattern}


আগের অংশে আমরা `Option<T>` ব্যবহারের সময় `Some` ক্ষেত্রটি থেকে ভেতরের `T` মানটি বের করতে
চেয়েছিলাম; `Coin` enum-এর মতোই আমরা `match` ব্যবহার করে `Option<T>`-ও সামলাতে পারি! মুদ্রা
তুলনার বদলে আমরা `Option<T>`-এর variant-গুলো তুলনা করব, কিন্তু `match` expression কীভাবে
কাজ করে, তা একই থাকে।

ধরুন আমরা এমন একটি function লিখতে চাই, যা একটি `Option<i32>` নেয়, আর ভেতরে কোনো মান থাকলে
সেই মানের সঙ্গে 1 যোগ করে। ভেতরে কোনো মান না থাকলে function-টি `None` মান return করবে, আর
কোনো operation চালানোর চেষ্টা করবে না।

`match`-এর কল্যাণে এই function লেখা খুবই সহজ, আর এটি দেখতে হবে
Listing 6-5-এর মতো।

<Listing number="6-5" caption="একটি function, যা একটি `Option<i32>`-এর ওপর `match` expression ব্যবহার করে">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

</Listing>

চলুন `plus_one`-এর প্রথমবার চলাটি আরও বিস্তারিত দেখি। আমরা যখন `plus_one(five)` call করি,
তখন `plus_one`-এর body-তে `x` variable-এর মান হবে `Some(5)`। তারপর আমরা সেটিকে প্রতিটি match
arm-এর সঙ্গে তুলনা করি:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

`Some(5)` মানটি `None` pattern-এর সঙ্গে মেলে না, তাই আমরা পরের arm-এ
যাই:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` কি `Some(i)`-এর সঙ্গে মেলে? মেলে! আমাদের variant একই। `i` `Some`-এর ভেতরের মানের
সঙ্গে bind হয়, তাই `i` পায় `5` মান। তারপর match arm-এর কোড চলে, তাই আমরা `i`-এর মানের সঙ্গে
1 যোগ করি এবং ভেতরে মোট `6` রেখে একটি নতুন `Some` মান তৈরি করি।

এবার Listing 6-5-এ `plus_one`-এর দ্বিতীয় call-টি দেখা যাক, যেখানে `x` হলো `None`। আমরা
`match`-এ ঢুকি এবং প্রথম arm-এর সঙ্গে তুলনা করি:

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

মিলে গেছে! যোগ করার মতো কোনো মান নেই, তাই প্রোগ্রাম থেমে যায় এবং `=>`-এর ডান পাশের `None`
মানটি return করে। প্রথম arm মিলে যাওয়ায় অন্য কোনো arm-এর সঙ্গে আর তুলনা করা হয় না।

অনেক পরিস্থিতিতে `match` আর enum একসঙ্গে ব্যবহার করা কাজের। Rust কোডে এই ধরনটি আপনি প্রচুর
দেখবেন: একটি enum-এর ওপর `match` করা, ভেতরের ডেটার সঙ্গে একটি variable bind করা, তারপর তার
ভিত্তিতে কোড চালানো। শুরুতে একটু জটিল লাগে, কিন্তু একবার অভ্যস্ত হয়ে গেলে মনে হবে, সব ভাষাতেই
যদি এটা থাকত! ব্যবহারকারীদের কাছে এটি বরাবরই প্রিয়।

### match exhaustive {#matches-are-exhaustive}

`match`-এর আরেকটি দিক আমাদের আলোচনা করা দরকার: arm-গুলোর pattern-কে সব সম্ভাবনা ঢাকতে হবে।
আমাদের `plus_one` function-এর এই সংস্করণটি দেখুন, যাতে একটি বাগ আছে এবং যা compile হবে না:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

আমরা `None` ক্ষেত্রটি সামলাইনি, তাই এই কোড একটি বাগ ঘটাবে। সৌভাগ্যক্রমে, এটি এমন একটি বাগ,
যা Rust ধরতে জানে। এই কোড compile করার চেষ্টা করলে এই error পাব:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust জানে যে আমরা সব সম্ভাব্য ক্ষেত্র ঢাকিনি, এমনকি কোন pattern ভুলে গেছি, তা-ও জানে! Rust-এ
match _exhaustive_ (সব সম্ভাবনা সামলানো): কোড বৈধ হতে হলে আমাদের প্রতিটি সম্ভাবনা শেষ পর্যন্ত
সামলাতে হবে। বিশেষ করে `Option<T>`-এর ক্ষেত্রে, Rust যখন আমাদের `None` ক্ষেত্রটি স্পষ্টভাবে
সামলাতে ভুলে যাওয়া আটকায়, তখন এটি আমাদের এমন ভুল ধারণা থেকে রক্ষা করে যে আমাদের কাছে মান
আছে, অথচ হয়তো আছে null, ফলে আগে আলোচিত বিলিয়ন ডলারের ভুলটি অসম্ভব হয়ে যায়।

### catch-all pattern আর `_` placeholder {#catch-all-patterns-and-the-_-placeholder}

enum ব্যবহার করে আমরা কয়েকটি নির্দিষ্ট মানের জন্য বিশেষ কাজ করতে পারি, আর বাকি সব মানের জন্য
একটি ডিফল্ট কাজ করতে পারি। কল্পনা করুন, আমরা এমন একটি খেলা implement করছি, যেখানে ছক্কা
চালিয়ে 3 উঠলে আপনার খেলোয়াড় নড়ে না, বরং একটি দারুণ নতুন টুপি পায়। 7 উঠলে আপনার খেলোয়াড়
একটি দারুণ টুপি হারায়। বাকি সব মানের জন্য আপনার খেলোয়াড় খেলার বোর্ডে ততগুলো ঘর এগোয়।
এখানে একটি `match` আছে, যা সেই যুক্তি implement করে; এতে ছক্কার ফল র‍্যান্ডম মানের বদলে
hardcoded (কোডে সরাসরি লিখে দেওয়া), আর বাকি সব যুক্তি body-ছাড়া function দিয়ে প্রকাশ করা
হয়েছে, কারণ সেগুলো আসলে implement করা এই উদাহরণের পরিধির বাইরে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

প্রথম দুটি arm-এর pattern হলো literal মান `3` আর `7`। বাকি সব সম্ভাব্য মান ঢাকা শেষ arm-এর
pattern হলো একটি variable, যার নাম আমরা দিয়েছি `other`। `other` arm-এর জন্য চলা কোড variable-টি
`move_player` function-এ পাঠিয়ে সেটি ব্যবহার করে।

একটি `u8`-এর সম্ভাব্য সব মান তালিকাভুক্ত না করলেও এই কোড compile হয়, কারণ শেষ pattern-টি
নির্দিষ্টভাবে তালিকাভুক্ত নয় এমন সব মানের সঙ্গে মিলবে। এই catch-all (বাকি সব কিছুর সঙ্গে মিলে
যায় এমন) pattern `match`-এর exhaustive হওয়ার শর্ত পূরণ করে। লক্ষ করুন, catch-all arm-টি
আমাদের সবার শেষে রাখতে হয়, কারণ pattern-গুলো ক্রমানুসারে evaluate হয়। catch-all arm-টি আগে
রাখলে অন্য arm-গুলো কখনো চলত না, তাই catch-all-এর পরে arm যোগ করলে Rust আমাদের সতর্ক করবে!

যখন আমরা একটি catch-all চাই, কিন্তু catch-all pattern-এর মানটি _ব্যবহার_ করতে চাই না, তখনের
জন্যও Rust-এ একটি pattern আছে: `_` একটি বিশেষ pattern, যা যেকোনো মানের সঙ্গে মেলে কিন্তু সেই
মানের সঙ্গে bind হয় না। এতে Rust বোঝে আমরা মানটি ব্যবহার করব না, তাই Rust অব্যবহৃত variable
নিয়ে আমাদের সতর্ক করবে না।

চলুন খেলার নিয়ম বদলাই: এখন 3 বা 7 ছাড়া অন্য কিছু উঠলে আপনাকে আবার ছক্কা চালাতে হবে। আমাদের
আর catch-all মানটি ব্যবহারের দরকার নেই, তাই `other` নামের variable-এর বদলে `_` ব্যবহার করতে
কোডটি বদলাতে পারি:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

এই উদাহরণও exhaustive হওয়ার শর্ত পূরণ করে, কারণ শেষ arm-এ আমরা বাকি সব মান স্পষ্টভাবে
উপেক্ষা করছি; কিছুই ভুলে যাইনি।

সবশেষে, খেলার নিয়ম আরও একবার বদলাব, যাতে 3 বা 7 ছাড়া অন্য কিছু উঠলে আপনার পালায় আর কিছুই
না ঘটে। এটি প্রকাশ করতে পারি `_` arm-এর কোড হিসেবে unit মান ([“tuple
type”][tuples]<!-- ignore --> অংশে উল্লেখ করা খালি tuple type) ব্যবহার করে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

এখানে আমরা Rust-কে স্পষ্টভাবে বলছি যে আগের কোনো arm-এর pattern-এর সঙ্গে মেলে না এমন অন্য
কোনো মান আমরা ব্যবহার করব না, আর এই ক্ষেত্রে কোনো কোড চালাতে চাই না।

pattern আর matching নিয়ে আরও কথা আছে, যা আমরা [অধ্যায়
19][ch19-00-patterns]<!-- ignore -->-এ আলোচনা করব।

<!-- BEGIN INTERVENTION: 1e4f082c-ffa4-4d33-8726-2dbcd72e1aa2 -->
### match কীভাবে ownership-এর সঙ্গে মিথস্ক্রিয়া করে {#how-matches-interact-with-ownership}

কোনো enum-এ যদি String-এর মতো non-copyable ডেটা থাকে, তাহলে একটি match সেই ডেটা move করবে নাকি borrow করবে, সে বিষয়ে সতর্ক থাকা উচিত। যেমন, একটি `Option<String>` ব্যবহার করা এই প্রোগ্রামটি compile হবে:

```aquascope,permissions,stepper,boundaries
# fn main() {
let opt: Option<String> = 
    Some(String::from("Hello world"));

match opt {
    Some(_) => println!("Some!"),
    None => println!("None!")
};

println!("{:?}", opt);
# }
```

কিন্তু `Some(_)`-এর placeholder-টির বদলে যদি একটি variable-এর নাম বসাই, যেমন `Some(s)`, তাহলে প্রোগ্রামটি compile **হবে না**:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let opt: Option<String> = 
    Some(String::from("Hello world"));

match opt {
    // _ became s
    Some(s) => println!("Some: {}", s),
    None => println!("None!")
};

println!("{:?}", opt);`{}`
#}
```


`opt` একটি সাধারণ enum &mdash; এর type হলো `Option<String>`, `&Option<String>`-এর মতো কোনো reference নয়। তাই `opt`-এর ওপর একটি match `s`-এর মতো উপেক্ষা না করা field-গুলো move করে ফেলবে। লক্ষ করুন, প্রথম প্রোগ্রামের তুলনায় দ্বিতীয় প্রোগ্রামে `opt` আগেভাগেই read আর own permission হারায়। match expression-এর পরে `opt`-এর ভেতরের ডেটা move হয়ে গেছে, তাই `println`-এ `opt` পড়া অবৈধ।

যদি `opt`-এর বিষয়বস্তু move না করে এর ভেতরে উঁকি দিতে চাই, তাহলে Rust-সুলভ সমাধান হলো একটি reference-এর ওপর match করা:

```aquascope,permissions,stepper,boundaries
#fn main() {
let opt: Option<String> = 
    Some(String::from("Hello world"));

// opt became &opt
match &opt {
    Some(s) => println!("Some: {}", s),
    None => println!("None!")
};

println!("{:?}", opt);
#}
```

Rust বাইরের enum `&Option<String>` থেকে reference-টিকে ভেতরের field `&String`-এ “নামিয়ে দেয়”। তাই `s`-এর type হয় `&String`, আর match-এর পরেও `opt` ব্যবহার করা যায়। এই “নামিয়ে দেওয়া”র কৌশল আরও ভালোভাবে বুঝতে Rust Reference-এর [binding mode](https://doc.rust-lang.org/reference/patterns.html#binding-modes) (pattern-এ মান move না reference হিসেবে bind হবে, তার নিয়ম) অংশটি দেখুন।
<!-- END INTERVENTION -->

{{#quiz ../quizzes/ch06-02-match.toml}}

[tuples]: ch03-02-data-types.html#the-tuple-type

[ch19-00-patterns]: ch19-00-patterns.html

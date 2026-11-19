## pattern syntax {#pattern-syntax}

এই অংশে pattern-এ বৈধ সব syntax আমরা এক জায়গায় জড়ো করেছি, আর আলোচনা করেছি কেন ও কখন এগুলোর প্রতিটি ব্যবহার করতে চাইতে পারেন।

### literal-এর সঙ্গে মেলানো {#matching-literals}

অধ্যায় 6-এ যেমন দেখেছেন, pattern-কে সরাসরি literal-এর সঙ্গে মেলানো যায়। নিচের কোডে কয়েকটি উদাহরণ আছে:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

এই কোড print করে `one`, কারণ `x`-এর মান `1`। কোনো নির্দিষ্ট concrete মান পেলে আপনার কোড দিয়ে কোনো কাজ করাতে চাইলে এই syntax কাজে লাগে।

### নাম দেওয়া variable-এর সঙ্গে মেলানো {#matching-named-variables}

নাম দেওয়া variable হলো এমন irrefutable pattern, যা যেকোনো মানের সঙ্গে মেলে, আর এই বইয়ে আমরা এগুলো বহুবার ব্যবহার করেছি। তবে `match`, `if let` বা `while let` expression-এ নাম দেওয়া variable
ব্যবহার করলে একটি জটিলতা আছে। এই ধরনের প্রতিটি expression একটি নতুন scope শুরু করে বলে এই expression-গুলোর ভেতরে pattern-এর অংশ হিসেবে ঘোষিত variable গঠনের বাইরের একই নামের variable-কে shadow করবে,
সব variable-এর ক্ষেত্রে যেমন হয়। Listing 19-11-এ আমরা `Some(5)` মানসহ `x` নামের একটি variable আর `10` মানসহ `y` নামের একটি variable ঘোষণা করি। তারপর `x` মানের ওপর একটি `match` expression তৈরি
করি। match arm-গুলোর pattern আর শেষের `println!` দেখুন, এবং কোডটি চালানোর বা আরও পড়ার আগে বের করার চেষ্টা করুন কোডটি কী print করবে।

<Listing number="19-11" file-name="src/main.rs" caption="এমন একটি arm-যুক্ত `match` expression, যা একটি নতুন variable আনে যা বিদ্যমান variable `y`-কে shadow করে">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-11/src/main.rs:here}}
```

</Listing>

`match` expression চলার সময় কী ঘটে তা একে একে দেখা যাক। প্রথম match arm-এর pattern `x`-এর সংজ্ঞায়িত মানের সঙ্গে মেলে না, তাই কোড এগিয়ে যায়।

দ্বিতীয় match arm-এর pattern `y` নামের একটি নতুন variable আনে, যা একটি `Some` মানের ভেতরের যেকোনো মানের সঙ্গে মিলবে। আমরা `match` expression-এর ভেতরে একটি নতুন scope-এ আছি বলে এটি একটি নতুন
`y` variable, শুরুতে `10` মানসহ ঘোষণা করা `y` নয়। এই নতুন `y` binding একটি `Some`-এর ভেতরের যেকোনো মানের সঙ্গে মিলবে, আর `x`-এ আমাদের ঠিক তা-ই আছে। তাই এই নতুন `y` `x`-এর `Some`-এর ভেতরের
মানের সঙ্গে bind হয়। সেই মান `5`, তাই সেই arm-এর expression চলে এবং print করে `Matched, y = 5`।

`x` যদি `Some(5)`-এর বদলে একটি `None` মান হতো, তাহলে প্রথম দুটি arm-এর pattern মিলত না, তাই মানটি underscore-এর সঙ্গে মিলত। underscore arm-এর pattern-এ আমরা `x` variable আনিনি, তাই expression-এর `x`
এখনো বাইরের সেই `x`, যা shadow হয়নি। এই কাল্পনিক ক্ষেত্রে `match` print করত `Default case,
x = None`।

`match` expression শেষ হলে এর scope শেষ হয়, সঙ্গে ভেতরের `y`-এর scope-ও। শেষ `println!` তৈরি করে `at the end: x = Some(5), y = 10`।

বিদ্যমান `y` variable-কে shadow করে এমন নতুন variable না এনে বাইরের `x` আর `y`-এর মান তুলনা করে এমন একটি `match` expression তৈরি করতে আমাদের বরং একটি match guard শর্ত ব্যবহার করতে হতো। match guard
নিয়ে পরে [“match guard দিয়ে শর্ত যোগ
করা”](#adding-conditionals-with-match-guards)<!-- ignore --> অংশে কথা বলব।

<!-- Old headings. Do not remove or links may break. -->
<a id="multiple-patterns"></a>

### একাধিক pattern-এর সঙ্গে মেলানো {#matching-multiple-patterns}

`match` expression-এ `|` syntax ব্যবহার করে একাধিক pattern-এর সঙ্গে মেলানো যায়, যা pattern-এর _or_ (অথবা) operator। যেমন, নিচের কোডে আমরা `x`-এর মান match arm-গুলোর সঙ্গে মেলাই, যার প্রথমটিতে একটি
_or_ বিকল্প আছে, অর্থাৎ `x`-এর মান সেই arm-এর যেকোনো একটি মানের সঙ্গে মিললে সেই arm-এর কোড চলবে:


```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

এই কোড print করে `one or two`।

### `..=` দিয়ে মানের range-এর সঙ্গে মেলানো {#matching-ranges-of-values-with-}

`..=` syntax দিয়ে মানের একটি অন্তর্ভুক্তিমূলক (দুই প্রান্তসহ) range-এর সঙ্গে মেলানো যায়। নিচের কোডে একটি pattern প্রদত্ত range-এর যেকোনো মানের সঙ্গে মিললে সেই arm চলবে:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

`x` যদি `1`, `2`, `3`, `4` বা `5` হয়, তাহলে প্রথম arm মিলবে। একই ধারণা প্রকাশে `|` operator ব্যবহারের চেয়ে একাধিক মানের জন্য এই syntax বেশি সুবিধাজনক; `|` ব্যবহার করলে আমাদের লিখতে হতো `1 | 2 |
3 | 4 | 5`। একটি range উল্লেখ করা অনেক সংক্ষিপ্ত, বিশেষ করে যদি, ধরুন, 1 থেকে 1,000-এর মধ্যের যেকোনো সংখ্যার সঙ্গে মেলাতে চাই!

compiler compile time-এ যাচাই করে যে range খালি নয়, আর একটি range খালি কি না তা Rust শুধু `char` আর সংখ্যার মানের ক্ষেত্রেই বলতে পারে বলে range শুধু সংখ্যা বা `char` মানের সঙ্গেই অনুমোদিত।

এখানে `char` মানের range ব্যবহারের একটি উদাহরণ:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

Rust বলতে পারে যে `'c'` প্রথম pattern-এর range-এর ভেতরে, এবং print করে `early
ASCII letter`।

### মান ভেঙে নিতে destructuring {#destructuring-to-break-apart-values}

struct, enum আর tuple-এর বিভিন্ন অংশ ব্যবহার করতে pattern দিয়ে এগুলো destructure করাও যায়। চলুন প্রতিটি একে একে দেখি।

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-structs"></a>

#### struct {#structs}

Listing 19-12-এ `x` আর `y` দুটি field-যুক্ত একটি `Point` struct দেখানো হয়েছে, যা একটি `let` statement-এর সঙ্গে pattern ব্যবহার করে ভেঙে নেওয়া যায়।

<Listing number="19-12" file-name="src/main.rs" caption="একটি struct-এর field-গুলোকে আলাদা variable-এ destructure করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-12/src/main.rs}}
```

</Listing>

এই কোড `a` আর `b` variable তৈরি করে, যা `p` struct-এর `x` আর `y` field-এর মানের সঙ্গে মেলে। এই উদাহরণ দেখায় যে pattern-এর variable-গুলোর নাম struct-এর field-এর নামের সঙ্গে মিলতে হবে এমন
নয়। তবে কোন variable কোন field থেকে এসেছে তা মনে রাখা সহজ করতে variable-এর নাম field-এর নামের সঙ্গে মেলানোই প্রচলিত। এই প্রচলিত ব্যবহারের কারণে, আর `let Point { x: x, y: y } = p;` লেখায় অনেক
পুনরাবৃত্তি থাকে বলে, struct-এর field-এর সঙ্গে মেলা pattern-এর জন্য Rust-এ একটি সংক্ষিপ্ত রূপ আছে: শুধু struct field-এর নাম লিখলেই চলে, আর pattern থেকে তৈরি variable-গুলোর নামও একই হবে। Listing
19-13 Listing 19-12-এর কোডের মতোই আচরণ করে, তবে `let` pattern-এ তৈরি variable-গুলো `a` আর `b`-এর বদলে `x` আর `y`।

<Listing number="19-13" file-name="src/main.rs" caption="struct field-এর সংক্ষিপ্ত রূপ ব্যবহার করে struct-এর field destructure করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-13/src/main.rs}}
```

</Listing>

এই কোড `x` আর `y` variable তৈরি করে, যা `p` variable-এর `x` আর `y` field-এর সঙ্গে মেলে। ফলে `x` আর `y` variable-এ `p` struct-এর মানগুলো থাকে।

সব field-এর জন্য variable তৈরি না করে struct pattern-এর অংশ হিসেবে literal মান দিয়েও destructure করা যায়। এতে কিছু field নির্দিষ্ট মানের জন্য যাচাই করার পাশাপাশি অন্য field-গুলো destructure
করতে variable তৈরি করা যায়।

Listing 19-14-এ আমাদের একটি `match` expression আছে, যা `Point` মানগুলোকে তিনটি ক্ষেত্রে ভাগ করে: যে বিন্দু সরাসরি `x` অক্ষের ওপর আছে (`y = 0` হলে যা সত্য), `y` অক্ষের ওপর (`x = 0`), বা কোনো অক্ষের
ওপরই নয়।

<Listing number="19-14" file-name="src/main.rs" caption="একটি pattern-এ destructure করা এবং literal মানের সঙ্গে মেলানো">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-14/src/main.rs:here}}
```

</Listing>

প্রথম arm `x` অক্ষের ওপর থাকা যেকোনো বিন্দুর সঙ্গে মিলবে, কারণ এটি বলে দেয় যে `y` field মিলবে যদি এর মান literal `0`-এর সঙ্গে মেলে। pattern-টি তবুও একটি `x` variable তৈরি করে, যা এই arm-এর কোডে
ব্যবহার করা যায়।

একইভাবে দ্বিতীয় arm `y` অক্ষের ওপর থাকা যেকোনো বিন্দুর সঙ্গে মেলে, কারণ এটি বলে দেয় যে `x` field মিলবে যদি এর মান `0` হয়, এবং `y` field-এর মানের জন্য একটি `y` variable তৈরি করে। তৃতীয় arm কোনো
literal উল্লেখ করে না, তাই এটি অন্য যেকোনো `Point`-এর সঙ্গে মেলে এবং `x` আর `y` দুটি field-এর জন্যই variable তৈরি করে।

এই উদাহরণে `x`-এ একটি `0` থাকায় `p` মান দ্বিতীয় arm-এর সঙ্গে মেলে, তাই এই কোড print করবে `On the y axis at 7`।

মনে রাখবেন, প্রথম মিলে যাওয়া pattern পেলেই একটি `match` expression arm যাচাই করা বন্ধ করে, তাই `Point { x: 0, y: 0 }` `x` অক্ষ আর `y` অক্ষ দুটির ওপর থাকলেও এই কোড শুধু print করত `On the x axis at 0`।

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-enums"></a>

#### enum {#enums}

এই বইয়ে আমরা enum destructure করেছি (যেমন অধ্যায় 6-এর Listing 6-5), কিন্তু এখনো স্পষ্টভাবে আলোচনা করিনি যে একটি enum destructure করার pattern enum-এর ভেতরে রাখা ডেটা যেভাবে সংজ্ঞায়িত, তার সঙ্গে মেলে।
উদাহরণ হিসেবে Listing 19-15-এ আমরা Listing 6-2-এর `Message` enum ব্যবহার করি এবং এমন pattern-সহ একটি `match` লিখি, যা প্রতিটি ভেতরের মান destructure করবে।

<Listing number="19-15" file-name="src/main.rs" caption="ভিন্ন ভিন্ন ধরনের মান রাখা enum variant destructure করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-15/src/main.rs}}
```

</Listing>

এই কোড print করবে `Change color to red 0, green 160, and blue 255`। অন্য arm-গুলোর কোড চলতে দেখতে `msg`-এর মান বদলে দেখুন।

`Message::Quit`-এর মতো কোনো ডেটা ছাড়া enum variant-এর ক্ষেত্রে মানটি আর destructure করা যায় না। শুধু literal `Message::Quit` মানের সঙ্গে match করা যায়, আর সেই pattern-এ কোনো variable নেই।

`Message::Move`-এর মতো struct-সদৃশ enum variant-এর ক্ষেত্রে struct মেলাতে যে pattern দিই তার মতো pattern ব্যবহার করা যায়। variant-এর নামের পরে curly bracket দিয়ে variable-সহ field-গুলো লিখি, যাতে
এই arm-এর কোডে ব্যবহারের জন্য অংশগুলো ভেঙে নেওয়া যায়। এখানে আমরা Listing 19-13-এর মতো সংক্ষিপ্ত রূপ ব্যবহার করি।

tuple-সদৃশ enum variant-এর ক্ষেত্রে, যেমন এক উপাদানের tuple রাখা `Message::Write` আর তিন উপাদানের tuple রাখা `Message::ChangeColor`, pattern-টি tuple মেলাতে যে pattern দিই তার মতো। pattern-এর variable-এর
সংখ্যা যে variant-এর সঙ্গে মেলাচ্ছি তার উপাদানের সংখ্যার সমান হতে হবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-nested-structs-and-enums"></a>

#### nested struct ও enum {#nested-structs-and-enums}

এ পর্যন্ত আমাদের সব উদাহরণ এক স্তর গভীরের struct বা enum মেলাচ্ছিল, তবে nested (একটির ভেতরে আরেকটি) item-এও matching কাজ করে! যেমন, Listing 19-15-এর কোড refactor করে `ChangeColor` message-এ
RGB আর HSV রং সমর্থন করানো যায়, যেমনটি Listing 19-16-এ দেখানো হয়েছে।

<Listing number="19-16" caption="nested enum-এর ওপর match করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-16/src/main.rs}}
```

</Listing>

`match` expression-এর প্রথম arm-এর pattern এমন একটি `Message::ChangeColor` enum variant-এর সঙ্গে মেলে, যাতে একটি `Color::Rgb` variant আছে; তারপর pattern-টি ভেতরের তিনটি `i32` মানের সঙ্গে bind হয়।
দ্বিতীয় arm-এর pattern-ও একটি `Message::ChangeColor` enum variant-এর সঙ্গে মেলে, তবে ভেতরের enum বরং `Color::Hsv`-এর সঙ্গে মেলে। দুটি enum জড়িত থাকলেও এই জটিল শর্তগুলো একটি `match` expression-এ উল্লেখ
করা যায়।

<!-- Old headings. Do not remove or links may break. -->

<a id="destructuring-structs-and-tuples"></a>

#### struct ও tuple {#structs-and-tuples}

destructuring pattern-গুলো আরও জটিলভাবে মেশানো, মেলানো আর একটির ভেতরে আরেকটি রাখা যায়। নিচের উদাহরণে একটি জটিল destructure দেখানো হয়েছে, যেখানে একটি tuple-এর ভেতরে struct আর tuple রেখে সব
primitive মান বের করে আনা হয়েছে:

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

এই কোড জটিল type-কে তাদের উপাদান অংশে ভেঙে নিতে দেয়, যাতে আমাদের আগ্রহের মানগুলো আলাদাভাবে ব্যবহার করা যায়।

pattern দিয়ে destructure করা মানের অংশগুলো, যেমন একটি struct-এর প্রতিটি field-এর মান, একে অপর থেকে আলাদাভাবে ব্যবহারের একটি সুবিধাজনক উপায়।

### pattern-এ মান উপেক্ষা করা {#ignoring-values-in-a-pattern}

আপনি দেখেছেন যে কখনো কখনো pattern-এ মান উপেক্ষা করা কাজে লাগে, যেমন একটি `match`-এর শেষ arm-এ, এমন একটি সব-ধরা arm পেতে যা আসলে কিছু করে না, কিন্তু বাকি সব সম্ভাব্য মান বিবেচনায় আনে। pattern-এ
পুরো মান বা মানের অংশ উপেক্ষা করার কয়েকটি উপায় আছে: `_` pattern ব্যবহার (যা আপনি দেখেছেন), আরেকটি pattern-এর ভেতরে `_` pattern ব্যবহার, underscore দিয়ে শুরু হওয়া একটি নাম ব্যবহার, অথবা মানের
বাকি অংশ উপেক্ষা করতে `..` ব্যবহার। চলুন দেখি এই pattern-গুলোর প্রতিটি কীভাবে ও কেন ব্যবহার করতে হয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-an-entire-value-with-_"></a>

#### `_` দিয়ে পুরো মান {#an-entire-value-with-_}

underscore-কে আমরা একটি wildcard pattern হিসেবে ব্যবহার করেছি, যা যেকোনো মানের সঙ্গে মেলে কিন্তু মানের সঙ্গে bind হয় না। এটি বিশেষ করে একটি `match` expression-এর শেষ arm হিসেবে কাজে লাগে, তবে
function parameter-সহ যেকোনো pattern-এও ব্যবহার করা যায়, যেমনটি Listing 19-17-এ দেখানো হয়েছে।

<Listing number="19-17" file-name="src/main.rs" caption="একটি function signature-এ `_` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-17/src/main.rs}}
```

</Listing>

এই কোড প্রথম argument হিসেবে পাঠানো `3` মানটি পুরোপুরি উপেক্ষা করবে, এবং print করবে `This code only uses the y parameter: 4`।

বেশিরভাগ ক্ষেত্রে কোনো নির্দিষ্ট function parameter আর দরকার না হলে আপনি signature বদলে অব্যবহৃত parameter-টি সরিয়ে দেবেন। function parameter উপেক্ষা করা বিশেষ করে কাজে লাগে যখন, যেমন,
আপনি একটি trait implement করছেন, যেখানে একটি নির্দিষ্ট type signature দরকার, কিন্তু আপনার implementation-এর function body-র একটি parameter দরকার নেই। তখন অব্যবহৃত function parameter নিয়ে compiler
warning পাওয়া এড়াতে পারেন, যা নাম ব্যবহার করলে পেতেন।

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-parts-of-a-value-with-a-nested-_"></a>

#### nested `_` দিয়ে মানের অংশ {#parts-of-a-value-with-a-nested-_}

মানের শুধু একটি অংশ উপেক্ষা করতে আরেকটি pattern-এর ভেতরেও `_` ব্যবহার করা যায়, যেমন যখন মানের শুধু একটি অংশ যাচাই করতে চাই, কিন্তু চালাতে চাওয়া সংশ্লিষ্ট কোডে অন্য অংশগুলোর কোনো দরকার নেই।
Listing 19-18-এ একটি setting-এর মান সামলানোর দায়িত্বে থাকা কোড দেখানো হয়েছে। ব্যবসায়িক চাহিদা হলো, ব্যবহারকারীকে একটি setting-এর বিদ্যমান নিজস্ব মান overwrite করতে দেওয়া উচিত নয়, তবে
setting-টি এখন সেট করা না থাকলে তা সেট করতে ও মান দিতে পারবে।

<Listing number="19-18" caption="`Some`-এর ভেতরের মান ব্যবহারের দরকার না থাকলে `Some` variant-এর সঙ্গে মেলা pattern-এর ভেতরে একটি underscore ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-18/src/main.rs:here}}
```

</Listing>

এই কোড print করবে `Can't overwrite an existing customized value` আর তারপর
`setting is Some(5)`। প্রথম match arm-এ কোনো `Some` variant-এর ভেতরের মানের সঙ্গে match করার বা সেগুলো ব্যবহারের দরকার নেই, তবে `setting_value` আর `new_setting_value` যখন `Some` variant, সেই
ক্ষেত্রটি যাচাই করতে হবে। সেই ক্ষেত্রে আমরা `setting_value` না বদলানোর কারণ print করি, আর এটি বদলায় না।

দ্বিতীয় arm-এর `_` pattern দিয়ে প্রকাশ করা বাকি সব ক্ষেত্রে (যদি `setting_value` বা `new_setting_value`, যেকোনোটি `None` হয়) আমরা
`setting_value`-কে `new_setting_value`-তে সেট হতে দিতে চাই।

নির্দিষ্ট কিছু মান উপেক্ষা করতে একটি pattern-এর ভেতরে একাধিক জায়গায় underscore ব্যবহার করা যায়। Listing 19-19-এ পাঁচটি item-এর একটি tuple-এর দ্বিতীয় আর চতুর্থ মান উপেক্ষা করার একটি উদাহরণ
দেখানো হয়েছে।

<Listing number="19-19" caption="একটি tuple-এর একাধিক অংশ উপেক্ষা করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-19/src/main.rs:here}}
```

</Listing>

এই কোড print করবে `Some numbers: 2, 8, 32`, আর `4` ও `16` মান উপেক্ষিত হবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-an-unused-variable-by-starting-its-name-with-_"></a>

#### নাম `_` দিয়ে শুরু করে একটি অব্যবহৃত variable {#an-unused-variable-by-starting-its-name-with-_}

একটি variable তৈরি করে কোথাও ব্যবহার না করলে Rust সাধারণত একটি warning দেয়, কারণ অব্যবহৃত variable একটি বাগ হতে পারে। তবে কখনো কখনো এমন একটি variable তৈরি করতে পারা কাজে লাগে যা এখনো ব্যবহার
করবেন না, যেমন যখন prototype বানাচ্ছেন বা সবে একটি project শুরু করছেন। এই পরিস্থিতিতে variable-এর নাম একটি underscore দিয়ে শুরু করে Rust-কে বলতে পারেন অব্যবহৃত variable নিয়ে warning না দিতে।
Listing 19-20-এ আমরা দুটি অব্যবহৃত variable তৈরি করি, কিন্তু এই কোড compile করলে শুধু একটি নিয়ে warning পাওয়ার কথা।

<Listing number="19-20" file-name="src/main.rs" caption="অব্যবহৃত variable-এর warning এড়াতে variable-এর নাম একটি underscore দিয়ে শুরু করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-20/src/main.rs}}
```

</Listing>

এখানে `y` variable ব্যবহার না করা নিয়ে একটি warning পাই, কিন্তু `_x` ব্যবহার না করা নিয়ে কোনো warning পাই না।

খেয়াল করুন, শুধু `_` ব্যবহার আর underscore দিয়ে শুরু হওয়া একটি নাম ব্যবহারের মধ্যে একটি সূক্ষ্ম পার্থক্য আছে। `_x` syntax তবুও মানটিকে variable-এর সঙ্গে bind করে, যেখানে `_` মোটেই bind করে না। এই
পার্থক্য যেখানে গুরুত্বপূর্ণ তা দেখাতে Listing 19-21 আমাদের একটি error দেবে।

<Listing number="19-21" caption="underscore দিয়ে শুরু হওয়া একটি অব্যবহৃত variable তবুও মানটি bind করে, যা মানটির ownership নিয়ে নিতে পারে।">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-21/src/main.rs:here}}
```

</Listing>

আমরা একটি error পাব, কারণ `s` মান তবুও `_s`-এ সরে যাবে, যা আমাদের আবার `s` ব্যবহার করতে দেয় না। তবে শুধু underscore ব্যবহার করলে তা কখনো মানের সঙ্গে bind হয় না। Listing 19-22 কোনো error ছাড়াই
compile হবে, কারণ `s` `_`-এ সরে যায় না।

<Listing number="19-22" caption="underscore ব্যবহার করলে মান bind হয় না।">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-22/src/main.rs:here}}
```

</Listing>

এই কোড ঠিকঠাক কাজ করে, কারণ আমরা কখনো `s`-কে কোনো কিছুর সঙ্গে bind করি না; এটি সরে যায় না।

<a id="ignoring-remaining-parts-of-a-value-with-"></a>

#### `..` দিয়ে মানের বাকি অংশ {#remaining-parts-of-a-value-with-}

অনেক অংশযুক্ত মানের ক্ষেত্রে `..` syntax ব্যবহার করে নির্দিষ্ট অংশগুলো ব্যবহার করে বাকিগুলো উপেক্ষা করা যায়, ফলে প্রতিটি উপেক্ষিত মানের জন্য underscore লিখতে হয় না। `..` pattern মানের সেই সব অংশ
উপেক্ষা করে, যা pattern-এর বাকি অংশে আমরা স্পষ্টভাবে মেলাইনি। Listing 19-23-এ আমাদের একটি `Point` struct আছে, যা ত্রিমাত্রিক স্থানে একটি স্থানাঙ্ক রাখে। `match` expression-এ আমরা শুধু `x` স্থানাঙ্কের
ওপর কাজ করতে চাই এবং `y` আর `z` field-এর মান উপেক্ষা করতে চাই।

<Listing number="19-23" caption="`..` ব্যবহার করে `x` ছাড়া একটি `Point`-এর সব field উপেক্ষা করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-23/src/main.rs:here}}
```

</Listing>

আমরা `x` মান লিখি, তারপর শুধু `..` pattern যোগ করি। এটি `y: _` আর `z: _` লেখার চেয়ে দ্রুত, বিশেষ করে যখন অনেক field-যুক্ত struct নিয়ে এমন পরিস্থিতিতে কাজ করছি যেখানে মাত্র এক-দুটি field
প্রাসঙ্গিক।

`..` syntax যতগুলো মান দরকার ততগুলোতে বিস্তৃত হবে। Listing 19-24 দেখায় একটি tuple-এর সঙ্গে কীভাবে `..` ব্যবহার করতে হয়।

<Listing number="19-24" file-name="src/main.rs" caption="একটি tuple-এর শুধু প্রথম আর শেষ মানের সঙ্গে মেলানো এবং বাকি সব মান উপেক্ষা করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-24/src/main.rs}}
```

</Listing>

এই কোডে প্রথম আর শেষ মান `first` আর `last`-এর সঙ্গে মেলানো হয়। `..` মাঝখানের সবকিছুর সঙ্গে মিলবে এবং উপেক্ষা করবে।

তবে `..`-এর ব্যবহার দ্ব্যর্থহীন হতে হবে। কোন মানগুলো মেলানোর জন্য আর কোনগুলো উপেক্ষা করার জন্য তা অস্পষ্ট হলে Rust আমাদের একটি error দেবে। Listing 19-25-এ দ্ব্যর্থকভাবে `..` ব্যবহারের একটি উদাহরণ
দেখানো হয়েছে, তাই এটি compile হবে না।

<Listing number="19-25" file-name="src/main.rs" caption="দ্ব্যর্থকভাবে `..` ব্যবহারের একটি চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-25/src/main.rs}}
```

</Listing>

এই উদাহরণ compile করলে এই error পাই:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-25/output.txt}}
```

`second`-এর সঙ্গে একটি মান মেলানোর আগে tuple-এর কতগুলো মান উপেক্ষা করতে হবে, আর তারপর আরও কতগুলো মান উপেক্ষা করতে হবে, তা নির্ধারণ করা Rust-এর পক্ষে অসম্ভব। এই কোডের মানে হতে পারে আমরা `2`
উপেক্ষা করে `second`-কে `4`-এর সঙ্গে bind করতে, তারপর `8`, `16` আর `32` উপেক্ষা করতে চাই; অথবা `2` আর `4` উপেক্ষা করে `second`-কে `8`-এর সঙ্গে bind করতে, তারপর `16` আর `32` উপেক্ষা করতে চাই;
ইত্যাদি। `second` variable নামটি Rust-এর কাছে বিশেষ কিছু বোঝায় না, তাই আমরা compiler error পাই, কারণ এভাবে দুই জায়গায় `..` ব্যবহার দ্ব্যর্থক।

<!-- Old headings. Do not remove or links may break. -->

<a id="extra-conditionals-with-match-guards"></a>

### match guard দিয়ে শর্ত যোগ করা {#adding-conditionals-with-match-guards}

একটি _match guard_ হলো একটি `match` arm-এর pattern-এর পরে উল্লেখ করা একটি বাড়তি `if` শর্ত, যা সেই arm বাছাই হতে হলে মিলতেই হবে। শুধু একটি pattern যতটা প্রকাশ করতে দেয় তার চেয়ে জটিল ধারণা প্রকাশে match guard
কাজে লাগে। তবে খেয়াল করুন, এগুলো শুধু `match` expression-এ পাওয়া যায়, `if let` বা
`while let` expression-এ নয়।

শর্তটি pattern-এ তৈরি variable ব্যবহার করতে পারে। Listing 19-26-এ একটি `match` দেখানো হয়েছে, যার প্রথম arm-এর pattern `Some(x)` এবং এতে `if x % 2 == 0`-এর একটি match guard-ও আছে (সংখ্যাটি জোড়
হলে যা `true` হবে)।

<Listing number="19-26" caption="একটি pattern-এ match guard যোগ করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-26/src/main.rs:here}}
```

</Listing>

এই উদাহরণ print করবে `The number 4 is even`। `num`-কে প্রথম arm-এর pattern-এর সঙ্গে তুলনা করলে এটি মেলে, কারণ `Some(4)` `Some(x)`-এর সঙ্গে মেলে। তারপর match guard যাচাই করে `x`-কে 2 দিয়ে ভাগ
করলে ভাগশেষ 0-এর সমান কি না, আর যেহেতু তা-ই, তাই প্রথম arm বাছাই হয়।

`num` যদি বরং `Some(5)` হতো, তাহলে প্রথম arm-এর match guard `false` হতো, কারণ 5-কে 2 দিয়ে ভাগ করলে ভাগশেষ 1, যা 0-এর সমান নয়। তখন Rust দ্বিতীয় arm-এ যেত, যা মিলত, কারণ দ্বিতীয় arm-এর কোনো match guard নেই
এবং তাই এটি যেকোনো `Some` variant-এর সঙ্গে মেলে।

`if x % 2 == 0` শর্তটি একটি pattern-এর ভেতরে প্রকাশ করার কোনো উপায় নেই, তাই match guard আমাদের এই logic প্রকাশের ক্ষমতা দেয়। এই বাড়তি প্রকাশক্ষমতার অসুবিধা হলো, match guard expression জড়িত থাকলে compiler
exhaustiveness যাচাই করার চেষ্টা করে না।

Listing 19-11 নিয়ে আলোচনার সময় আমরা বলেছিলাম যে pattern-এর shadowing সমস্যা সমাধানে match guard ব্যবহার করা যায়। মনে করুন, `match`-এর বাইরের variable ব্যবহার না করে আমরা `match` expression-এর
pattern-এর ভেতরে একটি নতুন variable তৈরি করেছিলাম। সেই নতুন variable-এর কারণে বাইরের variable-এর মানের সঙ্গে যাচাই করতে পারছিলাম না। Listing 19-27 দেখায় এই সমস্যা ঠিক করতে কীভাবে একটি match guard
ব্যবহার করা যায়।

<Listing number="19-27" file-name="src/main.rs" caption="বাইরের একটি variable-এর সঙ্গে সমতা যাচাই করতে একটি match guard ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-27/src/main.rs}}
```

</Listing>

এই কোড এখন print করবে `Default case, x = Some(5)`। দ্বিতীয় match arm-এর pattern এমন কোনো নতুন variable `y` আনে না যা বাইরের `y`-কে shadow করত, অর্থাৎ match guard-এ বাইরের `y` ব্যবহার করা যায়।
pattern হিসেবে `Some(y)` উল্লেখ না করে, যা বাইরের `y`-কে shadow করত, আমরা উল্লেখ করি `Some(n)`। এটি একটি নতুন variable `n` তৈরি করে, যা কিছুই shadow করে না, কারণ `match`-এর বাইরে কোনো `n` variable
নেই।

match guard `if n == y` কোনো pattern নয় এবং তাই নতুন variable আনে না। এই `y` _সত্যিই_ বাইরের `y`, একে shadow করা নতুন কোনো `y` নয়, আর `n`-কে `y`-এর সঙ্গে তুলনা করে আমরা বাইরের `y`-এর সমান মান খুঁজতে
পারি।

একাধিক pattern উল্লেখ করতে একটি match guard-এ _or_ operator `|`-ও ব্যবহার করা যায়; match guard-এর শর্তটি সব pattern-এ প্রযোজ্য হবে। Listing
19-28 দেখায় `|` ব্যবহার করা একটি pattern-কে একটি match guard-এর সঙ্গে মেলালে অগ্রাধিকার কেমন হয়। এই উদাহরণের গুরুত্বপূর্ণ অংশ হলো, `if y` match guard `4`, `5` _এবং_ `6`-এ প্রযোজ্য, যদিও দেখে মনে
হতে পারে `if y` শুধু `6`-এ প্রযোজ্য।

<Listing number="19-28" caption="একটি match guard-এর সঙ্গে একাধিক pattern মেলানো">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-28/src/main.rs:here}}
```

</Listing>

match-এর শর্ত বলে যে arm-টি কেবল তখনই মিলবে, যখন `x`-এর মান `4`, `5` বা `6`-এর সমান _এবং_ `y` হলো `true`। এই কোড চলার সময় প্রথম arm-এর pattern মেলে, কারণ `x` হলো `4`, কিন্তু match guard `if y`
হলো `false`, তাই প্রথম arm বাছাই হয় না। কোড দ্বিতীয় arm-এ যায়, যা মেলে, এবং এই প্রোগ্রাম print করে `no`। কারণ `if` শর্তটি শুধু শেষ মান `6`-এ নয়, পুরো pattern `4 | 5 | 6`-এ প্রযোজ্য। অন্যভাবে বললে,
একটি pattern-এর তুলনায় match guard-এর অগ্রাধিকার এমন আচরণ করে:

```text
(4 | 5 | 6) if y => ...
```

এমন নয়:

```text
4 | 5 | (6 if y) => ...
```

কোড চালানোর পর অগ্রাধিকারের আচরণ স্পষ্ট: match guard যদি শুধু `|` operator দিয়ে উল্লেখ করা মানের তালিকার শেষ মানে প্রযোজ্য হতো, তাহলে arm-টি মিলত, আর প্রোগ্রাম print করত
`yes`।

<!-- Old headings. Do not remove or links may break. -->

<a id="-bindings"></a>

### `@` binding ব্যবহার করা {#using--bindings}

_at_ operator `@` দিয়ে এমন একটি variable তৈরি করা যায়, যা একটি মান রাখে, আর একই সময়ে আমরা সেই মান একটি pattern-এর সঙ্গে মেলে কি না যাচাই করি। Listing 19-29-এ আমরা যাচাই করতে চাই যে একটি
`Message::Hello`-এর `id` field `3..=7` range-এর ভেতরে আছে কি না। সঙ্গে মানটিকে `id` variable-এর সঙ্গে bind করতেও চাই, যাতে arm-এর সঙ্গে যুক্ত কোডে এটি ব্যবহার করতে পারি।

<Listing number="19-29" caption="একটি pattern-এ একটি মান যাচাই করার পাশাপাশি তার সঙ্গে bind করতে `@` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-29/src/main.rs:here}}
```

</Listing>

এই উদাহরণ print করবে `Found an id in range: 5`। `3..=7` range-এর আগে `id @` উল্লেখ করে আমরা range-এর সঙ্গে মিলে যাওয়া মানটি `id` নামের একটি variable-এ capture করছি, পাশাপাশি যাচাই করছি যে মানটি
range pattern-এর সঙ্গে মিলেছে।

দ্বিতীয় arm-এ, যেখানে pattern-এ শুধু একটি range উল্লেখ করা আছে, arm-এর সঙ্গে যুক্ত কোডে এমন কোনো variable নেই, যাতে `id` field-এর আসল মান আছে। `id` field-এর মান 10, 11 বা 12 হতে পারত, কিন্তু
সেই pattern-এর সঙ্গে যাওয়া কোড জানে না কোনটি। pattern-এর কোড `id` field-এর মান ব্যবহার করতে পারে না, কারণ আমরা `id` মান একটি variable-এ রাখিনি।

শেষ arm-এ, যেখানে range ছাড়া একটি variable উল্লেখ করেছি, সেখানে arm-এর কোডে ব্যবহারের জন্য `id` নামের একটি variable-এ মানটি পাওয়া যায়। কারণ আমরা struct field-এর সংক্ষিপ্ত syntax ব্যবহার করেছি।
কিন্তু প্রথম দুটি arm-এর মতো এই arm-এ `id` field-এর মানের ওপর কোনো যাচাই প্রয়োগ করিনি: যেকোনো মান এই pattern-এর সঙ্গে মিলবে।

`@` ব্যবহার করে একটি pattern-এর ভেতরেই একটি মান যাচাই করে একটি variable-এ রেখে দেওয়া যায়।

{{#quiz ../quizzes/ch18-03-pattern-syntax.toml}}

## সারসংক্ষেপ {#summary}

বিভিন্ন ধরনের ডেটার মধ্যে পার্থক্য করতে Rust-এর pattern খুবই কাজের। `match` expression-এ ব্যবহার করলে Rust নিশ্চিত করে যে আপনার pattern প্রতিটি সম্ভাব্য মান ঢেকে দেয়, নইলে প্রোগ্রাম compile হবে না।
`let` statement আর function parameter-এর pattern এই গঠনগুলোকে আরও কাজের করে, মানকে ছোট ছোট অংশে destructure করে সেই অংশগুলো variable-এ বসাতে দেয়। প্রয়োজন অনুযায়ী আমরা সরল বা জটিল pattern
তৈরি করতে পারি।

এরপর, বইয়ের শেষের আগের অধ্যায়ে আমরা Rust-এর নানা বৈশিষ্ট্যের কিছু উন্নত দিক দেখব।

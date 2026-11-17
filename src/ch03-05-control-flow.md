## control flow {#control-flow}

কোনো শর্ত `true` কি না তার ওপর নির্ভর করে কিছু কোড চালানোর ক্ষমতা, আর কোনো শর্ত
`true` থাকা পর্যন্ত কিছু কোড বারবার চালানোর ক্ষমতা—এ দুটি বেশিরভাগ প্রোগ্রামিং ভাষার
মৌলিক ভিত্তি। Rust কোড চলার ক্রম নিয়ন্ত্রণ করার সবচেয়ে প্রচলিত গঠনগুলো হলো `if`
expression আর loop (বারবার চলতে থাকা কোড-অংশ)।

### `if` expression {#if-expressions}

একটি `if` expression শর্তের ওপর নির্ভর করে আপনার কোডকে আলাদা আলাদা পথে চালাতে দেয়।
আপনি একটি শর্ত দেন, তারপর বলেন, “এই শর্ত পূরণ হলে কোডের এই block চালাও। শর্ত পূরণ না
হলে কোডের এই block চালিও না।”

`if` expression খুঁটিয়ে দেখতে আপনার _projects_ ডিরেক্টরিতে _branches_ নামে একটি
নতুন প্রজেক্ট তৈরি করুন। _src/main.rs_ ফাইলে নিচের কোডটি লিখুন:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

সব `if` expression শুরু হয় `if` keyword দিয়ে, তারপর থাকে একটি শর্ত। এই ক্ষেত্রে শর্তটি
যাচাই করে `number` variable-এর মান 5-এর চেয়ে কম কি না। শর্ত `true` হলে যে কোডের block
চলবে, সেটি আমরা শর্তের ঠিক পরেই কার্লি ব্র্যাকেটের ভেতরে বসাই। `if` expression-এর
শর্তগুলোর সঙ্গে যুক্ত কোডের block-গুলোকে কখনো কখনো _arm_ (শাখা)
বলা হয়, ঠিক `match` expression-এর arm-এর মতো, যা নিয়ে আমরা অধ্যায় 2-এর [“আন্দাজের সঙ্গে
গোপন সংখ্যা মেলানো”][comparing-the-guess-to-the-secret-number]<!--
ignore --> অংশে আলোচনা করেছিলাম।

ঐচ্ছিকভাবে আমরা একটি `else` expression-ও যোগ করতে পারি, যা এখানে আমরা করেছি; শর্তটি
`false` evaluate হলে প্রোগ্রাম যাতে বিকল্প একটি কোডের block চালাতে পারে, সেজন্য। আপনি
`else` expression না দিলে এবং শর্তটি `false` হলে প্রোগ্রাম শুধু `if` block-টি এড়িয়ে
পরের কোডে চলে যাবে।

কোডটি চালিয়ে দেখুন; নিচের আউটপুট দেখার কথা:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

চলুন `number`-এর মান বদলে এমন একটি মান দিই, যাতে শর্তটি `false` হয়, আর দেখি কী
ঘটে:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

প্রোগ্রামটি আবার চালান, আর আউটপুটটি দেখুন:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

এটাও উল্লেখ করার মতো যে এই কোডের শর্তটি _অবশ্যই_ একটি `bool` হতে হবে। শর্তটি `bool`
না হলে error পাব। যেমন, নিচের কোডটি চালিয়ে দেখুন:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

এবার `if` শর্তটি evaluate হয়ে `3` মান দেয়, আর Rust একটি error দেখায়:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

error-টি বোঝায় যে Rust একটি `bool` আশা করেছিল, কিন্তু পেয়েছে একটি integer। Ruby বা
JavaScript-এর মতো ভাষার বিপরীতে Rust নিজে থেকে Boolean নয় এমন type-কে Boolean-এ
রূপান্তরের চেষ্টা করবে না। আপনাকে স্পষ্ট হতে হবে এবং সবসময় `if`-কে শর্ত হিসেবে একটি
Boolean দিতে হবে। যেমন, শুধু কোনো সংখ্যা `0`-এর সমান না হলে `if` কোড block চালাতে চাইলে
`if` expression-টি এভাবে বদলাতে পারি:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

এই কোড চালালে প্রিন্ট হবে `number was something other than zero`।

#### `else if` দিয়ে একাধিক শর্ত সামলানো {#handling-multiple-conditions-with-else-if}

`else if` expression-এ `if` আর `else` মিলিয়ে একাধিক শর্ত ব্যবহার করতে পারেন। যেমন:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

এই প্রোগ্রাম চারটি সম্ভাব্য পথের যেকোনো একটিতে যেতে পারে। চালানোর পর নিচের আউটপুট
দেখার কথা:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

এই প্রোগ্রাম চলার সময় একে একে প্রতিটি `if` expression যাচাই করে এবং যার শর্ত প্রথম
`true` evaluate হয়, সেই body চালায়। লক্ষ করুন, 6 যদিও 2 দিয়ে বিভাজ্য, তবু আমরা
`number is divisible by 2` আউটপুট দেখি না, আবার `else` block-এর
`number is not divisible by 4, 3, or 2` লেখাটিও দেখি না। কারণ Rust শুধু প্রথম `true`
শর্তের block-টিই চালায়, আর একটি পেয়ে গেলে বাকিগুলো আর যাচাই-ও করে না।

খুব বেশি `else if` expression ব্যবহার করলে কোড এলোমেলো হয়ে যেতে পারে, তাই একটির বেশি
থাকলে হয়তো আপনার কোড refactor (আচরণ না বদলে কোড পুনর্গঠন) করতে চাইবেন। এসব ক্ষেত্রের
জন্য অধ্যায় 6-এ `match` নামের Rust-এর একটি শক্তিশালী শাখা-বিভাজন গঠন বর্ণনা করা হয়েছে।

#### `let` statement-এ `if` ব্যবহার {#using-if-in-a-let-statement}

`if` যেহেতু একটি expression, তাই ফলাফলটি একটি variable-এ assign করতে আমরা একে একটি
`let` statement-এর ডান পাশে ব্যবহার করতে পারি, Listing 3-2-এর মতো।

<Listing number="3-2" file-name="src/main.rs" caption="একটি `if` expression-এর ফলাফল একটি variable-এ assign করা">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

</Listing>

`if` expression-এর ফলাফলের ভিত্তিতে `number` variable একটি মানের সঙ্গে bind হবে। কী
ঘটে দেখতে কোডটি চালান:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

মনে রাখবেন, কোডের block evaluate হয় তার ভেতরের শেষ expression-এ, আর সংখ্যা নিজেও
একটি expression। এই ক্ষেত্রে পুরো `if` expression-এর মান নির্ভর করে কোন কোডের block
চলে তার ওপর। এর মানে, `if`-এর প্রতিটি arm থেকে যেসব মান ফলাফল হতে পারে, সেগুলোর type
একই হতে হবে; Listing 3-2-এ `if` arm আর `else` arm—দুটোরই ফলাফল ছিল `i32` integer।
নিচের উদাহরণের মতো type না মিললে error পাব:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

এই কোড compile করার চেষ্টা করলে error পাব। `if` আর `else` arm-এর মানের type
পরস্পরের সঙ্গে খাপ খায় না, আর Rust প্রোগ্রামের ঠিক কোথায় সমস্যাটি আছে তা দেখিয়ে দেয়:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`if` block-এর expression evaluate হয় একটি integer-এ, আর `else` block-এর expression
evaluate হয় একটি string-এ। এটি কাজ করবে না, কারণ variable-এর একটিমাত্র type থাকতে
হবে, আর `number` variable-টি কোন type-এর, তা Rust-কে compile time-এই নিশ্চিতভাবে জানতে
হবে। `number`-এর type জানা থাকলে compiler যাচাই করতে পারে যে আমরা যেখানেই `number`
ব্যবহার করি, সেখানেই type-টি বৈধ। `number`-এর type যদি শুধু runtime-এ নির্ধারিত হতো,
তাহলে Rust তা পারত না; যেকোনো variable-এর জন্য একাধিক সম্ভাব্য type-এর হিসাব রাখতে
হলে compiler আরও জটিল হতো এবং কোড সম্পর্কে কম নিশ্চয়তা দিতে পারত।

{{#quiz ../quizzes/ch03-05-control-flow-sec1-if.toml}}

### loop দিয়ে পুনরাবৃত্তি {#repetition-with-loops}

কোনো কোডের block একাধিকবার চালানো প্রায়ই কাজে লাগে। এই কাজের জন্য Rust কয়েক ধরনের
_loop_ দেয়, যা loop body-র ভেতরের কোড শেষ পর্যন্ত চালিয়ে আবার সঙ্গে সঙ্গে শুরু থেকে
চালাতে থাকে। loop নিয়ে পরীক্ষা-নিরীক্ষা করতে চলুন _loops_ নামে একটি নতুন প্রজেক্ট বানাই।

Rust-এ তিন ধরনের loop আছে: `loop`, `while` আর `for`। চলুন প্রতিটি চেষ্টা করে দেখি।

#### `loop` দিয়ে কোড বারবার চালানো {#repeating-code-with-loop}

`loop` keyword Rust-কে বলে একটি কোডের block বারবার চালাতে থাকতে—হয় চিরকাল, নয়তো
আপনি স্পষ্টভাবে থামতে না বলা পর্যন্ত।

উদাহরণ হিসেবে, আপনার _loops_ ডিরেক্টরির _src/main.rs_ ফাইলটি বদলে এরকম করুন:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

এই প্রোগ্রাম চালালে আমরা নিজ হাতে প্রোগ্রামটি না থামানো পর্যন্ত `again!` একটানা বারবার
প্রিন্ট হতে দেখব। বেশিরভাগ টার্মিনালে অবিরাম loop-এ আটকে থাকা কোনো প্রোগ্রাম থামাতে
<kbd>ctrl</kbd>-<kbd>C</kbd> কিবোর্ড শর্টকাট কাজ করে। একবার চেষ্টা করে দেখুন:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

`^C` চিহ্নটি দেখায় আপনি কোথায় <kbd>ctrl</kbd>-<kbd>C</kbd> চেপেছেন।

থামানোর সংকেত পাওয়ার সময় কোড loop-এর কোন জায়গায় ছিল, তার ওপর নির্ভর করে `^C`-এর পরে
`again!` শব্দটি প্রিন্ট হতেও পারে, না-ও হতে পারে।

সৌভাগ্যক্রমে, কোড দিয়ে loop থেকে বেরিয়ে আসার উপায়ও Rust দেয়। loop কখন থামাতে হবে,
তা প্রোগ্রামকে জানাতে loop-এর ভেতরে `break` keyword বসাতে পারেন। মনে আছে, অধ্যায় 2-এর
[“সঠিক আন্দাজের পর খেলা শেষ করা”][quitting-after-a-correct-guess]<!-- ignore
--> অংশে সংখ্যা আন্দাজের খেলায় আমরা এটি করেছিলাম, যাতে ব্যবহারকারী সঠিক সংখ্যা আন্দাজ
করে খেলায় জিতলে প্রোগ্রাম থেকে বেরিয়ে যাওয়া যায়।

সংখ্যা আন্দাজের খেলায় আমরা `continue`-ও ব্যবহার করেছিলাম, যা loop-এর ভেতরে প্রোগ্রামকে
বলে loop-এর এই iteration (loop-এর একেকবার চলা)-এর বাকি কোড এড়িয়ে পরের iteration-এ
যেতে।

#### loop থেকে মান return করা {#returning-values-from-loops}

`loop`-এর একটি ব্যবহার হলো এমন কোনো কাজ আবার চেষ্টা করা, যা ব্যর্থ হতে পারে বলে আপনি
জানেন, যেমন কোনো thread (প্রোগ্রামের ভেতরের আলাদা কাজের ধারা) তার কাজ শেষ করেছে কি না
তা যাচাই করা। সেই কাজের ফলাফল loop-এর বাইরে আপনার বাকি কোডে পাঠানোরও দরকার হতে পারে।
এটি করতে loop থামানোর জন্য ব্যবহৃত `break` expression-এর পরে যে মানটি return করতে চান,
তা যোগ করতে পারেন; সেই মানটি loop থেকে বেরিয়ে আসবে, যাতে আপনি সেটি ব্যবহার করতে পারেন,
এখানে যেমন দেখানো হয়েছে:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

loop-এর আগে আমরা `counter` নামে একটি variable ঘোষণা করে সেটিকে `0` দিয়ে initialize
করি। তারপর loop থেকে return হওয়া মান রাখার জন্য `result` নামে একটি variable ঘোষণা
করি। loop-এর প্রতিটি iteration-এ আমরা `counter` variable-এ `1` যোগ করি, তারপর যাচাই
করি `counter` `10`-এর সমান কি না। 
সমান হলে আমরা `counter * 2` মানসহ `break` keyword ব্যবহার করি। 
loop-এর পরে `result`-এ মান assign করা statement-টি শেষ করতে আমরা একটি সেমিকোলন দিই। সবশেষে আমরা
`result`-এর মান প্রিন্ট করি, যা এই ক্ষেত্রে `20`।

loop-এর ভেতর থেকে `return`-ও করা যায়। `break` শুধু বর্তমান loop থেকে বেরোয়, কিন্তু
`return` সবসময় বর্তমান function থেকে বেরিয়ে যায়।

> *দ্রষ্টব্য:* `break counter * 2`-এর পরের সেমিকোলনটি প্রযুক্তিগতভাবে ঐচ্ছিক। `break` অনেকটা `return`-এর মতো:
> দুটোই ঐচ্ছিকভাবে argument হিসেবে একটি expression নিতে পারে, দুটোই control flow বদলে দেয়।
> `break` বা `return`-এর পরের কোড কখনো চলে না, তাই Rust compiler একটি `break` expression আর
> একটি `return` expression-কে unit মান, অর্থাৎ `()` হিসেবে গণ্য করে।

<!-- Old headings. Do not remove or links may break. -->
<a id="loop-labels-to-disambiguate-between-multiple-loops"></a>

#### loop label দিয়ে দ্ব্যর্থতা দূর করা {#disambiguating-with-loop-labels}

loop-এর ভেতরে loop থাকলে `break` আর `continue` ওই জায়গার সবচেয়ে ভেতরের loop-এর ওপর
কাজ করে। চাইলে কোনো loop-এ একটি _loop label_ (loop-কে চেনানোর নাম-চিহ্ন) দিতে পারেন,
তারপর `break` বা `continue`-এর সঙ্গে সেটি ব্যবহার করে জানাতে পারেন যে keyword-গুলো
সবচেয়ে ভেতরের loop-এর বদলে label দেওয়া loop-টির ওপর কাজ করবে। loop label-কে অবশ্যই
একটি একক উদ্ধৃতিচিহ্ন দিয়ে শুরু হতে হবে। একটির ভেতরে আরেকটি, এমন দুটি loop-এর একটি
উদাহরণ:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

বাইরের loop-টির label হলো `'counting_up`, আর এটি 0 থেকে 2 পর্যন্ত গুনবে। label ছাড়া
ভেতরের loop-টি 10 থেকে 9 পর্যন্ত উল্টো দিকে গোনে। label উল্লেখ না করা প্রথম `break`
শুধু ভেতরের loop থেকে বেরোবে। `break
'counting_up;` statement বাইরের loop থেকে বেরিয়ে যাবে। এই কোড প্রিন্ট করে:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

<!-- Old headings. Do not remove or links may break. -->
<a id="conditional-loops-with-while"></a>

#### while দিয়ে শর্তসাপেক্ষ loop সহজ করা {#streamlining-conditional-loops-with-while}

প্রোগ্রামকে প্রায়ই loop-এর ভেতরে একটি শর্ত evaluate করতে হয়। শর্তটি `true` থাকা
পর্যন্ত loop চলে। শর্তটি আর `true` না থাকলে প্রোগ্রাম `break` call করে loop থামিয়ে দেয়।
`loop`, `if`, `else` আর `break` মিলিয়ে এমন আচরণ implement (বাস্তবায়ন) করা সম্ভব; চাইলে
এখনই কোনো প্রোগ্রামে সেটা চেষ্টা করে দেখতে পারেন। তবে এই ধরনটি এত প্রচলিত যে Rust-এ এর
জন্য একটি বিল্ট-ইন গঠন আছে, যার নাম `while` loop। Listing 3-3-এ আমরা `while` ব্যবহার করে
প্রোগ্রামটিকে তিনবার loop করাই, প্রতিবার এক করে উল্টো দিকে গুনে, তারপর loop-এর পরে একটি
বার্তা প্রিন্ট করে বেরিয়ে যাই।

<Listing number="3-3" file-name="src/main.rs" caption="একটি শর্ত `true` evaluate হওয়া পর্যন্ত কোড চালাতে `while` loop ব্যবহার">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

</Listing>

`loop`, `if`, `else` আর `break` ব্যবহার করলে যে অনেক স্তরের ভেতরে-ভেতরে লেখা দরকার হতো,
এই গঠন তা দূর করে, আর এটি আরও স্পষ্ট। শর্ত `true` evaluate হওয়া পর্যন্ত কোড চলে;
অন্যথায় loop থেকে বেরিয়ে যায়।

#### `for` দিয়ে collection-এর ওপর loop চালানো {#looping-through-a-collection-with-for}

array-র মতো কোনো collection-এর উপাদানগুলোর ওপর loop চালাতেও
`while` গঠন ব্যবহার করতে পারেন। যেমন, Listing 3-4-এর loop-টি `a` array-র প্রতিটি উপাদান
প্রিন্ট করে।

<Listing number="3-4" file-name="src/main.rs" caption="`while` loop ব্যবহার করে collection-এর প্রতিটি উপাদানের ওপর loop চালানো">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

</Listing>

এখানে কোডটি array-র উপাদানগুলো ধরে ধরে গুনে এগোয়। এটি index
`0` থেকে শুরু করে array-র শেষ index-এ পৌঁছানো পর্যন্ত loop চালায় (অর্থাৎ যখন
`index < 5` আর `true` থাকে না)। এই কোড চালালে array-র প্রতিটি উপাদান প্রিন্ট হবে:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

প্রত্যাশামতোই array-র পাঁচটি মানই টার্মিনালে দেখা যায়। `index` কোনো এক সময় `5` মানে
পৌঁছালেও array থেকে ষষ্ঠ মান আনার চেষ্টার আগেই loop চলা বন্ধ হয়ে যায়।

তবে এই পদ্ধতিতে ভুল হওয়ার সম্ভাবনা বেশি; index মান বা পরীক্ষার শর্ত ভুল হলে আমরা
প্রোগ্রামকে panic করাতে পারি। যেমন, আপনি যদি `a` array-র
সংজ্ঞা বদলে চারটি উপাদান রাখেন, কিন্তু শর্তটি `while index < 4`-এ আপডেট করতে ভুলে যান,
তাহলে কোডটি panic করবে। এটি ধীরও, কারণ loop-এর প্রতিটি iteration-এ index array-র সীমার
মধ্যে আছে কি না, সেই শর্ত যাচাইয়ের জন্য compiler runtime-এর কোড যোগ করে।

আরও সংক্ষিপ্ত বিকল্প হিসেবে একটি `for` loop ব্যবহার করে collection-এর প্রতিটি আইটেমের
জন্য কিছু কোড চালাতে পারেন। একটি `for` loop দেখতে Listing 3-5-এর কোডের মতো।

<Listing number="3-5" file-name="src/main.rs" caption="`for` loop ব্যবহার করে collection-এর প্রতিটি উপাদানের ওপর loop চালানো">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

</Listing>

এই কোড চালালে Listing 3-4-এর মতোই আউটপুট দেখব। আরও গুরুত্বপূর্ণ হলো, এখন আমরা কোডের
নিরাপত্তা বাড়িয়েছি এবং array-র শেষ পেরিয়ে যাওয়া, কিংবা যথেষ্ট দূর না গিয়ে কিছু আইটেম
বাদ পড়ে যাওয়া থেকে যেসব বাগ হতে পারত, সেগুলোর সম্ভাবনা দূর করেছি। `for` loop থেকে তৈরি
machine code (প্রসেসর সরাসরি চালাতে পারে এমন কোড)-ও আরও দক্ষ হতে পারে, কারণ প্রতিটি
iteration-এ index-কে array-র দৈর্ঘ্যের সঙ্গে তুলনা করতে হয় না।

`for` loop ব্যবহার করলে array-তে মানের সংখ্যা বদলালে অন্য কোনো কোড বদলানোর কথা মনে রাখতে
হবে না, যেমনটি Listing 3-4-এ ব্যবহৃত পদ্ধতিতে হতো।

নিরাপত্তা আর সংক্ষিপ্ততার কারণে `for` loop-ই Rust-এ সবচেয়ে বেশি ব্যবহৃত loop গঠন।
এমনকি যেসব পরিস্থিতিতে আপনি কিছু কোড নির্দিষ্টসংখ্যকবার চালাতে চান, যেমন Listing 3-3-এ
`while` loop ব্যবহার করা উল্টো গোনার উদাহরণে, সেখানেও বেশিরভাগ Rustacean একটি `for` loop
ব্যবহার করবেন। এর উপায় হলো standard library-র দেওয়া একটি `Range` ব্যবহার করা, যা একটি
সংখ্যা থেকে শুরু করে আরেকটি সংখ্যার আগ পর্যন্ত ক্রমানুসারে সব সংখ্যা তৈরি করে।

`for` loop আর এখনো আলোচনা না করা আরেকটি method `rev` দিয়ে range উল্টে দিলে উল্টো গোনাটি
দেখতে এরকম হবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

এই কোডটি একটু বেশি সুন্দর, তাই না?

{{#quiz ../quizzes/ch03-05-control-flow-sec2-loops.toml}}

## সারসংক্ষেপ {#summary}

পেরে গেছেন! এটি বেশ বড় একটি অধ্যায় ছিল: আপনি শিখেছেন variable, scalar ও compound
data type, function, কমেন্ট, `if` expression আর loop! এই অধ্যায়ে আলোচিত ধারণাগুলো
অনুশীলন করতে নিচের কাজগুলো করে এমন প্রোগ্রাম বানানোর চেষ্টা করুন:

- ফারেনহাইট আর সেলসিয়াসের মধ্যে তাপমাত্রা রূপান্তর করা।
- *n*-তম ফিবোনাচি সংখ্যা তৈরি করা।
- গানের পুনরাবৃত্তির সুবিধা নিয়ে বড়দিনের গান “The Twelve Days of Christmas”-এর কথাগুলো
  প্রিন্ট করা।

পরের ধাপে যাওয়ার জন্য প্রস্তুত হলে আমরা Rust-এর এমন একটি ধারণা নিয়ে কথা বলব, যা অন্যান্য
প্রোগ্রামিং ভাষায় সাধারণত _থাকে না_: ownership (মালিকানা)।

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess

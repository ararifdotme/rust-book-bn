## function {#functions}

Rust কোডে function সর্বত্র। ভাষাটির সবচেয়ে গুরুত্বপূর্ণ function-গুলোর একটি আপনি
আগেই দেখেছেন: `main` function, যা অনেক প্রোগ্রামের শুরুর জায়গা। `fn` keyword-ও
দেখেছেন, যা দিয়ে নতুন function declare (ঘোষণা) করা যায়।

Rust কোডে function আর variable-এর নামের প্রচলিত ধরন হলো _snake case_ (ছোট হাতের
অক্ষর আর আন্ডারস্কোর দিয়ে নাম লেখার রীতি), যেখানে সব অক্ষর ছোট হাতের, আর শব্দগুলোর
মাঝে আন্ডারস্কোর থাকে। এখানে একটি প্রোগ্রাম দেওয়া হলো, যাতে একটি function-এর সংজ্ঞার
উদাহরণ আছে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Rust-এ আমরা function সংজ্ঞায়িত করি `fn`, তারপর function-এর নাম আর একজোড়া বন্ধনী
লিখে। কার্লি ব্র্যাকেট compiler-কে জানায় function body (function-এর মূল অংশ) কোথায়
শুরু আর কোথায় শেষ।

সংজ্ঞায়িত যেকোনো function-কে তার নাম আর তারপর একজোড়া বন্ধনী লিখে call (চালানোর
নির্দেশ দেওয়া) করা যায়। `another_function` প্রোগ্রামে সংজ্ঞায়িত, তাই `main`
function-এর ভেতর থেকে একে call করা যায়। লক্ষ করুন, সোর্স কোডে আমরা
`another_function` সংজ্ঞায়িত করেছি `main` function-এর _পরে_; চাইলে আগেও করতে
পারতাম। আপনি function কোথায় সংজ্ঞায়িত করছেন, তা নিয়ে Rust মাথা ঘামায় না; শুধু
দেখে যে সেগুলো এমন কোনো scope-এ সংজ্ঞায়িত আছে কি না, যা caller (যে কোড function-টি
call করে) দেখতে পায়।

function আরও খুঁটিয়ে দেখতে চলুন _functions_ নামে একটি নতুন binary প্রজেক্ট শুরু করি।
`another_function` উদাহরণটি _src/main.rs_-এ বসিয়ে চালান। নিচের আউটপুট দেখার কথা:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

`main` function-এ লাইনগুলো যে ক্রমে আছে, সেই ক্রমেই চলে। প্রথমে “Hello, world!”
বার্তা প্রিন্ট হয়, তারপর `another_function` call হয় এবং তার বার্তা প্রিন্ট হয়।

### parameter {#parameters}

আমরা function-কে _parameter_ (function-এ পাঠানো মানের জন্য রাখা নাম) রাখার মতো করে
সংজ্ঞায়িত করতে পারি; parameter হলো বিশেষ variable, যা function-এর signature
(function-এর নাম, parameter ও return type-এর ঘোষণা)-এর অংশ। কোনো function-এর
parameter থাকলে সেই parameter-গুলোর জন্য আপনি নির্দিষ্ট মান দিতে পারেন। প্রযুক্তিগতভাবে
নির্দিষ্ট মানগুলোকে বলা হয় _argument_ (function-এ পাঠানো আসল মান), তবে সাধারণ কথাবার্তায়
মানুষ function-এর সংজ্ঞার variable আর function call করার সময় পাঠানো নির্দিষ্ট মান—দুটো
বোঝাতেই _parameter_ আর _argument_ শব্দ দুটি অদলবদল করে ব্যবহার করেন।

`another_function`-এর এই সংস্করণে আমরা একটি parameter যোগ করছি:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

প্রোগ্রামটি চালিয়ে দেখুন; নিচের আউটপুট পাওয়ার কথা:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

`another_function`-এর ঘোষণায় `x` নামে একটি parameter আছে। `x`-এর type নির্দিষ্ট করা
হয়েছে `i32`। আমরা যখন `another_function`-এ `5` পাঠাই, তখন `println!` macro (কোড
তৈরি করে এমন কোড) format string (placeholder-সহ string)-এ `x`-সহ কার্লি ব্র্যাকেট
জোড়াটি যেখানে ছিল, সেখানে `5` বসিয়ে দেয়।

function signature-এ প্রতিটি parameter-এর type _অবশ্যই_ ঘোষণা করতে হবে। এটি Rust-এর
নকশার একটি সচেতন সিদ্ধান্ত: function-এর সংজ্ঞায় type annotation বাধ্যতামূলক হওয়ায়
আপনি কোন type বোঝাতে চান, তা বুঝতে compiler-এর প্রায় কখনোই কোডের অন্য কোথাও আপনার
annotation দরকার হয় না। function কোন type আশা করে, তা জানা থাকলে compiler আরও
কাজের error message-ও দিতে পারে।

একাধিক parameter সংজ্ঞায়িত করার সময় parameter-এর ঘোষণাগুলো কমা দিয়ে আলাদা করুন,
এভাবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

এই উদাহরণটি দুটি parameter-সহ `print_labeled_measurement` নামে একটি function তৈরি
করে। প্রথম parameter-এর নাম `value`, এবং এটি একটি `i32`। দ্বিতীয়টির নাম `unit_label`,
এবং এর type `char`। তারপর function-টি `value` আর `unit_label`—দুটোই থাকা একটি লেখা
প্রিন্ট করে।

চলুন কোডটি চালিয়ে দেখি। আপনার _functions_ প্রজেক্টের _src/main.rs_ ফাইলে এখন যে
প্রোগ্রাম আছে, সেটি ওপরের উদাহরণ দিয়ে বদলে `cargo
run` দিয়ে চালান:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

আমরা function-টি call করেছি `value`-এর মান হিসেবে `5` আর `unit_label`-এর মান হিসেবে
`'h'` দিয়ে, তাই প্রোগ্রামের আউটপুটে সেই মানগুলোই আছে।

{{#quiz ../quizzes/ch03-03-functions-sec1-parameters.toml}}

### statement ও expression {#statements-and-expressions}

function body গঠিত হয় কয়েকটি statement দিয়ে, যার শেষে ঐচ্ছিকভাবে একটি expression
থাকতে পারে। এ পর্যন্ত আমরা যেসব function দেখেছি, সেগুলোর শেষে কোনো expression ছিল
না, তবে statement-এর অংশ হিসেবে expression আপনি দেখেছেন। Rust একটি
expression-based language (expression-কেন্দ্রিক ভাষা), তাই এই পার্থক্য বোঝা জরুরি।
অন্যান্য ভাষায় একই পার্থক্য নেই, তাই চলুন দেখি statement আর expression কী, আর এদের
পার্থক্য কীভাবে function body-কে প্রভাবিত করে।

- _statement_ হলো এমন নির্দেশ, যা কোনো কাজ করে কিন্তু কোনো মান return
  করে না।
- _expression_ evaluate হয়ে একটি ফলাফল-মান তৈরি করে।

চলুন কয়েকটি উদাহরণ দেখি।
আসলে আমরা statement আর expression আগেই ব্যবহার করেছি। `let` keyword দিয়ে একটি
variable তৈরি করে তাতে মান assign করা একটি statement। Listing 3-1-এ
`let y = 6;` একটি statement।

<Listing number="3-1" file-name="src/main.rs" caption="একটি statement-যুক্ত `main` function-এর ঘোষণা">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

function-এর সংজ্ঞাও statement; ওপরের পুরো উদাহরণটি নিজেই একটি statement। (তবে একটু
পরেই দেখব, function call করা statement নয়।)

statement কোনো মান return করে না। তাই নিচের কোডের মতো একটি `let` statement-কে অন্য
কোনো variable-এ assign করা যায় না; এতে error পাবেন:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

এই প্রোগ্রাম চালালে error-টি দেখতে এরকম হবে:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

`let y = 6` statement কোনো মান return করে না, তাই `x`-এর সঙ্গে bind করার মতো কিছু
নেই। C বা Ruby-র মতো অন্যান্য ভাষায় যা ঘটে, এটি তার থেকে আলাদা; সেসব ভাষায় assignment
নিজেই assign হওয়া মানটি return করে। ওই ভাষাগুলোতে আপনি `x = y = 6` লিখতে পারেন, আর
`x` ও `y` দুটোর মানই হবে `6`; Rust-এ তা হয় না।

expression evaluate হয়ে একটি মান তৈরি করে, আর Rust-এ আপনি যে কোড লিখবেন তার বাকি
বেশিরভাগই expression। একটি গাণিতিক operation ভাবুন, যেমন `5 + 6`; এটি একটি
expression, যা evaluate হয়ে `11` মান দেয়। expression statement-এর অংশ হতে পারে:
Listing 3-1-এ `let y = 6;` statement-এর `6` একটি expression, যা evaluate হয়ে `6`
মান দেয়। function call করা একটি expression। macro call করা একটি expression। কার্লি
ব্র্যাকেট দিয়ে তৈরি একটি নতুন scope block (`{}` দিয়ে ঘেরা কোড-অংশ)-ও একটি
expression, যেমন:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

এই expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

একটি block, যা এই ক্ষেত্রে evaluate হয়ে `4` দেয়। `let` statement-এর অংশ হিসেবে
সেই মানটি `y`-এর সঙ্গে bind হয়। লক্ষ করুন, `x + 1` লাইনের শেষে কোনো সেমিকোলন নেই,
যা এ পর্যন্ত দেখা বেশিরভাগ লাইনের মতো নয়। expression-এর শেষে সেমিকোলন থাকে না।
expression-এর শেষে সেমিকোলন যোগ করলে আপনি সেটিকে statement-এ পরিণত করেন, আর তখন সেটি
কোনো মান return করবে না। এরপর function-এর return মান আর expression দেখার সময় এটা
মনে রাখবেন।

### return মানসহ function {#functions-with-return-values}

function যে কোড তাকে call করে, তার কাছে মান return করতে পারে। return মানের আমরা নাম দিই
না, কিন্তু একটি তীরচিহ্ন (`->`)-এর পরে এর type ঘোষণা করতে হয়। Rust-এ function-এর
return মান আর function body-র block-এর শেষ expression-এর মান একই কথা। `return`
keyword দিয়ে একটি মান নির্দিষ্ট করে function থেকে আগেভাগেও return করা যায়, তবে বেশিরভাগ
function শেষ expression-টিই নিজে থেকে return করে। মান return করে এমন একটি function-এর
উদাহরণ:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

`five` function-এ কোনো function call, macro, এমনকি `let` statement-ও নেই—শুধু একা
সংখ্যা `5`। Rust-এ এটি সম্পূর্ণ বৈধ একটি function। লক্ষ করুন, function-এর return
type-ও নির্দিষ্ট করা আছে: `-> i32`। কোডটি চালিয়ে
দেখুন; আউটপুট এরকম হওয়ার কথা:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

`five`-এর ভেতরের `5` হলো function-এর return মান, আর এ কারণেই return type `i32`। চলুন
এটি আরও খুঁটিয়ে দেখি। দুটি গুরুত্বপূর্ণ বিষয় আছে:
প্রথমত, `let x = five();` লাইনটি দেখায় যে আমরা একটি function-এর return মান দিয়ে একটি
variable initialize করছি। `five` function একটি `5` return করে, তাই
ওই লাইনটি নিচের লাইনের মতোই:

```rust
let x = 5;
```

দ্বিতীয়ত, `five` function-এর কোনো parameter নেই, আর এটি return মানের type সংজ্ঞায়িত
করে। function-এর body হলো সেমিকোলন ছাড়া একলা একটি `5`, কারণ এটি এমন একটি
expression, যার মান আমরা return করতে চাই।

আরেকটি উদাহরণ দেখা যাক:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

এই কোড চালালে প্রিন্ট হবে `The value of x is: 6`। কিন্তু `x + 1` থাকা লাইনের শেষে
একটি সেমিকোলন বসিয়ে সেটিকে expression থেকে statement-এ বদলে দিলে কী হবে?

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

এই কোড compile করলে নিচের মতো একটি error আসবে:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

মূল error message `mismatched types` এই কোডের মূল সমস্যাটি দেখিয়ে দেয়। `plus_one`
function-এর সংজ্ঞা বলছে যে এটি একটি `i32` return করবে, কিন্তু statement কোনো মানে
evaluate হয় না, যা প্রকাশ করা হয় `()` দিয়ে, অর্থাৎ unit type।
ফলে কিছুই return হয় না, যা function-এর সংজ্ঞার বিরোধী, আর তাই error হয়। এই আউটপুটে
Rust সমস্যাটি সমাধানে সম্ভবত সাহায্য করবে এমন একটি বার্তা দেয়: এটি সেমিকোলনটি মুছে
দিতে পরামর্শ দেয়, যাতে error-টি ঠিক হয়ে যাবে।

{{#quiz ../quizzes/ch03-03-functions-sec2-expressions.toml}}

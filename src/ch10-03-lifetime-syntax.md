## lifetime দিয়ে reference যাচাই করা {#validating-references-with-lifetimes}

lifetime (reference কতক্ষণ বৈধ থাকে) হলো আরেক ধরনের generic, যা আমরা আগে থেকেই ব্যবহার করে আসছি। একটি type-এর
কাঙ্ক্ষিত আচরণ আছে কি না তা নিশ্চিত করার বদলে lifetime নিশ্চিত করে যে reference-গুলো যতক্ষণ দরকার ততক্ষণ বৈধ থাকবে।

অধ্যায় 4-এর [“reference ও borrowing”][references-and-borrowing]<!-- ignore --> অংশে একটি বিষয় আমরা আলোচনা করিনি: Rust-এর
প্রতিটি reference-এর একটি lifetime আছে, অর্থাৎ যে scope-এর জন্য reference-টি বৈধ। বেশিরভাগ সময় lifetime অপ্রকাশ্য
থাকে এবং অনুমান করে নেওয়া হয়, ঠিক যেমন বেশিরভাগ সময় type অনুমান করে নেওয়া হয়। একাধিক type সম্ভব হলেই কেবল আমাদের
type annotate করতে হয়। একইভাবে, যখন reference-গুলোর lifetime কয়েকটি ভিন্ন উপায়ে সম্পর্কিত হতে পারে, তখন lifetime
annotate করতে হয়। runtime-এ ব্যবহৃত আসল reference-গুলো যে নিশ্চিতভাবে বৈধ থাকবে, তা নিশ্চিত করতে Rust চায় আমরা generic
lifetime parameter (reference-এর lifetime বোঝানোর parameter) দিয়ে সম্পর্কগুলো annotate করি।

lifetime annotate করার ধারণাটি অন্য বেশিরভাগ প্রোগ্রামিং ভাষায় নেই, তাই এটি অপরিচিত লাগবে। এই অধ্যায়ে lifetime-এর
পুরোটা আলোচনা না করলেও lifetime syntax-এর সঙ্গে সাধারণত যেভাবে দেখা হতে পারে তা নিয়ে আলোচনা করব, যাতে ধারণাটির সঙ্গে
আপনি স্বচ্ছন্দ হতে পারেন।

<!-- Old headings. Do not remove or links may break. -->

<a id="preventing-dangling-references-with-lifetimes"></a>

### dangling reference {#dangling-references}

lifetime-এর মূল লক্ষ্য হলো dangling reference (অবৈধ ডেটার দিকে নির্দেশ করা reference) ঠেকানো; এগুলো থাকতে দিলে একটি
প্রোগ্রাম যে ডেটাকে নির্দেশ করার কথা তার বদলে অন্য ডেটাকে নির্দেশ করত। Listing 10-16-এর প্রোগ্রামটি দেখুন, যেখানে একটি
বাইরের scope আর একটি ভেতরের scope আছে।

<!-- TODO(aquascope): support for nested scopes -->
<Listing number="10-16" caption="এমন একটি reference ব্যবহারের চেষ্টা, যার মান scope-এর বাইরে চলে গেছে">

```rust,ignore,does_not_compile
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

</Listing>

> Note: Listing 10-16, 10-17 ও 10-23-এর উদাহরণে variable-কে কোনো প্রাথমিক মান না দিয়েই ঘোষণা করা হয়েছে, ফলে variable-এর
> নামটি বাইরের scope-এ থাকে। প্রথম দেখায় মনে হতে পারে এটি Rust-এ null মান না থাকার সঙ্গে সাংঘর্ষিক। কিন্তু মান দেওয়ার
> আগেই কোনো variable ব্যবহার করার চেষ্টা করলে আমরা compile-time error পাব, যা দেখায় যে Rust সত্যিই null মান অনুমোদন
> করে না।

বাইরের scope কোনো প্রাথমিক মান ছাড়া `r` নামের একটি variable ঘোষণা করে, আর ভেতরের scope `5` প্রাথমিক মানসহ `x` নামের
একটি variable ঘোষণা করে। ভেতরের scope-এর মধ্যে আমরা `r`-এর মান হিসেবে `x`-এর একটি reference বসানোর চেষ্টা করি। তারপর
ভেতরের scope শেষ হয়, আর আমরা `r`-এর মান print করার চেষ্টা করি। এই কোড compile হবে না, কারণ `r` যে মানকে নির্দেশ করছে
তা আমরা ব্যবহার করার চেষ্টার আগেই scope-এর বাইরে চলে গেছে। error message-টি এই:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/output.txt}}
```

error message বলছে variable `x` “does not live long enough” (যথেষ্ট সময় বেঁচে থাকে না)। কারণ লাইন 7-এ ভেতরের scope শেষ
হলে `x` scope-এর বাইরে চলে যাবে। কিন্তু `r` বাইরের scope-এর জন্য তখনো বৈধ; এর scope বড় বলে আমরা বলি এটি “বেশি সময় বেঁচে
থাকে”। Rust এই কোডকে চলতে দিলে `r` এমন মেমোরিকে নির্দেশ করত যা `x` scope-এর বাইরে যাওয়ার সময় deallocate (মুক্ত) হয়ে
গেছে, আর `r` দিয়ে যা-ই করার চেষ্টা করতাম তা ঠিকমতো কাজ করত না। তাহলে Rust কীভাবে নির্ধারণ করে যে এই কোড অবৈধ? এটি
একটি borrow checker ব্যবহার করে।

### borrow checker নিশ্চিত করে যে ডেটা তার reference-এর চেয়ে বেশি সময় বাঁচে {#the-borrow-checker-ensures-data-outlives-its-references}

Rust compiler-এর একটি _borrow checker_ (borrow-এর নিয়ম যাচাই করা compiler-এর অংশ) আছে, যা scope তুলনা করে নির্ধারণ করে
সব borrow বৈধ কি না। Listing 10-17-এ Listing 10-16-এর একই কোড দেখানো হয়েছে, তবে variable-গুলোর lifetime দেখানো annotation-সহ।

<Listing number="10-17" caption="`r` ও `x`-এর lifetime-এর annotation, যাদের নাম যথাক্রমে `'a` ও `'b`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

</Listing>

এখানে আমরা `r`-এর lifetime-কে `'a` আর `x`-এর lifetime-কে `'b` দিয়ে annotate করেছি। দেখতেই পাচ্ছেন, ভেতরের `'b` block বাইরের
`'a` lifetime block-এর চেয়ে অনেক ছোট। compile time-এ Rust দুটি lifetime-এর আকার তুলনা করে দেখে যে `r`-এর lifetime `'a`, কিন্তু
এটি `'b` lifetime-এর মেমোরিকে নির্দেশ করে। প্রোগ্রামটি বাতিল হয়, কারণ `'b` `'a`-এর চেয়ে ছোট: reference যাকে নির্দেশ করছে, সেটি
reference-এর মতো বেশি সময় বাঁচে না।

Listing 10-18-এ কোডটি এমনভাবে ঠিক করা হয়েছে যাতে কোনো dangling reference না থাকে এবং এটি কোনো error ছাড়া compile হয়।

<Listing number="10-18" caption="একটি বৈধ reference, কারণ ডেটার lifetime reference-এর চেয়ে দীর্ঘ">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

</Listing>

এখানে `x`-এর lifetime `'b`, যা এই ক্ষেত্রে `'a`-এর চেয়ে বড়। এর মানে `r` `x`-কে নির্দেশ করতে পারে, কারণ Rust জানে `x` যতক্ষণ বৈধ,
`r`-এর reference-টিও ততক্ষণ সবসময় বৈধ থাকবে।

reference-গুলোর lifetime কোথায় থাকে, আর reference সবসময় বৈধ থাকবে তা নিশ্চিত করতে Rust কীভাবে lifetime বিশ্লেষণ করে, তা
জানলেন। এবার function-এর parameter ও return মানে generic lifetime নিয়ে দেখা যাক।

### function-এ generic lifetime {#generic-lifetimes-in-functions}

আমরা এমন একটি function লিখব যা দুটি string slice-এর মধ্যে লম্বাটি return করে। এই function দুটি string slice নেবে এবং
একটি string slice return করবে। `longest` function implement করার পর Listing 10-19-এর কোড print করবে
`The longest string is abcd`।

<Listing number="10-19" file-name="src/main.rs" caption="একটি `main` function, যা দুটি string slice-এর মধ্যে লম্বাটি খুঁজতে `longest` function call করে">

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs}}
```

</Listing>

খেয়াল করুন, আমরা চাই function-টি string-এর বদলে string slice নিক, যা reference, কারণ আমরা চাই না `longest` function তার
parameter-গুলোর ownership নিয়ে নিক। Listing 10-19-এ আমরা কেন এই parameter-গুলোই চাই, তা নিয়ে আরও আলোচনার জন্য অধ্যায়
4-এর [“parameter হিসেবে string slice”][string-slices-as-parameters]<!-- ignore --> অংশটি দেখুন।

Listing 10-20-এর মতো করে `longest` function implement করার চেষ্টা করলে এটি compile হবে না।

<Listing number="10-20" file-name="src/main.rs" caption="`longest` function-এর একটি implementation, যা দুটি string slice-এর মধ্যে লম্বাটি return করে কিন্তু এখনো compile হয় না">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

</Listing>

বরং lifetime নিয়ে কথা বলা এই error-টি পাই:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/output.txt}}
```

help-এর লেখা থেকে বোঝা যায় যে return type-এ একটি generic lifetime parameter দরকার, কারণ return করা reference-টি `x`-কে নির্দেশ
করে নাকি `y`-কে, Rust তা বলতে পারে না। আসলে আমরাও জানি না, কারণ এই function-এর body-তে `if` block `x`-এর একটি reference
return করে আর `else` block `y`-এর একটি reference return করে!

এই function সংজ্ঞায়িত করার সময় আমরা জানি না function-এ কোন concrete মান পাঠানো হবে, তাই `if` অংশ চলবে নাকি `else` অংশ,
তা-ও জানি না। পাঠানো reference-গুলোর concrete lifetime-ও আমরা জানি না, তাই Listing 10-17 ও 10-18-এর মতো scope দেখে
নির্ধারণ করতে পারি না যে আমাদের return করা reference সবসময় বৈধ থাকবে কি না। borrow checker-ও এটি নির্ধারণ করতে পারে না,
কারণ এটি জানে না `x` ও `y`-এর lifetime return মানের lifetime-এর সঙ্গে কীভাবে সম্পর্কিত। এই error ঠিক করতে আমরা এমন generic
lifetime parameter যোগ করব যা reference-গুলোর মধ্যে সম্পর্ক সংজ্ঞায়িত করে, যাতে borrow checker তার বিশ্লেষণ চালাতে পারে।

### lifetime annotation syntax {#lifetime-annotation-syntax}

lifetime annotation কোনো reference কত সময় বাঁচবে তা বদলায় না। বরং এগুলো lifetime-কে প্রভাবিত না করেই একাধিক
reference-এর lifetime একে অপরের সঙ্গে কীভাবে সম্পর্কিত তা বর্ণনা করে। signature-এ generic type parameter থাকলে function যেমন
যেকোনো type গ্রহণ করতে পারে, তেমনি generic lifetime parameter উল্লেখ করলে function যেকোনো lifetime-এর reference গ্রহণ করতে
পারে।

lifetime annotation-এর syntax একটু অস্বাভাবিক: lifetime parameter-এর নাম একটি apostrophe (`'`) দিয়ে শুরু হতে হয়, আর generic
type-এর মতোই সাধারণত পুরোটা ছোট হাতের অক্ষরে ও খুব ছোট হয়। বেশিরভাগ মানুষ প্রথম lifetime annotation-এর জন্য `'a` নামটি
ব্যবহার করে। আমরা lifetime parameter-এর annotation বসাই reference-এর `&`-এর পরে, annotation আর reference-এর type-এর মাঝে
একটি space রেখে।

কয়েকটি উদাহরণ: lifetime parameter ছাড়া একটি `i32`-এর reference, `'a` নামের lifetime parameter-সহ একটি `i32`-এর reference,
আর একই `'a` lifetime-সহ একটি `i32`-এর mutable reference:

```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

একটিমাত্র lifetime annotation নিজে থেকে খুব একটা অর্থ বহন করে না, কারণ annotation-এর কাজই হলো Rust-কে জানানো যে একাধিক
reference-এর generic lifetime parameter একে অপরের সঙ্গে কীভাবে সম্পর্কিত। চলুন `longest` function-এর প্রেক্ষাপটে দেখি
lifetime annotation-গুলো একে অপরের সঙ্গে কীভাবে সম্পর্কিত।

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-function-signatures"></a>

### function signature-এ {#in-function-signatures}

function signature-এ lifetime annotation ব্যবহার করতে আমাদের function-এর নাম আর parameter তালিকার মাঝে angle bracket-এর
ভেতরে generic lifetime parameter ঘোষণা করতে হয়, ঠিক যেমন generic type parameter-এর ক্ষেত্রে করেছিলাম।

আমরা চাই signature-টি এই শর্ত প্রকাশ করুক: যতক্ষণ দুটি parameter-ই বৈধ, ততক্ষণ return করা reference-টিও বৈধ থাকবে। এটাই
parameter-গুলোর lifetime আর return মানের মধ্যে সম্পর্ক। আমরা lifetime-টির নাম দেব `'a`, তারপর Listing 10-21-এর মতো প্রতিটি
reference-এ এটি যোগ করব।

<Listing number="10-21" file-name="src/main.rs" caption="`longest` function-এর সংজ্ঞা, যা বলে দেয় signature-এর সব reference-এর lifetime একই `'a` হতে হবে">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

</Listing>

Listing 10-19-এর `main` function-এর সঙ্গে ব্যবহার করলে এই কোড compile হবে এবং আমাদের কাঙ্ক্ষিত ফল দেবে।

function signature এখন Rust-কে বলছে যে কোনো এক lifetime `'a`-এর জন্য function-টি দুটি parameter নেয়, যার দুটিই এমন string
slice যা অন্তত `'a` lifetime-এর সমান সময় বাঁচে। function signature Rust-কে এটাও বলছে যে function থেকে return করা string
slice অন্তত `'a` lifetime-এর সমান সময় বাঁচবে। বাস্তবে এর মানে হলো, `longest` function-এর return করা reference-এর lifetime
function-এর argument-গুলো যে মানগুলোকে নির্দেশ করে তাদের lifetime-এর মধ্যে ছোটটির সমান। এই কোড বিশ্লেষণের সময় আমরা
চাই Rust এই সম্পর্কগুলোই ব্যবহার করুক।

মনে রাখবেন, এই function signature-এ lifetime parameter উল্লেখ করে আমরা পাঠানো বা return করা কোনো মানের lifetime বদলাচ্ছি
না। বরং বলে দিচ্ছি যে এই শর্ত মানে না এমন যেকোনো মান borrow checker-কে বাতিল করতে হবে। খেয়াল করুন, `longest`
function-এর জানার দরকার নেই `x` আর `y` ঠিক কত সময় বাঁচবে; শুধু এটুকু দরকার যে `'a`-এর জায়গায় এমন কোনো scope বসানো যায়
যা এই signature পূরণ করে।

function-এ lifetime annotate করার সময় annotation-গুলো যায় function signature-এ, function-এর body-তে নয়। lifetime
annotation-গুলো function-এর চুক্তির অংশ হয়ে যায়, অনেকটা signature-এর type-গুলোর মতো। function signature-এ lifetime-এর চুক্তি
থাকায় Rust compiler-এর বিশ্লেষণ সহজ হতে পারে। কোনো function যেভাবে annotate করা হয়েছে বা যেভাবে call করা হয়েছে তাতে
সমস্যা থাকলে compiler error আমাদের কোডের অংশ আর শর্তগুলোকে আরও নিখুঁতভাবে দেখিয়ে দিতে পারে। অন্যদিকে, যদি Rust compiler
lifetime-গুলোর সম্পর্ক নিয়ে আমাদের উদ্দেশ্য সম্পর্কে আরও বেশি অনুমান করত, তাহলে compiler হয়তো শুধু আমাদের কোডের এমন
কোনো ব্যবহার দেখাতে পারত যা সমস্যার আসল কারণ থেকে অনেক ধাপ দূরে।

যখন আমরা `longest`-এ concrete reference পাঠাই, তখন `'a`-এর জায়গায় যে concrete lifetime বসে, তা হলো `x`-এর scope-এর সেই অংশ
যা `y`-এর scope-এর সঙ্গে মিলে যায়। অন্যভাবে বললে, generic lifetime `'a` পাবে এমন concrete lifetime, যা `x` ও `y`-এর lifetime-এর
মধ্যে ছোটটির সমান। return করা reference-কে আমরা একই lifetime parameter `'a` দিয়ে annotate করেছি বলে return করা reference-ও
`x` ও `y`-এর lifetime-এর মধ্যে ছোটটির সমান সময় পর্যন্ত বৈধ থাকবে।

চলুন ভিন্ন ভিন্ন concrete lifetime-এর reference পাঠিয়ে দেখি lifetime annotation কীভাবে `longest` function-কে সীমাবদ্ধ করে।
Listing 10-22 একটি সোজাসাপ্টা উদাহরণ।

<Listing number="10-22" file-name="src/main.rs" caption="ভিন্ন ভিন্ন concrete lifetime-এর `String` মানের reference দিয়ে `longest` function ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

</Listing>

এই উদাহরণে `string1` বাইরের scope-এর শেষ পর্যন্ত বৈধ, `string2` ভেতরের scope-এর শেষ পর্যন্ত বৈধ, আর `result` এমন কিছুকে
নির্দেশ করে যা ভেতরের scope-এর শেষ পর্যন্ত বৈধ। কোডটি চালালে দেখবেন borrow checker অনুমোদন দেয়; এটি compile হবে এবং
print করবে `The longest string
is long string is long`।

এবার এমন একটি উদাহরণ দেখি যা দেখায় যে `result`-এর reference-এর lifetime-কে দুটি argument-এর মধ্যে ছোট lifetime-টি হতে হবে।
আমরা `result` variable-এর ঘোষণা ভেতরের scope-এর বাইরে সরিয়ে নেব, কিন্তু `result` variable-এ মান বসানোটা `string2`-সহ
scope-এর ভেতরেই রাখব। তারপর `result` ব্যবহার করা `println!`-কে ভেতরের scope-এর বাইরে, ভেতরের scope শেষ হওয়ার পরে সরিয়ে
নেব। Listing 10-23-এর কোড compile হবে না।

<Listing number="10-23" file-name="src/main.rs" caption="`string2` scope-এর বাইরে যাওয়ার পর `result` ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

</Listing>

এই কোড compile করার চেষ্টা করলে এই error পাই:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/output.txt}}
```

error-টি দেখায় যে `println!` statement-এর জন্য `result` বৈধ থাকতে হলে `string2`-কে বাইরের scope-এর শেষ পর্যন্ত বৈধ থাকতে
হতো। Rust এটা জানে, কারণ আমরা function-এর parameter আর return মানের lifetime একই lifetime parameter `'a` দিয়ে annotate
করেছি।

মানুষ হিসেবে আমরা এই কোড দেখে বুঝতে পারি যে `string1` `string2`-এর চেয়ে লম্বা, তাই `result`-এ থাকবে `string1`-এর একটি
reference। `string1` যেহেতু এখনো scope-এর বাইরে যায়নি, তাই `println!` statement-এর জন্য `string1`-এর reference তখনো বৈধ
থাকবে। কিন্তু compiler এই ক্ষেত্রে দেখতে পায় না যে reference-টি বৈধ। আমরা Rust-কে বলেছি যে `longest` function-এর return
করা reference-এর lifetime পাঠানো reference-গুলোর lifetime-এর মধ্যে ছোটটির সমান। তাই borrow checker Listing 10-23-এর
কোডকে সম্ভাব্য অবৈধ reference থাকতে পারে বলে অনুমোদন দেয় না।

`longest` function-এ পাঠানো reference-গুলোর মান ও lifetime, আর return করা reference কীভাবে ব্যবহার হচ্ছে, তা বদলে আরও
পরীক্ষা সাজিয়ে দেখুন। compile করার আগে অনুমান করুন আপনার পরীক্ষাগুলো borrow checker পার হবে কি না; তারপর মিলিয়ে দেখুন
আপনার অনুমান ঠিক ছিল কি না!

{{#quiz ../quizzes/ch10-03-lifetimes-sec1.toml}}

<!-- Old headings. Do not remove or links may break. -->

<a id="thinking-in-terms-of-lifetimes"></a>

### সম্পর্ক {#relationships}

lifetime parameter কীভাবে উল্লেখ করতে হবে, তা নির্ভর করে আপনার function কী করছে তার ওপর। যেমন, `longest` function-এর
implementation বদলে যদি সবসময় সবচেয়ে লম্বা string slice-এর বদলে প্রথম parameter-টি return করতাম, তাহলে `y` parameter-এ
lifetime উল্লেখ করার দরকার হতো না। নিচের কোড compile হবে:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

</Listing>

আমরা `x` parameter আর return type-এর জন্য lifetime parameter `'a` উল্লেখ করেছি, কিন্তু `y` parameter-এর জন্য নয়, কারণ `y`-এর
lifetime-এর সঙ্গে `x`-এর বা return মানের lifetime-এর কোনো সম্পর্ক নেই।

function থেকে reference return করার সময় return type-এর lifetime parameter-কে parameter-গুলোর কোনো একটির lifetime
parameter-এর সঙ্গে মিলতে হবে। return করা reference যদি parameter-গুলোর কোনোটিকে নির্দেশ _না_ করে, তাহলে এটিকে এই function-এর
ভেতরে তৈরি কোনো মানকে নির্দেশ করতে হবে। কিন্তু সেটি হবে একটি dangling reference, কারণ function-এর শেষে মানটি scope-এর বাইরে
চলে যাবে। `longest` function-এর এই implementation-এর চেষ্টাটি দেখুন, যা compile হবে না:

<Listing file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

</Listing>

এখানে return type-এর জন্য lifetime parameter `'a` উল্লেখ করা সত্ত্বেও এই implementation compile হবে না, কারণ return মানের
lifetime parameter-গুলোর lifetime-এর সঙ্গে মোটেই সম্পর্কিত নয়। আমরা যে error message পাই:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

সমস্যা হলো `longest` function-এর শেষে `result` scope-এর বাইরে চলে যায় এবং পরিষ্কার হয়ে যায়। অথচ আমরা function থেকে
`result`-এর একটি reference return করার চেষ্টা করছি। এমন কোনো lifetime parameter উল্লেখ করার উপায় নেই যা dangling
reference-টিকে বদলে দেবে, আর Rust আমাদের dangling reference তৈরি করতে দেবে না। এই ক্ষেত্রে সবচেয়ে ভালো সমাধান হবে
reference-এর বদলে একটি owned data type (নিজের ডেটার মালিক এমন type) return করা, যাতে মানটি পরিষ্কার করার দায়িত্ব call
করা function-এর ওপর পড়ে।

শেষ পর্যন্ত lifetime syntax-এর কাজ হলো function-এর বিভিন্ন parameter আর return মানের lifetime-গুলোকে যুক্ত করা। একবার এগুলো
যুক্ত হলে Rust-এর কাছে যথেষ্ট তথ্য থাকে, যাতে মেমোরি-নিরাপদ operation অনুমোদন করতে পারে এবং dangling pointer তৈরি করবে
বা অন্য কোনোভাবে memory safety ভঙ্গ করবে এমন operation বাতিল করতে পারে।

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-struct-definitions"></a>

### struct-এর সংজ্ঞায় {#in-struct-definitions}

এ পর্যন্ত আমরা যত struct সংজ্ঞায়িত করেছি, সবগুলোই owned type রাখে। reference রাখার জন্যও struct সংজ্ঞায়িত করা যায়, তবে
সেক্ষেত্রে struct-এর সংজ্ঞার প্রতিটি reference-এ একটি lifetime annotation যোগ করতে হবে। Listing 10-24-এ `ImportantExcerpt`
নামের একটি struct আছে, যা একটি string slice রাখে।

<Listing number="10-24" file-name="src/main.rs" caption="একটি reference রাখে এমন struct, যার জন্য lifetime annotation দরকার">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

</Listing>

এই struct-এর একটিমাত্র field `part`, যা একটি string slice রাখে, আর string slice হলো reference। generic data type-এর মতোই
আমরা struct-এর নামের পরে angle bracket-এর ভেতরে generic lifetime parameter-এর নাম ঘোষণা করি, যাতে struct-এর সংজ্ঞার
body-তে lifetime parameter-টি ব্যবহার করতে পারি। এই annotation-এর মানে হলো, `ImportantExcerpt`-এর একটি instance তার `part`
field-এ রাখা reference-এর চেয়ে বেশি সময় বাঁচতে পারবে না।

এখানে `main` function `ImportantExcerpt` struct-এর একটি instance তৈরি করে, যা `novel` variable-এর মালিকানাধীন `String`-এর
প্রথম বাক্যের একটি reference রাখে। `ImportantExcerpt` instance তৈরি হওয়ার আগেই `novel`-এর ডেটা থাকে। তা ছাড়া
`ImportantExcerpt` scope-এর বাইরে যাওয়ার পরেই কেবল `novel` scope-এর বাইরে যায়, তাই `ImportantExcerpt` instance-এর
reference-টি বৈধ।

### lifetime elision {#lifetime-elision}

আপনি শিখেছেন যে প্রতিটি reference-এর একটি lifetime আছে, আর reference ব্যবহার করা function বা struct-এর জন্য lifetime
parameter উল্লেখ করতে হয়। কিন্তু Listing 4-9-এ আমাদের একটি function ছিল, যা Listing 10-25-এ আবার দেখানো হয়েছে, যেটি
lifetime annotation ছাড়াই compile হয়েছিল।

<Listing number="10-25" file-name="src/lib.rs" caption="Listing 4-9-এ সংজ্ঞায়িত একটি function, যা parameter আর return type reference হওয়া সত্ত্বেও lifetime annotation ছাড়াই compile হয়েছিল">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

</Listing>

এই function lifetime annotation ছাড়া compile হওয়ার কারণ ঐতিহাসিক: Rust-এর প্রথম দিকের সংস্করণে (1.0-এর আগে) এই কোড
compile হতো না, কারণ প্রতিটি reference-এর একটি স্পষ্ট lifetime দরকার হতো। সেই সময় function signature-টি এভাবে লেখা হতো:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

অনেক Rust কোড লেখার পর Rust দল দেখল যে নির্দিষ্ট কিছু পরিস্থিতিতে Rust প্রোগ্রামাররা বারবার একই lifetime annotation
লিখছেন। এই পরিস্থিতিগুলো অনুমানযোগ্য ছিল এবং কয়েকটি সুনির্দিষ্ট pattern মেনে চলত। ডেভেলপাররা এই pattern-গুলো compiler-এর
কোডে যুক্ত করে দিলেন, যাতে borrow checker এই পরিস্থিতিগুলোতে lifetime অনুমান করে নিতে পারে এবং স্পষ্ট annotation-এর
দরকার না হয়।

Rust-এর ইতিহাসের এই অংশটি প্রাসঙ্গিক, কারণ এমন হতে পারে যে আরও সুনির্দিষ্ট pattern সামনে আসবে এবং compiler-এ যুক্ত হবে।
ভবিষ্যতে হয়তো আরও কম lifetime annotation লাগবে।

reference সম্পর্কে Rust-এর বিশ্লেষণে যুক্ত করা pattern-গুলোকে বলা হয় _lifetime elision rules_ (lifetime বাদ দিয়ে লেখার
নিয়ম)। এগুলো প্রোগ্রামারদের মেনে চলার নিয়ম নয়; এগুলো কিছু নির্দিষ্ট ক্ষেত্রের একটি সেট, যা compiler বিবেচনা করবে, আর
আপনার কোড এই ক্ষেত্রগুলোর সঙ্গে মিলে গেলে lifetime স্পষ্ট করে লেখার দরকার নেই।

elision rule-গুলো পূর্ণ অনুমান দেয় না। Rust নিয়মগুলো প্রয়োগ করার পরও যদি reference-গুলোর lifetime নিয়ে অস্পষ্টতা থেকে
যায়, তাহলে compiler বাকি reference-গুলোর lifetime কী হওয়া উচিত তা আন্দাজ করবে না। আন্দাজ না করে compiler আপনাকে একটি
error দেবে, যা আপনি lifetime annotation যোগ করে সমাধান করতে পারেন।

function বা method-এর parameter-এর lifetime-কে বলা হয় _input lifetimes_, আর return মানের lifetime-কে বলা হয় _output
lifetimes_।

স্পষ্ট annotation না থাকলে reference-গুলোর lifetime বের করতে compiler তিনটি নিয়ম ব্যবহার করে। প্রথম নিয়মটি input
lifetime-এর ক্ষেত্রে প্রযোজ্য, আর দ্বিতীয় ও তৃতীয় নিয়ম output lifetime-এর ক্ষেত্রে। তিনটি নিয়মের শেষে পৌঁছেও যদি এমন
reference থাকে যার lifetime compiler বের করতে পারেনি, তাহলে compiler একটি error দিয়ে থেমে যাবে। এই নিয়মগুলো `fn`-এর সংজ্ঞা
এবং `impl` block, দুই ক্ষেত্রেই প্রযোজ্য।

<!-- BEGIN INTERVENTION: d03748df-8dcf-4ec8-bd30-341927544665 -->
প্রথম নিয়মটি হলো, compiler প্রতিটি input type-এর প্রতিটি lifetime-কে আলাদা একটি lifetime parameter দেয়। `&'_ i32`-এর মতো reference-এর একটি lifetime parameter দরকার, আর `ImportantExcerpt<'_>`-এর মতো structure-এরও একটি lifetime parameter দরকার। যেমন:
* `fn foo(x: &i32)` function একটি lifetime parameter পাবে এবং হয়ে যাবে `fn foo<'a>(x: &'a i32)`। 
* `fn foo(x: &i32, y: &i32)` function দুটি lifetime parameter পাবে এবং হয়ে যাবে `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`।
* `fn foo(x: &ImportantExcerpt)` function দুটি lifetime parameter পাবে এবং হয়ে যাবে `fn foo<'a, 'b>(x: &'a ImportantExcerpt<'b>)`।
<!-- END INTERVENTION -->

দ্বিতীয় নিয়মটি হলো, যদি ঠিক একটি input lifetime parameter থাকে, তাহলে সেই lifetime সব output lifetime parameter-কে দেওয়া হয়:
`fn foo<'a>(x: &'a i32)
-> &'a i32`।

তৃতীয় নিয়মটি হলো, যদি একাধিক input lifetime parameter থাকে, কিন্তু method হওয়ার কারণে তাদের একটি `&self` বা `&mut self`
হয়, তাহলে `self`-এর lifetime সব output lifetime parameter-কে দেওয়া হয়। এই তৃতীয় নিয়মের কারণে method পড়া ও লেখা অনেক
সহজ হয়, কারণ কম চিহ্ন লাগে।

চলুন নিজেদের compiler ভেবে নিই। Listing 10-25-এর `first_word` function-এর signature-এ reference-গুলোর lifetime বের করতে
আমরা এই নিয়মগুলো প্রয়োগ করব। signature-টি শুরুতে reference-গুলোর সঙ্গে কোনো lifetime ছাড়া থাকে:

```rust,ignore
fn first_word(s: &str) -> &str {
```

তারপর compiler প্রথম নিয়মটি প্রয়োগ করে, যা বলে প্রতিটি parameter নিজস্ব lifetime পায়। বরাবরের মতো আমরা একে `'a` বলব,
তাই signature-টি এখন এই:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```

দ্বিতীয় নিয়মটি প্রযোজ্য, কারণ ঠিক একটি input lifetime আছে। দ্বিতীয় নিয়ম বলে, একমাত্র input parameter-এর lifetime
output lifetime-কে দেওয়া হয়, তাই signature-টি এখন এই:

```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```

এখন এই function signature-এর সব reference-এর lifetime আছে, আর প্রোগ্রামারকে এই function signature-এ lifetime annotate
করতে না হয়েই compiler তার বিশ্লেষণ চালিয়ে যেতে পারে।

আরেকটি উদাহরণ দেখা যাক, এবার `longest` function দিয়ে, যা Listing 10-20-এ কাজ শুরু করার সময় কোনো lifetime parameter ছাড়া
ছিল:

```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```

প্রথম নিয়মটি প্রয়োগ করি: প্রতিটি parameter নিজস্ব lifetime পায়। এবার একটির বদলে দুটি parameter আছে, তাই দুটি lifetime:

```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

দেখতে পাচ্ছেন দ্বিতীয় নিয়মটি প্রযোজ্য নয়, কারণ একাধিক input lifetime আছে। তৃতীয় নিয়মটিও প্রযোজ্য নয়, কারণ `longest`
method নয়, একটি function, তাই কোনো parameter-ই `self` নয়। তিনটি নিয়ম পার হয়েও আমরা return type-এর lifetime কী তা বের
করতে পারিনি। এ কারণেই Listing 10-20-এর কোড compile করতে গিয়ে আমরা error পেয়েছিলাম: compiler lifetime elision rule-গুলো পার
হয়েও signature-এর সব reference-এর lifetime বের করতে পারেনি।

তৃতীয় নিয়মটি যেহেতু আসলে শুধু method signature-এ প্রযোজ্য, তাই এবার সেই প্রেক্ষাপটে lifetime দেখব, যাতে বোঝা যায় কেন
তৃতীয় নিয়মের কারণে method signature-এ খুব বেশি lifetime annotate করতে হয় না।

<!-- Old headings. Do not remove or links may break. -->

<a id="lifetime-annotations-in-method-definitions"></a>

### method-এর সংজ্ঞায় {#in-method-definitions}

lifetime-যুক্ত struct-এর ওপর method implement করার সময় আমরা generic type parameter-এর মতোই syntax ব্যবহার করি, যেমন
Listing 10-11-এ দেখানো হয়েছে। lifetime parameter কোথায় ঘোষণা ও ব্যবহার করব, তা নির্ভর করে সেগুলো struct-এর field-এর সঙ্গে
সম্পর্কিত নাকি method-এর parameter ও return মানের সঙ্গে।

struct-এর field-এর lifetime-এর নাম সবসময় `impl` keyword-এর পরে ঘোষণা করতে হয় এবং তারপর struct-এর নামের পরে ব্যবহার করতে
হয়, কারণ ওই lifetime-গুলো struct-এর type-এর অংশ।

`impl` block-এর ভেতরে method signature-এ reference-গুলো struct-এর field-এ থাকা reference-এর lifetime-এর সঙ্গে বাঁধা থাকতে
পারে, আবার স্বাধীনও হতে পারে। তা ছাড়া lifetime elision rule-এর কারণে প্রায়ই method signature-এ lifetime annotation লাগে না।
Listing 10-24-এ সংজ্ঞায়িত `ImportantExcerpt` নামের struct ব্যবহার করে কয়েকটি উদাহরণ দেখা যাক।

প্রথমে আমরা `level` নামের একটি method ব্যবহার করব, যার একমাত্র parameter হলো `self`-এর একটি reference এবং যার return মান একটি
`i32`, যা কোনো কিছুর reference নয়:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

`impl`-এর পরে lifetime parameter-এর ঘোষণা এবং type-এর নামের পরে এর ব্যবহার বাধ্যতামূলক, কিন্তু প্রথম elision rule-এর কারণে
`self`-এর reference-এর lifetime annotate করার দরকার নেই।

এখানে একটি উদাহরণ, যেখানে তৃতীয় lifetime elision rule প্রযোজ্য:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

দুটি input lifetime আছে, তাই Rust প্রথম lifetime elision rule প্রয়োগ করে `&self` আর `announcement` দুটিকেই নিজস্ব lifetime
দেয়। তারপর parameter-গুলোর একটি `&self` হওয়ায় return type পায় `&self`-এর lifetime, আর সব lifetime-এর হিসাব মিলে যায়।

### static lifetime {#the-static-lifetime}

একটি বিশেষ lifetime নিয়ে আলোচনা করা দরকার, তা হলো `'static`, যা বোঝায় সংশ্লিষ্ট reference প্রোগ্রামের পুরো সময় জুড়ে বেঁচে
থাকতে _পারে_। সব string literal-এর lifetime `'static`, যা আমরা এভাবে annotate করতে পারি:

```rust
let s: &'static str = "I have a static lifetime.";
```

এই string-এর লেখাটি সরাসরি প্রোগ্রামের binary-তে রাখা থাকে, যা সবসময় পাওয়া যায়। তাই সব string literal-এর lifetime হলো
`'static`।

error message-এ হয়তো `'static` lifetime ব্যবহারের পরামর্শ দেখতে পাবেন। কিন্তু কোনো reference-এর lifetime হিসেবে `'static`
উল্লেখ করার আগে ভেবে দেখুন, আপনার reference-টি আসলেই প্রোগ্রামের পুরো lifetime জুড়ে বাঁচে কি না, আর আপনি তা চান কি না।
বেশিরভাগ সময় `'static` lifetime-এর পরামর্শ দেওয়া error message আসে একটি dangling reference তৈরির চেষ্টা থেকে, অথবা
পাওয়া lifetime-গুলোর মধ্যে অমিল থেকে। এমন ক্ষেত্রে সমাধান হলো সেই সমস্যাগুলো ঠিক করা, `'static` lifetime উল্লেখ করা নয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="generic-type-parameters-trait-bounds-and-lifetimes-together"></a>

## generic type parameter, trait bound ও lifetime {#generic-type-parameters-trait-bounds-and-lifetimes}

চলুন সংক্ষেপে দেখি একটি function-এ একসঙ্গে generic type parameter, trait bound আর lifetime উল্লেখ করার syntax কেমন!

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

এটি Listing 10-21-এর `longest` function, যা দুটি string slice-এর মধ্যে লম্বাটি return করে। কিন্তু এখন এতে generic type `T`-এর
`ann` নামে একটি বাড়তি parameter আছে, যার জায়গায় `where` clause-এ উল্লেখ করা অনুযায়ী `Display` trait implement করে এমন
যেকোনো type বসানো যায়। এই বাড়তি parameter-টি `{}` দিয়ে print করা হবে, তাই `Display` trait bound দরকার। lifetime যেহেতু এক
ধরনের generic, তাই lifetime parameter `'a` আর generic type parameter `T`-এর ঘোষণা function-এর নামের পরে angle bracket-এর
ভেতরে একই তালিকায় যায়।

{{#quiz ../quizzes/ch10-03-lifetimes-sec2.toml}}

### সারসংক্ষেপ {#summary}

এই অধ্যায়ে আমরা অনেক কিছু শিখলাম! generic type parameter, trait ও trait bound, আর generic lifetime parameter সম্পর্কে
জানার পর আপনি এমন কোড লিখতে প্রস্তুত, যাতে পুনরাবৃত্তি নেই এবং যা অনেক ভিন্ন পরিস্থিতিতে কাজ করে। generic type parameter
দিয়ে কোডকে ভিন্ন ভিন্ন type-এ প্রয়োগ করা যায়। trait ও trait bound নিশ্চিত করে যে type-গুলো generic হলেও কোডের প্রয়োজনীয়
আচরণ তাদের থাকবে। আপনি শিখেছেন কীভাবে lifetime annotation ব্যবহার করে নিশ্চিত করতে হয় যে এই নমনীয় কোডে কোনো dangling
reference থাকবে না। আর এই সব বিশ্লেষণ হয় compile time-এ, যা runtime-এর performance-এ কোনো প্রভাব ফেলে না!

বিশ্বাস করুন বা না করুন, এই অধ্যায়ে আলোচিত বিষয়গুলোতে শেখার আরও অনেক কিছু আছে: অধ্যায় 18-এ trait object (trait-এর মাধ্যমে
বিভিন্ন type ব্যবহার করার মান) নিয়ে আলোচনা করা হয়েছে, যা trait ব্যবহারের আরেকটি উপায়। lifetime annotation নিয়ে আরও জটিল
কিছু পরিস্থিতিও আছে, যা শুধু খুব উন্নত পর্যায়ের ক্ষেত্রে দরকার হবে; সেগুলোর জন্য [Rust Reference][reference] পড়া উচিত। তবে
এরপর আপনি শিখবেন কীভাবে Rust-এ test লিখতে হয়, যাতে নিশ্চিত হতে পারেন যে আপনার কোড ঠিকঠাক কাজ করছে।

[references-and-borrowing]: ch04-02-references-and-borrowing.html#references-and-borrowing
[lifetime-permissions]: ch04-02-references-and-borrowing.html#permissions-are-returned-at-the-end-of-a-references-lifetime
[string-slices-as-parameters]: ch04-04-slices.html#string-slices-as-parameters
[reference]: https://doc.rust-lang.org/reference/trait-bounds.html

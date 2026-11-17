## ownership কী? {#what-is-ownership}

ownership হলো Rust প্রোগ্রামের **নিরাপত্তা** নিশ্চিত করার একটি নিয়মশৃঙ্খলা। ownership বুঝতে হলে আগে বুঝতে হবে, কোন জিনিস একটি Rust প্রোগ্রামকে নিরাপদ (বা অনিরাপদ) করে।

### নিরাপত্তা মানে undefined behavior-এর অনুপস্থিতি {#safety-is-the-absence-of-undefined-behavior}

একটি উদাহরণ দিয়ে শুরু করা যাক। এই প্রোগ্রামটি চালানো নিরাপদ:

```rust
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    let x = true;
    read(x);
}
```

`read` call-টি `x`-এর সংজ্ঞার আগে সরিয়ে নিলে এই প্রোগ্রাম চালানো অনিরাপদ হয়ে যায়:

```rust,ignore,does_not_compile
fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn main() {
    read(x); // oh no! x isn't defined!
    let x = true;
}
```

> *দ্রষ্টব্য*: এই অধ্যায়ে আমরা এমন অনেক কোডের উদাহরণ ব্যবহার করব, যেগুলো compile হয় না। কোনো প্রোগ্রাম compile হওয়ার কথা কি না, তা নিয়ে নিশ্চিত না হলে প্রশ্নবোধক চিহ্নওয়ালা কাঁকড়াটি খুঁজে দেখবেন।

দ্বিতীয় প্রোগ্রামটি অনিরাপদ, কারণ `read(x)` আশা করে `x`-এর একটি `bool` type-এর মান থাকবে, কিন্তু `x`-এর এখনো কোনো মানই নেই।

এমন একটি প্রোগ্রাম যখন একটি interpreter (কোড লাইন ধরে ধরে চালানোর প্রোগ্রাম) চালায়, তখন সংজ্ঞায়িত হওয়ার আগে `x` পড়লে একটি exception (চলার সময় ঘটা error-এর সংকেত) ঘটবে, যেমন Python-এর [`NameError`] বা Javascript-এর [`ReferenceError`]। কিন্তু exception-এর একটা দাম আছে। interpreter-এ চলা প্রোগ্রাম যতবার কোনো variable পড়ে, ততবার interpreter-কে যাচাই করতে হয় variable-টি সংজ্ঞায়িত কি না।

Rust-এর লক্ষ্য হলো প্রোগ্রামকে এমন দক্ষ binary-তে compile করা, যাতে runtime-এ যত কম সম্ভব যাচাই লাগে। তাই কোনো variable ব্যবহারের আগে সংজ্ঞায়িত হয়েছে কি না, তা Rust *runtime*-এ যাচাই করে না। বরং Rust তা যাচাই করে *compile-time*-এ। অনিরাপদ প্রোগ্রামটি compile করার চেষ্টা করলে এই error পাবেন:

```text
error[E0425]: cannot find value `x` in this scope
 --> src/main.rs:8:10
  |
8 |     read(x); // oh no! x isn't defined!
  |          ^ not found in this scope
```

আপনার হয়তো মনে হচ্ছে, variable ব্যবহারের আগে সেটি সংজ্ঞায়িত কি না Rust তা নিশ্চিত করলে ভালোই হয়। কিন্তু কেন? নিয়মটির যৌক্তিকতা বুঝতে আমাদের প্রশ্ন করতে হবে: **Rust যদি একটি বাতিল করা প্রোগ্রামকেও compile হতে দিত, তাহলে কী ঘটত?**

প্রথমে দেখা যাক নিরাপদ প্রোগ্রামটি কীভাবে compile হয় এবং চলে। [x86](https://en.wikipedia.org/wiki/X86) architecture-এর প্রসেসরযুক্ত কম্পিউটারে Rust নিরাপদ প্রোগ্রামের `main` function-এর জন্য নিচের assembly code (প্রসেসরের নির্দেশের মানুষের পড়ার উপযোগী রূপ) তৈরি করে ([পুরো assembly code এখানে দেখুন](https://rust.godbolt.org/z/xnT1fzsqv)):

```x86asm
main:
    ; ...
    mov     edi, 1
    call    read
    ; ...
```

> _দ্রষ্টব্য_: assembly code-এর সঙ্গে পরিচিত না হলেও কোনো সমস্যা নেই! Rust ভেতরে ভেতরে আসলে কীভাবে কাজ করে, শুধু তা দেখাতে এই অংশে assembly-র কয়েকটি উদাহরণ আছে। Rust বুঝতে সাধারণত assembly জানার দরকার নেই।

এই assembly code যা করবে:

- `true` বোঝানো সংখ্যা 1-কে `edi` নামের একটি "register" (এক ধরনের assembly variable)-এ রাখবে।
- `read` function call করবে, যা আশা করে তার প্রথম argument `y` থাকবে `edi` register-এ।

অনিরাপদ function-টি compile হতে দিলে তার assembly হয়তো এরকম দেখাত:

```x86asm
main:
    ; ...
    call    read
    mov     edi, 1    ; mov is after call
    ; ...
```

এই প্রোগ্রামটি অনিরাপদ, কারণ `read` আশা করবে `edi` একটি boolean হবে, অর্থাৎ হয় সংখ্যা `0` নয়তো `1`। কিন্তু `edi`-তে যা খুশি থাকতে পারে: `2`, `100`, `0x1337BEEF`। `read` যখনই কোনো কাজে তার argument `y` ব্যবহার করতে চাইবে, সঙ্গে সঙ্গে ঘটবে _**UNDEFINED BEHAVIOR!**_ (অনির্ধারিত আচরণ!)

`y` যখন `true` বা `false` কোনোটাই নয়, তখন `if y { .. }` চালাতে গেলে কী ঘটবে, Rust তা নির্দিষ্ট করে না। ওই *আচরণ*, অর্থাৎ নির্দেশটি চালানোর পর কী ঘটে, তা *অনির্ধারিত*। কিছু একটা ঘটবেই, যেমন:

- কোড ক্র্যাশ না করে চলে, আর কেউ কোনো সমস্যা টেরই পায় না।
- [segmentation fault](https://en.wikipedia.org/wiki/Segmentation_fault) (অবৈধ মেমোরিতে ঢোকার ফলে হওয়া error) বা অপারেটিং সিস্টেমের অন্য কোনো error-এর কারণে কোড সঙ্গে সঙ্গে ক্র্যাশ করে।
- কোড ক্র্যাশ না করে চলতে থাকে, যতক্ষণ না কোনো অসৎ উদ্দেশ্যের লোক ঠিক এমন ইনপুট তৈরি করে, যা দিয়ে আপনার production database মুছে দেওয়া যায়, ব্যাকআপ মুছে নতুন করে লেখা যায়, আর আপনার টিফিনের টাকাও চুরি হয়ে যায়।

**Rust-এর একটি মৌলিক লক্ষ্য হলো নিশ্চিত করা যে আপনার প্রোগ্রামে কখনো undefined behavior (অনির্ধারিত আচরণ, যার ফল ভাষা নির্দিষ্ট করে না) থাকবে না।** "নিরাপত্তা" বলতে এটিই বোঝায়। মেমোরিতে সরাসরি প্রবেশাধিকার থাকা low-level (নিম্ন-স্তরের) প্রোগ্রামের জন্য undefined behavior বিশেষভাবে বিপজ্জনক। low-level সিস্টেমে [রিপোর্ট হওয়া নিরাপত্তা-দুর্বলতার প্রায় 70%](https://msrc.microsoft.com/blog/2019/07/a-proactive-approach-to-more-secure-code/)-এর কারণ memory corruption (মেমোরির ডেটা অবৈধভাবে নষ্ট হওয়া), যা undefined behavior-এরই একটি রূপ।

Rust-এর একটি দ্বিতীয় লক্ষ্য হলো undefined behavior _run-time_-এর বদলে _compile-time_-এ আটকানো। এই লক্ষ্যের পেছনে দুটি কারণ আছে:

1. compile-time-এ বাগ ধরা মানে production-এ সেই বাগগুলো এড়ানো, ফলে আপনার সফটওয়্যারের নির্ভরযোগ্যতা বাড়ে।
2. compile-time-এ বাগ ধরা মানে সেই বাগগুলোর জন্য runtime-এ কম যাচাই, ফলে আপনার সফটওয়্যারের পারফরম্যান্স বাড়ে।

Rust সব বাগ আটকাতে পারে না। কোনো অ্যাপ্লিকেশন যদি সবার জন্য খোলা, পরিচয় যাচাইহীন একটি `/delete-production-database` endpoint (API-র একটি নির্দিষ্ট ঠিকানা) রেখে দেয়, তাহলে database মুছতে অসৎ লোকের কোনো সন্দেহজনক if-statement-এর দরকার নেই। তবু কম সুরক্ষাযুক্ত ভাষার তুলনায় Rust-এর সুরক্ষাগুলো সম্ভবত প্রোগ্রামকে বেশি নিরাপদ করে, যেমনটি দেখেছে [Google-এর Android টিম](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)।

### memory safety-র নিয়মশৃঙ্খলা হিসেবে ownership {#ownership-as-a-discipline-for-memory-safety}

নিরাপত্তা যেহেতু undefined behavior-এর অনুপস্থিতি, আর ownership যেহেতু নিরাপত্তা নিয়েই, তাই ownership বুঝতে হবে সেটি কোন কোন undefined behavior আটকায় তার ভিত্তিতে। Rust Reference ["Behavior considered undefined"](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)-এর একটি বড় তালিকা রাখে। আপাতত আমরা একটি শ্রেণিতে মনোযোগ দেব: মেমোরির ওপর operation।

মেমোরি হলো সেই জায়গা, যেখানে প্রোগ্রাম চলার সময় ডেটা রাখা হয়। মেমোরি নিয়ে ভাবার অনেক উপায় আছে:

- systems programming (সিস্টেম-স্তরের প্রোগ্রামিং)-এর সঙ্গে পরিচিত না হলে আপনি হয়তো মেমোরিকে high-level (উচ্চ-স্তরের) দৃষ্টিতে ভাবেন, যেমন "মেমোরি মানে আমার কম্পিউটারের RAM" বা "মেমোরি হলো সেই জিনিস, যা বেশি ডেটা লোড করলে ফুরিয়ে যায়"।
- systems programming-এর সঙ্গে পরিচিত হলে আপনি হয়তো মেমোরিকে low-level দৃষ্টিতে ভাবেন, যেমন "মেমোরি হলো byte-এর একটি array" বা "মেমোরি হলো `malloc` থেকে ফিরে পাওয়া pointer-গুলো"।

মেমোরির এই দুটি মডেলই _সঠিক_, কিন্তু Rust কীভাবে কাজ করে তা ভাবার জন্য _কাজের_ নয়। high-level মডেলটি Rust-এর কাজ ব্যাখ্যা করার জন্য অতিরিক্ত বিমূর্ত। যেমন, আপনাকে pointer-এর ধারণা বুঝতে হবে। আর low-level মডেলটি Rust-এর কাজ ব্যাখ্যা করার জন্য অতিরিক্ত বাস্তব। যেমন, Rust আপনাকে মেমোরিকে byte-এর array হিসেবে ব্যাখ্যা করতে দেয় না।

Rust মেমোরি নিয়ে ভাবার একটি নির্দিষ্ট উপায় দেয়। সেই চিন্তাধারার ভেতরে নিরাপদে মেমোরি ব্যবহারের নিয়মশৃঙ্খলাই হলো ownership। এই অধ্যায়ের বাকি অংশে Rust-এর মেমোরি-মডেল ব্যাখ্যা করা হবে।

### variable থাকে stack-এ {#variables-live-in-the-stack}

এখানে অধ্যায় 3.3-এ দেখা প্রোগ্রামের মতো একটি প্রোগ্রাম আছে, যা একটি সংখ্যা `n` সংজ্ঞায়িত করে এবং `n`-এর ওপর `plus_one` function call করে। প্রোগ্রামের নিচে একটি নতুন ধরনের ডায়াগ্রাম আছে। এই ডায়াগ্রাম প্রোগ্রাম চলার সময় চিহ্নিত তিনটি জায়গায় মেমোরিতে কী আছে, তা দেখায়।

```aquascope,interpreter,horizontal
fn main() {
    let n = 5;`[]`
    let y = plus_one(n);`[]`
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    `[]`x + 1
}
```

variable থাকে **frame**-এ। frame হলো একটি scope (কোডের যে অংশে একটি নাম ব্যবহার করা যায়)-এর ভেতরে variable থেকে মানের একটি ম্যাপিং, যেমন একটি function-এর ভেতরে। যেমন:

- L1 অবস্থানে `main`-এর frame-এ আছে `n = 5`।
- L2-তে `plus_one`-এর frame-এ আছে `x = 5`।
- L3 অবস্থানে `main`-এর frame-এ আছে `n = 5; y = 6`।

frame-গুলো সাজানো থাকে বর্তমানে call হওয়া function-গুলোর একটি **stack** (ক্রমানুসারে মান রাখার মেমোরি-অংশ)-এ। যেমন, L2-তে `main`-এর frame-টি call হওয়া function `plus_one`-এর frame-এর ওপরে বসে আছে। একটি function return করার পর Rust সেই function-এর frame deallocate (বরাদ্দ করা মেমোরি ফেরত দেওয়া) করে। (deallocation-কে **freeing** বা **dropping**-ও বলা হয়, আর আমরা এই শব্দগুলো অদলবদল করে ব্যবহার করি।) frame-এর এই ক্রমকে stack বলা হয়, কারণ সবচেয়ে শেষে যোগ হওয়া frame-টিই সবসময় সবার আগে free হয়।

> _দ্রষ্টব্য:_ এই মেমোরি-মডেল Rust আসলে কীভাবে কাজ করে তার পুরোটা বর্ণনা করে না! assembly code-এ আগে যেমন দেখেছি, Rust compiler `n` বা `x`-কে stack frame-এর বদলে একটি register-এ রাখতে পারে। তবে এই পার্থক্যটি implementation-এর খুঁটিনাটি। এটি Rust-এর নিরাপত্তা সম্পর্কে আপনার বোঝাপড়া বদলানোর কথা নয়, তাই আমরা শুধু frame-এ থাকা variable-এর সহজ ক্ষেত্রটিতে মনোযোগ দিতে পারি।

কোনো expression যখন একটি variable পড়ে, তখন variable-টির মান stack frame-এ তার ঘর থেকে কপি করা হয়। যেমন, এই প্রোগ্রামটি চালালে:

```aquascope,interpreter,horizontal
#fn main() {
let a = 5;`[]`
let mut b = a;`[]`
b += 1;`[]`
#}
```

`a`-এর মান `b`-তে কপি হয়, আর `b` বদলানোর পরও `a` অপরিবর্তিত থাকে।

### box থাকে heap-এ {#boxes-live-in-the-heap}

তবে ডেটা কপি করতে অনেক মেমোরি লাগতে পারে। যেমন, এখানে একটু আলাদা একটি প্রোগ্রাম। এই প্রোগ্রামটি 10 লাখ উপাদানের একটি array কপি করে:

```aquascope,interpreter
#fn main() {
let a = [0; 1_000_000];`[]`
let b = a;`[]`
#}
```

লক্ষ করুন, `a`-কে `b`-তে কপি করার ফলে `main` frame-এ 20 লাখ উপাদান জমা হয়েছে। 

ডেটা কপি না করে তার প্রবেশাধিকার হস্তান্তর করতে Rust **pointer** ব্যবহার করে। pointer (মেমোরির একটি অবস্থান নির্দেশকারী মান) হলো এমন একটি মান, যা মেমোরির একটি অবস্থান বর্ণনা করে। pointer যে মানটিকে নির্দেশ করে, তাকে বলা হয় তার **pointee।** pointer তৈরির একটি প্রচলিত উপায় হলো **heap** (প্রয়োজনমতো জায়গা বরাদ্দের মেমোরি-অংশ)-এ মেমোরি allocate (মেমোরিতে জায়গা বরাদ্দ) করা।  heap হলো মেমোরির একটি আলাদা অংশ, যেখানে ডেটা অনির্দিষ্টকাল থাকতে পারে। heap-এর ডেটা কোনো নির্দিষ্ট stack frame-এর সঙ্গে বাঁধা নয়। heap-এ ডেটা রাখার জন্য Rust [`Box`](https://doc.rust-lang.org/std/boxed/index.html) নামের একটি গঠন দেয়। যেমন, 10 লাখ উপাদানের array-টিকে আমরা `Box::new` দিয়ে মুড়ে দিতে পারি, এভাবে:

```aquascope,interpreter
#fn main() {
let a = Box::new([0; 1_000_000]);`[]`
let b = a;`[]`
#}
```

লক্ষ করুন, এখন যেকোনো সময়ে একটিমাত্র array থাকে। L1-এ `a`-এর মান হলো heap-এর ভেতরের array-টির দিকে একটি pointer (তীরসহ একটি বিন্দু দিয়ে দেখানো)। `let b = a` statement pointer-টিকে `a` থেকে `b`-তে কপি করে, কিন্তু যে ডেটাকে নির্দেশ করা হচ্ছে, তা কপি হয় না। লক্ষ করুন, `a` এখন ধূসর হয়ে গেছে, কারণ এটি *move* হয়েছে &mdash; এর অর্থ কী, তা আমরা একটু পরেই দেখব।

{{#quiz ../quizzes/ch04-01-ownership-sec1-stackheap.toml}}

### Rust নিজ হাতে memory management করতে দেয় না {#rust-does-not-permit-manual-memory-management}

memory management (মেমোরি বরাদ্দ ও ফেরতের ব্যবস্থাপনা) হলো মেমোরি allocate আর deallocate করার প্রক্রিয়া। অন্যভাবে বললে, এটি অব্যবহৃত মেমোরি খুঁজে বের করা, আর পরে সেই মেমোরির ব্যবহার শেষ হলে তা ফেরত দেওয়ার প্রক্রিয়া। stack frame-গুলো Rust আপনা-আপনিই সামলায়। কোনো function call হলে Rust call হওয়া function-টির জন্য একটি stack frame allocate করে। call শেষ হলে Rust সেই stack frame deallocate করে।

ওপরে যেমন দেখেছি, `Box::new(..)` call করলে heap-এর ডেটা allocate হয়। কিন্তু heap-এর ডেটা deallocate হয় কখন? কল্পনা করুন, Rust-এ একটি `free()` function আছে, যা heap-এর একটি বরাদ্দ মুক্ত করে। কল্পনা করুন, Rust প্রোগ্রামারকে যখন খুশি `free` call করতে দেয়। এ ধরনের "নিজ হাতে" memory management সহজেই বাগের জন্ম দেয়। যেমন, আমরা free করা মেমোরির একটি pointer পড়ে ফেলতে পারি:

```aquascope,interpreter,shouldFail
#fn free<T>(_t: T) {}
#fn main() {
let b = Box::new([0; 100]);`[]`
free(b);`[]`
assert!(b[0] == 0);`[]`
#}
```

> *দ্রষ্টব্য:* আপনি হয়তো ভাবছেন, compile হয় না এমন এই Rust প্রোগ্রামটি আমরা চালাচ্ছি কীভাবে। শেখানোর উদ্দেশ্যে আমরা [বিশেষ টুল](https://github.com/cognitive-engineering-lab/aquascope) ব্যবহার করি, যা borrow checker বন্ধ থাকলে Rust কেমন চলত তা অনুকরণ করে দেখায়। এভাবে আমরা "যদি এমন হতো" ধরনের প্রশ্নের উত্তর দিতে পারি, যেমন: Rust যদি এই অনিরাপদ প্রোগ্রামটিকে compile হতে দিত, তাহলে কী হতো?

এখানে আমরা heap-এ একটি array allocate করি। তারপর `free(b)` call করি, যা `b`-এর heap মেমোরি deallocate করে। ফলে `b`-এর মান হয়ে যায় অবৈধ মেমোরির দিকে একটি pointer, যা আমরা "⦻" আইকন দিয়ে দেখাই। এখনো কোনো undefined behavior ঘটেনি! L2-তে প্রোগ্রামটি এখনো নিরাপদ। একটি অবৈধ pointer থাকা মানেই সমস্যা, এমন নয়।

undefined behavior ঘটে তখন, যখন আমরা `b[0]` পড়ে pointer-টি *ব্যবহার* করার চেষ্টা করি। এতে অবৈধ মেমোরিতে ঢোকার চেষ্টা হবে, যার ফলে প্রোগ্রাম ক্র্যাশ করতে পারে। অথবা আরও খারাপ, ক্র্যাশ না করে এলোমেলো ডেটা ফেরত দিতে পারে। তাই এই প্রোগ্রামটি **অনিরাপদ**।

Rust প্রোগ্রামকে নিজ হাতে মেমোরি deallocate করতে দেয় না। এই নীতি ওপরে দেখানো ধরনের undefined behavior এড়ায়।

### box-এর owner deallocation সামলায় {#a-boxs-owner-manages-deallocation}

বরং Rust _আপনা-আপনিই_ একটি box (heap-এর ডেটার মালিক এমন pointer)-এর heap মেমোরি free করে। box free করার Rust-এর নীতির একটি _প্রায়_ সঠিক বর্ণনা এখানে দেওয়া হলো:

> **box deallocation-এর নীতি (Box deallocation principle, প্রায় সঠিক):** কোনো variable যদি একটি box-এর সঙ্গে bind থাকে, তাহলে Rust যখন variable-টির frame deallocate করে, তখন Rust box-টির heap মেমোরিও deallocate করে।

যেমন, চলুন এমন একটি প্রোগ্রাম ধাপে ধাপে দেখি, যা একটি box allocate করে আবার free করে:

```aquascope,interpreter,horizontal
fn main() {
    let a_num = 4;`[]`
    make_and_drop();`[]`
}

fn make_and_drop() {
    let a_box = Box::new(5);`[]`
}
```

L1-এ, `make_and_drop` call করার আগে, মেমোরির অবস্থা বলতে শুধু `main`-এর stack frame। তারপর L2-তে, `make_and_drop` call হওয়ার সময়, `a_box` heap-এর `5`-কে নির্দেশ করে। `make_and_drop` শেষ হলে Rust এর stack frame deallocate করে। `make_and_drop`-এ `a_box` variable-টি আছে, তাই Rust `a_box`-এর heap ডেটাও deallocate করে। ফলে L3-তে heap খালি।

box-এর heap মেমোরি সফলভাবে সামলানো হয়েছে। কিন্তু আমরা যদি এই ব্যবস্থার অপব্যবহার করি? আগের উদাহরণে ফিরে যাই: একটি box-এর সঙ্গে দুটি variable bind করলে কী ঘটে?

```rust,ignore
# fn main() {
let a = Box::new([0; 1_000_000]);
let b = a;
# }
```

box-এ রাখা array-টি এখন `a` আর `b` দুটোর সঙ্গেই bind হয়েছে। আমাদের "প্রায় সঠিক" নীতি অনুযায়ী, Rust দুটি variable-এর হয়ে box-এর heap মেমোরি *দুবার* free করার চেষ্টা করবে। সেটাও undefined behavior!

এই পরিস্থিতি এড়াতে আমরা শেষ পর্যন্ত পৌঁছাই ownership-এ। `a` যখন `Box::new([0; 1_000_000])`-এর সঙ্গে bind হয়, তখন আমরা বলি `a` box-টিকে **own** করে, অর্থাৎ এটি box-টির **owner** (মালিক)। `let b = a` statement box-টির ownership `a` থেকে `b`-তে **move** (ownership এক variable থেকে আরেকটিতে সরানো) করে। এই ধারণাগুলোর আলোকে box free করার Rust-এর নীতিকে আরও সঠিকভাবে বর্ণনা করা যায় এভাবে:

> **box deallocation-এর নীতি (সম্পূর্ণ সঠিক):** কোনো variable যদি একটি box own করে, তাহলে Rust যখন variable-টির frame deallocate করে, তখন Rust box-টির heap মেমোরিও deallocate করে।

ওপরের উদাহরণে `b` box-এ রাখা array-টি own করে। তাই scope শেষ হলে Rust box-টি মাত্র একবার deallocate করে, `a`-এর হয়ে নয়, `b`-এর হয়ে।


### collection box ব্যবহার করে {#collections-use-boxes}

Rust-এর data structure (ডেটা সাজানোর কাঠামো)[^boxed-data-structures], যেমন [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html), [`String`](https://doc.rust-lang.org/std/string/struct.String.html) আর [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html), পরিবর্তনশীল সংখ্যক উপাদান রাখতে box ব্যবহার করে। যেমন, এখানে একটি প্রোগ্রাম আছে, যা একটি string তৈরি করে, move করে এবং mutate (ডেটা বদলানো) করে:

```aquascope,interpreter,horizontal
fn main() {
    let first = String::from("Ferris");`[]`
    let full = add_suffix(first);`[]`
    println!("{full}");
}

fn add_suffix(mut name: String) -> String {
    `[]`name.push_str(" Jr.");`[]`
    name
}
```

এই প্রোগ্রামটি একটু জটিল, তাই প্রতিটি ধাপ ভালো করে খেয়াল করুন:

1. L1-এ "Ferris" string-টি heap-এ allocate হয়েছে। এর owner `first`।
2. L2-তে `add_suffix(first)` function call হয়েছে। এটি string-টির ownership `first` থেকে `name`-এ move করে। string-এর ডেটা কপি হয় না, কিন্তু ডেটার pointer-টি কপি হয়।
3. L3-তে `name.push_str(" Jr.")` function string-টির heap বরাদ্দের আকার বদলায়। এতে তিনটি জিনিস ঘটে। প্রথমত, এটি আরও বড় একটি নতুন বরাদ্দ তৈরি করে। দ্বিতীয়ত, নতুন বরাদ্দে "Ferris Jr." লেখে। তৃতীয়ত, মূল heap মেমোরি free করে। `first` এখন deallocate হওয়া মেমোরিকে নির্দেশ করে।
4. L4-এ `add_suffix`-এর frame আর নেই। এই function `name` return করেছে, ফলে string-টির ownership `full`-এ হস্তান্তর হয়েছে।


### move হওয়ার পর variable ব্যবহার করা যায় না {#variables-cannot-be-used-after-being-moved}

string-এর এই প্রোগ্রামটি ownership-এর একটি মূল নিরাপত্তা-নীতি বুঝতে সাহায্য করে। কল্পনা করুন, `add_suffix` call করার পর `main`-এ `first` ব্যবহার করা হলো। এমন একটি প্রোগ্রাম অনুকরণ করে দেখা যাক, এর ফলে কী undefined behavior ঘটে:

```aquascope,interpreter,shouldFail
fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    println!("{full}, originally {first}");`[]` // first is now used here
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```

`add_suffix` call করার পর `first` deallocate হওয়া মেমোরিকে নির্দেশ করে। তাই `println!`-এ `first` পড়া হবে memory safety-র লঙ্ঘন (undefined behavior)। মনে রাখবেন: `first` deallocate হওয়া মেমোরিকে নির্দেশ করছে, এটা সমস্যা নয়। সমস্যা হলো, অবৈধ হয়ে যাওয়ার পর আমরা `first`-কে *ব্যবহার* করার চেষ্টা করেছি।

সৌভাগ্যক্রমে, Rust এই প্রোগ্রাম compile করতে রাজি হবে না, আর নিচের error দেবে:

```text
error[E0382]: borrow of moved value: `first`
 --> test.rs:4:35
  |
2 |     let first = String::from("Ferris");
  |         ----- move occurs because `first` has type `String`, which does not implement the `Copy` trait
3 |     let full = add_suffix(first);
  |                           ----- value moved here
4 |     println!("{full}, originally {first}"); // first is now used here
  |                                   ^^^^^ value borrowed here after move
```

চলুন error-টির ধাপগুলো দেখি। Rust বলছে, লাইন 3-এ `add_suffix(first)` call করার সময় `first` move হয়েছে। error-টি স্পষ্ট করে দিচ্ছে যে `first` move হয়েছে, কারণ এর type `String`, যা `Copy` implement করে না। `Copy` নিয়ে আমরা শিগগিরই আলোচনা করব &mdash; সংক্ষেপে, `String`-এর বদলে `i32` ব্যবহার করলে এই error পেতেন না। সবশেষে error বলছে যে move হওয়ার পর আমরা `first` ব্যবহার করেছি (এটি "borrowed" হয়েছে, যা নিয়ে পরের অংশে আলোচনা করব)।

তাহলে আপনি একটি variable move করলে Rust পরে সেই variable ব্যবহার করতে দেবে না। আরও সাধারণভাবে, compiler এই নীতি বলবৎ করবে:

> **move হওয়া heap ডেটার নীতি (Moved heap data principle):** কোনো variable `x` যদি heap ডেটার ownership অন্য একটি variable `y`-তে move করে, তাহলে move-এর পর `x` আর ব্যবহার করা যাবে না।

এখন ownership, move আর নিরাপত্তার সম্পর্কটি আপনার চোখে পড়তে শুরু করার কথা। heap ডেটার ownership move করলে deallocate হওয়া মেমোরি পড়ার ফলে ঘটা undefined behavior এড়ানো যায়।

### clone করলে move এড়ানো যায় {#cloning-avoids-moves}

ডেটা move করা এড়ানোর একটি উপায় হলো `.clone()` method দিয়ে সেটিকে *clone* (ডেটার হুবহু নতুন কপি তৈরি) করা। যেমন, আগের প্রোগ্রামের নিরাপত্তার সমস্যাটি আমরা একটি clone দিয়ে ঠিক করতে পারি:

```aquascope,interpreter
fn main() {
    let first = String::from("Ferris");
    let first_clone = first.clone();`[]`
    let full = add_suffix(first_clone);`[]`
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}
```

লক্ষ করুন, L1-এ `first_clone` `first`-এর pointer-টি "shallow" কপি (শুধু pointer কপি) করেনি, বরং string-এর ডেটা একটি নতুন heap বরাদ্দে "deep" কপি (পুরো ডেটা কপি) করেছে। তাই L2-তে `first_clone` `add_suffix`-এর কারণে move ও অবৈধ হয়ে গেলেও মূল `first` variable অপরিবর্তিত থাকে। `first` ব্যবহার চালিয়ে যাওয়া নিরাপদ।

{{#quiz ../quizzes/ch04-01-ownership-sec2-moves.toml}}

### সারসংক্ষেপ {#summary}

ownership মূলত heap ব্যবস্থাপনার একটি নিয়মশৃঙ্খলা:[^pointer-management]

- heap-এর সব ডেটার ঠিক একটি করে variable owner থাকতে হবে।
- owner scope-এর বাইরে চলে গেলে Rust heap-এর ডেটা deallocate করে।
- move-এর মাধ্যমে ownership হস্তান্তর করা যায়, আর move ঘটে assignment ও function call-এর সময়।
- heap-এর ডেটায় শুধু তার বর্তমান owner-এর মাধ্যমেই পৌঁছানো যায়, আগের কোনো owner-এর মাধ্যমে নয়।

আমরা শুধু Rust-এর সুরক্ষাগুলো _কীভাবে_ কাজ করে তা নয়, সেগুলো _কেন_ undefined behavior এড়ায়, তাতেও জোর দিয়েছি। Rust compiler থেকে কোনো error message পেলে Rust কেন আপত্তি করছে তা না বুঝলে সহজেই বিরক্তি আসে। এই ধারণাগত ভিত্তিগুলো Rust-এর error message বুঝতে আপনাকে সাহায্য করবে।  আরও Rust-সুলভ API নকশা করতেও এগুলো সাহায্য করবে।

[^boxed-data-structures]: এই data structure-গুলো সরাসরি `Box` type ব্যবহার করে না। যেমন, `String` implement করা হয়েছে `Vec` দিয়ে, আর `Vec` implement করা হয়েছে `Box`-এর বদলে [`RawVec`](https://doc.rust-lang.org/nomicon/vec/vec-raw.html) দিয়ে। তবে `RawVec`-এর মতো type-গুলোও box-এর মতোই: এরা heap-এর মেমোরি own করে।

[^pointer-management]: আরেক অর্থে ownership হলো *pointer* ব্যবস্থাপনার একটি নিয়মশৃঙ্খলা। তবে heap ছাড়া অন্য কোথাও pointer কীভাবে তৈরি করতে হয়, তা আমরা এখনো বলিনি। পরের অংশে সেখানে পৌঁছাব।

[`NameError`]: https://docs.python.org/3/library/exceptions.html#NameError
[`ReferenceError`]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/ReferenceError
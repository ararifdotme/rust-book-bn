## reference ও borrowing {#references-and-borrowing}

ownership, box আর move মিলে heap নিয়ে নিরাপদে প্রোগ্রামিংয়ের একটি ভিত্তি তৈরি করে। তবে শুধু move-নির্ভর API ব্যবহার করা অসুবিধাজনক হতে পারে। যেমন, ধরুন আপনি কিছু string দুবার পড়তে চান:

```aquascope,interpreter,shouldFail,horizontal
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);`[]`
    let s = format!("{} {}", m1, m2);`[]` // Error: m1 and m2 are moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);`[]`
}
```

এই উদাহরণে `greet` call করলে `m1` আর `m2`-এর ডেটা `greet`-এর parameter-গুলোতে move হয়ে যায়। দুটি string-ই `greet`-এর শেষে drop হয়ে যায়, তাই `main`-এর ভেতরে আর ব্যবহার করা যায় না। `format!(..)` operation-এর মতো করে সেগুলো পড়ার চেষ্টা করলে তা হবে undefined behavior। তাই Rust compiler আগের অংশে দেখা একই error দিয়ে এই প্রোগ্রাম বাতিল করে দেয়:

```text
error[E0382]: borrow of moved value: `m1`
 --> test.rs:5:30
 (...rest of the error...)
```

move-এর এই আচরণ ভীষণ অসুবিধাজনক। প্রোগ্রামে প্রায়ই একটি string একাধিকবার ব্যবহার করার দরকার হয়। একটি বিকল্প `greet` string-গুলোর ownership ফেরত দিতে পারে, এভাবে:

```aquascope,interpreter,horizontal
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");`[]`
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again);`[]`
}

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}
```

তবে এ ধরনের প্রোগ্রাম বেশ লম্বা-চওড়া হয়ে যায়। reference-এর মাধ্যমে Rust move ছাড়াই পড়া আর লেখার একটি সংক্ষিপ্ত উপায় দেয়।

### reference হলো non-owning pointer {#references-are-non-owning-pointers}

একটি **reference** (ডেটা কপি না করে তার দিকে নির্দেশ করার উপায়) হলো এক ধরনের pointer। এখানে reference-এর একটি উদাহরণ আছে, যা আমাদের `greet` প্রোগ্রামটিকে আরও সুবিধাজনকভাবে নতুন করে লেখে:

```aquascope,interpreter,horizontal
fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");`[]`
    greet(&m1, &m2);`[]` // note the ampersands
    let s = format!("{} {}", m1, m2);
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    `[]`println!("{} {}!", g1, g2);
}
```

`&m1` expression ampersand operator ব্যবহার করে `m1`-এর একটি reference তৈরি করে (বা একে "borrow" করে)। `greet`-এর parameter `g1`-এর type বদলে `&String` করা হয়েছে, যার মানে "একটি `String`-এর reference"।

<!-- At runtime, the references look like this:

<img src="img/experiment/ch04-02-stack1.jpg" class="center" width="350" /> -->

L2-তে লক্ষ করুন, `g1` থেকে "Hello" string পর্যন্ত যেতে দুটি ধাপ লাগে। `g1` একটি reference, যা stack-এ থাকা `m1`-কে নির্দেশ করে, আর `m1` হলো একটি String, যার ভেতরে একটি box আছে, যা heap-এ থাকা "Hello"-কে নির্দেশ করে।

`m1` heap ডেটা "Hello"-এর owner হলেও `g1` `m1` বা "Hello" কোনোটিরই owner _নয়_। তাই `greet` শেষ হয়ে প্রোগ্রাম L3-তে পৌঁছানোর পরও কোনো heap ডেটা deallocate হয়নি। শুধু `greet`-এর stack frame মুছে যায়। এই ঘটনা আমাদের *box deallocation-এর নীতি*-র সঙ্গে সামঞ্জস্যপূর্ণ। `g1` "Hello"-এর owner ছিল না বলে Rust `g1`-এর হয়ে "Hello" deallocate করেনি।

reference হলো **non-owning pointer** (ডেটার মালিক নয় এমন pointer), কারণ এরা যে ডেটাকে নির্দেশ করে, তার owner নয়।

### pointer dereference করলে তার ডেটায় পৌঁছানো যায় {#dereferencing-a-pointer-accesses-its-data}

box আর string নিয়ে আগের উদাহরণগুলোতে দেখানো হয়নি, Rust কীভাবে একটি pointer "অনুসরণ" করে তার ডেটায় পৌঁছায়। যেমন, `println!` macro (কোড তৈরি করে এমন কোড) রহস্যজনকভাবে `String` type-এর owned string আর `&String` type-এর string reference—দুটোর জন্যই কাজ করেছে। এর পেছনের কৌশল হলো **dereference** (pointer ধরে তার ডেটায় পৌঁছানো) operator, যা লেখা হয় একটি তারকাচিহ্ন (`*`) দিয়ে। যেমন, এখানে একটি প্রোগ্রাম আছে, যা কয়েকটি আলাদা উপায়ে dereference ব্যবহার করে:

```aquascope,interpreter
# fn main() {
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;         // *x reads the heap value, so a = 1
*x += 1;                 // *x on the left-side modifies the heap value,
                         //     so x points to the value 2

let r1: &Box<i32> = &x;  // r1 points to x on the stack
let b: i32 = **r1;       // two dereferences get us to the heap value

let r2: &i32 = &*x;      // r2 points to the heap value directly
let c: i32 = *r2;`[]`    // so only one dereference is needed to read it
# }
```

stack-এ থাকা `x`-কে নির্দেশ করা `r1` আর heap-এর মান `2`-কে নির্দেশ করা `r2`-এর পার্থক্য লক্ষ করুন।

Rust কোড পড়ার সময় আপনি হয়তো dereference operator খুব একটা দেখবেন না। কিছু কিছু ক্ষেত্রে Rust নিজে থেকেই dereference আর reference বসিয়ে দেয়, যেমন dot operator দিয়ে method call করার সময়। যেমন, এই প্রোগ্রাম [`i32::abs`](https://doc.rust-lang.org/std/primitive.i32.html#method.abs) (পরম মান) আর [`str::len`](https://doc.rust-lang.org/std/primitive.str.html#method.len) (string-এর দৈর্ঘ্য) function call করার দুটি সমতুল্য উপায় দেখায়:

```rust,ignore
# fn main()  {
let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); // স্পষ্ট dereference
let x_abs2 = x.abs();      // অন্তর্নিহিত dereference
assert_eq!(x_abs1, x_abs2);

let r: &Box<i32> = &x;
let r_abs1 = i32::abs(**r); // স্পষ্ট dereference (দুবার)
let r_abs2 = r.abs();       // অন্তর্নিহিত dereference (দুবার)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); // স্পষ্ট reference
let s_len2 = s.len();      // অন্তর্নিহিত reference
assert_eq!(s_len1, s_len2);
# }
```

এই উদাহরণ তিনভাবে অন্তর্নিহিত রূপান্তর দেখায়:
1. `i32::abs` function `i32` type-এর ইনপুট আশা করে। `Box<i32>` দিয়ে `abs` call করতে হলে `i32::abs(*x)`-এর মতো করে box-টিকে স্পষ্টভাবে dereference করতে পারেন। আবার `x.abs()`-এর মতো method-call syntax ব্যবহার করে box-টিকে অন্তর্নিহিতভাবেও dereference করতে পারেন। dot syntax হলো function-call syntax-এর syntactic sugar (একই কাজের সহজতর লেখার রূপ)।

2. এই অন্তর্নিহিত রূপান্তর pointer-এর একাধিক স্তরেও কাজ করে। যেমন, box-এর একটি reference `r: &Box<i32>`-এর ওপর `abs` call করলে দুটি dereference বসে যাবে।

3. এই রূপান্তর উল্টো দিকেও কাজ করে। `str::len` function একটি reference `&str` আশা করে। owned `String`-এর ওপর `len` call করলে Rust একটি borrowing operator বসিয়ে দেবে। (আসলে `String` থেকে `str`-এ আরও একটি রূপান্তর ঘটে!)

method call আর অন্তর্নিহিত রূপান্তর নিয়ে পরের অধ্যায়গুলোতে আরও বলব। আপাতত গুরুত্বপূর্ণ কথা হলো, এই রূপান্তরগুলো ঘটে method call আর `println`-এর মতো কিছু macro-র ক্ষেত্রে। আমরা Rust-এর সব "জাদু"-র রহস্য খুলে দিতে চাই, যাতে Rust কীভাবে কাজ করে তার একটি পরিষ্কার মানসিক মডেল আপনার থাকে।

{{#quiz ../quizzes/ch04-02-references-sec1-basics.toml}}

### Rust একসঙ্গে aliasing আর mutation এড়ায় {#rust-avoids-simultaneous-aliasing-and-mutation}

pointer একটি শক্তিশালী কিন্তু বিপজ্জনক বৈশিষ্ট্য, কারণ এটি **aliasing** (একই ডেটা একাধিক variable দিয়ে ব্যবহার করা) সম্ভব করে। aliasing মানে একই ডেটা বিভিন্ন variable-এর মাধ্যমে ব্যবহার করা। আলাদাভাবে aliasing ক্ষতিকর নয়। কিন্তু **mutation**-এর সঙ্গে মিললে এটি বিপর্যয়ের রেসিপি। একটি variable নানা উপায়ে আরেকটি variable-এর "পায়ের তলার মাটি সরিয়ে দিতে" পারে, যেমন:

- alias করা ডেটা deallocate করে, ফলে অন্য variable-টি deallocate হওয়া মেমোরিকে নির্দেশ করতে থাকে।
- alias করা ডেটা mutate করে, ফলে অন্য variable-টি runtime-এ যেসব বৈশিষ্ট্য আশা করে, সেগুলো অবৈধ হয়ে যায়।
- alias করা ডেটা _একসঙ্গে_ (concurrently) mutate করে, ফলে একটি data race (একাধিক thread একই ডেটা একসঙ্গে বদলালে হওয়া অনিশ্চিত আচরণ) ঘটে, আর অন্য variable-টির আচরণ অনিশ্চিত হয়ে যায়।

চলমান উদাহরণ হিসেবে আমরা vector data structure [`Vec`] ব্যবহার করা প্রোগ্রাম দেখব। নির্দিষ্ট দৈর্ঘ্যের array-র বিপরীতে vector (পরিবর্তনশীল দৈর্ঘ্যের তালিকা)-এর দৈর্ঘ্য পরিবর্তনশীল, কারণ এরা উপাদানগুলো heap-এ রাখে। যেমন, [`Vec::push`] একটি vector-এর শেষে একটি উপাদান যোগ করে, এভাবে:

```aquascope,interpreter,horizontal
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];`[]`
v.push(4);`[]`
#}
```

`vec!` macro ব্র্যাকেটের ভেতরের উপাদানগুলো দিয়ে একটি vector তৈরি করে। `v` vector-টির type হলো `Vec<i32>`। `<i32>` syntax-এর মানে vector-এর উপাদানগুলোর type `i32`।

implementation-এর একটি গুরুত্বপূর্ণ খুঁটিনাটি হলো, `v` একটি নির্দিষ্ট *ধারণক্ষমতা*-র heap array allocate করে। `Vec`-এর ভেতরে উঁকি দিয়ে আমরা নিজেরাই এই খুঁটিনাটি দেখতে পারি:

```aquascope,interpreter,horizontal,concreteTypes
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];`[]`
#}
```

> *দ্রষ্টব্য:* যেকোনো runtime ডায়াগ্রামে এই বিস্তারিত দৃশ্য চালু বা বন্ধ করতে ডায়াগ্রামের ওপরের ডান কোণের দূরবিন আইকনে ক্লিক করুন।

লক্ষ করুন, vector-টির দৈর্ঘ্য (`len`) 3 এবং ধারণক্ষমতা (`cap`) 3। vector-টি পূর্ণ ধারণক্ষমতায় আছে। তাই আমরা যখন `push` করি, vector-টিকে বড় ধারণক্ষমতার একটি নতুন বরাদ্দ তৈরি করতে হয়, সব উপাদান সেখানে কপি করতে হয়, আর মূল heap array deallocate করতে হয়। ওপরের ডায়াগ্রামে `1 2 3 4` array-টি মূল `1 2 3` array থেকে (সম্ভবত) আলাদা মেমোরি-অবস্থানে আছে।

memory safety-র সঙ্গে এটি মেলাতে চলুন reference-কে এর মধ্যে আনি। ধরুন আমরা একটি vector-এর heap ডেটার একটি reference তৈরি করলাম। তাহলে নিচের অনুকরণে যেমন দেখানো হয়েছে, একটি push সেই reference অবৈধ করে দিতে পারে:

```aquascope,interpreter,shouldFail,horizontal
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];`[]`
v.push(4);`[]`
println!("Third element is {}", *num);`[]`
#}
```

শুরুতে `v` heap-এ থাকা 3 উপাদানের একটি array-কে নির্দেশ করে। তারপর L1-এ যেমন দেখা যাচ্ছে, তৃতীয় উপাদানের একটি reference হিসেবে `num` তৈরি হয়। কিন্তু `v.push(4)` operation `v`-এর আকার বদলায়। আকার বদলানোর সময় আগের array deallocate হবে এবং একটি নতুন, বড় array allocate হবে। এই প্রক্রিয়ায় `num` অবৈধ মেমোরিকে নির্দেশ করতে থাকে। তাই L3-তে `*num` dereference করলে অবৈধ মেমোরি পড়া হয়, যা undefined behavior ঘটায়।

আরও বিমূর্তভাবে বললে, সমস্যা হলো `v` vector-টি একই সঙ্গে alias হয়েছে (reference `num` দিয়ে) এবং mutate হয়েছে (`v.push(4)` operation দিয়ে)। তাই এ ধরনের সমস্যা এড়াতে Rust একটি মৌলিক নীতি মেনে চলে:

> **pointer নিরাপত্তার নীতি (Pointer Safety Principle)**: ডেটা কখনো একই সঙ্গে alias ও mutate হওয়া উচিত নয়।

ডেটা alias হতে পারে। ডেটা mutate হতে পারে। কিন্তু ডেটা একই সঙ্গে alias _এবং_ mutate হতে পারে _না_। যেমন, box (owned pointer)-এর ক্ষেত্রে Rust aliasing নিষিদ্ধ করে এই নীতি বলবৎ করে। একটি variable থেকে আরেকটিতে box assign করলে ownership move হয়, ফলে আগের variable অবৈধ হয়ে যায়। owned ডেটায় শুধু owner-এর মাধ্যমেই পৌঁছানো যায় &mdash; কোনো alias নেই।

তবে reference যেহেতু non-owning pointer, তাই *pointer নিরাপত্তার নীতি* নিশ্চিত করতে এদের জন্য box-এর চেয়ে আলাদা নিয়ম দরকার। নকশাগতভাবেই reference-এর উদ্দেশ্য সাময়িকভাবে alias তৈরি করা। এই অংশের বাকিটায় আমরা ব্যাখ্যা করব, **borrow checker** (borrow-এর নিয়ম যাচাই করা compiler-এর অংশ)-এর মাধ্যমে Rust কীভাবে reference-এর নিরাপত্তা নিশ্চিত করে, তার মূল কথাগুলো।

### reference place-এর permission বদলায় {#references-change-permissions-on-places}

borrow checker-এর পেছনের মূল ভাবনা হলো, variable-এর তার ডেটার ওপর তিন ধরনের **permission** (অনুমতি) থাকে:

- **Read** (পড়া) (@Perm{read}): ডেটা অন্য জায়গায় কপি করা যায়।
- **Write** (লেখা/বদলানো) (@Perm{write}): ডেটা mutate করা যায়।
- **Own** (মালিকানা) (@Perm{own}): ডেটা move বা drop করা যায়।

এই permission-গুলো runtime-এ থাকে না, শুধু compiler-এর ভেতরে থাকে। প্রোগ্রাম চালানোর আগে compiler আপনার প্রোগ্রাম নিয়ে কীভাবে "ভাবে", এগুলো তা বর্ণনা করে।

ডিফল্টভাবে একটি variable-এর তার ডেটার ওপর read/own permission (@Perm{read}@Perm{own}) থাকে। কোনো variable `let mut` দিয়ে annotate করা থাকলে তার write permission (@Perm{write})-ও থাকে। মূল ভাবনাটি হলো
**reference সাময়িকভাবে এই permission-গুলো সরিয়ে নিতে পারে।**

এই ভাবনাটি দেখাতে চলুন ওপরের প্রোগ্রামের এমন একটি রূপের permission দেখি, যা আসলে নিরাপদ। `push`-কে `println!`-এর পরে সরিয়ে দেওয়া হয়েছে। এই প্রোগ্রামের permission-গুলো একটি নতুন ধরনের ডায়াগ্রাম দিয়ে দেখানো হয়েছে। ডায়াগ্রামটি প্রতিটি লাইনে permission-এর পরিবর্তন দেখায়।

```aquascope,permissions,stepper
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];
println!("Third element is {}", *num);
v.push(4);
#}
```

চলুন প্রতিটি লাইন ধরে দেখি:

1. `let mut v = (...)`-এর পরে `v` variable-টি initialize হয়েছে (<i class="fa fa-arrow-turn-up"></i> দিয়ে দেখানো)। এটি @Perm[gained]{read}@Perm[gained]{write}@Perm[gained]{own} permission পায় (যোগচিহ্ন পাওয়া বোঝায়)।
2. `let num = &v[2]`-এর পরে `v`-এর ডেটা `num` দ্বারা **borrow** হয়েছে (<i class="fa fa-arrow-right"></i> দিয়ে দেখানো)। তিনটি জিনিস ঘটে:
   - borrow-টি `v` থেকে @Perm[lost]{write}@Perm[lost]{own} permission সরিয়ে নেয় (স্ল্যাশ হারানো বোঝায়)। `v`-তে লেখা যায় না, এর মালিকানাও নেওয়া যায় না, তবে এখনো পড়া যায়।
   - `num` variable-টি @Perm{read}@Perm{own} permission পেয়েছে। `num`-এ লেখা যায় না (অনুপস্থিত @Perm{write} permission একটি ড্যাশ <span class="perm write">‒</span> দিয়ে দেখানো হয়েছে), কারণ এটি `let mut` হিসেবে চিহ্নিত করা হয়নি।
   - **place** `*num` @Perm{read} permission পেয়েছে।
3. `println!(...)`-এর পরে `num` আর ব্যবহার হচ্ছে না, তাই `v` আর borrow অবস্থায় নেই। ফলে:
   - `v` তার @Perm{write}@Perm{own} permission ফিরে পায় (<i class="fa fa-rotate-left"></i> দিয়ে দেখানো)।
   - `num` আর `*num` তাদের সব permission হারিয়েছে (<i class="fa fa-arrow-turn-down"></i> দিয়ে দেখানো)।
4. `v.push(4)`-এর পরে `v` আর ব্যবহার হচ্ছে না, আর এটি তার সব permission হারায়।

এরপর ডায়াগ্রামটির কয়েকটি সূক্ষ্ম দিক দেখা যাক। প্রথমত, `num` আর `*num` দুটোই কেন দেখছেন? কারণ reference-এর মাধ্যমে ডেটায় পৌঁছানো আর reference-টিকেই নাড়াচাড়া করা এক জিনিস নয়। যেমন, ধরুন আমরা `let mut` দিয়ে একটি সংখ্যার reference ঘোষণা করলাম:

```aquascope,permissions,stepper
#fn main() {
let x = 0;
let mut x_ref = &x;
# println!("{x_ref} {x}");
#}
```

লক্ষ করুন, `x_ref`-এর @Perm{write} permission আছে, কিন্তু `*x_ref`-এর নেই। এর মানে আমরা `x_ref` variable-এ অন্য একটি reference assign করতে পারি (যেমন `x_ref = &y`), কিন্তু এটি যে ডেটাকে নির্দেশ করে, তা mutate করতে পারি না (যেমন `*x_ref += 1`)।

আরও সাধারণভাবে বললে, permission শুধু variable-এর ওপর নয়, **place**-এর ওপর সংজ্ঞায়িত। place হলো এমন যেকোনো কিছু, যা একটি assignment-এর বাঁ পাশে বসানো যায়। place-এর মধ্যে আছে:

- variable, যেমন `a`।
- place-এর dereference, যেমন `*a`।
- place-এর array access, যেমন `a[0]`।
- place-এর field (struct বা tuple-এর একেকটি অংশ), যেমন tuple-এর জন্য `a.0` বা struct (নিজস্ব গঠনের ডেটা টাইপ)-এর জন্য `a.field` (পরের অধ্যায়ে আলোচিত)।
- ওপরের যেকোনোটির সমন্বয়, যেমন `*((*a)[0].1)`।


দ্বিতীয়ত, অব্যবহৃত হয়ে গেলে place-গুলো permission হারায় কেন? কারণ কিছু permission পরস্পর-বর্জনশীল। আপনি `num = &v[2]` লিখলে `num` ব্যবহারে থাকা অবস্থায় `v` mutate বা drop করা যায় না। কিন্তু তার মানে এই নয় যে `num` আবার ব্যবহার করা অবৈধ। যেমন, ওপরের প্রোগ্রামে আরেকটি `println!` যোগ করলে `num` শুধু এক লাইন পরে তার permission হারায়:

```aquascope,permissions,stepper
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];
println!("Third element is {}", *num);
println!("Again, the third element is {}", *num);
v.push(4);
#}
```

সমস্যা হয় কেবল তখনই, যখন `v` mutate করার *পরে* আবার `num` ব্যবহারের চেষ্টা করেন। চলুন এটি আরও বিস্তারিত দেখি।


### borrow checker permission-এর লঙ্ঘন খুঁজে বের করে {#the-borrow-checker-finds-permission-violations}

*pointer নিরাপত্তার নীতি* মনে করুন: ডেটা একই সঙ্গে alias আর mutate হওয়া উচিত নয়। এই permission-গুলোর উদ্দেশ্য হলো নিশ্চিত করা যে ডেটা alias হলে তা mutate করা যাবে না। কোনো ডেটার reference তৈরি করলে (ডেটাটিকে "borrow" করলে) reference-টি ব্যবহার শেষ না হওয়া পর্যন্ত সেই ডেটা সাময়িকভাবে শুধু-পড়ার উপযোগী হয়ে যায়।

Rust এই permission-গুলো ব্যবহার করে তার **borrow checker**-এ। borrow checker reference-জড়িত সম্ভাব্য অনিরাপদ operation খোঁজে। চলুন আগে দেখা সেই অনিরাপদ প্রোগ্রামে ফিরে যাই, যেখানে `push` একটি reference অবৈধ করে দেয়। এবার permission-এর ডায়াগ্রামে আরেকটি দিক যোগ করব:

```aquascope,permissions,boundaries,stepper,shouldFail
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &i32 = &v[2];`{}`
v.push(4);`{}`
println!("Third element is {}", *num);
#}
```

যখনই একটি place ব্যবহার করা হয়, operation অনুযায়ী Rust আশা করে সেই place-এর নির্দিষ্ট কিছু permission থাকবে। যেমন, `&v[2]` borrow-এর জন্য `v`-কে পড়ার উপযোগী হতে হবে। তাই `&` operation আর place `v`-এর মাঝখানে @Perm{read} permission দেখানো হয়েছে। অক্ষরটি ভরাট, কারণ ওই লাইনে `v`-এর read permission আছে।

অন্যদিকে mutate করা operation `v.push(4)`-এর জন্য `v`-কে পড়া ও লেখা দুটোরই উপযোগী হতে হবে। @Perm{read} আর @Perm{write} দুটোই দেখানো হয়েছে। কিন্তু `v`-এর write permission নেই (এটি `num` দ্বারা borrow হয়ে আছে)। তাই @Perm[missing]{write} অক্ষরটি ফাঁপা, যা বোঝায় write permission-টি *প্রত্যাশিত*, কিন্তু `v`-এর তা নেই।

এই প্রোগ্রাম compile করার চেষ্টা করলে Rust compiler নিচের error দেবে:

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> test.rs:4:1
  |
3 | let num: &i32 = &v[2];
  |                  - immutable borrow occurs here
4 | v.push(4);
  | ^^^^^^^^^ mutable borrow occurs here
5 | println!("Third element is {}", *num);
  |                                 ---- immutable borrow later used here
```

error message ব্যাখ্যা করছে যে reference `num` ব্যবহারে থাকা অবস্থায় `v` mutate করা যায় না। এটি ওপর-ওপর কারণ &mdash; আসল সমস্যা হলো `push` `num`-কে অবৈধ করে দিতে পারে। Rust memory safety-র সেই সম্ভাব্য লঙ্ঘন ধরে ফেলে।


### mutable reference ডেটায় একক ও non-owning প্রবেশাধিকার দেয় {#mutable-references-provide-unique-and-non-owning-access-to-data}

এ পর্যন্ত আমরা যেসব reference দেখেছি, সেগুলো শুধু-পড়ার **immutable reference** (শুধু পড়ার reference; এদের **shared reference**-ও বলা হয়)। immutable reference aliasing-এর অনুমতি দেয়, কিন্তু mutation নিষিদ্ধ করে। তবে ডেটা move না করে সাময়িকভাবে তাতে mutate করার প্রবেশাধিকার দেওয়াও কাজের।

এর কৌশল হলো **mutable reference** (বদলানোর অনুমতিসহ একক reference; এদের **unique reference**-ও বলা হয়)। এখানে mutable reference-এর একটি সহজ উদাহরণ, সঙ্গে permission-এর পরিবর্তন:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &mut i32 = &mut v[2];
*num += 1;
println!("Third element is {}", *num);
println!("Vector is now {:?}", v);
#}
```

<blockquote><div style="margin-block-start: 1em; margin-block-end: 1em"><i>দ্রষ্টব্য:</i> কোনো উদাহরণের জন্য প্রত্যাশিত permission-গুলো খুব প্রাসঙ্গিক না হলে আমরা সেগুলো সংক্ষেপে বিন্দু দিয়ে দেখাব, যেমন <div class="permission-stack stack-size-2"><div class="perm read"><div class="small">•</div><div class="big">R</div></div><div class="perm write"><div class="small">•</div><div class="big">W</div></div></div>। সংশ্লিষ্ট permission-এর অক্ষর দেখতে বৃত্তগুলোর ওপর মাউস রাখুন (বা টাচস্ক্রিনে ট্যাপ করুন)।</div></blockquote>

mutable reference তৈরি হয় `&mut` operator দিয়ে। `num`-এর type লেখা হয় `&mut i32`। immutable reference-এর তুলনায় permission-এ দুটি গুরুত্বপূর্ণ পার্থক্য দেখতে পাবেন:

1. `num` যখন immutable reference ছিল, তখনও `v`-এর @Perm{read} permission ছিল। এখন `num` একটি mutable reference, তাই `num` ব্যবহারে থাকা অবস্থায় `v` তার _সব_ permission হারিয়েছে।
2. `num` যখন immutable reference ছিল, তখন place `*num`-এর শুধু @Perm{read} permission ছিল। এখন `num` একটি mutable reference, তাই `*num` @Perm{write} permission-ও পেয়েছে।

প্রথম পর্যবেক্ষণটিই mutable reference-কে *নিরাপদ* করে। mutable reference mutation-এর অনুমতি দেয়, কিন্তু aliasing আটকায়। borrow হওয়া place `v` সাময়িকভাবে ব্যবহারের অযোগ্য হয়ে যায়, তাই কার্যত এটি আর alias নয়।

দ্বিতীয় পর্যবেক্ষণটিই mutable reference-কে *কাজের* করে। `*num`-এর মাধ্যমে `v[2]` mutate করা যায়। যেমন, `*num += 1` `v[2]`-কে mutate করে। লক্ষ করুন, `*num`-এর @Perm{write} permission আছে, কিন্তু `num`-এর নেই। `num` বোঝায় mutable reference-টিকেই, যেমন `num`-এ একটি *ভিন্ন* mutable reference আবার assign করা যায় না।

mutable reference-কে সাময়িকভাবে শুধু-পড়ার reference-এ "নামিয়ে আনা"-ও যায়। যেমন:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut v: Vec<i32> = vec![1, 2, 3];
let num: &mut i32 = &mut v[2];`(focus,paths:*num)`
let num2: &i32 = &*num;`(focus,paths:*num)`
println!("{} {}", *num, *num2);
#}
```

> *দ্রষ্টব্য:* কোনো উদাহরণের জন্য permission-এর পরিবর্তন প্রাসঙ্গিক না হলে আমরা সেগুলো লুকিয়ে রাখব। লুকানো ধাপগুলো দেখতে "»"-এ ক্লিক করুন, আর কোনো ধাপের ভেতরের লুকানো permission দেখতে "● ● ●"-এ ক্লিক করুন।

এই প্রোগ্রামে `&*num` borrow-টি `*num` থেকে @Perm{write} permission সরিয়ে নেয়, কিন্তু @Perm{read} permission _নয়_, তাই `println!(..)` `*num` আর `*num2` দুটোই পড়তে পারে।


### reference-এর lifetime শেষে permission ফিরে আসে {#permissions-are-returned-at-the-end-of-a-references-lifetime}

ওপরে আমরা বলেছি, একটি reference "ব্যবহারে থাকা" অবস্থায় permission বদলায়। "ব্যবহারে থাকা" কথাটি একটি reference-এর **lifetime** (reference কতক্ষণ বৈধ থাকে) বর্ণনা করে, অর্থাৎ এর জন্ম (যেখানে reference তৈরি হয়) থেকে মৃত্যু (শেষবার(গুলো) যেখানে reference ব্যবহার হয়) পর্যন্ত কোডের পরিসর।

যেমন, এই প্রোগ্রামে `y`-এর lifetime শুরু হয় `let y = &x` দিয়ে, আর শেষ হয় `let z = *y` দিয়ে:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut x = 1;
let y = &x;`(focus,paths:x)`
let z = *y;`(focus,paths:x)`
x += z;
#}
```

আগে যেমন দেখেছি, `y`-এর lifetime শেষ হওয়ার পর `x`-এর @Perm{write} permission `x`-এর কাছে ফিরে আসে।

আগের উদাহরণগুলোতে lifetime ছিল কোডের একটানা একটি অংশ। তবে control flow (কোড চলার ক্রম নিয়ন্ত্রণ) এলে তা আর সবসময় হয় না। যেমন, এখানে একটি function আছে, যা ASCII অক্ষরের একটি vector-এর প্রথম অক্ষরটিকে বড় হাতের করে:

```aquascope,permissions,stepper,boundaries
fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];`(focus,paths:*v)`
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();`(focus,paths:*v)`
        v[0] = up;
    } else {`(focus,paths:*v)`
        println!("Already capitalized: {:?}", v);
    }
}
```

if-statement-এর প্রতিটি শাখায় `c` variable-এর lifetime আলাদা। then-block-এ `c` ব্যবহার হয় `c.to_ascii_uppercase()` expression-এ। তাই ওই লাইনের পরে না যাওয়া পর্যন্ত `*v` @Perm{write} permission ফিরে পায় না।

তবে else-block-এ `c` ব্যবহার হয় না। else-block-এ ঢোকামাত্রই `*v` @Perm{write} permission ফিরে পায়।

{{#quiz ../quizzes/ch04-02-references-sec2-perms.toml}}


### ডেটাকে তার সব reference-এর চেয়ে বেশিদিন টিকে থাকতে হবে {#data-must-outlive-all-of-its-references}

*pointer নিরাপত্তার নীতি*-র অংশ হিসেবে borrow checker বলবৎ করে যে **ডেটাকে তার যেকোনো reference-এর চেয়ে বেশিদিন টিকে থাকতে হবে।** Rust এই বৈশিষ্ট্য দুইভাবে বলবৎ করে। প্রথম উপায়টি এমন reference নিয়ে, যা একটিমাত্র function-এর scope-এর ভেতরেই তৈরি ও drop হয়। যেমন, ধরুন আমরা একটি string-এর reference ধরে রাখা অবস্থায় string-টি drop করার চেষ্টা করলাম:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let s = String::from("Hello world");
let s_ref = &s;`(focus,rxpaths:s$)`
drop(s);`{}`
println!("{}", s_ref);
#}
```

এ ধরনের error ধরতে Rust আগে আলোচিত permission-গুলোই ব্যবহার করে। `&s` borrow `s` থেকে @Perm{own} permission সরিয়ে নেয়। কিন্তু `drop` @Perm{own} permission আশা করে, ফলে permission মেলে না।

মূল কথা হলো, এই উদাহরণে Rust জানে `s_ref` কতক্ষণ টিকে থাকে। কিন্তু কোনো reference কতক্ষণ টিকে থাকে Rust যখন তা জানে না, তখন তার আলাদা একটি বলবৎ-কৌশল দরকার। নির্দিষ্ট করে বললে, যখন reference কোনো function-এর ইনপুট, অথবা কোনো function-এর আউটপুট। যেমন, এখানে একটি নিরাপদ function আছে, যা একটি vector-এর প্রথম উপাদানের একটি reference return করে:

```aquascope,permissions,boundaries,showFlows
fn first(strings: &Vec<String>) -> &String {
    let s_ref = &strings[0];
    s_ref`{}`
}
```

এই কোড-অংশ একটি নতুন ধরনের permission-এর সঙ্গে পরিচয় করিয়ে দেয়: flow permission @Perm{flow}, বা **Flow** (প্রবাহ)। কোনো expression যখনই একটি ইনপুট reference ব্যবহার করে (যেমন `&strings[0]`), অথবা একটি আউটপুট reference return করে (যেমন `return s_ref`), তখনই @Perm{flow} permission প্রত্যাশিত।

@Perm{read}@Perm{write}@Perm{own} permission-এর বিপরীতে @Perm{flow} একটি function-এর body জুড়ে বদলায় না। কোনো নির্দিষ্ট expression-এ একটি reference ব্যবহার করার (অর্থাৎ *প্রবাহিত* হওয়ার) অনুমতি থাকলে সেই reference-এর @Perm{flow} permission থাকে। যেমন, ধরুন আমরা `first`-কে বদলে `first_or` নামের একটি নতুন function বানালাম, যাতে একটি `default` parameter আছে:

```aquascope,permissions,boundaries,showFlows,shouldFail
fn first_or<'a, 'b, 'c>(strings: &'a Vec<String>, default: &'b String) -> &'c String {
    if strings.len() > 0 {
        &strings[0]`{}`
    } else {
        default`{}`
    }
}
```

এই function আর compile হয় না, কারণ `&strings[0]` আর `default` expression-এর return হওয়ার জন্য প্রয়োজনীয় @Perm{flow} permission নেই। কিন্তু কেন? Rust নিচের error দেয়:

```text
error[E0106]: missing lifetime specifier
 --> test.rs:1:57
  |
1 | fn first_or(strings: &Vec<String>, default: &String) -> &String {
  |                      ------------           -------     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `strings` or `default`
```

"missing lifetime specifier" বার্তাটি একটু রহস্যময়, তবে help বার্তাটি কিছু কাজের প্রেক্ষাপট দেয়। Rust যদি *শুধু* function signature দেখে, তাহলে সে জানে না আউটপুট `&String` `strings`-এর reference নাকি `default`-এর। এটা কেন গুরুত্বপূর্ণ, তা বুঝতে ধরুন আমরা `first_or` এভাবে ব্যবহার করলাম:

```rust,ignore
fn main() {
    let strings = vec![];
    let default = String::from("default");
    let s = first_or(&strings, &default);
    drop(default);
    println!("{}", s);
}
```

`first_or` যদি `default`-কে return মানে *প্রবাহিত* হতে দেয়, তাহলে এই প্রোগ্রামটি অনিরাপদ। আগের উদাহরণের মতোই `drop` `s`-কে অবৈধ করে দিতে পারে। Rust এই প্রোগ্রাম compile হতে দেবে কেবল তখনই, যদি সে *নিশ্চিত* হয় যে `default` return মানে প্রবাহিত হতে পারে না।

`default` return করা যাবে কি না তা নির্দিষ্ট করতে Rust *lifetime parameter* (reference-এর lifetime বোঝানোর parameter) নামের একটি কৌশল দেয়। এই বৈশিষ্ট্যটি আমরা পরে অধ্যায় 10.3-এর ["Validating References with Lifetimes"](ch10-03-lifetime-syntax.html)-এ ব্যাখ্যা করব। আপাতত এটুকু জানলেই চলবে: (1) ইনপুট/আউটপুট reference-কে function body-র ভেতরের reference থেকে আলাদাভাবে দেখা হয়, এবং (2) সেই reference-গুলোর নিরাপত্তা যাচাইয়ে Rust একটি আলাদা কৌশল, @Perm{flow} permission, ব্যবহার করে।

আরেকটি প্রেক্ষাপটে @Perm{flow} permission দেখতে ধরুন আপনি stack-এ থাকা একটি variable-এর reference return করার চেষ্টা করলেন, এভাবে:

```aquascope,permissions,boundaries,showFlows,shouldFail
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    let s_ref = &s;
    s_ref`{}`
}
```

এই প্রোগ্রামটি অনিরাপদ, কারণ `return_a_string` return করলেই `&s` reference অবৈধ হয়ে যাবে। আর Rust একই রকম `missing lifetime specifier` error দিয়ে এই প্রোগ্রাম বাতিল করবে। এখন আপনি বুঝতে পারছেন, ওই error-এর মানে হলো `s_ref`-এর উপযুক্ত flow permission নেই।


{{#quiz ../quizzes/ch04-02-references-sec3-safety.toml}}


### সারসংক্ষেপ {#summary}

reference ownership না নিয়েই ডেটা পড়া ও লেখার সুযোগ দেয়। reference তৈরি হয় borrow (`&` আর `&mut`) দিয়ে, আর ব্যবহার হয় dereference (`*`) দিয়ে, প্রায়ই অন্তর্নিহিতভাবে।

তবে reference-এর অপব্যবহার সহজেই হতে পারে। Rust-এর borrow checker permission-এর এমন একটি ব্যবস্থা বলবৎ করে, যা নিশ্চিত করে reference নিরাপদে ব্যবহার হচ্ছে:

- সব variable তার ডেটা পড়তে, own করতে এবং (ঐচ্ছিকভাবে) লিখতে পারে।
- একটি reference তৈরি করলে borrow হওয়া place থেকে permission reference-টির কাছে হস্তান্তর হয়।
- reference-এর lifetime শেষ হলে permission ফিরে আসে।
- ডেটাকে তার সব reference-এর চেয়ে বেশিদিন টিকে থাকতে হবে।

এই অংশে হয়তো মনে হচ্ছে, Rust কী _পারে_ তার চেয়ে কী _পারে না_ সেটাই বেশি বলেছি। এটা ইচ্ছাকৃত! Rust-এর একটি মূল বৈশিষ্ট্য হলো garbage collection ছাড়াই pointer ব্যবহারের সুযোগ দেওয়া, আবার একই সঙ্গে undefined behavior এড়ানো। এই নিরাপত্তার নিয়মগুলো এখন বুঝে নিলে পরে compiler নিয়ে বিরক্তি এড়াতে পারবেন।

[`String::push_str`]: https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`Vec::push`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push
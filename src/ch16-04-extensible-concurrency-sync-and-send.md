<!-- Old headings. Do not remove or links may break. -->

<a id="extensible-concurrency-with-the-sync-and-send-traits"></a>
<a id="extensible-concurrency-with-the-send-and-sync-traits"></a>

## `Send` ও `Sync` দিয়ে বাড়ানো যায় এমন concurrency {#extensible-concurrency-with-send-and-sync}

মজার ব্যাপার হলো, এই অধ্যায়ে এ পর্যন্ত আলোচিত প্রায় প্রতিটি concurrency বৈশিষ্ট্য standard library-র অংশ, ভাষার নয়। concurrency সামলানোর জন্য আপনার বিকল্প ভাষা বা standard
library-তে সীমাবদ্ধ নয়; আপনি নিজের concurrency বৈশিষ্ট্য লিখতে পারেন বা অন্যদের লেখা বৈশিষ্ট্য ব্যবহার করতে পারেন।

তবে standard library-র বদলে ভাষার ভেতরেই বসানো মূল concurrency ধারণাগুলোর মধ্যে আছে `std::marker`-এর trait `Send` আর `Sync`।

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-transference-of-ownership-between-threads-with-send"></a>

### thread-এর মধ্যে ownership হস্তান্তর {#transferring-ownership-between-threads}

`Send` marker trait (কোনো method ছাড়া শুধু একটি বৈশিষ্ট্য চিহ্নিত করে এমন trait) বোঝায় যে `Send` implement করা type-এর মানের ownership thread-এর মধ্যে হস্তান্তর করা যায়। প্রায়
প্রতিটি Rust type `Send` implement করে, তবে কিছু ব্যতিক্রম আছে, যার মধ্যে `Rc<T>`-ও আছে: এটি `Send` implement করতে পারে না, কারণ আপনি যদি একটি `Rc<T>` মান clone করে সেই
clone-এর ownership অন্য thread-এ হস্তান্তর করার চেষ্টা করেন, তাহলে দুটি thread-ই একই সময়ে reference count হালনাগাদ করতে পারে। এ কারণে `Rc<T>` এমন single-threaded পরিস্থিতিতে
ব্যবহারের জন্য implement করা, যেখানে আপনি thread-safe হওয়ার performance মাশুল দিতে চান না।

তাই Rust-এর type system আর trait bound নিশ্চিত করে যে আপনি কখনো ভুল করে অনিরাপদভাবে thread-এর মধ্যে একটি `Rc<T>` মান পাঠাতে পারবেন না। Listing 16-14-এ এটা করার চেষ্টা
করলে আমরা `` the trait `Send` is not implemented
for `Rc<Mutex<i32>>` `` error পেয়েছিলাম। যখন `Arc<T>`-তে বদলে নিলাম, যা `Send` implement করে, তখন কোড compile হয়েছিল।

পুরোপুরি `Send` type দিয়ে গঠিত যেকোনো type স্বয়ংক্রিয়ভাবে `Send` হিসেবে চিহ্নিত হয়। raw pointer ছাড়া প্রায় সব primitive type `Send`; raw pointer নিয়ে অধ্যায় 20-এ আলোচনা
করব।

<!-- Old headings. Do not remove or links may break. -->

<a id="allowing-access-from-multiple-threads-with-sync"></a>

### একাধিক thread থেকে প্রবেশ {#accessing-from-multiple-threads}

`Sync` marker trait বোঝায় যে `Sync` implement করা type-কে একাধিক thread থেকে reference করা নিরাপদ। অন্যভাবে বললে, যেকোনো type `T` `Sync` implement করে যদি `&T` (`T`-এর
একটি immutable reference) `Send` implement করে, অর্থাৎ reference-টি নিরাপদে অন্য thread-এ পাঠানো যায়। `Send`-এর মতোই সব primitive type `Sync` implement করে, আর পুরোপুরি
`Sync` implement করা type দিয়ে গঠিত type-ও `Sync` implement করে।

<!-- BEGIN INTERVENTION: 43081862-aac8-4e18-9c55-1107ea4c7cc1 -->

"thread-safe" কথাটির চলতি অর্থের সবচেয়ে কাছের ধারণা Rust-এ হলো `Sync`, অর্থাৎ নির্দিষ্ট কোনো ডেটা একাধিক concurrent thread নিরাপদে ব্যবহার করতে পারে। আলাদা `Send` আর `Sync` trait রাখার কারণ হলো, একটি type কখনো কখনো একটি, দুটিই, বা কোনোটিই না হতে পারে। যেমন:
*  ওপরে বর্ণিত কারণে smart pointer `Rc<T>` `Send` বা `Sync` কোনোটিই নয়।
* `RefCell<T>` type (যা নিয়ে অধ্যায় 15-এ কথা বলেছি) আর এর সম্পর্কিত
`Cell<T>` type-এর পরিবার `Send` (যদি `T: Send` হয়), কিন্তু এরা `Sync` নয়। একটি `RefCell` thread-এর সীমানা পেরিয়ে পাঠানো যায়, কিন্তু একসঙ্গে একাধিক thread থেকে প্রবেশ করা যায় না, কারণ `RefCell<T>` runtime-এ borrow checking-এর যে implementation করে তা thread-safe নয়। 
* smart pointer `Mutex<T>` `Send` এবং `Sync`, আর [“`Mutex<T>`-এ ভাগ করা প্রবেশাধিকার”][sharing-a-mutext-between-multiple-threads]<!-- ignore --> অংশে যেমন দেখেছেন, একাধিক thread-এর সঙ্গে প্রবেশাধিকার ভাগ করতে এটি ব্যবহার করা যায়।
* `Mutex::lock`-এর return করা `MutexGuard<'a, T>` type `Sync` (যদি `T: Sync` হয়), কিন্তু `Send` নয়। এটি বিশেষভাবে `Send` নয়, কারণ [কিছু platform-এ নিয়ম আছে যে mutex যে thread lock করেছে, সেই thread-কেই unlock করতে হবে][mutex-guards-are-not-send]।

<!-- END INTERVENTION: 43081862-aac8-4e18-9c55-1107ea4c7cc1 -->



### হাতে `Send` ও `Sync` implement করা unsafe {#implementing-send-and-sync-manually-is-unsafe}

`Send` আর `Sync` trait implement করে এমন অন্য type দিয়ে পুরোপুরি গঠিত type-গুলোও যেহেতু স্বয়ংক্রিয়ভাবে `Send` আর `Sync` implement করে, তাই এই trait-গুলো আমাদের হাতে
implement করতে হয় না। marker trait হিসেবে এদের implement করার মতো কোনো method-ও নেই। concurrency-সংক্রান্ত invariant প্রয়োগ করতেই শুধু এগুলো কাজে লাগে।

এই trait-গুলো হাতে implement করতে unsafe Rust কোড implement করতে হয়। unsafe Rust কোড ব্যবহার নিয়ে অধ্যায় 20-এ কথা বলব; আপাতত গুরুত্বপূর্ণ তথ্য হলো, `Send` আর `Sync`
অংশ দিয়ে তৈরি নয় এমন নতুন concurrent type বানাতে নিরাপত্তার নিশ্চয়তা বজায় রাখার জন্য সতর্ক চিন্তা দরকার। এই নিশ্চয়তাগুলো এবং কীভাবে সেগুলো বজায় রাখতে হয় সে সম্পর্কে [“The
Rustonomicon”][nomicon]-এ আরও তথ্য আছে।

## সারসংক্ষেপ {#summary}

এই বইয়ে concurrency-র সঙ্গে এটিই আপনার শেষ দেখা নয়: পরের অধ্যায় async programming নিয়ে, আর অধ্যায় 21-এর project এখানে আলোচিত ছোট উদাহরণগুলোর চেয়ে আরও বাস্তব পরিস্থিতিতে
এই অধ্যায়ের ধারণাগুলো ব্যবহার করবে।

আগে যেমন বলা হয়েছে, Rust যেভাবে concurrency সামলায় তার খুব সামান্যই ভাষার অংশ, তাই concurrency-র অনেক সমাধান crate হিসেবে implement করা। এগুলো standard library-র চেয়ে
দ্রুত বিকশিত হয়, তাই multithreaded পরিস্থিতিতে ব্যবহারের জন্য বর্তমানের সবচেয়ে আধুনিক crate-গুলো online-এ খুঁজে দেখতে ভুলবেন না।

Rust standard library message passing-এর জন্য channel দেয়, আর `Mutex<T>` আর `Arc<T>`-এর মতো smart pointer type দেয়, যা concurrent প্রেক্ষাপটে ব্যবহার করা নিরাপদ। type system
আর borrow checker নিশ্চিত করে যে এই সমাধানগুলো ব্যবহার করা কোডে data race বা অবৈধ reference থাকবে না। একবার আপনার কোড compile করাতে পারলে নিশ্চিন্ত থাকতে পারেন যে এটি অন্য
ভাষায় প্রচলিত খুঁজে বের করা কঠিন ধরনের বাগ ছাড়াই আনন্দের সঙ্গে একাধিক thread-এ চলবে। concurrent programming আর ভয় পাওয়ার মতো কোনো ধারণা নয়: এগিয়ে যান এবং নির্ভয়ে আপনার
প্রোগ্রামগুলোকে concurrent করুন!

{{#quiz ../quizzes/ch16-04-extensible-concurrency-send-and-sync.toml}}

[sharing-a-mutext-between-multiple-threads]: ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads
[nomicon]: https://doc.rust-lang.org/nomicon/index.html
[mutex-guards-are-not-send]: https://github.com/rust-lang/rust/issues/23465#issuecomment-82730326

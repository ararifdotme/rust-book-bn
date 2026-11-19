## সব একসঙ্গে: future, task ও thread {#putting-it-all-together-futures-tasks-and-threads}

[অধ্যায় 16][ch16]<!-- ignore -->-এ যেমন দেখেছি, thread concurrency-র একটি পদ্ধতি দেয়। এই অধ্যায়ে আরেকটি পদ্ধতি দেখলাম: future আর stream-সহ async ব্যবহার। কখন কোন পদ্ধতি বেছে নেবেন তা নিয়ে
ভাবলে উত্তর হলো: নির্ভর করে! আর অনেক ক্ষেত্রে পছন্দটি thread _অথবা_ async নয়, বরং thread _এবং_ async।

অনেক operating system কয়েক দশক ধরে thread-ভিত্তিক concurrency model দিয়ে আসছে, আর ফলে অনেক প্রোগ্রামিং ভাষা এগুলো সমর্থন করে। তবে এই model-গুলোর সুবিধা-অসুবিধা আছে। অনেক operating
system-এ এরা প্রতিটি thread-এর জন্য বেশ খানিকটা মেমোরি ব্যবহার করে। তা ছাড়া thread একটি বিকল্প কেবল তখনই, যখন আপনার operating system আর hardware এগুলো সমর্থন করে। মূলধারার desktop আর
mobile কম্পিউটারের মতো নয়, কিছু embedded system-এ কোনো OS-ই নেই, তাই সেগুলোতে thread-ও নেই।

async model ভিন্ন—এবং শেষ পর্যন্ত পরিপূরক—কিছু সুবিধা-অসুবিধা দেয়। async model-এ concurrent operation-এর নিজস্ব thread লাগে না। বরং এগুলো task-এর ওপর চলতে পারে, যেমন stream অংশে একটি
synchronous function থেকে কাজ শুরু করতে আমরা `trpl::spawn_task` ব্যবহার করেছিলাম। একটি task thread-এর মতোই, তবে operating system-এর বদলে এটি সামলায় library-স্তরের কোড: runtime।

thread চালু করা আর task চালু করার API এত কাছাকাছি হওয়ার একটি কারণ আছে। thread synchronous operation-এর সেটের সীমানা হিসেবে কাজ করে; concurrency সম্ভব thread-গুলোর _মধ্যে_। task
_asynchronous_ operation-এর সেটের সীমানা হিসেবে কাজ করে; concurrency সম্ভব task-গুলোর _মধ্যে_ এবং _ভেতরে_ দুই জায়গাতেই, কারণ একটি task তার body-তে future-গুলোর মধ্যে বদল করতে পারে।
শেষে, future হলো Rust-এর concurrency-র সবচেয়ে সূক্ষ্ম একক, আর প্রতিটি future অন্য future-এর একটি tree বোঝাতে পারে। runtime—নির্দিষ্ট করে বললে এর executor—task সামলায়, আর task সামলায়
future। সেদিক থেকে task হলো runtime-এর সামলানো হালকা thread-এর মতো, সঙ্গে বাড়তি কিছু ক্ষমতা, যা আসে operating system-এর বদলে runtime-এর সামলানো হওয়ার কারণে।

এর মানে এই নয় যে async task সবসময় thread-এর চেয়ে ভালো (বা উল্টোটা)। কিছু দিক থেকে thread দিয়ে concurrency `async` দিয়ে concurrency-র চেয়ে সরল প্রোগ্রামিং model। এটি শক্তি বা দুর্বলতা,
দুটোই হতে পারে। thread কিছুটা “চালু করো আর ভুলে যাও” ধরনের; এদের future-এর কোনো স্বাভাবিক সমতুল্য নেই, তাই operating system ছাড়া আর কারও বাধা ছাড়াই এরা শেষ পর্যন্ত চলে।

আর দেখা যায় thread আর task প্রায়ই
একসঙ্গে খুব ভালো কাজ করে, কারণ task-কে (অন্তত কিছু runtime-এ) thread-গুলোর মধ্যে এদিক-ওদিক সরানো যায়। আসলে ভেতরে ভেতরে আমরা যে runtime ব্যবহার করে আসছি—`spawn_blocking` আর `spawn_task`
function-সহ—তা default-ভাবেই multithreaded! অনেক runtime _work stealing_ (কাজ চুরি) নামের একটি পদ্ধতি ব্যবহার করে, যাতে thread-গুলো এই মুহূর্তে কতটা ব্যবহৃত হচ্ছে তার ভিত্তিতে অদৃশ্যভাবে
task-গুলোকে thread-এর মধ্যে সরিয়ে system-এর সামগ্রিক performance উন্নত করা যায়। এই পদ্ধতির আসলে thread _এবং_ task, আর তাই future-ও দরকার।

কখন কোন পদ্ধতি ব্যবহার করবেন তা ভাবার সময় এই মোটামুটি নিয়মগুলো বিবেচনা করুন:

- কাজটি যদি _খুব বেশি parallel করা যায়_ (অর্থাৎ CPU-bound), যেমন অনেক ডেটা প্রক্রিয়া করা যেখানে প্রতিটি অংশ আলাদাভাবে প্রক্রিয়া করা যায়, তাহলে thread ভালো পছন্দ।
- কাজটি যদি _খুব বেশি concurrent_ হয় (অর্থাৎ I/O-bound), যেমন অনেক ভিন্ন উৎস থেকে আসা message সামলানো, যা ভিন্ন ভিন্ন বিরতিতে বা ভিন্ন গতিতে আসতে পারে, তাহলে async ভালো পছন্দ।

আর যদি parallelism ও concurrency দুটিই দরকার হয়, তাহলে thread আর async-এর মধ্যে বেছে নিতে হবে না। এদের স্বাধীনভাবে একসঙ্গে ব্যবহার করতে পারেন, প্রত্যেককে তার সবচেয়ে ভালো কাজটি করতে দিয়ে।
যেমন, Listing 17-25-এ বাস্তবের Rust কোডে এই ধরনের মিশ্রণের একটি বেশ প্রচলিত উদাহরণ দেখানো হয়েছে।

<Listing number="17-25" caption="একটি thread-এ block করা কোড দিয়ে message পাঠানো এবং একটি async block-এ message-গুলোর জন্য await করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-25/src/main.rs:all}}
```

</Listing>

প্রথমে আমরা একটি async channel তৈরি করি, তারপর এমন একটি thread চালু করি যা `move` keyword ব্যবহার করে channel-এর পাঠানোর দিকের ownership নেয়। thread-এর ভেতরে আমরা 1 থেকে 10 পর্যন্ত সংখ্যা
পাঠাই, প্রতিটির মাঝে এক সেকেন্ড sleep করে। শেষে অধ্যায় জুড়ে যেমন করেছি, তেমনভাবে `trpl::block_on`-এ পাঠানো একটি async block দিয়ে তৈরি future চালাই। সেই future-এ আমরা message-গুলোর জন্য
await করি, ঠিক যেমন অন্য message passing উদাহরণগুলোতে দেখেছি।

অধ্যায়টি যে পরিস্থিতি দিয়ে শুরু করেছিলাম সেখানে ফিরে গেলে, কল্পনা করুন একটি নিবেদিত thread ব্যবহার করে কয়েকটি video encoding-এর কাজ চালাচ্ছেন (কারণ video encoding compute-bound), কিন্তু
একটি async channel দিয়ে UI-কে জানাচ্ছেন যে সেই operation-গুলো শেষ হয়েছে। বাস্তব ব্যবহারে এমন সমন্বয়ের অসংখ্য উদাহরণ আছে।

## সারসংক্ষেপ {#summary}

এই বইয়ে concurrency-র সঙ্গে এটিই আপনার শেষ দেখা নয়। [অধ্যায় 21][ch21]<!-- ignore -->-এর project এখানে আলোচিত সরল উদাহরণগুলোর চেয়ে আরও বাস্তব পরিস্থিতিতে এই ধারণাগুলো প্রয়োগ
করবে, এবং threading বনাম task ও future দিয়ে সমস্যা সমাধানের আরও সরাসরি তুলনা করবে।

এই পদ্ধতিগুলোর যেটিই বেছে নিন না কেন, নিরাপদ, দ্রুত, concurrent কোড লেখার জন্য দরকারি হাতিয়ার Rust আপনাকে দেয়—উচ্চ-throughput web server হোক বা embedded operating system।

এরপর আমরা কথা বলব, আপনার Rust প্রোগ্রাম বড় হতে থাকলে সমস্যা সাজানো আর সমাধানের গঠন তৈরির idiomatic উপায় নিয়ে। এছাড়া object-oriented programming থেকে আপনার পরিচিত idiom-গুলোর সঙ্গে Rust-এর
idiom-গুলো কীভাবে সম্পর্কিত, তা নিয়েও আলোচনা করব।

[ch16]: http://localhost:3000/ch16-00-concurrency.html
[combining-futures]: ch17-03-more-futures.html#building-our-own-async-abstractions
[streams]: ch17-04-streams.html#composing-streams
[ch21]: ch21-00-final-project-a-web-server.html

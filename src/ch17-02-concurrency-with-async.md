<!-- Old headings. Do not remove or links may break. -->

<a id="concurrency-with-async"></a>

## async দিয়ে concurrency প্রয়োগ {#applying-concurrency-with-async}

এই অংশে আমরা অধ্যায় 16-এ thread দিয়ে মোকাবিলা করা কিছু একই concurrency চ্যালেঞ্জে async প্রয়োগ করব। সেখানে অনেক মূল ধারণা নিয়ে আগেই আলোচনা করেছি বলে এই অংশে আমরা thread আর
future-এর মধ্যে কী কী আলাদা সেদিকে মনোযোগ দেব।

অনেক ক্ষেত্রে async দিয়ে concurrency নিয়ে কাজ করার API thread ব্যবহারের API-র খুব কাছাকাছি। অন্য ক্ষেত্রে এরা বেশ আলাদা হয়ে যায়। thread আর async-এর API দেখতে একই রকম _মনে হলেও_
এদের আচরণ প্রায়ই আলাদা হয়—আর performance-এর বৈশিষ্ট্য প্রায় সবসময়ই আলাদা হয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="counting"></a>

### `spawn_task` দিয়ে নতুন task তৈরি করা {#creating-a-new-task-with-spawn_task}

অধ্যায় 16-এর [“`spawn` দিয়ে নতুন thread তৈরি
করা”][thread-spawn]<!-- ignore --> অংশে আমরা প্রথম যে কাজটি করেছিলাম তা হলো দুটি আলাদা thread-এ গোনা। চলুন async দিয়ে একই কাজ করি। `trpl` crate একটি `spawn_task` function দেয়, যা
দেখতে `thread::spawn` API-র খুব কাছাকাছি, আর একটি `sleep` function দেয়, যা `thread::sleep` API-র async সংস্করণ। এগুলো একসঙ্গে ব্যবহার করে গোনার উদাহরণটি implement করা যায়,
যেমনটি Listing 17-6-এ দেখানো হয়েছে।

<Listing number="17-6" caption="main task যখন অন্য কিছু print করছে, তখন একটি জিনিস print করতে নতুন task তৈরি করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-06/src/main.rs:all}}
```

</Listing>

শুরুর বিন্দু হিসেবে আমরা `trpl::block_on` দিয়ে `main` function প্রস্তুত করি, যাতে আমাদের top-level function async হতে পারে।

> Note: অধ্যায়ের এখান থেকে প্রতিটি উদাহরণে `main`-এ `trpl::block_on`-সহ হুবহু এই মোড়কের কোড থাকবে, তাই `main`-এর মতোই আমরা প্রায়ই এটি বাদ দিয়ে দেখাব। আপনার কোডে এটি যোগ করতে
> ভুলবেন না!

তারপর সেই block-এর ভেতরে দুটি loop লিখি, যার প্রতিটিতে একটি `trpl::sleep` call আছে, যা পরের message পাঠানোর আগে আধা সেকেন্ড (500 millisecond) অপেক্ষা করে। একটি loop রাখি একটি
`trpl::spawn_task`-এর body-তে, আর অন্যটি একটি top-level `for` loop-এ। `sleep` call-গুলোর পরে একটি করে `await`-ও যোগ করি।

এই কোড thread-ভিত্তিক implementation-এর মতোই আচরণ করে—এমনকি চালানোর সময় নিজের terminal-এ message-গুলো ভিন্ন ক্রমে দেখতে পাওয়ার ব্যাপারটিও:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
```

main async block-এর body-র `for` loop শেষ হওয়ার সঙ্গে সঙ্গে এই সংস্করণ থেমে যায়, কারণ `main` function শেষ হলে `spawn_task`-এর চালু করা task বন্ধ হয়ে যায়। task-টি পুরোপুরি শেষ হওয়া পর্যন্ত
চালাতে চাইলে প্রথম task শেষ হওয়ার জন্য অপেক্ষা করতে একটি join handle ব্যবহার করতে হবে। thread-এর ক্ষেত্রে thread-এর চলা শেষ না হওয়া পর্যন্ত “block” করতে আমরা `join` method ব্যবহার
করেছিলাম। Listing 17-7-এ একই কাজ করতে `await` ব্যবহার করা যায়, কারণ task handle নিজেই একটি future। এর `Output` type একটি `Result`, তাই await করার পর একে unwrap-ও করি।

<Listing number="17-7" caption="একটি task শেষ পর্যন্ত চালাতে join handle-এর সঙ্গে `await` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-07/src/main.rs:handle}}
```

</Listing>

এই হালনাগাদ করা সংস্করণ _দুটি_ loop-ই শেষ না হওয়া পর্যন্ত চলে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the second task!
hi number 1 from the first task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

এ পর্যন্ত মনে হচ্ছে async আর thread আমাদের একই রকম ফল দেয়, শুধু syntax আলাদা: join handle-এর ওপর `join` call করার বদলে `await` ব্যবহার, আর `sleep` call-গুলোর জন্য await করা।

বড় পার্থক্য হলো, এটা করতে আমাদের operating system-এর আরেকটি thread চালু করতে হয়নি। আসলে এখানে একটি task চালু করারও দরকার নেই। async block যেহেতু নামহীন future-এ compile হয়, তাই
প্রতিটি loop একটি async block-এ রেখে `trpl::join` function ব্যবহার করে runtime-কে দিয়ে দুটিই শেষ পর্যন্ত চালানো যায়।

অধ্যায় 16-এর [“সব thread শেষ হওয়ার জন্য অপেক্ষা করা”][join-handles]<!-- ignore -->
অংশে আমরা দেখিয়েছিলাম `std::thread::spawn` call করলে return হওয়া `JoinHandle` type-এর ওপর কীভাবে `join` method ব্যবহার করতে হয়। `trpl::join` function-ও একই রকম, তবে future-এর
জন্য। একে দুটি future দিলে এটি একটি নতুন future তৈরি করে, যার output হলো এমন একটি tuple, যাতে পাঠানো প্রতিটি future-এর output থাকে, যখন _দুটিই_ শেষ হয়। তাই Listing 17-8-এ আমরা
`fut1` আর `fut2` দুটিই শেষ হওয়ার জন্য অপেক্ষা করতে `trpl::join` ব্যবহার করি। আমরা `fut1` আর `fut2`-এর জন্য await _করি না_, বরং `trpl::join`-এর তৈরি নতুন future-এর জন্য করি। output
উপেক্ষা করি, কারণ এটি শুধু দুটি unit মানযুক্ত একটি tuple।

<Listing number="17-8" caption="দুটি নামহীন future-এর জন্য await করতে `trpl::join` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-08/src/main.rs:join}}
```

</Listing>

এটি চালালে দেখি দুটি future-ই শেষ পর্যন্ত চলে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the first task!
hi number 1 from the second task!
hi number 2 from the first task!
hi number 2 from the second task!
hi number 3 from the first task!
hi number 3 from the second task!
hi number 4 from the first task!
hi number 4 from the second task!
hi number 5 from the first task!
hi number 6 from the first task!
hi number 7 from the first task!
hi number 8 from the first task!
hi number 9 from the first task!
```

এখন প্রতিবার হুবহু একই ক্রম দেখবেন, যা thread আর Listing 17-7-এর `trpl::spawn_task`-এর সঙ্গে দেখা আচরণ থেকে একেবারে আলাদা। কারণ `trpl::join` function _fair_ (ন্যায্য), অর্থাৎ এটি
প্রতিটি future সমান ঘন ঘন পরীক্ষা করে, পালা করে, আর অন্যটি প্রস্তুত থাকলে কোনোটিকে এগিয়ে যেতে দেয় না। thread-এর ক্ষেত্রে operating system ঠিক করে কোন thread পরীক্ষা করবে এবং কতক্ষণ
চলতে দেবে। async Rust-এ runtime ঠিক করে কোন task পরীক্ষা করবে। (বাস্তবে খুঁটিনাটি জটিল হয়ে যায়, কারণ একটি async runtime concurrency সামলানোর অংশ হিসেবে ভেতরে ভেতরে operating system-এর
thread ব্যবহার করতে পারে, তাই ন্যায্যতার নিশ্চয়তা দেওয়া runtime-এর জন্য বাড়তি কাজ হতে পারে—তবে তবুও সম্ভব!) কোনো নির্দিষ্ট operation-এর জন্য runtime-কে ন্যায্যতার নিশ্চয়তা দিতে হয়
না, আর ন্যায্যতা চান কি না তা বেছে নেওয়ার জন্য এরা প্রায়ই ভিন্ন ভিন্ন API দেয়।

future-এর জন্য await করার এই কয়েকটি ভিন্ন রূপ চেষ্টা করে দেখুন এরা কী করে:

- একটি বা দুটি loop-এর চারপাশ থেকে async block সরিয়ে দিন।
- প্রতিটি async block সংজ্ঞায়িত করার ঠিক পরেই সেটির জন্য await করুন।
- শুধু প্রথম loop-টি একটি async block-এ মুড়ে দিন, আর দ্বিতীয় loop-এর body-র পরে ফলাফলের future-এর জন্য await করুন।

বাড়তি চ্যালেঞ্জ হিসেবে, কোড চালানোর _আগেই_ প্রতিটি ক্ষেত্রে output কী হবে তা বের করতে পারেন কি না দেখুন!

<!-- Old headings. Do not remove or links may break. -->

<a id="message-passing"></a>
<a id="counting-up-on-two-tasks-using-message-passing"></a>

### message passing দিয়ে দুটি task-এর মধ্যে ডেটা পাঠানো {#sending-data-between-two-tasks-using-message-passing}

future-এর মধ্যে ডেটা ভাগ করাও পরিচিত লাগবে: আমরা আবার message passing ব্যবহার করব, তবে এবার type আর function-গুলোর async সংস্করণ দিয়ে। thread-ভিত্তিক আর future-ভিত্তিক
concurrency-র কিছু মূল পার্থক্য দেখাতে আমরা অধ্যায় 16-এর [“message passing দিয়ে thread-এর মধ্যে ডেটা
আদান-প্রদান”][message-passing-threads]<!-- ignore --> অংশের চেয়ে একটু ভিন্ন পথ নেব। Listing 17-9-এ আমরা শুরু করব শুধু একটি async block দিয়ে—যেভাবে আলাদা thread চালু করেছিলাম,
সেভাবে আলাদা task চালু _না_ করে।

<Listing number="17-9" caption="একটি async channel তৈরি করে এর দুটি অংশ `tx` আর `rx`-এ রাখা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-09/src/main.rs:channel}}
```

</Listing>

এখানে আমরা `trpl::channel` ব্যবহার করি, যা অধ্যায় 16-এ thread-এর সঙ্গে ব্যবহার করা multiple-producer, single-consumer channel API-র একটি async সংস্করণ। API-র async সংস্করণ thread-ভিত্তিক
সংস্করণ থেকে সামান্যই আলাদা: এটি immutable-এর বদলে একটি mutable receiver `rx` ব্যবহার করে, আর এর `recv` method সরাসরি মান তৈরি না করে এমন একটি future তৈরি করে, যার জন্য await
করতে হয়।
এখন sender থেকে receiver-এ message পাঠাতে পারি। খেয়াল করুন, একটি আলাদা thread, এমনকি একটি task-ও চালু করতে হয় না; শুধু `rx.recv` call-এর জন্য await করলেই চলে।

`std::mpsc::channel`-এর synchronous `Receiver::recv` method একটি message না পাওয়া পর্যন্ত block করে। `trpl::Receiver::recv` method তা করে না, কারণ এটি async। block না করে এটি runtime-এর
হাতে নিয়ন্ত্রণ ফিরিয়ে দেয়, যতক্ষণ না একটি message আসে বা channel-এর পাঠানোর দিক বন্ধ হয়। বিপরীতে, `send` call-এর জন্য আমরা await করি না, কারণ এটি block করে না। এর দরকারও নেই,
কারণ যে channel-এ পাঠাচ্ছি সেটি unbounded (সীমাহীন)।

> Note: এই সব async কোড যেহেতু একটি `trpl::block_on` call-এর async block-এ চলে, তাই এর ভেতরের সবকিছু block করা এড়াতে পারে। তবে এর _বাইরের_ কোড `block_on` function return হওয়া
> পর্যন্ত block হয়ে থাকবে। `trpl::block_on` function-এর মূল কথাই এটি: এটি আপনাকে _বেছে নিতে_ দেয় async কোডের কোন অংশের জন্য কোথায় block করবেন, এবং তাই কোথায় sync আর async কোডের
> মধ্যে বদল হবে।

এই উদাহরণে দুটি বিষয় খেয়াল করুন। প্রথমত, message সঙ্গে সঙ্গে পৌঁছাবে। দ্বিতীয়ত, এখানে future ব্যবহার করলেও এখনো কোনো concurrency নেই। listing-এর সবকিছু একটির পর একটি ঘটে, ঠিক
যেমনটি ঘটত কোনো future জড়িত না থাকলে।

প্রথম বিষয়টি সমাধান করতে চলুন একাধিক message পাঠাই এবং মাঝে মাঝে sleep করি, যেমনটি Listing 17-10-এ দেখানো হয়েছে।

<!-- We cannot test this one because it never stops! -->

<Listing number="17-10" caption="async channel দিয়ে একাধিক message পাঠানো ও গ্রহণ করা, এবং প্রতিটি message-এর মাঝে `await` দিয়ে sleep করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-10/src/main.rs:many-messages}}
```

</Listing>

message পাঠানোর পাশাপাশি সেগুলো গ্রহণও করতে হবে। এই ক্ষেত্রে কতগুলো message আসছে তা জানি বলে চারবার `rx.recv().await` call করে হাতে সেটা করা যেত। তবে বাস্তবে আমরা সাধারণত _অজানা_
সংখ্যক message-এর জন্য অপেক্ষা করব, তাই আর কোনো message নেই তা নির্ধারণ না করা পর্যন্ত অপেক্ষা করে যেতে হবে।

Listing 16-10-এ একটি synchronous channel থেকে পাওয়া সব item প্রক্রিয়া করতে আমরা একটি `for` loop ব্যবহার করেছিলাম। তবে _asynchronous-ভাবে তৈরি_ item-এর ধারার সঙ্গে `for` loop
ব্যবহারের কোনো উপায় Rust-এ এখনো নেই, তাই আমাদের এমন একটি loop ব্যবহার করতে হবে যা আগে দেখিনি: `while let` শর্তসাপেক্ষ loop। এটি অধ্যায় 6-এর [“`if
let` আর `let...else` দিয়ে সংক্ষিপ্ত control flow”][if-let]<!-- ignore --> অংশে দেখা `if let` গঠনের loop সংস্করণ। এটি যে pattern উল্লেখ করে তা মানের সঙ্গে মিলতে থাকা পর্যন্ত loop চলতে
থাকবে।

`rx.recv` call একটি future তৈরি করে, যার জন্য আমরা await করি। runtime future-টিকে প্রস্তুত না হওয়া পর্যন্ত থামিয়ে রাখবে। একটি message এলে future `Some(message)`-এ পরিণত হবে, যতবার message
আসে ততবার। channel বন্ধ হলে, _কোনো_ message এসেছে কি না তা নির্বিশেষে, future বরং `None`-এ পরিণত হবে, যা বোঝায় যে আর কোনো মান নেই এবং তাই আমাদের polling থামানো উচিত—অর্থাৎ await
করা থামানো উচিত।

`while let` loop এই সবকিছু একসঙ্গে জুড়ে দেয়। `rx.recv().await` call করার ফল যদি `Some(message)` হয়, তাহলে আমরা message-টি পাই এবং loop-এর body-তে ব্যবহার করতে পারি, ঠিক যেমন `if let`-এর
সঙ্গে পারতাম। ফল যদি `None` হয়, loop শেষ হয়। প্রতিবার loop শেষ হলে এটি আবার await point-এ পৌঁছায়, তাই আরেকটি message না আসা পর্যন্ত runtime একে আবার থামিয়ে রাখে।

কোডটি এখন সফলভাবে সব message পাঠায় ও গ্রহণ করে। দুর্ভাগ্যবশত, এখনো কয়েকটি সমস্যা আছে। একটি হলো, message-গুলো আধা সেকেন্ড পরপর আসে না। প্রোগ্রাম শুরু করার 2 সেকেন্ড (2,000
millisecond) পরে সবগুলো একসঙ্গে আসে। আরেকটি হলো, এই প্রোগ্রাম কখনো বন্ধও হয় না! বরং নতুন message-এর জন্য চিরকাল অপেক্ষা করে। আপনাকে <kbd>ctrl</kbd>-<kbd>C</kbd> দিয়ে এটি বন্ধ
করতে হবে।

#### একটি async block-এর কোড সরলরৈখিকভাবে চলে {#code-within-one-async-block-executes-linearly}

চলুন প্রথমে দেখি কেন message-গুলো প্রতিটির মাঝে বিরতি দিয়ে না এসে পুরো বিলম্বের পরে একসঙ্গে আসে। একটি নির্দিষ্ট async block-এর ভেতরে `await` keyword-গুলো কোডে যে ক্রমে থাকে, প্রোগ্রাম চলার
সময় সেগুলো সেই ক্রমেই চলে।

Listing 17-10-এ মাত্র একটি async block আছে, তাই এর ভেতরের সবকিছু সরলরৈখিকভাবে চলে। এখনো কোনো concurrency নেই। সব `tx.send` call ঘটে, মাঝে মাঝে সব `trpl::sleep` call আর তাদের
সংশ্লিষ্ট await point-সহ। কেবল তারপরই `while let` loop `recv` call-গুলোর কোনো `await` point পার হওয়ার সুযোগ পায়।

আমাদের কাঙ্ক্ষিত আচরণ পেতে, যেখানে প্রতিটি message-এর মাঝে sleep-এর বিরতি ঘটে, `tx` আর `rx` operation-গুলোকে তাদের নিজস্ব async block-এ রাখতে হবে, যেমনটি Listing 17-11-এ দেখানো হয়েছে।
তাহলে runtime `trpl::join` ব্যবহার করে প্রতিটি আলাদাভাবে চালাতে পারবে, ঠিক Listing 17-8-এর মতো। আবারও, আমরা আলাদা আলাদা future-এর জন্য নয়, `trpl::join` call করার ফলের জন্য await
করি। আলাদা future-গুলোর জন্য পরপর await করলে আমরা আবার একটি ক্রমানুসারী প্রবাহে ফিরে যেতাম—ঠিক যা আমরা করতে _চাই না_।

<!-- We cannot test this one because it never stops! -->

<Listing number="17-11" caption="`send` আর `recv`-কে তাদের নিজস্ব `async` block-এ আলাদা করে সেই block-গুলোর future-এর জন্য await করা" file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch17-async-await/listing-17-11/src/main.rs:futures}}
```

</Listing>

Listing 17-11-এর হালনাগাদ করা কোডে message-গুলো 2 সেকেন্ড পরে একসঙ্গে হুড়মুড় করে না এসে
500 millisecond পরপর print হয়।

#### async block-এর ভেতরে ownership সরিয়ে নেওয়া {#moving-ownership-into-an-async-block}

তবে `while let` loop যেভাবে `trpl::join`-এর সঙ্গে কাজ করে, তার কারণে প্রোগ্রামটি এখনো কখনো বন্ধ হয় না:

- `trpl::join` থেকে return হওয়া future শেষ হয় কেবল তখনই, যখন তাকে পাঠানো _দুটি_ future-ই শেষ হয়েছে।
- `vals`-এর শেষ message পাঠানোর পর sleep শেষ হলেই `tx_fut` future শেষ হয়।
- `while let` loop শেষ না হওয়া পর্যন্ত `rx_fut` future শেষ হবে না।
- `rx.recv`-এর জন্য await করে `None` না পাওয়া পর্যন্ত `while let` loop শেষ হবে না।
- channel-এর অন্য প্রান্ত বন্ধ হলেই কেবল `rx.recv`-এর জন্য await করলে `None` return হবে।
- channel বন্ধ হবে কেবল তখনই, যখন আমরা `rx.close` call করব, অথবা যখন পাঠানোর দিক, `tx`, drop হবে।
- আমরা কোথাও `rx.close` call করি না, আর `trpl::block_on`-এ পাঠানো সবচেয়ে বাইরের async block শেষ না হওয়া পর্যন্ত `tx` drop হবে না।
- block-টি শেষ হতে পারে না, কারণ এটি `trpl::join` শেষ হওয়ার জন্য block হয়ে আছে, যা আমাদের আবার এই তালিকার শুরুতে নিয়ে যায়।

এই মুহূর্তে যে async block-এ message পাঠাই, সেটি শুধু `tx` _borrow_ করে, কারণ message পাঠাতে ownership দরকার হয় না, কিন্তু যদি `tx`-কে সেই async block-এর ভেতরে _সরিয়ে_ নিতে পারতাম, তাহলে
block শেষ হলেই এটি drop হতো। অধ্যায় 13-এর [“reference capture করা বা ownership সরিয়ে নেওয়া”][capture-or-move]<!-- ignore -->
অংশে আপনি শিখেছেন closure-এর সঙ্গে কীভাবে `move` keyword ব্যবহার করতে হয়, আর অধ্যায় 16-এর [“thread-এর সঙ্গে `move` closure
ব্যবহার”][move-threads]<!-- ignore --> অংশে যেমন আলোচনা করা হয়েছে, thread নিয়ে কাজ করার সময় প্রায়ই closure-এর ভেতরে ডেটা সরিয়ে নিতে হয়। async block-এর ক্ষেত্রেও একই মৌলিক গতিপ্রকৃতি
প্রযোজ্য, তাই `move` keyword closure-এর মতোই async block-এর সঙ্গে কাজ করে।

Listing 17-12-এ আমরা message পাঠানোর block-টি `async` থেকে বদলে
`async move` করি।

<Listing number="17-12" caption="Listing 17-11-এর কোডের একটি সংশোধিত রূপ, যা কাজ শেষে ঠিকমতো বন্ধ হয়" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-12/src/main.rs:with-move}}
```

</Listing>

কোডের _এই_ সংস্করণ চালালে শেষ message পাঠানো ও গ্রহণ করার পর এটি সুন্দরভাবে বন্ধ হয়। এরপর দেখা যাক একাধিক future থেকে ডেটা পাঠাতে কী বদলাতে হবে।

#### `join!` macro দিয়ে কয়েকটি future একসঙ্গে জোড়া {#joining-a-number-of-futures-with-the-join-macro}

এই async channel-টিও একটি multiple-producer channel, তাই একাধিক future থেকে message পাঠাতে চাইলে `tx`-এর ওপর `clone` call করতে পারি, যেমনটি Listing
17-13-এ দেখানো হয়েছে।

<Listing number="17-13" caption="async block-এর সঙ্গে একাধিক producer ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-13/src/main.rs:here}}
```

</Listing>

প্রথমে আমরা `tx` clone করে প্রথম async block-এর বাইরে `tx1` তৈরি করি। আগে `tx`-এর ক্ষেত্রে যেমন করেছিলাম, তেমনভাবে `tx1`-কে সেই block-এ সরিয়ে নিই। তারপর পরে মূল `tx`-কে একটি _নতুন_
async block-এ সরিয়ে নিই, যেখানে একটু বেশি বিলম্বে আরও message পাঠাই। ঘটনাচক্রে এই নতুন async block-টি message গ্রহণের async block-এর পরে রেখেছি, তবে আগেও রাখা যেত। মূল কথা হলো
future-গুলোর জন্য কোন ক্রমে await করা হচ্ছে, কোন ক্রমে তৈরি হচ্ছে তা নয়।

message পাঠানোর দুটি async block-কেই `async move` block হতে হবে, যাতে সেই block-গুলো শেষ হলে `tx` আর `tx1` দুটিই drop হয়। নইলে আমরা শুরুর সেই একই অসীম loop-এ ফিরে যাব।

শেষে বাড়তি future সামলাতে আমরা `trpl::join` থেকে `trpl::join!`-এ বদলাই: `join!` macro যেকোনো সংখ্যক future-এর জন্য await করে, যখন compile time-এ future-এর সংখ্যা জানা থাকে।
অজানা সংখ্যক future-এর একটি collection-এর জন্য await করা নিয়ে এই অধ্যায়ের পরের দিকে আলোচনা করব।

এখন দুটি পাঠানোর future থেকেই সব message দেখি, আর পাঠানোর future-গুলো পাঠানোর পর একটু ভিন্ন বিলম্ব ব্যবহার করে বলে message-গুলোও সেই ভিন্ন ভিন্ন বিরতিতে গৃহীত হয়:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
received 'hi'
received 'more'
received 'from'
received 'the'
received 'messages'
received 'future'
received 'for'
received 'you'
```

আমরা দেখেছি কীভাবে future-গুলোর মধ্যে ডেটা পাঠাতে message passing ব্যবহার করতে হয়, কীভাবে একটি async block-এর কোড ক্রমানুসারে চলে, কীভাবে একটি async block-এ ownership সরিয়ে নিতে হয়,
এবং কীভাবে একাধিক future জোড়া লাগাতে হয়। এরপর চলুন আলোচনা করি runtime-কে কীভাবে এবং কেন বলতে হয় যে এটি অন্য task-এ যেতে পারে।

{{#quiz ../quizzes/async-02-concurrency-with-async.toml}}

[thread-spawn]: ch16-01-threads.html#creating-a-new-thread-with-spawn
[join-handles]: ch16-01-threads.html#waiting-for-all-threads-to-finish
[message-passing-threads]: ch16-02-message-passing.html
[if-let]: ch06-03-if-let.html
[capture-or-move]: ch13-01-closures.html#capturing-references-or-moving-ownership
[move-threads]: ch16-01-threads.html#using-move-closures-with-threads

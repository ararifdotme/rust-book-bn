## পরিশিষ্ট G - Rust কীভাবে তৈরি হয় এবং “Nightly Rust” {#appendix-g---how-rust-is-made-and-nightly-rust}

এই পরিশিষ্ট Rust কীভাবে তৈরি হয় এবং একজন Rust developer হিসেবে তা আপনাকে কীভাবে প্রভাবিত করে, তা নিয়ে।

### স্থবিরতা ছাড়াই স্থিতিশীলতা {#stability-without-stagnation}

একটি ভাষা হিসেবে Rust আপনার কোডের স্থিতিশীলতা নিয়ে _অনেক_ যত্নশীল। আমরা চাই Rust এমন একটি মজবুত ভিত্তি হোক, যার ওপর আপনি কিছু গড়তে পারেন, আর সবকিছু যদি অনবরত বদলাতে থাকত, তাহলে তা অসম্ভব হতো। একই সঙ্গে আমরা যদি নতুন feature নিয়ে পরীক্ষা-নিরীক্ষা করতে না পারি, তাহলে release-এর পরে, যখন আর কিছু বদলানো যায় না, তার আগে হয়তো গুরুত্বপূর্ণ ত্রুটিগুলো খুঁজেই পাব না।

এই সমস্যার জন্য আমাদের সমাধানকে আমরা বলি “স্থবিরতা ছাড়াই স্থিতিশীলতা”, আর আমাদের পথনির্দেশক নীতি হলো: stable Rust-এর নতুন version-এ upgrade করতে আপনার কখনো ভয় পাওয়া উচিত নয়। প্রতিটি upgrade হওয়া উচিত কষ্টহীন, তবে সঙ্গে নতুন feature, কম bug আর দ্রুততর compile time-ও নিয়ে আসা উচিত।

### ঝিক ঝিক! release channel এবং ট্রেনে চড়া {#choo-choo-release-channels-and-riding-the-trains}

Rust-এর development চলে একটি _ট্রেনের সময়সূচি_ অনুযায়ী। অর্থাৎ সব development হয় Rust repository-র main branch-এ। release-গুলো একটি software release train model অনুসরণ করে, যা Cisco IOS আর অন্যান্য software project ব্যবহার করেছে। Rust-এর তিনটি _release channel_ আছে:

- Nightly
- Beta
- Stable

বেশিরভাগ Rust developer মূলত stable channel ব্যবহার করেন, তবে যারা পরীক্ষামূলক নতুন feature চেখে দেখতে চান, তারা nightly বা beta ব্যবহার করতে পারেন।

development আর release প্রক্রিয়া কীভাবে কাজ করে তার একটি উদাহরণ এখানে: ধরা যাক Rust team Rust 1.5-এর release নিয়ে কাজ করছে। সেই release হয়েছিল 2015 সালের ডিসেম্বরে, তবে এটি আমাদের বাস্তবসম্মত version নম্বর দেবে। Rust-এ একটি নতুন feature যোগ হলো: main branch-এ একটি নতুন commit এল। প্রতি রাতে Rust-এর একটি নতুন nightly version তৈরি হয়। প্রতিটি দিনই release-এর দিন, আর এই release-গুলো আমাদের release infrastructure স্বয়ংক্রিয়ভাবে তৈরি করে। তাই সময় গড়ানোর সঙ্গে সঙ্গে আমাদের release-গুলো দেখতে এমন হয়, রাতে একবার করে:

```text
nightly: * - - * - - *
```

প্রতি ছয় সপ্তাহে একটি নতুন release প্রস্তুত করার সময় আসে! Rust repository-র `beta` branch nightly-র ব্যবহৃত main branch থেকে আলাদা হয়ে যায়। এখন দুটি release আছে:

```text
nightly: * - - * - - *
                     |
beta:                *
```

বেশিরভাগ Rust ব্যবহারকারী সক্রিয়ভাবে beta release ব্যবহার করেন না, তবে Rust-কে সম্ভাব্য regression (আগে কাজ করা জিনিস ভেঙে যাওয়া) খুঁজে পেতে সাহায্য করতে নিজেদের CI system-এ beta দিয়ে test চালান। এর মধ্যে তবুও প্রতি রাতে একটি nightly release হয়:

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

ধরা যাক একটি regression পাওয়া গেল। ভাগ্যিস regression-টি চুপিসারে একটি stable release-এ ঢুকে পড়ার আগে beta release test করার কিছু সময় আমাদের হাতে ছিল! সমাধানটি main branch-এ প্রয়োগ করা হয়, যাতে nightly ঠিক হয়ে যায়, তারপর সমাধানটি `beta` branch-এ backport করা হয়, এবং beta-র একটি নতুন release তৈরি হয়:

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

প্রথম beta তৈরির ছয় সপ্তাহ পর একটি stable release-এর সময় আসে! `beta` branch থেকে `stable` branch তৈরি হয়:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

হুররে! Rust 1.5 সম্পন্ন! তবে আমরা একটি জিনিস ভুলে গেছি: ছয় সপ্তাহ পার হয়ে যাওয়ায় Rust-এর _পরের_ version, 1.6-এর একটি নতুন beta-ও দরকার। তাই `beta` থেকে `stable` আলাদা হওয়ার পর `beta`-র পরের version আবার `nightly` থেকে আলাদা হয়:

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

একে “train model” বলা হয়, কারণ প্রতি ছয় সপ্তাহে একটি release “স্টেশন ছেড়ে যায়”, তবে stable release হিসেবে পৌঁছানোর আগে তাকে beta channel-এর মধ্য দিয়ে একটি যাত্রা পাড়ি দিতে হয়।

ঘড়ির কাঁটার মতো নিয়ম করে Rust প্রতি ছয় সপ্তাহে release হয়। একটি Rust release-এর তারিখ জানলে পরেরটির তারিখও জানতে পারেন: ছয় সপ্তাহ পরে। প্রতি ছয় সপ্তাহে release নির্ধারিত থাকার একটি ভালো দিক হলো, পরের ট্রেন শিগগিরই আসছে। কোনো feature কোনো নির্দিষ্ট release ধরতে না পারলে চিন্তার কিছু নেই: অল্প সময়ের মধ্যেই আরেকটি release হচ্ছে! এতে release-এর শেষ সময়ের কাছাকাছি সম্ভাব্য অপরিপক্ব feature চুপিসারে ঢুকিয়ে দেওয়ার চাপ কমে।

এই প্রক্রিয়ার কল্যাণে আপনি সবসময় Rust-এর পরের build দেখে নিজেই যাচাই করতে পারেন যে তাতে upgrade করা সহজ: কোনো beta release প্রত্যাশামতো কাজ না করলে team-কে জানাতে পারেন এবং পরের stable release হওয়ার আগেই তা ঠিক করিয়ে নিতে পারেন! beta release-এ কিছু ভেঙে যাওয়া তুলনামূলক বিরল, তবে `rustc` তবুও একটি software, আর bug থাকেই।

### রক্ষণাবেক্ষণের সময় {#maintenance-time}

Rust project সবচেয়ে সাম্প্রতিক stable version support করে। একটি নতুন stable version release হলে পুরোনো version তার আয়ুষ্কালের শেষে (end of life, EOL) পৌঁছায়। এর মানে প্রতিটি version ছয় সপ্তাহ support পায়।

### unstable feature {#unstable-features}

এই release model-এ আরেকটি ব্যাপার আছে: unstable feature। কোনো নির্দিষ্ট release-এ কোন feature চালু থাকবে তা ঠিক করতে Rust “feature flag” নামের একটি কৌশল ব্যবহার করে। কোনো নতুন feature সক্রিয় development-এর মধ্যে থাকলে সেটি main branch-এ, তাই nightly-তেও, যুক্ত হয়, তবে একটি _feature flag_-এর আড়ালে। একজন ব্যবহারকারী হিসেবে আপনি যদি কাজ চলমান feature-টি চেখে দেখতে চান, তা পারেন, তবে আপনাকে Rust-এর একটি nightly release ব্যবহার করতে হবে এবং বেছে নিতে আপনার source code-এ উপযুক্ত flag দিয়ে annotate করতে হবে।

Rust-এর beta বা stable release ব্যবহার করলে কোনো feature flag ব্যবহার করতে পারবেন না। নতুন feature-গুলোকে চিরকালের জন্য stable ঘোষণা করার আগে সেগুলোর ব্যবহারিক প্রয়োগের সুযোগ পাওয়ার মূল চাবি এটিই। যারা একেবারে সর্বাধুনিক জিনিস বেছে নিতে চান, তারা তা করতে পারেন, আর যারা মজবুত অভিজ্ঞতা চান, তারা stable-এ থেকে নিশ্চিত থাকতে পারেন যে তাদের কোড ভাঙবে না। স্থবিরতা ছাড়াই স্থিতিশীলতা।

এই বইয়ে শুধু stable feature সম্পর্কে তথ্য আছে, কারণ চলমান feature-গুলো এখনো বদলাচ্ছে, এবং এই বই লেখার সময় আর stable build-এ সেগুলো চালু হওয়ার সময়ের মধ্যে নিশ্চয়ই সেগুলো আলাদা হবে। শুধু nightly-তে থাকা feature-এর documentation online-এ পাওয়া যায়।

### Rustup এবং Rust Nightly-র ভূমিকা {#rustup-and-the-role-of-rust-nightly}

Rustup global-ভাবে বা প্রতিটি project-এর ভিত্তিতে Rust-এর বিভিন্ন release channel-এর মধ্যে বদল সহজ করে। default-ভাবে আপনার কাছে stable Rust install থাকবে। যেমন, nightly install করতে:

```console
$ rustup toolchain install nightly
```

`rustup` দিয়ে আপনার install করা সব _toolchain_ (Rust-এর release ও সংশ্লিষ্ট component)-ও দেখতে পারেন। আমাদের একজন লেখকের Windows computer-এ এর একটি উদাহরণ এখানে:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

দেখতেই পাচ্ছেন, stable toolchain হলো default। বেশিরভাগ Rust ব্যবহারকারী বেশিরভাগ সময় stable ব্যবহার করেন। আপনি হয়তো বেশিরভাগ সময় stable ব্যবহার করতে চাইবেন, কিন্তু কোনো নির্দিষ্ট project-এ nightly, কারণ সেখানে একটি একেবারে নতুন feature আপনার দরকার। তা করতে সেই project-এর directory-তে `rustup override` ব্যবহার করে nightly toolchain-কে এমনভাবে ঠিক করে দিতে পারেন, যাতে আপনি সেই directory-তে থাকলে `rustup` সেটিই ব্যবহার করে:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

এখন _~/projects/needs-nightly_-এর ভেতরে যতবার `rustc` বা `cargo` call করবেন, `rustup` নিশ্চিত করবে যে আপনি default stable Rust-এর বদলে nightly Rust ব্যবহার করছেন। আপনার অনেক Rust project থাকলে এটি কাজে আসে!

### RFC প্রক্রিয়া ও team {#the-rfc-process-and-teams}

তাহলে এই নতুন feature-গুলো সম্পর্কে কীভাবে জানবেন? Rust-এর development model একটি _Request For Comments (RFC) প্রক্রিয়া_ অনুসরণ করে। Rust-এ কোনো উন্নতি চাইলে আপনি একটি প্রস্তাব লিখতে পারেন, যাকে RFC বলা হয়।

Rust উন্নত করতে যে কেউ RFC লিখতে পারেন, আর প্রস্তাবগুলো Rust team পর্যালোচনা ও আলোচনা করে, যা বিষয়ভিত্তিক অনেক subteam নিয়ে গঠিত। [Rust-এর website-এ](https://www.rust-lang.org/governance) team-গুলোর একটি পূর্ণ তালিকা আছে, যার মধ্যে project-এর প্রতিটি ক্ষেত্রের team আছে: ভাষার নকশা, compiler implementation, infrastructure, documentation ইত্যাদি। উপযুক্ত team প্রস্তাব আর মন্তব্যগুলো পড়ে, নিজেরাও কিছু মন্তব্য লেখে, এবং শেষ পর্যন্ত feature-টি গ্রহণ বা বাতিল করার বিষয়ে ঐকমত্য হয়।

feature গৃহীত হলে Rust repository-তে একটি issue খোলা হয়, এবং কেউ একজন সেটি implement করতে পারেন। যিনি implement করেন, তিনি খুব সম্ভবত সেই ব্যক্তি নন যিনি প্রথমে feature-টি প্রস্তাব করেছিলেন! implementation প্রস্তুত হলে সেটি একটি feature gate-এর আড়ালে main branch-এ যুক্ত হয়, যেমনটি আমরা [“unstable feature”](#unstable-features)<!-- ignore --> অংশে আলোচনা করেছি।

কিছু সময় পর, nightly release ব্যবহারকারী Rust developer-রা নতুন feature চেখে দেখার সুযোগ পেলে team-এর সদস্যরা feature-টি নিয়ে, nightly-তে এটি কেমন কাজ করেছে তা নিয়ে আলোচনা করবেন, এবং ঠিক করবেন এটি stable Rust-এ যাবে কি না। এগিয়ে যাওয়ার সিদ্ধান্ত হলে feature gate সরিয়ে দেওয়া হয়, আর feature-টি এখন stable বলে গণ্য হয়! এটি ট্রেনে চড়ে Rust-এর একটি নতুন stable release-এ পৌঁছে যায়।

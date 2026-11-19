<!-- Old headings. Do not remove or links may break. -->
<a id="developing-the-librarys-functionality-with-test-driven-development"></a>

## test-driven development দিয়ে কার্যক্ষমতা যোগ করা {#adding-functionality-with-test-driven-development}

খোঁজার logic এখন `main` function থেকে আলাদা হয়ে _src/lib.rs_-এ আছে, তাই আমাদের কোডের মূল কার্যক্ষমতার জন্য test লেখা অনেক সহজ।
command line থেকে binary call না করেই আমরা বিভিন্ন argument দিয়ে সরাসরি function call করতে পারি এবং return মান যাচাই করতে পারি।

এই অংশে আমরা test-driven development (TDD, আগে test লিখে তারপর কোড লেখার পদ্ধতি) প্রক্রিয়া ব্যবহার করে `minigrep` প্রোগ্রামে
খোঁজার logic যোগ করব, এই ধাপগুলো মেনে:

1. এমন একটি test লিখুন যা fail করে, এবং চালিয়ে নিশ্চিত হোন যে এটি আপনার প্রত্যাশিত কারণেই fail করছে।
2. নতুন test-টি pass করানোর জন্য যতটুকু দরকার ঠিক ততটুকু কোড লিখুন বা বদলান।
3. এইমাত্র যোগ করা বা বদলানো কোড refactor করুন, এবং নিশ্চিত হোন যে test-গুলো তখনো pass করছে।
4. ধাপ 1 থেকে আবার শুরু করুন!

software লেখার অনেক উপায়ের মধ্যে এটি একটি মাত্র হলেও TDD কোডের নকশা এগিয়ে নিতে সাহায্য করতে পারে। test pass করানোর কোড লেখার
আগে test লিখলে পুরো প্রক্রিয়া জুড়ে উচ্চ test coverage (কোডের কতটা অংশ test-এর আওতায়) বজায় রাখা যায়।

ফাইলের বিষয়বস্তুতে query string আসলে খোঁজা এবং query-র সঙ্গে মেলে এমন লাইনের একটি তালিকা তৈরি করার কার্যক্ষমতাটি আমরা test-এর
নির্দেশনায় implement করব। এই কার্যক্ষমতা যোগ করব `search` নামের একটি function-এ।

### fail করে এমন একটি test লেখা {#writing-a-failing-test}

_src/lib.rs_-এ আমরা একটি test function-সহ একটি `tests` module যোগ করব, যেমনটি [অধ্যায় 11][ch11-anatomy]<!-- ignore -->-এ
করেছিলাম। test function-টি বলে দেয় আমরা `search` function-এর কাছ থেকে কেমন আচরণ চাই: এটি একটি query আর খোঁজার লেখা নেবে, এবং
লেখার শুধু সেই লাইনগুলো return করবে যাতে query আছে। Listing 12-15-এ এই test দেখানো হয়েছে।

<Listing number="12-15" file-name="src/lib.rs" caption="যে কার্যক্ষমতা আমরা চাই, তার জন্য `search` function-এর একটি fail করা test তৈরি করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

</Listing>

এই test `"duct"` string খোঁজে। যে লেখায় খুঁজছি তা তিন লাইনের, যার শুধু একটিতে `"duct"` আছে (খেয়াল করুন, শুরুর double quote-এর পরের
backslash Rust-কে বলে এই string literal-এর বিষয়বস্তুর শুরুতে কোনো newline অক্ষর না বসাতে)। আমরা assert করি যে `search` function-এর
return করা মানে শুধু আমাদের প্রত্যাশিত লাইনটিই আছে।

এই test এখন চালালে fail করবে, কারণ `unimplemented!` macro “not implemented” message দিয়ে panic করে। TDD-র নীতি অনুযায়ী আমরা একটি
ছোট পদক্ষেপ নেব: `search` function-কে সবসময় একটি খালি vector return করার মতো করে সংজ্ঞায়িত করব, যাতে function call করার সময়
test panic না করে, ঠিক ততটুকু কোড যোগ করব, যেমনটি Listing 12-16-এ দেখানো হয়েছে। তখন test compile হবে এবং fail করবে, কারণ একটি
খালি vector `"safe,
fast, productive."` লাইনযুক্ত vector-এর সঙ্গে মেলে না।

<Listing number="12-16" file-name="src/lib.rs" caption="`search` function ঠিক ততটুকু সংজ্ঞায়িত করা, যাতে call করলে panic না হয়">

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

</Listing>

এবার আলোচনা করা যাক, কেন `search`-এর signature-এ একটি স্পষ্ট lifetime `'a` সংজ্ঞায়িত করে সেই lifetime `contents` argument আর return
মানের সঙ্গে ব্যবহার করতে হবে। [অধ্যায় 10][ch10-lifetimes]<!-- ignore --> থেকে মনে করুন, lifetime parameter বলে দেয় কোন argument-এর
lifetime return মানের lifetime-এর সঙ্গে যুক্ত। এই ক্ষেত্রে আমরা বোঝাচ্ছি যে return করা vector-এ এমন string slice থাকবে যা `contents`
argument-এর (`query` argument-এর নয়) slice-কে নির্দেশ করে।

অন্যভাবে বললে, আমরা Rust-কে বলছি যে `search` function-এর return করা ডেটা ততক্ষণ বাঁচবে, যতক্ষণ `contents` argument-এ `search`
function-এ পাঠানো ডেটা বাঁচে। এটা গুরুত্বপূর্ণ! slice _যে_ ডেটাকে নির্দেশ করে, reference বৈধ থাকতে হলে সেই ডেটাকে বৈধ থাকতে হবে;
compiler যদি ধরে নেয় আমরা `contents`-এর বদলে `query`-এর string slice তৈরি করছি, তাহলে এটি ভুলভাবে নিরাপত্তা যাচাই করবে।

lifetime annotation ভুলে গিয়ে এই function compile করার চেষ্টা করলে আমরা এই error পাব:

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

output-এর জন্য দুটি parameter-এর কোনটি দরকার তা Rust জানতে পারে না, তাই আমাদের স্পষ্ট করে বলে দিতে হবে। খেয়াল করুন, help-এর লেখা
সব parameter আর output type-এর জন্য একই lifetime parameter উল্লেখ করার পরামর্শ দেয়, যা ভুল! `contents` যেহেতু সেই parameter যাতে
আমাদের সব লেখা আছে, আর আমরা সেই লেখার মিলে যাওয়া অংশগুলো return করতে চাই, তাই আমরা জানি lifetime syntax দিয়ে শুধু `contents`
parameter-কেই return মানের সঙ্গে যুক্ত করা উচিত।

অন্য প্রোগ্রামিং ভাষায় signature-এ argument-কে return মানের সঙ্গে যুক্ত করতে হয় না, তবে সময়ের সঙ্গে এই চর্চা সহজ হয়ে আসবে। এই
উদাহরণটি অধ্যায় 10-এর [“lifetime দিয়ে reference যাচাই
করা”][validating-references-with-lifetimes]<!-- ignore --> অংশের উদাহরণগুলোর সঙ্গে তুলনা করে দেখতে পারেন।

### test pass করানোর কোড লেখা {#writing-code-to-pass-the-test}

এই মুহূর্তে আমাদের test fail করছে, কারণ আমরা সবসময় একটি খালি vector return করি। এটি ঠিক করে `search` implement করতে আমাদের
প্রোগ্রামকে এই ধাপগুলো মানতে হবে:

1. বিষয়বস্তুর প্রতিটি লাইনের ওপর দিয়ে iterate করা।
2. লাইনটিতে আমাদের query string আছে কি না যাচাই করা।
3. থাকলে, return করা মানের তালিকায় এটি যোগ করা।
4. না থাকলে, কিছুই না করা।
5. মিলে যাওয়া ফলের তালিকা return করা।

চলুন লাইনগুলোর ওপর দিয়ে iterate করা থেকে শুরু করে প্রতিটি ধাপ একে একে করি।

#### `lines` method দিয়ে লাইনগুলোর ওপর দিয়ে iterate করা {#iterating-through-lines-with-the-lines-method}

string-এর লাইন ধরে ধরে iteration সামলানোর জন্য Rust-এ একটি কাজের method আছে, সুবিধাজনকভাবে যার নাম `lines`, যা Listing 12-17-এর
মতো কাজ করে। খেয়াল করুন, এটি এখনো compile হবে না।

<Listing number="12-17" file-name="src/lib.rs" caption="`contents`-এর প্রতিটি লাইনের ওপর দিয়ে iterate করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

</Listing>

`lines` method একটি iterator return করে। iterator নিয়ে গভীরভাবে আলোচনা করব [অধ্যায় 13][ch13-iterators]<!-- ignore -->-এ। তবে মনে
করুন, iterator ব্যবহারের এই ধরনটি আপনি [Listing 3-5][ch3-iter]<!-- ignore -->-এ দেখেছিলেন, যেখানে একটি collection-এর প্রতিটি item-এর
ওপর কিছু কোড চালাতে iterator-সহ একটি `for` loop ব্যবহার করেছিলাম।

#### প্রতিটি লাইনে query খোঁজা {#searching-each-line-for-the-query}

এরপর আমরা যাচাই করব বর্তমান লাইনে আমাদের query string আছে কি না। সৌভাগ্যক্রমে string-এর `contains` নামে একটি কাজের method আছে, যা
আমাদের হয়ে এটা করে দেয়! `search` function-এ `contains` method-এর একটি call যোগ করুন, যেমনটি Listing 12-18-এ দেখানো হয়েছে।
খেয়াল করুন, এটি এখনো compile হবে না।

<Listing number="12-18" file-name="src/lib.rs" caption="লাইনটিতে `query`-এর string আছে কি না দেখার কার্যক্ষমতা যোগ করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

</Listing>

এই মুহূর্তে আমরা কার্যক্ষমতা গড়ে তুলছি। কোডটি compile করাতে হলে body থেকে একটি মান return করতে হবে, যেমনটি function signature-এ
বলেছি।

#### মিলে যাওয়া লাইন সংরক্ষণ করা {#storing-matching-lines}

এই function শেষ করতে মিলে যাওয়া যে লাইনগুলো return করতে চাই সেগুলো সংরক্ষণের একটি উপায় লাগবে। এজন্য `for` loop-এর আগে একটি
mutable vector বানিয়ে `push` method call করে vector-এ একটি `line` রাখতে পারি। `for` loop-এর পরে vector-টি return করি, যেমনটি
Listing 12-19-এ দেখানো হয়েছে।

<Listing number="12-19" file-name="src/lib.rs" caption="মিলে যাওয়া লাইনগুলো সংরক্ষণ করা, যাতে সেগুলো return করা যায়">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

</Listing>

এখন `search` function-এর শুধু সেই লাইনগুলো return করার কথা যাতে `query` আছে, আর আমাদের test pass করার কথা। চলুন test চালাই:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

আমাদের test pass করেছে, তাই আমরা জানি এটি কাজ করে!

এই পর্যায়ে আমরা test-গুলো pass করিয়ে একই কার্যক্ষমতা বজায় রেখে search function-এর implementation refactor করার সুযোগ বিবেচনা
করতে পারি। search function-এর কোড খুব খারাপ নয়, কিন্তু এটি iterator-এর কিছু কাজের বৈশিষ্ট্যের সুবিধা নেয় না। আমরা এই উদাহরণে
ফিরে আসব [অধ্যায় 13][ch13-iterators]<!-- ignore -->-এ, যেখানে iterator বিস্তারিতভাবে দেখব এবং দেখব কীভাবে এটি উন্নত করা যায়।

এখন পুরো প্রোগ্রামটি কাজ করার কথা! চলুন চালিয়ে দেখি, প্রথমে এমন একটি শব্দ দিয়ে যা Emily Dickinson-এর কবিতা থেকে ঠিক একটি লাইন
return করবে: _frog_।

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

দারুণ! এবার এমন একটি শব্দ দিয়ে চেষ্টা করি যা একাধিক লাইনের সঙ্গে মিলবে, যেমন _body_:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

আর শেষে, নিশ্চিত হই যে কবিতার কোথাও নেই এমন একটি শব্দ, যেমন _monomorphization_, খুঁজলে আমরা কোনো লাইন পাই না:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

চমৎকার! আমরা একটি ক্লাসিক tool-এর নিজস্ব ছোট সংস্করণ বানিয়েছি এবং application-এর গঠন কীভাবে সাজাতে হয় সে সম্পর্কে অনেক কিছু
শিখেছি। ফাইলের input ও output, lifetime, testing আর command line parsing সম্পর্কেও কিছুটা শিখেছি।

এই project পূর্ণ করতে আমরা সংক্ষেপে দেখাব কীভাবে environment variable নিয়ে কাজ করতে হয় এবং কীভাবে standard error-এ print করতে
হয়; দুটিই command line প্রোগ্রাম লেখার সময় কাজে লাগে।

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html

<!-- Old headings. Do not remove or links may break. -->

<a id="comparing-performance-loops-vs-iterators"></a>

## loop বনাম iterator-এর performance {#performance-in-loops-vs-iterators}

loop নাকি iterator ব্যবহার করবেন তা ঠিক করতে আপনাকে জানতে হবে কোন implementation বেশি দ্রুত: স্পষ্ট `for` loop-যুক্ত `search` function-এর
সংস্করণ, নাকি iterator-যুক্ত সংস্করণ।

আমরা একটি benchmark চালিয়েছি: Sir Arthur Conan Doyle-এর _The Adventures of
Sherlock Holmes_-এর পুরো লেখা একটি `String`-এ load করে বিষয়বস্তুতে _the_ শব্দটি খুঁজেছি। `for` loop ব্যবহার করা `search`-এর সংস্করণ আর
iterator ব্যবহার করা সংস্করণের benchmark-এর ফল এই:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

দুটি implementation-এর performance প্রায় একই! benchmark-এর কোড আমরা এখানে ব্যাখ্যা করব না, কারণ উদ্দেশ্য দুটি সংস্করণ যে সমতুল্য তা প্রমাণ
করা নয়, বরং performance-এর দিক থেকে এই দুটি implementation-এর তুলনা সম্পর্কে একটি সাধারণ ধারণা পাওয়া।

আরও পূর্ণাঙ্গ benchmark-এর জন্য `contents` হিসেবে বিভিন্ন আকারের বিভিন্ন লেখা, `query` হিসেবে ভিন্ন ভিন্ন শব্দ ও ভিন্ন দৈর্ঘ্যের শব্দ, এবং
আরও নানা রকম পরিবর্তন দিয়ে যাচাই করা উচিত। মূল কথা হলো: iterator উচ্চ-স্তরের abstraction হলেও compile হয়ে মোটামুটি সেই একই কোডে পরিণত
হয়, যা আপনি নিজে নিম্ন-স্তরের কোড লিখলে হতো। iterator হলো Rust-এর _zero-cost abstractions_ (খরচহীন abstraction)-এর একটি, অর্থাৎ এই
abstraction ব্যবহার করলে runtime-এ কোনো বাড়তি খরচ চাপে না। এটি C++-এর মূল নকশাকার ও implementor Bjarne Stroustrup তাঁর 2012 সালের ETAPS
keynote উপস্থাপনা “Foundations of C++”-এ zero-overhead-কে যেভাবে সংজ্ঞায়িত করেছেন তার সঙ্গে তুলনীয়:

> সাধারণভাবে, C++ implementation zero-overhead নীতি মেনে চলে: যা আপনি ব্যবহার করেন না, তার জন্য আপনাকে মূল্য দিতে হয় না। আর আরও
> বলা যায়: যা আপনি ব্যবহার করেন, তা আপনি হাতে লিখে এর চেয়ে ভালো করতে পারতেন না।

অনেক ক্ষেত্রে iterator ব্যবহার করা Rust কোড compile হয়ে সেই একই assembly-তে পরিণত হয়, যা আপনি হাতে লিখতেন। loop unrolling (loop খুলে
পুনরাবৃত্তিগুলো সরাসরি লিখে দেওয়া) আর array-তে প্রবেশের সময় সীমা যাচাই বাদ দেওয়ার মতো optimization প্রযোজ্য হয় এবং ফলাফলের কোডকে অত্যন্ত দক্ষ
করে তোলে। এটা যেহেতু জানলেন, এখন নির্ভয়ে iterator আর closure ব্যবহার করতে পারেন! এরা কোডকে উচ্চ-স্তরের মনে করায়, কিন্তু তার জন্য runtime-এ
performance-এর কোনো মাশুল দিতে হয় না।

## সারসংক্ষেপ {#summary}

closure আর iterator হলো functional programming ভাষার ধারণা থেকে অনুপ্রাণিত Rust-এর বৈশিষ্ট্য। নিম্ন-স্তরের performance বজায় রেখে উচ্চ-স্তরের
ধারণা পরিষ্কারভাবে প্রকাশ করার Rust-এর ক্ষমতায় এরা অবদান রাখে। closure আর iterator এমনভাবে implement করা যে runtime-এর performance প্রভাবিত
হয় না। এটি zero-cost abstraction দেওয়ার চেষ্টা করার Rust-এর লক্ষ্যের অংশ।

আমাদের I/O project-এর প্রকাশক্ষমতা উন্নত করার পর এবার `cargo`-র আরও কিছু বৈশিষ্ট্য দেখা যাক, যা project-টি বিশ্বের সঙ্গে ভাগ করে নিতে সাহায্য
করবে।

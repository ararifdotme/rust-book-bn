# একটি I/O project: command line প্রোগ্রাম তৈরি {#an-io-project-building-a-command-line-program}

এই অধ্যায়টি এ পর্যন্ত শেখা অনেক দক্ষতার পুনরালোচনা, আর standard library-র আরও কয়েকটি বৈশিষ্ট্যের সঙ্গে পরিচয়। আমরা একটি
command line tool বানাব, যা ফাইল এবং command line-এর input/output-এর সঙ্গে কাজ করে, যাতে এখন আপনার আয়ত্তে থাকা Rust-এর
কিছু ধারণা অনুশীলন করা যায়।

> **Note:** এই অধ্যায়ে কোনো কুইজ নেই, কারণ এটি শুধু হাতে-কলমে ধাপে ধাপে করে দেখার জন্য।

Rust-এর গতি, নিরাপত্তা, একটিমাত্র binary output আর বিভিন্ন platform-এ চলার সুবিধা একে command line tool তৈরির জন্য আদর্শ ভাষা
করে তোলে, তাই আমাদের project-এ আমরা ক্লাসিক command line search tool `grep`
(**g**lobally search a **r**egular **e**xpression and **p**rint)-এর নিজস্ব একটি সংস্করণ বানাব। সবচেয়ে সরল ব্যবহারে `grep`
একটি নির্দিষ্ট ফাইলে একটি নির্দিষ্ট string খোঁজে। এজন্য `grep` argument হিসেবে একটি file path আর একটি string নেয়। তারপর
ফাইলটি পড়ে, সেই ফাইলের যে লাইনগুলোতে string argument-টি আছে সেগুলো খুঁজে বের করে এবং print করে।

চলার পথে আমরা দেখাব কীভাবে আমাদের command line tool-কে terminal-এর সেই বৈশিষ্ট্যগুলো ব্যবহার করানো যায়, যা আরও অনেক command
line tool ব্যবহার করে। ব্যবহারকারী যাতে আমাদের tool-এর আচরণ নিজের মতো সাজাতে পারে, সেজন্য একটি environment variable-এর মান
পড়ব। error message-গুলো standard output (`stdout`)-এর বদলে standard error console stream (`stderr`)-এ print করব, যাতে, যেমন,
ব্যবহারকারী সফল output একটি ফাইলে পাঠিয়ে দিয়েও error message-গুলো স্ক্রিনে দেখতে পারে।

Rust community-র একজন সদস্য, Andrew Gallant, ইতিমধ্যে `grep`-এর পূর্ণাঙ্গ ও খুব দ্রুতগতির একটি সংস্করণ বানিয়েছেন, যার নাম
`ripgrep`। তুলনায় আমাদের সংস্করণটি বেশ সরল হবে, তবে এই অধ্যায় আপনাকে `ripgrep`-এর মতো বাস্তব জীবনের project বোঝার জন্য
প্রয়োজনীয় কিছু পটভূমি জ্ঞান দেবে।

আমাদের `grep` project এ পর্যন্ত শেখা বেশ কিছু ধারণা একসঙ্গে কাজে লাগাবে:

- কোড গুছিয়ে রাখা ([অধ্যায় 7][ch7]<!-- ignore -->)
- vector ও string ব্যবহার ([অধ্যায় 8][ch8]<!-- ignore -->)
- error সামলানো ([অধ্যায় 9][ch9]<!-- ignore -->)
- যেখানে উপযুক্ত সেখানে trait ও lifetime ব্যবহার ([অধ্যায় 10][ch10]<!-- ignore -->)
- test লেখা ([অধ্যায় 11][ch11]<!-- ignore -->)

এছাড়া আমরা সংক্ষেপে closure, iterator ও trait object-এর সঙ্গে পরিচয় করিয়ে দেব, যা [অধ্যায় 13][ch13]<!-- ignore --> ও [অধ্যায়
18][ch18]<!-- ignore -->-এ বিস্তারিত আলোচনা করা হবে।

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch18]: ch18-00-oop.html
